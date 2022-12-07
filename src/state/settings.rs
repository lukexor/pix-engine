//! Settings methods for the [`Engine`].
//!
//! Methods for reading and setting various engine configuration values.
//!
//! Provided types:
//!
//! - [`DrawMode`]: Determines how `(x, y)` coordinates are used for rendering.
//! - [`RectMode`]: Alias for `DrawMode`.
//! - [`EllipseMode`]: Alias for `DrawMode`.
//! - [`ImageMode`]: Alias for `DrawMode`.
//! - [`ArcMode`]: Determines how arcs are rendered.
//! - [`BlendMode`]: Determines how images and textures are blended.
//! - [`AngleMode`]: Determines how angles are interpreted.
//! - [`FontStyle`]: Determines how text is rendered.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::background`]: Sets the [Color] used by [`PixState::clear`] to clear the canvas.
//! - [`PixState::fill`]: Sets the [Color] used to fill shapes.
//! - [`PixState::stroke`]: Sets the [Color] used to stroke shapes and text.
//! - [`PixState::stroke_weight`]: Sets the stroke line thickness for lines and text.
//! - [`PixState::text_shadow`]: Sets the shadow distance for drawing text.
//! - [`PixState::smooth`]: Enables the anti-alias smoothing option for drawing shapes.
//! - [`PixState::bezier_detail`]: Set the resolution at which Bezier curves are dispalyed.
//! - [`PixState::wrap`]: Sets the wrap width for rendering text.
//! - [`PixState::clip`]: Sets a clip rectangle for rendering.
//! - [`PixState::fullscreen`]: Sets fullscreen mode to enabled or disabled.
//! - [`PixState::toggle_fullscreen`]: Toggles fullscreen.
//! - [`PixState::vsync`]: Sets vertical sync mode to enabled or disabled.
//! - [`PixState::toggle_vsync`]: Toggles vertical sync.
//! - [`PixState::cursor`]: Set a custom window cursor or hide the cursor.
//! - [`PixState::disable`]: Disable UI elements from being interactive.
//! - [`PixState::running`]: Whether the render loop is running (calling [`PixEngine::on_update`]).
//! - [`PixState::run`]: Enable or disable the render loop.
//! - [`PixState::show_frame_rate`]: Display the average frame rate in the title bar.
//! - [`PixState::target_frame_rate`]: Return the current targeted frame rate.
//! - [`PixState::frame_rate`]: Set or clear a targeted frame rate.
//! - [`PixState::scale`]: Set the rendering scale of the current canvas.
//! - [`PixState::rect_mode`]: Change the [`RectMode`] for rendering rectangles.
//! - [`PixState::ellipse_mode`]: Change the [`EllipseMode`] for rendering ellipses.
//! - [`PixState::image_mode`]: Change the [`ImageMode`] for rendering images.
//! - [`PixState::image_tint`]: Set or clear a [Color] used to tint [Image]s.
//! - [`PixState::arc_mode`]: Change the [`ArcMode`] for rendering arcs.
//! - [`PixState::angle_mode`]: Change the [`AngleMode`] for angle interpretation.
//! - [`PixState::blend_mode`]: Change the [`BlendMode`] for rendering images and textures.
//! - [`PixState::push`]: Push a copy of all the current settings to a stack.
//! - [`PixState::pop`]: Pop the previously pushed settings off the stack, restoring them.

use crate::{
    error::Result,
    prelude::*,
    renderer::{Rendering, WindowRenderer},
};
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

/// Drawing mode which determines how `(x, y)` coordinates are interpreted when drawing [Rect]s.
pub type RectMode = DrawMode;

/// Drawing mode which determines how `(x, y)` coordinates are interpreted when drawing [Ellipse]s.
pub type EllipseMode = DrawMode;

/// Drawing mode which determines how `(x, y)` coordinates are interpreted when drawing [Image]s.
pub type ImageMode = DrawMode;

/// Drawing mode which determines how arcs are drawn.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArcMode {
    /// Draws arc as an open, unfilled pie segment using `stroke` color.
    Default,
    /// Draws arc as a closed pie segment using `stroke` and `fill` colors.
    Pie,
}

/// Drawing mode which determines how textures are blended together.
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

/// Determines how angles are interpreted.
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
    pub(crate) stroke_weight: u16,
    pub(crate) font_size: u32,
    pub(crate) font_style: FontStyle,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) font_family: Font,
    pub(crate) text_shadow: Option<u16>,
    pub(crate) smooth: bool,
    pub(crate) bezier_detail: i32,
    pub(crate) wrap_width: Option<u32>,
    pub(crate) clip: Option<Rect<i32>>,
    pub(crate) running: bool,
    pub(crate) show_frame_rate: bool,
    pub(crate) target_frame_rate: Option<usize>,
    pub(crate) target_delta_time: Option<Duration>,
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
    pub(crate) disabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Color::BLACK,
            fill: Some(Color::WHITE),
            stroke: None,
            stroke_weight: 1,
            font_size: 14,
            font_style: FontStyle::NORMAL,
            font_family: Font::default(),
            text_shadow: None,
            smooth: true,
            bezier_detail: 20,
            wrap_width: None,
            clip: None,
            running: true,
            show_frame_rate: false,
            target_frame_rate: None,
            target_delta_time: None,
            scale_x: 1.0,
            scale_y: 1.0,
            rect_mode: RectMode::Corner,
            ellipse_mode: EllipseMode::Center,
            image_mode: ImageMode::Corner,
            image_tint: None,
            arc_mode: ArcMode::Default,
            angle_mode: AngleMode::Radians,
            blend_mode: BlendMode::None,
            cursor: Some(Cursor::default()),
            disabled: false,
        }
    }
}

impl PixState {
    /// Sets the [Color] value used to clear the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.background(Color::ALICE_BLUE);
    ///     s.clear();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn background<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.background = color.into();
        let _result = self.clear(); // If this errors, something is very wrong
    }

    /// Sets the [Color] value used to fill shapes drawn on the canvas. `None` disables fill
    /// entirely.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.fill(Color::ALICE_BLUE);
    ///     s.rect([0, 0, 100, 100])?;
    ///     s.fill((None));
    ///     s.rect([25, 25, 75, 75])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn fill<C>(&mut self, color: C)
    where
        C: Into<Option<Color>>,
    {
        self.settings.fill = color.into();
    }

    /// Sets the [Color] value used to outline shapes drawn on the canvas. `None` disables stroke
    /// entirely.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.stroke(Color::BLACK);
    ///     s.rect([0, 0, 100, 100])?;
    ///     s.stroke((None));
    ///     s.rect([25, 25, 75, 75])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn stroke<C>(&mut self, color: C)
    where
        C: Into<Option<Color>>,
    {
        self.settings.stroke = color.into();
    }

    /// Sets the width used to draw lines on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.stroke(Color::BLUE);
    ///     s.stroke_weight(2);
    ///     // Draws a 2-pixel wide diagonal line
    ///     s.line(line_![0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn stroke_weight(&mut self, weight: u16) {
        self.settings.stroke_weight = weight;
    }

    /// Set the font size for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the given font size from the currently loaded font data, then
    /// an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.font_size(22);
    ///     s.text("Some big text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn font_size(&mut self, size: u32) -> Result<()> {
        self.settings.font_size = size;
        self.theme.font_size = size;
        self.renderer.font_size(size)
    }

    /// Set the font style for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the current font, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.font_style(FontStyle::BOLD);
    ///     s.text("Some bold text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn font_style(&mut self, style: FontStyle) {
        self.settings.font_style = style;
        self.renderer.font_style(style);
    }

    /// Set the font family for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the given font size from the currently loaded font data, then
    /// an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.font_family(Font::NOTO)?;
    ///     s.text("Some NOTO family text")?;
    ///     s.font_family(Font::from_file("Custom font", "./custom_font.ttf"))?;
    ///     s.text("Some custom family text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn font_family(&mut self, font: Font) -> Result<()> {
        self.settings.font_family = font;
        self.renderer.font_family(&self.settings.font_family)
    }

    /// Sets the text shadow distance used to draw text on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text_shadow(2);
    ///     // Draws a 2-pixel offset shhadow
    ///     s.text("Shadowed")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn text_shadow<D>(&mut self, distance: D)
    where
        D: Into<Option<u16>>,
    {
        self.settings.text_shadow = distance.into();
    }

    /// Enable or disable the anti-alias option used for drawing shapes on the canvas. `smooth` is
    /// enabled by default.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Draws a anti-aliased diagonal line
    ///     s.smooth(true);
    ///     s.line(line_![0, 0, 100, 100])?;
    ///     // Disables anti-aliasing
    ///     s.smooth(false);
    ///     s.line(line_![0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn smooth(&mut self, val: bool) {
        self.settings.smooth = val;
    }

    /// Set the resolution at which [`PixState::bezier`] curves are displayed. The default is `20`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.bezier_detail(5);
    ///     s.stroke(Color::RED);
    ///     s.bezier([[85, 20], [10, 10], [90, 90], [15, 80]])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn bezier_detail(&mut self, detail: i32) {
        self.settings.bezier_detail = detail;
    }

    /// Sets the wrap width used to draw text on the canvas. `None` disables text wrap.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Renders as (depending on font width):
    ///     //
    ///     // Lorem ipsum
    ///     // dollor sit amet,
    ///     // consetetur
    ///     // sadipscing
    ///     // elitr, sed diam
    ///     s.wrap(100);
    ///     s.text("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam")?;
    ///
    ///     // Disable wrapping
    ///     s.wrap((None));
    ///     s.text("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn wrap<W>(&mut self, width: W)
    where
        W: Into<Option<u32>>,
    {
        self.settings.wrap_width = width.into();
    }

    /// Sets the clip [Rect] used by the renderer to draw to the current canvas. `None` disables
    /// clipping.
    ///
    /// # Errors
    ///
    /// If the current render target is closed or dropped, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.clip(rect![0, 0, 100, 100])?;
    ///     // Renders a quarter pie-slice with radius 100
    ///     s.circle([100, 100, 200, 200])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn clip<R>(&mut self, rect: R) -> Result<()>
    where
        R: Into<Option<Rect<i32>>>,
    {
        self.settings.clip = rect.into();
        self.renderer.clip(self.settings.clip)
    }

    /// Set the application to fullscreen or not.
    ///
    /// # Errors
    ///
    /// If the current render target is closed or dropped or the renderer fails to set
    /// fullscreen, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Return = event.key {
    ///         s.fullscreen(true)?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn fullscreen(&mut self, val: bool) -> Result<()> {
        self.renderer.set_fullscreen(val)
    }

    /// Toggle fullscreen.
    ///
    /// # Errors
    ///
    /// If the current render target is closed or dropped or the renderer fails to toggle
    /// fullscreen, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Return = event.key {
    ///         s.toggle_fullscreen()?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn toggle_fullscreen(&mut self) -> Result<()> {
        let is_fullscreen = self.renderer.fullscreen()?;
        self.renderer.set_fullscreen(!is_fullscreen)
    }

    /// Set the window to synchronize frame rate to the screens refresh rate ([`VSync`]).
    ///
    /// # Note
    ///
    /// Due to the current limitation with changing `VSync` at runtime, this method creates a new
    /// window using the properties of the current window and returns the new `WindowId`.
    ///
    /// If you are storing and interacting with this window using the `WindowId`, make sure to
    /// use the newly returned `WindowId`.
    ///
    /// [`VSync`]: https://en.wikipedia.org/wiki/Screen_tearing#Vertical_synchronization
    ///
    /// # Errors
    ///
    /// If the current render target is closed or dropped or the renderer fails to set
    /// vsync, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Return = event.key {
    ///         s.vsync(true)?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn vsync(&mut self, val: bool) -> Result<WindowId> {
        self.renderer.set_vsync(val)
    }

    /// Toggle synchronizing frame rate to the screens refresh rate ([`VSync`]).
    ///
    /// # Note
    ///
    /// Due to the current limitation with changing `VSync` at runtime, this method creates a new
    /// window using the properties of the current window and returns the new `WindowId`.
    ///
    /// If you are storing and interacting with this window using the `WindowId`, make sure to
    /// use the newly returned `WindowId`.
    ///
    /// [`VSync`]: https://en.wikipedia.org/wiki/Screen_tearing#Vertical_synchronization
    ///
    /// # Errors
    ///
    /// If the current render target is closed or dropped or the renderer fails to toggle
    /// vsync, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Return = event.key {
    ///         s.toggle_vsync()?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn toggle_vsync(&mut self) -> Result<WindowId> {
        let vsync_enabled = self.renderer.vsync();
        self.renderer.set_vsync(vsync_enabled)
    }

    /// Set the mouse cursor to a predefined symbol or image. `None` hides the cursor.
    ///
    /// # Errors
    ///
    /// If the renderer fails to set the cursor or load it from an image file, then an error is
    /// returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text("Hover me")?;
    ///     if s.hovered() {
    ///         s.cursor(Cursor::hand())?;
    ///     } else {
    ///         s.cursor(Cursor::arrow())?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn cursor<C>(&mut self, cursor: C) -> Result<()>
    where
        C: Into<Option<Cursor>>,
    {
        self.settings.cursor = cursor.into();
        self.renderer.cursor(self.settings.cursor.as_ref())
    }

    /// Disables any UI elements drawn after this is called, preventing them from being interacted
    /// with.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool };
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     if s.button("Disable UI")? {
    ///         s.disable(true);
    ///     }
    ///     s.checkbox("Disabled checkbox", &mut self.checkbox)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn disable(&mut self, disabled: bool) {
        self.settings.disabled = disabled;
        self.ui.disabled = disabled;
    }

    /// Whether the render loop is running or not.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Return = event.key {
    ///         // Toggle pausing rendering
    ///         let running = s.running();
    ///         s.run(!running);
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn running(&mut self) -> bool {
        self.settings.running
    }

    /// Pause or resume the render loop called by [`PixEngine::on_update`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Return = event.key {
    ///         // Toggle rendering
    ///         let running = s.running();
    ///         s.run(running);
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn run(&mut self, val: bool) {
        self.settings.running = val;
    }

    /// Set whether to show the current frame rate per second in the title or not.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Return = event.key {
    ///         s.show_frame_rate(true);
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn show_frame_rate(&mut self, show: bool) {
        self.settings.show_frame_rate = show;
    }

    /// Get the target frame rate to render at.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Down = event.key {
    ///         let target = s.target_frame_rate().unwrap_or(60);
    ///         s.frame_rate(target - 10);
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn target_frame_rate(&mut self) -> Option<usize> {
        self.settings.target_frame_rate
    }

    /// Set a target frame rate to render at, controls how often [`PixEngine::on_update`] is
    /// called. `None` clears the target frame rate.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Target a lower FPS than natively possible
    ///     s.frame_rate(30);
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn frame_rate<R>(&mut self, frame_rate: R)
    where
        R: Into<Option<usize>>,
    {
        let frame_rate = frame_rate.into();
        self.settings.target_frame_rate = frame_rate;
        self.settings.target_delta_time =
            frame_rate.map(|frame_rate| Duration::from_secs(1) / frame_rate as u32);
    }

    /// Set the rendering scale of the current canvas. Drawing coordinates are scaled by x/y
    /// factors before being drawn to the canvas.
    ///
    /// # Errors
    ///
    /// If the current render target is closed or dropped, or `(x, y`) contain invalid values,
    /// then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::Plus = event.key {
    ///         s.scale(2.0, 2.0)?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn scale(&mut self, x: f32, y: f32) -> Result<()> {
        let mut s = &mut self.settings;
        s.scale_x = x;
        s.scale_y = y;
        self.renderer.scale(s.scale_x, s.scale_y)
    }

    /// Change the way parameters are interpreted for drawing [Square](Rect)s and
    /// [Rectangle](Rect)s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.rect_mode(RectMode::Center);
    ///     // Draw rect with center at `(100, 100)`
    ///     s.rect([100, 100, 50, 50])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn rect_mode(&mut self, mode: RectMode) {
        self.settings.rect_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Ellipse]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.ellipse_mode(EllipseMode::Center);
    ///     // Draw ellipse with center at `(100, 100)`
    ///     s.ellipse([100, 100, 50, 50])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn ellipse_mode(&mut self, mode: EllipseMode) {
        self.settings.ellipse_mode = mode;
    }

    /// Change the way parameters are interpreted for drawing [Image]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.image_mode(ImageMode::Center);
    ///     // Draw image with center at `(100, 100)`
    ///     s.image(&Image::from_file("./some_image.png")?, [100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn image_mode(&mut self, mode: ImageMode) {
        self.settings.image_mode = mode;
    }

    /// Add a color tint to [Image]s when drawing.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.image_tint(Color::RED);
    ///     // Draw image tinted red
    ///     s.image(&Image::from_file("./some_image.png")?, [0, 0])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn image_tint<C>(&mut self, tint: C)
    where
        C: Into<Option<Color>>,
    {
        self.settings.image_tint = tint.into();
    }

    /// Change the way arcs are drawn.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // Draw arc as a open, unfilled pie segment using only the `stroke` (The default)
    ///     s.arc_mode(ArcMode::Default);
    ///     s.arc([100, 100], 20, 0, 180)?;
    ///     s.arc_mode(ArcMode::Pie);
    ///     // Draw arc as a closed pie segment using both `fill` and `stroke`
    ///     s.arc([200, 200], 20, 0, 180)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn arc_mode(&mut self, mode: ArcMode) {
        self.settings.arc_mode = mode;
    }

    /// Change the way angles are interpreted for rotation and matrix transformations.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.angle_mode(AngleMode::Degrees);
    ///     let angle = 10.0;
    ///     let center = point!(10, 10);
    ///     s.text_transformed("Rotated text", angle, center, None)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn angle_mode(&mut self, mode: AngleMode) {
        self.settings.angle_mode = mode;
    }

    /// Change the way textures are blended together.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.blend_mode(BlendMode::Blend);
    ///     // Draw image with alpha blended with background
    ///     s.image(&Image::from_file("./some_image.png")?, [0, 0])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn blend_mode(&mut self, mode: BlendMode) {
        self.settings.blend_mode = mode;
        self.renderer.blend_mode(mode);
    }

    /// Saves the current draw settings and transforms.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.fill(Color::BLUE);
    ///     s.stroke(Color::WHITE);
    ///
    ///     s.push(); // Save settings
    ///
    ///     s.fill(Color::RED);
    ///     s.stroke(Color::BLACK);
    ///     s.rect([0, 0, 100, 100])?;
    ///
    ///     s.pop(); // Restore settings
    ///
    ///     // Rectangle is blue with a white outline
    ///     s.rect([0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    #[inline]
    pub fn push(&mut self) {
        self.setting_stack.push(self.settings.clone());
    }

    /// Restores the previous draw settings and transforms, if present. If the settings stack is
    /// empty, the settings will remain unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.fill(Color::BLUE);
    ///     s.stroke(Color::WHITE);
    ///
    ///     s.push(); // Save settings
    ///
    ///     s.fill(Color::RED);
    ///     s.stroke(Color::BLACK);
    ///     s.rect([0, 0, 100, 100])?;
    ///
    ///     s.pop(); // Restore settings
    ///
    ///     // Rectangle is blue with a white outline
    ///     s.rect([0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    #[inline]
    pub fn pop(&mut self) {
        if let Some(settings) = self.setting_stack.pop() {
            self.settings = settings;
            self.ui.disabled = self.settings.disabled;
        }
        let s = &self.settings;
        // All of these settings should be valid since they were set prior to `pop()` being
        // called.
        let _ = self.renderer.clip(s.clip);
        // Excluding restoring cursor - as it's used for mouse hover.
        let _ = self.renderer.font_size(s.font_size);
        self.renderer.font_style(s.font_style);
        let _ = self.renderer.font_family(&s.font_family);
        self.renderer.blend_mode(s.blend_mode);
    }
}

impl PixState {
    /// Set the mouse cursor to a predefined symbol or image for a single frame.
    ///
    /// Cursor will get reset to the current setting next frame.
    #[inline]
    pub(crate) fn frame_cursor(&mut self, cursor: &Cursor) -> Result<()> {
        self.renderer.cursor(Some(cursor))
    }

    /// Get the target delta time between frames.
    #[inline]
    pub(crate) fn target_delta_time(&self) -> Option<Duration> {
        self.settings.target_delta_time
    }

    /// Get whether `VSync` is enabled.
    #[inline]
    pub(crate) fn vsync_enabled(&self) -> bool {
        self.renderer.vsync()
    }
}
