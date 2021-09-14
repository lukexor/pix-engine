//! Graphical User Interface

use crate::{prelude::*, renderer::Rendering};

pub mod button;

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<P>(&mut self, p: P, text: &str) -> PixResult<()>
    where
        P: Into<Point<i32>>,
    {
        let s = &self.settings;
        let mut p = p.into();
        if let DrawMode::Center = s.rect_mode {
            let (width, height) = self.renderer.size_of(text)?;
            p -= point!(width as i32 / 2, height as i32 / 2);
        };
        Ok(self.renderer.text(&p, text, s.fill, s.stroke)?)
    }
}
