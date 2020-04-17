use super::{Result, State};
use crate::renderer::Renderer;

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
    /// Settings

    /// Set title for the current window target.
    ///
    /// Errors if the title contains a nul byte.
    pub fn set_title(&mut self, title: &str) -> Result<()> {
        Ok(self.renderer.set_title(title)?)
    }

    /// Sets the audio sample rate for the audio playback in Hz.
    pub fn set_audio_sample_rate(&mut self, rate: i32) -> Result<()> {
        Ok(self.renderer.set_audio_sample_rate(rate)?)
    }

    /// Rendering

    /// Presents changes made to the canvas on the current window target since present was last
    /// called.
    pub fn present(&mut self) {
        self.renderer.present()
    }

    /// Presents changes made to the canvases of all windows since present was last called.
    pub fn present_all(&mut self) {
        self.renderer.present_all()
    }

    /// Clears the canvas on the current window target to the current draw color.
    pub fn clear(&mut self) {
        self.renderer.clear()
    }

    /// Clears all canvases of all windows to their current draw colors.
    pub fn clear_all(&mut self) {
        self.renderer.clear_all();
    }

    // /// Get the scale_x and scale_y factors for the current window target.
    // pub fn scale(&self) -> (f32, f32) {
    //     self.renderer.scale()
    // }

    // /// Set the scale_x and scale_y factors for the current window target.
    // pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
    //     self.renderer.set_scale(scale_x, scale_y)
    // }

    // /// Get the clipping rectangle for the current window target.
    // // pub fn clip_rect(&self){}

    // /// Set the clipping rectangle for the current window target.
    // // pub fn set_clip_rect<R: Into<Option<Rect>>>(&mut self, rect: R){}

    // /// Get the viewport rectangle for the current window target.
    // // pub fn viewport(&self) -> Rect{}

    // /// Set the viewport rectangle for the current window target.
    // // pub fn set_viewport<R: Into<Option<Rect>>>(&mut self, rect: R){}

    // /// Drawing

    // /// Draw a point on the current window target.
    // // pub fn draw_point<P: Into<Point>>(&mut self, point: P){}

    // /// Draw multiple points on the current window target.
    // // pub fn draw_points<'a, P: Into<&'a [Point]>>(&mut self, points: P){}

    // /// Draw a line on the current window target.
    // // pub fn draw_line<P1: Into<Point>, P2: Into<Point>>(&mut self, start: P1, end: P2){}

    // /// Draw a series of lines on the current window target.
    // // pub fn draw_lines<'a, P: Into<&'a [Point]>>(&mut self, points: P){}

    // /// Draw a rectangle on the current window target.
    // // pub fn draw_rect<R: Into<Rect>>(&mut self, rect: R){}

    // /// Draw multiple rectangles on the current window target.
    // // pub fn draw_rects<'a, R: Into<&'a [Rect]>>(&mut self, rects: R)

    // /// Draw a filled rectangle on the current window target. Passing None will fill the entire
    // /// rendering target.
    // // pub fn fill_rect<R: Into<Option<Rect>>>(&mut self, rect: R){}

    // /// Draw multiple filled rectangles on the current window target.
    // // pub fn fill_rects<'a, R: Into<&'a [Rect]>>(&mut self, rects: R)

    // /// Reads pixels from the current window target.
    // /// # Remarks
    // /// WARNING: This is a very slow operation, and should not be used frequently.
    // // pub fn read_pixels<R: Into<Option<Rect>>>(&self, rect: R, format: PixelFormatEnum)

    // /// Textures
    // TODO
    // copy
    // copy_ex
}
