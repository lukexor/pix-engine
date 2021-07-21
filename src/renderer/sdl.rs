use crate::{
    core::window::Error as WindowError,
    prelude::*,
    renderer::{Error, RendererSettings, Rendering, Result},
};
use lazy_static::lazy_static;
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    gfx::primitives::{DrawRenderer, ToColor},
    image::LoadSurface,
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
use std::{borrow::Cow, collections::HashMap, path::PathBuf};

mod audio;
mod event;
mod window;

lazy_static! {
    static ref TTF: Sdl2TtfContext = sdl2::ttf::init().expect("sdl2_ttf initialized");
}

/// An SDL [Renderer] implementation.
pub(crate) struct Renderer {
    context: Sdl,
    font: (PathBuf, u16),
    font_cache: HashMap<(PathBuf, u16), Font<'static, 'static>>,
    font_style: SdlFontStyle,
    event_pump: EventPump,
    window_id: WindowId,
    canvas: Canvas<SdlWindow>,
    audio_device: AudioQueue<f32>,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<SdlTexture>,
    blend_mode: SdlBlendMode,
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
        window_builder.opengl();
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
        if s.borderless {
            window_builder.borderless();
        }

        let window = window_builder.build()?;
        let window_id = window.id() as usize;
        let mut canvas_builder = window.into_canvas().target_texture();
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

        let mut font_cache = HashMap::new();
        let font = (s.font.clone(), s.font_size);
        font_cache.insert(font.clone(), TTF.load_font(&s.font, s.font_size)?);

        Ok(Self {
            context,
            font,
            font_cache,
            font_style: SdlFontStyle::NORMAL,
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
    fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect<Primitive>>) {
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
        rect: Option<Rect<Primitive>>,
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
        src: Option<Rect<Primitive>>,
        dst: Option<Rect<Primitive>>,
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
        self.font.1 = size as u16;
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
        pos: &Point<Primitive>,
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
                    Some(SdlRect::new(pos.x, pos.y, width, height)),
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
            Some(font) => Ok(font.size_of(text)?),
            None => Err(Error::InvalidFont(self.font.0.to_owned())),
        }
    }

    /// Draw a pixel to the current canvas.
    fn point(&mut self, p: &Point<DrawPrimitive>, color: Color) -> Result<()> {
        Ok(self.canvas.pixel(p.x, p.y, color)?)
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, line: &Line<DrawPrimitive>, color: Color) -> Result<()> {
        let [x1, y1, _, x2, y2, _] = line.values();
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
        tri: &Triangle<DrawPrimitive>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [x1, y1, _, x2, y2, _, x3, y3, _] = tri.values();
        if let Some(fill) = fill {
            self.canvas.filled_trigon(x1, y1, x2, y2, x3, y3, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas.trigon(x1, y1, x2, y2, x3, y3, stroke)?;
        }
        Ok(())
    }

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        rect: &Rect<DrawPrimitive>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [x, y, width, height] = rect.values();
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
        rect: &Rect<DrawPrimitive>,
        radius: DrawPrimitive,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [x, y, width, height] = rect.values();
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
    fn quad(
        &mut self,
        quad: &Quad<DrawPrimitive>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [x1, y1, _, x2, y2, _, x3, y3, _, x4, y4, _] = quad.values();
        let vx = [x1, x2, x3, x4];
        let vy = [y1, y2, y3, y4];
        if let Some(fill) = fill {
            self.canvas.filled_polygon(&vx, &vy, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas.polygon(&vx, &vy, stroke)?;
        }
        Ok(())
    }

    /// Draw a polygon to the current canvas.
    fn polygon(
        &mut self,
        vx: &[DrawPrimitive],
        vy: &[DrawPrimitive],
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        if let Some(fill) = fill {
            self.canvas.filled_polygon(vx, vy, fill)?;
        }
        if let Some(stroke) = stroke {
            self.canvas.polygon(vx, vy, stroke)?;
        }
        Ok(())
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: &Ellipse<DrawPrimitive>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let [x, y, width, height] = ellipse.values();
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
        p: &Point<DrawPrimitive>,
        radius: DrawPrimitive,
        start: DrawPrimitive,
        end: DrawPrimitive,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        use ArcMode::*;
        let [x, y, _] = p.values();
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
    fn image(&mut self, pos: &Point<Primitive>, img: &Image, tint: Option<Color>) -> Result<()> {
        let texture_id = img.texture_id();
        match self.textures.get_mut(texture_id) {
            Some(texture) => {
                texture.update(
                    None,
                    img.bytes(),
                    img.format().channels() * img.width() as usize,
                )?;
                if let Some(tint) = tint {
                    self.canvas.with_texture_canvas(texture, |tex_canvas| {
                        tex_canvas.set_blend_mode(SdlBlendMode::Mod);
                        tex_canvas.set_draw_color(tint);
                        let _ = tex_canvas.fill_rect(None);
                    })?;
                }
                texture.set_blend_mode(self.blend_mode);
                let dst = SdlRect::new(pos.x, pos.y, img.width() as u32, img.height() as u32);
                Ok(self.canvas.copy(&texture, None, dst)?)
            }
            None => Err(Error::InvalidTexture(texture_id)),
        }
    }

    /// Draw an image to the current canvas.
    fn image_resized(
        &mut self,
        dst_rect: &Rect<Primitive>,
        img: &Image,
        tint: Option<Color>,
    ) -> Result<()> {
        let texture_id = img.texture_id();
        match self.textures.get_mut(texture_id) {
            Some(texture) => {
                texture.update(
                    None,
                    img.bytes(),
                    img.format().channels() * img.width() as usize,
                )?;
                if let Some(tint) = tint {
                    self.canvas.with_texture_canvas(texture, |canvas| {
                        canvas.set_blend_mode(SdlBlendMode::Add);
                        canvas.set_draw_color(tint);
                        let _ = canvas.fill_rect(None);
                    })?;
                }
                texture.set_blend_mode(self.blend_mode);
                let dst_rect: SdlRect = dst_rect.into();
                Ok(self.canvas.copy(&texture, None, dst_rect)?)
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

impl From<Rect<Primitive>> for SdlRect {
    fn from(rect: Rect<Primitive>) -> Self {
        Self::new(rect.x, rect.y, rect.width as u32, rect.height as u32)
    }
}

impl From<&Rect<Primitive>> for SdlRect {
    fn from(rect: &Rect<Primitive>) -> Self {
        Self::new(rect.x, rect.y, rect.width as u32, rect.height as u32)
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
