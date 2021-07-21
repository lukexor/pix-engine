//! Drawing functions.

use crate::{
    prelude::*,
    renderer::{Error as RendererError, Rendering},
};
use std::{borrow::Cow, iter::Iterator};

/// Default primitive type used for drawing.
pub type DrawPrimitive = i16;

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
        let (tx, ty): (Vec<i16>, Vec<i16>) = vertexes
            .iter()
            .map(|v| {
                let x = (v.x * cos - v.y * sin) * scale + p.x;
                let y = (v.x * sin + v.y * cos) * scale + p.y;
                (x as i16, y as i16)
            })
            .unzip();
        if tx.is_empty() || ty.is_empty() {
            Err(RendererError::Other(Cow::from("no vertexes to render")).into())
        } else {
            Ok(self.renderer.polygon(&tx, &ty, s.fill, s.stroke)?)
        }
    }
}
