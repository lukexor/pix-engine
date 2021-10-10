//! Immediiate-GUI functions related to rendering and interacting with text and inputs.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;

use super::get_hash;

const TEXT_CURSOR: &str = "│";

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<P, S>(&mut self, p: P, text: S) -> PixResult<()>
    where
        P: Into<PointI2>,
        S: AsRef<str>,
    {
        self.text_transformed(p, text, 0.0, None, None)
    }

    /// Draw transformed text to the current canvas, optionally rotated about a `center` by `angle`
    /// or `flipped`. `angle` can be in radians or degrees depending on [AngleMode].
    pub fn text_transformed<P, S, T, C, F>(
        &mut self,
        p: P,
        text: S,
        angle: T,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        P: Into<PointI2>,
        S: AsRef<str>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
        T: AsPrimitive<Scalar>,
    {
        self._text_transformed(
            p.into(),
            text.as_ref(),
            angle.as_(),
            center.into(),
            flipped.into(),
        )
    }

    fn _text_transformed(
        &mut self,
        mut p: PointI2,
        text: &str,
        mut angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
    ) -> PixResult<()> {
        let s = &self.settings;
        if let RectMode::Center = s.rect_mode {
            let (width, height) = self.renderer.size_of(text)?;
            p = point!(p.x() - width as i32 / 2, p.y() - height as i32 / 2);
        };
        if let AngleMode::Radians = s.angle_mode {
            angle = angle.to_degrees();
        };
        Ok(self
            .renderer
            .text(p, text, angle, center, flipped, s.fill)?)
    }

    /// Draw a text field to the current canvas.
    /// Affected by `PixState::same_line`.
    pub fn text_field<R, L>(&mut self, rect: R, label: L, value: &mut String) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
        L: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._text_field(rect, label.as_ref(), value)
    }

    fn _text_field(&mut self, rect: Rect<i32>, label: &str, value: &mut String) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&label);

        // Calculate input rect
        let mut input = rect;
        let pad = s.theme.padding;
        let same_line = s.ui_state.same_line;
        if !label.is_empty() {
            // Resize input area to fit label
            let (w, h) = s.size_of(label)?;
            if same_line {
                let offset = w as i32 + pad;
                input.set_x(input.x() + offset);
                input.set_width(input.width() - offset);
            } else {
                let offset = h as i32 + pad;
                input.set_y(input.y() + offset);
                input.set_height(input.height() - offset);
            }
        }

        // Check hover/active/keyboard focus
        let disabled = s.ui_state.disabled;
        if !disabled && input.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);
        let focused = !disabled && s.ui_state.is_focused(id);
        let hovered = s.ui_state.is_hovered(id);
        let active = !disabled && s.ui_state.is_active(id);

        s.push();
        let mut changed = false;

        // Render
        s.rect_mode(RectMode::Corner);
        s.renderer.font_family(&s.theme.fonts.body)?;

        // Label
        if !label.is_empty() {
            s.fill(s.text_color());
            if same_line {
                let (_, h) = s.size_of(label)?;
                let y = rect.center().y();
                s.text([rect.x(), y - h as i32 / 2], label)?;
            } else {
                s.text([rect.x(), rect.y()], label)?;
            }
        }

        // Input
        if focused || active {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(&Cursor::ibeam())?;
        }
        s.fill(s.primary_color());
        s.rect(input)?;

        // Text
        s.fill(s.text_color());
        s.clip(input)?;
        let (vw, vh) = s.size_of(&value)?;
        let (cw, _) = s.size_of(&TEXT_CURSOR)?;
        let mut x = input.x() + pad;
        let y = input.center().y() - vh as i32 / 2;
        let width = (vw + cw) as i32;
        if width > input.width() {
            x -= width - input.width();
        }
        s.text([x, y], &value)?;
        if focused && !s.ui_state.disabled && s.frame_count() >> 8 & 1 > 0 {
            let offset = 2; // Remove some left space of the text cursor
            s.text([x + vw as i32 - offset, y], &TEXT_CURSOR)?;
        }
        s.no_clip()?;

        s.pop();

        // Process input
        if focused {
            if let Some(key) = s.ui_state.key_entered() {
                match key {
                    Key::Backspace if !value.is_empty() => {
                        value.pop();
                        changed = true;
                    }
                    _ => (),
                }
            }
            if let Some(text) = s.ui_state.keys.typed.take() {
                value.push_str(&text);
                changed = true;
            }
        }
        s.ui_state.handle_input(id);

        Ok(changed)
    }
}
