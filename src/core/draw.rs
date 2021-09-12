//! Drawing functions.

use crate::{prelude::*, renderer::Rendering};
use std::iter::Iterator;

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw shape to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()>;
}

impl PixState {
    /// Clears the render target to the current background [Color] set by [PixState::background].
    pub fn clear(&mut self) {
        let color = self.settings.background;
        self.renderer.set_draw_color(self.settings.background);
        self.renderer.clear();
        self.renderer.set_draw_color(color);
    }

    /// Draw a wireframe to the current canvas.
    pub fn wireframe<P>(
        &mut self,
        vertexes: &[Vector],
        p: P,
        angle: Scalar,
        scale: Scalar,
    ) -> PixResult<()>
    where
        P: Into<Vector>,
    {
        let s = &self.settings;
        let p = p.into();
        let (sin, cos) = angle.sin_cos();
        let vs = vertexes.iter().map(|v| {
            let x = (v.x() * cos - v.y() * sin) * scale + p.x();
            let y = (v.x() * sin + v.y() * cos) * scale + p.y();
            point!(x as i32, y as i32)
        });
        Ok(self.renderer.polygon(vs, s.fill, s.stroke)?)
    }
}
