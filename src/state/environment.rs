use super::State;
use crate::gui::CursorType;

const DEFAULT_TARGET_FRAME_RATE: u32 = 60;

pub(crate) struct StateEnvironment {
    pub(super) focused: bool,
    pub(super) frame_rate: u32,
    pub(super) target_frame_rate: u32,
    pub(super) delta_time: f32,
    pub(super) scale: u32,
    pub(super) display_width: u32,
    pub(super) display_height: u32,
    pub(super) display_density: u32,
    pub(super) windows: Vec<u32>,
    pub(super) cursor: Option<CursorType>,
}

impl StateEnvironment {
    pub(crate) fn new() -> Self {
        let (width, height, density) = Self::determine_display_dimensions();
        Self {
            focused: false,
            frame_rate: 0,
            target_frame_rate: Self::default_target_frame_rate(),
            delta_time: 0.0,
            scale: 1,
            display_width: width,
            display_height: height,
            display_density: density,
            windows: Vec::new(),
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
