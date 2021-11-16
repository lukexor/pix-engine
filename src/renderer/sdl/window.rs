use super::{texture::RendererTexture, FontId, Renderer};
use crate::{
    prelude::{Cursor, Event, SystemCursor},
    renderer::*,
    window::{Position, WindowId},
};
use anyhow::Context;
use lru::LruCache;
use sdl2::{
    image::LoadSurface,
    mouse::{Cursor as SdlCursor, SystemCursor as SdlSystemCursor},
    render::{Canvas, TextureCreator, TextureQuery},
    surface::Surface,
    ttf::Font as SdlFont,
    video::{FullscreenType, Window, WindowContext},
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

pub(super) struct WindowCanvas {
    pub(super) id: WindowId,
    pub(super) canvas: Canvas<Window>,
    pub(super) texture_creator: TextureCreator<WindowContext>,
    pub(super) textures: HashMap<TextureId, RefCell<RendererTexture>>,
    pub(super) text_cache: LruCache<TextCacheKey, RendererTexture>,
    pub(super) image_cache: LruCache<*const Image, RendererTexture>,
}

impl WindowCanvas {
    pub(super) fn new(context: &Sdl, s: &RendererSettings) -> PixResult<Self> {
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

        let texture_creator = canvas.texture_creator();

        Ok(Self {
            id: window_id,
            canvas,
            texture_creator,
            textures: HashMap::new(),
            text_cache: LruCache::new(s.text_cache_size),
            image_cache: LruCache::new(s.texture_cache_size),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn text_texture_mut<'a>(
        text_cache: &'a mut LruCache<TextCacheKey, RendererTexture>,
        texture_creator: &TextureCreator<WindowContext>,
        text: &str,
        wrap_width: Option<u32>,
        fill: Color,
        outline: u8,
        font: &mut SdlFont<'static, 'static>,
        current_font: FontId,
        font_size: u16,
    ) -> PixResult<&'a mut RendererTexture> {
        let current_outline = font.get_outline_width();
        if current_outline != outline as u16 {
            font.set_outline_width(outline as u16);
        }

        let key = TextCacheKey::new(text, current_font, fill, font_size);
        if !text_cache.contains(&key) {
            let surface = if let Some(width) = wrap_width {
                font.render(text).blended_wrapped(fill, width)
            } else {
                font.render(text).blended(fill)
            }
            .context("invalid text")?;
            text_cache.put(
                key,
                RendererTexture::new(
                    surface
                        .as_texture(texture_creator)
                        .context("failed to create text surface")?,
                ),
            );
        }

        // SAFETY: We just checked or inserted a texture.
        Ok(text_cache.get_mut(&key).expect("valid text cache"))
    }

    pub(super) fn image_texture_mut<'a>(
        image_cache: &'a mut LruCache<*const Image, RendererTexture>,
        texture_creator: &TextureCreator<WindowContext>,
        img: &Image,
    ) -> PixResult<&'a mut RendererTexture> {
        let key: *const Image = img;
        if !image_cache.contains(&key) {
            image_cache.put(
                key,
                RendererTexture::new(
                    texture_creator
                        .create_texture_static(Some(img.format().into()), img.width(), img.height())
                        .context("failed to create image texture")?,
                ),
            );
        }
        // SAFETY: We just checked or inserted a texture.
        Ok(image_cache.get_mut(&key).expect("valid image cache"))
    }
}

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
    fn create_window(&mut self, s: &RendererSettings) -> PixResult<WindowId> {
        let window_canvas = WindowCanvas::new(&self.context, s)?;
        let window_id = window_canvas.id;
        self.windows.insert(window_id, window_canvas);
        Ok(window_id)
    }

    /// Close a window.
    fn close_window(&mut self, id: WindowId) -> PixResult<()> {
        if id == self.window_target {
            self.reset_window_target();
        }
        self.windows
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
        self.window_mut()?.set_title(title).context("invalid title")
    }

    #[inline]
    fn set_fps(&mut self, fps: usize) -> PixResult<()> {
        self.fps = fps;
        self.title.clear();
        write!(self.title, "{} - FPS: {}", &self.settings.title, self.fps)
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
        if let Some(texture_id) = self.texture_target {
            if let Some(texture) = self
                .windows
                .values()
                .find_map(|w| w.textures.get(&texture_id))
            {
                let query = texture.borrow().query();
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
    fn viewport(&self) -> PixResult<Rect<i32>> {
        Ok(self.canvas()?.viewport().into())
    }

    /// Set the rendering viewport of the current render target.
    fn set_viewport(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        self.canvas_mut()?.set_viewport(rect.map(|r| r.into()));
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
        use FullscreenType::*;
        Ok(matches!(self.window()?.fullscreen_state(), True | Desktop))
    }

    /// Set the application to fullscreen or not.
    #[inline]
    fn set_fullscreen(&mut self, val: bool) -> PixResult<()> {
        use FullscreenType::*;
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
    fn set_vsync(&mut self, val: bool) -> PixResult<()> {
        let window_canvas = self
            .windows
            .get_mut(&self.window_target)
            .ok_or(PixError::InvalidWindow(self.window_target))?;
        let window = window_canvas.canvas.window();
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

        let mut new_window = WindowCanvas::new(&self.context, &self.settings)?;
        let new_texture_creator = new_window.canvas.texture_creator();

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
                    new_texture_creator.create_texture_target(format, width, height)?,
                )),
            );
        }

        self.windows.remove(&previous_window_id);
        self.window_target = new_window.id;
        self.windows.insert(new_window.id, new_window);
        Ok(())
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
        self.window_target = self.primary_window_id
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

impl Renderer {}

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
