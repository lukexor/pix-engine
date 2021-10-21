use super::{Renderer, WindowCanvas};
use crate::{
    core::window::{Error, Position, Result, WindowId},
    prelude::{Cursor, Event, SystemCursor},
    renderer::*,
};
use sdl2::{
    image::LoadSurface,
    mouse::{Cursor as SdlCursor, SystemCursor as SdlSystemCursor},
    render::TextureValueError,
    surface::Surface,
    video::{FullscreenType, WindowBuildError},
    IntegerOrSdlError, Sdl,
};
use std::borrow::Cow;

impl WindowRenderer for Renderer {
    /// Get the primary window ID.
    #[inline]
    fn primary_window_id(&self) -> WindowId {
        self.window_id
    }

    /// Get the current window target ID.
    #[inline]
    fn window_id(&self) -> WindowId {
        self.window_target
    }

    /// Create a new window.
    fn create_window(&mut self, s: &RendererSettings) -> Result<WindowId> {
        let (window_id, canvas) = Self::create_window_canvas(&self.context, s)?;
        let texture_creator = canvas.texture_creator();
        self.canvases.insert(window_id, (canvas, texture_creator));
        Ok(window_id)
    }

    /// Close a window.
    fn close_window(&mut self, id: WindowId) -> Result<()> {
        if id == self.window_target {
            self.reset_window_target();
        }
        self.canvases
            .remove(&id)
            .map_or(Err(Error::InvalidWindow(id)), |_| Ok(()))
    }

    /// Set the mouse cursor to a predefined symbol or image, or hides cursor if `None`.
    fn cursor(&mut self, cursor: Option<&Cursor>) -> Result<()> {
        match cursor {
            Some(cursor) => {
                self.cursor = match cursor {
                    Cursor::System(cursor) => SdlCursor::from_system(cursor.into())?,
                    Cursor::Image(path) => {
                        let surface = Surface::from_file(path)?;
                        SdlCursor::from_surface(surface, 0, 0)?
                    }
                };
                self.cursor.set();
                if !self.context.mouse().is_cursor_showing() {
                    self.context.mouse().show_cursor(true);
                }
            }
            None => self.context.mouse().show_cursor(false),
        }
        Ok(())
    }

    /// Returns a single event or None if the event pump is empty.
    #[inline]
    fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event().map(|evt| evt.into())
    }

    /// Get the current window title.
    #[inline]
    fn title(&self) -> &str {
        &self.settings.title
    }

    /// Set the current window title.
    #[inline]
    fn set_title(&mut self, title: &str) -> Result<()> {
        self.settings.title = title.to_owned();
        let (canvas, _) = self
            .canvases
            .get_mut(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        Ok(canvas.window_mut().set_title(title)?)
    }

    #[inline]
    fn set_fps_title(&mut self, fps: usize) -> Result<()> {
        let (canvas, _) = self
            .canvases
            .get_mut(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        Ok(canvas
            .window_mut()
            .set_title(&format!("{} - FPS: {}", self.settings.title, fps))?)
    }

    /// Dimensions of the primary window as `(width, height)`.
    #[inline]
    fn dimensions(&self) -> Result<(u32, u32)> {
        let (canvas, _) = self
            .canvases
            .get(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        Ok(canvas.window().size())
    }

    /// Set dimensions of the primary window as `(width, height)`.
    #[inline]
    fn set_dimensions(&mut self, (width, height): (u32, u32)) -> Result<()> {
        let (canvas, _) = self
            .canvases
            .get_mut(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        Ok(canvas.window_mut().set_size(width, height)?)
    }

    /// Dimensions of the primary display as `(width, height)`.
    #[inline]
    fn display_dimensions(&self) -> Result<(u32, u32)> {
        let (canvas, _) = self
            .canvases
            .get(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        let window = canvas.window();
        let display_index = window.display_index()?;
        let bounds = window.subsystem().display_usable_bounds(display_index)?;
        Ok((bounds.width(), bounds.height()))
    }

    /// Returns whether the application is fullscreen or not.
    #[inline]
    fn fullscreen(&self) -> Result<bool> {
        use FullscreenType::*;
        let (canvas, _) = self
            .canvases
            .get(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        Ok(matches!(canvas.window().fullscreen_state(), True | Desktop))
    }

    /// Set the application to fullscreen or not.
    #[inline]
    fn set_fullscreen(&mut self, val: bool) -> Result<()> {
        use FullscreenType::*;
        let fullscreen_type = if val { True } else { Off };
        let (canvas, _) = self
            .canvases
            .get_mut(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        Ok(canvas.window_mut().set_fullscreen(fullscreen_type)?)
    }

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    #[inline]
    fn vsync(&self) -> bool {
        self.settings.vsync
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    fn set_vsync(&mut self, val: bool) -> Result<()> {
        let (canvas, _) = self
            .canvases
            .get(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;

        self.settings.vsync = val;
        let window = canvas.window();
        let (x, y) = window.position();
        let (w, h) = window.size();
        self.settings.width = (w as f32 / self.settings.scale_x).floor() as u32;
        self.settings.height = (h as f32 / self.settings.scale_y).floor() as u32;
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self.settings.fullscreen = matches!(
            window.fullscreen_state(),
            FullscreenType::True | FullscreenType::Desktop
        );

        let (window_id, new_canvas) = Self::create_window_canvas(&self.context, &self.settings)?;
        let new_texture_creator = new_canvas.texture_creator();

        self.text_cache.clear();
        self.image_cache.clear();

        self.canvases.remove(&self.window_target);

        if self.window_id == self.window_target {
            self.window_id = window_id;
        }
        self.window_target = window_id;
        self.canvases
            .insert(window_id, (new_canvas, new_texture_creator));
        Ok(())
    }

    /// Set window as the target for drawing operations.
    #[inline]
    fn set_window_target(&mut self, id: WindowId) -> Result<()> {
        if !self.canvases.contains_key(&id) {
            Err(Error::InvalidWindow(id))
        } else {
            self.window_target = id;
            Ok(())
        }
    }

    /// Reset main window as the target for drawing operations.
    #[inline]
    fn reset_window_target(&mut self) {
        self.window_target = self.window_id;
    }

    /// Show the current window target.
    #[inline]
    fn show(&mut self) -> Result<()> {
        let (canvas, _) = self
            .canvases
            .get_mut(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        canvas.window_mut().show();
        Ok(())
    }

    /// Hide the current window target.
    #[inline]
    fn hide(&mut self) -> Result<()> {
        let (canvas, _) = self
            .canvases
            .get_mut(&self.window_target)
            .ok_or(WindowError::InvalidWindow(self.window_target))?;
        canvas.window_mut().hide();
        Ok(())
    }
}

impl Renderer {
    pub(crate) fn create_window_canvas(
        context: &Sdl,
        s: &RendererSettings,
    ) -> Result<(WindowId, WindowCanvas)> {
        let video_subsys = context.video()?;

        // TODO: more testing - macOS performance seems low with default "metal" renderer
        // However: https://github.com/libsdl-org/SDL/issues/4001
        #[cfg(feature = "opengl")]
        {
            sdl2::hint::set_with_priority(
                "SDL_RENDER_DRIVER",
                "opengl",
                &sdl2::hint::Hint::Override,
            );
        }

        // Set up window with options
        let win_width = (s.scale_x * s.width as f32).floor() as u32;
        let win_height = (s.scale_y * s.height as f32).floor() as u32;
        let mut window_builder = video_subsys.window(&s.title, win_width, win_height);
        #[cfg(feature = "opengl")]
        {
            window_builder.opengl();
        }
        match (s.x, s.y) {
            (Position::Centered, Position::Centered) => {
                window_builder.position_centered();
            }
            (Position::Positioned(x), Position::Positioned(y)) => {
                window_builder.position(x, y);
            }
            _ => return Err(Error::InvalidPosition(s.x, s.y)),
        };
        if s.fullscreen {
            window_builder.fullscreen();
        }
        if s.resizable {
            window_builder.resizable();
        }
        if s.borderless {
            window_builder.borderless();
        }
        if s.allow_highdpi {
            window_builder.allow_highdpi();
        }
        if s.hidden {
            window_builder.hidden();
        }

        let window = window_builder.build()?;

        let window_id = window.id() as usize;
        let mut canvas_builder = window.into_canvas().accelerated().target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build()?;
        canvas.set_logical_size(win_width, win_height)?;
        canvas.set_scale(s.scale_x, s.scale_y)?;

        if let Some(icon) = &s.icon {
            let surface = Surface::from_file(icon)?;
            canvas.window_mut().set_icon(surface);
        }

        Ok((window_id, canvas))
    }
}

impl From<&SystemCursor> for SdlSystemCursor {
    fn from(cursor: &SystemCursor) -> Self {
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

impl From<WindowBuildError> for Error {
    fn from(err: WindowBuildError) -> Self {
        use WindowBuildError::*;
        match err {
            HeightOverflows(h) => Self::Overflow(Cow::from("window height"), h),
            WidthOverflows(w) => Self::Overflow(Cow::from("window width"), w),
            InvalidTitle(e) => Self::InvalidText("invalid title", e),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<TextureValueError> for Error {
    fn from(err: TextureValueError) -> Self {
        use TextureValueError::*;
        match err {
            HeightOverflows(h) => Self::Overflow(Cow::from("texture height"), h),
            WidthOverflows(w) => Self::Overflow(Cow::from("texture width"), w),
            WidthMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("width must be multiple of 2"))
            }
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}
