use super::{Error, Renderer, Result};
use crate::{color::Color, event::PixEvent, state::rendering::BlendMode};
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    controller::GameController,
    pixels,
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
    window_target: u32,
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
        let canvases = vec![Self::new_canvas(&context, title, width, height)?];

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

        Ok(Self {
            context,
            window_target: canvases[0].window().id(),
            canvases,
            audio_device,
            event_pump,
            controller_sub,
            controllers: Vec::new(),
        })
    }

    /// Static method to create a new window and associated canvas.
    fn new_canvas(context: &Sdl, title: &str, width: u32, height: u32) -> Result<Canvas<Window>> {
        // Set up the window
        let video_sub = context.video()?;
        let window = video_sub
            .window(title, width, height)
            .position_centered() // TODO make this an option
            .resizable() // TODO make this an option
            .build()?;

        // Set up canvas
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .target_texture()
            .present_vsync() // TODO make this an option
            .build()?;
        canvas.set_logical_size(width, height)?;
        Ok(canvas)
    }

    /// Get a canvas based on the current window target.
    fn get_canvas(&self) -> &Canvas<Window> {
        let target = self.window_target();
        self.canvases
            .iter()
            .find(|c| target == c.window().id())
            .expect("valid window target")
    }

    /// Get a mutable canvas based on the current window target.
    fn get_canvas_mut(&mut self) -> &mut Canvas<Window> {
        let target = self.window_target();
        self.canvases
            .iter_mut()
            .find(|c| target == c.window().id())
            .expect("valid window target")
    }
}

impl Renderer for Sdl2Renderer {
    /// Settings

    /// Set title for the current window target.
    ///
    /// Errors if the title contains a nul byte.
    fn set_title(&mut self, title: &str) -> Result<()> {
        self.get_canvas_mut().window_mut().set_title(title)?;
        Ok(())
    }

    /// Get draw color for the current window target.
    fn draw_color(&self) -> Color {
        self.get_canvas().draw_color().into()
    }

    /// Set draw color for drawing operations on the current window target.
    fn set_draw_color<C: Into<Color>>(&mut self, color: C) {
        self.get_canvas_mut().set_draw_color(color.into())
    }

    /// Get the blending mode for the current window target.
    fn blend_mode(&self) -> BlendMode {
        self.get_canvas().blend_mode().into()
    }

    /// Set the blending mode for drawing operations on the current window target.
    fn set_blend_mode(&mut self, mode: BlendMode) {
        self.get_canvas_mut().set_blend_mode(mode.into());
    }

    /// Returns a list of events from the event queue since last time poll_events
    /// was called.
    fn poll_events(&mut self) -> Vec<PixEvent> {
        self.sdl_poll_events()
    }

    /// Presents changes made to the canvas on the current window target since present was last
    /// called.
    fn present(&mut self) {
        self.get_canvas_mut().present();
    }

    /// Presents changes made to the canvases of all windows since present was last called.
    fn present_all(&mut self) {
        for canvas in self.canvases.iter_mut() {
            canvas.present();
        }
    }

    /// Clears the canvas on the current window target to the current draw color.
    fn clear(&mut self) {
        self.get_canvas_mut().clear();
    }

    /// Clears all canvases of all windows to their current draw colors.
    fn clear_all(&mut self) {
        for canvas in self.canvases.iter_mut() {
            canvas.clear();
        }
    }

    /// Window Management

    /// Returns the id of the default window created on instantiation.
    fn default_window_id(&self) -> u32 {
        self.canvases[0].window().id()
    }

    /// Returns the window_id of the current window target
    fn window_target(&self) -> u32 {
        self.window_target
    }

    /// Set the current window target.
    fn set_window_target<I: Into<Option<u32>>>(&mut self, window_id: I) {
        match window_id.into() {
            Some(id) => {
                if self.canvases.iter().any(|c| id == c.window().id()) {
                    self.window_target = id;
                } else {
                }
            }
            None => self.window_target = self.canvases[0].window().id(),
        }
    }

    /// Create and open a new window.
    ///
    /// Errors if the window can't be created for any reason.
    fn create_window(&mut self, title: &str, width: u32, height: u32) -> Result<u32> {
        let canvas = Self::new_canvas(&self.context, title, width, height)?;
        let window_id = canvas.window().id();
        self.canvases.push(canvas);
        Ok(window_id)
    }

    /// Hide the current window target.
    ///
    /// Returns true when all windows are hidden.
    fn hide_window(&mut self) {
        self.get_canvas_mut().window_mut().hide();
    }

    /// Show the current window target.
    fn show_window(&mut self) {
        self.get_canvas_mut().window_mut().show();
    }

    /// Close the current window target.
    ///
    /// Returns true when all windows are closed.
    fn close_window(&mut self) -> bool {
        let target = self.window_target();
        self.canvases.retain(|c| target != c.window().id());
        self.canvases.is_empty()
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

impl From<Color> for pixels::Color {
    fn from(color: Color) -> Self {
        color.rgb().into()
    }
}
