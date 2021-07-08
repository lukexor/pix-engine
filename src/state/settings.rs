//! Settings for the current [PixEngine] [PixState].
//!
//! [PixEngine]: crate::prelude::PixEngine

use crate::{
    prelude::{Color, PixResult, PixState, Rect},
    renderer::Rendering,
    window::Window,
};
use bitflags::bitflags;
use num_traits::AsPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Drawing mode which changes how `(x, y)` coordinates are interpreted.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DrawMode {
    /// Use `(x, y)` as the top-left corner. Default.
    Corner,
    /// Use `(x, y)` as the center.
    Center,
}

/// Drawing mode which changes how arcs are drawn.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArcMode {
    /// Draws arc with fill as an open pie segment.
    Default,
    /// Draws arc with fill as an closed pie segment.
    Pie,
}

/// Drawing mode which changes how textures are blended together.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BlendMode {
    /// Disable blending.
    None,
    /// Alpha blending.
    Blend,
    /// Additive blending.
    Add,
    /// Color modulate.
    Mod,
}

/// Angle mode which changes how math functions interpreted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AngleMode {
    /// Radians.
    Radians,
    /// Degrees.
    Degrees,
}

bitflags! {
    /// Font style for drawing text.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct FontStyle: i32 {
        /// Normal.
        const NORMAL = 0x00;
        /// Bold.
        const BOLD = 0x01;
        /// Italic.
        const ITALIC = 0x02;
        /// Underline
        const UNDERLINE = 0x03;
        /// Strike-through
        const STRIKETHROUGH = 0x04;
    }
}

/// Several settings used to change various functionality of the engine.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Settings {
    pub(crate) background: Color,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Color>,
    pub(crate) running: bool,
    pub(crate) run_count: usize,
    pub(crate) show_frame_rate: bool,
    pub(crate) rect_mode: DrawMode,
    pub(crate) ellipse_mode: DrawMode,
    pub(crate) image_mode: DrawMode,
    pub(crate) image_tint: Option<Color>,
    pub(crate) arc_mode: ArcMode,
    pub(crate) angle_mode: AngleMode,
    pub(crate) blend_mode: BlendMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Color::default(),
            fill: None,
            stroke: None,
            running: true,
            run_count: 0,
            show_frame_rate: false,
            rect_mode: DrawMode::Corner,
            ellipse_mode: DrawMode::Corner,
            image_mode: DrawMode::Corner,
            image_tint: None,
            arc_mode: ArcMode::Default,
            angle_mode: AngleMode::Radians,
            blend_mode: BlendMode::None,
        }
    }
}

impl PixState {
    /// Sets the [Color] value used to clear the canvas.
    pub fn background<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.background = color.into();
    }

    /// Sets the [Color] value used to fill shapes drawn on the canvas.
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

    /// Sets the [Color] value used to outline shapes drawn on the canvas.
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

    /// Sets the clip [Rect] used by the renderer to draw to the current canvas.
    pub fn clip<R>(&mut self, rect: R)
    where
        R: Into<Rect<i32>>,
    {
        self.renderer.clip(Some(rect.into()));
    }

    /// Clears the clip [Rect] used by the renderer to draw to the current canvas.
    pub fn no_clip(&mut self) {
        self.renderer.clip(None);
    }

    /// Returns whether the application is fullscreen or not.
    pub fn fullscreen(&mut self) -> bool {
        self.renderer.fullscreen()
    }

    /// Set the application to fullscreen or not.
    pub fn set_fullscreen(&mut self, val: bool) {
        self.renderer.set_fullscreen(val)
    }

    /// Set whether the cursor is shown or not.
    pub fn cursor(&mut self, show: bool) {
        self.renderer.cursor(show);
    }

    /// Whether the render loop is running or not.
    pub fn running(&mut self) -> bool {
        self.settings.running
    }

    /// Unpause the render loop.
    pub fn run(&mut self) {
        self.settings.running = true;
    }

    /// Pause the render loop.
    pub fn no_run(&mut self) {
        self.settings.running = false;
    }

    /// Run the render loop N times.
    pub fn run_times(&mut self, n: usize) {
        self.settings.run_count = n;
    }

    /// Set whether to show the current frame rate per second in the title or not.
    pub fn show_frame_rate(&mut self, show: bool) {
        self.settings.show_frame_rate = show;
    }

    /// Set the rendering scale of the current canvas.
    pub fn scale<T: AsPrimitive<f32>>(&mut self, x: T, y: T) -> PixResult<()> {
        Ok(self.renderer.scale(x.as_(), y.as_())?)
    }

    /// Set the font size for drawing to the current canvas.
    pub fn font_size(&mut self, size: u32) -> PixResult<()> {
        Ok(self.renderer.font_size(size)?)
    }

    /// Set the font style for drawing to the current canvas.
    pub fn font_style(&mut self, style: FontStyle) {
        self.renderer.font_style(style);
    }

    /// Set the font family for drawing to the current canvas.
    pub fn font_family<S>(&mut self, family: &str) -> PixResult<()> {
        Ok(self.renderer.font_family(family)?)
    }

    /// Change the way parameters are interpreted for drawing [Square](Rect)s and
    /// [Rectangle](Rect)s.
    pub fn rect_mode(&mut self, mode: DrawMode) {
        self.settings.rect_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Circle]s and
    /// [Ellipse]s.
    ///
    /// [Circle]: crate::prelude::Circle
    /// [Ellipse]: crate::prelude::Ellipse
    pub fn ellipse_mode(&mut self, mode: DrawMode) {
        self.settings.ellipse_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Image]s.
    pub fn image_mode(&mut self, mode: DrawMode) {
        self.settings.image_mode = mode;
    }

    /// Add a color tint to [Image]s when drawing.
    pub fn image_tint<C>(&mut self, tint: C)
    where
        C: Into<Option<Color>>,
    {
        let tint = tint.into();
        self.settings.image_tint = tint;
    }

    /// Change the way arcs are drawn.
    pub fn arc_mode(&mut self, mode: ArcMode) {
        self.settings.arc_mode = mode;
    }

    /// Change the way angles are interprted for matrix transformations.
    pub fn angle_mode(&mut self, mode: AngleMode) {
        self.settings.angle_mode = mode;
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
