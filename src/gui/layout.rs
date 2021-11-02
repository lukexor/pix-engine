//! UI spacing & layout rendering methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::same_line]
//! - [PixState::next_width]
//! - [PixState::spacing]
//! - [PixState::indent]
//! - [PixState::separator]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, text_field: String };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.text("Text")?;
//!     s.same_line(None);
//!     s.text("Same line")?;
//!     s.same_line([20, 0]);
//!     s.text("Same line with a +20 horizontal pixel offset")?;
//!
//!     s.separator();
//!
//!     s.spacing()?;
//!     s.indent()?;
//!     s.text("Indented!")?;
//!
//!     s.next_width(200);
//!     if s.button("Button")? {
//!         // was clicked
//!     }
//!     Ok(())
//! }
//! # }
//! ```

use crate::prelude::*;

impl PixState {
    /// Reset current UI rendering position back to the previous line with item padding, and
    /// continue with horizontal layout.
    ///
    /// You can optionally change the item padding, or set a different horizontal or vertical
    /// position by passing in an `offset`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Text")?;
    ///     s.same_line(None);
    ///     s.text("Same line")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn same_line<O>(&mut self, offset: O)
    where
        O: Into<Option<[i32; 2]>>,
    {
        let [x, y] = self.ui.pcursor.values();
        let offset = offset.into().unwrap_or([0; 2]);
        let item_pad = self.theme.style.item_pad;
        self.ui
            .set_cursor([x + item_pad.x() + offset[0], y + offset[1]]);
        self.ui.line_height = self.ui.pline_height - offset[1];
    }

    /// Change the default width of the next rendered element for elements that typically take up
    /// the remaining width of the window/frame they are rendered in.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.next_width(200);
    ///     if s.button("Button")? {
    ///         // was clicked
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn next_width(&mut self, width: u32) {
        self.ui.next_width = Some(width);
    }
}

impl PixState {
    /// Draw a newline worth of spacing to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Some text")?;
    ///     s.spacing()?;
    ///     s.text("Some other text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn spacing(&mut self) -> PixResult<()> {
        let s = self;
        let (_, height) = s.size_of(" ")?;
        s.advance_cursor([0, 0, 0, height as i32]);
        Ok(())
    }

    /// Draw an indent worth of spacing to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.indent()?;
    ///     s.text("Indented!")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn indent(&mut self) -> PixResult<()> {
        let s = self;
        let (width, height) = s.size_of("    ")?;
        s.advance_cursor([0, 0, width as i32, height as i32]);
        s.same_line(None);
        Ok(())
    }

    /// Draw a horizontal or vertical separator to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool, text_field: String };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Some text")?;
    ///     s.separator()?;
    ///     s.text("Some other text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn separator(&mut self) -> PixResult<()> {
        // TODO: Add s.layout(Direction) method
        let s = self;
        let pos = s.cursor_pos();
        let pad = s.theme.style.frame_pad;
        let height = s.theme.font_sizes.body as i32;
        let y = pos.y() + height / 2;

        s.push();

        s.stroke(s.primary_color());
        s.line([point![pad.x(), y], point![s.width()? as i32 - pad.x(), y]])?;

        s.pop();
        s.advance_cursor([0, 0, 0, height]);

        Ok(())
    }
}
