//! Shape functions for drawing.

use crate::{prelude::*, renderer::Rendering};

#[macro_use]
pub mod ellipse;
#[macro_use]
pub mod line;
#[macro_use]
pub mod point;
#[macro_use]
pub mod rect;
pub mod quad;
#[macro_use]
pub mod sphere;
pub mod triangle;

pub use ellipse::*;
pub use line::*;
pub use point::*;
pub use quad::*;
pub use rect::*;
pub use sphere::*;
pub use triangle::*;

/// Trait for shape containing operations.
pub trait Contains {
    /// The generic type of Self::Shape.
    type Type;
    /// The shape type. e.g. [Rect<Self::Type>].
    type Shape;

    /// Returns whether this shape contains a given [Point].
    fn contains_point<P>(&self, _p: P) -> bool
    where
        P: Into<Point<Self::Type, 2>>,
    {
        unimplemented!("contains_point is not implemented")
    }

    /// Returns whether this shape completely contains another shape of the same type.
    fn contains_shape<O>(&self, _other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        unimplemented!("contains is not implemented")
    }
}

/// Trait for shape intersection operations.
pub trait Intersects {
    /// The generic type of Self::Shape.
    type Type;
    /// The shape type. e.g. [Rect<Self::Type>].
    type Shape;

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, _line: L) -> Option<(PointF2, Scalar)>
    where
        L: Into<Line<Self::Type, 2>>,
    {
        unimplemented!("intersects_line is not implemented")
    }

    /// Returns whether this shape intersects with another shape of the same type.
    fn intersects_shape<O>(&self, _other: O) -> bool
    where
        O: Into<Self::Shape>,
    {
        unimplemented!("intersects is not implemented")
    }
}

impl PixState {
    /// Draw a [Point] to the current canvas.
    pub fn point<P>(&mut self, p: P) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        if let Some(stroke) = self.settings.stroke {
            self.renderer.point(&p.into(), stroke)?;
        }
        Ok(())
    }

    /// Draw a [Line] to the current canvas.
    pub fn line<L>(&mut self, line: L) -> PixResult<()>
    where
        L: Into<LineI2>,
    {
        if let Some(stroke) = self.settings.stroke {
            self.renderer.line(&line.into(), stroke)?;
        }
        Ok(())
    }

    /// Draw a [Triangle][Tri] to the current canvas.
    pub fn triangle<T>(&mut self, tri: T) -> PixResult<()>
    where
        T: Into<TriI2>,
    {
        let s = &self.settings;
        Ok(self.renderer.triangle(&tri.into(), s.fill, s.stroke)?)
    }

    /// Draw a square [Rect] to the current canvas.
    pub fn square<R>(&mut self, square: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.rect(square)
    }

    /// Draw a rounded [Square](Rect) to the current canvas.
    pub fn rounded_square<R, T>(&mut self, square: R, radius: T) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
        T: Into<i32>,
    {
        self.rounded_rect(square, radius)
    }

    /// Draw a [Rectangle](Rect) to the current canvas.
    pub fn rect<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        let s = &self.settings;
        let mut rect = rect.into();
        if let DrawMode::Center = s.rect_mode {
            rect.center_on(rect.center());
        };
        Ok(self.renderer.rect(&rect, s.fill, s.stroke)?)
    }

    /// Draw a rounded [Rectangle](Rect) to the current canvas.
    pub fn rounded_rect<R, T>(&mut self, rect: R, radius: T) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
        T: Into<i32>,
    {
        let s = &self.settings;
        let mut rect = rect.into();
        if let DrawMode::Center = s.rect_mode {
            rect.center_on(rect.center());
        };
        Ok(self
            .renderer
            .rounded_rect(&rect, radius.into(), s.fill, s.stroke)?)
    }

    /// Draw a [Quadrilateral](Quad) to the current canvas.
    pub fn quad<Q>(&mut self, quad: Q) -> PixResult<()>
    where
        Q: Into<QuadI2>,
    {
        let s = &self.settings;
        Ok(self.renderer.quad(&quad.into(), s.fill, s.stroke)?)
    }

    /// Draw a polygon to the current canvas.
    pub fn polygon(&mut self, points: &[PointI2]) -> PixResult<()> {
        let s = &self.settings;
        Ok(self.renderer.polygon(points, s.fill, s.stroke)?)
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
        let mut ellipse = ellipse.into();
        if let DrawMode::Center = s.ellipse_mode {
            ellipse.center_on(ellipse.center());
        };
        Ok(self.renderer.ellipse(&ellipse, s.fill, s.stroke)?)
    }

    /// Draw an arc to the current canvas.
    pub fn arc<P, T>(&mut self, p: P, radius: T, start: T, end: T) -> PixResult<()>
    where
        P: Into<PointI2>,
        T: Into<i32>,
    {
        let s = &self.settings;
        let p = p.into();
        Ok(self.renderer.arc(
            &p,
            radius.into(),
            start.into(),
            end.into(),
            s.arc_mode,
            s.fill,
            s.stroke,
        )?)
    }
}
