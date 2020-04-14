use super::State;
use crate::gui::CursorType;

const DEFAULT_TARGET_FRAME_RATE: u32 = 60;

pub(crate) struct StateEnvironment {
    /// Number of frames since start of execution
    frame_count: u32,
    /// Current frame rate per second
    frame_rate: u32,
    /// Target frame rate per second
    target_frame_rate: u32,
    /// Time since last frame
    delta_time: f32,
    /// Scale factor
    scale: u32,
    /// Width of the display screen
    display_width: u32,
    /// Height of the display screen
    display_height: u32,
    /// Pixel density of the display screen for high resolution displays
    display_density: u32,
    /// Current system cursor type
    cursor: Option<CursorType>,
}

impl StateEnvironment {
    pub(crate) fn new() -> Self {
        let (width, height, density) = Self::determine_display_dimensions();
        Self {
            frame_count: 0,
            frame_rate: 0,
            target_frame_rate: Self::default_target_frame_rate(),
            delta_time: 0.0,
            scale: 1,
            display_width: width,
            display_height: height,
            display_density: density,
            cursor: Some(CursorType::Arrow),
        }
    }

    fn default_target_frame_rate() -> u32 {
        // TODO - Determine monitor refresh rate
        DEFAULT_TARGET_FRAME_RATE
    }

    fn determine_display_dimensions() -> (u32, u32, u32) {
        // TODO - Determine monitor display dimensions and pixel density
        (800, 600, 1)
    }
}

impl State {
    /// Returns whether the application has focus in any of it's windows
    pub fn focused(&self) -> bool {
        false
    }

    /// Returns the delta time in seconds since the last frame update
    pub fn delta_time(&mut self) -> f32 {
        self.environment.delta_time
    }

    // Used by the core update loop to set the delta time
    pub(crate) fn set_delta_time(&mut self, time: f32) {
        self.environment.delta_time = time;
    }
}

impl Default for StateEnvironment {
    fn default() -> Self {
        Self::new()
    }
}
