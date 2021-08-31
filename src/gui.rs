//! Graphical User Interface

use crate::{prelude::*, renderer::Rendering};

mod button;

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<P>(&mut self, p: P, text: &str) -> PixResult<()>
    where
        P: Into<Point<Primitive>>,
    {
        let s = &self.settings;
        let mut p = p.into();
        if let DrawMode::Center = s.rect_mode {
            let (width, height) = self.renderer.size_of(text)?;
            p = point!(p.x() - width / 2, p.y() - height / 2);
        };
        Ok(self.renderer.text(&p, text, s.fill, s.stroke)?)
    }
}
