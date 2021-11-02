use super::{Renderer, WindowCanvas};
use crate::{
    ops::LruCacheExt,
    prelude::{Cursor, Event, SystemCursor},
    renderer::*,
    window::{Position, WindowId},
};
use anyhow::Context;
use sdl2::{
    image::LoadSurface,
    mouse::{Cursor as SdlCursor, SystemCursor as SdlSystemCursor},
    surface::Surface,
    video::FullscreenType,
    Sdl,
};
use std::fmt::Write;

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
    fn create_window(&mut self, s: &RendererSettings) -> PixResult<WindowId> {
        let (window_id, canvas) = Self::create_window_canvas(&self.context, s)?;
        let texture_creator = canvas.texture_creator();
        self.canvases.insert(window_id, (canvas, texture_creator));
        Ok(window_id)
    }

    /// Close a window.
    fn close_window(&mut self, id: WindowId) -> PixResult<()> {
        if id == self.window_target {
            self.reset_window_target();
        }
        self.text_cache.retain(|key, _| key.0 != id);
        self.image_cache.retain(|key, _| key.0 != id);
        self.canvases
            .remove(&id)
            .map_or(Err(PixError::InvalidWindow(id).into()), |_| Ok(()))
    }

    /// Set the mouse cursor to a predefined symbol or image, or hides cursor if `None`.
    fn cursor(&mut self, cursor: Option<&Cursor>) -> PixResult<()> {
        match cursor {
            Some(cursor) => {
                self.cursor = match cursor {
                    Cursor::System(cursor) => {
                        SdlCursor::from_system((*cursor).into()).map_err(PixError::Renderer)?
                    }
                    Cursor::Image(path, (x, y)) => {
                        let surface = Surface::from_file(path).map_err(PixError::Renderer)?;
                        SdlCursor::from_surface(surface, *x, *y).map_err(PixError::Renderer)?
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
    fn set_title(&mut self, title: &str) -> PixResult<()> {
        self.settings.title.replace_range(.., title);
        let window = get_window_mut!(self);
        window.set_title(title).context("invalid title")
    }

    #[inline]
    fn set_fps(&mut self, fps: usize) -> PixResult<()> {
        self.fps = fps;
        self.title.clear();
        write!(self.title, "{} - FPS: {}", &self.settings.title, self.fps)
            .context("invalid title")?;
        let window = get_window_mut!(self);
        window.set_title(&self.title).context("invalid title")
    }

    /// Dimensions of the current render target as `(width, height)`.
    #[inline]
    fn dimensions(&self) -> PixResult<(u32, u32)> {
        if let Some(texture_id) = self.texture_target {
            if let Some((_, texture)) = self.textures.get(texture_id) {
                let query = texture.query();
                Ok((query.width, query.height))
            } else {
                Err(PixError::InvalidTexture(texture_id).into())
            }
        } else {
            self.window_dimensions()
        }
    }

    /// Dimensions of the current window target as `(width, height)`.
    #[inline]
    fn window_dimensions(&self) -> PixResult<(u32, u32)> {
        Ok(get_window!(self).size())
    }

    /// Set dimensions of the current window target as `(width, height)`.
    #[inline]
    fn set_window_dimensions(&mut self, (width, height): (u32, u32)) -> PixResult<()> {
        self.settings.width = width;
        self.settings.height = height;
        let canvas = get_canvas_mut!(self);
        canvas
            .window_mut()
            .set_size(width, height)
            .context("invalid window dimensions")?;
        canvas
            .set_logical_size(width, height)
            .context("invalid logical window size")?;
        Ok(())
    }

    /// Returns the rendering viewport of the current render target.
    fn viewport(&self) -> PixResult<Rect<i32>> {
        Ok(get_canvas!(self).viewport().into())
    }

    /// Set the rendering viewport of the current render target.
    fn set_viewport(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        get_canvas_mut!(self).set_viewport(rect.map(|r| r.into()));
        Ok(())
    }

    /// Dimensions of the primary display as `(width, height)`.
    #[inline]
    fn display_dimensions(&self) -> PixResult<(u32, u32)> {
        let window = get_window!(self);
        let display_index = window.display_index().map_err(PixError::Renderer)?;
        let bounds = window
            .subsystem()
            .display_usable_bounds(display_index)
            .map_err(PixError::Renderer)?;
        Ok((bounds.width(), bounds.height()))
    }

    /// Returns whether the application is fullscreen or not.
    #[inline]
    fn fullscreen(&self) -> PixResult<bool> {
        use FullscreenType::*;
        Ok(matches!(
            get_window!(self).fullscreen_state(),
            True | Desktop
        ))
    }

    /// Set the application to fullscreen or not.
    #[inline]
    fn set_fullscreen(&mut self, val: bool) -> PixResult<()> {
        use FullscreenType::*;
        let fullscreen_type = if val { True } else { Off };
        Ok(get_window_mut!(self)
            .set_fullscreen(fullscreen_type)
            .map_err(PixError::Renderer)?)
    }

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    #[inline]
    fn vsync(&self) -> bool {
        self.settings.vsync
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    fn set_vsync(&mut self, val: bool) -> PixResult<()> {
        let window = get_window!(self);
        let (x, y) = window.position();
        let (w, h) = window.size();
        self.settings.width = (w as f32 / self.settings.scale_x).floor() as u32;
        self.settings.height = (h as f32 / self.settings.scale_y).floor() as u32;
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self.settings.vsync = val;
        self.settings.fullscreen = matches!(
            window.fullscreen_state(),
            FullscreenType::True | FullscreenType::Desktop
        );

        let (window_id, new_canvas) = Self::create_window_canvas(&self.context, &self.settings)?;
        let new_texture_creator = new_canvas.texture_creator();

        let previous_window_id = self.window_target;
        self.text_cache.retain(|key, _| key.0 != previous_window_id);
        self.image_cache
            .retain(|key, _| key.0 != previous_window_id);

        self.canvases.remove(&previous_window_id);

        if self.window_id == previous_window_id {
            self.window_id = window_id;
        }
        self.window_target = window_id;
        self.canvases
            .insert(window_id, (new_canvas, new_texture_creator));
        Ok(())
    }

    /// Set window as the target for drawing operations.
    #[inline]
    fn set_window_target(&mut self, id: WindowId) -> PixResult<()> {
        if self.canvases.contains_key(&id) {
            self.window_target = id;
            Ok(())
        } else {
            Err(PixError::InvalidWindow(id).into())
        }
    }

    /// Reset main window as the target for drawing operations.
    #[inline]
    fn reset_window_target(&mut self) {
        self.window_target = self.window_id;
    }

    /// Show the current window target.
    #[inline]
    fn show(&mut self) -> PixResult<()> {
        get_window_mut!(self).show();
        Ok(())
    }

    /// Hide the current window target.
    #[inline]
    fn hide(&mut self) -> PixResult<()> {
        get_window_mut!(self).hide();
        Ok(())
    }
}

impl Renderer {
    pub(crate) fn create_window_canvas(
        context: &Sdl,
        s: &RendererSettings,
    ) -> PixResult<(WindowId, WindowCanvas)> {
        let video_subsys = context.video().map_err(PixError::Renderer)?;

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
            _ => unreachable!("invalid window position combination"),
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

        let window = window_builder.build().context("failed to build window")?;

        let window_id = window.id() as usize;
        let mut canvas_builder = window.into_canvas().accelerated().target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build().context("failed to build canvas")?;
        canvas
            .set_logical_size(win_width, win_height)
            .context("invalid logical canvas size")?;
        canvas
            .set_scale(s.scale_x, s.scale_y)
            .map_err(PixError::Renderer)?;

        if let Some(icon) = &s.icon {
            let surface = Surface::from_file(icon).map_err(PixError::Renderer)?;
            canvas.window_mut().set_icon(surface);
        }

        Ok((window_id, canvas))
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
