use super::Renderer;
use crate::{
    core::window::{Error, Result, Window, WindowId},
    prelude::{Cursor, Event, Primitive, SystemCursor},
};
use sdl2::{
    image::LoadSurface,
    mouse::{Cursor as SdlCursor, SystemCursor as SdlSystemCursor},
    surface::Surface,
    video::FullscreenType,
    IntegerOrSdlError,
};
use std::borrow::Cow;

impl Window for Renderer {
    /// Get the primary window id.
    fn window_id(&self) -> WindowId {
        self.window_id
    }

    /// Set the mouse cursor to a predefined symbol or image, or hides cursor if `None`.
    fn cursor(&mut self, cursor: Option<Cursor>) -> Result<()> {
        match cursor {
            Some(cursor) => {
                let cursor = match cursor {
                    Cursor::System(cursor) => SdlCursor::from_system(cursor.into())?,
                    Cursor::Image(path) => {
                        let surface = Surface::from_file(path)?;
                        SdlCursor::from_surface(surface, 0, 0)?
                    }
                };
                cursor.set();
                self.context.mouse().show_cursor(true);
            }
            None => self.context.mouse().show_cursor(false),
        }
        Ok(())
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

impl From<SystemCursor> for SdlSystemCursor {
    fn from(cursor: SystemCursor) -> Self {
        use SdlSystemCursor::*;
        match cursor {
            SystemCursor::Arrow => Arrow,
            SystemCursor::IBeam => IBeam,
            SystemCursor::Wait => Wait,
            SystemCursor::Crosshair => Crosshair,
            SystemCursor::WaitArrow => WaitArrow,
            SystemCursor::SizeNWSE => SizeNWSE,
            SystemCursor::SizeNESW => SizeNESW,
            SystemCursor::SizeWE => SizeWE,
            SystemCursor::SizeNS => SizeNS,
            SystemCursor::SizeAll => SizeAll,
            SystemCursor::No => No,
            SystemCursor::Hand => Hand,
        }
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from(err: IntegerOrSdlError) -> Self {
        use IntegerOrSdlError::*;
        match err {
            IntegerOverflows(s, v) => Self::Overflow(Cow::from(s), v),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}
