use crate::{
    common::PixEngineResult,
    event::{PixEvent, WindowEvent},
    renderer::Renderer,
    state::State,
    time,
};

/// Defines operations that the engine will call on the enclosed app
pub trait PixApp {
    /// Called once upon engine start (when run() is called).
    ///
    /// Return true to continue running
    /// Reeturn false to shutdown the engine and close the application
    fn on_start(&mut self, _state: &mut State) -> PixEngineResult<bool> {
        Ok(true)
    }
    /// Called once when the engine detects a close/exit event.
    ///
    /// Return true to continue exiting
    /// Return false to keep running
    fn on_stop(&mut self, _state: &mut State) -> PixEngineResult<bool> {
        Ok(true)
    }
    /// Called every frame based on the target_frame_rate
    ///
    /// Return true to continue running
    /// Return false to shutdown the engine and close the application
    fn on_update(&mut self, _state: &mut State) -> PixEngineResult<bool> {
        Ok(true)
    }
}

/// Primary PixEngine object that controls update loop and engine state
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
    /// Create a new PixEngine instance, consuming the app in the process
    pub fn new(title: &str, app: A, width: u32, height: u32) -> PixEngineResult<Self> {
        let state = State::new(title, width, height)?;
        Ok(Self {
            app,
            should_close: false,
            state,
        })
    }

    pub fn run(&mut self) -> PixEngineResult<()> {
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
        let mut frame_timer = 0.0;
        let mut frame_count = 0;
        while !self.should_close {
            // Extra loop allows on_stop to prevent closing
            while !self.should_close {
                self.state.renderer.clear_all();

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
                            if self.state.renderer.push_window_target(window_id).is_ok() {
                                self.should_close = self.state.renderer.close_window();
                            }
                        }
                        _ => (),
                    }
                    self.state.events.push(event);
                }

                if self.should_close {
                    break;
                }

                // Update app
                match self.app.on_update(&mut self.state) {
                    Ok(false) => self.should_close = true,
                    Err(e) => return Err(e),
                    _ => (), // continue on
                }

                self.state.renderer.present_all();

                if self.state.show_frame_rate() {
                    frame_timer += self.state.delta_time();
                    frame_count += 1;
                    if frame_timer >= one_second {
                        frame_timer -= one_second;
                        let mut title = self.state.title.to_owned();
                        title.push_str(&format!("- FPS: {}", frame_count));
                        self.state.renderer.set_title(&title)?;
                        frame_count = 0;
                    }
                } else {
                    self.state.renderer.set_title(&self.state.title)?;
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
