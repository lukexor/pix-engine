use crate::{
    core::window::Error as WindowError,
    prelude::*,
    renderer::{Error, RendererSettings, Rendering, Result},
    ASSETS,
};
use lazy_static::lazy_static;
use lru::LruCache;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    gfx::primitives::{DrawRenderer, ToColor},
    image::{InitFlag, Sdl2ImageContext},
    mouse::{Cursor, SystemCursor},
    pixels::{Color as SdlColor, PixelFormatEnum as SdlPixelFormat},
    rect::{Point as SdlPoint, Rect as SdlRect},
    render::{
        BlendMode as SdlBlendMode, Canvas, SdlError, TargetRenderError, TextureCreator,
        TextureQuery, TextureValueError, UpdateTextureError,
    },
    rwops::RWops,
    ttf::{Font as SdlFont, FontError, FontStyle as SdlFontStyle, InitError, Sdl2TtfContext},
    video::{Window as SdlWindow, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError, Sdl,
};
use std::{borrow::Cow, cmp, collections::HashMap};

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
            .ok_or(WindowError::InvalidWindow($self.window_target))?;
        if let Some(texture_id) = $self.texture_target {
            if let Some((_, texture)) = $self.textures.get_mut(texture_id) {
                Ok(canvas.with_texture_canvas(texture, |canvas| {
                    let _ = $func(canvas);
                })?)
            } else {
                Err(Error::InvalidTexture(texture_id))
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
            match $font.0.source {
                FontSrc::Library(ref name) => {
                    let contents = ASSETS
                        .get_file(name)
                        .expect("valid included font")
                        .contents();
                    let rwops = RWops::from_bytes(contents)?;
                    $cache.put(key, TTF.load_font_from_rwops(rwops, size)?);
                }
                FontSrc::Bytes(bytes) => {
                    let rwops = RWops::from_bytes(bytes)?;
                    $cache.put(key, TTF.load_font_from_rwops(rwops, size)?);
                }
                FontSrc::Custom(ref path) => {
                    $cache.put(key, TTF.load_font(path, size)?);
                }
            }
        }
    }};
}

/// An SDL [Renderer] implementation.
pub(crate) struct Renderer {
    context: Sdl,
    event_pump: EventPump,
    audio_device: AudioQueue<f32>,
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
    fn new(s: RendererSettings) -> Result<Self> {
        let context = sdl2::init()?;
        let event_pump = context.event_pump()?;

        let (window_id, canvas) = Self::create_window_canvas(&context, &s)?;
        let cursor = Cursor::from_system(SystemCursor::Arrow)?;
        cursor.set();
        let texture_creator = canvas.texture_creator();
        let mut canvases = HashMap::new();
        canvases.insert(window_id, (canvas, texture_creator));

        // Set up Audio
        let audio_sub = context.audio()?;
        let desired_spec = AudioSpecDesired {
            freq: Some(s.audio_sample_rate),
            channels: Some(1),
            samples: None,
        };
        let audio_device = audio_sub.open_queue(None, &desired_spec)?;
        audio_device.resume();

        let font_size = s.theme.font_sizes.body as u16;
        let font = (s.theme.fonts.body.clone(), font_size);
        let mut font_cache = LruCache::new(s.texture_cache_size);
        update_font_cache!(font, font_cache);
        let text_cache = LruCache::new(s.text_cache_size);
        let image_cache = LruCache::new(s.texture_cache_size);

        Ok(Self {
            context,
            event_pump,
            audio_device,
            settings: s,
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
    fn clear(&mut self) -> Result<()> {
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            canvas.clear();
            Ok(())
        })
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    #[inline]
    fn set_draw_color(&mut self, color: Color) -> Result<()> {
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            canvas.set_draw_color(color);
            Ok(())
        })
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    #[inline]
    fn clip(&mut self, rect: Option<Rect<i32>>) -> Result<()> {
        let rect = rect.map(|rect| rect.into());
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
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
    fn scale(&mut self, x: f32, y: f32) -> Result<()> {
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            Ok(canvas.set_scale(x, y)?)
        })
    }

    /// Set the font size for drawing to the current canvas.
    #[inline]
    fn font_size(&mut self, size: u32) -> Result<()> {
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
    fn font_family(&mut self, font: &Font) -> Result<()> {
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
    ) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }
        let key = (self.font.0.name, self.font.1);
        let font = self.font_cache.get_mut(&key);
        match (fill, font) {
            (Some(fill), Some(font)) => {
                let current_outline = font.get_outline_width();
                if current_outline != outline as u16 {
                    font.set_outline_width(outline as u16);
                }

                let key = (self.window_target, text.to_string(), fill);
                let (_, texture_creator) = self
                    .canvases
                    .get(&self.window_target)
                    .ok_or(WindowError::InvalidWindow(self.window_target))?;
                if !self.text_cache.contains(&key) {
                    let surface = if let Some(width) = wrap_width {
                        font.render(text).blended_wrapped(fill, width)?
                    } else {
                        font.render(text).blended(fill)?
                    };
                    self.text_cache.put(
                        key.clone(),
                        texture_creator.create_texture_from_surface(&surface)?,
                    );
                }
                // SAFETY: We just checked or inserted a texture.
                let texture = self.text_cache.get_mut(&key).expect("valid text cache");
                let TextureQuery { width, height, .. } = texture.query();
                update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
                    if angle.is_some() || center.is_some() || flipped.is_some() {
                        Ok(canvas.copy_ex(
                            texture,
                            None,
                            Some(SdlRect::new(pos.x(), pos.y(), width, height)),
                            angle.unwrap_or(0.0),
                            center.map(|c| c.into()),
                            matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                            matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                        )?)
                    } else {
                        Ok(canvas.copy(
                            texture,
                            None,
                            Some(SdlRect::new(pos.x(), pos.y(), width, height)),
                        )?)
                    }
                })
            }
            (Some(_), None) => Err(Error::InvalidFont(self.font.0.name)),
            (None, _) => Ok(()),
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
    fn set_clipboard_text(&self, value: &str) -> Result<()> {
        Ok(self
            .context
            .video()?
            .clipboard()
            .set_clipboard_text(value)?)
    }

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    #[inline]
    fn size_of(&mut self, text: &str) -> Result<(u32, u32)> {
        let key = (self.font.0.name, self.font.1);
        match self.font_cache.get(&key) {
            Some(font) => {
                if text.contains('\n') {
                    let mut size = (0, 0);
                    for line in text.split('\n') {
                        let (w, h) = font.size_of(line)?;
                        size.0 = cmp::max(size.0, w);
                        size.1 += h;
                    }
                    Ok(size)
                } else {
                    Ok(font.size_of(text)?)
                }
            }
            None => Err(Error::InvalidFont(self.font.0.name)),
        }
    }

    /// Draw a pixel to the current canvas.
    #[inline]
    fn point(&mut self, p: PointI2, color: Color) -> Result<()> {
        let p: Point<i16, 2> = p.into();
        let [x, y]: [i16; 2] = p.into();
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            Ok(canvas.pixel(x, y, color)?)
        })
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, line: LineI2, stroke: u8, color: Color) -> Result<()> {
        let [start, end]: [Point<i16, 2>; 2] = line.into();
        let [x1, y1] = start.values();
        let [x2, y2] = end.values();
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            if stroke == 1 {
                if y1 == y2 {
                    Ok(canvas.hline(x1, x2, y1, color)?)
                } else if x1 == x2 {
                    Ok(canvas.vline(x1, y1, y2, color)?)
                } else {
                    Ok(canvas.aa_line(x1, y1, x2, y2, color)?)
                }
            } else {
                Ok(canvas.thick_line(x1, y1, x2, y2, stroke, color)?)
            }
        })
    }

    /// Draw a triangle to the current canvas.
    fn triangle(&mut self, tri: TriI2, fill: Option<Color>, stroke: Option<Color>) -> Result<()> {
        let [p1, p2, p3]: [Point<i16, 2>; 3] = tri.into();
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            if let Some(fill) = fill {
                canvas.filled_trigon(p1.x(), p1.y(), p2.x(), p2.y(), p3.x(), p3.y(), fill)?;
            }
            if let Some(stroke) = stroke {
                canvas.trigon(p1.x(), p1.y(), p2.x(), p2.y(), p3.x(), p3.y(), stroke)?;
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
    ) -> Result<()> {
        let rect: Rect<i16> = rect.into();
        let [x, y, width, height] = rect.values();
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            if let Some(radius) = radius {
                let radius = radius as i16;
                if let Some(fill) = fill {
                    canvas.rounded_box(x, y, x + width, y + height, radius, fill)?
                }
                if let Some(stroke) = stroke {
                    canvas.rounded_rectangle(x, y, x + width, y + height, radius, stroke)?
                }
            } else {
                if let Some(fill) = fill {
                    canvas.box_(x, y, x + width, y + height, fill)?
                }
                if let Some(stroke) = stroke {
                    // EXPL: SDL2_gfx renders this 1px smaller than it should.
                    canvas.rectangle(x, y, x + width + 1, y + height + 1, stroke)?;
                }
            }
            Ok(())
        })
    }

    /// Draw a quadrilateral to the current canvas.
    fn quad(&mut self, quad: QuadI2, fill: Option<Color>, stroke: Option<Color>) -> Result<()> {
        let [p1, p2, p3, p4]: [Point<i16, 2>; 4] = quad.into();
        let vx = [p1.x(), p2.x(), p3.x(), p4.x()];
        let vy = [p1.y(), p2.y(), p3.y(), p4.y()];
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            if let Some(fill) = fill {
                canvas.filled_polygon(&vx, &vy, fill)?;
            }
            if let Some(stroke) = stroke {
                canvas.polygon(&vx, &vy, stroke)?;
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
    ) -> Result<()> {
        let (vx, vy): (Vec<i16>, Vec<i16>) = ps
            .iter()
            .map(|&p| -> (i16, i16) {
                let p: Point<i16, 2> = p.into();
                let [x, y] = p.values();
                (x, y)
            })
            .unzip();
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            if let Some(fill) = fill {
                canvas.filled_polygon(&vx, &vy, fill)?;
            }
            if let Some(stroke) = stroke {
                canvas.polygon(&vx, &vy, stroke)?;
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
    ) -> Result<()> {
        let ellipse: Ellipse<i16> = ellipse.into();
        let [x, y, width, height] = ellipse.values();
        let rw = width / 2;
        let rh = height / 2;
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            if let Some(fill) = fill {
                canvas.filled_ellipse(x, y, rw, rh, fill)?;
            }
            if let Some(stroke) = stroke {
                canvas.aa_ellipse(x, y, rw, rh, stroke)?;
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
    ) -> Result<()> {
        let p: Point<i16, 2> = p.into();
        let [x, y] = p.values();
        let radius = radius as i16;
        let start = start as i16;
        let end = end as i16;
        self.update_canvas(|canvas: &mut WindowCanvas| -> Result<()> {
            match mode {
                ArcMode::Default => {
                    if let Some(stroke) = stroke {
                        canvas.arc(x, y, radius, start, end, stroke)?;
                    }
                }
                ArcMode::Pie => {
                    if let Some(fill) = fill {
                        canvas.filled_pie(x, y, radius, start, end, fill)?;
                    }
                    if let Some(stroke) = stroke {
                        canvas.pie(x, y, radius, start, end, stroke)?;
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
    ) -> Result<()> {
        let (_, texture_creator) = self
            .canvases
            .get(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        let img_ptr: *const Image = img;
        let key = (self.window_target, img_ptr);
        if !self.image_cache.contains(&key) {
            self.image_cache.put(
                key,
                texture_creator.create_texture_static(
                    Some(img.format().into()),
                    img.width(),
                    img.height(),
                )?,
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
        texture.update(
            None,
            img.as_bytes(),
            img.format().channels() * img.width() as usize,
        )?;
        let src = src.map(|r| r.into());
        let dst = dst.map(|r| r.into());
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            if angle > 0.0 || center.is_some() || flipped.is_some() {
                Ok(canvas.copy_ex(
                    texture,
                    src,
                    dst,
                    angle,
                    center.map(|c| c.into()),
                    matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                    matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                )?)
            } else {
                Ok(canvas.copy(texture, src, dst)?)
            }
        })
    }
}

impl Renderer {
    pub(crate) fn update_canvas<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut WindowCanvas) -> Result<()>,
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
        match style {
            FontStyle::NORMAL => SdlFontStyle::NORMAL,
            FontStyle::BOLD => SdlFontStyle::BOLD,
            FontStyle::ITALIC => SdlFontStyle::ITALIC,
            FontStyle::UNDERLINE => SdlFontStyle::UNDERLINE,
            FontStyle::STRIKETHROUGH => SdlFontStyle::STRIKETHROUGH,
            _ => unreachable!("invalid FontStyle"),
        }
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

/*
 * Error Conversions
 */

impl From<InitError> for Error {
    /// Convert [InitError] to [Error].
    fn from(err: InitError) -> Self {
        use InitError::*;
        match err {
            InitializationError(err) => Self::IoError(err),
            AlreadyInitializedError => Self::InitError,
        }
    }
}

impl From<FontError> for Error {
    /// Convert [FontError] to [Error].
    fn from(err: FontError) -> Self {
        use FontError::*;
        match err {
            InvalidLatin1Text(e) => Self::InvalidText("invalid latin1 text", e),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<WindowError> for Error {
    /// Convert [WindowError] to [Error].
    fn from(err: WindowError) -> Self {
        Error::WindowError(err)
    }
}

impl From<WindowBuildError> for Error {
    /// Convert [WindowBuildError] to [Error].
    fn from(err: WindowBuildError) -> Self {
        use WindowBuildError::*;
        match err {
            HeightOverflows(h) => Self::Overflow(Cow::from("window height"), h),
            WidthOverflows(w) => Self::Overflow(Cow::from("window width"), w),
            InvalidTitle(e) => Self::InvalidText("invalid title", e),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<IntegerOrSdlError> for Error {
    /// Convert [IntegerOrSdlError] to [Error].
    fn from(err: IntegerOrSdlError) -> Self {
        use IntegerOrSdlError::*;
        match err {
            IntegerOverflows(s, v) => Self::Overflow(Cow::from(s), v),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<TextureValueError> for Error {
    /// Convert [TextureValueError] to [Error].
    fn from(err: TextureValueError) -> Self {
        use TextureValueError::*;
        match err {
            HeightOverflows(h) => Self::Overflow(Cow::from("texture height"), h),
            WidthOverflows(w) => Self::Overflow(Cow::from("texture width"), w),
            WidthMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("width must be multiple of 2"))
            }
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<TargetRenderError> for Error {
    /// Convert [TargetRenderError] to [Error].
    fn from(err: TargetRenderError) -> Self {
        use TargetRenderError::*;
        match err {
            NotSupported => Self::Other(Cow::from("Not supported")),
            SdlError(s) => s.into(),
        }
    }
}

impl From<SdlError> for Error {
    /// Convert [SdlError] to [Error].
    fn from(err: SdlError) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}

impl From<UpdateTextureError> for Error {
    /// Convert [UpdateTextureError] to [Error].
    fn from(err: UpdateTextureError) -> Self {
        use UpdateTextureError::*;
        match err {
            PitchOverflows(p) => Self::Overflow(Cow::from("pitch"), p as u32),
            PitchMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("pitch must be multiple of 2"))
            }
            XMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("x must be multiple of 2"))
            }
            YMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("y must be multiple of 2"))
            }
            WidthMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("width must be multiple of 2"))
            }
            HeightMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("height must be multiple of 2"))
            }
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}
