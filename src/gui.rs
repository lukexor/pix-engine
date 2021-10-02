//! Graphical User Interface

use num_traits::AsPrimitive;

use crate::{prelude::*, renderer::Rendering};

pub mod button;

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
}
