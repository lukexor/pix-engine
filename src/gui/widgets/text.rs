//! Text UI widgets.

use crate::{prelude::*, renderer::Rendering};
use std::cmp;

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        self.text_transformed(text, 0.0, None, None)
    }

    /// Draw bulleted text to the current canvas.
    pub fn bullet<S>(&mut self, text: S) -> PixResult<()>
    where
        S: AsRef<str>,
    {
        self.text("•")?;
        self.same_line(None);
        self.text_transformed(text, 0.0, None, None)
    }

    /// Draw transformed text to the current canvas, optionally rotated about a `center` by `angle`
    /// or `flipped`. `angle` can be in radians or degrees depending on [AngleMode].
    pub fn text_transformed<S, A, C, F>(
        &mut self,
        text: S,
        angle: A,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        S: AsRef<str>,
        A: Into<Option<Scalar>>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
    {
        let text = text.as_ref();
        let mut angle = angle.into();
        let center = center.into();
        let flipped = flipped.into();

        let s = self;
        let disabled = s.ui.disabled;
        let mut pos = s.cursor_pos();
        if let RectMode::Center = s.settings.rect_mode {
            let (width, height) = s.renderer.size_of(text)?;
            pos.offset([-(width as i32 / 2), -(height as i32 / 2)]);
        };
        if let AngleMode::Radians = s.settings.angle_mode {
            angle = angle.map(|a| a.to_degrees());
        };

        s.push();

        s.no_stroke();
        if disabled {
            s.fill(s.text_color() / 2);
        } else {
            s.fill(s.text_color());
        }

        let mut rect = rect![pos.x(), pos.y(), 0, 0];
        let mut y = pos.y();
        for line in text.split('\n') {
            s.renderer.text(
                point![rect.x(), y],
                line,
                s.settings.wrap_width,
                angle,
                center,
                flipped,
                s.settings.fill,
                s.settings.stroke_weight.saturating_sub(1),
            )?;
            let (w, h) = s.size_of(line)?;
            y += h as i32;
            rect.set_width(cmp::max(w as i32, rect.width()));
            rect.offset_height(h as i32);
        }

        s.pop();
        s.advance_cursor(rect);

        Ok(())
    }
}
