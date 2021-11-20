//! Settings methods for the [`PixEngine`].
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
//! - [`PixState::no_fill`]: Clears the [Color] used to fill shapes.
//! - [`PixState::stroke`]: Sets the [Color] used to stroke shapes and text.
//! - [`PixState::no_stroke`]: Clears the [Color] used to stroke shapes and text.
//! - [`PixState::stroke_weight`]: Sets the stroke line thickness for lines and text.
//! - [`PixState::wrap`]: Sets the wrap width for rendering text.
//! - [`PixState::no_wrap`]: Clears the wrap width for rendering text.
//! - [`PixState::clip`]: Sets a clip rectangle for rendering.
//! - [`PixState::no_clip`]: Clears the clip rectangle for rendering.
//! - [`PixState::fullscreen`]: Sets fullscreen mode to enabled or disabled.
//! - [`PixState::toggle_fullscreen`]: Toggles fullscreen.
//! - [`PixState::vsync`]: Sets vertical sync mode to enabled or disabled.
//! - [`PixState::toggle_vsync`]: Toggles vertical sync.
//! - [`PixState::cursor`]: Set a custom window cursor.
//! - [`PixState::no_cursor`]: Hide the window cursor.
//! - [`PixState::running`]: Whether the render loop is running (calling [`AppState::on_update`]).
//! - [`PixState::run`]: Enable the render loop.
//! - [`PixState::no_run`]: Disable the render loop.
//! - [`PixState::show_frame_rate`]: Display the average frame rate in the title bar.
//! - [`PixState::target_frame_rate`]: Return the current targeted frame rate.
//! - [`PixState::frame_rate`]: Set a targeted frame rate.
//! - [`PixState::clear_frame_rate`]: Clears the targeted frame rate.
//! - [`PixState::scale`]: Scale the current canvas.
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
            background: Color::BLACK,
            fill: Some(Color::WHITE),
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
            ellipse_mode: EllipseMode::Center,
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
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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

    /// Sets the [Color] value used to fill shapes drawn on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::ALICE_BLUE);
    ///     s.rect([0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn fill<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.fill = Some(color.into());
    }

    /// Disables filling shapes drawn on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.stroke(Color::BLACK);
    ///     s.no_fill();
    ///     // Draws a black outlined rectangle, with the background showing through
    ///     s.rect([0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn no_fill(&mut self) {
        self.settings.fill = None;
    }

    /// Sets the [Color] value used to outline shapes drawn on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.stroke(Color::BLACK);
    ///     s.rect([0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn stroke<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        self.settings.stroke = Some(color.into());
    }

    /// Disables outlining shapes drawn on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLUE);
    ///     s.no_stroke();
    ///     // Shows a solid blue rectangle with no outline
    ///     s.rect([0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn no_stroke(&mut self) {
        self.settings.stroke = None;
    }

    /// Sets the width used to draw lines on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.stroke(Color::BLUE);
    ///     s.stroke_weight(2);
    ///     // Shows a 2-pixel wide diagonal line
    ///     s.line(line_![0, 0, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn stroke_weight(&mut self, weight: u8) {
        self.settings.stroke_weight = weight;
    }

    /// Sets the wrap width used to draw text on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.wrap(100);
    ///     // Renders as (depending on font width):
    ///     //
    ///     // Lorem ipsum
    ///     // dollor sit amet,
    ///     // consetetur
    ///     // sadipscing
    ///     // elitr, sed diam
    ///     s.text("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn wrap(&mut self, width: u32) {
        self.settings.wrap_width = Some(width);
    }

    /// Disable wrapping when drawing text on the canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.no_wrap();
    ///     // Renders all on one line, which may extend beyond the window width
    ///     s.text("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam")?;
    ///     // Still honors newlines and so renders as two lines
    ///     s.text("Lorem ipsum dolor sit amet,\nconsetetur sadipscing elitr, sed diam")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn no_wrap(&mut self) {
        self.settings.wrap_width = None;
    }

    /// Sets the clip [Rect] used by the renderer to draw to the current canvas.
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.clip([0, 0, 100, 100])?;
    ///     // Renders a quarter pie-slice with radius 100
    ///     s.circle([100, 100, 200, 200])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn clip<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.settings.clip = Some(rect.into());
        self.renderer.clip(self.settings.clip)
    }

    /// Clears the clip [Rect] used by the renderer to draw to the current canvas.
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.clip([0, 0, 100, 100])?;
    ///     // Renders a quarter pie-slice with radius 100
    ///     s.circle([100, 100, 200, 200])?;
    ///     s.no_clip()?;
    ///     // Renders a circle with radius 100 in the center of the pie-slice
    ///     s.circle([100, 100, 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn no_clip(&mut self) -> PixResult<()> {
        self.settings.clip = None;
        self.renderer.clip(None)
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
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Return = event.key {
    ///         s.fullscreen(true)?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn fullscreen(&mut self, val: bool) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Return = event.key {
    ///         s.toggle_fullscreen()?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn toggle_fullscreen(&mut self) -> PixResult<()> {
        let is_fullscreen = self.renderer.fullscreen()?;
        self.renderer.set_fullscreen(!is_fullscreen)
    }

    /// Set the window to synchronize frame rate to the screens refresh rate ([`VSync`]).
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
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Return = event.key {
    ///         s.vsync(true)?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn vsync(&mut self, val: bool) -> PixResult<()> {
        self.renderer.set_vsync(val)
    }

    /// Toggle synchronizing frame rate to the screens refresh rate ([`VSync`]).
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
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Return = event.key {
    ///         s.toggle_vsync()?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn toggle_vsync(&mut self) -> PixResult<()> {
        let vsync_enabled = self.renderer.vsync();
        self.renderer.set_vsync(vsync_enabled)
    }

    /// Set the mouse cursor to a predefined symbol or image.
    ///
    /// # Errors
    ///
    /// If the rendere fails to set the cursor or load it from an image file, then an error is
    /// returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    pub fn cursor(&mut self, cursor: Cursor) -> PixResult<()> {
        self.settings.cursor = Some(cursor);
        self.renderer.cursor(self.settings.cursor.as_ref())
    }

    /// Hide the mouse cursor.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.no_cursor();
    ///     if s.button("Hover me")? {
    ///         println!("Interactable elements show a cursor temporarily");
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn no_cursor(&mut self) {
        self.settings.cursor = None;
        // SAFETY: Setting to NONE to hide cursor can't error.
        let _cant_fail = self.renderer.cursor(None);
    }

    /// Whether the render loop is running or not.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Return = event.key {
    ///         // Pause rendering
    ///         if s.running() {
    ///             s.no_run();
    ///         } else {
    ///             s.run();
    ///         }
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

    /// Unpause the render loop.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Return = event.key {
    ///         // Pause rendering
    ///         if s.running() {
    ///             s.no_run();
    ///         } else {
    ///             s.run();
    ///         }
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn run(&mut self) {
        self.settings.running = true;
    }

    /// Pause the render loop by no longer calling [`AppState::on_update`] every frame.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Return = event.key {
    ///         // Pause rendering
    ///         if s.running() {
    ///             s.no_run();
    ///         } else {
    ///             s.run();
    ///         }
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn no_run(&mut self) {
        self.settings.running = false;
    }

    /// Set whether to show the current frame rate per second in the title or not.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
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
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
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

    /// Set a target frame rate to render at, controls how often [`AppState::on_update`] is called.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     // Target a lower FPS than natively possible
    ///     s.frame_rate(30);
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn frame_rate(&mut self, rate: usize) {
        self.settings.target_frame_rate = Some(rate);
    }

    /// Remove target frame rate and call [`AppState::on_update`] as often as possible.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Escape = event.key {
    ///         // Resume rendering as many frames as possible per second
    ///         s.clear_frame_rate();
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn clear_frame_rate(&mut self) {
        self.settings.target_frame_rate = None;
    }

    /// Set the rendering scale of the current canvas.
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
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::Plus = event.key {
    ///         s.scale(2.0, 2.0)?;
    ///         return Ok(true);
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn scale(&mut self, x: f32, y: f32) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
        self.setting_stack
            .push((self.settings.clone(), self.theme.clone()));
    }

    /// Restores the previous draw settings and transforms, if present. If the settings stack is
    /// empty, the settings will remain unchanged.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
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
        if let Some((settings, theme)) = self.setting_stack.pop() {
            self.settings = settings;
            self.theme = theme;
        }
        let s = &self.settings;
        let t = &self.theme;
        // All of these settings should be valid since they were set prior to `pop()` being
        // called.
        self.renderer.clip(s.clip).expect("valid clip setting");
        // Excluding restoring cursor - as it's used for mouse hover.
        self.renderer
            .font_size(t.sizes.body)
            .expect("valid font size");
        self.renderer.font_style(t.styles.body);
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
    pub(crate) fn frame_cursor(&mut self, cursor: &Cursor) -> PixResult<()> {
        self.renderer.cursor(Some(cursor))
    }

    /// Get the target delta time between frames.
    #[inline]
    pub(crate) fn target_delta_time(&self) -> Duration {
        self.settings
            .target_frame_rate
            .map_or_else(Duration::default, |rate| Duration::from_secs(rate as u64))
    }
}
