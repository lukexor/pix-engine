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
    rect::Rect as SdlRect,
    render::{
        BlendMode as SdlBlendMode, Canvas, SdlError, TargetRenderError, Texture as SdlTexture,
        TextureCreator, TextureQuery, TextureValueError, UpdateTextureError,
    },
    surface::Surface,
    ttf::{Font, FontError, FontStyle as SdlFontStyle, InitError, Sdl2TtfContext},
    video::{Window as SdlWindow, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError, Sdl,
};
use std::{borrow::Cow, collections::HashMap, convert::TryInto, path::PathBuf};

mod audio;
mod event;
mod window;

lazy_static! {
    static ref TTF: Sdl2TtfContext = sdl2::ttf::init().expect("sdl2_ttf initialized");
    static ref IMAGE: Sdl2ImageContext =
        sdl2::image::init(InitFlag::PNG | InitFlag::JPG).expect("sdl2_image initialized");
}

/// An SDL [Renderer] implementation.
pub(crate) struct Renderer {
    pub(crate) settings: RendererSettings,
    context: Sdl,
    font: (PathBuf, u16),
    font_cache: HashMap<(PathBuf, u16), Font<'static, 'static>>,
    font_style: SdlFontStyle,
    event_pump: EventPump,
    window_id: WindowId,
    cursor: Cursor,
    canvas: Canvas<SdlWindow>,
    audio_device: AudioQueue<f32>,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<SdlTexture>,
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
            event_pump,
            window_id,
            cursor,
            canvas,
            audio_device,
            texture_creator,
            textures: Vec::new(),
            blend_mode: SdlBlendMode::None,
        })
    }

    /// Clears the canvas to the current clear color.
    fn clear(&mut self) {
        self.canvas.clear();
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect<i32>>) {
        let rect = rect.map(|rect| rect.into());
        self.canvas.set_clip_rect(rect);
    }

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode.into();
    }

    /// Updates the canvas from the current back buffer.
    fn present(&mut self) {
        self.canvas.present();
    }

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> Result<()> {
        Ok(self.canvas.set_scale(x, y)?)
    }

    /// Create a texture to render to.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> Result<TextureId> {
        let texture_id = self.textures.len();
        self.textures
            .push(self.texture_creator.create_texture_target(
                format.map(|f| f.into()),
                width,
                height,
            )?);
        Ok(texture_id)
    }

    /// Delete a texture.
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
        if texture_id < self.textures.len() {
            let texture = self.textures.remove(texture_id);
            // SAFETY: self.texture_creator can not be destroyed while PixEngine is running
            unsafe { texture.destroy() };
            Ok(())
        } else {
            Err(Error::InvalidTexture(texture_id))
        }
    }

    /// Update texture with pixel data.
    fn update_texture(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: &[u8],
        pitch: usize,
    ) -> Result<()> {
        if let Some(texture) = self.textures.get_mut(texture_id) {
            let rect: Option<SdlRect> = rect.map(|r| r.into());
            Ok(texture.update(rect, pixels, pitch)?)
        } else {
            Err(Error::InvalidTexture(texture_id))
        }
    }

    /// Draw texture canvas.
    fn texture(
        &mut self,
        texture_id: usize,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
    ) -> Result<()> {
        if let Some(texture) = self.textures.get(texture_id) {
            let src = src.map(|r| r.into());
            let dst = dst.map(|r| r.into());
            Ok(self.canvas.copy(texture, src, dst)?)
        } else {
            Err(Error::InvalidTexture(texture_id))
        }
    }

    /// Set the font size for drawing to the current canvas.
    fn font_size(&mut self, size: u32) -> Result<()> {
        self.font.1 = size.try_into()?;
        if self.font_cache.get(&self.font).is_none() {
            self.font_cache
                .insert(self.font.clone(), TTF.load_font(&self.font.0, self.font.1)?);
        }
        Ok(())
    }

    /// Set the font style for drawing to the current canvas.
    fn font_style(&mut self, style: FontStyle) {
        if let Some(font) = self.font_cache.get_mut(&self.font) {
            let style = style.into();
            self.font_style = style;
            font.set_style(style);
        }
    }

    /// Set the font family for drawing to the current canvas.
    fn font_family(&mut self, family: &str) -> Result<()> {
        self.font.0 = PathBuf::from(&family);
        if self.font_cache.get(&self.font).is_none() {
            self.font_cache
                .insert(self.font.clone(), TTF.load_font(&self.font.0, self.font.1)?);
        }
        Ok(())
    }

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        pos: &Point<i32>,
        text: &str,
        fill: Option<Color>,
        _stroke: Option<Color>,
    ) -> Result<()> {
        let font = self.font_cache.get(&self.font);
        match (fill, font) {
            (Some(fill), Some(font)) => {
                let surface = font.render(text).blended(fill)?;
                let texture = self.texture_creator.create_texture_from_surface(&surface)?;
                let TextureQuery { width, height, .. } = texture.query();
                Ok(self.canvas.copy(
                    &texture,
                    None,
                    Some(SdlRect::new(pos.x(), pos.y(), width, height)),
                )?)
            }
            (Some(_), None) => Err(Error::InvalidFont(self.font.0.to_owned())),
            (None, _) => Ok(()),
        }
    }

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    fn size_of(&self, text: &str) -> Result<(u32, u32)> {
        let font = self.font_cache.get(&self.font);
        match font {
            Some(font) => {
                let (w, h) = font.size_of(text)?;
                Ok((w, h))
            }
            None => Err(Error::InvalidFont(self.font.0.to_owned())),
        }
    }

    /// Draw a pixel to the current canvas.
    fn point(&mut self, p: &Point<i32>, color: Color) -> Result<()> {
        let [x, y, _] = p.try_into_values()?;
        Ok(self.canvas.pixel(x, y, color)?)
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, line: &Line<i32>, color: Color) -> Result<()> {
        let [x1, y1, _, x2, y2, _] = line.try_into_values()?;
        if y1 == y2 {
            self.canvas.hline(x1, x2, y1, color)?;
        } else if x1 == x2 {
            self.canvas.vline(x1, y1, y2, color)?;
        } else {
            self.canvas.line(x1, y1, x2, y2, color)?;
        }
        Ok(())
    }

    /// Draw a triangle to the current canvas.
    fn triangle(
        &mut self,
        tri: &Triangle<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [p1, p2, p3] = tri.try_into_values()?;
        if let Some(fill) = fill {
            self.canvas
                .filled_trigon(p1.x(), p1.y(), p2.x(), p2.y(), p3.x(), p3.y(), fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas
                .trigon(p1.x(), p1.y(), p2.x(), p2.y(), p3.x(), p3.y(), stroke)?;
        }
        Ok(())
    }

    /// Draw a rectangle to the current canvas.
    fn rect(&mut self, rect: &Rect<i32>, fill: Option<Color>, stroke: Option<Color>) -> Result<()> {
        let [x, y, width, height] = rect.try_into_values()?;
        if let Some(fill) = fill {
            self.canvas.box_(x, y, x + width, y + height, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas.rectangle(x, y, x + width, y + height, stroke)?;
        }
        Ok(())
    }

    /// Draw a rounded rectangle to the current canvas.
    fn rounded_rect(
        &mut self,
        rect: &Rect<i32>,
        radius: i32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [x, y, width, height] = rect.try_into_values()?;
        let radius = radius.try_into()?;
        if let Some(fill) = fill {
            self.canvas
                .rounded_box(x, y, x + width, y + height, radius, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas
                .rounded_rectangle(x, y, x + width, y + height, radius, stroke)?;
        }
        Ok(())
    }

    /// Draw a quadrilateral to the current canvas.
    fn quad(&mut self, quad: &Quad<i32>, fill: Option<Color>, stroke: Option<Color>) -> Result<()> {
        let [p1, p2, p3, p4] = quad.try_into_values()?;
        let vx = [p1.x(), p2.x(), p3.x(), p4.x()];
        let vy = [p1.y(), p2.y(), p3.y(), p4.y()];
        if let Some(fill) = fill {
            self.canvas.filled_polygon(&vx, &vy, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas.polygon(&vx, &vy, stroke)?;
        }
        Ok(())
    }

    /// Draw a polygon to the current canvas.
    fn polygon<P>(&mut self, ps: P, fill: Option<Color>, stroke: Option<Color>) -> Result<()>
    where
        P: IntoIterator<Item = Point<i32>>,
    {
        let (vx, vy): (Vec<i16>, Vec<i16>) = Self::try_convert_points(ps)?.into_iter().unzip();
        if let Some(fill) = fill {
            self.canvas.filled_polygon(&vx, &vy, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas.polygon(&vx, &vy, stroke)?;
        }
        Ok(())
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: &Ellipse<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [x, y, width, height] = ellipse.try_into_values()?;
        if let Some(fill) = fill {
            self.canvas.filled_ellipse(x, y, width, height, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas.ellipse(x, y, width, height, stroke)?;
        }
        Ok(())
    }

    /// Draw an arc to the current canvas.
    fn arc(
        &mut self,
        p: &Point<i32>,
        radius: i32,
        start: i32,
        end: i32,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        use ArcMode::*;
        let [x, y, _] = p.try_into_values()?;
        let radius = radius.try_into()?;
        let start = start.try_into()?;
        let end = end.try_into()?;
        match mode {
            Default => {
                if let Some(stroke) = stroke {
                    self.canvas.arc(x, y, radius, start, end, stroke)?;
                }
            }
            Pie => {
                if let Some(stroke) = stroke {
                    self.canvas.pie(x, y, radius, start, end, stroke)?;
                }
                if let Some(fill) = fill {
                    self.canvas.filled_pie(x, y, radius, start, end, fill)?;
                }
            }
        }
        Ok(())
    }

    /// Draw an image to the current canvas.
    fn image(&mut self, pos: &Point<i32>, img: &Image, tint: Option<Color>) -> Result<()> {
        let dst = SdlRect::new(pos.x(), pos.y(), img.width(), img.height());
        self.image_texture(img, tint, dst, None)
    }

    /// Draw an image to the current canvas.
    fn image_resized(&mut self, img: &Image, dst: &Rect<i32>, tint: Option<Color>) -> Result<()> {
        self.image_texture(img, tint, dst.into(), None)
    }

    /// Draw a rotated image to the current canvas.
    fn image_rotated(
        &mut self,
        pos: &Point<i32>,
        img: &Image,
        angle: Scalar,
        tint: Option<Color>,
    ) -> Result<()> {
        let dst = SdlRect::new(pos.x(), pos.y(), img.width(), img.height());
        self.image_texture(img, tint, dst, Some(angle))
    }
}

impl Renderer {
    fn try_convert_points<P>(ps: P) -> Result<Vec<(i16, i16)>>
    where
        P: IntoIterator<Item = Point<i32>>,
    {
        ps.into_iter()
            .map(|p| -> Result<(i16, i16)> {
                let [x, y, _]: [i16; 3] = p.try_into_values()?;
                Ok((x, y))
            })
            .collect::<Result<Vec<(i16, i16)>>>()
    }

    fn get_texture_cache(&mut self, img: &Image) -> Result<(TextureId, bool)> {
        match img.texture_cache() {
            Some(texture_cache) => Ok(texture_cache),
            None => {
                let texture_id =
                    self.create_texture(img.width(), img.height(), img.format().into())?;
                img.set_texture_id(texture_id);
                Ok((texture_id, false))
            }
        }
    }

    fn image_texture(
        &mut self,
        img: &Image,
        tint: Option<Color>,
        dst: SdlRect,
        angle: Option<Scalar>,
    ) -> Result<()> {
        let (texture_id, updated) = self.get_texture_cache(img)?;
        match self.textures.get_mut(texture_id) {
            Some(texture) => {
                if !updated {
                    texture.update(
                        None,
                        img.bytes(),
                        img.format().channels() * img.width() as usize,
                    )?;
                    img.set_updated(true);
                }
                if let Some(tint) = tint {
                    let [r, g, b, a] = tint.channels();
                    texture.set_color_mod(r, g, b);
                    texture.set_alpha_mod(a);
                }
                texture.set_blend_mode(self.blend_mode);
                match angle {
                    Some(angle) => Ok(self
                        .canvas
                        .copy_ex(&texture, None, dst, angle, None, false, false)?),
                    None => Ok(self.canvas.copy(&texture, None, dst)?),
                }
            }
            None => Err(Error::InvalidTexture(texture_id)),
        }
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
            .field("texture_count", &self.textures.len())
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
    fn from(color: Color) -> Self {
        let [r, g, b, a] = color.channels();
        Self::RGBA(r, g, b, a)
    }
}

impl From<FontStyle> for SdlFontStyle {
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
    fn from(rect: &Rect<i32>) -> Self {
        Self::new(
            rect.x(),
            rect.y(),
            rect.width() as u32,
            rect.height() as u32,
        )
    }
}

impl From<BlendMode> for SdlBlendMode {
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
    fn from(err: InitError) -> Self {
        use InitError::*;
        match err {
            InitializationError(err) => Self::IoError(err),
            AlreadyInitializedError => Self::InitError,
        }
    }
}

impl From<FontError> for Error {
    fn from(err: FontError) -> Self {
        use FontError::*;
        match err {
            InvalidLatin1Text(e) => Self::InvalidText("invalid latin1 text", e),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<WindowError> for Error {
    fn from(err: WindowError) -> Self {
        Error::WindowError(err)
    }
}

impl From<WindowBuildError> for Error {
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
    fn from(err: IntegerOrSdlError) -> Self {
        use IntegerOrSdlError::*;
        match err {
            IntegerOverflows(s, v) => Self::Overflow(Cow::from(s), v),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<TextureValueError> for Error {
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
    fn from(err: TargetRenderError) -> Self {
        use TargetRenderError::*;
        match err {
            NotSupported => Self::Other(Cow::from("Not supported")),
            SdlError(s) => s.into(),
        }
    }
}

impl From<SdlError> for Error {
    fn from(err: SdlError) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}

impl From<UpdateTextureError> for Error {
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
