//! Operating System related methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::clipboard_text`]
//! - [`PixState::set_clipboard_text`]
//! - [`PixState::open_url`]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { text_entry: String };
//! # impl PixEngine for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     if s.button("Open Homepage")? {
//!         s.open_url("https://example.com")?;
//!     }
//!     Ok(())
//! }
//!
//! fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
//!     match (event.keymod, event.key) {
//!         (KeyMod::CTRL, Key::C) => s.set_clipboard_text(&self.text_entry)?,
//!         (KeyMod::CTRL, Key::V) => self.text_entry = s.clipboard_text(),
//!         _ => (),
//!     }
//!     Ok(false)
//! }
//! # }
//! ```

use crate::{prelude::*, renderer::Rendering};

impl PixState {
    /// Get clipboard text from the system clipboard.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl PixEngine for App {
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
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.set_clipboard_text("Copied to clipboard!")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn set_clipboard_text<S>(&self, value: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        self.renderer.set_clipboard_text(value.as_ref())
    }

    /// Open a URL in the default system browser.
    ///
    /// # Errors
    ///
    /// If the renderer fails to launch an external application, then an error is returned. Note,
    /// there is no way to know if the URL opened successfully or not. An `Ok` result only means an
    /// application was successfully launched to handle the URL.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     if s.button("Open Homepage")? {
    ///         s.open_url("https://example.com")?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn open_url<S>(&self, url: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        self.renderer.open_url(url.as_ref())
    }
}
