//! Drawing functions

use crate::{common::Result, renderer::sdl::SdlRect, renderer::Rendering, state::State};

/// Wrapper for SdlRect.
pub type Rect = SdlRect;

impl State {
    /// Draw an array of pixels to the current canvas.
    pub fn draw_pixels(&mut self, pixels: &[u8], pitch: usize) -> Result<()> {
        self.renderer.draw_pixels(pixels, pitch)
    }

    /// Create a texture to render to.
    pub fn create_texture(&mut self, width: u32, height: u32) -> Result<usize> {
        self.renderer.create_texture(width, height)
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
        let s = &self.settings;
        self.renderer.rect(x, y, width, height, s.fill, s.stroke)
    }
}
