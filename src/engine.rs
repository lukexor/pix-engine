use crate::{
    driver::Driver,
    event::PixEvent,
    state::{State, StateData},
    PixEngineErr, PixEngineResult,
};

/// Primary PixEngine object that controls Window and StateData
pub struct PixEngine<S>
where
    S: State,
{
    app_name: String,
    state: S,
    should_close: bool,
    data: StateData,
}

impl<S> PixEngine<S>
where
    S: State,
{
    /// Create a new PixEngine instance
    pub fn new(
        app_name: String,
        state: S,
        screen_width: u32,
        screen_height: u32,
    ) -> PixEngineResult<Self> {
        let data = StateData::new(&app_name, screen_width, screen_height)?;
        Ok(Self {
            app_name,
            state,
            should_close: false,
            data,
        })
    }
    /// Set a custom window icon
    pub fn set_icon(&mut self, path: &str) -> PixEngineResult<()> {
        self.data.driver.load_icon(path)
    }
    /// Toggle fullscreen
    pub fn fullscreen(&mut self, val: bool) -> PixEngineResult<()> {
        self.data.fullscreen(val)
    }
    /// Toggle vsync
    pub fn vsync(&mut self, val: bool) -> PixEngineResult<()> {
        self.data.vsync(val)
    }
    /// Set audio sample rate
    pub fn set_audio_sample_rate(&mut self, sample_rate: i32) -> PixEngineResult<()> {
        self.data.set_audio_sample_rate(sample_rate)
    }

    /// Starts the engine loop. Will execute until one of on_create, on_update, or on_destroy
    /// returns false or the Window receives a termination event
    pub fn run(&mut self) -> PixEngineResult<()> {
        use std::time::{Duration, Instant};

        if self.data.screen_width() == 0 || self.data.screen_height() == 0 {
            return Err(PixEngineErr::new("invalid screen dimensions"));
        }

        // Create user resources on start up
        match self.state.on_start(&mut self.data) {
            Ok(false) => return Ok(()),
            Err(e) => return Err(e),
            _ => (), // continue on
        }

        // Start main loop
        let main_screen = format!("screen{}", self.data.main_window_id()); // TODO abstract this out
        let one_second = Duration::new(1, 0);
        let zero_seconds = Duration::new(0, 0);
        let mut frame_timer = zero_seconds;
        let mut last_frame_time = Instant::now();
        let epsilon = Duration::from_millis(5);
        while !self.should_close {
            // Extra loop allows on_destroy to prevent closing
            while !self.should_close {
                let now = Instant::now();
                let time_since_last = now - last_frame_time;
                let target_time_between_frames = one_second / 60; // TODO replace with target_frame_rate

                let events: Vec<PixEvent> = self.data.driver.poll()?;
                self.data.events.clear();
                for event in events {
                    self.data.events.push(event);
                    match event {
                        PixEvent::Quit | PixEvent::AppTerminating => self.should_close = true,
                        PixEvent::WinClose(window_id) => {
                            if window_id == self.data.main_window_id() {
                                self.should_close = true;
                            } else {
                                self.data.driver.close_window(window_id);
                            }
                        }
                        PixEvent::KeyPress(key, pressed, ..) => {
                            self.data.set_new_key_state(key, pressed);
                        }
                        PixEvent::MousePress(button, .., pressed) => {
                            // TODO add functionality for mouse click coords
                            self.data.set_new_mouse_state(button, pressed);
                        }
                        PixEvent::MouseMotion(x, y) => self.data.update_mouse(x, y),
                        PixEvent::MouseWheel(delta) => self.data.update_mouse_wheel(delta),
                        PixEvent::Focus(_, focused) => self.data.set_focused(focused),
                        _ => (), // Skip anything else
                    }
                }

                self.data.update_key_states();
                self.data.update_mouse_states();

                // Handle user updates
                match self
                    .state
                    .on_update(time_since_last.as_secs_f32(), &mut self.data)
                {
                    Ok(false) => self.should_close = true,
                    Err(e) => return Err(e),
                    _ => (), // continue on
                }

                // Display updated frame
                if time_since_last >= target_time_between_frames - epsilon {
                    self.data.driver.clear()?;
                    self.data.copy_draw_target(&main_screen)?;
                    self.data.driver.present();

                    let frame_rate =
                        (one_second.as_secs_f32() / time_since_last.as_secs_f32()) as u32;
                    frame_timer = frame_timer
                        .checked_add(time_since_last)
                        .unwrap_or(one_second);
                    last_frame_time = now;
                    if frame_timer >= one_second {
                        frame_timer = frame_timer.checked_sub(one_second).unwrap_or(zero_seconds);
                        let mut title = format!("{} - FPS: {}", self.app_name, frame_rate);
                        if !self.data.title().is_empty() {
                            title.push_str(&format!(" - {}", self.data.title()));
                        }
                        self.data
                            .driver
                            .set_title(self.data.main_window_id(), &title)?;
                    }
                } else {
                    std::thread::sleep(target_time_between_frames - time_since_last - epsilon);
                }
            }

            match self.state.on_stop(&mut self.data) {
                Ok(false) => self.should_close = false,
                Err(e) => return Err(e),
                _ => (), // continue on
            }
        }

        Ok(())
    }
}
