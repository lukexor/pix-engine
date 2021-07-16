//! Drawing functions.

use num_traits::AsPrimitive;

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

    /// Draw text to the current canvas.
    pub fn text<P, S>(&mut self, p: P, text: S) -> PixResult<()>
    where
        P: Into<Point<Primitive>>,
        S: AsRef<str>,
    {
        let s = &self.settings;
        let p = p.into();
        let text = text.as_ref();
        let p = match s.rect_mode {
            DrawMode::Corner => p,
            DrawMode::Center => {
                let (width, height) = self.renderer.size_of(text)?;
                point!(p.x - width as Primitive / 2, p.y - height as Primitive / 2)
            }
        };
        Ok(self.renderer.text(&p, text, s.fill, s.stroke)?)
    }

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

    /// Draw an [Image] to the current canvas.
    pub fn image<P>(&mut self, position: P, img: &Image) -> PixResult<()>
    where
        P: Into<Point<Primitive>>,
    {
        let p = position.into();
        let s = &self.settings;
        let position = match s.image_mode {
            DrawMode::Corner => p,
            DrawMode::Center => point!(
                p.x - img.width() as Primitive / 2,
                p.y - img.height() as Primitive / 2
            ),
        };
        Ok(self.renderer.image(&position, img, s.image_tint)?)
    }

    /// Draw a resized [Image] to the current canvas.
    pub fn image_resized<R>(&mut self, rect: R, img: &Image) -> PixResult<()>
    where
        R: Into<Rect<Primitive>>,
    {
        let rect = rect.into();
        let s = &self.settings;
        let rect = match s.image_mode {
            DrawMode::Corner => rect,
            DrawMode::Center => rect!(
                rect.x - rect.width / 2,
                rect.y - rect.height / 2,
                rect.width,
                rect.height
            ),
        };
        Ok(self.renderer.image_resized(&rect, img, s.image_tint)?)
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