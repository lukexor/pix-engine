use crate::{
    common::Result,
    event::{PixEvent, WindowEvent},
    renderer::Renderer,
    state::State,
    time,
};

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
    pub fn new(title: &str, app: A, width: u32, height: u32) -> Result<Self> {
        let state = State::new(title, width, height)?;
        Ok(Self {
            app,
            should_close: false,
            state,
        })
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
                                self.state.set_window_target(None)?;
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
                    let primary = self.state.primary_window();
                    if frame_timer >= one_second && self.state.set_window_target(primary).is_ok() {
                        self.state.set_frame_rate(frame_count);
                        frame_timer -= one_second;
                        let mut title = self.state.get_window().title().to_owned();
                        title.push_str(&format!("- FPS: {}", frame_count));
                        self.state.renderer.set_title(&title)?;
                        frame_count = 0;
                        self.state.set_window_target(None)?;
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
