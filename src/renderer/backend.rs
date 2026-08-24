//! The `wgpu` renderer.
//!
//! Drawing records shapes into the current [`RenderTarget`] rather than issuing draw calls, and
//! [`Rendering::present`] tessellates and paints everything recorded since the last frame. A
//! target is an offscreen texture in every case, including a window canvas. That lets a canvas
//! keep its contents between frames, and lets a window be drawn into before the event loop has
//! created it.

use crate::{
    error::{Error, Result},
    image::Image,
    prelude::*,
    renderer::{
        event::Input,
        gpu::{Blit, Gpu, TARGET_FORMAT},
        painter::Painter,
        shapes,
        target::RenderTarget,
        text::TextRenderer,
        window::WindowState,
        RendererSettings, Rendering,
    },
};
use anyhow::{anyhow, Context};
use egui::epaint::{Shape, TextureId as PaintTextureId};
use egui_wgpu::wgpu;
use egui_winit::clipboard::Clipboard;
use log::warn;
use lru::LruCache;
use std::{cell::RefCell, collections::HashMap, collections::VecDeque, fmt};

/// Bytes a texture-to-buffer copy aligns each row to.
const COPY_ROW_ALIGNMENT: u32 = 256;

/// Stroke width every shape outline is drawn at.
///
/// The drawing API sets a stroke color but not a per-shape width, matching what the engine has
/// always drawn. [`Rendering::line`] is the exception and takes its own width.
const STROKE_WEIGHT: u16 = 1;

/// An image uploaded so the canvas can draw it.
pub(super) struct ImageTexture {
    /// Identifier the canvas refers to it by.
    pub(super) id: PaintTextureId,
    /// The allocation pixels are written into.
    texture: wgpu::Texture,
    /// Size in pixels. A resized image is reallocated at the new size.
    size: (u32, u32),
}

/// A `wgpu` [`Renderer`] implementation.
pub(crate) struct Renderer {
    /// Device, queue and the instance surfaces are created from.
    pub(super) gpu: Gpu,
    /// Draws recorded canvas geometry.
    pub(super) painter: Painter,
    /// Copies a canvas onto a surface.
    pub(super) blit: Blit,
    /// Fonts and text layout.
    pub(super) text: TextRenderer,
    /// Settings the primary window was built from, and the title shown without a frame rate.
    pub(super) settings: RendererSettings,
    /// Title currently set on the window, including the frame rate when it is shown.
    pub(super) title: String,
    /// Every open window, whether or not the event loop has created it yet.
    pub(super) windows: HashMap<WindowId, WindowState>,
    /// Order windows were opened in, so the primary window is created first.
    pub(super) window_order: Vec<WindowId>,
    /// Window the first canvas belongs to.
    pub(super) primary_window_id: WindowId,
    /// Window drawing goes to when no texture is targeted.
    pub(super) window_target: WindowId,
    /// Next window identifier to hand out.
    pub(super) next_window_id: u32,
    /// Texture drawing goes to, overriding the window canvas.
    pub(super) texture_target: Option<TextureId>,
    /// Every texture an application created.
    pub(super) textures: HashMap<TextureId, RenderTarget>,
    /// Order textures were created in, so a frame paints them the same way every run.
    pub(super) texture_order: Vec<TextureId>,
    /// Painter textures to release once the frame that may still draw them has been painted.
    pub(super) pending_free: Vec<PaintTextureId>,
    /// Next texture identifier to hand out.
    pub(super) next_texture_id: usize,
    /// Next painter identifier to hand out, shared by targets and images.
    pub(super) next_paint_id: u64,
    /// How subsequent shapes blend against what a target already contains.
    pub(super) blend_mode: BlendMode,
    /// Modifier and cursor state a single `winit` event does not include.
    pub(super) input: Input,
    /// Events translated but not yet polled.
    pub(super) events: VecDeque<Event>,
    /// Cursor requested by the application, or `None` to hide it.
    pub(super) cursor: Option<Cursor>,
    /// Cursor the windows are showing. A request matching it does no work.
    pub(super) shown_cursor: Option<Option<Cursor>>,
    /// Cursors built from an image, so a path is decoded once.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) custom_cursors: HashMap<std::path::PathBuf, winit::window::CustomCursor>,
    /// Textures backing images drawn this frame, keyed by the image they came from.
    pub(super) images: LruCache<*const Image, ImageTexture>,
    /// System clipboard, opened once a window exists to name a display.
    clipboard: RefCell<Option<Clipboard>>,
}

impl Renderer {
    /// Returns the target drawing currently goes to.
    pub(super) fn target(&self) -> Result<&RenderTarget> {
        match self.texture_target {
            Some(id) => self
                .textures
                .get(&id)
                .ok_or_else(|| anyhow!(Error::InvalidTexture(id))),
            None => self
                .windows
                .get(&self.window_target)
                .map(WindowState::canvas)
                .ok_or_else(|| anyhow!(Error::InvalidWindow(self.window_target))),
        }
    }

    /// Returns the target drawing currently goes to.
    pub(super) fn target_mut(&mut self) -> Result<&mut RenderTarget> {
        match self.texture_target {
            Some(id) => self
                .textures
                .get_mut(&id)
                .ok_or_else(|| anyhow!(Error::InvalidTexture(id))),
            None => self
                .windows
                .get_mut(&self.window_target)
                .map(WindowState::canvas_mut)
                .ok_or_else(|| anyhow!(Error::InvalidWindow(self.window_target))),
        }
    }

    /// Hands out the next identifier the painter refers to a texture by.
    pub(super) fn next_paint_id(&mut self) -> PaintTextureId {
        let id = self.next_paint_id;
        self.next_paint_id += 1;
        PaintTextureId::User(id)
    }

    /// Records a shape into the current target.
    ///
    /// Shapes and text always blend on alpha. Tessellated geometry keeps its anti-aliased edge in
    /// the alpha channel, and a glyph is a quad that is transparent outside the letter, so any
    /// other mode paints a box around it. [`BlendMode`] applies to images and textures.
    fn record(&mut self, shape: Shape, smooth: bool) -> Result<()> {
        self.target_mut()?.record(shape, BlendMode::Blend, smooth);
        Ok(())
    }

    /// Records a shape into the current target under the configured blend mode.
    pub(super) fn record_blended(&mut self, shape: Shape, smooth: bool) -> Result<()> {
        let blend_mode = self.blend_mode;
        self.target_mut()?.record(shape, blend_mode, smooth);
        Ok(())
    }

    /// Uploads an image and returns the identifier the canvas draws it by.
    ///
    /// The pixels are rewritten every call, because an application is free to mutate an image
    /// between frames and the cache is keyed by where the image lives, not by its contents.
    fn upload_image(&mut self, img: &Image) -> PaintTextureId {
        let key: *const Image = img;
        let size = (img.width(), img.height());
        let stale = self
            .images
            .peek(&key)
            .is_some_and(|cached| cached.size != size);
        if stale {
            if let Some(cached) = self.images.pop(&key) {
                self.painter.free_texture(cached.id);
            }
        }
        if self.images.peek(&key).is_none() {
            let id = self.next_paint_id();
            let texture = self.gpu.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("pix-engine image"),
                size: wgpu::Extent3d {
                    width: size.0.max(1),
                    height: size.1.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: TARGET_FORMAT,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.painter
                .register_texture(&self.gpu.device, id, &view, wgpu::FilterMode::Nearest);
            if let Some((_, evicted)) = self.images.push(key, ImageTexture { id, texture, size }) {
                self.painter.free_texture(evicted.id);
            }
        }

        #[allow(clippy::expect_used)]
        let cached = self.images.get(&key).expect("image was just cached");
        let pixels = rgba(img);
        self.gpu.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &cached.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.0.max(1)),
                rows_per_image: Some(size.1.max(1)),
            },
            wgpu::Extent3d {
                width: size.0.max(1),
                height: size.1.max(1),
                depth_or_array_layers: 1,
            },
        );
        cached.id
    }

    /// Releases what the event loop owns, before the loop tears itself down.
    ///
    /// The clipboard holds Wayland objects belonging to a window's display connection, so it has
    /// to go while that connection is still open.
    pub(crate) fn shut_down(&mut self) {
        self.clipboard.borrow_mut().take();
    }

    /// Opens the system clipboard, naming the display the given window is on.
    pub(super) fn open_clipboard(&self, window: &winit::window::Window) {
        use winit::raw_window_handle::HasDisplayHandle;
        if self.clipboard.borrow().is_some() {
            return;
        }
        let display = window.display_handle().ok().map(|handle| handle.as_raw());
        *self.clipboard.borrow_mut() = Some(Clipboard::new(display));
    }
}

impl Rendering for Renderer {
    /// Opens a graphics device and prepares the primary canvas.
    ///
    /// The window itself is created once the event loop is running. Drawing before that point
    /// records into the canvas as usual and shows up on the first frame the window is present for.
    fn new(settings: RendererSettings) -> Result<Self> {
        let gpu = Gpu::new()?;
        let painter = Painter::new(&gpu.device, TARGET_FORMAT);
        let blit = Blit::new(&gpu.device);
        let text = TextRenderer::new(gpu.device.limits().max_texture_dimension_2d as usize)?;

        let title = settings.title.clone();
        let primary_window_id = WindowId(1);
        let mut renderer = Self {
            gpu,
            painter,
            blit,
            text,
            images: LruCache::new(settings.texture_cache_size),
            settings,
            title,
            windows: HashMap::new(),
            window_order: Vec::new(),
            primary_window_id,
            window_target: primary_window_id,
            next_window_id: primary_window_id.0 + 1,
            texture_target: None,
            textures: HashMap::new(),
            texture_order: Vec::new(),
            pending_free: Vec::new(),
            next_texture_id: 0,
            next_paint_id: 0,
            blend_mode: BlendMode::None,
            input: Input::default(),
            events: VecDeque::new(),
            cursor: Some(Cursor::default()),
            shown_cursor: None,
            #[cfg(not(target_arch = "wasm32"))]
            custom_cursors: HashMap::new(),
            clipboard: RefCell::new(None),
        };
        let settings = renderer.settings.clone();
        renderer.open_canvas(primary_window_id, settings);
        Ok(renderer)
    }

    fn clear(&mut self) -> Result<()> {
        self.target_mut()?.clear();
        Ok(())
    }

    fn set_draw_color(&mut self, color: Color) -> Result<()> {
        self.target_mut()?.set_background(color);
        Ok(())
    }

    fn clip(&mut self, rect: Option<Rect<i32>>) -> Result<()> {
        self.target_mut()?.set_clip(rect);
        Ok(())
    }

    fn blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode;
    }

    fn present(&mut self) {
        if let Err(err) = self.paint_frame() {
            warn!("failed to present a frame: {err}");
        }
    }

    fn scale(&mut self, x: f32, y: f32) -> Result<()> {
        self.target_mut()?.set_scale(x, y);
        Ok(())
    }

    fn font_size(&mut self, size: u32) -> Result<()> {
        self.text.set_size(size);
        Ok(())
    }

    fn font_style(&mut self, style: FontStyle) {
        self.text.set_style(style);
    }

    fn font_family(&mut self, font: &Font) -> Result<()> {
        self.text.set_family(font)
    }

    fn clipboard_text(&self) -> String {
        self.clipboard
            .borrow_mut()
            .as_mut()
            .and_then(Clipboard::get)
            .unwrap_or_default()
    }

    fn set_clipboard_text(&self, value: &str) -> Result<()> {
        match self.clipboard.borrow_mut().as_mut() {
            Some(clipboard) => {
                clipboard.set_text(value.to_owned());
                Ok(())
            }
            None => Err(anyhow!("no clipboard is available")),
        }
    }

    fn open_url(&self, url: &str) -> Result<()> {
        webbrowser::open(url).with_context(|| format!("failed to open url {url}"))
    }

    fn text(
        &mut self,
        position: Point<i32>,
        text: &str,
        wrap_width: Option<u32>,
        angle: Option<f64>,
        _center: Option<Point<i32>>,
        _flipped: Option<Flipped>,
        fill: Option<Color>,
        outline: u16,
    ) -> Result<(u32, u32)> {
        let size = self.text.size_of(text, wrap_width);
        let Some(fill) = fill else {
            return Ok(size);
        };
        if text.is_empty() {
            return Ok(size);
        }
        // An outline draws the halo in the fill color and the glyphs on top in the same color,
        // matching how the drawing API asks for stroked text: one pass per color.
        let galley = self.text.layout(text, wrap_width, fill);
        #[allow(clippy::cast_possible_truncation)]
        let angle = angle.unwrap_or(0.0).to_radians() as f32;
        let shape = self
            .text
            .shape(position, &galley, angle, fill, outline, fill);
        self.record(shape, true)?;
        Ok(size)
    }

    fn size_of(&self, text: &str, wrap_width: Option<u32>) -> Result<(u32, u32)> {
        Ok(self.text.size_of(text, wrap_width))
    }

    fn point(&mut self, p: Point<i32>, color: Color) -> Result<()> {
        self.target_mut()?.record_point(p, color, BlendMode::Blend);
        Ok(())
    }

    fn line(&mut self, line: Line<i32>, smooth: bool, width: u8, color: Color) -> Result<()> {
        self.record(shapes::line(line, color, u16::from(width)), smooth)
    }

    fn bezier<I>(&mut self, ps: I, detail: i32, stroke: Option<Color>) -> Result<()>
    where
        I: Iterator<Item = Point<i32>>,
    {
        let Some(stroke) = stroke else {
            return Ok(());
        };
        let control: Vec<Point<i32>> = ps.collect();
        self.record(
            shapes::bezier(&control, detail, stroke, STROKE_WEIGHT),
            true,
        )
    }

    fn triangle(
        &mut self,
        tri: Tri<i32>,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        self.record(shapes::triangle(tri, fill, stroke, STROKE_WEIGHT), smooth)
    }

    fn rect(
        &mut self,
        rect: Rect<i32>,
        radius: Option<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let smooth = radius.is_some();
        self.record(
            shapes::rectangle(rect, radius.unwrap_or(0), fill, stroke, STROKE_WEIGHT),
            smooth,
        )
    }

    fn quad(
        &mut self,
        quad: Quad<i32>,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        self.record(shapes::quad(quad, fill, stroke, STROKE_WEIGHT), smooth)
    }

    fn polygon<I>(
        &mut self,
        ps: I,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>
    where
        I: Iterator<Item = Point<i32>>,
    {
        let vertices: Vec<Point<i32>> = ps.collect();
        self.record(
            shapes::polygon(&vertices, fill, stroke, STROKE_WEIGHT),
            smooth,
        )
    }

    fn ellipse(
        &mut self,
        ellipse: Ellipse<i32>,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        self.record(
            shapes::ellipse(ellipse, fill, stroke, STROKE_WEIGHT),
            smooth,
        )
    }

    fn arc(
        &mut self,
        p: Point<i32>,
        radius: i32,
        start: i32,
        end: i32,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        #[allow(clippy::cast_precision_loss)]
        let (start, end) = ((start as f32).to_radians(), (end as f32).to_radians());
        let shape = shapes::arc(
            p,
            radius,
            start,
            end,
            matches!(mode, ArcMode::Pie),
            fill,
            stroke,
            STROKE_WEIGHT,
        );
        self.record(shape, true)
    }

    fn image(
        &mut self,
        img: &Image,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: f64,
        center: Option<Point<i32>>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> Result<()> {
        let texture = self.upload_image(img);
        #[allow(clippy::cast_possible_wrap)]
        let bounds = Rect::new(0, 0, img.width() as i32, img.height() as i32);
        let target_size = self.target()?.size();
        #[allow(clippy::cast_possible_wrap)]
        let full = Rect::new(0, 0, target_size[0] as i32, target_size[1] as i32);
        let shape = shapes::textured(
            texture,
            (img.width(), img.height()),
            src.unwrap_or(bounds),
            dst.unwrap_or(full),
            angle,
            center,
            flipped,
            tint,
        );
        self.record_blended(shape, false)
    }

    /// Returns the current target's pixels, painting what is recorded first.
    ///
    /// Drawing only reaches a target when the frame is painted, so a caller reading back
    /// mid-frame would otherwise see the frame before this one.
    fn to_bytes(&mut self) -> Result<Vec<u8>> {
        self.paint_current_target()?;
        let (size, texture) = {
            let target = self.target()?;
            (target.size(), target.texture().clone())
        };
        let mut pixels = self.read_texture(&texture, size)?;
        // A target stores premultiplied color. A caller writing a PNG or handing the bytes back to
        // `update_texture` wants straight alpha.
        for pixel in pixels.chunks_exact_mut(4) {
            let [r, g, b, a] = [pixel[0], pixel[1], pixel[2], pixel[3]];
            let unscale = |channel: u8| {
                if a == 0 {
                    0
                } else {
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        (u32::from(channel) * 255 / u32::from(a)).min(255) as u8
                    }
                }
            };
            pixel[0] = unscale(r);
            pixel[1] = unscale(g);
            pixel[2] = unscale(b);
        }
        Ok(pixels)
    }
}

impl Renderer {
    /// Copies a texture back into memory as tightly packed RGBA bytes.
    ///
    /// A copy to a buffer pads every row out to [`COPY_ROW_ALIGNMENT`], so the padding is stripped
    /// on the way out.
    fn read_texture(&self, texture: &wgpu::Texture, size: [u32; 2]) -> Result<Vec<u8>> {
        let [width, height] = size;
        let unpadded = 4 * width;
        let padded = unpadded.div_ceil(COPY_ROW_ALIGNMENT) * COPY_ROW_ALIGNMENT;
        let buffer = self.gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("pix-engine readback"),
            size: u64::from(padded) * u64::from(height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("pix-engine readback encoder"),
            });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        self.gpu.queue.submit([encoder.finish()]);

        let slice = buffer.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| ());
        self.gpu
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .context("failed to read back the render target")?;

        let mapped = slice.get_mapped_range();
        let mut pixels = Vec::with_capacity((unpadded * height) as usize);
        for row in 0..height {
            let start = (row * padded) as usize;
            pixels.extend_from_slice(&mapped[start..start + unpadded as usize]);
        }
        drop(mapped);
        buffer.unmap();
        Ok(pixels)
    }
}

impl fmt::Debug for Renderer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Renderer")
            .field("title", &self.title)
            .field("settings", &self.settings)
            .field("blend_mode", &self.blend_mode)
            .field("primary_window_id", &self.primary_window_id)
            .field("window_target", &self.window_target)
            .field("texture_target", &self.texture_target)
            .field("windows", &self.window_order)
            .field("texture_count", &self.textures.len())
            .finish_non_exhaustive()
    }
}

/// Returns an image's pixels as premultiplied RGBA, widening a three-channel image.
fn rgba(img: &Image) -> Vec<u8> {
    match img.format() {
        PixelFormat::Rgba => img
            .as_bytes()
            .chunks_exact(4)
            .flat_map(premultiply)
            .collect(),
        PixelFormat::Rgb => img
            .as_bytes()
            .chunks_exact(3)
            .flat_map(|pixel| [pixel[0], pixel[1], pixel[2], u8::MAX])
            .collect(),
    }
}

/// Scales a straight-alpha pixel's channels by its alpha.
///
/// Every color in the pipeline is premultiplied: `epaint` produces colors that way and the blend
/// states expect it. Image and texture data arrives straight from the application, so it is
/// converted on the way in.
pub(super) fn premultiply(pixel: &[u8]) -> [u8; 4] {
    let alpha = u32::from(pixel[3]);
    let scale = |channel: u8| {
        #[allow(clippy::cast_possible_truncation)]
        {
            (u32::from(channel) * alpha / 255) as u8
        }
    };
    [scale(pixel[0]), scale(pixel[1]), scale(pixel[2]), pixel[3]]
}
