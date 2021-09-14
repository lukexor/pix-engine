use super::Renderer;
use crate::{
    core::window::{Error, Position, Result, Window, WindowId},
    prelude::{Cursor, Event, SystemCursor},
    renderer::RendererSettings,
};
use sdl2::{
    image::LoadSurface,
    mouse::{Cursor as SdlCursor, SystemCursor as SdlSystemCursor},
    render::{Canvas, TextureQuery, TextureValueError},
    surface::Surface,
    video::{FullscreenType, Window as SdlWindow, WindowBuildError},
    IntegerOrSdlError, Sdl,
};
use std::borrow::Cow;

impl Window for Renderer {
    /// Get the primary window id.
    fn window_id(&self) -> WindowId {
        self.window_id
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
    fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event().map(|evt| evt.into())
    }

    /// Get the current window title.
    fn title(&self) -> &str {
        &self.settings.title
    }

    /// Set the current window title.
    fn set_title(&mut self, title: &str) -> Result<()> {
        self.settings.title = title.to_owned();
        Ok(self.canvas.window_mut().set_title(title)?)
    }

    #[inline(always)]
    fn set_fps_title(&mut self, fps: usize) -> Result<()> {
        Ok(self
            .canvas
            .window_mut()
            .set_title(&format!("{} - FPS: {}", self.settings.title, fps))?)
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
        let (width, height) = if id == self.window_id {
            self.canvas.window().size()
        } else {
            todo!("secondary windows are not yet implemented");
        };
        Ok((width, height))
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

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    fn vsync(&self) -> bool {
        self.settings.vsync
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    fn set_vsync(&mut self, val: bool) -> Result<()> {
        self.settings.vsync = val;
        let window = self.canvas.window();
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

        let (window_id, canvas) = Self::create_window_canvas(&self.context, &self.settings)?;
        self.window_id = window_id;
        self.texture_creator = canvas.texture_creator();
        let mut textures = Vec::with_capacity(self.textures.len());
        for texture in &self.textures {
            let TextureQuery {
                width,
                height,
                format,
                ..
            } = texture.query();
            textures.push(
                self.texture_creator
                    .create_texture_target(format, width, height)?,
            );
        }
        self.textures = textures;
        self.canvas = canvas;
        Ok(())
    }
}

impl Renderer {
    pub(crate) fn create_window_canvas(
        context: &Sdl,
        s: &RendererSettings,
    ) -> Result<(WindowId, Canvas<SdlWindow>)> {
        let video_subsys = context.video()?;

        // TODO: more testing - macOS performance seems low with default "metal" renderer
        // However: https://github.com/libsdl-org/SDL/issues/4001
        if cfg!(feature = "opengl") {
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

        let window = window_builder.build()?;
        let window_id = window.id() as usize;
        let mut canvas_builder = window.into_canvas().accelerated().target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build()?;
        canvas.set_logical_size(win_width, win_height)?;
        canvas.set_scale(s.scale_x, s.scale_y)?;

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
