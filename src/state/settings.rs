//! Settings for the current engine `PixState`.

use super::PixState;
use crate::{
    color::{constants::*, Color},
    renderer::{self, Rendering},
    shape::Rect,
};
use std::path::Path;

/// Drawing mode which changes how (x, y) coordinates are interpreted.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DrawMode {
    /// Use (x, y) as the top-left corner. Default.
    Corner,
    /// Use (x, y) as the center.
    Center,
}

/// Drawing mode which changes how arcs are drawn.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArcMode {
    /// Draws arc with fill as an open pie segment.
    None,
    /// Draws arc with fill as an closed pie segment.
    Pie,
    /// Draws arc with fill as an open semi-circle.
    Open,
    /// Draws arc with fill as a closed semi-circle.
    Chord,
}

/// Drawing mode which changes how textures are blended together.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// Disable blending
    None,
    /// Alpha blending
    Blend,
    /// Additive blending
    Add,
    /// Color modulate
    Mod,
}

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
    pub(crate) blend_mode: BlendMode,
    // TODO: arc_mode: default ArcMode::Pie
    // TODO: stroke_weight: u32, default 1
    // TODO: stroke_cap: StrokeCap, Default StrokeCap::Round
    // TODO: stroke_join: StrokeJoin, StrokeJoin::Miter
    // TODO: angle_mode: AngleMode, Default AngleMode::Radians
    // TODO: image_tint: Option<Color>, Default None
    // TODO: image_mode: DrawMode, default DrawMode::Corner
    // TODO: text_align_hori: TextAlignHori, Default TextAlignHori::Left
    // TODO: text_align_vert: TextAlignVert, TextAlignVert::Top
    // TODO: text_style: TextStyle, Default TextStyle::Normal
    // TODO: text_font: Font, Default emulogic - Add attribution
    // TODO: blend_factor: f32, Default 1.0
    // TODO: transformation: Option<Transform>, Default None
}

// TODO: TextAlignHori { Left, Center, Right }
// TODO: TextAlignVert { Top, Center, Bottom, Baseline }
// TODO: TextStyle { Normal, Italic, Bold, BoldItalic }
// TODO: Font { Arial, .., Custom(String, PathBuf) }

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
            blend_mode: BlendMode::None,
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
        self.renderer.clip(Some(rect.into()));
    }

    /// Clears the clip rect used by the renderer to draw to the current canvas.
    pub fn no_clip(&mut self) {
        self.renderer.clip(None);
    }

    /// Returns whether the application is fullscreen or not.
    pub fn is_fullscreen(&mut self) -> bool {
        self.renderer.is_fullscreen()
    }

    /// Set the application to fullscreen or not.
    pub fn fullscreen(&mut self, val: bool) {
        self.renderer.fullscreen(val)
    }

    /// Set whether the cursor is shown or not.
    pub fn cursor(&mut self, show: bool) {
        self.renderer.cursor(show);
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

    /// Set the rendering scale of the current canvas.
    pub fn scale(&mut self, x: f32, y: f32) -> renderer::Result<()> {
        self.renderer.scale(x, y)
    }

    /// Set the text size for drawing to the current canvas.
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

    /// Change the way textures are blended together.
    pub fn blend_mode(&mut self, mode: BlendMode) {
        self.settings.blend_mode = mode;
        self.renderer.blend_mode(mode);
    }

    /// Saves the current draw settings and transforms.
    pub fn push(&mut self) {
        todo!("push");
    }

    /// Restores the current draw settings and transforms.
    pub fn pop(&mut self) {
        todo!("pop");
    }
}
