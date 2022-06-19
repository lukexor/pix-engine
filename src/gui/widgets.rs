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

use crate::{gui::Direction, ops::clamp_size, prelude::*};

pub mod field;
pub mod select;
pub mod slider;
pub mod text;
pub mod tooltip;

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
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let fpad = s.theme.spacing.frame_pad;

        // Calculate button size
        let (label_width, label_height) = s.text_size(label)?;
        let width = s.ui.next_width.take().unwrap_or(label_width);
        let button = rect![pos, width, label_height].offset_size(2 * fpad);

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
        }
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Primary);
        s.stroke(stroke);
        s.fill(bg);
        if active {
            s.rect(button.offset([1, 1]))?;
        } else {
            s.rect(button)?;
        }

        // Button text
        s.rect_mode(RectMode::Center);
        s.clip(button)?;
        s.set_cursor_pos(button.center());
        s.stroke(None);
        s.fill(fg);
        s.text(label)?;
        s.clip(None)?;

        s.ui.pop_cursor();
        s.pop();

        // Process input
        s.ui.handle_focus(id);
        s.advance_cursor(button.size());
        Ok(!disabled && s.ui.was_clicked(id))
    }

    /// Draw a text link to the current canvas that returns `true` when clicked.
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
    ///     if s.link("Link")? {
    ///         // was clicked
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn link<S>(&mut self, text: S) -> PixResult<bool>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let s = self;
        let id = s.ui.get_id(&text);
        let text = s.ui.get_label(text);
        let pos = s.cursor_pos();
        let pad = s.theme.spacing.item_pad;

        // Calculate button size
        let (width, height) = s.text_size(text)?;
        let bounding_box = rect![pos, width, height].grow(pad / 2);

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &bounding_box);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();

        // Render
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
        }
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Primary);
        if focused {
            s.stroke(stroke);
            s.fill(None);
            s.rect(bounding_box)?;
        }

        // Button text
        s.stroke(None);
        if active {
            s.fill(fg.blended(bg, 0.04));
        } else {
            s.fill(bg);
        }
        s.text(text)?;

        s.pop();

        // Process input
        s.ui.handle_focus(id);
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
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let (_, checkbox_size) = s.text_size(label)?;

        // Calculate checkbox rect
        let checkbox = square![pos, checkbox_size];

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
            let half = checkbox_size / 2;
            let third = checkbox_size / 3;
            let x = checkbox.left() + half - 1;
            let y = checkbox.bottom() - third + 1;
            let start = point![x - third + 2, y - third + 2];
            let mid = point![x, y];
            let end = point![x + third + 1, y - half + 2];
            s.line([start, mid])?;
            s.line([mid, end])?;
        }
        s.advance_cursor(checkbox.size());
        s.pop();

        // Label
        s.same_line(None);
        s.text(label)?;

        // Process input
        s.ui.handle_focus(id);
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
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let (_, label_height) = s.text_size(label)?;
        let radio_size = label_height / 2;

        // Calculate radio rect
        let radio = circle![pos + radio_size, radio_size];

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &radio);
        s.ui.try_focus(id);
        let disabled = s.ui.disabled;

        s.push();

        // Radio
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
            s.fill(None);
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
        s.advance_cursor(radio.bounding_rect().size());
        s.pop();

        // Label
        s.same_line(None);
        s.text(label)?;

        // Process input
        s.ui.handle_focus(id);
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

    /// Render an arrow aligned with the current font size.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    pub fn arrow<P, S>(&mut self, pos: P, direction: Direction, scale: S) -> PixResult<()>
    where
        P: Into<Point<i32>>,
        S: Into<f64>,
    {
        let pos: Point<f64> = pos.into().as_();
        let scale = scale.into();

        let s = self;
        let font_size = clamp_size(s.theme.font_size);

        let height = f64::from(font_size);
        let mut ratio = height * 0.4 * scale;
        let center = pos + point![height * 0.5, height * 0.5 * scale];

        if let Direction::Up | Direction::Left = direction {
            ratio = -ratio;
        }
        let (p1, p2, p3) = match direction {
            Direction::Up | Direction::Down => (
                point![0.0, 0.75],
                point![-0.866, -0.75],
                point![0.866, -0.75],
            ),
            Direction::Left | Direction::Right => (
                point![0.75, 0.0],
                point![-0.75, 0.866],
                point![-0.75, -0.866],
            ),
        };

        s.triangle([
            (center + p1 * ratio).round().as_(),
            (center + p2 * ratio).round().as_(),
            (center + p3 * ratio).round().as_(),
        ])?;

        Ok(())
    }
}
