//! SDL Renderer implementation

use super::{Position, RendererError, RendererResult, RendererSettings, Rendering};
use crate::{color::Color, image::Image, shape::Rect};
use sdl2::{
    gfx::primitives::{DrawRenderer, ToColor},
    pixels::PixelFormatEnum,
    render::{
        Canvas, Texture, TextureCreator, TextureQuery, TextureValueError, UpdateTextureError,
    },
    ttf::{self, FontError, InitError},
    video::WindowBuildError,
    video::{FullscreenType, Window, WindowContext},
    EventPump, IntegerOrSdlError, Sdl,
};
use std::{borrow::Cow, convert::TryFrom, ffi::NulError};

/// Wrapper for Sdl2 Color
pub type SdlColor = sdl2::pixels::Color;
/// Wrapper for Sdl2 EventPollIterator
pub type SdlEventIterator<'a> = sdl2::event::EventPollIterator<'a>;
/// Wrapper for Sdl2 Event
pub type SdlEvent = sdl2::event::Event;
/// Wrapper for Sdl2 WindowEvent
pub type SdlWindowEvent = sdl2::event::WindowEvent;
/// Wrapper for Sdl2 Rect
pub type SdlRect = sdl2::rect::Rect;
/// Wrapper for Sdl2 MouseButton
pub type SdlMouseButton = sdl2::mouse::MouseButton;
/// Wrapper for Sdl2 Keycode
pub type SdlKeycode = sdl2::keyboard::Keycode;

/// An SDL renderer implementation.
pub struct SdlRenderer {
    title: String,
    context: Sdl,
    ttf_context: ttf::Sdl2TtfContext,
    event_pump: EventPump,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    textures: Vec<Texture>,
}

impl std::fmt::Debug for SdlRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SdlRenderer {{}}")
    }
}

impl Rendering for SdlRenderer {
    /// Initializes the Sdl2Renderer using the given settings and opens a new window.
    fn init(settings: &RendererSettings) -> RendererResult<Self> {
        let s = settings;

        let context = sdl2::init()?;
        let ttf_context = ttf::init()?;
        let video_subsys = context.video()?;
        let event_pump = context.event_pump()?;

        // Set up window with options
        let win_width = (s.scale_x * s.width as f32).floor() as u32;
        let win_height = (s.scale_y * s.height as f32).floor() as u32;
        let mut window_builder = video_subsys.window(&s.title, win_width, win_height);
        match (s.x, s.y) {
            (Position::Centered, Position::Centered) => {
                let _ = window_builder.position_centered();
            }
            (Position::Positioned(x), Position::Positioned(y)) => {
                let _ = window_builder.position(x, y);
            }
            _ => return Err(RendererError::InvalidPosition(s.x, s.y)),
        };
        if s.fullscreen {
            let _ = window_builder.fullscreen();
        }
        if s.resizable {
            let _ = window_builder.resizable();
        }

        let mut canvas_builder = window_builder
            .build()?
            .into_canvas()
            .accelerated()
            .target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build()?;
        canvas.set_logical_size(win_width, win_height)?;
        canvas.set_scale(s.scale_x, s.scale_y)?;

        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        // TODO font context
        // let font = ttf_context.load_font("static/emulogic.ttf", 16)?;

        Ok(Self {
            title: s.title.to_owned(),
            context,
            ttf_context,
            event_pump,
            canvas,
            texture_creator,
            textures: Vec::new(),
            // font,
        })
    }

    /// Clears the canvas to the current clear color.
    fn clear(&mut self) {
        self.canvas.clear();
    }

    /// Set whether the cursor is shown or not.
    fn show_cursor(&mut self, show: bool) {
        self.context.mouse().show_cursor(show);
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn set_clip_rect(&mut self, rect: Option<Rect>) {
        let rect = rect.map(|rect| rect.into());
        self.canvas.set_clip_rect(rect);
    }

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<SdlEvent> {
        self.event_pump.poll_event()
    }

    /// Returns an iterator over events in the event pump.
    fn poll_event_iter(&mut self) -> SdlEventIterator<'_> {
        self.event_pump.poll_iter()
    }

    /// Updates the canvas from the current back buffer.
    fn present(&mut self) {
        self.canvas.present();
    }

    /// Get the current window title.
    fn title(&self) -> &str {
        &self.title
    }

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> RendererResult<()> {
        self.canvas.window_mut().set_title(title)?;
        Ok(())
    }

    /// Width of the current canvas.
    fn width(&self) -> u32 {
        let (width, _) = self.canvas.output_size().unwrap_or((0, 0));
        let (scale_x, _) = self.canvas.scale();
        ((width as f32) / scale_x) as u32
    }

    /// Height of the current canvas.
    fn height(&self) -> u32 {
        let (_, height) = self.canvas.output_size().unwrap_or((0, 0));
        let (_, scale_y) = self.canvas.scale();
        ((height as f32) / scale_y) as u32
    }

    /// Scale the current canvas.
    fn set_scale(&mut self, x: f32, y: f32) -> RendererResult<()> {
        self.canvas.set_scale(x, y)?;
        Ok(())
    }

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool {
        use FullscreenType::*;
        match self.canvas.window().fullscreen_state() {
            True | Desktop => true,
            _ => false,
        }
    }

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool) {
        let fullscreen_type = if val {
            FullscreenType::True
        } else {
            FullscreenType::Off
        };
        // Don't care if this fails or not.
        let _ = self.canvas.window_mut().set_fullscreen(fullscreen_type);
    }

    /// Create a texture to render to.
    fn create_texture(&mut self, width: u32, height: u32) -> RendererResult<usize> {
        let texture = self
            .texture_creator
            .create_texture_streaming(None, width, height)?;
        self.textures.push(texture);
        Ok(self.textures.len())
    }

    /// Draw an array of pixels to the canvas.
    fn draw_pixels(&mut self, pixels: &[u8], pitch: usize) -> RendererResult<()> {
        self.textures[0].update(None, pixels, pitch)?;
        self.canvas.copy(&self.textures[0], None, None)?;
        Ok(())
    }

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        fill: Option<Color>,
        _stroke: Option<Color>,
    ) -> RendererResult<()> {
        let font = self.ttf_context.load_font("static/emulogic.ttf", 16)?;
        if let Some(fill) = fill {
            let surface = font.render(text).blended(fill)?;
            let texture = self.texture_creator.create_texture_from_surface(&surface)?;
            let TextureQuery { width, height, .. } = texture.query();
            self.canvas
                .copy(&texture, None, Some(SdlRect::new(x, y, width, height)))?;
        }
        Ok(())
    }

    /// Draw a pixel to the current canvas.
    fn pixel(&mut self, x: i32, y: i32, stroke: Option<Color>) -> RendererResult<()> {
        let x = i16::try_from(x)?;
        let y = i16::try_from(y)?;
        if let Some(stroke) = stroke {
            self.canvas.pixel(x, y, stroke)?;
        }
        Ok(())
    }

    /// Draw a line to the current canvas.
    fn line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        let x1 = i16::try_from(x1)?;
        let y1 = i16::try_from(y1)?;
        let x2 = i16::try_from(x2)?;
        let y2 = i16::try_from(y2)?;
        if let Some(stroke) = stroke {
            self.canvas.line(x1, y1, x2, y2, stroke)?;
        }
        Ok(())
    }

    /// Draw a triangle to the current canvas.
    fn triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        let x1 = i16::try_from(x1)?;
        let y1 = i16::try_from(y1)?;
        let x2 = i16::try_from(x2)?;
        let y2 = i16::try_from(y2)?;
        let x3 = i16::try_from(x3)?;
        let y3 = i16::try_from(y3)?;
        if let Some(stroke) = stroke {
            self.canvas.trigon(x1, y1, x2, y2, x3, y3, stroke)?;
        }
        if let Some(fill) = fill {
            self.canvas.filled_trigon(x1, y1, x2, y2, x3, y3, fill)?;
        }
        Ok(())
    }

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        let x = i16::try_from(x)?;
        let y = i16::try_from(y)?;
        let w = i16::try_from(width)?;
        let h = i16::try_from(height)?;
        if let Some(stroke) = stroke {
            self.canvas.rectangle(x, y, x + w - 1, y + h - 1, stroke)?;
        }
        if let Some(fill) = fill {
            self.canvas.box_(x, y, x + w - 1, y + h - 1, fill)?;
        }
        Ok(())
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> RendererResult<()> {
        let x = i16::try_from(x)?;
        let y = i16::try_from(y)?;
        let w = i16::try_from(width)?;
        let h = i16::try_from(height)?;
        if let Some(stroke) = stroke {
            self.canvas.ellipse(x, y, w, h, stroke)?;
        }
        if let Some(fill) = fill {
            self.canvas.filled_ellipse(x, y, w, h, fill)?;
        }
        Ok(())
    }

    // TODO: Move texture creation into image object?
    /// Draw an image to the current canvas.
    fn image(&mut self, x: i32, y: i32, img: &Image) -> RendererResult<()> {
        let mut texture = self.texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24,
            img.width,
            img.height,
        )?;
        texture.update(None, &img.data, 3 * img.width as usize)?;
        texture.set_blend_mode(sdl2::render::BlendMode::Mod);
        let dst = SdlRect::new(x, y, img.width, img.height);
        self.canvas.copy(&texture, None, dst)?;
        Ok(())
    }
}

/*
 * Type Conversions
 */

impl ToColor for Color {
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        use Color::*;
        match self {
            Rgb(rgb) => rgb.channels(),
            Hsv(hsv) => hsv.to_rgb().channels(),
        }
    }
}

impl From<Color> for SdlColor {
    fn from(color: Color) -> Self {
        use Color::*;
        let rgb = match color {
            Rgb(rgb) => rgb,
            Hsv(hsv) => hsv.to_rgb(),
        };
        Self::RGBA(rgb.red(), rgb.green(), rgb.blue(), rgb.alpha())
    }
}

impl From<Rect> for SdlRect {
    fn from(rect: Rect) -> Self {
        Self::new(rect.x, rect.y, rect.w, rect.h)
    }
}

/*
 * Error Conversions
 */

impl From<String> for RendererError {
    fn from(err: String) -> Self {
        Self::Other(Cow::from(err))
    }
}

impl From<InitError> for RendererError {
    fn from(err: InitError) -> Self {
        use InitError::*;
        match err {
            InitializationError(err) => Self::IoError(err),
            AlreadyInitializedError => Self::InitError,
        }
    }
}

impl From<FontError> for RendererError {
    fn from(err: FontError) -> Self {
        use FontError::*;
        match err {
            InvalidLatin1Text(e) => Self::InvalidText("invalid latin1 text", e),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<WindowBuildError> for RendererError {
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

impl From<IntegerOrSdlError> for RendererError {
    fn from(err: IntegerOrSdlError) -> Self {
        use IntegerOrSdlError::*;
        match err {
            IntegerOverflows(s, v) => Self::Overflow(Cow::from(s), v),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<TextureValueError> for RendererError {
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

impl From<UpdateTextureError> for RendererError {
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

impl From<NulError> for RendererError {
    fn from(err: NulError) -> Self {
        Self::InvalidText("Unknown nul error", err)
    }
}
