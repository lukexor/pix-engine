//! Graphical User Interface

use crate::{prelude::*, renderer::Rendering};

pub mod button;

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<P>(&mut self, p: P, text: &str) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        self.text_transformed(p, text, 0.0, None, None)
    }

    /// Draw transformed text to the current canvas.
    pub fn text_transformed<P, C, F>(
        &mut self,
        p: P,
        text: &str,
        angle: Scalar,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        P: Into<PointI2>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
    {
        let s = &self.settings;
        let mut p = p.into();
        if let DrawMode::Center = s.rect_mode {
            let (width, height) = self.renderer.size_of(text)?;
            p -= point!(width as i32 / 2, height as i32 / 2);
        };
        Ok(self
            .renderer
            .text(p, text, angle, center.into(), flipped.into(), s.fill)?)
    }
}
