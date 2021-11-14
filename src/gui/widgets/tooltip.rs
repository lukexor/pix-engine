//! Tooltip widget rendering methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::help_marker]
//! - [PixState::tooltip]
//! - [PixState::advanced_tooltip]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { select_box: usize };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.help_marker("Help marker icon w/ tooltip")?;
//!
//!     s.text("Hover me")?;
//!     if s.hovered() {
//!         s.tooltip("Basic tooltip")?;
//!     }
//!
//!     s.text("Hover me too!")?;
//!     if s.hovered() {
//!         s.advanced_tooltip(
//!             "Advanced tooltip",
//!             rect![s.mouse_pos(), 200, 100],
//!             |s: &mut PixState| {
//!                 s.background(CADET_BLUE);
//!                 s.font_color(BLACK);
//!                 s.bullet("Advanced tooltip")?;
//!                 Ok(())
//!             }
//!         )?;
//!     }
//!     Ok(())
//! }
//! # }
//! ```

use crate::prelude::*;

impl PixState {
    /// Draw help marker text that, when hovered, displays a help box with text to the current
    /// canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.help_marker("Help marker icon w/ tooltip")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn help_marker<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let pos = s.cursor_pos();
        let style = s.theme.style;
        let pad = style.frame_pad;

        // Calculate hover area
        let marker = "?";
        let (w, h) = s.size_of(marker)?;
        let hover = square![pos, h as i32];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, hover);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);
        s.no_fill();
        if focused {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.text_color() / 2);
        }
        s.square(hover)?;

        // Marker
        s.no_stroke();
        s.disable();
        s.set_cursor_pos([pos.x() + w as i32, pos.y()]);
        s.text(marker)?;
        if !disabled {
            s.no_disable();
        }

        // Tooltip
        if focused {
            let (w, h) = s.size_of(text)?;
            let w = w as i32 + 2 * pad.x();
            let h = h as i32 + 2 * pad.y();
            s.push_id(id);
            s.advanced_tooltip(
                text,
                rect![hover.bottom_right() - 10, w, h],
                |s: &mut PixState| {
                    s.background(s.primary_color())?;
                    s.rect_mode(RectMode::Corner);
                    s.push();
                    s.stroke(s.muted_color());
                    s.no_fill();
                    s.rect([0, 0, w - 1, h - 1])?;
                    s.pop();
                    s.text(text)?;
                    Ok(())
                },
            )?;
            s.pop_id();
        } else if hovered {
            s.tooltip(text)?;
        }

        s.ui.pop_cursor();
        s.pop();

        // Process input
        s.ui.handle_events(id);
        s.advance_cursor(rect![pos, hover.right() - pos.x(), hover.height()]);

        Ok(())
    }

    /// Draw tooltip box at the mouse cursor with text to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Hover me")?;
    ///     if s.hovered() {
    ///         s.tooltip("Basic tooltip")?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn tooltip<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let style = s.theme.style;
        let pad = style.frame_pad;

        let (w, h) = s.size_of(text)?;
        let w = w as i32 + 2 * pad.x();
        let h = h as i32 + 2 * pad.y();

        // Render
        s.push_id(id);
        s.advanced_tooltip(text, rect![s.mouse_pos(), w, h], |s: &mut PixState| {
            s.background(s.primary_color())?;
            s.rect_mode(RectMode::Corner);
            s.push();
            s.stroke(s.muted_color());
            s.no_fill();
            s.rect([0, 0, w - 1, h - 1])?;
            s.pop();
            s.text(text)?;
            Ok(())
        })?;
        s.pop_id();

        Ok(())
    }

    /// Draw an advanced tooltip box at the mouse cursor to the current canvas. It accepts a
    /// closure that is passed [`&mut PixState`][PixState] which you can use to draw all the
    /// standard drawing primitives and change any drawing settings. Settings changed inside the
    /// closure will not persist, similar to [PixState::with_texture].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.text("Hover me")?;
    ///     if s.hovered() {
    ///         s.advanced_tooltip(
    ///             "Tooltip",
    ///             rect![s.mouse_pos(), 200, 100],
    ///             |s: &mut PixState| {
    ///                 s.background(CADET_BLUE);
    ///                 s.font_color(BLACK);
    ///                 s.bullet("Advanced tooltip")?;
    ///                 Ok(())
    ///             },
    ///         )?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_tooltip<S, R, F>(&mut self, label: S, rect: R, f: F) -> PixResult<()>
    where
        S: AsRef<str>,
        R: Into<Rect<i32>>,
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let pad = s.theme.style.frame_pad;

        // Calculate rect
        let mut rect = s.get_rect(rect);
        rect.offset([15, 15]);

        // Ensure rect stays inside window
        let (win_width, win_height) = s.window_dimensions()?;
        if rect.right() > win_width as i32 {
            let offset = (rect.right() - win_width as i32) + pad.x();
            rect.offset_x(-offset);
        }
        if rect.bottom() > win_height as i32 {
            let offset = (rect.bottom() - win_height as i32) + pad.y();
            rect.offset_y(-offset);
            let mpos = s.mouse_pos();
            if rect.contains_point(mpos) {
                rect.set_bottom(mpos.y() - pad.y());
            }
        }

        let texture_id = s.get_or_create_texture(id, None, rect)?;
        s.ui.set_mouse_offset(rect.top_left());
        s.with_texture(texture_id, f)?;
        s.ui.clear_mouse_offset();

        Ok(())
    }
}
