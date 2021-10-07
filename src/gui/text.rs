//! Immediiate-GUI functions related to rendering and interacting with text and inputs.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;

use super::get_hash;

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
        let s = &self.settings;
        let mut p = p.into();
        let text = text.as_ref();
        if let RectMode::Center = s.rect_mode {
            let (width, height) = self.renderer.size_of(text)?;
            p.offset(-point!(width as i32 / 2, height as i32 / 2));
        };
        let mut angle: Scalar = angle.as_();
        if let AngleMode::Radians = s.angle_mode {
            angle = angle.to_degrees();
        };
        Ok(self
            .renderer
            .text(p, text, angle, center.into(), flipped.into(), s.fill)?)
    }

    /// Draw a text field to the current canvas.
    pub fn text_field<R>(&mut self, rect: R, label: &str, value: &mut String) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
    {
        let s = self;
        let rect = s.get_rect(rect);
        let id = get_hash(&rect);
        let mut changed = false;

        s.push();

        let (_, h) = s.size_of(label)?;
        let pad = s.theme.padding;

        let mut input = rect;
        input.set_y(input.y() + h as i32 + pad);

        // Check hover/active/keyboard focus
        if input.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);

        // Render

        // Label
        s.rect_mode(RectMode::Corner);
        s.renderer.font_family(&s.theme.fonts.body)?;
        s.fill(s.text_color());
        s.text([rect.x(), rect.y()], label)?;

        // Input
        let focused = s.ui_state.is_focused(id);
        let active = s.ui_state.is_active(id);
        if focused || active {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        let hovered = s.ui_state.is_hovered(id);
        if hovered {
            s.frame_cursor(&Cursor::ibeam())?;
        }
        s.fill(s.primary_color());
        s.rounded_rect(input, 3.0)?;

        // Text
        s.fill(s.text_color());
        s.text([input.x() + pad, input.y() + pad], &value)?;
        if focused && s.frame_count() >> 8 & 1 > 0 {
            let (w, _) = if value.is_empty() {
                (0, 0)
            } else {
                s.size_of(&value)?
            };
            s.text([input.x() + w as i32 + pad + 1, input.y() + pad], "_")?;
        }

        s.pop();

        // Process input
        s.ui_state.handle_tab(id);
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
        let clicked = !s.mouse_down(Mouse::Left) && hovered && active;
        if clicked {
            s.ui_state.focus(id);
        }
        s.ui_state.set_last(id);

        Ok(changed)
    }
}
