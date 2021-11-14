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
        let [x, y] = self.ui.pcursor.as_array();
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

    /// Draw a tabbed view to the current canvas. It accepts a list of tabs to be rendered and a
    /// closure that is passed the current tab and [`&mut PixState`][PixState] which you can use to draw all the
    /// standard drawing primitives and change any drawing settings. Settings changed inside the
    /// closure will not persist, similar to [PixState::with_texture].
    pub fn tab_bar<S, I, F>(&mut self, label: S, tabs: &[I], f: F) -> PixResult<()>
    where
        S: AsRef<str>,
        I: AsRef<str>,
        F: FnOnce(&str, &mut PixState) -> PixResult<()>,
    {
        let label = label.as_ref();

        let s = self;
        let tab_id = s.ui.get_id(&label);
        let fpad = s.theme.style.frame_pad;
        let ipad = s.theme.style.item_pad;

        for (i, tab_label) in tabs.iter().enumerate() {
            if i > 0 {
                s.same_line(None);
            }
            let tab_label = tab_label.as_ref();
            let id = s.ui.get_id(&tab_label);
            let pos = s.cursor_pos();

            // Calculate tab size
            let (width, height) = s.size_of(tab_label)?;
            let mut tab = rect![
                pos.x() + fpad.x(),
                pos.y(),
                width as i32 + 2 * ipad.x(),
                height as i32 + 2 * ipad.y()
            ];

            // Check hover/active/keyboard focus
            let hovered = s.ui.try_hover(id, tab);
            let focused = s.ui.try_focus(id);
            let disabled = s.ui.disabled;
            let active = s.ui.is_active(id);

            s.push();
            s.ui.push_cursor();

            // Render
            s.rect_mode(RectMode::Corner);

            s.push();
            if focused {
                s.stroke(s.highlight_color());
            } else {
                s.stroke(s.muted_color());
            }
            if hovered {
                s.frame_cursor(Cursor::hand())?;
                s.fill(s.highlight_color());
                if active {
                    tab.offset([1, 1]);
                }
            } else if disabled {
                s.fill(s.primary_color() / 2);
            } else if i == s.ui.current_tab(tab_id) {
                s.fill(s.highlight_color());
            } else {
                s.fill(s.primary_color());
            }
            let radius = 8;
            s.clip([tab.x(), tab.y(), tab.width() + 1, tab.height()])?;
            s.rounded_rect(
                rect![tab.top_left(), tab.width(), tab.height() + radius],
                radius,
            )?;
            s.pop();

            // Tab text
            s.rect_mode(RectMode::Center);
            s.clip(tab)?;
            s.set_cursor_pos(tab.center());
            s.text(tab_label)?;
            s.no_clip()?;

            s.ui.pop_cursor();
            s.pop();

            // Process input
            s.ui.handle_events(id);
            s.advance_cursor(tab);
            if !disabled && s.ui.was_clicked(id) {
                s.ui.set_current_tab(tab_id, i);
            }
        }

        let pos = s.cursor_pos();
        let fpad = s.theme.style.frame_pad;
        s.push();
        s.stroke(s.primary_color());
        let y = pos.y() - ipad.y();
        s.line(line_![fpad.x(), y, s.width()? as i32 - ipad.x(), y])?;
        s.pop();
        s.advance_cursor([0, 0, 0, fpad.y()]);

        f(tabs[s.ui.current_tab(tab_id)].as_ref(), s)?;

        Ok(())
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
        s.line(line_![pad.x(), y, s.width()? as i32 - pad.x(), y])?;

        s.pop();
        s.advance_cursor([0, 0, 0, height]);

        Ok(())
    }
}
