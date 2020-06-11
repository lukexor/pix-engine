//! Settings for the current engine State.

use super::State;
use crate::{color::Color, common::Result, renderer::Rendering};

/// Several settings used to change various functionality of the engine.
#[derive(Debug, Default, Clone)]
pub(crate) struct Settings {
    pub(crate) background: Color,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Color>,
    pub(crate) show_frame_rate: bool,
}

impl State {
    /// Sets the `Color` value used to clear the canvas.
    pub fn background<C: Into<Color>>(&mut self, color: C) {
        self.settings.background = color.into();
    }

    /// Sets the `Color` value used to fill shapes drawn on the canvas.
    pub fn fill<C: Into<Color>>(&mut self, color: C) {
        self.settings.fill = Some(color.into());
    }

    /// Disables filling shapes drawn on the canvas.
    pub fn no_fill(&mut self) {
        self.settings.fill = None;
    }

    /// Sets the `Color` value used to outline shapes drawn on the canvas.
    pub fn stroke<C: Into<Color>>(&mut self, color: C) {
        self.settings.stroke = Some(color.into());
    }

    /// Disables outlining shapes drawn on the canvas.
    pub fn no_stroke(&mut self) {
        self.settings.stroke = None;
    }

    /// Set whether to show the current frame rate per second in the title or not.
    pub fn show_frame_rate(&mut self, show: bool) {
        self.settings.show_frame_rate = show;
    }

    /// Change the rendering scale of the current canvas.
    pub fn set_scale(&mut self, x: f32, y: f32) -> Result<()> {
        self.renderer.set_scale(x, y)
    }
}
