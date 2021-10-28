use crate::{
    prelude::*,
    renderer::{RendererSettings, Rendering},
    window::WindowRenderer,
};
use anyhow::Context;
use lazy_static::lazy_static;
use lru::LruCache;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    gfx::primitives::{DrawRenderer, ToColor},
    image::{InitFlag, Sdl2ImageContext},
    mouse::{Cursor, SystemCursor},
    pixels::{Color as SdlColor, PixelFormatEnum as SdlPixelFormat},
    rect::{Point as SdlPoint, Rect as SdlRect},
    render::{BlendMode as SdlBlendMode, Canvas, TextureCreator, TextureQuery},
    rwops::RWops,
    ttf::{Font as SdlFont, FontStyle as SdlFontStyle, Sdl2TtfContext},
    video::{Window as SdlWindow, WindowContext},
    EventPump, Sdl,
};
use std::{cmp, collections::HashMap};

mod audio;
mod event;
mod textures;
mod window;

pub(crate) use textures::RendererTexture;
pub(crate) type WindowCanvas = Canvas<SdlWindow>;

lazy_static! {
    static ref TTF: Sdl2TtfContext = sdl2::ttf::init().expect("sdl2_ttf initialized");
    static ref IMAGE: Sdl2ImageContext =
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("sdl2_image initialized");
}

/// Allows optionally updating the main window canvas, or targeting a given texture target
macro_rules! update_canvas {
    ($self:expr, $func:expr) => {{
        let (canvas, _) = $self
            .canvases
            .get_mut(&$self.window_target)
            .ok_or(PixError::InvalidWindow($self.window_target))?;
        if let Some(texture_id) = $self.texture_target {
            if let Some((_, texture)) = $self.textures.get_mut(texture_id) {
                let mut result = Ok(());
                canvas
                    .with_texture_canvas(texture, |canvas| {
                        result = $func(canvas);
                    })
                    .with_context(|| format!("failed to update texture target {}", texture_id))?;
                result
            } else {
                Err(PixError::InvalidTexture(texture_id).into())
            }
        } else {
            $func(canvas)
        }
    }};
}

/// Update font cache
macro_rules! update_font_cache {
    ($font:expr, $cache:expr) => {{
        let name = $font.0.name;
        let size = $font.1;
        let key = (name, size);
        if !$cache.contains(&key) {
            let font = match $font.0.source {
                FontSrc::Bytes(bytes) => {
                    let rwops = RWops::from_bytes(bytes).map_err(PixError::Renderer)?;
                    TTF.load_font_from_rwops(rwops, size)
                        .map_err(PixError::Renderer)?
                }
                FontSrc::Path(ref path) => TTF.load_font(path, size).map_err(PixError::Renderer)?,
            };
            $cache.put(key, font);
        }
    }};
}

/// An SDL [Renderer] implementation.
pub(crate) struct Renderer {
    context: Sdl,
    event_pump: EventPump,
    audio_device: AudioQueue<f32>,
    title: String,
    fps: usize,
    settings: RendererSettings,
    cursor: Cursor,
    blend_mode: SdlBlendMode,
    font: (Font, u16),
    font_style: SdlFontStyle,
    window_id: WindowId,
    window_target: WindowId,
    texture_target: Option<TextureId>,
    canvases: HashMap<WindowId, (WindowCanvas, TextureCreator<WindowContext>)>,
    textures: Vec<(WindowId, RendererTexture)>,
    font_cache: LruCache<(&'static str, u16), SdlFont<'static, 'static>>,
    text_cache: LruCache<(WindowId, String, Color), RendererTexture>,
    image_cache: LruCache<(WindowId, *const Image), RendererTexture>,
}

impl Rendering for Renderer {
    /// Initializes the Sdl2Renderer using the given settings and opens a new window.
    fn new(s: RendererSettings) -> PixResult<Self> {
        let context = sdl2::init().map_err(PixError::Renderer)?;
        let event_pump = context.event_pump().map_err(PixError::Renderer)?;

        let title = s.title.clone();
        let (window_id, canvas) = Self::create_window_canvas(&context, &s)?;
        let cursor = Cursor::from_system(SystemCursor::Arrow).map_err(PixError::Renderer)?;
        cursor.set();
        let texture_creator = canvas.texture_creator();
        let mut canvases = HashMap::new();
        canvases.insert(window_id, (canvas, texture_creator));

        // Set up Audio
        let audio_sub = context.audio().map_err(PixError::Renderer)?;
        let desired_spec = AudioSpecDesired {
            freq: Some(s.audio_sample_rate),
            channels: Some(1),
            samples: None,
        };
        let audio_device = audio_sub
            .open_queue(None, &desired_spec)
            .map_err(PixError::Renderer)?;
        audio_device.resume();

        let font = (Font::default(), 14);
        let mut font_cache = LruCache::new(s.texture_cache_size);
        update_font_cache!(font, font_cache);
        let text_cache = LruCache::new(s.text_cache_size);
        let image_cache = LruCache::new(s.texture_cache_size);

        Ok(Self {
            context,
            event_pump,
            audio_device,
            settings: s,
            title,
            fps: 0,
            cursor,
            blend_mode: SdlBlendMode::None,
            font,
            font_style: SdlFontStyle::NORMAL,
            window_id,
            window_target: window_id,
            texture_target: None,
            canvases,
            textures: Vec::new(),
            font_cache,
            text_cache,
            image_cache,
        })
    }

    /// Clears the canvas to the current clear color.
    #[inline]
    fn clear(&mut self) -> PixResult<()> {
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            canvas.clear();
            Ok(())
        })
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    #[inline]
    fn set_draw_color(&mut self, color: Color) -> PixResult<()> {
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            canvas.set_draw_color(color);
            Ok(())
        })
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    #[inline]
    fn clip(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        let rect = rect.map(|rect| rect.into());
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            canvas.set_clip_rect(rect);
            Ok(())
        })
    }

    /// Sets the blend mode used by the renderer to drawing.
    #[inline]
    fn blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode.into();
    }

    /// Updates the canvas from the current back buffer.
    #[inline]
    fn present(&mut self) {
        for (canvas, _) in self.canvases.values_mut() {
            canvas.present();
        }
    }

    /// Scale the current canvas.
    #[inline]
    fn scale(&mut self, x: f32, y: f32) -> PixResult<()> {
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            Ok(canvas.set_scale(x, y).map_err(PixError::Renderer)?)
        })
    }

    /// Set the font size for drawing to the current canvas.
    #[inline]
    fn font_size(&mut self, size: u32) -> PixResult<()> {
        let size = size as u16;
        if self.font.1 != size {
            self.font.1 = size;
            update_font_cache!(self.font, self.font_cache);
        }
        Ok(())
    }

    /// Set the font style for drawing to the current canvas.
    #[inline]
    fn font_style(&mut self, style: FontStyle) {
        let key = (self.font.0.name, self.font.1);
        if let Some(font) = self.font_cache.get_mut(&key) {
            let style = style.into();
            if self.font_style != style {
                self.font_style = style;
                font.set_style(style);
            }
        }
    }

    /// Set the font family for drawing to the current canvas.
    #[inline]
    fn font_family(&mut self, font: &Font) -> PixResult<()> {
        if self.font.0.name != font.name {
            self.font.0 = font.clone();
            update_font_cache!(self.font, self.font_cache);
        }
        Ok(())
    }

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        pos: PointI2,
        text: &str,
        wrap_width: Option<u32>,
        angle: Option<Scalar>,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        fill: Option<Color>,
        outline: u8,
    ) -> PixResult<(u32, u32)> {
        if text.is_empty() {
            return Ok((0, 0));
        }

        let key = (self.font.0.name, self.font.1);
        if !self.font_cache.contains(&key) {
            update_font_cache!(self.font, self.font_cache);
        }
        let (win_width, _) = self.dimensions()?;
        let font = self.font_cache.get_mut(&key).expect("valid font");

        if let Some(fill) = fill {
            let current_outline = font.get_outline_width();
            if current_outline != outline as u16 {
                font.set_outline_width(outline as u16);
            }

            let key = (self.window_target, text.to_string(), fill);
            let (_, texture_creator) = self
                .canvases
                .get(&self.window_target)
                .ok_or(PixError::InvalidWindow(self.window_target))?;
            if !self.text_cache.contains(&key) {
                let surface = if let Some(width) = wrap_width {
                    font.render(text).blended_wrapped(fill, width)
                } else {
                    font.render(text).blended_wrapped(fill, win_width)
                };
                let surface = surface.context("invalid text")?;
                self.text_cache.put(
                    key.clone(),
                    texture_creator
                        .create_texture_from_surface(&surface)
                        .context("failed to create text surface")?,
                );
            }
            // SAFETY: We just checked or inserted a texture.
            let texture = self.text_cache.get_mut(&key).expect("valid text cache");
            let TextureQuery { width, height, .. } = texture.query();
            update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
                let result = if angle.is_some() || center.is_some() || flipped.is_some() {
                    canvas.copy_ex(
                        texture,
                        None,
                        Some(SdlRect::new(pos.x(), pos.y(), width, height)),
                        angle.unwrap_or(0.0),
                        center.map(|c| c.into()),
                        matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                        matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                    )
                } else {
                    canvas.copy(
                        texture,
                        None,
                        Some(SdlRect::new(pos.x(), pos.y(), width, height)),
                    )
                };
                Ok(result.map_err(PixError::Renderer)?)
            })?;
            Ok((width, height))
        } else {
            Ok((0, 0))
        }
    }

    /// Get clipboard text from the system clipboard.
    fn clipboard_text(&self) -> String {
        if let Ok(video) = self.context.video() {
            video.clipboard().clipboard_text().unwrap_or_default()
        } else {
            String::default()
        }
    }

    /// Set clipboard text to the system clipboard.
    fn set_clipboard_text(&self, value: &str) -> PixResult<()> {
        Ok(self
            .context
            .video()
            .map_err(PixError::Renderer)?
            .clipboard()
            .set_clipboard_text(value)
            .map_err(PixError::Renderer)?)
    }

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    #[inline]
    fn size_of(&mut self, text: &str) -> PixResult<(u32, u32)> {
        let key = (self.font.0.name, self.font.1);
        if !self.font_cache.contains(&key) {
            update_font_cache!(self.font, self.font_cache);
        }
        let font = self.font_cache.get(&key).expect("valid font");

        if text.is_empty() {
            return Ok(font.size_of("").unwrap_or_default());
        }
        let mut size = (0, 0);
        for line in text.lines() {
            let (w, h) = font.size_of(line).context("failed to get text size")?;
            size.0 = cmp::max(size.0, w);
            size.1 += h;
        }
        Ok(size)
    }

    /// Draw a pixel to the current canvas.
    #[inline]
    fn point(&mut self, p: PointI2, color: Color) -> PixResult<()> {
        let p: Point<i16, 2> = p.into();
        let [x, y]: [i16; 2] = p.into();
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            Ok(canvas.pixel(x, y, color).map_err(PixError::Renderer)?)
        })
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, line: LineI2, stroke: u8, color: Color) -> PixResult<()> {
        let [start, end]: [Point<i16, 2>; 2] = line.into();
        let [x1, y1] = start.values();
        let [x2, y2] = end.values();
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            let result = if stroke == 1 {
                if y1 == y2 {
                    canvas.hline(x1, x2, y1, color)
                } else if x1 == x2 {
                    canvas.vline(x1, y1, y2, color)
                } else {
                    canvas.aa_line(x1, y1, x2, y2, color)
                }
            } else {
                canvas.thick_line(x1, y1, x2, y2, stroke, color)
            };
            Ok(result.map_err(PixError::Renderer)?)
        })
    }

    /// Draw a triangle to the current canvas.
    fn triangle(
        &mut self,
        tri: TriI2,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        let [p1, p2, p3]: [Point<i16, 2>; 3] = tri.into();
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            if let Some(fill) = fill {
                canvas
                    .filled_trigon(p1.x(), p1.y(), p2.x(), p2.y(), p3.x(), p3.y(), fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .trigon(p1.x(), p1.y(), p2.x(), p2.y(), p3.x(), p3.y(), stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        rect: Rect<i32>,
        radius: Option<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        let rect: Rect<i16> = rect.into();
        let [x, y, width, height] = rect.values();
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            if let Some(radius) = radius {
                let radius = radius as i16;
                if let Some(fill) = fill {
                    canvas
                        .rounded_box(x, y, x + width, y + height, radius, fill)
                        .map_err(PixError::Renderer)?;
                }
                if let Some(stroke) = stroke {
                    canvas
                        .rounded_rectangle(x, y, x + width, y + height, radius, stroke)
                        .map_err(PixError::Renderer)?;
                }
            } else {
                if let Some(fill) = fill {
                    canvas
                        .box_(x, y, x + width, y + height, fill)
                        .map_err(PixError::Renderer)?;
                }
                if let Some(stroke) = stroke {
                    // EXPL: SDL2_gfx renders this 1px smaller than it should.
                    canvas
                        .rectangle(x, y, x + width + 1, y + height + 1, stroke)
                        .map_err(PixError::Renderer)?;
                }
            }
            Ok(())
        })
    }

    /// Draw a quadrilateral to the current canvas.
    fn quad(&mut self, quad: QuadI2, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()> {
        let [p1, p2, p3, p4]: [Point<i16, 2>; 4] = quad.into();
        let vx = [p1.x(), p2.x(), p3.x(), p4.x()];
        let vy = [p1.y(), p2.y(), p3.y(), p4.y()];
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            if let Some(fill) = fill {
                canvas
                    .filled_polygon(&vx, &vy, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .polygon(&vx, &vy, stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw a polygon to the current canvas.
    fn polygon(
        &mut self,
        ps: &[PointI2],
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        let (vx, vy): (Vec<i16>, Vec<i16>) = ps
            .iter()
            .map(|&p| -> (i16, i16) {
                let p: Point<i16, 2> = p.into();
                let [x, y] = p.values();
                (x, y)
            })
            .unzip();
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            if let Some(fill) = fill {
                canvas
                    .filled_polygon(&vx, &vy, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .polygon(&vx, &vy, stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: Ellipse<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        let ellipse: Ellipse<i16> = ellipse.into();
        let [x, y, width, height] = ellipse.values();
        let rw = width / 2;
        let rh = height / 2;
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            if let Some(fill) = fill {
                canvas
                    .filled_ellipse(x, y, rw, rh, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .aa_ellipse(x, y, rw, rh, stroke)
                    .map_err(PixError::Renderer)?;
            }
            Ok(())
        })
    }

    /// Draw an arc to the current canvas.
    fn arc(
        &mut self,
        p: PointI2,
        radius: i32,
        start: i32,
        end: i32,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        let p: Point<i16, 2> = p.into();
        let [x, y] = p.values();
        let radius = radius as i16;
        let start = start as i16;
        let end = end as i16;
        self.update_canvas(|canvas: &mut WindowCanvas| -> PixResult<()> {
            match mode {
                ArcMode::Default => {
                    if let Some(stroke) = stroke {
                        canvas
                            .arc(x, y, radius, start, end, stroke)
                            .map_err(PixError::Renderer)?;
                    }
                }
                ArcMode::Pie => {
                    if let Some(fill) = fill {
                        canvas
                            .filled_pie(x, y, radius, start, end, fill)
                            .map_err(PixError::Renderer)?;
                    }
                    if let Some(stroke) = stroke {
                        canvas
                            .pie(x, y, radius, start, end, stroke)
                            .map_err(PixError::Renderer)?;
                    }
                }
            }
            Ok(())
        })
    }

    /// Draw an image to the current canvas, optionally rotated about a `center`, flipped or tinted
    fn image(
        &mut self,
        img: &Image,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> PixResult<()> {
        let (_, texture_creator) = self
            .canvases
            .get(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?;
        let img_ptr: *const Image = img;
        let key = (self.window_target, img_ptr);
        if !self.image_cache.contains(&key) {
            self.image_cache.put(
                key,
                texture_creator
                    .create_texture_static(Some(img.format().into()), img.width(), img.height())
                    .context("failed to create image texture")?,
            );
        }
        // SAFETY: We just checked or inserted a texture.
        let texture = self.image_cache.get_mut(&key).expect("valid image cache");
        match tint {
            Some(tint) => {
                let [r, g, b, a] = tint.channels();
                texture.set_color_mod(r, g, b);
                texture.set_alpha_mod(a);
            }
            None => {
                texture.set_color_mod(255, 255, 255);
                texture.set_alpha_mod(255);
            }
        }
        texture.set_blend_mode(self.blend_mode);
        texture
            .update(
                None,
                img.as_bytes(),
                img.format().channels() * img.width() as usize,
            )
            .context("failed to update image texture")?;
        let src = src.map(|r| r.into());
        let dst = dst.map(|r| r.into());
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let result = if angle > 0.0 || center.is_some() || flipped.is_some() {
                canvas.copy_ex(
                    texture,
                    src,
                    dst,
                    angle,
                    center.map(|c| c.into()),
                    matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                    matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                )
            } else {
                canvas.copy(texture, src, dst)
            };
            Ok(result.map_err(PixError::Renderer)?)
        })
    }
}

impl Renderer {
    pub(crate) fn update_canvas<F>(&mut self, mut f: F) -> PixResult<()>
    where
        F: FnMut(&mut WindowCanvas) -> PixResult<()>,
    {
        update_canvas!(self, f)
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (canvas, _) = self
            .canvases
            .get(&self.window_id)
            .expect("valid main window");
        f.debug_struct("SdlRenderer")
            .field("settings", &self.settings)
            .field("blend_mode", &self.blend_mode)
            .field("font_path", &self.font.0)
            .field("font_size", &self.font.1)
            .field("font_style", &self.font_style)
            .field("window_id", &self.window_id)
            .field("window_target", &self.texture_target)
            .field("texture_target", &self.texture_target)
            .field("primary_title", &canvas.window().title())
            .field("primary_dimensions", &canvas.output_size())
            .field("primary_scale", &canvas.scale())
            .field("primary_draw_color", &canvas.draw_color())
            .field("primary_clip", &canvas.clip_rect())
            .field("window_count", &self.canvases.len())
            .field("font_cache_count", &self.font_cache.len())
            .field("text_cache_count", &self.text_cache.len())
            .field("image_cache_count", &self.image_cache.len())
            .finish()
    }
}

/*
 * Type Conversions
 */

impl ToColor for Color {
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        let [r, g, b, a] = self.channels();
        (r, g, b, a)
    }
}

impl From<Color> for SdlColor {
    /// Convert [Color] to [SdlColor].
    fn from(color: Color) -> Self {
        let [r, g, b, a] = color.channels();
        Self::RGBA(r, g, b, a)
    }
}

impl From<FontStyle> for SdlFontStyle {
    /// Convert [FontStyle] to [SdlFontStyle].
    fn from(style: FontStyle) -> Self {
        SdlFontStyle::from_bits(style.bits()).expect("valid FontStyle")
    }
}

impl From<Rect<i32>> for SdlRect {
    /// Convert [`Rect<i32>`] to [SdlRect].
    fn from(rect: Rect<i32>) -> Self {
        Self::new(
            rect.x(),
            rect.y(),
            rect.width() as u32,
            rect.height() as u32,
        )
    }
}

impl From<SdlRect> for Rect<i32> {
    /// Convert [`Rect<i32>`] to [SdlRect].
    fn from(rect: SdlRect) -> Self {
        Self::new(
            rect.x(),
            rect.y(),
            rect.width() as i32,
            rect.height() as i32,
        )
    }
}

impl From<&Rect<i32>> for SdlRect {
    /// Convert &[`Rect<i32>`] to [SdlRect].
    fn from(rect: &Rect<i32>) -> Self {
        Self::new(
            rect.x(),
            rect.y(),
            rect.width() as u32,
            rect.height() as u32,
        )
    }
}

impl From<PointI2> for SdlPoint {
    /// Convert [PointI2] to [SdlPoint].
    fn from(p: PointI2) -> Self {
        Self::new(p.x(), p.y())
    }
}

impl From<&PointI2> for SdlPoint {
    /// Convert &[PointI2] to [SdlPoint].
    fn from(p: &PointI2) -> Self {
        Self::new(p.x(), p.y())
    }
}

impl From<BlendMode> for SdlBlendMode {
    /// Convert [BlendMode] to [SdlBlendMode].
    fn from(mode: BlendMode) -> Self {
        use BlendMode::*;
        match mode {
            None => SdlBlendMode::None,
            Blend => SdlBlendMode::Blend,
            Add => SdlBlendMode::Add,
            Mod => SdlBlendMode::Mod,
        }
    }
}

impl From<PixelFormat> for SdlPixelFormat {
    /// Convert [PixelFormat] to [SdlPixelFormat].
    fn from(format: PixelFormat) -> Self {
        use PixelFormat::*;
        match format {
            Rgb => SdlPixelFormat::RGB24,
            Rgba => SdlPixelFormat::RGBA32,
        }
    }
}
