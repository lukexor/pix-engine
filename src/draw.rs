//! Draw functions.

use crate::{
    prelude::*,
    renderer::{Error as RendererError, Rendering},
};
use num_traits::{AsPrimitive, Float};
use std::{borrow::Cow, iter::Iterator};

impl PixState {
    /// Draw the `Texture` to the current canvas.
    pub fn texture<R, T>(
        &mut self,
        texture_id: usize,
        src: Option<R>,
        dst: Option<R>,
    ) -> PixResult<()>
    where
        R: Into<Rect<T>>,
        T: AsPrimitive<i32> + AsPrimitive<u32>,
    {
        Ok(self.renderer.texture(texture_id, src, dst)?)
    }

    /// Draw text to the current canvas.
    pub fn text<T>(&mut self, p: impl Into<Point<T>>, text: impl AsRef<str>) -> PixResult<()>
    where
        T: AsPrimitive<i16>,
    {
        let s = &self.settings;
        let p = p.into();
        let p = match s.rect_mode {
            DrawMode::Corner => point!(p.x.as_(), p.y.as_()),
            DrawMode::Center => {
                let height = s.text_size as i16;
                let width = text.as_ref().len() as i16 * height;
                point!(p.x.as_() - width / 2, p.y.as_() - height / 2)
            }
        };
        Ok(self.renderer.text(p, text, s.text_size, s.fill, s.stroke)?)
    }

    /// Draw a [`Point<T>`] to the current canvas.
    pub fn point<T>(&mut self, p: impl Into<Point<T>>) -> PixResult<()>
    where
        T: AsPrimitive<i16>,
    {
        let p = p.into();
        Ok(self
            .renderer
            .point(p.x.as_(), p.y.as_(), self.settings.stroke)?)
    }

    /// Draw a line to the current canvas.
    pub fn line<T>(&mut self, line: impl Into<Line<T>>) -> PixResult<()>
    where
        T: AsPrimitive<i16>,
    {
        let line = line.into();
        let Point { x: x1, y: y1, .. } = line.p1;
        let Point { x: x2, y: y2, .. } = line.p2;
        Ok(self
            .renderer
            .line(x1.as_(), y1.as_(), x2.as_(), y2.as_(), self.settings.stroke)?)
    }

    /// Draw a triangle to the current canvas.
    pub fn triangle<T>(&mut self, triangle: impl Into<Triangle<T>>) -> PixResult<()>
    where
        T: AsPrimitive<i16>,
    {
        let s = &self.settings;
        let triangle = triangle.into();
        let Point { x: x1, y: y1, .. } = triangle.p1;
        let Point { x: x2, y: y2, .. } = triangle.p2;
        let Point { x: x3, y: y3, .. } = triangle.p3;
        Ok(self.renderer.triangle(
            x1.as_(),
            y1.as_(),
            x2.as_(),
            y2.as_(),
            x3.as_(),
            y3.as_(),
            s.fill,
            s.stroke,
        )?)
    }

    /// Draw a square to the current canvas.
    pub fn square<T>(&mut self, square: impl Into<Square<T>>) -> PixResult<()>
    where
        T: AsPrimitive<i16>,
    {
        let square = square.into();
        self.rect(square)
    }

    /// Draw a rectangle to the current canvas.
    pub fn rect<R, T>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect<T>>,
        T: AsPrimitive<i16>,
    {
        let s = &self.settings;
        let rect = rect.into();
        let x: i16 = rect.x.as_();
        let y: i16 = rect.y.as_();
        let width: i16 = rect.w.as_();
        let height: i16 = rect.h.as_();
        let (x, y) = match s.rect_mode {
            DrawMode::Corner => (x, y),
            DrawMode::Center => (x - width / 2, y - height / 2),
        };
        Ok(self
            .renderer
            .rect(x, y, rect.w.as_(), rect.h.as_(), s.fill, s.stroke)?)
    }

    /// Draw a polygon to the current canvas.
    pub fn polygon(&mut self, vx: &[i16], vy: &[i16]) -> PixResult<()> {
        let s = &self.settings;
        Ok(self.renderer.polygon(vx, vy, s.fill, s.stroke)?)
    }

    /// Draw a circle to the current canvas.
    pub fn circle<T>(&mut self, circle: impl Into<Circle<T>>) -> PixResult<()>
    where
        T: AsPrimitive<i16>,
    {
        let circle = circle.into();
        self.ellipse(circle)
    }

    /// Draw a ellipse to the current canvas.
    pub fn ellipse<T>(&mut self, ellipse: impl Into<Ellipse<T>>) -> PixResult<()>
    where
        T: AsPrimitive<i16>,
    {
        let s = &self.settings;
        let ellipse = ellipse.into();
        let x: i16 = ellipse.x.as_();
        let y: i16 = ellipse.y.as_();
        let width: i16 = ellipse.w.as_();
        let height: i16 = ellipse.h.as_();
        let (x, y) = match s.ellipse_mode {
            DrawMode::Corner => (x, y),
            DrawMode::Center => (x - width / 2, y - height / 2),
        };
        Ok(self
            .renderer
            .ellipse(x, y, ellipse.w.as_(), ellipse.h.as_(), s.fill, s.stroke)?)
    }

    /// Draw an image to the current canvas.
    pub fn image(&mut self, x: i32, y: i32, img: &Image) -> PixResult<()> {
        Ok(self.renderer.image(x, y, img)?)
    }

    /// Draw a resized image to the current canvas.
    pub fn image_resized(&mut self, x: i32, y: i32, w: u32, h: u32, img: &Image) -> PixResult<()> {
        Ok(self.renderer.image_resized(x, y, w, h, img)?)
    }

    /// Draw a wireframe to the current canvas.
    pub fn wireframe<T>(
        &mut self,
        vertexes: &[Vector<T>],
        p: impl Into<Vector<T>>,
        angle: T,
        scale: T,
    ) -> PixResult<()>
    where
        T: Float + AsPrimitive<i16>,
    {
        let p = p.into();
        let (sin, cos) = angle.sin_cos();
        let (tx, ty): (Vec<i16>, Vec<i16>) = vertexes
            .iter()
            .map(|v| {
                let x = (v.x * cos - v.y * sin) * scale + p.x;
                let y = (v.x * sin + v.y * cos) * scale + p.y;
                (x.round().as_(), y.round().as_())
            })
            .unzip();
        if tx.is_empty() || ty.is_empty() {
            Err(RendererError::Other(Cow::from("no vertexes to render")).into())
        } else {
            self.polygon(&tx, &ty)
        }
    }
}
