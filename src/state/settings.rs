//! Settings for the current engine `PixState`.

use super::PixState;
use crate::{
    color::{constants::*, Color},
    draw::DrawMode,
    renderer::{RendererResult, Rendering},
    shape::Rect,
};
use std::path::Path;

/// Several settings used to change various functionality of the engine.
#[derive(Debug, Clone)]
pub(crate) struct Settings {
    pub(crate) background: Color,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Color>,
    pub(crate) text_size: u32,
    pub(crate) paused: bool,
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
            paused: false,
            show_frame_rate: false,
            rect_mode: DrawMode::Corner,
            ellipse_mode: DrawMode::Corner,
        }
    }
}

impl PixState {
    /// Sets the `Color` value used to clear the canvas.
    pub fn background<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.background = color.into();
    }

    /// Sets the `Color` value used to fill shapes drawn on the canvas.
    pub fn fill<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.fill = Some(color.into());
    }

    /// Disables filling shapes drawn on the canvas.
    pub fn no_fill(&mut self) {
        self.settings.fill = None;
    }

    /// Sets the `Color` value used to outline shapes drawn on the canvas.
    pub fn stroke<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.stroke = Some(color.into());
    }

    /// Disables outlining shapes drawn on the canvas.
    pub fn no_stroke(&mut self) {
        self.settings.stroke = None;
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    pub fn clip<R>(&mut self, rect: R)
    where
        R: Into<Rect>,
    {
        self.renderer.set_clip_rect(Some(rect.into()));
    }

    /// Clears the clip rect used by the renderer to draw to the current canvas.
    pub fn no_clip(&mut self) {
        self.renderer.set_clip_rect(None);
    }

    /// Set the application to fullscreen or not.
    pub fn fullscreen(&mut self, val: bool) {
        self.renderer.set_fullscreen(val)
    }

    /// Set whether the cursor is shown or not.
    pub fn cursor(&mut self, show: bool) {
        self.renderer.show_cursor(show);
    }

    /// Set the cursor icon.
    pub fn cursor_icon<P>(&mut self, _path: P)
    where
        P: AsRef<Path>,
    {
        todo!("cursor_icon");
    }

    /// Whether the game look is paused or not.
    pub fn paused(&mut self) -> bool {
        self.settings.paused
    }

    /// Pause the game loop or not.
    pub fn pause(&mut self, paused: bool) {
        self.settings.paused = paused;
    }

    /// Set whether to show the current frame rate per second in the title or not.
    pub fn show_frame_rate(&mut self, show: bool) {
        self.settings.show_frame_rate = show;
    }

    /// Change the rendering scale of the current canvas.
    pub fn scale(&mut self, x: f32, y: f32) -> RendererResult<()> {
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
