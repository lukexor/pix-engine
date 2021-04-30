//! Drawing functions

use crate::{common::Result, renderer::sdl::SdlRect, renderer::Rendering, state::State};

/// Wrapper for SdlRect.
pub type Rect = SdlRect;

/// Drawing mode which changes how (x, y) coordinates are interpreted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DrawMode {
    /// Use (x, y) as the top-left corner.
    Corner,
    /// Use (x, y) as the center.
    Center,
    /// Use (x, y) as the center and size, width, height as the radius.
    Radius,
}

impl State {
    /// Draw an array of pixels to the current canvas.
    pub fn draw_pixels(&mut self, pixels: &[u8], pitch: usize) -> Result<()> {
        self.renderer.draw_pixels(pixels, pitch)
    }

    /// Create a texture to render to.
    pub fn create_texture(&mut self, width: u32, height: u32) -> Result<usize> {
        self.renderer.create_texture(width, height)
    }

    /// Draw text to the current canvas.
    pub fn text(&mut self, text: &str, x: i32, y: i32) -> Result<()> {
        let s = &self.settings;
        // TODO Add rect_mode
        // TODO Add text_size
        self.renderer.text(text, x, y, s.fill, s.stroke)
    }

    /// Draw a point to the current canvas.
    pub fn pixel(&mut self, x: i32, y: i32) -> Result<()> {
        self.renderer.pixel(x, y, self.settings.stroke)
    }

    /// Draw a triangle to the current canvas.
    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<()> {
        self.renderer.line(x1, y1, x2, y2, self.settings.stroke)
    }

    /// Draw a triangle to the current canvas.
    pub fn triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Result<()> {
        let s = &self.settings;
        self.renderer
            .triangle(x1, y1, x2, y2, x3, y3, s.fill, s.stroke)
    }

    /// Draw a square to the current canvas.
    pub fn square(&mut self, x: i32, y: i32, s: u32) -> Result<()> {
        self.rect(x, y, s, s)
    }

    /// Draw a rectangle to the current canvas.
    pub fn rect(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()> {
        self.env.render = true;
        // TODO Add rect_mode
        let s = &self.settings;
        self.renderer.rect(x, y, width, height, s.fill, s.stroke)
    }

    /// Draw a circle to the current canvas.
    pub fn circle(&mut self, x: i32, y: i32, radius: u32) -> Result<()> {
        self.ellipse(x, y, radius, radius)
    }

    /// Draw a ellipse to the current canvas.
    pub fn ellipse(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<()> {
        // TODO Add ellipse_mode
        let s = &self.settings;
        self.renderer.ellipse(x, y, width, height, s.fill, s.stroke)
    }

    /// Draw a wireframe to the current canvas.
    pub fn wireframe(
        &mut self,
        vertexes: &[(f64, f64)],
        x: i32,
        y: i32,
        angle: f64,
        scale: f64,
    ) -> Result<()> {
        let x = x as f64;
        let y = y as f64;
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
            let v1 = transformed_verts[i];
            let v2 = transformed_verts[i + 1];
            self.line(
                v1.0.round() as i32,
                v1.1.round() as i32,
                v2.0.round() as i32,
                v2.1.round() as i32,
            )?;
        }
        let v1 = transformed_verts.last().unwrap();
        let v2 = transformed_verts[0];
        self.line(
            v1.0.round() as i32,
            v1.1.round() as i32,
            v2.0.round() as i32,
            v2.1.round() as i32,
        )?;

        Ok(())
    }
}
