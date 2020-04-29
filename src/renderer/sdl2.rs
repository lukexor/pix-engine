use super::{Error, Renderer, Result};
use crate::{
    color::Color,
    common::Scalar,
    event::PixEvent,
    shape::{Point, Rect},
    state_data::rendering::{BlendMode, Texture},
};
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    gfx::primitives::{DrawRenderer, ToColor},
    // controller::GameController,
    pixels,
    rect,
    render::Canvas,
    video::{self, Window},
    EventPump,
    // GameControllerSubsystem,
    Sdl,
};

mod event;
mod render;

/// Default audio sampling rate in Hertz
pub const DEFAULT_SAMPLE_RATE: i32 = 44_100;

/// An Sdl2Renderer that handles drawing, input, and audio in a cross-platform way.
pub(crate) struct Sdl2Renderer {
    context: Sdl,
    default_window_target: u32,
    window_target: Option<u32>,
    bg_color: pixels::Color,
    fill: Option<pixels::Color>,
    stroke: Option<pixels::Color>,
    canvases: Vec<Canvas<Window>>,
    audio_device: AudioQueue<f32>,
    event_pump: EventPump,
    // controller_sub: GameControllerSubsystem,
    // controllers: Vec<GameController>,
}

impl Sdl2Renderer {
    /// Creates a new instance of an SDL2 Renderer
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let context = sdl2::init()?;

        // Event pump & controller subsystem
        let event_pump = context.event_pump()?;
        // let controller_sub = context.game_controller()?;

        // Set up Audio
        let audio_sub = context.audio()?;
        let desired_spec = AudioSpecDesired {
            freq: Some(DEFAULT_SAMPLE_RATE),
            channels: Some(1),
            samples: None,
        };
        let audio_device = audio_sub.open_queue(None, &desired_spec)?;
        audio_device.resume();

        // Create default window
        let canvas = Self::new_canvas(&context, title, width, height)?;
        let default_window_target = canvas.window().id();

        Ok(Self {
            context,
            default_window_target,
            window_target: None,
            bg_color: pixels::Color::RGB(0, 0, 0),
            fill: None,
            stroke: None,
            canvases: vec![canvas],
            audio_device,
            event_pump,
            // controller_sub,
            // controllers: Vec::new(),
        })
    }

    /// Static method to create a new window and associated canvas.
    // TODO Make Windowbuilder interface for defining position, resizable, vsync, etc
    fn new_canvas(context: &Sdl, title: &str, width: u32, height: u32) -> Result<Canvas<Window>> {
        // Set up the window
        let video_sub = context.video()?;
        let window = video_sub
            .window(title, width, height)
            .position_centered()
            .build()?;

        // Set up canvas
        let mut canvas = window.into_canvas().build()?;
        canvas.set_logical_size(width, height)?;
        Ok(canvas)
    }

    /// Get a canvas based on the current window target.
    fn canvas(&self) -> &Canvas<Window> {
        let target = self.window_target();
        self.canvases
            .iter()
            .find(|c| target == c.window().id())
            .expect("valid window target")
    }

    /// Get a mutable canvas based on the current window target.
    fn canvas_mut(&mut self) -> &mut Canvas<Window> {
        let target = self.window_target();
        self.canvases
            .iter_mut()
            .find(|c| target == c.window().id())
            .expect("valid window target")
    }

    /// Window Management

    /// Returns the window_id of the current window target.
    fn window_target(&self) -> u32 {
        self.window_target.unwrap_or(self.default_window_target)
    }
}

impl Renderer for Sdl2Renderer {
    /// # Settings

    /// Set title for the current window target.
    ///
    /// Errors if the title contains a nul byte.
    fn title(&mut self, title: &str) -> Result<()> {
        Ok(self.canvas_mut().window_mut().set_title(title)?)
    }

    /// Sets the audio sample rate for the audio playback in Hz.
    fn audio_sample_rate(&mut self, rate: i32) -> Result<()> {
        let audio_sub = self.context.audio()?;
        let desired_spec = AudioSpecDesired {
            freq: Some(rate),
            channels: Some(1),
            samples: None,
        };
        self.audio_device = audio_sub.open_queue(None, &desired_spec)?;
        self.audio_device.resume();
        Ok(())
    }

    /// Set draw color for drawing operations on the current window target.
    fn background(&mut self, color: Color) {
        self.bg_color = color.into();
    }

    /// Set draw color for the fill operations on the current window target.
    fn fill(&mut self, color: Option<Color>) {
        self.fill = color.map(|c| c.into());
    }

    /// Set draw color for the drawing outlines on the current window target.
    fn stroke(&mut self, color: Option<Color>) {
        self.stroke = color.map(|c| c.into());
    }

    /// Get the blending mode for the current window target.
    fn get_blend_mode(&self) -> BlendMode {
        self.canvas().blend_mode().into()
    }

    /// Set the blending mode for drawing operations on the current window target.
    fn blend_mode(&mut self, mode: BlendMode) {
        self.canvas_mut().set_blend_mode(mode.into());
    }

    /// Get the scale_x and scale_y factors for the current window target.
    fn get_scale(&self) -> (f32, f32) {
        self.canvas().scale()
    }

    /// Set the scale_x and scale_y factors for the current window target.
    fn scale(&mut self, scale_x: f32, scale_y: f32) -> Result<()> {
        Ok(self.canvas_mut().set_scale(scale_x, scale_y)?)
    }

    /// # Input

    /// Returns a list of events from the event queue since last time poll_events
    /// was called.
    fn poll_events(&mut self) -> Vec<PixEvent> {
        self.sdl_poll_events()
    }

    /// # Rendering

    /// Presents changes made to the canvas on the current window target since present was last
    /// called.
    fn present(&mut self) {
        self.canvas_mut().present();
    }

    /// Presents changes made to the canvases of all windows since present was last called.
    fn present_all(&mut self) {
        for canvas in self.canvases.iter_mut() {
            canvas.present();
        }
    }

    /// Clears the canvas on the current window target to the current draw color.
    fn clear(&mut self) {
        let c = self.bg_color;
        let canvas = self.canvas_mut();
        canvas.set_draw_color(c);
        canvas.clear();
    }

    /// Clears all canvases of all windows to their current draw colors.
    fn clear_all(&mut self) {
        let c = self.bg_color;
        for canvas in self.canvases.iter_mut() {
            canvas.set_draw_color(c);
            canvas.clear();
        }
    }

    /// Get the clipping rectangle for the current window target.
    fn get_clip_rect(&self) -> Option<Rect> {
        self.canvas().clip_rect().map(|r| r.into())
    }

    /// Set the clipping rectangle for the current window target.
    fn clip_rect(&mut self, rect: Option<Rect>) {
        let rect: Option<rect::Rect> = rect.map(|r| r.into());
        self.canvas_mut().set_clip_rect(rect);
    }

    /// Get the viewport rectangle for the current window target.
    fn get_viewport(&self) -> Rect {
        self.canvas().viewport().into()
    }

    /// Set the viewport rectangle for the current window target.
    fn viewport(&mut self, rect: Option<Rect>) {
        let rect: Option<rect::Rect> = rect.map(|r| r.into());
        self.canvas_mut().set_viewport(rect);
    }

    /// # Drawing

    /// Draw a point on the current window target.
    fn point(&mut self, point: Point) -> Result<()> {
        if let Some(c) = self.stroke {
            let p = rect::Point::from((point.x, point.y));
            let canvas = self.canvas_mut();
            canvas.set_draw_color(c);
            canvas.draw_point(p)?;
        }
        Ok(())
    }

    /// Draw a line on the current window target.
    fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) -> Result<()> {
        if let Some(c) = self.stroke {
            let canvas = self.canvas_mut();
            canvas.set_draw_color(c);
            let p1 = rect::Point::from((x0, y0));
            let p2 = rect::Point::from((x1, y1));
            canvas.draw_line(p1, p2)?;
        }
        Ok(())
    }

    /// Draw a rectangle on the current window target.
    fn rect(&mut self, rect: Rect) -> Result<()> {
        let rect: rect::Rect = rect.into();
        if let Some(c) = self.stroke {
            let canvas = self.canvas_mut();
            canvas.set_draw_color(c);
            canvas.draw_rect(rect)?;
        }
        if let Some(c) = self.fill {
            let canvas = self.canvas_mut();
            canvas.set_draw_color(c);
            canvas.fill_rect(rect)?;
        }
        Ok(())
    }

    /// Reads pixels from the current window target.
    ///
    /// # Remarks
    /// WARNING: This is a very slow operation, and should not be used frequently.
    fn read_pixels(&self, rect: Rect) -> Result<Vec<u8>> {
        // Ok(self.canvas_mut().read_pixels(rect.into())?)
        unimplemented!();
    }

    /// # Textures

    /// Copy all or a portion of a texture to the current window target.
    ///
    /// - If `src` is `None`, the entire texture is copied.
    /// - If `dst` is `None`, the texture will be stretched to fill the entire target.
    fn copy(&mut self, texture: Texture, src: Option<Rect>, dst: Option<Rect>) -> Result<()> {
        // TODO Sdl2Renderer::copy
        // Ok(self.canvas_mut().copy(texture.into())?)
        unimplemented!();
    }

    fn triangle(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
        x3: Scalar,
        y3: Scalar,
        color: Color,
    ) -> Result<()> {
        let (x1, y1) = (x1.round() as i16, y1.round() as i16);
        let (x2, y2) = (x2.round() as i16, y2.round() as i16);
        let (x3, y3) = (x3.round() as i16, y3.round() as i16);
        let canvas = self.canvas_mut();
        canvas.trigon(x1, y1, x2, y2, x3, y3, color)?;
        canvas.filled_trigon(x1, y1, x2, y2, x3, y3, color)?;
        Ok(())
    }
}

impl ToColor for Color {
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        (
            self.red().round() as u8,
            self.green().round() as u8,
            self.blue().round() as u8,
            self.alpha().round() as u8,
        )
    }
}

impl From<video::WindowBuildError> for Error {
    fn from(err: video::WindowBuildError) -> Self {
        use video::WindowBuildError::*;
        match err {
            HeightOverflows(h) => Self::InvalidHeight(h),
            WidthOverflows(w) => Self::InvalidWidth(w),
            InvalidTitle(err) => Self::InvalidString(err),
            SdlError(err) => Self::Other(err.into()),
        }
    }
}

impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(err: sdl2::IntegerOrSdlError) -> Self {
        use sdl2::IntegerOrSdlError::*;
        match err {
            IntegerOverflows(err, val) => Self::IntegerOverflows(err.into(), val),
            SdlError(err) => Self::Other(err.into()),
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(err: std::ffi::NulError) -> Self {
        Self::InvalidString(err)
    }
}

impl From<pixels::Color> for Color {
    fn from(color: pixels::Color) -> Self {
        let (r, g, b) = color.rgb();
        [r, g, b].into()
    }
}

impl Into<pixels::Color> for Color {
    fn into(self) -> pixels::Color {
        pixels::Color::RGBA(
            self.red().round() as u8,
            self.green().round() as u8,
            self.blue().round() as u8,
            self.alpha().round() as u8,
        )
    }
}

impl From<rect::Rect> for Rect {
    fn from(rect: rect::Rect) -> Self {
        Rect::new(rect.x(), rect.y(), rect.width(), rect.height())
    }
}

impl Into<rect::Rect> for Rect {
    fn into(self) -> rect::Rect {
        rect::Rect::new(self.x, self.y, self.w, self.h)
    }
}

impl From<rect::Point> for Point {
    fn from(point: rect::Point) -> Self {
        Point::new((point.x(), point.y()))
    }
}

impl Into<rect::Point> for Point {
    fn into(self) -> rect::Point {
        rect::Point::new(self.x, self.y)
    }
}
