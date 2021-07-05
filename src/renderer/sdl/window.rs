use super::Renderer;
use crate::{
    event::Event,
    window::{Result, Window, WindowId},
};
use num_traits::AsPrimitive;
use sdl2::video::FullscreenType;

impl Window for Renderer {
    /// Get the primary window id.
    fn window_id(&self) -> WindowId {
        self.window_id
    }

    /// Set whether the cursor is shown or not.
    fn cursor(&mut self, show: bool) {
        self.context.mouse().show_cursor(show);
    }

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event().map(|evt| evt.into())
    }

    /// Get the current window title.
    fn title(&self) -> &str {
        self.canvas.window().title()
    }

    /// Set the current window title.
    fn set_title<S>(&mut self, title: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        Ok(self.canvas.window_mut().set_title(title.as_ref())?)
    }

    /// Width of the window.
    fn window_width(&self) -> Result<u32> {
        let (width, _) = self.canvas.window().size();
        Ok(width)
    }

    /// Height of the window.
    fn window_height(&self) -> Result<u32> {
        let (_, height) = self.canvas.window().size();
        Ok(height)
    }

    /// Resize the window.
    fn resize<T>(&mut self, width: T, height: T) -> Result<()>
    where
        T: AsPrimitive<u32>,
    {
        Ok(self
            .canvas
            .window_mut()
            .set_size(width.as_(), height.as_())?)
    }

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool {
        use FullscreenType::*;
        matches!(self.canvas.window().fullscreen_state(), True | Desktop)
    }

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool) {
        let fullscreen_type = if val {
            FullscreenType::True
        } else {
            FullscreenType::Off
        };
        // Don't care if this fails or not.
        let _ = self.canvas.window_mut().set_fullscreen(fullscreen_type);
    }
}
