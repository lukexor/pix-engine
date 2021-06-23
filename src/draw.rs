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
        R: Into<Option<Rect<f64>>>,
    {
        Ok(self.renderer.texture(texture_id, src, dst)?)
    }

    /// Draw text to the current canvas.
    pub fn text(&mut self, p: impl Into<Point<f64>>, text: impl AsRef<str>) -> PixResult<()> {
        let s = &self.settings;
        let p = p.into();
        let p = match s.rect_mode {
            DrawMode::Corner => p,
            DrawMode::Center => {
                let height = s.text_size as f64;
                let width = text.as_ref().len() as f64 * height;
                point!(p.x - width / 2.0, p.y - height / 2.0)
            }
        };
        Ok(self.renderer.text(p, text, s.text_size, s.fill, s.stroke)?)
    }

    /// Draw a [`Point`] to the current canvas.
    pub fn point(&mut self, p: impl Into<Point<f64>>) -> PixResult<()> {
        Ok(self.renderer.point(p, self.settings.stroke)?)
    }

    /// Draw a line to the current canvas.
    pub fn line(&mut self, line: impl Into<Line<f64>>) -> PixResult<()> {
        Ok(self.renderer.line(line, self.settings.stroke)?)
    }

    /// Draw a triangle to the current canvas.
    pub fn triangle(&mut self, tri: impl Into<Triangle<f64>>) -> PixResult<()> {
        let s = &self.settings;
        Ok(self.renderer.triangle(tri, s.fill, s.stroke)?)
    }

    /// Draw a square to the current canvas.
    pub fn square(&mut self, square: impl Into<Rect<f64>>) -> PixResult<()> {
        self.rect(square)
    }

    /// Draw a rectangle to the current canvas.
    pub fn rect(&mut self, rect: impl Into<Rect<f64>>) -> PixResult<()> {
        let s = &self.settings;
        let rect = rect.into();
        let rect = match s.rect_mode {
            DrawMode::Corner => rect,
            DrawMode::Center => {
                let [x, y, width, height]: [f64; 4] = rect.into();
                rect!(x - width / 2.0, y - height / 2.0, width, height)
            }
        };
        Ok(self.renderer.rect(rect, s.fill, s.stroke)?)
    }

    /// Draw a polygon to the current canvas.
    pub fn polygon(&mut self, vx: &[f64], vy: &[f64]) -> PixResult<()> {
        let s = &self.settings;
        Ok(self.renderer.polygon(vx, vy, s.fill, s.stroke)?)
    }

    /// Draw a circle to the current canvas.
    pub fn circle(&mut self, circle: impl Into<Circle<f64>>) -> PixResult<()> {
        self.ellipse(circle.into())
    }

    /// Draw a ellipse to the current canvas.
    pub fn ellipse(&mut self, ellipse: impl Into<Ellipse<f64>>) -> PixResult<()> {
        let s = &self.settings;
        let ellipse = ellipse.into();
        let ellipse = match s.ellipse_mode {
            DrawMode::Corner => ellipse,
            DrawMode::Center => {
                let [x, y, width, height]: [f64; 4] = ellipse.into();
                ellipse!(x - width / 2.0, y - height / 2.0, width, height)
            }
        };
        Ok(self.renderer.ellipse(ellipse, s.fill, s.stroke)?)
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
        T: Float + Into<f64>,
    {
        let p = p.into();
        let (sin, cos) = angle.sin_cos();
        let (tx, ty): (Vec<f64>, Vec<f64>) = vertexes
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
