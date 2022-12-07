//! Graphical User Interface methods.
//!
//! Uses [immediate mode](https://en.wikipedia.org/wiki/Immediate_mode_GUI). See the `gui` example
//! in the `examples/` folder for a full demo.
//!
//! # Note
//!
//! Many widgets rely on unique labels or IDs that are consistent across frames for internal state
//! management. This ID is generally built using the hash of the label combined with any parents
//! they may be under, for example items rendered under a tab are combined with the tabs label
//! hash.
//!
//! e.g.
//!
//! ```
//! # use pix_engine::prelude::*;
//! # fn draw(s: &mut PixState) -> PixResult<()> {
//! if s.button("Click")? {     // Label = "Click", ID = hash of "Click"
//!     // Handle click action
//! }
//! s.advanced_tooltip(
//!     "Advanced Tooltip",
//!     rect![s.mouse_pos(), 300, 100],
//!     |s: &mut PixState| {
//!         // Label = "Click", ID = hash of "Click" + "Advanced Tooltip"
//!         if s.button("Click")? {
//!             // Handle click action
//!         }
//!         Ok(())
//!     },
//! )?;
//! # Ok(())
//! # }
//! ```
//!
//! There is an ID stack system in place in addition to a `##` string pattern that can be used to
//! uniquely identify elements that may require an empty label or conflict with another element.
//!
//! If you find that your widget is not properly interacting with user events or maintaining state
//! correctly, try one of the following methods to ensure the label is unique.
//!
//! You can append a unique identifier after your label with `##`. Anything after this pattern
//! won't be visible to the user:
//!
//! ```
//! # use pix_engine::prelude::*;
//! # fn draw(s: &mut PixState) -> PixResult<()> {
//! if s.button("Click##action1")? {     // Label = "Click", ID = hash of "Click##action1"
//!     // Handle action 1
//! }
//! if s.button("Click##action2")? {     // Label = "Click", ID = hash of "Click##action2"
//!     // Handle action 2
//! }
//! # Ok(())
//! # }
//! ```
//!
//! You can use [`PixState::push_id`] and [`PixState::pop_id`] either by itself, or as part of a loop:
//!
//! ```
//! # use pix_engine::prelude::*;
//! # fn draw(s: &mut PixState) -> PixResult<()> {
//! for i in 0..5 {
//!   s.push_id(i);             // Push i to the ID stack
//!   if s.button("Click")? {   // Label = "Click",  ID = hash of "Click" + i
//!     // Handle click action
//!   }
//!   s.pop_id();
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, text_field: String };
//! # impl PixEngine for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.text("Some text")?;
//!     s.separator()?; // Adds a horizontal line separator
//!     s.spacing(); // Adds a line of spacing
//!
//!     if s.button("Button")? {
//!         // Button was clicked!
//!     }
//!
//!     s.checkbox("Checkbox", &mut self.checkbox)?;
//!
//!     s.next_width(200);
//!     s.text_field("Text Field", &mut self.text_field)?;
//!     Ok(())
//! }
//! # }
//! ```

use self::state::ElementId;
use crate::{
    ops::{clamp_dimensions, clamp_size},
    prelude::*,
    renderer::Rendering,
};

pub mod layout;
pub mod system;
pub mod theme;
pub mod widgets;

pub(crate) mod keys;
pub(crate) mod mouse;
pub(crate) mod scroll;
pub(crate) mod state;

/// Platform-specific control modifier key. `CTRL` on most platforms.
#[cfg(not(target_os = "macos"))]
pub const MOD_CTRL: KeyMod = KeyMod::CTRL;
/// Platform-specific control modifier key. `Command` on macOS.
#[cfg(target_os = "macos")]
pub const MOD_CTRL: KeyMod = KeyMod::GUI;

/// Coordinate Direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Up.
    Up,
    /// Down.
    Down,
    /// Left.
    Left,
    /// Right.
    Right,
}

impl PixState {
    /// Return usable UI width given the current UI cursor position and padding clamped to i32.
    ///
    /// # Errors
    ///
    /// If the current window target has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn ui_width(&self) -> PixResult<i32> {
        let pos = self.cursor_pos();
        let fpad = self.theme.spacing.frame_pad;
        Ok(clamp_size(self.width()?) - pos.x() - fpad.x())
    }

    /// Return usable UI height given the current UI cursor position and padding clamped to i32.
    ///
    /// # Errors
    ///
    /// If the current window target has been closed or is invalid, then an error is returned.
    #[inline]
    pub fn ui_height(&self) -> PixResult<i32> {
        let pos = self.cursor_pos();
        let fpad = self.theme.spacing.frame_pad;
        Ok(clamp_size(self.height()?) - pos.y() - fpad.y())
    }
}

impl PixState {
    /// Set and return default colors based on widget state for the given surface type.
    #[inline]
    pub(crate) fn widget_colors(&mut self, id: ElementId, surface_color: ColorType) -> [Color; 3] {
        // "On" overlay opacity:
        // - High emphasis: 87%
        // - Med emphasis: 60%
        // - Disabled: 38%
        // - Error: 100%
        // Stroke & Fill opacity:
        // - Focused: 12%
        // - Hovered: 4%
        // - Active: 8%

        let s = self;
        let focused = s.ui.is_focused(id);
        let active = s.ui.is_active(id);
        let hovered = s.ui.is_hovered(id);
        let disabled = s.ui.disabled;
        let c = s.theme.colors;

        let (bg, overlay) = match surface_color {
            ColorType::Background => (c.background, c.on_background),
            ColorType::Surface => (c.surface, c.on_surface),
            ColorType::Primary => (c.primary, c.on_primary),
            ColorType::PrimaryVariant => (c.primary_variant, c.on_primary),
            ColorType::Secondary => (c.secondary, c.on_secondary),
            ColorType::SecondaryVariant => (c.secondary_variant, c.on_secondary),
            ColorType::Error => (c.error, c.on_error),
            _ => panic!("invalid surface color"),
        };
        let branded = matches!(
            surface_color,
            ColorType::Primary
                | ColorType::PrimaryVariant
                | ColorType::Secondary
                | ColorType::SecondaryVariant,
        );

        let stroke_overlay = if branded {
            bg.blended(Color::WHITE, 0.60)
        } else {
            overlay
        };
        let stroke = if focused {
            stroke_overlay
        } else if disabled {
            stroke_overlay.blended(bg, 0.18)
        } else {
            stroke_overlay.blended(bg, 0.38)
        };

        let bg_overlay = if branded { Color::WHITE } else { overlay };
        let bg = if focused {
            bg_overlay.blended(bg, 0.12)
        } else if active {
            bg_overlay.blended(bg, 0.08)
        } else if hovered {
            if branded {
                bg_overlay.blended(bg, 0.12)
            } else {
                bg_overlay.blended(bg, 0.04)
            }
        } else if branded && disabled {
            overlay.blended(bg, 0.38)
        } else {
            bg
        };

        let fg = if disabled {
            overlay.blended(bg, 0.38)
        } else {
            overlay.blended(bg, 0.87)
        };

        [stroke, bg, fg]
    }

    /// Return the size of text, clamped to i32.
    #[inline]
    pub(crate) fn text_size(&self, text: &str) -> PixResult<(i32, i32)> {
        let s = &self.settings;
        let wrap_width = s.wrap_width;
        let ipad = self.theme.spacing.item_pad;
        let pos = self.cursor_pos();
        let wrap_width = if wrap_width.is_none() && text.contains('\n') {
            text.lines()
                .map(|line| {
                    let (line_width, _) = self.renderer.size_of(line, None).unwrap_or_default();
                    line_width
                })
                .max()
                .map(|width| width + (pos.x() + ipad.x()) as u32)
        } else {
            wrap_width
        };
        let (w, h) = self.renderer.size_of(text, wrap_width)?;
        // EXPL: Add same padding that `text_transformed` uses.
        Ok(clamp_dimensions(w + 3, h + 3))
    }
}
