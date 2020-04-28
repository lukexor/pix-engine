use crate::{
    common::Result,
    event::{PixEvent, WindowEvent},
    renderer::Renderer,
    state_data::StateData,
    time,
};

pub mod prelude {
    pub use super::{PixEngine, State};
}

/// Default audio sampling rate in Hertz
pub const DEFAULT_SAMPLE_RATE: i32 = 44_100;

/// Defines operations that the `PixEngine` can call on the enclosed application.
pub trait State {
    /// Called once upon engine start (when `PixEngine::run()` is called).
    ///
    /// Return true to continue running.
    /// Reeturn false to shutdown the engine and close the application.
    fn on_start(&mut self, _s: &mut StateData) -> Result<bool> {
        Ok(true)
    }

    /// Called every frame based on the target_frame_rate. By default this is as often as possible.
    ///
    /// Return true to continue running.
    /// Return false to shutdown the engine and close the application.
    fn on_update(&mut self, _s: &mut StateData) -> Result<bool> {
        Ok(true)
    }

    /// Called once when the engine detects a close/exit event.
    ///
    /// Return true to continue exiting.
    /// Return false to keep running.
    fn on_stop(&mut self, _s: &mut StateData) -> Result<bool> {
        Ok(true)
    }

    /// Called every time the mouse button is pressed.
    fn on_mouse_pressed(&mut self, _s: &mut StateData) {}

    /// Called every time the mouse button is released.
    fn on_mouse_released(&mut self, _s: &mut StateData) {}

    /// Called every time the mouse is moved while a mouse button is pressed.
    fn on_mouse_dragged(&mut self, _s: &mut StateData) {}
}

/// Builds an instance of the `PixEngine` by allowing various settings to be defined before
/// creation.
#[derive(Clone)]
pub struct PixEngineBuilder<S>
where
    S: State,
{
    title: String,
    width: u32,
    height: u32,
    state: S,
    audio_sample_rate: i32,
}

impl<S> PixEngineBuilder<S>
where
    S: State,
{
    /// Initializes a new `PixEngineBuilder`.
    pub fn new(title: &str, state: S, width: u32, height: u32) -> Self {
        Self {
            title: title.to_owned(),
            width,
            height,
            state,
            audio_sample_rate: DEFAULT_SAMPLE_RATE,
        }
    }

    /// Sets the audio sample rate in Hz
    pub fn audio_sample_rate(&mut self, rate: i32) -> &mut Self {
        self.audio_sample_rate = rate;
        self
    }

    /// Builds a `PixEngine` instance using the settings from the `PixEngineBuilder` consuming it
    /// in the process.
    pub fn build(self) -> Result<PixEngine<S>> {
        let mut data = StateData::new(&self.title, self.width, self.height)?;
        data.audio_sample_rate(self.audio_sample_rate)?;
        Ok(PixEngine::new(self.state, data))
    }
}

/// Primary PixEngine object that controls update loop and engine state.
pub struct PixEngine<S>
where
    S: State,
{
    state: S,
    data: StateData,
    should_close: bool,
}

impl<S> PixEngine<S>
where
    S: State,
{
    /// Create a new `PixEngine` instance via a `PixEngineBuilder`, consuming the state in the
    /// process.
    pub fn create(title: &str, state: S, width: u32, height: u32) -> PixEngineBuilder<S> {
        PixEngineBuilder::new(title, state, width, height)
    }

    /// Used by `PixEngineBuilder` to construct a new `PixEngine`.
    pub fn new(state: S, data: StateData) -> Self {
        Self {
            state,
            should_close: false,
            data,
        }
    }

    /// Start the engine loop. This will only exit and return if an error is encountered, the state
    /// returns false in any of the `State` trait methods, or all open windows receive close events.
    ///
    /// Errors if the renderer or the state returns an error.
    pub fn run(&mut self) -> Result<()> {
        // Clear and present once on start
        self.data.background(0);
        self.data.clear_all();
        self.data.present_all();

        // Pump event queue once before starting to initialize default window
        let _ = self.data.renderer.poll_events();

        match self.state.on_start(&mut self.data) {
            Ok(false) => return Ok(()),
            Err(e) => return Err(e),
            _ => (), // continue on
        }

        let mut last_frame_time = time::now();
        let one_second = 1.0;
        let mut frame_timer = 1.0; // Start at 1.0 to update title on first frame
        let mut frame_count = 0;
        while !self.should_close {
            // Extra loop allows on_stop to prevent closing
            'main: while !self.should_close {
                let now = time::now();
                self.data.set_delta_time(time::sub(now, last_frame_time));
                last_frame_time = now;

                self.data.events.clear();
                for event in self.data.renderer.poll_events() {
                    match event {
                        PixEvent::Quit { .. }
                        | PixEvent::AppTerminating { .. }
                        | PixEvent::Window {
                            win_event: WindowEvent::Close,
                            ..
                        } => {
                            self.should_close = true;
                            break 'main;
                        }
                        PixEvent::MouseMotion { x, y, .. } => {
                            self.data.pmouse_pos = self.data.mouse_pos;
                            self.data.mouse_pos = (x, y).into();
                            if self.data.mouse_is_pressed {
                                self.state.on_mouse_dragged(&mut self.data);
                            }
                        }
                        PixEvent::MouseButtonDown { mouse_btn, .. } => {
                            self.data.mouse_is_pressed = true;
                            self.data.mouse_buttons.insert(mouse_btn);
                            self.state.on_mouse_pressed(&mut self.data);
                        }
                        PixEvent::MouseButtonUp { mouse_btn, .. } => {
                            self.data.mouse_is_pressed = false;
                            self.data.mouse_buttons.remove(&mouse_btn);
                            self.state.on_mouse_released(&mut self.data);
                        }
                        _ => (),
                    }
                    self.data.events.push(event);
                }

                // Update app
                if self.data.should_loop || self.data.manual_update > 0 {
                    if self.data.manual_update > 0 {
                        self.data.manual_update -= 1;
                    }
                    self.should_close = !self.state.on_update(&mut self.data)?;

                    self.data.present_all();

                    if self.data.get_show_frame_rate() {
                        frame_timer += self.data.delta_time();
                        frame_count += 1;
                        self.data.inc_frame_count();
                        if frame_timer >= one_second {
                            self.data.set_frame_rate(frame_count);
                            let mut title = self.data.title.to_owned();
                            title.push_str(&format!(" - FPS: {}", frame_count));
                            self.data.renderer.title(&title)?;
                            frame_timer -= one_second;
                            frame_count = 0;
                        }
                    }
                }
            }

            self.should_close = self.state.on_stop(&mut self.data)?;
        }

        Ok(())
    }
}
