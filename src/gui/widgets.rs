//! UI widget rendering methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::button`]
//! - [`PixState::checkbox`]
//! - [`PixState::radio`]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, radio: usize };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     if s.button("Button")? {
//!         // was clicked
//!     }
//!     s.checkbox("Checkbox", &mut self.checkbox)?;
//!     s.radio("Radio 1", &mut self.radio, 0)?;
//!     s.radio("Radio 2", &mut self.radio, 1)?;
//!     s.radio("Radio 3", &mut self.radio, 2)?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::prelude::*;

pub mod field;
pub mod select;
pub mod slider;
pub mod text;
pub mod tooltip;

const CHECKBOX_SIZE: i32 = 16;
const RADIO_SIZE: i32 = 8;

impl PixState {
    /// Draw a button to the current canvas that returns `true` when clicked.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     if s.button("Button")? {
    ///         // was clicked
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn button<L>(&mut self, label: L) -> PixResult<bool>
    where
        L: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let spacing = s.theme.spacing;
        let pad = spacing.item_pad;

        // Calculate button size
        let (mut width, height) = s.size_of(label)?;
        if let Some(next_width) = s.ui.next_width {
            width = next_width;
        }
        let mut button = rect![pos, width as i32 + 2 * pad.x(), height as i32 + 2 * pad.y()];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &button);
        s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            if active {
                button.offset([1, 1]);
            }
        }
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Primary);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(button)?;

        // Button text
        s.rect_mode(RectMode::Center);
        s.clip(button)?;
        s.set_cursor_pos(button.center());
        s.no_stroke();
        s.fill(fg);
        s.text(label)?;
        s.no_clip()?;

        s.ui.pop_cursor();
        s.pop();

        // Process input
        s.ui.handle_events(id);
        s.advance_cursor(button);
        Ok(!disabled && s.ui.was_clicked(id))
    }

    /// Draw a checkbox to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { checkbox: bool };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.checkbox("Checkbox", &mut self.checkbox)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn checkbox<S>(&mut self, label: S, checked: &mut bool) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let colors = s.theme.colors;

        // Calculate checkbox rect
        let checkbox = square![pos, CHECKBOX_SIZE];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &checkbox);
        s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();

        // Checkbox
        s.rect_mode(RectMode::Corner);
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        let [stroke, bg, fg] = if *checked {
            s.widget_colors(id, ColorType::Primary)
        } else {
            s.widget_colors(id, ColorType::Background)
        };
        s.stroke(stroke);
        s.fill(bg);
        s.rect(checkbox)?;

        if *checked {
            s.stroke(fg);
            s.stroke_weight(2);
            let half = CHECKBOX_SIZE / 2;
            let third = CHECKBOX_SIZE / 3;
            let x = checkbox.left() + half - 1;
            let y = checkbox.bottom() - third;
            let start = point![x - third + 2, y - third + 2];
            let mid = point![x, y];
            let end = point![x + third + 1, y - half + 2];
            s.line([start, mid])?;
            s.line([mid, end])?;
        }
        s.advance_cursor(checkbox);
        s.pop();

        // Label
        s.same_line(None);
        s.no_stroke();
        s.fill(colors.on_background());
        s.text(label)?;

        // Process input
        s.ui.handle_events(id);
        if disabled {
            Ok(false)
        } else {
            let clicked = s.ui.was_clicked(id);
            if clicked {
                *checked = !(*checked);
            }
            Ok(clicked)
        }
    }

    /// Draw a set of radio buttons to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { radio: usize };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.radio("Radio 1", &mut self.radio, 0)?;
    ///     s.radio("Radio 2", &mut self.radio, 1)?;
    ///     s.radio("Radio 3", &mut self.radio, 2)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn radio<S>(&mut self, label: S, selected: &mut usize, index: usize) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let label = label.as_ref();

        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let colors = s.theme.colors;

        // Calculate radio rect
        let radio = circle![pos + RADIO_SIZE, RADIO_SIZE];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &radio);
        s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();

        // Checkbox
        s.rect_mode(RectMode::Corner);
        s.ellipse_mode(EllipseMode::Center);
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        let is_selected = *selected == index;
        let [stroke, bg, _] = if is_selected {
            s.widget_colors(id, ColorType::Primary)
        } else {
            s.widget_colors(id, ColorType::Background)
        };
        if is_selected {
            s.stroke(bg);
            s.no_fill();
        } else {
            s.stroke(stroke);
            s.fill(bg);
        }
        s.circle(radio)?;

        if is_selected {
            s.stroke(bg);
            s.fill(bg);
            s.circle([radio.x(), radio.y(), radio.radius() - 3])?;
        }
        s.advance_cursor(radio.bounding_rect());
        s.pop();

        // Label
        s.same_line(None);
        s.no_stroke();
        s.fill(colors.on_background());
        s.text(label)?;

        // Process input
        s.ui.handle_events(id);
        if disabled {
            Ok(false)
        } else {
            let clicked = s.ui.was_clicked(id);
            if clicked {
                *selected = index;
            }
            Ok(clicked)
        }
    }
}
