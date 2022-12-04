//! Tooltip widget rendering methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::help_marker`]
//! - [`PixState::tooltip`]
//! - [`PixState::advanced_tooltip`]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { select_box: usize };
//! # impl PixEngine for App {
//! fn on_update(&mut self, s: &mut PixState) -> Result<()> {
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
//!                 s.background(Color::CADET_BLUE);
//!                 s.bullet("Advanced tooltip")?;
//!                 Ok(())
//!             }
//!         )?;
//!     }
//!     Ok(())
//! }
//! # }
//! ```

use crate::{ops::clamp_dimensions, prelude::*};

impl PixState {
    /// Draw help marker text that, when hovered, displays a help box with text to the current
    /// canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.help_marker("Help marker icon w/ tooltip")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn help_marker<S>(&mut self, text: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let text = s.ui.get_label(text);
        let pos = s.cursor_pos();
        let spacing = s.theme.spacing;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Calculate hover area
        let marker = "?";
        let (marker_width, marker_height) = s.text_size(marker)?;
        let hover = rect![
            pos,
            marker_width + 2 * ipad.x(),
            marker_height + 2 * ipad.y()
        ];

        // Check hover/active/keyboard focus
        let hovered = s.focused() && s.ui.try_hover(id, &hover);
        let focused = s.focused() && s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();
        s.ui.push_cursor();

        // Marker outline
        s.rect_mode(RectMode::Corner);
        let [_, bg, fg] = s.widget_colors(id, ColorType::Background);
        s.disable(true);
        s.stroke(None);
        s.fill(bg);
        s.square(hover)?;

        // Marker
        s.rect_mode(RectMode::Center);
        s.set_cursor_pos([hover.center().x(), hover.center().y() - 3]);
        s.stroke(None);
        s.fill(fg);
        s.text(marker)?;
        if !disabled {
            s.disable(false);
        }

        // Tooltip
        if focused {
            let (text_width, text_height) = s.text_size(text)?;
            let text_width = text_width + 2 * fpad.x();
            let text_height = text_height + 2 * fpad.y();
            s.push_id(id);
            s.advanced_tooltip(
                text,
                rect![hover.bottom_right() - 10, text_width, text_height],
                |s: &mut PixState| {
                    let [stroke, bg, fg] = s.widget_colors(id, ColorType::Surface);
                    s.background(bg);

                    s.stroke(stroke);
                    s.fill(None);
                    s.rect([0, 0, text_width - 1, text_height - 1])?;

                    s.stroke(None);
                    s.fill(fg);
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
        s.ui.handle_focus(id);
        s.advance_cursor([hover.width(), hover.height() - ipad.y()]);

        Ok(())
    }

    /// Draw tooltip box at the mouse cursor with text to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text("Hover me")?;
    ///     if s.hovered() {
    ///         s.tooltip("Basic tooltip")?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn tooltip<S>(&mut self, text: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let text = s.ui.get_label(text);
        let spacing = s.theme.spacing;
        let pad = spacing.frame_pad;

        let (text_width, text_height) = s.text_size(text)?;
        let text_width = text_width + 2 * pad.x();
        let text_height = text_height + 2 * pad.y();

        // Render
        s.push_id(id);
        s.advanced_tooltip(
            text,
            rect![s.mouse_pos(), text_width, text_height],
            |s: &mut PixState| {
                let [stroke, bg, fg] = s.widget_colors(id, ColorType::Surface);
                s.background(bg);

                s.stroke(stroke);
                s.fill(None);
                s.rect([0, 0, text_width - 1, text_height - 1])?;

                s.stroke(None);
                s.fill(fg);
                s.text(text)?;
                Ok(())
            },
        )?;
        s.pop_id();

        Ok(())
    }

    /// Draw an advanced tooltip box at the mouse cursor to the current canvas. It accepts a
    /// closure that is passed [`&mut PixState`][`PixState`] which you can use to draw all the
    /// standard drawing primitives and change any drawing settings. Settings changed inside the
    /// closure will not persist.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { select_box: usize };
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.text("Hover me")?;
    ///     if s.hovered() {
    ///         s.advanced_tooltip(
    ///             "Tooltip",
    ///             rect![s.mouse_pos(), 200, 100],
    ///             |s: &mut PixState| {
    ///                 s.background(Color::CADET_BLUE);
    ///                 s.bullet("Advanced tooltip")?;
    ///                 Ok(())
    ///             },
    ///         )?;
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_tooltip<S, R, F>(&mut self, label: S, rect: R, f: F) -> Result<()>
    where
        S: AsRef<str>,
        R: Into<Rect<i32>>,
        F: FnOnce(&mut PixState) -> Result<()>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let pad = s.theme.spacing.frame_pad;

        s.rect_mode(RectMode::Corner);

        // Calculate rect
        let mut rect = s.get_rect(rect).offset([15, 15]);

        // Ensure rect stays inside window
        let (win_width, win_height) = s.window_dimensions()?;
        let (win_width, win_height) = clamp_dimensions(win_width, win_height);
        if rect.right() > win_width {
            let offset = (rect.right() - win_width) + pad.x();
            rect = rect.offset([-offset, 0]);
        }
        if rect.bottom() > win_height {
            let offset = (rect.bottom() - win_height) + pad.y();
            rect = rect.offset([0, -offset]);
            let mpos = s.mouse_pos();
            if rect.contains(mpos) {
                rect.set_bottom(mpos.y() - pad.y());
            }
        }

        let texture_id = s.get_or_create_texture(id, None, rect)?;
        s.ui.offset_mouse(rect.top_left());

        s.set_texture_target(texture_id)?;
        f(s)?;
        s.clear_texture_target();

        s.ui.clear_mouse_offset();

        Ok(())
    }
}
