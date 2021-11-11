//! Slider and drag widget rendering methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::drag]
//! - [PixState::advanced_drag]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { drag_int: i32, drag_float: f32};
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.drag("Drag Int", &mut self.drag_int, 1)?;
//!     s.drag("Drag Float", &mut self.drag_float, 0.005)?;
//!     s.advanced_drag(
//!         "Advanced Drag Int",
//!         &mut self.drag_int,
//!         1,
//!         0,
//!         100,
//!         None,
//!     )?;
//!     s.advanced_drag(
//!         "Advanced Drag Float",
//!         &mut self.drag_float,
//!         0.005,
//!         0.0,
//!         1.0,
//!         Some(|val| format!("{:.3}", val).into()),
//!     )?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::prelude::*;
use num_traits::{clamp, Bounded, NumCast};
use std::{borrow::Cow, fmt};

impl PixState {
    /// Draw a draggable number widget to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { drag_int: i32, drag_float: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.drag("Drag Int", &mut self.drag_int, 1)?;
    ///     s.drag("Drag Float", &mut self.drag_float, 0.005)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn drag<T, L>(&mut self, label: L, value: &mut T, speed: T) -> PixResult<bool>
    where
        T: Num + NumCast + Bounded + fmt::Display,
        L: AsRef<str>,
    {
        self.advanced_drag(label, value, speed, T::min_value(), T::max_value(), None)
    }

    /// Draw an advanced draggable number widget to the current canvas.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { drag_int: i32, drag_float: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.advanced_drag(
    ///         "Advanced Drag Int",
    ///         &mut self.drag_int,
    ///         1,
    ///         0,
    ///         100,
    ///         None,
    ///     )?;
    ///     s.advanced_drag(
    ///         "Advanced Drag Float",
    ///         &mut self.drag_float,
    ///         0.005,
    ///         0.0,
    ///         1.0,
    ///         Some(|val| format!("{:.3}", val).into()),
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_drag<'a, T, L>(
        &mut self,
        label: L,
        value: &mut T,
        speed: T,
        min: T,
        max: T,
        formatter: Option<fn(&T) -> Cow<'a, str>>,
    ) -> PixResult<bool>
    where
        T: Num + NumCast + fmt::Display,
        L: AsRef<str>,
    {
        let label = label.as_ref();
        let s = self;
        let id = s.ui.get_id(&label);
        let label = label.split('#').next().unwrap_or("");
        let pos = s.cursor_pos();
        let font_size = s.theme.font_sizes.body as i32;
        let style = s.theme.style;
        let fpad = style.frame_pad;
        let ipad = style.item_pad;

        // Calculate drag rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100) - 2 * fpad.x() as u32);
        let mut drag = rect![pos, width as i32, font_size + 2 * ipad.y()];
        let (lwidth, lheight) = s.size_of(label)?;
        if !label.is_empty() {
            drag.offset_x(lwidth as i32 + ipad.x());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, drag);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        s.push();
        s.ui.push_cursor();

        // Render
        s.rect_mode(RectMode::Corner);

        // Label
        if !label.is_empty() {
            s.set_cursor_pos([pos.x(), pos.y() + drag.height() / 2 - lheight as i32 / 2]);
            s.text(label)?;
        }

        // Rect
        s.push();
        if focused || active {
            s.stroke(s.highlight_color());
        } else {
            s.stroke(s.muted_color());
        }
        if active {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.highlight_color());
        } else if hovered {
            s.frame_cursor(Cursor::hand())?;
            s.fill(s.secondary_color());
        } else if disabled {
            s.fill(s.primary_color() / 2);
        } else {
            s.fill(s.primary_color());
        }
        s.same_line(None);
        s.rect(drag)?;
        s.pop();

        // Value
        let text = if let Some(formatter) = formatter {
            formatter(value)
        } else {
            format!("{}", value).into()
        };
        let (vw, vh) = s.size_of(&text)?;
        let center = drag.center();
        let x = center.x() - vw as i32 / 2;
        let y = center.y() - vh as i32 / 2;
        s.set_cursor_pos([x, y]);
        s.text(&text)?;

        s.ui.pop_cursor();
        s.pop();

        // Process drag
        let mut changed = false;
        let mut new_value = *value;
        if active {
            let delta = s.mouse_pos().x() - s.pmouse_pos().x();
            let mut delta: T = NumCast::from(delta).expect("valid i32 cast");
            if s.keymod_down(KeyMod::ALT) {
                delta /= NumCast::from(100).expect("valid number cast");
            } else if s.keymod_down(KeyMod::SHIFT) {
                delta *= NumCast::from(10).expect("valid number cast");
            }
            new_value = clamp(new_value + (delta * speed), min, max);
        }
        if new_value != *value {
            *value = new_value;
            changed = true;
        }
        s.ui.handle_events(id);

        s.advance_cursor(rect![pos, drag.right() - pos.x(), drag.height()]);

        Ok(changed)
    }
}
