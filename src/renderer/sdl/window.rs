use super::Renderer;
use crate::{
    core::window::{Result, Window, WindowId},
    event::Event,
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
    #[inline]
    fn set_title(&mut self, title: &str) -> Result<()> {
        Ok(self.canvas.window_mut().set_title(title)?)
    }

    /// Set dimensions of the primary window as `(width, height)`.
    fn set_dimensions(&mut self, id: WindowId, (width, height): (u32, u32)) -> Result<()> {
        if id == self.window_id {
            self.canvas.window_mut().set_size(width, height)?
        } else {
            todo!("secondary windows are not yet implemented");
        };
        Ok(())
    }

    /// Dimensions of the primary window as `(width, height)`.
    fn dimensions(&self, id: WindowId) -> Result<(u32, u32)> {
        let dimensions = if id == self.window_id {
            self.canvas.window().size()
        } else {
            todo!("secondary windows are not yet implemented");
        };
        Ok(dimensions)
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
