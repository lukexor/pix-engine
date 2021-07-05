//! Draw functions.

use crate::{
    prelude::*,
    renderer::{Error as RendererError, Rendering},
};
use std::{borrow::Cow, iter::Iterator};

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw shape to the current [`PixState`] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()>;
}

impl PixState {
    /// Draw text to the current canvas.
    pub fn text<P, S>(&mut self, p: P, text: S) -> PixResult<()>
    where
        P: Into<Point<i32>>,
        S: AsRef<str>,
    {
        let s = &self.settings;
        Ok(self
            .renderer
            .text(p.into(), text.as_ref(), s.fill, s.stroke)?)
    }

    /// Draw a [`Point`] to the current canvas.
    pub fn point<P>(&mut self, p: P) -> PixResult<()>
    where
        P: Into<Point>,
    {
        if let Some(stroke) = self.settings.stroke {
            Ok(self.renderer.point(p.into().as_(), stroke)?)
        } else {
            Ok(())
        }
    }

    /// Draw a line to the current canvas.
    pub fn line<L>(&mut self, line: L) -> PixResult<()>
    where
        L: Into<Line>,
    {
        if let Some(stroke) = self.settings.stroke {
            Ok(self.renderer.line(line.into().as_(), stroke)?)
        } else {
            Ok(())
        }
    }

    /// Draw a triangle to the current canvas.
    pub fn triangle<T>(&mut self, tri: T) -> PixResult<()>
    where
        T: Into<Triangle>,
    {
        let s = &self.settings;
        Ok(self.renderer.triangle(tri.into().as_(), s.fill, s.stroke)?)
    }

    /// Draw a square to the current canvas.
    pub fn square<R>(&mut self, square: R) -> PixResult<()>
    where
        R: Into<Rect>,
    {
        self.rect(square)
    }

    /// Draw a rectangle to the current canvas.
    pub fn rect<R>(&mut self, rect: R) -> PixResult<()>
    where
        R: Into<Rect>,
    {
        let s = &self.settings;
        let rect = rect.into().as_();
        let rect = match s.rect_mode {
            DrawMode::Corner => rect,
            DrawMode::Center => {
                let [x, y, width, height]: [i16; 4] = rect.into();
                rect!(x - width / 2, y - height / 2, width, height)
            }
        };
        Ok(self.renderer.rect(rect, s.fill, s.stroke)?)
    }

    /// Draw a polygon to the current canvas.
    pub fn polygon(&mut self, vx: &[Scalar], vy: &[Scalar]) -> PixResult<()> {
        let s = &self.settings;
        let vx: Vec<i16> = vx.iter().map(|x| *x as i16).collect();
        let vy: Vec<i16> = vy.iter().map(|y| *y as i16).collect();
        Ok(self.renderer.polygon(&vx, &vy, s.fill, s.stroke)?)
    }

    /// Draw a circle to the current canvas.
    pub fn circle<C>(&mut self, circle: C) -> PixResult<()>
    where
        C: Into<Circle>,
    {
        self.ellipse(circle.into())
    }

    /// Draw a ellipse to the current canvas.
    pub fn ellipse<E>(&mut self, ellipse: E) -> PixResult<()>
    where
        E: Into<Ellipse>,
    {
        let s = &self.settings;
        let ellipse = ellipse.into().as_();
        let ellipse = match s.ellipse_mode {
            DrawMode::Corner => ellipse,
            DrawMode::Center => {
                let [x, y, width, height]: [i16; 4] = ellipse.into();
                ellipse!(x - width / 2, y - height / 2, width, height)
            }
        };
        Ok(self.renderer.ellipse(ellipse, s.fill, s.stroke)?)
    }

    /// Draw an image to the current canvas.
    pub fn image<P>(&mut self, position: P, img: &Image) -> PixResult<()>
    where
        P: Into<Point<i32>>,
    {
        Ok(self.renderer.image(position.into(), img)?)
    }

    /// Draw a resized image to the current canvas.
    pub fn image_resized<R>(&mut self, dst_rect: R, img: &Image) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
    {
        Ok(self.renderer.image_resized(dst_rect.into(), img)?)
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
