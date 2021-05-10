//! Drawing functions

use crate::{
    image::Image,
    renderer::{self, Rendering},
    shape::{Line, Point, Triangle},
    state::PixState,
};

/// Drawing mode which changes how (x, y) coordinates are interpreted.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DrawMode {
    /// Use (x, y) as the top-left corner. Default.
    Corner,
    /// Use (x, y) as the center.
    Center,
}

/// Drawing mode which changes how arcs are drawn.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArcMode {
    /// Draws arc with fill as an open pie segment.
    None,
    /// Draws arc with fill as an closed pie segment.
    Pie,
    /// Draws arc with fill as an open semi-circle.
    Open,
    /// Draws arc with fill as a closed semi-circle.
    Chord,
}

// TODO: StrokeCap { Round, Square, PRoject }
// TODO: StrokeJoin { Miter, Bevel, Round }
// TODO: AngleMode { Radians, Degrees }
// TODO: BlendMode { Blend, Add, Replace, .. }
//   Blend: A * factor + B
//   DARKEST, LIGHTEST, DIFFERENCE, EXCLUSION, MULTIPLY, SCREEN, REMOVE, OVERLAY, HARD_LIGHT, SOFT_LIGHT, DODGE, BURN, SUBTRACT

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
    pub fn text<S>(&mut self, x: i32, y: i32, text: S) -> renderer::Result<()>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();
        let s = &self.settings;
        let width = text.len() as u32 * s.text_size;
        let (x, y) = match s.rect_mode {
            DrawMode::Corner => (x, y),
            DrawMode::Center => (x - width as i32 / 2, y - s.text_size as i32 / 2),
        };
        self.renderer
            .text(text, x, y, s.text_size, s.fill, s.stroke)
    }

    /// Draw a point to the current canvas.
    pub fn point(&mut self, x: i32, y: i32) -> renderer::Result<()> {
        self.renderer.point(x, y, self.settings.stroke)
    }

    /// Draw an array of pixels to the current canvas.
    pub fn points(&mut self, pixels: &[u8], pitch: usize) -> renderer::Result<()> {
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
        let triangle = triangle.into();
        let s = &self.settings;
        let Point { x: x1, y: y1, .. } = triangle.p1;
        let Point { x: x2, y: y2, .. } = triangle.p2;
        let Point { x: x3, y: y3, .. } = triangle.p3;
        self.renderer
            .triangle(x1, y1, x2, y2, x3, y3, s.fill, s.stroke)
    }

    /// Draw a square to the current canvas.
    pub fn square(&mut self, x: i32, y: i32, s: u32) -> renderer::Result<()> {
        self.rect(x, y, s, s)
    }

    /// Draw a rectangle to the current canvas.
    pub fn rect(&mut self, x: i32, y: i32, width: u32, height: u32) -> renderer::Result<()> {
        let s = &self.settings;
        let (x, y) = match s.rect_mode {
            DrawMode::Corner => (x, y),
            DrawMode::Center => (x - width as i32 / 2, y - height as i32 / 2),
        };
        self.renderer.rect(x, y, width, height, s.fill, s.stroke)
    }

    /// Draw a circle to the current canvas.
    pub fn circle(&mut self, x: i32, y: i32, radius: u32) -> renderer::Result<()> {
        self.ellipse(x, y, radius, radius)
    }

    /// Draw a ellipse to the current canvas.
    pub fn ellipse(&mut self, x: i32, y: i32, width: u32, height: u32) -> renderer::Result<()> {
        let s = &self.settings;
        let (x, y) = match s.ellipse_mode {
            DrawMode::Corner => (x, y),
            DrawMode::Center => (x - width as i32 / 2, y - height as i32 / 2),
        };
        self.renderer.ellipse(x, y, width, height, s.fill, s.stroke)
    }

    /// Draw an image to the current canvas.
    pub fn image(&mut self, x: i32, y: i32, img: &Image) -> renderer::Result<()> {
        self.renderer.image(x, y, img)
    }

    /// Draw a wireframe to the current canvas.
    pub fn wireframe(
        &mut self,
        vertexes: &[(f64, f64)],
        x: i32,
        y: i32,
        angle: f64,
        scale: f64,
    ) -> renderer::Result<()> {
        if vertexes.is_empty() {
            return Ok(());
        }
        let x = x as f64;
        let y = y as f64;
        // TODO: cache?
        let mut transformed_verts = vertexes.to_vec();
        for (i, vertex) in transformed_verts.iter_mut().enumerate() {
            // Rotate
            vertex.0 = vertexes[i].0 * angle.cos() - vertexes[i].1 * angle.sin();
            vertex.1 = vertexes[i].0 * angle.sin() + vertexes[i].1 * angle.cos();

            // Scale
            vertex.0 *= scale;
            vertex.1 *= scale;

            // Translate
            vertex.0 += x;
            vertex.1 += y;
        }

        // Draw
        for i in 0..transformed_verts.len() - 1 {
            let p1: Point = transformed_verts[i].into();
            let p2: Point = transformed_verts[i + 1].into();
            self.line((p1, p2))?;
        }
        // Safety: We can only get here if there is at least one vetex
        let p1: Point = transformed_verts[transformed_verts.len() - 1].into();
        let p2: Point = transformed_verts[0].into();
        self.line((p1, p2))?;

        Ok(())
    }
}
