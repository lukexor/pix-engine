//! Draw functions.

use crate::{
    prelude::*,
    renderer::{Error as RendererError, Rendering},
};
use num_traits::Float;
use std::{borrow::Cow, iter::Iterator};

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw shape to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()>;
}

impl PixState {
    /// Draw the `Texture` to the current canvas.
    pub fn texture<R>(&mut self, texture_id: usize, src: R, dst: R) -> PixResult<()>
    where
        R: Into<Option<Rect<Scalar>>>,
    {
        Ok(self.renderer.texture(texture_id, src, dst)?)
    }

    /// Draw text to the current canvas.
    pub fn text(&mut self, p: impl Into<Point<Scalar>>, text: impl AsRef<str>) -> PixResult<()> {
        let s = &self.settings;
        let p = p.into();
        let p = match s.rect_mode {
            DrawMode::Corner => p,
            DrawMode::Center => {
                let height = s.text_size as Scalar;
                let width = text.as_ref().len() as Scalar * height;
                point!(p.x - width / 2.0, p.y - height / 2.0)
            }
        };
        Ok(self.renderer.text(p, text, s.text_size, s.fill, s.stroke)?)
    }

    /// Draw a [`Point`] to the current canvas.
    pub fn point(&mut self, p: impl Into<Point<Scalar>>) -> PixResult<()> {
        Ok(self.renderer.point(p, self.settings.stroke)?)
    }

    /// Draw a line to the current canvas.
    pub fn line(&mut self, line: impl Into<Line<Scalar>>) -> PixResult<()> {
        Ok(self.renderer.line(line, self.settings.stroke)?)
    }

    /// Draw a triangle to the current canvas.
    pub fn triangle(&mut self, tri: impl Into<Triangle<Scalar>>) -> PixResult<()> {
        let s = &self.settings;
        Ok(self.renderer.triangle(tri, s.fill, s.stroke)?)
    }

    /// Draw a square to the current canvas.
    pub fn square(&mut self, square: impl Into<Rect<Scalar>>) -> PixResult<()> {
        self.rect(square)
    }

    /// Draw a rectangle to the current canvas.
    pub fn rect(&mut self, rect: impl Into<Rect<Scalar>>) -> PixResult<()> {
        let s = &self.settings;
        let rect = rect.into();
        let rect = match s.rect_mode {
            DrawMode::Corner => rect,
            DrawMode::Center => {
                let [x, y, width, height]: [Scalar; 4] = rect.into();
                rect!(x - width / 2.0, y - height / 2.0, width, height)
            }
        };
        Ok(self.renderer.rect(rect, s.fill, s.stroke)?)
    }

    /// Draw a polygon to the current canvas.
    pub fn polygon(&mut self, vx: &[Scalar], vy: &[Scalar]) -> PixResult<()> {
        let s = &self.settings;
        Ok(self.renderer.polygon(vx, vy, s.fill, s.stroke)?)
    }

    /// Draw a circle to the current canvas.
    pub fn circle(&mut self, circle: impl Into<Circle<Scalar>>) -> PixResult<()> {
        self.ellipse(circle.into())
    }

    /// Draw a ellipse to the current canvas.
    pub fn ellipse(&mut self, ellipse: impl Into<Ellipse<Scalar>>) -> PixResult<()> {
        let s = &self.settings;
        let ellipse = ellipse.into();
        let ellipse = match s.ellipse_mode {
            DrawMode::Corner => ellipse,
            DrawMode::Center => {
                let [x, y, width, height]: [Scalar; 4] = ellipse.into();
                ellipse!(x - width / 2.0, y - height / 2.0, width, height)
            }
        };
        Ok(self.renderer.ellipse(ellipse, s.fill, s.stroke)?)
    }

    /// Draw an image to the current canvas.
    pub fn image<P>(&mut self, position: P, img: &Image) -> PixResult<()>
    where
        P: Into<Point<Scalar>>,
    {
        Ok(self.renderer.image(position, img)?)
    }

    /// Draw a resized image to the current canvas.
    pub fn image_resized<R>(&mut self, dst_rect: R, img: &Image) -> PixResult<()>
    where
        R: Into<Rect<Scalar>>,
    {
        Ok(self.renderer.image_resized(dst_rect, img)?)
    }

    /// Draw a wireframe to the current canvas.
    pub fn wireframe<T, P>(
        &mut self,
        vertexes: &[Vector<T>],
        p: P,
        angle: T,
        scale: T,
    ) -> PixResult<()>
    where
        T: Float + Into<Scalar>,
        P: Into<Vector<T>>,
    {
        let p = p.into();
        let (sin, cos) = angle.sin_cos();
        let (tx, ty): (Vec<Scalar>, Vec<Scalar>) = vertexes
            .iter()
            .map(|v| {
                let x = (v.x * cos - v.y * sin) * scale + p.x;
                let y = (v.x * sin + v.y * cos) * scale + p.y;
                (x.into(), y.into())
            })
            .unzip();
        if tx.is_empty() || ty.is_empty() {
            Err(RendererError::Other(Cow::from("no vertexes to render")).into())
        } else {
            self.polygon(&tx, &ty)
        }
    }
}
