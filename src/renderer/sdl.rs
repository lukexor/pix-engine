use crate::{
    prelude::*,
    renderer::{RendererSettings, Rendering},
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
    render::{BlendMode as SdlBlendMode, Canvas, Texture, TextureCreator, TextureQuery},
    rwops::RWops,
    ttf::{Font as SdlFont, FontStyle as SdlFontStyle, Sdl2TtfContext},
    video::{Window as SdlWindow, WindowContext},
    EventPump, Sdl,
};
use std::{cmp, collections::HashMap};

lazy_static! {
    static ref TTF: Sdl2TtfContext = sdl2::ttf::init().expect("sdl2_ttf initialized");
    static ref IMAGE: Sdl2ImageContext =
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("sdl2_image initialized");
}

/// Helper macro for partial borrow of window target `(canvas, texture_creator)`.
macro_rules! get_window_target {
    ($self:expr) => {{
        let target = $self.window_target;
        $self
            .canvases
            .get(&target)
            .ok_or(PixError::InvalidWindow(target))?
    }};
}
/// Helper macro for partial mutable borrow of window target `(canvas, texture_creator)`.
macro_rules! get_window_target_mut {
    ($self:expr) => {{
        let target = $self.window_target;
        $self
            .canvases
            .get_mut(&target)
            .ok_or(PixError::InvalidWindow(target))?
    }};
}

/// Helper macro for partial borrow of window target `canvas`.
macro_rules! get_canvas {
    ($self:expr) => {
        &get_window_target!($self).0
    };
}
/// Helper macro for partial mutable borrow of window target `canvas`.
macro_rules! get_canvas_mut {
    ($self:expr) => {
        &mut get_window_target_mut!($self).0
    };
}
/// Helper macro for partial borrow of window target.
macro_rules! get_window {
    ($self:expr) => {
        &get_window_target!($self).0.window()
    };
}
/// Helper macro for partial mutable borrow of window target.
macro_rules! get_window_mut {
    ($self:expr) => {
        &mut get_window_target_mut!($self).0.window_mut()
    };
}
/// Helper macro for partial borrow of window target `texture_creator`.
macro_rules! get_texture_creator {
    ($self:expr) => {
        &get_window_target!($self).1
    };
}
/// Helper macro to get current loaded font.
macro_rules! get_loaded_font {
    ($self:expr) => {{
        let key = ($self.font_name, $self.font_size);
        $self.loaded_fonts.get_mut(&key).expect("valid font")
    }};
}

/// Helper macro for partial borrow that updates either the window target `canvas` or a
/// `texture_target`.
macro_rules! update_canvas {
    ($self:expr, $func:expr) => {{
        let canvas = get_canvas_mut!($self);
        if let Some(texture_id) = $self.texture_target {
            if let Some((_, texture)) = $self.textures.get_mut(&texture_id) {
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

/// Helper macro for partial borrow that loads the font cache, if not already populated.
macro_rules! load_font {
    ($self:expr) => {{
        let key = ($self.font_name, $self.font_size);
        if !$self.loaded_fonts.contains(&key) {
            let font_data = $self
                .font_data
                .get(&$self.font_name)
                .expect("valid loaded font");
            let loaded_font = match font_data.source {
                FontSrc::Bytes(bytes) => {
                    let rwops = RWops::from_bytes(bytes).map_err(PixError::Renderer)?;
                    TTF.load_font_from_rwops(rwops, $self.font_size)
                        .map_err(PixError::Renderer)?
                }
                FontSrc::Path(ref path) => TTF
                    .load_font(path, $self.font_size)
                    .map_err(PixError::Renderer)?,
            };
            $self.loaded_fonts.put(key, loaded_font);
        }
    }};
}

mod audio;
mod event;
mod textures;
mod window;

pub(crate) use textures::RendererTexture;
pub(crate) type WindowCanvas = Canvas<SdlWindow>;

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
    font_name: &'static str,
    font_size: u16,
    font_style: SdlFontStyle,
    window_id: WindowId,
    window_target: WindowId,
    texture_target: Option<TextureId>,
    canvases: HashMap<WindowId, (WindowCanvas, TextureCreator<WindowContext>)>,
    textures: HashMap<TextureId, (WindowId, RendererTexture)>,
    next_texture_id: usize,
    font_data: LruCache<&'static str, Font>,
    loaded_fonts: LruCache<(&'static str, u16), SdlFont<'static, 'static>>,
    text_cache: LruCache<(WindowId, Color), HashMap<String, RendererTexture>>,
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

        let texture_cache_size = s.texture_cache_size;
        let text_cache_size = s.text_cache_size;
        let default_font = Font::default();
        let font_name = default_font.name;
        let mut font_data = LruCache::new(text_cache_size);
        font_data.put(default_font.name, default_font);

        let mut renderer = Self {
            context,
            event_pump,
            audio_device,
            settings: s,
            title,
            fps: 0,
            cursor,
            blend_mode: SdlBlendMode::None,
            font_name,
            font_size: 14,
            font_style: SdlFontStyle::NORMAL,
            window_id,
            window_target: window_id,
            texture_target: None,
            canvases,
            textures: HashMap::new(),
            next_texture_id: 0,
            font_data,
            loaded_fonts: LruCache::new(texture_cache_size),
            text_cache: LruCache::new(text_cache_size),
            image_cache: LruCache::new(texture_cache_size),
        };
        load_font!(renderer);

        Ok(renderer)
    }

    /// Clears the canvas to the current clear color.
    #[inline]
    fn clear(&mut self) -> PixResult<()> {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            canvas.clear();
            Ok(())
        })
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    #[inline]
    fn set_draw_color(&mut self, color: Color) -> PixResult<()> {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            canvas.set_draw_color(color);
            Ok(())
        })
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    #[inline]
    fn clip(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            canvas.set_clip_rect(rect.map(|rect| rect.into()));
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            Ok(canvas.set_scale(x, y).map_err(PixError::Renderer)?)
        })
    }

    /// Set the font size for drawing to the current canvas.
    #[inline]
    fn font_size(&mut self, size: u32) -> PixResult<()> {
        self.font_size = size as u16;
        load_font!(self);
        Ok(())
    }

    /// Set the font style for drawing to the current canvas.
    #[inline]
    fn font_style(&mut self, style: FontStyle) {
        let style = style.into();
        if self.font_style != style {
            self.font_style = style;
            get_loaded_font!(self).set_style(style);
        }
    }

    /// Set the font family for drawing to the current canvas.
    #[inline]
    fn font_family(&mut self, font: &Font) -> PixResult<()> {
        self.font_name = font.name;
        if !self.font_data.contains(&self.font_name) {
            self.font_data.put(self.font_name, font.clone());
        }
        load_font!(self);
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

        let font = get_loaded_font!(self);
        let texture_creator = get_texture_creator!(self);

        if let Some(fill) = fill {
            let current_outline = font.get_outline_width();
            if current_outline != outline as u16 {
                font.set_outline_width(outline as u16);
            }

            let create_texture = |font: &SdlFont<'_, '_>, fill: Color| -> PixResult<Texture> {
                let surface = if let Some(width) = wrap_width {
                    font.render(text).blended_wrapped(fill, width)
                } else {
                    font.render(text).blended(fill)
                }
                .context("invalid text")?;
                texture_creator
                    .create_texture_from_surface(&surface)
                    .context("failed to create text surface")
            };

            let key = (self.window_target, fill);
            match self.text_cache.get_mut(&key) {
                Some(cache) if !cache.contains_key(text) => {
                    cache.insert(text.to_string(), create_texture(font, fill)?);
                }
                None => {
                    self.text_cache.put(
                        key,
                        HashMap::from([(text.to_string(), create_texture(font, fill)?)]),
                    );
                }
                _ => (),
            }

            let texture = self
                .text_cache
                .get_mut(&key)
                .context("valid text cache")?
                .get_mut(text)
                .context("valid text cache")?;
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
        let font = get_loaded_font!(self);
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let [x, y] = p.map(|v| v as i16);
            Ok(canvas.pixel(x, y, color).map_err(PixError::Renderer)?)
        })
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, line: LineI2, stroke: u8, color: Color) -> PixResult<()> {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let [x1, y1] = line.start().map(|v| v as i16);
            let [x2, y2] = line.end().map(|v| v as i16);
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let [x1, y1] = tri.p1().map(|v| v as i16);
            let [x2, y2] = tri.p2().map(|v| v as i16);
            let [x3, y3] = tri.p3().map(|v| v as i16);
            if let Some(fill) = fill {
                canvas
                    .filled_trigon(x1, y1, x2, y2, x3, y3, fill)
                    .map_err(PixError::Renderer)?;
            }
            if let Some(stroke) = stroke {
                canvas
                    .trigon(x1, y1, x2, y2, x3, y3, stroke)
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let [x, y, width, height] = rect.map(|v| v as i16);
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let [x1, y1] = quad.p1().map(|v| v as i16);
            let [x2, y2] = quad.p2().map(|v| v as i16);
            let [x3, y3] = quad.p3().map(|v| v as i16);
            let [x4, y4] = quad.p4().map(|v| v as i16);
            let vx = [x1, x2, x3, x4];
            let vy = [y1, y2, y3, y4];
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
    fn polygon<I>(&mut self, ps: I, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()>
    where
        I: Iterator<Item = PointI2>,
    {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let (vx, vy): (Vec<i16>, Vec<i16>) = ps
                .map(|p| -> (i16, i16) {
                    let [x, y] = p.map(|v| v as i16);
                    (x, y)
                })
                .unzip();
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let [x, y, width, height] = ellipse.map(|v| v as i16);
            let rw = width / 2;
            let rh = height / 2;
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> PixResult<()> {
            let [x, y] = p.map(|v| v as i16);
            let radius = radius as i16;
            let start = start as i16;
            let end = end as i16;
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
        let img_ptr: *const Image = img;
        let key = (self.window_target, img_ptr);
        let texture = {
            if !self.image_cache.contains(&key) {
                let texture_creator = get_texture_creator!(self);
                self.image_cache.put(
                    key,
                    texture_creator
                        .create_texture_static(Some(img.format().into()), img.width(), img.height())
                        .context("failed to create image texture")?,
                );
            }
            // SAFETY: We just checked or inserted a texture.
            self.image_cache.get_mut(&key).expect("valid image cache")
        };
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

    /// Return the current rendered target pixels as an array of bytes.
    fn to_bytes(&mut self) -> PixResult<Vec<u8>> {
        let canvas = get_canvas_mut!(self);
        if let Some(texture_id) = self.texture_target {
            if let Some((_, texture)) = self.textures.get_mut(&texture_id) {
                let mut result = Ok(vec![]);
                canvas
                    .with_texture_canvas(texture, |canvas| {
                        result = canvas.read_pixels(None, SdlPixelFormat::RGBA32)
                    })
                    .with_context(|| format!("failed to read texture target {}", texture_id))?;
                Ok(result.map_err(PixError::Renderer)?)
            } else {
                Err(PixError::InvalidTexture(texture_id).into())
            }
        } else {
            Ok(canvas
                .read_pixels(None, SdlPixelFormat::RGBA32)
                .map_err(PixError::Renderer)?)
        }
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
            .field("font_name", &self.font_name)
            .field("font_size", &self.font_size)
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
            .field("loaded_fonts_count", &self.loaded_fonts.len())
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
