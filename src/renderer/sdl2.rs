use super::{Error, Renderer, Result};
use crate::{
    color::Color,
    event::PixEvent,
    shape::{Point, Rect},
    state::rendering::BlendMode,
};
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    controller::GameController,
    pixels, rect,
    render::Canvas,
    video::{self, Window},
    EventPump, GameControllerSubsystem, Sdl,
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
    controller_sub: GameControllerSubsystem,
    controllers: Vec<GameController>,
}

impl Sdl2Renderer {
    /// Creates a new instance of an SDL2 Renderer
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        let context = sdl2::init()?;

        // Event pump & controller subsystem
        let event_pump = context.event_pump()?;
        let controller_sub = context.game_controller()?;

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
            controller_sub,
            controllers: Vec::new(),
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
            .resizable()
            .build()?;

        // Set up canvas
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .target_texture()
            .present_vsync()
            .build()?;
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

    /// Returns the window_id of the default window target.
    fn default_window(&self) -> u32 {
        self.default_window_target
    }

    /// Returns the window_id of the current window target.
    fn window_target(&self) -> u32 {
        self.window_target.unwrap_or(self.default_window_target)
    }
}

impl Renderer for Sdl2Renderer {
    /// Settings

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
    fn background<C: Into<Color>>(&mut self, color: C) {
        let c = color.into();
        self.bg_color = c.into();
    }

    /// Set draw color for the fill operations on the current window target.
    fn fill<C: Into<Option<Color>>>(&mut self, color: C) {
        let c = color.into();
        self.fill = c.map(|c| c.into());
    }

    /// Set draw color for the drawing outlines on the current window target.
    fn stroke<C: Into<Option<Color>>>(&mut self, color: C) {
        let c = color.into();
        self.stroke = c.map(|c| c.into());
    }

    /// Get the blending mode for the current window target.
    fn get_blend_mode(&self) -> BlendMode {
        self.canvas().blend_mode().into()
    }

    /// Set the blending mode for drawing operations on the current window target.
    fn blend_mode(&mut self, mode: BlendMode) {
        self.canvas_mut().set_blend_mode(mode.into());
    }

    /// Returns a list of events from the event queue since last time poll_events
    /// was called.
    fn poll_events(&mut self) -> Vec<PixEvent> {
        self.sdl_poll_events()
    }

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

    /// Get the scale_x and scale_y factors for the current window target.
    fn get_scale(&self) -> (f32, f32) {
        self.canvas().scale()
    }

    /// Set the scale_x and scale_y factors for the current window target.
    fn scale(&mut self, scale_x: f32, scale_y: f32) -> Result<()> {
        Ok(self.canvas_mut().set_scale(scale_x, scale_y)?)
    }

    /// Get the clipping rectangle for the current window target.
    fn get_clip_rect(&self) -> Option<Rect> {
        self.canvas().clip_rect().map(|r| r.into())
    }

    /// Set the clipping rectangle for the current window target.
    fn clip_rect<R: Into<Option<Rect>>>(&mut self, rect: R) {
        // First convert to Rect, then to rect::Rect
        let rect = rect.into().map(|r| r.into());
        self.canvas_mut().set_clip_rect(rect);
    }

    /// Get the viewport rectangle for the current window target.
    fn get_viewport(&self) -> Rect {
        self.canvas().viewport().into()
    }

    /// Set the viewport rectangle for the current window target.
    fn viewport<R: Into<Option<Rect>>>(&mut self, rect: R) {
        // First convert to Rect, then to rect::Rect
        let rect = rect.into().map(|r| r.into());
        self.canvas_mut().set_viewport(rect);
    }

    /// Draw a point on the current window target.
    fn draw_point<P: Into<Point>>(&mut self, point: P) -> Result<()> {
        let point: rect::Point = point.into().into();
        Ok(self.canvas_mut().draw_point(point)?)
    }

    /// Draw multiple points on the current window target.
    fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Draw a line on the current window target.
    fn draw_line<P1: Into<Point>, P2: Into<Point>>(&mut self, start: P1, end: P2) -> Result<()> {
        if let Some(c) = self.stroke {
            let start: rect::Point = start.into().into();
            let end: rect::Point = end.into().into();
            let canvas = self.canvas_mut();
            canvas.set_draw_color(c);
            canvas.draw_line(start, end)?;
        }
        Ok(())
    }

    /// Draw a series of lines on the current window target.
    fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Draw a rectangle on the current window target.
    fn draw_rect<R: Into<Rect>>(&mut self, rect: R) -> Result<()> {
        if let Some(c) = self.stroke {
            let rect: rect::Rect = rect.into().into();
            let canvas = self.canvas_mut();
            canvas.set_draw_color(c);
            canvas.draw_rect(rect)?;
        }
        Ok(())
    }

    /// Draw multiple rectangles on the current window target.
    fn draw_rects<'a, R: Into<&'a [Rect]>>(&mut self, rects: R) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Draw a filled rectangle on the current window target. Passing None will fill the entire
    /// rendering target.
    fn fill_rect<R: Into<Option<Rect>>>(&mut self, rect: R) -> Result<()> {
        if let Some(c) = self.fill {
            let rect: Option<rect::Rect> = rect.into().map(|r| r.into());
            let canvas = self.canvas_mut();
            canvas.set_draw_color(c);
            canvas.fill_rect(rect)?;
        }
        Ok(())
    }

    /// Draw multiple filled rectangles on the current window target.
    fn fill_rects<'a, R: Into<&'a [Rect]>>(&mut self, rects: R) -> Result<()> {
        // TODO
        Ok(())
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
        color.rgb().into()
    }
}

impl Into<pixels::Color> for Color {
    fn into(self) -> pixels::Color {
        self.rgb().into()
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
