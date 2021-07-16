use super::Renderer;
use crate::{
    core::window::{Result, Window, WindowId},
    prelude::{Event, Primitive},
};
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
    fn set_dimensions(
        &mut self,
        id: WindowId,
        (width, height): (Primitive, Primitive),
    ) -> Result<()> {
        if id == self.window_id {
            self.canvas
                .window_mut()
                .set_size(width as u32, height as u32)?
        } else {
            todo!("secondary windows are not yet implemented");
        };
        Ok(())
    }

    /// Dimensions of the primary window as `(width, height)`.
    fn dimensions(&self, id: WindowId) -> Result<(Primitive, Primitive)> {
        let (width, height) = if id == self.window_id {
            self.canvas.window().size()
        } else {
            todo!("secondary windows are not yet implemented");
        };
        Ok((width as i32, height as i32))
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
