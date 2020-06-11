//! SDL Renderer implementation

use super::{Position, RendererSettings, Rendering};
use crate::{
    color::Color,
    common::{Error, Result},
};
use sdl2::{
    gfx::primitives::{DrawRenderer, ToColor},
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator, TextureQuery},
    ttf,
    video::{FullscreenType, Window, WindowContext},
    EventPump,
};
use std::convert::TryFrom;

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
    // context: Sdl,
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
    fn init(settings: &RendererSettings) -> Result<Self> {
        let s = settings;

        let context = sdl2::init().map_err(Error::renderer)?;
        let ttf_context = ttf::init().map_err(Error::renderer)?;
        let video_subsys = context.video().map_err(Error::renderer)?;
        let event_pump = context.event_pump().map_err(Error::renderer)?;

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
            .build()
            .map_err(Error::renderer)?
            .into_canvas()
            .accelerated()
            .target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build().map_err(Error::renderer)?;
        canvas
            .set_logical_size(win_width, win_height)
            .map_err(Error::renderer)?;
        canvas
            .set_scale(s.scale_x, s.scale_y)
            .map_err(Error::renderer)?;

        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
        let textures = vec![texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, s.width, s.height)
            .map_err(Error::renderer)?];

        // let font = ttf_context.load_font("static/emulogic.ttf", 16)?;

        Ok(Self {
            title: s.title.to_owned(),
            // context,
            ttf_context,
            event_pump,
            canvas,
            texture_creator,
            textures,
            // font,
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
        self.canvas
            .window_mut()
            .set_title(title)
            .map_err(Error::renderer)?;
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
            .create_texture_streaming(None, width, height)
            .map_err(Error::renderer)?;
        self.textures.push(texture);
        Ok(self.textures.len())
    }

    /// Draw an array of pixels to the canvas.
    fn draw_pixels(&mut self, pixels: &[u8], pitch: usize) -> Result<()> {
        self.textures[0]
            .update(None, pixels, pitch)
            .map_err(Error::renderer)?;
        self.canvas
            .copy(&self.textures[0], None, None)
            .map_err(Error::renderer)?;
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
    ) -> Result<()> {
        let font = self.ttf_context.load_font("static/emulogic.ttf", 16)?;
        if let Some(fill) = fill {
            let surface = font.render(text).blended(fill).map_err(Error::renderer)?;
            let texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .map_err(Error::renderer)?;
            let TextureQuery { width, height, .. } = texture.query();
            self.canvas
                .copy(&texture, None, Some(Rect::new(x, y, width, height)))?;
        }
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

    /// Draw a ellipse to the current canvas.
    fn ellipse(
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
            self.canvas.ellipse(x, y, w, h, stroke)?;
        }
        if let Some(fill) = fill {
            self.canvas.filled_ellipse(x, y, w, h, fill)?;
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

impl From<Color> for sdl2::pixels::Color {
    fn from(color: Color) -> Self {
        let rgb = match color {
            Color::Rgb(rgb) => rgb,
            Color::Hsv(hsv) => hsv.to_rgb(),
        };
        Self::RGBA(rgb.red(), rgb.green(), rgb.blue(), rgb.alpha())
    }
}
