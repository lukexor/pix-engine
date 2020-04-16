use crate::{
    common::Result,
    event::{PixEvent, WindowEvent},
    renderer::Renderer,
    state::{window::WindowPos, State},
    time,
};

/// Default audio sampling rate in Hertz
pub const DEFAULT_SAMPLE_RATE: i32 = 44_100;

/// Defines operations that the engine will call on the enclosed app.
pub trait PixApp {
    /// Called once upon engine start (when `PixEngine::run()` is called).
    ///
    /// Return true to continue running.
    /// Reeturn false to shutdown the engine and close the application.
    fn on_start(&mut self, _state: &mut State) -> Result<bool> {
        Ok(true)
    }
    /// Called once when the engine detects a close/exit event.
    ///
    /// Return true to continue exiting.
    /// Return false to keep running.
    fn on_stop(&mut self, _state: &mut State) -> Result<bool> {
        Ok(true)
    }
    /// Called every frame based on the target_frame_rate. By default this is as often as possible.
    ///
    /// Return true to continue running.
    /// Return false to shutdown the engine and close the application.
    fn on_update(&mut self, _state: &mut State) -> Result<bool> {
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub struct PixEngineBuilder<A>
where
    A: PixApp,
{
    title: String,
    app: A,
    width: u32,
    height: u32,
    scale: u32,
    x: WindowPos,
    y: WindowPos,
    audio_sample_rate: i32,
    fullscreen: bool,
    no_vsync: bool,
    hidden: bool,
    borderless: bool,
    resizable: bool,
    minimized: bool,
    maximized: bool,
    input_grabbed: bool,
}

impl<A> PixEngineBuilder<A>
where
    A: PixApp,
{
    /// Initializes a new `PixEngineBuilder`.
    pub fn new(title: &str, app: A, width: u32, height: u32) -> Self {
        Self {
            title: title.to_owned(),
            app,
            width,
            height,
            scale: 1,
            x: WindowPos::default(),
            y: WindowPos::default(),
            audio_sample_rate: DEFAULT_SAMPLE_RATE,
            fullscreen: false,
            no_vsync: false,
            hidden: false,
            borderless: false,
            resizable: false,
            minimized: false,
            maximized: false,
            input_grabbed: false,
        }
    }

    /// Sets the window position.
    pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
        self.x = WindowPos::Positioned(x);
        self.y = WindowPos::Positioned(y);
        self
    }

    /// Centers the window.
    pub fn position_centered(&mut self) -> &mut Self {
        self.x = WindowPos::Centered;
        self.y = WindowPos::Centered;
        self
    }

    /// Sets the audio sample rate in Hz
    pub fn audio_sample_rate(&mut self, rate: i32) -> &mut Self {
        self.audio_sample_rate = rate;
        self
    }

    /// Sets the window to fullscreen.
    pub fn fullscreen(&mut self) -> &mut Self {
        self.fullscreen = true;
        self
    }

    /// Disables vsync.
    pub fn no_vsync(&mut self) -> &mut Self {
        self.no_vsync = true;
        self
    }

    /// Hides the window.
    pub fn hidden(&mut self) -> &mut Self {
        self.hidden = true;
        self
    }

    /// Removes the window decoration.
    pub fn borderless(&mut self) -> &mut Self {
        self.borderless = true;
        self
    }

    /// Sets the window to be resizable.
    pub fn resizable(&mut self) -> &mut Self {
        self.resizable = true;
        self
    }

    /// Minimizes the window.
    pub fn minimized(&mut self) -> &mut Self {
        self.minimized = true;
        self
    }

    /// Maximizes the window.
    pub fn maximized(&mut self) -> &mut Self {
        self.maximized = true;
        self
    }

    /// Sets the window to have grabbed input focus.
    pub fn input_grabbed(&mut self) -> &mut Self {
        self.input_grabbed = true;
        self
    }

    pub fn build(self) -> Result<PixEngine<A>> {
        let mut state = State::new()?;
        let id = state.create_window(&self.title, self.width, self.height)?;
        state.set_window_target(id)?;
        Ok(PixEngine {
            app: self.app,
            should_close: false,
            state,
        })
    }
}

/// Primary PixEngine object that controls update loop and engine state.
pub struct PixEngine<A>
where
    A: PixApp,
{
    app: A,
    should_close: bool,
    state: State,
}

impl<A> PixEngine<A>
where
    A: PixApp,
{
    /// Create a new PixEngine instance, consuming the app in the process.
    pub fn create(title: &str, app: A, width: u32, height: u32) -> PixEngineBuilder<A> {
        PixEngineBuilder::new(title, app, width, height)
    }

    /// Start the engine loop. This will only exit and return if an error is encountered, the app
    /// returns false in any of the App trait methods, or all open windows receive close events.
    ///
    /// Errors if the renderer or the app returns an error.
    pub fn run(&mut self) -> Result<()> {
        // Pump event queue once before starting to initialize default window
        let _ = self.state.renderer.poll_events();

        // Start app
        match self.app.on_start(&mut self.state) {
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
            while !self.should_close {
                self.state.clear_all();

                let now = time::now();
                self.state.set_delta_time(time::sub(now, last_frame_time));
                last_frame_time = now;

                self.state.events.clear();
                for event in self.state.renderer.poll_events() {
                    match event {
                        PixEvent::Quit { .. } | PixEvent::AppTerminating { .. } => {
                            self.should_close = true
                        }
                        PixEvent::Window {
                            window_id,
                            win_event: WindowEvent::Close,
                            ..
                        } => {
                            if self.state.set_window_target(window_id).is_ok() {
                                self.state.hide_window();
                                self.state.revert_window_target();
                            }
                        }
                        _ => (),
                    }
                    self.state.events.push(event);
                }

                // Update app
                if self.state.should_loop || self.state.manual_update > 0 {
                    if self.state.manual_update > 0 {
                        self.state.manual_update -= 1;
                    }
                    match self.app.on_update(&mut self.state) {
                        Ok(false) => self.should_close = true,
                        Err(e) => return Err(e),
                        _ => (), // continue on
                    }
                }

                self.state.present_all();

                if self.state.show_frame_rate() {
                    frame_timer += self.state.delta_time();
                    frame_count += 1;
                    self.state.inc_frame_count();
                    if let Some(primary) = self.state.primary_window() {
                        if frame_timer >= one_second
                            && self.state.set_window_target(primary).is_ok()
                        {
                            self.state.set_frame_rate(frame_count);
                            if let Some(w) = self.state.get_window() {
                                let mut title = w.title().to_owned();
                                title.push_str(&format!("- FPS: {}", frame_count));
                                self.state.renderer.set_title(&title)?;
                            };
                            self.state.revert_window_target();
                            frame_timer -= one_second;
                            frame_count = 0;
                        }
                    }
                }
            }

            match self.app.on_stop(&mut self.state) {
                Ok(false) => self.should_close = false,
                Err(e) => return Err(e),
                _ => (), // continue on
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_engine_test() {
        let eng = PixEngine::new("Test", app, 100, 100);
    }

    #[test]
    fn run_engine_test() {}
}
