//! SDL Renderer implementation

use super::{Position, RendererSettings, Rendering};
use crate::{
    color::Color,
    common::{Error, Result},
};
use sdl2::{
    gfx::primitives::{DrawRenderer, ToColor},
    pixels::PixelFormatEnum,
    render::{Canvas, Texture, TextureCreator},
    video::{FullscreenType, Window, WindowContext},
    EventPump, Sdl,
};
use std::{borrow::Cow, convert::TryFrom};

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
    fn init(settings: &RendererSettings) -> Result<Self> {
        let s = settings;

        let context = sdl2::init()?;
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
            _ => return Err(Error::InvalidSetting),
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
        let textures = vec![texture_creator.create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            s.width,
            s.height,
        )?];

        Ok(Self {
            title: s.title.to_owned(),
            context,
            event_pump,
            canvas,
            texture_creator,
            textures,
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
    fn set_title(&mut self, title: &str) -> Result<()> {
        self.canvas.window_mut().set_title(title)?;
        Ok(())
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
    fn set_scale(&mut self, x: f32, y: f32) -> Result<()> {
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
    fn create_texture(&mut self, width: u32, height: u32) -> Result<usize> {
        let texture = self
            .texture_creator
            .create_texture_streaming(None, width, height)?;
        self.textures.push(texture);
        Ok(self.textures.len())
    }

    /// Draw an array of pixels to the canvas.
    fn draw_pixels(&mut self, pixels: &[u8], pitch: usize) -> Result<()> {
        self.textures[0].update(None, pixels, pitch)?;
        self.canvas.copy(&self.textures[0], None, None)?;
        Ok(())
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, stroke: Option<Color>) -> Result<()> {
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
    ) -> Result<()> {
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
    ) -> Result<()> {
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
}

impl ToColor for Color {
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        match self {
            Color::Rgb(rgb) => rgb.channels(),
            Color::Hsv(hsv) => hsv.to_rgb().channels(),
        }
    }
}

impl From<sdl2::video::WindowBuildError> for Error {
    fn from(err: sdl2::video::WindowBuildError) -> Error {
        Error::Other(Cow::from(err.to_string()))
    }
}
impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(err: sdl2::IntegerOrSdlError) -> Error {
        Error::Other(Cow::from(err.to_string()))
    }
}
impl From<std::ffi::NulError> for Error {
    fn from(err: std::ffi::NulError) -> Error {
        Error::Other(Cow::from(err.to_string()))
    }
}
impl From<sdl2::render::TextureValueError> for Error {
    fn from(err: sdl2::render::TextureValueError) -> Error {
        Error::Other(Cow::from(err.to_string()))
    }
}
impl From<sdl2::render::UpdateTextureError> for Error {
    fn from(err: sdl2::render::UpdateTextureError) -> Error {
        Error::Other(Cow::from(err.to_string()))
    }
}
impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Other(Cow::from(err))
    }
}
impl From<Color> for sdl2::pixels::Color {
    fn from(color: Color) -> Self {
        let rgb = match color {
            Color::Rgb(rgb) => rgb,
            Color::Hsv(hsv) => hsv.to_rgb(),
        };
        Self::RGBA(rgb.red(), rgb.green(), rgb.blue(), rgb.alpha())
    }
}
