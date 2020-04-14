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

pub const DEFAULT_SAMPLE_RATE: i32 = 44_100; // in Hz

pub(crate) struct Sdl2Renderer {
    context: Sdl,
    window_target: Vec<u32>,
    canvases: Vec<Canvas<Window>>,
    audio_device: AudioQueue<f32>,
    event_pump: EventPump,
    controller_sub: GameControllerSubsystem,
    controllers: Vec<GameController>,
}

impl Sdl2Renderer {
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

        let default_window_id = canvases[0].window().id();
        Ok(Self {
            context,
            window_target: vec![default_window_id],
            canvases,
            audio_device,
            event_pump,
            controller_sub,
            controllers: Vec::new(),
        })
    }

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

    fn get_canvas(&self) -> &Canvas<Window> {
        let window_target = self.current_window_target();
        self.canvases
            .iter()
            .find(|c| window_target == c.window().id())
            .expect("valid window_target")
    }

    fn get_canvas_mut(&mut self) -> &mut Canvas<Window> {
        let window_target = self.current_window_target();
        self.canvases
            .iter_mut()
            .find(|c| window_target == c.window().id())
            .expect("valid window_target")
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

    /// Set a new window target.
    ///
    /// Errors if the window_id is not a valid window_id.
    fn push_window_target(&mut self, window_id: u32) -> Result<()> {
        if window_id == self.current_window_target() {
            Ok(())
        } else if self.canvases.iter().any(|c| window_id == c.window().id()) {
            self.window_target.push(window_id);
            Ok(())
        } else {
            Err(Error::InvalidWindowTarget(window_id))
        }
    }

    /// Removes the current window target and switches it to the previous
    /// current window target.
    ///
    /// Will not remove the last window target (the one created upon engine creation).
    fn pop_window_target(&mut self) -> Option<u32> {
        self.window_target.pop()
    }

    /// Returns the window_id of the current window target
    fn current_window_target(&self) -> u32 {
        self.window_target
            .last()
            .copied()
            .or_else(|| self.canvases.last().map(|c| c.window().id()))
            .expect("should have at least one window")
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

    /// Close the current window target.
    fn close_window(&mut self) -> bool {
        let window_target = *self.window_target.last().unwrap();
        self.canvases.retain(|c| window_target != c.window().id());
        self.window_target.retain(|id| window_target != *id);
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
