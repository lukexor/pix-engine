//! Graphical User Interface functions.
//!
//! Uses [immediate mode](https://en.wikipedia.org/wiki/Immediate_mode_GUI). See the `gui` example
//! in the `examples/` folder for a full demo.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, text_field: String };
//! # impl AppState for App {
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
use crate::{prelude::*, renderer::Rendering};

pub mod layout;
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

/// Scroll direction.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Direction {
    /// Horizontal.
    Horizontal,
    /// Vertical.
    Vertical,
}

impl PixState {
    /// Get clipboard text from the system clipboard.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     println!("Current clipboard text: {}", s.clipboard_text());
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn clipboard_text(&self) -> String {
        self.renderer.clipboard_text()
    }

    /// Set clipboard text to the system clipboard.
    ///
    /// # Errors
    ///
    /// If the renderer fails to retrieve the clipboard text, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.set_clipboard_text("Copied to clipboard!")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn set_clipboard_text(&self, value: &str) -> PixResult<()> {
        self.renderer.set_clipboard_text(value)
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
}
