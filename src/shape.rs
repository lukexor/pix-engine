//! Shape functions for drawing.

use crate::{prelude::*, renderer::Rendering};
use std::iter::Iterator;

#[macro_use]
pub mod ellipse;
#[macro_use]
pub mod line;
#[macro_use]
pub mod point;
#[macro_use]
pub mod rect;
#[macro_use]
pub mod quad;
#[macro_use]
pub mod sphere;
#[macro_use]
pub mod triangle;

pub use ellipse::*;
pub use line::*;
pub use point::*;
pub use quad::*;
pub use rect::*;
pub use sphere::*;
pub use triangle::*;

/// Trait for shape containing operations.
pub trait Contains<T, const N: usize> {
    /// The shape type. e.g. [`Rect<T>`].
    type Shape;

    /// Returns whether this shape contains a given [Point].
    fn contains_point<P>(&self, _p: P) -> bool
    where
        P: Into<Point<T, N>>;

    /// Returns whether this shape completely contains another shape of the same type.
    fn contains_shape<O>(&self, _other: O) -> bool
    where
        O: Into<Self::Shape>;
}

/// Trait for shape intersection operations.
pub trait Intersects<T, const N: usize> {
    /// The shape type. e.g. [`Rect<T>`].
    type Shape;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, _line: L) -> Option<(Point<T, N>, T)>
    where
        L: Into<Line<T, N>>;

    /// Returns whether this shape intersects with another shape of the same type.
    fn intersects_shape<O>(&self, _other: O) -> bool
    where
        O: Into<Self::Shape>;
}

impl PixState {
    /// Draw a [Point] to the current canvas.
    pub fn point<P>(&mut self, p: P) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        if let Some(stroke) = self.settings.stroke {
            self.renderer.point(p.into(), stroke)?;
        }
        Ok(())
    }

    /// Draw a [Line] to the current canvas.
    pub fn line<L>(&mut self, line: L) -> PixResult<()>
    where
        L: Into<LineI2>,
    {
        let s = &self.settings;
        if let Some(stroke) = s.stroke {
            self.renderer.line(line.into(), s.stroke_weight, stroke)?;
        }
        Ok(())
    }

    /// Draw a [Triangle][Tri] to the current canvas.
    pub fn triangle<T>(&mut self, tri: T) -> PixResult<()>
    where
        T: Into<TriI2>,
    {
        let s = &self.settings;
        self.renderer.triangle(tri.into(), s.fill, s.stroke)
    }

    /// Draw a square [Rect] to the current canvas.
    pub fn square<R>(&mut self, square: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.rect(square)
    }

    /// Draw a rounded [Square](Rect) to the current canvas.
    pub fn rounded_square<R>(&mut self, square: R, radius: i32) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.rounded_rect(square, radius)
    }

    /// Draw a [Rectangle](Rect) to the current canvas.
    pub fn rect<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        let s = &self.settings;
        let rect = self.get_rect(rect);
        self.renderer.rect(rect, None, s.fill, s.stroke)
    }

    /// Draw a rounded [Rectangle](Rect) to the current canvas.
    pub fn rounded_rect<R>(&mut self, rect: R, radius: i32) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        let s = &self.settings;
        let rect = self.get_rect(rect);
        self.renderer.rect(rect, Some(radius), s.fill, s.stroke)
    }

    /// Draw a [Quadrilateral](Quad) to the current canvas.
    pub fn quad<Q>(&mut self, quad: Q) -> PixResult<()>
    where
        Q: Into<QuadI2>,
    {
        let s = &self.settings;
        self.renderer.quad(quad.into(), s.fill, s.stroke)
    }

    /// Draw a polygon to the current canvas.
    pub fn polygon<P, I>(&mut self, points: I) -> PixResult<()>
    where
        P: Into<PointI2>,
        I: IntoIterator<Item = P>,
    {
        let s = &self.settings;
        self.renderer
            .polygon(points.into_iter().map(|p| p.into()), s.fill, s.stroke)
    }

    /// Draw a wireframe to the current canvas, translated to a given [Point]
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.background(CADET_BLUE);
    ///     s.clear();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn wireframe<V, P1, P2, A, S>(
        &mut self,
        vertexes: V,
        pos: P2,
        angle: A,
        scale: S,
    ) -> PixResult<()>
    where
        V: IntoIterator<Item = P1>,
        P1: Into<PointF2>,
        P2: Into<PointI2>,
        A: Into<Option<Scalar>>,
        S: Into<Option<Scalar>>,
    {
        let s = &self.settings;
        let pos = pos.into();
        let mut angle = angle.into().unwrap_or(0.0);
        if let AngleMode::Degrees = s.angle_mode {
            angle = angle.to_radians();
        };
        let scale = scale.into().unwrap_or(1.0);
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

    /// Draw a circle [Ellipse] to the current canvas.
    pub fn circle<C>(&mut self, circle: C) -> PixResult<()>
    where
        C: Into<Ellipse<i32>>,
    {
        self.ellipse(circle)
    }

    /// Draw a [Ellipse] to the current canvas.
    pub fn ellipse<E>(&mut self, ellipse: E) -> PixResult<()>
    where
        E: Into<Ellipse<i32>>,
    {
        let s = &self.settings;
        let ellipse = self.get_ellipse(ellipse);
        self.renderer.ellipse(ellipse, s.fill, s.stroke)
    }

    /// Draw an arc to the current canvas.
    pub fn arc<P>(&mut self, p: P, radius: i32, start: i32, end: i32) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        let s = &self.settings;
        let p = p.into();
        self.renderer
            .arc(p, radius, start, end, s.arc_mode, s.fill, s.stroke)
    }
}
