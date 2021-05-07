//! Settings for the current engine State.

use super::State;
use crate::{
    color::{constants::*, Color},
    draw::DrawMode,
    renderer::{RendererResult, Rendering},
    shape::Rect,
};

/// Several settings used to change various functionality of the engine.
#[derive(Debug, Clone)]
pub(crate) struct Settings {
    pub(crate) background: Color,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Color>,
    pub(crate) text_size: u32,
    pub(crate) show_frame_rate: bool,
    pub(crate) rect_mode: DrawMode,
    pub(crate) ellipse_mode: DrawMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Color::Rgb(TRANSPARENT),
            fill: Some(Color::Rgb(BLACK)),
            stroke: None,
            text_size: 16,
            show_frame_rate: false,
            rect_mode: DrawMode::Corner,
            ellipse_mode: DrawMode::Corner,
        }
    }
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

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    pub fn set_clip(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.renderer.set_clip_rect(Some(Rect::new(x, y, w, h)));
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    pub fn no_clip(&mut self) {
        self.renderer.set_clip_rect(None);
    }

    /// Set whether the cursor is shown or not.
    pub fn show_cursor(&mut self, show: bool) {
        self.renderer.show_cursor(show);
    }

    /// Set whether to show the current frame rate per second in the title or not.
    pub fn show_frame_rate(&mut self, show: bool) {
        self.settings.show_frame_rate = show;
    }

    /// Change the rendering scale of the current canvas.
    pub fn set_scale(&mut self, x: f32, y: f32) -> RendererResult<()> {
        self.renderer.set_scale(x, y)
    }

    /// Change the text size for drawing to the current canvas.
    pub fn text_size(&mut self, size: u32) {
        self.settings.text_size = size;
    }

    /// Change the way parameters are interpreted for drawing squares and rectangles.
    pub fn rect_mode(&mut self, mode: DrawMode) {
        self.settings.rect_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing circles and ellipses.
    pub fn ellipse_mode(&mut self, mode: DrawMode) {
        self.settings.ellipse_mode = mode;
    }
}
