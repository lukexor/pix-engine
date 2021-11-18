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

#[cfg(not(target_os = "macos"))]
pub(crate) const MOD_CTRL: KeyMod = KeyMod::CTRL;
#[cfg(target_os = "macos")]
pub(crate) const MOD_CTRL: KeyMod = KeyMod::GUI;

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
    pub fn clipboard_text(&self) -> String {
        self.renderer.clipboard_text()
    }

    /// Set clipboard text to the system clipboard.
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
        // - Default: 12%
        // - Focused: +12%
        // - Hovered: +4%
        // - Active: +8%

        let s = self;
        let focused = s.ui.is_focused(id);
        let active = s.ui.is_active(id);
        let hovered = s.ui.is_hovered(id);
        let disabled = s.ui.disabled;
        let c = s.theme.colors;

        use ColorType::*;
        let (bg, overlay) = match surface_color {
            Background => (c.background, c.on_background),
            Surface => (c.surface, c.on_surface),
            Primary => (c.primary, c.on_primary),
            PrimaryVariant => (c.primary_variant, c.on_primary),
            Secondary => (c.secondary, c.on_secondary),
            SecondaryVariant => (c.secondary_variant, c.on_secondary),
            Error => (c.error, c.on_error),
            _ => panic!("invalid surface color"),
        };
        let stroke = if focused {
            overlay
        } else if disabled {
            overlay.blended(bg, 0.18)
        } else {
            overlay.blended(bg, 0.38)
        };
        let bg_overlay = match surface_color {
            Primary | PrimaryVariant | Secondary | SecondaryVariant => WHITE,
            _ => overlay,
        };
        let bg = if focused {
            bg_overlay.blended(bg, 0.12)
        } else if active {
            bg_overlay.blended(bg, 0.16)
        } else if disabled {
            if let Primary | PrimaryVariant | Secondary | SecondaryVariant = surface_color {
                overlay.blended(bg, 0.38)
            } else {
                bg
            }
        } else if hovered {
            bg_overlay.blended(bg, 0.16)
        } else if let Surface = surface_color {
            bg_overlay.blended(bg, 0.08)
        } else {
            bg
        };
        let fg = if disabled {
            overlay.blended(bg, 0.38)
        } else if let Background | Surface = surface_color {
            overlay.blended(bg, 0.87)
        } else {
            overlay
        };

        [stroke, bg, fg]
    }
}
