//! Drawing functions.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;
use std::iter::Iterator;

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw shape to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()>;
}

impl PixState {
    /// Clears the render target to the current background [Color] set by [PixState::background].
    #[inline]
    pub fn clear(&mut self) -> PixResult<()> {
        let color = self.settings.background;
        self.renderer.set_draw_color(self.settings.background)?;
        self.renderer.clear()?;
        self.renderer.set_draw_color(color)?;
        Ok(())
    }

    /// Draw a wireframe to the current canvas., translated to a given [Point]
    pub fn wireframe<V, P1, P2, T>(
        &mut self,
        vertexes: V,
        pos: P2,
        angle: T,
        scale: T,
    ) -> PixResult<()>
    where
        P1: Into<PointF2>,
        P2: Into<PointI2>,
        V: IntoIterator<Item = P1>,
        T: AsPrimitive<Scalar>,
    {
        let s = &self.settings;
        let pos = pos.into();
        let scale = scale.as_();
        let mut angle = angle.as_();
        if let AngleMode::Degrees = s.angle_mode {
            angle = angle.to_radians();
        };
        let (sin, cos) = angle.sin_cos();
        let (px, py) = (pos.x() as Scalar, pos.y() as Scalar);
        let vs = vertexes.into_iter().map(|v| {
            let v = v.into();
            let x = ((v.x() * cos - v.y() * sin) * scale + px).round() as i32;
            let y = ((v.x() * sin + v.y() * cos) * scale + py).round() as i32;
            point![x, y]
        });
        self.polygon(vs)
    }
}
