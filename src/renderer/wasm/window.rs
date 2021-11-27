use super::Renderer;
use crate::{
    prelude::*,
    renderer::{RendererSettings, WindowRenderer},
};

impl WindowRenderer for Renderer {
    /// Get the count of open windows.
    #[inline]
    fn window_count(&self) -> usize {
        todo!()
    }

    /// Get the current window target ID.
    #[inline]
    fn window_id(&self) -> WindowId {
        todo!()
    }

    /// Create a new window.
    #[inline]
    fn create_window(&mut self, s: &RendererSettings) -> PixResult<WindowId> {
        todo!()
    }

    /// Close a window.
    #[inline]
    fn close_window(&mut self, id: WindowId) -> PixResult<()> {
        todo!()
    }

    /// Set the mouse cursor to a predefined symbol or image, or hides cursor if `None`.
    #[inline]
    fn cursor(&mut self, cursor: Option<&Cursor>) -> PixResult<()> {
        todo!()
    }

    /// Returns a single event or None if the event pump is empty.
    #[inline]
    fn poll_event(&mut self) -> Option<Event> {
        todo!()
    }

    /// Get the current window title.
    #[inline]
    fn title(&self) -> &str {
        todo!()
    }

    /// Set the current window title.
    #[inline]
    fn set_title(&mut self, title: &str) -> PixResult<()> {
        todo!()
    }

    /// Set the average frames-per-second rendered.
    #[inline]
    fn set_fps(&mut self, fps: usize) -> PixResult<()> {
        todo!()
    }

    /// Dimensions of the current render target as `(width, height)`.
    #[inline]
    fn dimensions(&self) -> PixResult<(u32, u32)> {
        todo!()
    }

    /// Dimensions of the current window target as `(width, height)`.
    #[inline]
    fn window_dimensions(&self) -> PixResult<(u32, u32)> {
        todo!()
    }

    /// Set dimensions of the current window target as `(width, height)`.
    #[inline]
    fn set_window_dimensions(&mut self, dimensions: (u32, u32)) -> PixResult<()> {
        todo!()
    }

    /// Returns the rendering viewport of the current render target.
    #[inline]
    fn viewport(&self) -> PixResult<Rect<i32>> {
        todo!()
    }

    /// Set the rendering viewport of the current render target.
    #[inline]
    fn set_viewport(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        todo!()
    }

    /// Dimensions of the primary display as `(width, height)`.
    #[inline]
    fn display_dimensions(&self) -> PixResult<(u32, u32)> {
        todo!()
    }

    /// Returns whether the application is fullscreen or not.
    #[inline]
    fn fullscreen(&self) -> PixResult<bool> {
        todo!()
    }

    /// Set the application to fullscreen or not.
    #[inline]
    fn set_fullscreen(&mut self, val: bool) -> PixResult<()> {
        todo!()
    }

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    #[inline]
    fn vsync(&self) -> bool {
        todo!()
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    #[inline]
    fn set_vsync(&mut self, val: bool) -> PixResult<()> {
        todo!()
    }

    /// Set window as the target for drawing operations.
    #[inline]
    fn set_window_target(&mut self, id: WindowId) -> PixResult<()> {
        todo!()
    }

    /// Reset main window as the target for drawing operations.
    #[inline]
    fn reset_window_target(&mut self) {
        todo!()
    }

    /// Show the current window target.
    #[inline]
    fn show(&mut self) -> PixResult<()> {
        todo!()
    }

    /// Hide the current window target.
    #[inline]
    fn hide(&mut self) -> PixResult<()> {
        todo!()
    }
}
