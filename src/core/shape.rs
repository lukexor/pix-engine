//! Shape functions for drawing.

use crate::{prelude::*, renderer::Rendering};
use num_traits::{AsPrimitive, Float};

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

pub use ellipse::{Circle, Ellipse};
pub use line::Line;
pub use point::Point;
pub use quad::Quad;
pub use rect::Rect;
pub use sphere::Sphere;
pub use triangle::Triangle;

/// Trait for operations on a geometric shape.
pub trait Shape<T> {
    /// The shape type. e.g. [Rect<T>].
    type Item;

    /// Returns whether this shape contains a given [Point].
    fn contains_point<P: Into<Point<T>>>(&self, _p: P) -> bool {
        unimplemented!("contains_point is not implemented")
    }

    /// Returns whether this shape completely contains another shape of the same type.
    fn contains<O: Into<Self::Item>>(&self, _other: O) -> bool {
        unimplemented!("contains is not implemented")
    }

    /// Returns the closest intersection point with a given line and distance along the line or
    /// `None` if there is no intersection.
    fn intersects_line<L>(&self, _line: L) -> Option<(Point<T>, T)>
    where
        T: Float,
        L: Into<Line<T>>,
    {
        unimplemented!("intersects_line is not implemented")
    }

    /// Returns whether this shape intersects with another shape of the same type.
    fn intersects<O: Into<Self::Item>>(&self, _other: O) -> bool {
        unimplemented!("intersects is not implemented")
    }
}

impl PixState {
    /// Draw a [Point] to the current canvas.
    pub fn point<P: Into<Point>>(&mut self, p: P) -> PixResult<()> {
        if let Some(stroke) = self.settings.stroke {
            self.renderer.point(&p.into().round().as_(), stroke)?;
        }
        Ok(())
    }

    /// Draw a [Line] to the current canvas.
    pub fn line<L: Into<Line>>(&mut self, line: L) -> PixResult<()> {
        if let Some(stroke) = self.settings.stroke {
            self.renderer.line(&line.into().round().as_(), stroke)?;
        }
        Ok(())
    }

    /// Draw a [Triangle] to the current canvas.
    pub fn triangle<T: Into<Triangle>>(&mut self, tri: T) -> PixResult<()> {
        let s = &self.settings;
        Ok(self
            .renderer
            .triangle(&tri.into().round().as_(), s.fill, s.stroke)?)
    }

    /// Draw a [Square](Rect) to the current canvas.
    pub fn square<R: Into<Rect>>(&mut self, square: R) -> PixResult<()> {
        self.rect(square)
    }

    /// Draw a rounded [Square](Rect) to the current canvas.
    pub fn rounded_square<R: Into<Rect>, T: Into<Scalar>>(
        &mut self,
        square: R,
        radius: T,
    ) -> PixResult<()> {
        self.rounded_rect(square, radius)
    }

    /// Draw a [Rectangle](Rect) to the current canvas.
    pub fn rect<R: Into<Rect>>(&mut self, rect: R) -> PixResult<()> {
        let s = &self.settings;
        let rect = rect.into().round().as_();
        let rect = match s.rect_mode {
            DrawMode::Corner => rect,
            DrawMode::Center => {
                let x = rect.x - rect.width / 2;
                let y = rect.y - rect.height / 2;
                rect!(x, y, rect.width, rect.height)
            }
        };
        Ok(self.renderer.rect(&rect, s.fill, s.stroke)?)
    }

    /// Draw a rounded [Rectangle](Rect) to the current canvas.
    pub fn rounded_rect<R: Into<Rect>, T: Into<Scalar>>(
        &mut self,
        rect: R,
        radius: T,
    ) -> PixResult<()> {
        let s = &self.settings;
        let rect = rect.into().round().as_();
        let rect = match s.rect_mode {
            DrawMode::Corner => rect,
            DrawMode::Center => {
                let x = rect.x - rect.width / 2;
                let y = rect.y - rect.height / 2;
                rect!(x, y, rect.width, rect.height)
            }
        };
        Ok(self
            .renderer
            .rounded_rect(&rect, radius.into().round().as_(), s.fill, s.stroke)?)
    }

    /// Draw a [Quadrilateral](Quad) to the current canvas.
    pub fn quad<Q: Into<Quad>>(&mut self, quad: Q) -> PixResult<()> {
        let s = &self.settings;
        Ok(self
            .renderer
            .quad(&quad.into().round().as_(), s.fill, s.stroke)?)
    }

    /// Draw a polygon to the current canvas.
    pub fn polygon(&mut self, vx: &[Scalar], vy: &[Scalar]) -> PixResult<()> {
        let s = &self.settings;
        let vx: Vec<DrawPrimitive> = vx.iter().map(|x| x.round().as_()).collect();
        let vy: Vec<DrawPrimitive> = vy.iter().map(|y| y.round().as_()).collect();
        Ok(self.renderer.polygon(&vx, &vy, s.fill, s.stroke)?)
    }

    /// Draw a [Circle] to the current canvas.
    pub fn circle<C: Into<Circle>>(&mut self, circle: C) -> PixResult<()> {
        self.ellipse(circle.into())
    }

    /// Draw a [Ellipse] to the current canvas.
    pub fn ellipse<E: Into<Ellipse>>(&mut self, ellipse: E) -> PixResult<()> {
        let s = &self.settings;
        let ellipse = ellipse.into().round().as_();
        let ellipse = match s.ellipse_mode {
            DrawMode::Corner => ellipse,
            DrawMode::Center => {
                let width = ellipse.width;
                let height = ellipse.height;
                ellipse!(ellipse.x - width / 2, ellipse.y - height / 2, width, height)
            }
        };
        Ok(self.renderer.ellipse(&ellipse, s.fill, s.stroke)?)
    }

    /// Draw an arc to the current canvas.
    pub fn arc<P, T>(&mut self, p: P, radius: T, start: T, end: T) -> PixResult<()>
    where
        P: Into<Point>,
        T: Into<Scalar>,
    {
        let s = &self.settings;
        let p = p.into().round().as_();
        Ok(self.renderer.arc(
            &p,
            radius.into().round().as_(),
            start.into().round().as_(),
            end.into().round().as_(),
            s.arc_mode,
            s.fill,
            s.stroke,
        )?)
    }
}
