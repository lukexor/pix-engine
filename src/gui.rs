//! Graphical User Interface functions.

use crate::{prelude::*, renderer::Rendering};

pub mod keys;
pub mod layout;
pub mod mouse;
pub mod state;
pub mod theme;
pub mod widgets;

pub use theme::*;

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
    fn clipboard_text(&self) -> String {
        self.renderer.clipboard_text()
    }

    /// Set clipboard text to the system clipboard.
    fn set_clipboard_text(&self, value: &str) -> PixResult<()> {
        Ok(self.renderer.set_clipboard_text(value)?)
    }
}
