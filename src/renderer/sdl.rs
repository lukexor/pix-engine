use crate::{
    core::window::Error as WindowError,
    prelude::*,
    renderer::{Error, RendererSettings, Rendering, Result},
};
use lazy_static::lazy_static;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    gfx::primitives::{DrawRenderer, ToColor},
    image::{InitFlag, LoadSurface, Sdl2ImageContext},
    mouse::{Cursor, SystemCursor},
    pixels::{Color as SdlColor, PixelFormatEnum as SdlPixelFormat},
    rect::{Point as SdlPoint, Rect as SdlRect},
    render::{
        BlendMode as SdlBlendMode, Canvas, SdlError, TargetRenderError, TextureCreator,
        TextureQuery, TextureValueError, UpdateTextureError,
    },
    surface::Surface,
    ttf::{Font, FontError, FontStyle as SdlFontStyle, InitError, Sdl2TtfContext},
    video::{Window as SdlWindow, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError, Sdl,
};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};

mod audio;
mod event;
mod textures;
mod window;

pub(crate) use textures::RendererTexture;

type WindowCanvas = Canvas<SdlWindow>;

lazy_static! {
    static ref TTF: Sdl2TtfContext = sdl2::ttf::init().expect("sdl2_ttf initialized");
    static ref IMAGE: Sdl2ImageContext =
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("sdl2_image initialized");
}

/// Allows optionally updating the main window canvas, or targeting a given texture target
macro_rules! update_canvas {
    ($self:expr, $func:expr) => {
        if let Some(ptr) = $self.texture_target {
            // SAFETY: We know this is safe because core::texture::with_texture controls setting and clearing
            // texture_target and has exclusive access to Texture the entire time texture_target is
            // set.
            //
            // One other case that can invalidate this is toggling vsync - which checks for
            // texture_target being set.
            let mut texture = unsafe { &mut (*ptr).inner_mut() };
            Ok($self.canvas.with_texture_canvas(&mut texture, |canvas| {
                let _ = $func(canvas);
            })?)
        } else {
            $func(&mut $self.canvas)
        }
    };
}

/// An SDL [Renderer] implementation.
pub(crate) struct Renderer {
    settings: RendererSettings,
    context: Sdl,
    font: (PathBuf, u16),
    font_cache: HashMap<(PathBuf, u16), Font<'static, 'static>>,
    font_style: SdlFontStyle,
    text_cache: HashMap<(String, Color), RendererTexture>,
    image_cache: HashMap<*const Image, RendererTexture>,
    event_pump: EventPump,
    window_id: WindowId,
    cursor: Cursor,
    canvas: Canvas<SdlWindow>,
    audio_device: AudioQueue<f32>,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<RendererTexture>,
    texture_target: Option<*mut Texture>,
    blend_mode: SdlBlendMode,
}

impl Rendering for Renderer {
    /// Initializes the Sdl2Renderer using the given settings and opens a new window.
    fn new(s: RendererSettings) -> Result<Self> {
        let context = sdl2::init()?;
        let event_pump = context.event_pump()?;

        let (window_id, mut canvas) = Self::create_window_canvas(&context, &s)?;
        if let Some(icon) = &s.icon {
            let surface = Surface::from_file(icon)?;
            canvas.window_mut().set_icon(surface);
        }

        let cursor = Cursor::from_system(SystemCursor::Arrow)?;
        cursor.set();

        let texture_creator = canvas.texture_creator();

        // Set up Audio
        let audio_sub = context.audio()?;
        let desired_spec = AudioSpecDesired {
            freq: Some(s.audio_sample_rate),
            channels: Some(1),
            samples: None,
        };
        let audio_device = audio_sub.open_queue(None, &desired_spec)?;
        audio_device.resume();

        let mut font_cache = HashMap::new();
        let font = (s.font.clone(), s.font_size as u16);
        font_cache.insert(font.clone(), TTF.load_font(&s.font, s.font_size as u16)?);

        Ok(Self {
            settings: s,
            context,
            font,
            font_cache,
            font_style: SdlFontStyle::NORMAL,
            text_cache: HashMap::new(),
            image_cache: HashMap::new(),
            event_pump,
            window_id,
            cursor,
            canvas,
            audio_device,
            texture_creator,
            textures: Vec::new(),
            texture_target: None,
            blend_mode: SdlBlendMode::None,
        })
    }

    /// Clears the canvas to the current clear color.
    #[inline]
    fn clear(&mut self) -> Result<()> {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            canvas.clear();
            Ok(())
        })
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    #[inline]
    fn set_draw_color(&mut self, color: Color) -> Result<()> {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            canvas.set_draw_color(color);
            Ok(())
        })
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    #[inline]
    fn clip(&mut self, rect: Option<Rect<i32>>) -> Result<()> {
        let rect = rect.map(|rect| rect.into());
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
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
        self.canvas.present();
    }

    /// Scale the current canvas.
    #[inline]
    fn scale(&mut self, x: f32, y: f32) -> Result<()> {
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            Ok(canvas.set_scale(x, y)?)
        })
    }

    /// Set the font size for drawing to the current canvas.
    #[inline]
    fn font_size(&mut self, size: u32) -> Result<()> {
        self.font.1 = size as u16;
        self.update_font_cache()?;
        Ok(())
    }

    /// Set the font style for drawing to the current canvas.
    #[inline]
    fn font_style(&mut self, style: FontStyle) {
        if let Some(font) = self.font_cache.get_mut(&self.font) {
            let style = style.into();
            self.font_style = style;
            font.set_style(style);
        }
    }

    /// Set the font family for drawing to the current canvas.
    #[inline]
    fn font_family(&mut self, family: &str) -> Result<()> {
        self.font.0 = PathBuf::from(&family);
        self.update_font_cache()?;
        Ok(())
    }

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        pos: PointI2,
        text: &str,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        fill: Option<Color>,
    ) -> Result<()> {
        let font = self.font_cache.get(&self.font);
        match (fill, font) {
            (Some(fill), Some(font)) => {
                let key = (text.to_string(), fill);
                let texture = match self.text_cache.entry(key) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(v) => {
                        let surface = font.render(text).blended(fill)?;
                        v.insert(self.texture_creator.create_texture_from_surface(&surface)?)
                    }
                };
                let TextureQuery { width, height, .. } = texture.query();
                update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
                    if angle > 0.0 || center.is_some() || flipped.is_some() {
                        Ok(canvas.copy_ex(
                            texture,
                            None,
                            Some(SdlRect::new(pos.x(), pos.y(), width, height)),
                            angle,
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
            (Some(_), None) => Err(Error::InvalidFont(self.font.0.to_owned())),
            (None, _) => Ok(()),
        }
    }

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    #[inline]
    fn size_of(&self, text: &str) -> Result<(u32, u32)> {
        let cache = self.font_cache.get(&self.font);
        match cache {
            Some(font) => Ok(font.size_of(text)?),
            None => Err(Error::InvalidFont(self.font.0.to_owned())),
        }
    }

    /// Draw a pixel to the current canvas.
    #[inline]
    fn point(&mut self, p: PointI2, color: Color) -> Result<()> {
        let p: Point<i16, 2> = p.into();
        let [x, y]: [i16; 2] = p.into();
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            Ok(canvas.pixel(x, y, color)?)
        })
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, line: LineI2, color: Color) -> Result<()> {
        let [start, end]: [Point<i16, 2>; 2] = line.into();
        let [x1, y1] = start.values();
        let [x2, y2] = end.values();
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            if y1 == y2 {
                Ok(canvas.hline(x1, x2, y1, color)?)
            } else if x1 == x2 {
                Ok(canvas.vline(x1, y1, y2, color)?)
            } else {
                Ok(canvas.line(x1, y1, x2, y2, color)?)
            }
        })
    }

    /// Draw a triangle to the current canvas.
    fn triangle(&mut self, tri: TriI2, fill: Option<Color>, stroke: Option<Color>) -> Result<()> {
        let [p1, p2, p3]: [Point<i16, 2>; 3] = tri.into();
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            if let Some(radius) = radius {
                let radius = radius as i16;
                if let Some(fill) = fill {
                    canvas.rounded_box(x, y, x + width - 1, y + height - 1, radius, fill)?
                }
                if let Some(stroke) = stroke {
                    canvas.rounded_rectangle(x, y, x + width, y + height, radius, stroke)?
                }
            } else {
                if let Some(fill) = fill {
                    canvas.box_(x, y, x + width - 1, y + height - 1, fill)?
                }
                if let Some(stroke) = stroke {
                    canvas.rectangle(x, y, x + width, y + height, stroke)?;
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            if let Some(fill) = fill {
                canvas.filled_ellipse(x, y, width, height, fill)?;
            }
            if let Some(stroke) = stroke {
                canvas.ellipse(x, y, width, height, stroke)?;
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
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
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
        rect: Rect<i32>,
        img: &Image,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> Result<()> {
        let texture = match self.image_cache.entry(img) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(self.texture_creator.create_texture_static(
                Some(img.format().into()),
                img.width(),
                img.height(),
            )?),
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
        texture.update(
            None,
            img.as_bytes(),
            img.format().channels() * img.width() as usize,
        )?;
        let rect: SdlRect = rect.into();
        update_canvas!(self, |canvas: &mut WindowCanvas| -> Result<()> {
            if angle > 0.0 || center.is_some() || flipped.is_some() {
                Ok(canvas.copy_ex(
                    texture,
                    None,
                    rect,
                    angle,
                    center.map(|c| c.into()),
                    matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)),
                    matches!(flipped, Some(Flipped::Vertical | Flipped::Both)),
                )?)
            } else {
                Ok(canvas.copy(texture, None, rect)?)
            }
        })
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SdlRenderer")
            .field("title", &self.canvas.window().title())
            .field("window_id", &self.window_id)
            .field("dimensions", &self.canvas.output_size().unwrap_or((0, 0)))
            .field("scale", &self.canvas.scale())
            .field("draw_color", &self.canvas.draw_color())
            .field("blend_mode", &self.blend_mode)
            .field("clip", &self.canvas.clip_rect())
            .field("font_path", &self.font.0)
            .field("font_size", &self.font.1)
            .field("font_style", &self.font_style)
            .field("text_cache_size", &self.text_cache.len())
            .field("image_cache_size", &self.image_cache.len())
            .field("texture_count", &self.textures.len())
            .field("texture_target", &self.texture_target)
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
