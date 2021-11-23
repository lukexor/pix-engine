//! Slider and drag widget rendering methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::drag`]
//! - [`PixState::advanced_drag`]
//! - [`PixState::slider`]
//! - [`PixState::advanced_slider`]
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { drag: i32, advanced_drag: f32, slider: i32, advanced_slider: f32};
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.drag("Drag", &mut self.drag, 1)?;
//!     s.advanced_drag(
//!         "Advanced Drag",
//!         &mut self.advanced_drag,
//!         0.005,
//!         0.0,
//!         1.0,
//!         Some(|val| format!("{:.3}", val).into()),
//!     )?;
//!     s.slider("Slider", &mut self.slider, -5, 5)?;
//!     s.advanced_slider(
//!         "Advanced Slider",
//!         &mut self.advanced_slider,
//!         0.0,
//!         1.0,
//!         Some(|val| format!("ratio = {:.3}", val).into()),
//!     )?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::{
    gui::{scroll::THUMB_MIN, MOD_CTRL},
    ops::clamp_size,
    prelude::*,
};
use num_traits::{clamp, Bounded, NumCast};
use std::{borrow::Cow, error::Error, fmt, str::FromStr};

impl PixState {
    /// Draw a draggable number widget to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { int: i32, float: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.drag("Drag Int", &mut self.int, 1)?;
    ///     s.drag("Drag Float", &mut self.float, 0.005)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn drag<T, L>(&mut self, label: L, value: &mut T, speed: T) -> PixResult<bool>
    where
        T: Num + NumCast + Bounded + fmt::Display + FromStr,
        <T as FromStr>::Err: Error + Sync + Send + 'static,
        L: AsRef<str>,
    {
        self.advanced_drag(label, value, speed, T::min_value(), T::max_value(), None)
    }

    /// Draw an advanced draggable number widget to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { advanced_drag: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.advanced_drag(
    ///         "Advanced Drag",
    ///         &mut self.advanced_drag,
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
        T: Num + NumCast + fmt::Display + FromStr,
        <T as FromStr>::Err: Error + Sync + Send + 'static,
        L: AsRef<str>,
    {
        let label = label.as_ref();
        let s = self;
        let id = s.ui.get_id(&label);
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let font_size = clamp_size(s.theme.sizes.body);
        let spacing = s.theme.spacing;
        let colors = s.theme.colors;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Calculate drag rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100) - 2 * fpad.x() as u32);
        let mut drag = rect![pos, clamp_size(width), font_size + 2 * ipad.y()];
        let (label_width, label_height) = s.size_of(label)?;
        if !label.is_empty() {
            drag.offset_x(clamp_size(label_width) + ipad.x());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &drag);
        let focused = s.ui.try_focus(id);
        let disabled = s.ui.disabled;
        let active = s.ui.is_active(id);

        // If editing, render editable text field instead
        let editing = s.ui.is_editing(id);
        if editing {
            if !focused || disabled {
                s.ui.end_edit();
            } else {
                s.next_width(width);
                let mut text = s.ui.text_edit(id, value.to_string());
                let changed = s.advanced_text_field(
                    label,
                    "",
                    &mut text,
                    Some(|c| c.is_ascii_digit() || c == '.' || c == '-'),
                )?;
                s.ui.set_text_edit(id, text);

                if let Some(Key::Return | Key::Escape) = s.ui.key_entered() {
                    s.ui.end_edit();
                }
                return Ok(changed);
            }
        }
        let mut new_value = clamp(s.ui.parse_text_edit(id, *value), min, max);

        s.push();
        s.ui.push_cursor();

        // Label
        s.rect_mode(RectMode::Corner);
        if hovered || active {
            s.frame_cursor(&Cursor::hand())?;
        }

        if !label.is_empty() {
            s.fill(colors.on_background());
            s.set_cursor_pos([
                pos.x(),
                pos.y() + drag.height() / 2 - clamp_size(label_height) / 2,
            ]);
            s.text(label)?;
        }

        // Drag region
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Primary);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(drag)?;

        // Value
        let text = formatter.map_or_else(|| value.to_string().into(), |f| f(value));
        let (vw, vh) = s.size_of(&text)?;
        let center = drag.center();
        let x = center.x() - clamp_size(vw) / 2;
        let y = center.y() - clamp_size(vh) / 2;
        s.set_cursor_pos([x, y]);
        s.no_stroke();
        s.fill(fg);
        s.text(&text)?;

        s.ui.pop_cursor();
        s.pop();

        // Process drag
        if active {
            if s.keymod_down(MOD_CTRL) {
                // Process keyboard input
                s.ui.begin_edit(id);
            } else {
                let delta = s.mouse_pos().x() - s.pmouse_pos().x();
                let mut delta: T = NumCast::from(delta).expect("valid i32 cast");
                if s.keymod_down(KeyMod::ALT) {
                    delta /= NumCast::from(10).expect("valid number cast");
                } else if s.keymod_down(KeyMod::SHIFT) {
                    delta *= NumCast::from(10).expect("valid number cast");
                }
                new_value = clamp(new_value + (delta * speed), min, max);
            }
        }
        s.ui.handle_events(id);
        s.advance_cursor(rect![pos, drag.right() - pos.x(), drag.height()]);
        if new_value == *value {
            Ok(false)
        } else {
            *value = new_value;
            Ok(true)
        }
    }

    /// Draw a slider widget to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { int: i32, float: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.slider("Slider Int", &mut self.int, -5, 5)?;
    ///     s.slider("Slider Float", &mut self.float, 0.0, 1.0)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn slider<T, L>(&mut self, label: L, value: &mut T, min: T, max: T) -> PixResult<bool>
    where
        T: Num + NumCast + fmt::Display + FromStr,
        <T as FromStr>::Err: Error + Sync + Send + 'static,
        L: AsRef<str>,
    {
        self.advanced_slider(label, value, min, max, None)
    }

    /// Draw an advanced slider widget to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Panics
    ///
    /// Panics if `value`, `min`, or `max` can not be cast to a floating point value.
    ///
    /// # Example
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { advanced_slider: f32};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.advanced_slider(
    ///         "Advanced Slider",
    ///         &mut self.advanced_slider,
    ///         0.0,
    ///         1.0,
    ///         Some(|val| format!("ratio = {:.3}", val).into()),
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn advanced_slider<'a, T, L>(
        &mut self,
        label: L,
        value: &mut T,
        min: T,
        max: T,
        formatter: Option<fn(&T) -> Cow<'a, str>>,
    ) -> PixResult<bool>
    where
        T: Num + NumCast + fmt::Display + FromStr,
        <T as FromStr>::Err: Error + Sync + Send + 'static,
        L: AsRef<str>,
    {
        let label = label.as_ref();
        let s = self;
        let id = s.ui.get_id(&label);
        let label = s.ui.get_label(label);
        let pos = s.cursor_pos();
        let font_size = clamp_size(s.theme.sizes.body);
        let spacing = s.theme.spacing;
        let colors = s.theme.colors;
        let fpad = spacing.frame_pad;
        let ipad = spacing.item_pad;

        // Calculate slider rect
        let width =
            s.ui.next_width
                .take()
                .unwrap_or_else(|| s.width().unwrap_or(100) - 2 * fpad.x() as u32);
        let mut slider = rect![pos, clamp_size(width), font_size + 2 * ipad.y()];
        let (label_width, label_height) = s.size_of(label)?;
        if !label.is_empty() {
            slider.offset_x(clamp_size(label_width) + ipad.x());
        }

        // Check hover/active/keyboard focus
        let hovered = s.ui.try_hover(id, &slider);
        let focused = s.ui.try_focus(id);
        let active = s.ui.is_active(id);

        // If editing, render editable text field instead
        let editing = s.ui.is_editing(id);
        let disabled = s.ui.disabled;
        if editing {
            if !focused || disabled {
                s.ui.end_edit();
            } else {
                s.next_width(width);
                let mut text = s.ui.text_edit(id, value.to_string());
                s.advanced_text_field(
                    label,
                    "",
                    &mut text,
                    Some(|c| c.is_ascii_digit() || c == '.' || c == '-'),
                )?;
                s.ui.set_text_edit(id, text);

                if let Some(Key::Return | Key::Escape) = s.ui.key_entered() {
                    s.ui.end_edit();
                }
                return Ok(false);
            }
        }
        let mut new_value = clamp(s.ui.parse_text_edit(id, *value), min, max);

        s.push();
        s.ui.push_cursor();

        // Label
        s.rect_mode(RectMode::Corner);
        if hovered | active {
            s.frame_cursor(&Cursor::hand())?;
        }

        if !label.is_empty() {
            s.fill(colors.on_background());
            s.set_cursor_pos([
                pos.x(),
                pos.y() + slider.height() / 2 - clamp_size(label_height) / 2,
            ]);
            s.text(label)?;
        }

        // Slider region
        let [stroke, bg, fg] = s.widget_colors(id, ColorType::Primary);
        s.stroke(stroke);
        s.fill(bg);
        s.rect(slider)?;

        // Scroll thumb
        s.no_stroke();
        s.fill(fg.blended(bg, 0.60));
        let slider_w = slider.width() as Scalar;
        let vmin: Scalar = NumCast::from(min).expect("valid number cast");
        let vmax: Scalar = NumCast::from(max).expect("valid number cast");
        let val: Scalar = NumCast::from(*value).expect("valid number cast");
        let thumb_w = if vmax - vmin > 1.0 {
            slider_w / (vmax - vmin)
        } else {
            THUMB_MIN as Scalar
        };
        let thumb_w = thumb_w.min(slider_w);
        let offset = ((val - vmin) / (vmax - vmin)) * (slider_w - thumb_w);
        let x = slider.x() + offset as i32;
        let thumb = rect![
            x + 1,
            slider.y() + 1,
            thumb_w as i32 - 2,
            slider.height() - 2
        ];
        s.rect(thumb)?;

        // Value
        let text = formatter.map_or_else(|| value.to_string().into(), |f| f(value));
        let (vw, vh) = s.size_of(&text)?;
        let center = slider.center();
        let x = center.x() - clamp_size(vw) / 2;
        let y = center.y() - clamp_size(vh) / 2;
        s.set_cursor_pos([x, y]);
        s.no_stroke();
        s.fill(fg);
        s.text(&text)?;

        s.ui.pop_cursor();
        s.pop();

        if active {
            if s.keymod_down(MOD_CTRL) {
                // Process keyboard input
                s.ui.begin_edit(id);
            } else {
                // Process mouse input
                let mx = (s.mouse_pos().x() - slider.x()).clamp(0, slider.width()) as Scalar
                    / slider.width() as Scalar;
                new_value = NumCast::from(mx.mul_add(vmax - vmin, vmin)).unwrap();
            }
        }
        s.ui.handle_events(id);
        s.advance_cursor(rect![pos, slider.right() - pos.x(), slider.height()]);

        if new_value == *value {
            Ok(false)
        } else {
            *value = new_value;
            Ok(true)
        }
    }
}
