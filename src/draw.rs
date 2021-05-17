//! Drawing functions

use crate::{
    image::Image,
    renderer::{self, Rendering},
    shape::{Circle, Ellipse, Line, Point, Rect, Square, Triangle},
    state::{settings::DrawMode, PixState},
    vector::Vector,
};

// TODO: StrokeCap { Round, Square, PRoject }
// TODO: StrokeJoin { Miter, Bevel, Round }
// TODO: AngleMode { Radians, Degrees }
//   Blend: A * factor + B

impl PixState {
    // TODO:

    // 2D
    // arc(p: Point, w: u32, h: u32, start: f32, stop: f32)
    // quad(p1: Point, p2: Point, p3: Point, p4: Point)
    // square_rounded(x: i32, y: i32, s: u32, r: u32)
    // rect_rounded(x: i32, y: i32, w: u32, h: u32, r: u32)
    // texture(texture: &Texture)

    // 3D
    // geometry_detail(x_count: i32, y_count: i32)
    // plane(p1: Point, p2: Point)
    // sphere(c: Point, r: i32)
    // ellipsoid(rx: i32, ry: i32, rz: i32)
    // cube(p: Point, w: u32, h: u32, d: u32)
    // enum ShapeCap { None, Top, Bottom, Both }
    // cylinder(c: Point, r: u32, h: u32)
    // cone(c: Point, r: u32, h: u32)
    // torus(c: Point, r: u32, tr: u32)

    // image_resized(img: Image, x: i32, y: i32, w: u32, h: u32)
    // image_projected(img: Image, dx: i32, dy: i32, dw: u32, dh: u32, sx: i32, sy: i32, sw: u32, sh: u32)

    /// Create a texture to render to.
    pub fn create_texture(&mut self, _width: u32, _height: u32) -> renderer::Result<usize> {
        todo!("create_texture");
        // self.renderer.create_texture(width, height)
    }

    /// Draw text to the current canvas.
    pub fn text<S, P>(&mut self, p: P, text: S) -> renderer::Result<()>
    where
        S: AsRef<str>,
        P: Into<Point>,
    {
        let text = text.as_ref();
        let p = p.into();
        let s = &self.settings;
        let width = text.len() as u32 * s.text_size;
        let (x, y) = match s.rect_mode {
            DrawMode::Corner => (p.x, p.y),
            DrawMode::Center => (p.x - width as i32 / 2, p.y - s.text_size as i32 / 2),
        };
        self.renderer
            .text(text, x, y, s.text_size, s.fill, s.stroke)
    }

    /// Draw a point to the current canvas.
    pub fn point<P>(&mut self, p: P) -> renderer::Result<()>
    where
        P: Into<Point>,
    {
        let p = p.into();
        self.renderer.point(p.x, p.y, self.settings.stroke)
    }

    /// Draw an array of pixels to the current canvas.
    pub fn pixels(&mut self, pixels: &[u8], pitch: usize) -> renderer::Result<()> {
        self.renderer.points(pixels, pitch)
    }

    /// Draw a line to the current canvas.
    pub fn line<L>(&mut self, line: L) -> renderer::Result<()>
    where
        L: Into<Line>,
    {
        let line = line.into();
        let Point { x: x1, y: y1, .. } = line.p1;
        let Point { x: x2, y: y2, .. } = line.p2;
        self.renderer.line(x1, y1, x2, y2, self.settings.stroke)
    }

    /// Draw a triangle to the current canvas.
    pub fn triangle<T>(&mut self, triangle: T) -> renderer::Result<()>
    where
        T: Into<Triangle>,
    {
        let s = &self.settings;
        let triangle = triangle.into();
        let Point { x: x1, y: y1, .. } = triangle.p1;
        let Point { x: x2, y: y2, .. } = triangle.p2;
        let Point { x: x3, y: y3, .. } = triangle.p3;
        self.renderer
            .triangle(x1, y1, x2, y2, x3, y3, s.fill, s.stroke)
    }

    /// Draw a square to the current canvas.
    pub fn square<S>(&mut self, s: S) -> renderer::Result<()>
    where
        S: Into<Square>,
    {
        let s = s.into();
        self.rect(s)
    }

    /// Draw a rectangle to the current canvas.
    pub fn rect<R>(&mut self, r: R) -> renderer::Result<()>
    where
        R: Into<Rect>,
    {
        let s = &self.settings;
        let r = r.into();
        let (x, y) = match s.rect_mode {
            DrawMode::Corner => (r.x, r.y),
            DrawMode::Center => (r.x - r.w as i32 / 2, r.y - r.h as i32 / 2),
        };
        self.renderer.rect(x, y, r.w, r.h, s.fill, s.stroke)
    }

    /// Draw a circle to the current canvas.
    pub fn circle<C>(&mut self, c: C) -> renderer::Result<()>
    where
        C: Into<Circle>,
    {
        let c = c.into();
        self.ellipse(c)
    }

    /// Draw a ellipse to the current canvas.
    pub fn ellipse<E>(&mut self, e: E) -> renderer::Result<()>
    where
        E: Into<Ellipse>,
    {
        let s = &self.settings;
        let e = e.into();
        let (x, y) = match s.ellipse_mode {
            DrawMode::Corner => (e.x, e.y),
            DrawMode::Center => (e.x - e.w as i32 / 2, e.y - e.h as i32 / 2),
        };
        self.renderer.ellipse(x, y, e.w, e.h, s.fill, s.stroke)
    }

    /// Draw an image to the current canvas.
    pub fn image(&mut self, x: i32, y: i32, img: &Image) -> renderer::Result<()> {
        self.renderer.image(x, y, img)
    }

    /// Draw a resized image to the current canvas.
    pub fn image_resized(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        img: &Image,
    ) -> renderer::Result<()> {
        self.renderer.image_resized(x, y, w, h, img)
    }

    /// Draw a wireframe to the current canvas.
    pub fn wireframe<P>(
        &mut self,
        vertexes: &[(f64, f64)],
        p: P,
        angle: f64,
        scale: f64,
    ) -> renderer::Result<()>
    where
        P: Into<Vector>,
    {
        let p = p.into();
        if vertexes.is_empty() {
            return Ok(());
        }
        // TODO: cache?
        let mut transformed_verts = vertexes.to_vec();
        let (sin, cos) = angle.sin_cos();
        for (i, vertex) in transformed_verts.iter_mut().enumerate() {
            // Rotate
            vertex.0 = vertexes[i].0 * cos - vertexes[i].1 * sin;
            vertex.1 = vertexes[i].0 * sin + vertexes[i].1 * cos;

            // Scale
            vertex.0 *= scale;
            vertex.1 *= scale;

            // Translate
            vertex.0 += p.x;
            vertex.1 += p.y;
        }

        // Draw
        for i in 0..transformed_verts.len() - 1 {
            let p1: Point = transformed_verts[i].into();
            let p2: Point = transformed_verts[i + 1].into();
            self.line((p1, p2))?;
        }
        let p1: Point = transformed_verts[transformed_verts.len() - 1].into();
        let p2: Point = transformed_verts[0].into();
        self.line((p1, p2))?;

        Ok(())
    }
}
