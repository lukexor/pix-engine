use super::{texture::RendererTexture, FontId, Renderer};
use crate::{
    image::Icon,
    prelude::*,
    renderer::{RendererSettings, WindowRenderer},
    window::{Position, WindowId},
};
use anyhow::{bail, Context};
use lru::LruCache;
use sdl2::{
    image::LoadSurface,
    mouse::{Cursor as SdlCursor, SystemCursor as SdlSystemCursor},
    render::{Canvas, TextureQuery},
    surface::Surface,
    video::{FullscreenType, Window},
    Sdl,
};
use std::{
    cell::RefCell,
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::{self, Write},
    hash::{Hash, Hasher},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) struct TextCacheKey {
    pub(super) text_id: FontId,
    pub(super) font_id: FontId,
    pub(super) color: Color,
    pub(super) size: u16,
}

impl TextCacheKey {
    pub(super) fn new(text: &str, font_id: FontId, color: Color, size: u16) -> Self {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let text_id = hasher.finish();
        Self {
            text_id,
            font_id,
            color,
            size,
        }
    }
}

impl Renderer {
    /// Returns the current window canvas, holding the canvas and texture creators for a window.
    #[inline]
    pub(super) fn window_canvas(&self) -> PixResult<&WindowCanvas> {
        Ok(self
            .windows
            .get(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?)
    }

    /// Returns the current window canvas, holding the canvas and texture creators for a window.
    #[inline]
    pub(super) fn window_canvas_mut(&mut self) -> PixResult<&mut WindowCanvas> {
        Ok(self
            .windows
            .get_mut(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?)
    }

    /// Returns the current SDL canvas.
    #[inline]
    pub(super) fn canvas(&self) -> PixResult<&Canvas<Window>> {
        Ok(&self.window_canvas()?.canvas)
    }

    /// Returns the current SDL canvas.
    #[inline]
    pub(super) fn canvas_mut(&mut self) -> PixResult<&mut Canvas<Window>> {
        Ok(&mut self.window_canvas_mut()?.canvas)
    }

    /// Returns the current SDL window.
    #[inline]
    pub(super) fn window(&self) -> PixResult<&Window> {
        Ok(self.window_canvas()?.canvas.window())
    }

    /// Returns the current SDL window.
    #[inline]
    pub(super) fn window_mut(&mut self) -> PixResult<&mut Window> {
        Ok(self.window_canvas_mut()?.canvas.window_mut())
    }
}

pub(super) struct WindowCanvas {
    pub(super) id: WindowId,
    pub(super) canvas: Canvas<Window>,
    pub(super) textures: HashMap<TextureId, RefCell<RendererTexture>>,
    pub(super) text_cache: LruCache<TextCacheKey, RendererTexture>,
    pub(super) image_cache: LruCache<*const Image, RendererTexture>,
}

impl WindowCanvas {
    pub(super) fn new(context: &Sdl, s: &mut RendererSettings) -> PixResult<Self> {
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
        let mut window_builder = video_subsys.window(&s.title, s.width, s.height);
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

        let window_id = WindowId(window.id());
        let mut canvas_builder = window.into_canvas().accelerated().target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build().context("failed to build canvas")?;
        log::debug!("Using SDL Renderer `{}`", canvas.info().name);
        canvas
            .set_logical_size(s.width, s.height)
            .context("invalid logical canvas size")?;
        canvas
            .set_scale(s.scale_x, s.scale_y)
            .map_err(PixError::Renderer)?;

        if let Some(ref mut icon) = s.icon {
            let surface = match icon {
                Icon::Image(ref mut img) => {
                    let width = img.width();
                    let height = img.height();
                    let pitch = img.pitch() as u32;
                    let format = img.format().into();
                    let bytes = img.as_mut_bytes();
                    Surface::from_data(bytes, width, height, pitch, format)
                        .map_err(PixError::Renderer)?
                }
                Icon::Path(ref path) => Surface::from_file(path).map_err(PixError::Renderer)?,
            };
            canvas.window_mut().set_icon(surface);
        }

        log::debug!("Created new window: {}", window_id);
        Ok(Self {
            id: window_id,
            canvas,
            textures: HashMap::new(),
            text_cache: LruCache::new(s.text_cache_size),
            image_cache: LruCache::new(s.texture_cache_size),
        })
    }
}

#[doc(hidden)]
impl fmt::Debug for WindowCanvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let canvas = &self.canvas;
        f.debug_struct("WindowCanvas")
            .field("id", &self.id)
            .field("title", &canvas.window().title())
            .field("dimensions", &canvas.output_size())
            .field("scale", &canvas.scale())
            .field("draw_color", &canvas.draw_color())
            .field("clip", &canvas.clip_rect())
            .field("texture_count", &self.textures.len())
            .field("text_cache", &self.text_cache)
            .field("image_cache", &self.image_cache)
            .finish_non_exhaustive()
    }
}

impl WindowRenderer for Renderer {
    /// Get the count of open windows.
    fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Get the current window target ID.
    #[inline]
    fn window_id(&self) -> WindowId {
        self.window_target
    }

    /// Create a new window.
    fn create_window(&mut self, s: &mut RendererSettings) -> PixResult<WindowId> {
        let window_canvas = WindowCanvas::new(&self.context, s)?;
        let window_id = window_canvas.id;
        self.windows.insert(window_id, window_canvas);
        Ok(window_id)
    }

    /// Close a window.
    fn close_window(&mut self, id: WindowId) -> PixResult<()> {
        if self.windows.remove(&id).is_none() {
            return Err(PixError::InvalidWindow(id).into());
        }
        if id == self.window_target {
            if id == self.primary_window_id {
                if let Some(id) = self.windows.keys().last() {
                    self.window_target = *id;
                    self.primary_window_id = self.window_target;
                } else {
                    bail!("close_window can not be called on the last window, call quit() instead");
                }
            } else {
                self.reset_window_target();
            }
        }
        Ok(())
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
        self.event_pump.poll_event().map(Into::into)
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
        self.window_mut()?.set_title(title).context("invalid title")
    }

    #[inline]
    fn set_fps(&mut self, fps: f32) -> PixResult<()> {
        self.title.clear();
        write!(self.title, "{} - FPS: {:.02}", &self.settings.title, fps)
            .context("invalid title")?;
        // Can't use `self.window_mut` here due to needing split borrows
        self.windows
            .get_mut(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?
            .canvas
            .window_mut()
            .set_title(&self.title)
            .context("invalid title")
    }

    /// Dimensions of the current render target as `(width, height)`.
    #[inline]
    fn dimensions(&self) -> PixResult<(u32, u32)> {
        self.texture_target.map_or_else(
            || self.window_dimensions(),
            |texture_id| {
                self.windows
                    .values()
                    .find_map(|w| w.textures.get(&texture_id))
                    .map_or_else(
                        || Err(PixError::InvalidTexture(texture_id).into()),
                        |texture| {
                            let query = texture.borrow().query();
                            Ok((query.width, query.height))
                        },
                    )
            },
        )
    }

    /// Dimensions of the current window target as `(width, height)`.
    #[inline]
    fn window_dimensions(&self) -> PixResult<(u32, u32)> {
        Ok(self.window()?.size())
    }

    /// Set dimensions of the current window target as `(width, height)`.
    #[inline]
    fn set_window_dimensions(&mut self, (width, height): (u32, u32)) -> PixResult<()> {
        self.settings.width = width;
        self.settings.height = height;
        let canvas = self.canvas_mut()?;
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
    #[inline]
    fn viewport(&self) -> PixResult<Rect<i32>> {
        Ok(self.canvas()?.viewport().into())
    }

    /// Set the rendering viewport of the current render target.
    #[inline]
    fn set_viewport(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        self.canvas_mut()?.set_viewport(rect.map(Into::into));
        Ok(())
    }

    /// Dimensions of the primary display as `(width, height)`.
    #[inline]
    fn display_dimensions(&self) -> PixResult<(u32, u32)> {
        let window = self.window()?;
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
        use FullscreenType::{Desktop, True};
        Ok(matches!(self.window()?.fullscreen_state(), True | Desktop))
    }

    /// Set the application to fullscreen or not.
    #[inline]
    fn set_fullscreen(&mut self, val: bool) -> PixResult<()> {
        use FullscreenType::{Off, True};
        let fullscreen_type = if val { True } else { Off };
        Ok(self
            .window_mut()?
            .set_fullscreen(fullscreen_type)
            .map_err(PixError::Renderer)?)
    }

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    #[inline]
    fn vsync(&self) -> bool {
        self.settings.vsync
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    ///
    /// # Note
    ///
    /// Due to the current limitation with changing VSync at runtime, this method creates a new
    /// window using the properties of the current window and returns the new `WindowId`.
    ///
    /// If you are storing and interacting with this window using the `WindowId`, make sure to
    /// use the newly returned `WindowId`.
    fn set_vsync(&mut self, val: bool) -> PixResult<WindowId> {
        log::debug!("Set VSync: {}", val);
        let window_canvas = self
            .windows
            .get_mut(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?;
        let window = window_canvas.canvas.window();
        let (x, y) = window.position();
        let (w, h) = window.size();
        self.settings.width = w;
        self.settings.height = h;
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self.settings.vsync = val;
        self.settings.fullscreen = matches!(
            window.fullscreen_state(),
            FullscreenType::True | FullscreenType::Desktop
        );

        let mut new_window = WindowCanvas::new(&self.context, &mut self.settings)?;

        let previous_window_id = self.window_target;
        for (texture_id, texture) in &window_canvas.textures {
            let TextureQuery {
                width,
                height,
                format,
                ..
            } = texture.borrow().query();
            new_window.textures.insert(
                *texture_id,
                RefCell::new(RendererTexture::new(
                    new_window
                        .canvas
                        .create_texture_target(format, width, height)?,
                )),
            );
        }

        self.windows.remove(&previous_window_id);
        self.window_target = new_window.id;
        self.windows.insert(new_window.id, new_window);
        Ok(self.window_target)
    }

    /// Set window as the target for drawing operations.
    #[inline]
    fn set_window_target(&mut self, id: WindowId) -> PixResult<()> {
        if self.windows.contains_key(&id) {
            self.window_target = id;
            Ok(())
        } else {
            Err(PixError::InvalidWindow(id).into())
        }
    }

    /// Reset main window as the target for drawing operations.
    #[inline]
    fn reset_window_target(&mut self) {
        self.window_target = self.primary_window_id;
    }

    /// Show the current window target.
    #[inline]
    fn show(&mut self) -> PixResult<()> {
        self.window_mut()?.show();
        Ok(())
    }

    /// Hide the current window target.
    #[inline]
    fn hide(&mut self) -> PixResult<()> {
        self.window_mut()?.hide();
        Ok(())
    }
}

#[doc(hidden)]
impl From<SystemCursor> for SdlSystemCursor {
    fn from(cursor: SystemCursor) -> Self {
        match cursor {
            SystemCursor::Arrow => Self::Arrow,
            SystemCursor::IBeam => Self::IBeam,
            SystemCursor::Wait => Self::Wait,
            SystemCursor::Crosshair => Self::Crosshair,
            SystemCursor::WaitArrow => Self::WaitArrow,
            SystemCursor::SizeNWSE => Self::SizeNWSE,
            SystemCursor::SizeNESW => Self::SizeNESW,
            SystemCursor::SizeWE => Self::SizeWE,
            SystemCursor::SizeNS => Self::SizeNS,
            SystemCursor::SizeAll => Self::SizeAll,
            SystemCursor::No => Self::No,
            SystemCursor::Hand => Self::Hand,
        }
    }
}
