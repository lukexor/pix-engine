//! Settings for the current [PixEngine] [PixState].
//!
//! [PixEngine]: crate::prelude::PixEngine

use crate::{prelude::*, renderer::*};
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
    pub(crate) cursor: Option<Cursor>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Color::default(),
            fill: Some(WHITE),
            stroke: Some(BLACK),
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

    /// Disables outlining shapes drawn on the canvas.
    pub fn no_stroke(&mut self) {
        self.settings.stroke = None;
    }

    /// Sets the clip [Rect] used by the renderer to draw to the current canvas.
    pub fn clip<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        Ok(self.renderer.clip(Some(rect.into()))?)
    }

    /// Clears the clip [Rect] used by the renderer to draw to the current canvas.
    pub fn no_clip(&mut self) -> PixResult<()> {
        Ok(self.renderer.clip(None)?)
    }

    /// Returns whether the application is fullscreen or not.
    pub fn fullscreen(&mut self) -> bool {
        self.renderer.fullscreen()
    }

    /// Set the application to fullscreen or not.
    pub fn set_fullscreen(&mut self, val: bool) {
        self.renderer.set_fullscreen(val)
    }

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    pub fn vsync(&mut self) -> bool {
        self.renderer.vsync()
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    pub fn set_vsync(&mut self, val: bool) -> PixResult<()> {
        Ok(self.renderer.set_vsync(val)?)
    }

    /// Set the mouse cursor to a predefined symbol or image.
    pub fn cursor(&mut self, cursor: &Cursor) -> PixResult<()> {
        self.settings.cursor = Some(cursor.clone());
        Ok(self.renderer.cursor(Some(cursor))?)
    }

    /// Set the mouse cursor to a predefined symbol or image for a single frame.
    ///
    /// Cursor will get reset to the current setting next frame.
    pub(crate) fn frame_cursor(&mut self, cursor: &Cursor) -> PixResult<()> {
        Ok(self.renderer.cursor(Some(cursor))?)
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

    /// Run the render loop 1 time by calling [AppState::on_update].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [AppState::on_mouse_pressed] or [AppState::on_key_pressed].
    ///
    /// [AppState::on_update]: crate::prelude::AppState::on_update
    /// [AppState::on_mouse_pressed]: crate::prelude::AppState::on_mouse_pressed
    /// [AppState::on_key_pressed]: crate::prelude::AppState::on_key_pressed
    pub fn redraw(&mut self) {
        self.settings.run_count = 1;
    }

    /// Run the render loop N times by calling [AppState::on_update].
    ///
    /// This can be used to only redraw in response to user actions such as
    /// [AppState::on_mouse_pressed] or [AppState::on_key_pressed].
    ///
    /// [AppState::on_update]: crate::prelude::AppState::on_update
    /// [AppState::on_mouse_pressed]: crate::prelude::AppState::on_mouse_pressed
    /// [AppState::on_key_pressed]: crate::prelude::AppState::on_key_pressed
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
    pub fn font_size<S: AsPrimitive<u32>>(&mut self, size: S) -> PixResult<()> {
        Ok(self.renderer.font_size(size.as_())?)
    }

    /// Return the dimensions of given text for drawing to the current canvas.
    pub fn size_of<S: AsRef<str>>(&self, text: S) -> PixResult<(u32, u32)> {
        Ok(self.renderer.size_of(text.as_ref())?)
    }

    /// Set the font style for drawing to the current canvas.
    pub fn font_style(&mut self, style: FontStyle) {
        self.renderer.font_style(style);
    }

    /// Set the font family for drawing to the current canvas.
    pub fn font_family<S: AsRef<str>>(&mut self, family: S) -> PixResult<()> {
        Ok(self.renderer.font_family(family.as_ref())?)
    }

    /// Change the way parameters are interpreted for drawing [Square](Rect)s and
    /// [Rectangle](Rect)s.
    pub fn rect_mode(&mut self, mode: DrawMode) {
        self.settings.rect_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Ellipse]s.
    ///
    /// [Ellipse]: crate::prelude::Ellipse
    pub fn ellipse_mode(&mut self, mode: DrawMode) {
        self.settings.ellipse_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Image]s.
    ///
    /// [Image]: crate::prelude::Image
    pub fn image_mode(&mut self, mode: DrawMode) {
        self.settings.image_mode = mode;
    }

    /// Add a color tint to [Image]s when drawing.
    ///
    /// [Image]: crate::prelude::Image
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
        self.setting_stack.push(self.settings.clone());
    }

    /// Restores the previous draw settings and transforms, if present. If the settings stack is
    /// empty, the settings will remain unchanged.
    pub fn pop(&mut self) {
        if let Some(settings) = self.setting_stack.pop() {
            self.settings = settings;
        }
    }
}
