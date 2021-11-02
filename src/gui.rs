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
