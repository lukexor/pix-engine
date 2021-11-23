//! Shape methods for drawing.
//!
//! Methods for drawing and interacting with shapes such as points, lines, rectangles, etc.
//!
//! Provided traits:
//!
//! - [Contains]: Defines [`contains_point`] and [`contains_shape`].
//! - [Intersects]: Defines [`intersects_line`] and [`intersects_shape`].
//!
//! Provided [`PixState`] methods;
//!
//! - [`PixState::point`]: Draw a [Point] to the current canvas.
//! - [`PixState::line`]: Draw a [Line] to the current canvas.
//! - [`PixState::triangle`]: Draw a [Triangle][Tri] to the current canvas.
//! - [`PixState::square`]: Draw a square [Rect] to the current canvas.
//! - [`PixState::rounded_square`]: Draw a square [Rect] with rounded corners to the current canvas.
//! - [`PixState::rect`]: Draw a [Rect] to the current canvas.
//! - [`PixState::rounded_rect`]: Draw a [Rect] with rounded corners to the current canvas.
//! - [`PixState::quad`]: Draw a [Quad] to the current canvas.
//! - [`PixState::polygon`]: Draw a polygon defined by a set of [Point]s to the current canvas.
//! - [`PixState::wireframe`]: Draw a wireframe defined by a set vertexes to the current canvas.
//! - [`PixState::circle`]: Draw a circle [Ellipse] to the current canvas.
//! - [`PixState::ellipse`]: Draw an [Ellipse] to the current canvas.
//! - [`PixState::arc`]: Draw an arc to the current canvas.
//!
//! [`contains_point`]: Contains::contains_point
//! [`contains_shape`]: Contains::contains_shape
//! [`intersects_line`]: Intersects::intersects_line
//! [`intersects_shape`]: Intersects::intersects_shape

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

#[doc(inline)]
pub use ellipse::*;
#[doc(inline)]
pub use line::*;
#[doc(inline)]
pub use point::*;
#[doc(inline)]
pub use quad::*;
#[doc(inline)]
pub use rect::*;
#[doc(inline)]
pub use sphere::*;
#[doc(inline)]
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
    /// Draw a [Point] to the current canvas. [`PixState::stroke`] controls whether the point is
    /// drawn or not. [`PixState::stroke_weight`] and [`PixState::fill`] have no effect.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.stroke(Color::RED);
    ///     s.point(s.mouse_pos())?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn point<P>(&mut self, p: P) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        if let Some(stroke) = self.settings.stroke {
            self.renderer.point(p.into(), stroke)?;
        }
        Ok(())
    }

    /// Draw a [Line] to the current canvas. [`PixState::stroke`] controls whether the line is drawn
    /// or not. [`PixState::stroke_weight`] controls the line thickness. [`PixState::fill`] has no
    /// effect.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.stroke(Color::RED);
    ///     s.line([s.pmouse_pos(), s.mouse_pos()])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
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

    /// Draw a [Triangle][Tri] to the current canvas. [`PixState::fill`] and [`PixState::stroke`]
    /// control whether the triangle is filled or outlined.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.triangle(tri!([10, 0], [-5, 5], [5, 5]))?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn triangle<T>(&mut self, tri: T) -> PixResult<()>
    where
        T: Into<TriI2>,
    {
        let s = &self.settings;
        self.renderer.triangle(tri.into(), s.fill, s.stroke)
    }

    /// Draw a square [Rect] to the current canvas. [`PixState::fill`] and [`PixState::stroke`] control
    /// whether the square is filled or outlined. [`RectMode`] controls how the `(x, y)` position is
    /// interpreted.
    ///
    /// Alias for [`PixState::rect`].
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.square(square![s.mouse_pos(), 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[doc(alias = "rect")]
    pub fn square<R>(&mut self, square: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.rect(square)
    }

    /// Draw a rounded [Square](Rect) to the current canvas. [`PixState::fill`] and
    /// [`PixState::stroke`] control whether the square is filled or outlined. [`RectMode`] controls
    /// how the `(x, y)` position is interpreted.
    ///
    /// Alias for [`PixState::rounded_rect`].
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.rounded_square(square![s.mouse_pos(), 100], 10)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[doc(alias = "rounded_rect")]
    pub fn rounded_square<R>(&mut self, square: R, radius: i32) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        self.rounded_rect(square, radius)
    }

    /// Draw a [Rectangle](Rect) to the current canvas. [`PixState::fill`] and [`PixState::stroke`]
    /// control whether the rect is filled or outlined. [`RectMode`] controls how the `(x, y)`
    /// position is interpreted.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.rect(rect![s.mouse_pos(), 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn rect<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        let s = &self.settings;
        let rect = self.get_rect(rect);
        self.renderer.rect(rect, None, s.fill, s.stroke)
    }

    /// Draw a rounded [Rectangle](Rect) to the current canvas. [`PixState::fill`] and
    /// [`PixState::stroke`] control whether the rect is filled or outlined. [`RectMode`] controls how
    /// the `(x, y)` position is interpreted.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.rounded_rect(rect![s.mouse_pos(), 100, 100], 10)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn rounded_rect<R>(&mut self, rect: R, radius: i32) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        let s = &self.settings;
        let rect = self.get_rect(rect);
        self.renderer.rect(rect, Some(radius), s.fill, s.stroke)
    }

    /// Draw a [Quadrilateral](Quad) to the current canvas. [`PixState::fill`] and
    /// [`PixState::stroke`] control whether the quad is filled or outlined. [`RectMode`] has no
    /// effect.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.quad(quad![10, 20, 30, 10, 20, 25, 15, 15])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn quad<Q>(&mut self, quad: Q) -> PixResult<()>
    where
        Q: Into<QuadI2>,
    {
        let s = &self.settings;
        self.renderer.quad(quad.into(), s.fill, s.stroke)
    }

    /// Draw a polygon to the current canvas. [`PixState::fill`] and [`PixState::stroke`] control
    /// whether the polygon is filled or outlined. [`RectMode`] has no effect.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.polygon([[10, 10], [50, 20], [70, 30], [60, 50], [10, 50]])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn polygon<P, I>(&mut self, points: I) -> PixResult<()>
    where
        P: Into<PointI2>,
        I: IntoIterator<Item = P>,
    {
        let s = &self.settings;
        self.renderer
            .polygon(points.into_iter().map(Into::into), s.fill, s.stroke)
    }

    /// Draw a wireframe to the current canvas, translated to a given [Point] and optionally
    /// rotated by `angle` and `scaled`. [`PixState::fill`] and [`PixState::stroke`] control whether
    /// the wireframe is filled or outlined. `angle` can be in either radians or degrees based on
    /// [`AngleMode`].
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let wireframe = [
    ///         point!(5.0, 0.0),
    ///         point!(-2.5, -2.5),
    ///         point!(-2.5, 2.5)
    ///     ];
    ///     s.fill(Color::CADET_BLUE);
    ///     s.stroke(Color::BLACK);
    ///     s.angle_mode(AngleMode::Degrees);
    ///     s.wireframe(wireframe, [10, 10], 45.0, 2.0)?;
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
            // rotation * scale + translation
            let x = (v.x() * cos - v.y() * sin).mul_add(scale, px).round() as i32;
            let y = (v.x().mul_add(sin, v.y() * cos)).mul_add(scale, py).round() as i32;
            point![x, y]
        });
        self.polygon(vs)
    }

    /// Draw a circle [Ellipse] to the current canvas. [`PixState::fill`] and [`PixState::stroke`]
    /// control whether the circle is filled or outlined. [`EllipseMode`] controls how the `(x, y)`
    /// position is interpreted.
    ///
    /// Alias for [`PixState::ellipse`].
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.circle(circle![s.mouse_pos(), 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[doc(alias = "ellipse")]
    pub fn circle<C>(&mut self, circle: C) -> PixResult<()>
    where
        C: Into<Ellipse<i32>>,
    {
        self.ellipse(circle)
    }

    /// Draw a [Ellipse] to the current canvas. [`PixState::fill`] and [`PixState::stroke`] control
    /// whether the ellipse is filled or outlined. [`EllipseMode`] controls how the `(x, y)` position
    /// is interpreted.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.ellipse(ellipse![s.mouse_pos(), 100, 100])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn ellipse<E>(&mut self, ellipse: E) -> PixResult<()>
    where
        E: Into<Ellipse<i32>>,
    {
        let s = &self.settings;
        let ellipse = self.get_ellipse(ellipse);
        self.renderer.ellipse(ellipse, s.fill, s.stroke)
    }

    /// Draw an arc of a given `radius` and length defined by `start` and `end` to the current
    /// canvas. [`PixState::fill`] and [`PixState::stroke`] control whether the pie is filled or
    /// outlined. [`ArcMode`] changes whether the arc is drawn as an open segment or a pie shape.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.fill(Color::BLACK);
    ///     s.stroke(Color::RED);
    ///     s.arc_mode(ArcMode::Pie);
    ///     s.arc(s.mouse_pos(), 20, 0, 180)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
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
