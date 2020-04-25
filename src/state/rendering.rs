//! Drawing and Renderer-specific functionality. Relies on the underlying renderer chosen (either
//! `sdl2-renderer` or `wasm-renderer`).

use super::{State, StateResult};
use crate::{image::Image, renderer::Renderer, shape::Rect};

/// The default blending factor for rendering operations.
pub const DEFAULT_BLEND_FACTOR: f32 = 1.0;

/// A trait that allows an object to be drawn to the engine.
pub trait Drawable {
    fn draw(&mut self, s: &mut State) -> StateResult<()>;
}

/// Blend mode used by the renderer for drawing operations
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
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

#[derive(Clone)]
pub struct Texture {
    id: usize,
    image: Image,
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
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) -> StateResult<()> {
        Ok(self.renderer.scale(scale_x, scale_y)?)
    }

    /// Get the clipping rectangle for the current window target.
    pub fn get_clip_rect(&self) -> Option<Rect> {
        self.renderer.get_clip_rect()
    }

    /// Set the clipping rectangle for the current window target.
    pub fn clip_rect<R: Into<Option<Rect>>>(&mut self, rect: R) {
        self.renderer.clip_rect(rect.into());
    }

    /// Get the viewport rectangle for the current window target.
    pub fn get_viewport(&self) -> Rect {
        self.renderer.get_viewport()
    }

    /// Set the viewport rectangle for the current window target.
    pub fn viewport<R: Into<Option<Rect>>>(&mut self, rect: R) {
        self.renderer.viewport(rect.into());
    }
}
