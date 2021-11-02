//! Drawing functions.
//!
//! Provides a [Draw] trait as well standard draw methods like [PixState::clear].
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App;
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.background(ALICE_BLUE);
//!     s.clear();
//!     let rect = rect![0, 0, 100, 100];
//!     s.fill(RED);
//!     s.stroke(BLACK);
//!     s.rect(rect)?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{prelude::*, renderer::Rendering};

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw object to the current [PixState] canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let rect = rect![0, 0, 100, 100];
    ///     // The following two lines are equivalent.
    ///     s.rect(rect)?;
    ///     rect.draw(s)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn draw(&self, s: &mut PixState) -> PixResult<()>;
}

impl PixState {
    /// Clears the render target to the current background [Color] set by [PixState::background].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.background(CADET_BLUE);
    ///     s.clear();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn clear(&mut self) -> PixResult<()> {
        self.renderer.set_draw_color(self.settings.background)?;
        self.renderer.clear()
    }
}
