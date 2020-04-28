use super::StateData;
use crate::gui::CursorType;

const DEFAULT_TARGET_FRAME_RATE: u32 = 60;

#[derive(Default, Debug, Clone)]
pub(crate) struct Environment {
    /// Number of frames since start of execution
    frame_count: u32,
    /// Current frame rate per second
    frame_rate: u32,
    /// Target frame rate per second
    target_frame_rate: u32,
    /// Time since last frame
    delta_time: f64,
    /// Width of the display screen
    display_width: u32,
    /// Height of the display screen
    display_height: u32,
    /// Pixel density of the display screen for high resolution displays
    display_density: u32,
    /// Current system cursor type
    cursor: Option<CursorType>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        let (width, height, density) = Self::determine_display_dimensions();
        Self {
            target_frame_rate: Self::default_target_frame_rate(),
            display_width: width,
            display_height: height,
            display_density: density,
            cursor: Some(CursorType::default()),
            ..Default::default()
        }
    }

    fn default_target_frame_rate() -> u32 {
        // TODO - Determine monitor refresh rate
        // sdl2 window.display_mode().refresh_rate
        DEFAULT_TARGET_FRAME_RATE
    }

    fn determine_display_dimensions() -> (u32, u32, u32) {
        // TODO - Determine monitor display dimensions and pixel density
        // sdl2 window.display_mode().w/h
        (800, 600, 1)
    }
}

impl StateData {
    /// Get the frame count since execution started.
    pub fn frame_count(&self) -> u32 {
        self.environment.frame_count
    }

    /// Get the frame rate per second of the primary window.
    pub fn frame_rate(&self) -> u32 {
        self.environment.frame_rate
    }

    /// Get the target frame rate per second of the primary window.
    pub fn target_frame_rate(&self) -> u32 {
        self.environment.target_frame_rate
    }
    /// Set the target frame rate per second of the primary window.
    pub fn set_target_frame_rate(&mut self, target_frame_rate: u32) {
        self.environment.target_frame_rate = target_frame_rate;
    }

    /// Get the delta time in seconds since the last frame update.
    pub fn delta_time(&self) -> f64 {
        self.environment.delta_time
    }

    /// Get the display width of the primary monitor.
    pub fn display_width(&self) -> u32 {
        self.environment.display_width
    }

    /// Get the display height of the primary monitor.
    pub fn display_height(&self) -> u32 {
        self.environment.display_height
    }

    /// Get the display density of the primary monitor.
    pub fn display_density(&self) -> u32 {
        self.environment.display_density
    }

    /// Get the system cursor type.
    pub fn cursor(&self) -> Option<&CursorType> {
        self.environment.cursor.as_ref()
    }
    /// Get the system cursor type.
    pub fn set_cursor<C: Into<Option<CursorType>>>(&mut self, cursor: C) {
        self.environment.cursor = cursor.into();
    }

    /// Used by the core update loop to increment the frame count since execution started.
    pub(crate) fn inc_frame_count(&mut self) {
        self.environment.frame_count += 1;
    }
    /// Used by the core update loop to set the frame rate per second of the primary window.
    pub(crate) fn set_frame_rate(&mut self, frame_rate: u32) {
        self.environment.frame_rate = frame_rate;
    }
    /// Used by the core update loop to set the delta time
    pub(crate) fn set_delta_time(&mut self, time: f64) {
        self.environment.delta_time = time;
    }
}
