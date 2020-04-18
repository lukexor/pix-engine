//! Renderer-specific functionality. Relies on the underlying renderer chosen (either
//! `sdl2-renderer` or `wasm-renderer`).

use super::{Result, State};
use crate::{
    renderer::Renderer,
    shape::{Point, Rect},
};

pub const DEFAULT_BLEND_FACTOR: f32 = 1.0;

/// Blend mode used by the renderer for drawing operations
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlendMode {
    None,
    Blend,
    Add,
    Mod,
    Invalid,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Blend
    }
}

impl State {
    /// Presents changes made to the canvas since present was last called.
    pub fn present(&mut self) {
        self.renderer.present()
    }

    /// Presents changes made to the canvases of all windows since present was last called.
    pub fn present_all(&mut self) {
        self.renderer.present_all()
    }

    /// Clears the canvas to the current draw color.
    pub fn clear(&mut self) {
        self.renderer.clear()
    }

    /// Clears all canvases of all windows to their current draw colors.
    pub fn clear_all(&mut self) {
        self.renderer.clear_all();
    }

    /// Get the scale_x and scale_y factors for the current window target.
    pub fn get_scale(&self) -> (f32, f32) {
        self.renderer.get_scale()
    }

    /// Set the scale_x and scale_y factors for the current window target.
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) -> Result<()> {
        Ok(self.renderer.scale(scale_x, scale_y)?)
    }

    /// Get the clipping rectangle for the current window target.
    pub fn get_clip_rect(&self) -> Option<Rect> {
        self.renderer.get_clip_rect()
    }

    /// Set the clipping rectangle for the current window target.
    pub fn clip_rect<R: Into<Option<Rect>>>(&mut self, rect: R) {
        self.renderer.clip_rect(rect);
    }

    /// Get the viewport rectangle for the current window target.
    pub fn get_viewport(&self) -> Rect {
        self.renderer.get_viewport()
    }

    /// Set the viewport rectangle for the current window target.
    pub fn viewport<R: Into<Option<Rect>>>(&mut self, rect: R) {
        self.renderer.viewport(rect);
    }

    /// Drawing

    /// Draw a point.
    pub fn draw_point<P: Into<Point>>(&mut self, point: P) -> Result<()> {
        Ok(self.renderer.draw_point(point)?)
    }

    /// Draw multiple points.
    pub fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<()> {
        Ok(self.renderer.draw_points(points)?)
    }

    /// Draw a line.
    pub fn draw_line<P1: Into<Point>, P2: Into<Point>>(
        &mut self,
        start: P1,
        end: P2,
    ) -> Result<()> {
        Ok(self.renderer.draw_line(start, end)?)
    }

    /// Draw a series of lines.
    pub fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, points: P) -> Result<()> {
        Ok(self.renderer.draw_lines(points)?)
    }

    /// Draw a rectangle. If `None` is passed, the entire screen is used as the `Rect`. If both
    /// fill and stroke are set to None, nothing will be drawn.
    pub fn draw_rect<R: Into<Option<Rect>>>(&mut self, rect: R) -> Result<()> {
        let rect = rect.into();
        self.renderer.fill_rect(rect.clone())?;
        if let Some(rect) = rect {
            self.renderer.draw_rect(rect)?;
        }
        Ok(())
    }

    /// Draw multiple rectangles. If both fill and stroke are set to None, nothing will be drawn.
    pub fn draw_rects<'a, R: Into<&'a [Rect]>>(&mut self, rects: R) -> Result<()> {
        self.renderer.fill_rects(rects)?;
        // self.renderer.draw_rects(rects)?;
        Ok(())
    }

    // /// Reads pixels from the current window target.
    // /// # Remarks
    // /// WARNING: This is a very slow operation, and should not be used frequently.
    // pub fn read_pixels<R: Into<Option<Rect>>>(&self, rect: R, format: PixelFormatEnum) -> Result<()>;

    // /// Textures
    // TODO
    // copy
    // copy_ex
}
