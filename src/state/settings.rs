//! Settings for the current [PixEngine] [PixState].
//!
//! [PixEngine]: crate::prelude::PixEngine

use crate::{prelude::*, renderer::*};
use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::time::Duration;

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

/// Drawing mode which changes how `(x, y)` coordinates are interpreted when drawing [Rect]s.
pub type RectMode = DrawMode;

/// Drawing mode which changes how `(x, y)` coordinates are interpreted when drawing [Ellipse]s.
pub type EllipseMode = DrawMode;

/// Drawing mode which changes how `(x, y)` coordinates are interpreted when drawing [Image]s.
pub type ImageMode = DrawMode;

/// Drawing mode which changes how arcs are drawn.
#[non_exhaustive]
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
    #[derive(Default)]
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
        const UNDERLINE = 0x04;
        /// Strike-through
        const STRIKETHROUGH = 0x08;
    }
}

/// Several settings used to change various functionality of the engine.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Settings {
    pub(crate) background: Color,
    pub(crate) fill: Option<Color>,
    pub(crate) stroke: Option<Color>,
    pub(crate) stroke_weight: u8,
    pub(crate) wrap_width: Option<u32>,
    pub(crate) clip: Option<Rect<i32>>,
    pub(crate) running: bool,
    pub(crate) show_frame_rate: bool,
    pub(crate) target_frame_rate: Option<usize>,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) rect_mode: RectMode,
    pub(crate) ellipse_mode: EllipseMode,
    pub(crate) image_mode: ImageMode,
    pub(crate) image_tint: Option<Color>,
    pub(crate) arc_mode: ArcMode,
    pub(crate) angle_mode: AngleMode,
    pub(crate) blend_mode: BlendMode,
    pub(crate) cursor: Option<Cursor>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: BLACK,
            fill: Some(WHITE),
            stroke: None,
            stroke_weight: 1,
            wrap_width: None,
            clip: None,
            running: true,
            show_frame_rate: false,
            target_frame_rate: None,
            scale_x: 1.0,
            scale_y: 1.0,
            rect_mode: RectMode::Corner,
            ellipse_mode: EllipseMode::Corner,
            image_mode: ImageMode::Corner,
            image_tint: None,
            arc_mode: ArcMode::Default,
            angle_mode: AngleMode::Radians,
            blend_mode: BlendMode::None,
            cursor: Some(Cursor::default()),
        }
    }
}

impl PixState {
    /// Sets the [Color] value used to clear the canvas.
    pub fn background<C>(&mut self, color: C) -> PixResult<()>
    where
        C: Into<Color>,
    {
        self.settings.background = color.into();
        self.clear()
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

    /// Sets the width used to draw lines on the canvas.
    pub fn stroke_weight(&mut self, weight: u8) {
        self.settings.stroke_weight = weight;
    }

    /// Disables outlining shapes drawn on the canvas.
    pub fn no_stroke(&mut self) {
        self.settings.stroke = None;
    }

    /// Sets the wrap width used to draw text on the canvas.
    pub fn wrap_width(&mut self, width: u32) {
        self.settings.wrap_width = Some(width);
    }

    /// Disable wrapping when drawing text on the canvas.
    pub fn no_wrap(&mut self) {
        self.settings.wrap_width = None;
    }

    /// Sets the clip [Rect] used by the renderer to draw to the current canvas.
    pub fn clip<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.settings.clip = Some(rect.into());
        self.renderer.clip(self.settings.clip)
    }

    /// Clears the clip [Rect] used by the renderer to draw to the current canvas.
    pub fn no_clip(&mut self) -> PixResult<()> {
        self.settings.clip = None;
        self.renderer.clip(None)
    }

    /// Returns whether the application is fullscreen or not.
    pub fn fullscreen(&mut self) -> PixResult<bool> {
        self.renderer.fullscreen()
    }

    /// Set the application to fullscreen or not.
    pub fn set_fullscreen(&mut self, val: bool) -> PixResult<()> {
        self.renderer.set_fullscreen(val)
    }

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    pub fn vsync(&mut self) -> bool {
        self.renderer.vsync()
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    pub fn set_vsync(&mut self, val: bool) -> PixResult<()> {
        self.renderer.set_vsync(val)
    }

    /// Set the mouse cursor to a predefined symbol or image.
    pub fn cursor(&mut self, cursor: Cursor) -> PixResult<()> {
        self.settings.cursor = Some(cursor);
        self.renderer.cursor(self.settings.cursor.as_ref())
    }

    /// Hide the mouse cursor.
    pub fn no_cursor(&mut self) {
        self.settings.cursor = None;
        // SAFETY: Setting to NONE to hide cursor can't error.
        self.renderer.cursor(None).expect("hiding cursor");
    }

    /// Whether the render loop is running or not.
    pub fn running(&mut self) -> bool {
        self.settings.running
    }

    /// Unpause the render loop.
    pub fn run(&mut self) {
        self.settings.running = true;
    }

    /// Pause the render loop by no longer calling [AppState::on_update] every frame.
    ///
    /// [AppState::on_update]: crate::prelude::AppState::on_update
    pub fn no_run(&mut self) {
        self.settings.running = false;
    }

    /// Set whether to show the current frame rate per second in the title or not.
    pub fn show_frame_rate(&mut self, show: bool) {
        self.settings.show_frame_rate = show;
    }

    /// Get the target frame rate to render at.
    #[inline]
    pub fn target_frame_rate(&mut self) -> Option<usize> {
        self.settings.target_frame_rate
    }

    /// Set a target frame rate to render at, controls how often
    /// [on_update](crate::prelude::AppState::on_update) is called.
    pub fn set_frame_rate(&mut self, rate: usize) {
        self.settings.target_frame_rate = Some(rate);
    }

    /// Remove target frame rate and call [on_update](crate::prelude::AppState::on_update) as often
    /// as possible.
    pub fn clear_frame_rate(&mut self) {
        self.settings.target_frame_rate = None;
    }

    /// Set the rendering scale of the current canvas.
    pub fn scale(&mut self, x: f32, y: f32) -> PixResult<()> {
        let mut s = &mut self.settings;
        s.scale_x = x;
        s.scale_y = y;
        self.renderer.scale(s.scale_x, s.scale_y)
    }

    /// Change the way parameters are interpreted for drawing [Square](Rect)s and
    /// [Rectangle](Rect)s.
    pub fn rect_mode(&mut self, mode: RectMode) {
        self.settings.rect_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Ellipse]s.
    ///
    /// [Ellipse]: crate::prelude::Ellipse
    pub fn ellipse_mode(&mut self, mode: EllipseMode) {
        self.settings.ellipse_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Image]s.
    ///
    /// [Image]: crate::prelude::Image
    pub fn image_mode(&mut self, mode: ImageMode) {
        self.settings.image_mode = mode;
    }

    /// Add a color tint to [Image]s when drawing.
    ///
    /// [Image]: crate::prelude::Image
    pub fn image_tint<C>(&mut self, tint: C)
    where
        C: Into<Option<Color>>,
    {
        self.settings.image_tint = tint.into();
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
        self.setting_stack
            .push((self.settings.clone(), self.theme.clone()));
    }

    /// Restores the previous draw settings and transforms, if present. If the settings stack is
    /// empty, the settings will remain unchanged.
    pub fn pop(&mut self) {
        if let Some((settings, theme)) = self.setting_stack.pop() {
            self.settings = settings;
            self.theme = theme;
        }
        let s = &self.settings;
        let t = &self.theme;
        // SAFETY: All of these settings should be valid since they were set prior to `pop()` being
        // called.
        self.renderer.clip(s.clip).expect("valid clip setting");
        // Excluding restoring cursor - as it's used for mouse hover.
        self.renderer
            .font_size(t.font_sizes.body)
            .expect("valid font size");
        self.renderer.font_style(t.font_styles.body);
        self.renderer
            .font_family(&t.fonts.body)
            .expect("valid font family");
        self.renderer.blend_mode(s.blend_mode);
    }
}

impl PixState {
    /// Set the mouse cursor to a predefined symbol or image for a single frame.
    ///
    /// Cursor will get reset to the current setting next frame.
    #[inline]
    pub(crate) fn frame_cursor(&mut self, cursor: Cursor) -> PixResult<()> {
        self.renderer.cursor(Some(&cursor))
    }

    /// Get the target delta time between frames.
    #[inline]
    pub(crate) fn target_delta_time(&self) -> Duration {
        self.settings
            .target_frame_rate
            .map(|rate| Duration::from_secs(rate as u64))
            .unwrap_or_else(Duration::default)
    }
}
