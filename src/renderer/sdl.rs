//! SDL Renderer implementation

use crate::{
    prelude::*,
    renderer::{Error, RendererSettings, Rendering, Result},
    state::settings::FontSettings,
    window::Error as WindowError,
};
use lazy_static::lazy_static;
use num_traits::AsPrimitive;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    gfx::primitives::{DrawRenderer, ToColor},
    image::LoadSurface,
    pixels::{Color as SdlColor, PixelFormatEnum as SdlPixelFormat},
    rect::Rect as SdlRect,
    render::{
        BlendMode as SdlBlendMode, Canvas, Texture as SdlTexture, TextureCreator, TextureQuery,
        TextureValueError, UpdateTextureError,
    },
    surface::Surface,
    ttf::{Font, FontError, FontStyle as SdlFontStyle, InitError, Sdl2TtfContext},
    video::{Window as SdlWindow, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError, Sdl,
};
use std::{borrow::Cow, ffi::NulError};

mod audio;
mod event;
mod window;

lazy_static! {
    static ref TTF: Sdl2TtfContext = sdl2::ttf::init().unwrap();
}

/// An SDL [`Renderer`] implementation.
pub(crate) struct Renderer {
    context: Sdl,
    font: Font<'static, 'static>,
    event_pump: EventPump,
    window_id: WindowId,
    canvas: Canvas<SdlWindow>,
    audio_device: AudioQueue<f32>,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<SdlTexture>,
    blend_mode: SdlBlendMode,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new(&RendererSettings::default()).expect("SDL2 Renderer")
    }
}

impl Rendering for Renderer {
    /// Initializes the Sdl2Renderer using the given settings and opens a new window.
    fn new(s: &RendererSettings) -> Result<Self> {
        let context = sdl2::init()?;
        let video_subsys = context.video()?;
        let event_pump = context.event_pump()?;

        // Set up window with options
        let win_width = (s.scale_x * s.width as f32).floor() as u32;
        let win_height = (s.scale_y * s.height as f32).floor() as u32;
        let mut window_builder = video_subsys.window(&s.title, win_width, win_height);
        match (s.x, s.y) {
            (Position::Centered, Position::Centered) => {
                window_builder.position_centered();
            }
            (Position::Positioned(x), Position::Positioned(y)) => {
                window_builder.position(x, y);
            }
            _ => return Err(WindowError::InvalidPosition(s.x, s.y).into()),
        };
        if s.fullscreen {
            window_builder.fullscreen();
        }
        if s.resizable {
            window_builder.resizable();
        }

        let window = window_builder.build()?;
        let window_id = window.id();
        let mut canvas_builder = window.into_canvas().accelerated().target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build()?;
        canvas.set_logical_size(win_width, win_height)?;
        canvas.set_scale(s.scale_x, s.scale_y)?;

        if let Some(icon) = &s.icon {
            let surface = Surface::from_file(icon)?;
            canvas.window_mut().set_icon(surface);
        }

        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        // Set up Audio
        let audio_sub = context.audio()?;
        let desired_spec = AudioSpecDesired {
            freq: Some(s.audio_sample_rate),
            channels: Some(1),
            samples: None,
        };
        let audio_device = audio_sub.open_queue(None, &desired_spec)?;
        audio_device.resume();

        let font = TTF.load_font("static/Songti_SC.ttf", 16)?;

        Ok(Self {
            context,
            font,
            event_pump,
            window_id,
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
    fn set_draw_color(&mut self, color: impl Into<Color>) {
        self.canvas.set_draw_color(color.into());
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip<T, R>(&mut self, rect: R)
    where
        T: AsPrimitive<Scalar>,
        R: Into<Option<Rect<T>>>,
    {
        let rect = rect.into().map(|rect| rect.into());
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

    /// Width of the current canvas.
    fn width(&self) -> u32 {
        let (width, _) = self.canvas.output_size().unwrap_or((0, 0));
        width
    }

    /// Height of the current canvas.
    fn height(&self) -> u32 {
        let (_, height) = self.canvas.output_size().unwrap_or((0, 0));
        height
    }

    /// Scale the current canvas.
    fn scale<T: AsPrimitive<f32>>(&mut self, x: T, y: T) -> Result<()> {
        Ok(self.canvas.set_scale(x.as_(), y.as_())?)
    }

    /// Create a texture to render to.
    fn create_texture<T, F>(&mut self, width: T, height: T, format: F) -> Result<TextureId>
    where
        T: Into<Scalar>,
        F: Into<Option<PixelFormat>>,
    {
        let texture_id = self.textures.len();
        self.textures
            .push(self.texture_creator.create_texture_streaming(
                format.into().map(|f| f.into()),
                width.into().round() as u32,
                height.into().round() as u32,
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
    fn update_texture<R, P>(
        &mut self,
        texture_id: TextureId,
        rect: R,
        pixels: P,
        pitch: usize,
    ) -> Result<()>
    where
        R: Into<Option<Rect<Scalar>>>,
        P: AsRef<[u8]>,
    {
        if let Some(texture) = self.textures.get_mut(texture_id) {
            let rect: Option<SdlRect> = rect.into().map(|r| r.into());
            let pixels = pixels.as_ref();
            Ok(texture.update(rect, pixels, pitch)?)
        } else {
            Err(Error::InvalidTexture(texture_id))
        }
    }

    /// Draw texture canvas.
    fn texture<R>(&mut self, texture_id: usize, src: R, dst: R) -> Result<()>
    where
        R: Into<Option<Rect<Scalar>>>,
    {
        if let Some(texture) = self.textures.get_mut(texture_id) {
            let src: Option<SdlRect> = src.into().map(|r| r.into());
            let dst: Option<SdlRect> = dst.into().map(|r| r.into());
            Ok(self.canvas.copy(texture, src, dst)?)
        } else {
            Err(Error::InvalidTexture(texture_id))
        }
    }

    /// Draw text to the current canvas.
    fn text<P, T, C>(
        &mut self,
        position: P,
        text: T,
        s: &FontSettings,
        fill: C,
        _stroke: C,
    ) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        T: AsRef<str>,
        C: Into<Option<Color>>,
    {
        let p = position.into().as_i32();
        let text = text.as_ref();
        // TODO: This path only works locally
        if let Some(fill) = fill.into() {
            self.font.set_style(s.style.into());
            let surface = self.font.render(text.as_ref()).blended(fill)?;
            let texture = self.texture_creator.create_texture_from_surface(&surface)?;
            let TextureQuery { width, height, .. } = texture.query();
            self.canvas
                .copy(&texture, None, Some(SdlRect::new(p.x, p.y, width, height)))?;
            // self.canvas.string(p.x, p.y, text, fill)?;
        }
        Ok(())
    }

    /// Draw a pixel to the current canvas.
    fn point<P, C>(&mut self, p: P, color: C) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        C: Into<Option<Color>>,
    {
        if let Some(color) = color.into() {
            let p = p.into().as_i16();
            self.canvas.pixel(p.x, p.y, color)?;
        }
        Ok(())
    }

    /// Draw a line to the current canvas.
    fn line<L, C>(&mut self, line: L, color: C) -> Result<()>
    where
        L: Into<Line<Scalar>>,
        C: Into<Option<Color>>,
    {
        if let Some(color) = color.into() {
            let line = line.into().as_i16();
            let [x1, y1, x2, y2]: [i16; 4] = line.into();
            if y1 == y2 {
                self.canvas.hline(x1, x2, y1, color)?;
            } else if x1 == x2 {
                self.canvas.vline(y1, y2, x1, color)?;
            } else {
                self.canvas.line(x1, y1, x2, y2, color)?;
            }
        }
        Ok(())
    }

    /// Draw a triangle to the current canvas.
    fn triangle<T, C>(&mut self, tri: T, fill: C, stroke: C) -> Result<()>
    where
        T: Into<Triangle<Scalar>>,
        C: Into<Option<Color>>,
    {
        let tri = tri.into().as_i16();
        let [x1, y1, x2, y2, x3, y3]: [i16; 6] = tri.into();
        if let Some(fill) = fill.into() {
            self.canvas.filled_trigon(x1, y1, x2, y2, x3, y3, fill)?;
        }
        if let Some(stroke) = stroke.into() {
            self.canvas.trigon(x1, y1, x2, y2, x3, y3, stroke)?;
        }
        Ok(())
    }

    /// Draw a rectangle to the current canvas.
    fn rect<R, C>(&mut self, rect: R, fill: C, stroke: C) -> Result<()>
    where
        R: Into<Rect<Scalar>>,
        C: Into<Option<Color>>,
    {
        let rect = rect.into().as_i16();
        let [x, y, width, height]: [i16; 4] = rect.into();
        if let Some(fill) = fill.into() {
            self.canvas
                .box_(x, y, x + width - 1, y + height - 1, fill)?;
        }
        if let Some(stroke) = stroke.into() {
            self.canvas
                .rectangle(x, y, x + width - 1, y + height - 1, stroke)?;
        }
        Ok(())
    }

    /// Draw a polygon to the current canvas.
    fn polygon<C, V>(&mut self, vx: V, vy: V, fill: C, stroke: C) -> Result<()>
    where
        C: Into<Option<Color>>,
        V: AsRef<[Scalar]>,
    {
        let vx: Vec<i16> = vx.as_ref().iter().map(|v| v.round() as i16).collect();
        let vy: Vec<i16> = vy.as_ref().iter().map(|v| v.round() as i16).collect();
        if let Some(fill) = fill.into() {
            self.canvas.filled_polygon(&vx, &vy, fill)?;
        }
        if let Some(stroke) = stroke.into() {
            self.canvas.polygon(&vx, &vy, stroke)?;
        }
        Ok(())
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse<E, C>(&mut self, ellipse: E, fill: C, stroke: C) -> Result<()>
    where
        E: Into<Ellipse<Scalar>>,
        C: Into<Option<Color>>,
    {
        let ellipse = ellipse.into().as_i16();
        let [x, y, width, height]: [i16; 4] = ellipse.into();
        if let Some(fill) = fill.into() {
            self.canvas.filled_ellipse(x, y, width, height, fill)?;
        }
        if let Some(stroke) = stroke.into() {
            self.canvas.ellipse(x, y, width, height, stroke)?;
        }
        Ok(())
    }

    /// Draw an image to the current canvas.
    fn image<P>(&mut self, position: P, img: &Image) -> Result<()>
    where
        P: Into<Point<Scalar>>,
    {
        if let Some(texture) = self.textures.get_mut(img.texture_id()) {
            let pos = position.into();
            texture.update(
                None,
                img.bytes(),
                img.format().channels() * img.width() as usize,
            )?;
            texture.set_blend_mode(self.blend_mode);
            let dst = SdlRect::new(pos.x as i32, pos.y as i32, img.width(), img.height());
            self.canvas.copy(&texture, None, dst)?;
        }
        Ok(())
    }

    /// Draw an image to the current canvas.
    fn image_resized<R>(&mut self, dst_rect: R, img: &Image) -> Result<()>
    where
        R: Into<Rect<Scalar>>,
    {
        if let Some(texture) = self.textures.get_mut(img.texture_id()) {
            texture.update(
                None,
                img.bytes(),
                img.format().channels() * img.width() as usize,
            )?;
            texture.set_blend_mode(self.blend_mode);
            let dst: SdlRect = dst_rect.into().into();
            self.canvas.copy(&texture, None, dst)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.canvas.output_size().unwrap_or((0, 0));
        write!(
            f,
            "SdlRenderer {{ window_id: {}, title: {}, width: {}, height: {}, draw_color: {:?}, blend_mode: {:?} }}",
            self.window_id,
            self.canvas.window().title(),
            width,
            height,
            self.canvas.draw_color(),
            self.blend_mode,
        )
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

impl<T> From<Rect<T>> for SdlRect
where
    T: AsPrimitive<Scalar>,
{
    fn from(rect: Rect<T>) -> Self {
        Self::new(
            rect.x.as_() as i32,
            rect.y.as_() as i32,
            rect.width.as_() as u32,
            rect.height.as_() as u32,
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
            Indexed => SdlPixelFormat::Index8,
            Grayscale => SdlPixelFormat::Index8,
            GrayscaleAlpha => SdlPixelFormat::Index8, // TODO: This is likely not correct
            Rgb => SdlPixelFormat::RGB24,
            Rgba => SdlPixelFormat::RGBA32,
        }
    }
}

/*
 * Error Conversions
 */

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(Cow::from(err))
    }
}

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

impl From<IntegerOrSdlError> for WindowError {
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

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Self::InvalidText("unknown nul error", err)
    }
}
