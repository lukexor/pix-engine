//! `Texture` methods.
//!
//! Provides texture creation and rendering methods on [`PixState`].
//!
//! Provided methods:
//!
//! - [`PixState::texture`]: Render a portion of a texture to the current canvas.
//! - [`PixState::texture_transformed`]: Render a transformed portion of a texture to the current
//!   canvas.
//! - [`PixState::create_texture`]: Creates a new texture to render to.
//! - [`PixState::delete_texture`]: Delete a texture.
//! - [`PixState::update_texture`]: Update texture with [u8] [slice] of pixel data.
//! - [`PixState::with_texture`]: Target a texture for rendering.
//! - [`PixState::save_texture`]: Save a texture to a [png] file.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App;
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     let texture_id1 = s.create_texture(500, 600, PixelFormat::Rgb)?;
//!     // Does not actually render to the current canvas
//!     s.with_texture(texture_id1, |s: &mut PixState| -> PixResult<()> {
//!         s.background(Color::random());
//!         s.text("Rendered texture!")?;
//!         Ok(())
//!     })?;
//!
//!     // `None` uses PixelFormat::default() which defaults to PixelFormat::Rgba
//!     let texture_id2 = s.create_texture(500, 600, None)?;
//!
//!     // `None` updates the entire texture, pass a Rect<i32> to update a sub-rectangle area
//!     let image = Image::from_file("./some_image.png")?;
//!     let pitch = image.width() as usize;
//!     s.update_texture(texture_id2, None, image.as_bytes(), pitch)?;
//!
//!     // Draw both textures to the current canvas
//!     s.texture(texture_id1, None, rect![0, 0, 500, 600])?;
//!     s.texture(texture_id2, None, rect![500, 0, 500, 600])?;
//!
//!     // These could be stored in `self` to avoid re-creating every frame
//!     s.delete_texture(texture_id1)?;
//!     s.delete_texture(texture_id2)?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::prelude::*;
use log::{info, trace};
use std::{
    fmt,
    ops::{Deref, DerefMut},
    path::Path,
};

/// `Texture` identifier used to reference and target an internally managed texture.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextureId(pub(crate) usize);

impl fmt::Display for TextureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for TextureId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PixState {
    /// Draw a portion `src` of a texture to the current render target translated and resized to
    /// the target `dst`. Passing `None` for `src` renders the entire texture. Passing `None` for
    /// `dst` renders to the maximum size of the render target.
    ///
    /// # Note
    ///
    /// It's possible to render one texture onto another texture, but currently they both have to
    /// have been created in the same window. Attempting to render to a texture created with
    /// another window will result in a [`PixError::InvalidTexture`]. This restriction may be
    /// lifted in the future.
    ///
    /// # Errors
    ///
    /// Returns an error for any of the following:
    ///     - The current render target is closed or dropped.
    ///     - The texture being rendered has been dropped.
    ///     - The target texture is the same as the texture being rendered.
    ///     - The renderer fails to draw to the texture.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { texture_id: TextureId };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.with_texture(self.texture_id, |s: &mut PixState| -> PixResult<()> {
    ///         s.background(Color::random());
    ///         s.text("Rendered texture!")?;
    ///         Ok(())
    ///     })?;
    ///     let src = rect![10, 10, 100, 100]; // Render a sub-section of the texture
    ///     // translate and scale texture
    ///     let dst = rect![200, 200, 200, 200];
    ///     s.texture(self.texture_id, src, dst)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn texture<R1, R2>(&mut self, texture_id: TextureId, src: R1, dst: R2) -> PixResult<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
    {
        self.renderer
            .texture(texture_id, src.into(), dst.into(), 0.0, None, None, None)
    }

    /// Draw a transformed portion `src` of a texture to the current render target translated and
    /// resized to the target `dst`, optionally rotated by an `angle` about a `center` point or
    /// `flipped`. `angle` can be in either radians or degrees based on [`AngleMode`]. Passing
    /// `None` for `src` renders the entire texture. Passing `None` for `dst` renders to the
    /// maximum size of the render target. [`PixState::image_tint`] can optionally add a tint color
    /// to the rendered texture.
    ///
    /// # Note
    ///
    /// It's possible to render one texture onto another texture, but currently they both have to
    /// have been created in the same window. Attempting to render to a texture created with
    /// another window will result in a [`PixError::InvalidTexture`]. This restriction may be
    /// lifted in the future.
    ///
    /// # Errors
    ///
    /// Returns an error for any of the following:
    ///     - The current render target is closed or dropped.
    ///     - The texture being rendered has been dropped.
    ///     - The target texture is the same as the texture being rendered.
    ///     - The renderer fails to draw to the texture.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { texture_id: TextureId };
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.with_texture(self.texture_id, |s: &mut PixState| -> PixResult<()> {
    ///         s.background(Color::random());
    ///         s.text("Rendered texture!")?;
    ///         Ok(())
    ///     })?;
    ///     let src = None;
    ///     // translate and scale texture
    ///     let dst = rect![200, 200, 200, 200];
    ///     let angle = 10.0;
    ///     let center = point!(10, 10);
    ///     s.texture_transformed(
    ///         self.texture_id,
    ///         src,
    ///         dst,
    ///         angle,
    ///         center,
    ///         Flipped::Horizontal,
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn texture_transformed<R1, R2, C, F>(
        &mut self,
        texture_id: TextureId,
        src: R1,
        dst: R2,
        mut angle: Scalar,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
    {
        let s = &self.settings;
        if let AngleMode::Radians = s.angle_mode {
            angle = angle.to_degrees();
        };
        self.renderer.texture(
            texture_id,
            src.into(),
            dst.into(),
            angle,
            center.into(),
            flipped.into(),
            s.image_tint,
        )
    }

    /// Constructs a `Texture` to render to. Passing `None` for [`PixelFormat`] will use
    /// [`PixelFormat::default`].
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the texture dimensions are invalid,
    /// then an error is returned.
    ///
    /// # Note
    ///
    /// Textures are automatically dropped when the window they were created in is closed due to an
    /// implicit lifetime that the texture can not outlive the window it was created for. Calling
    /// this method will associate the texture to the current `window_target`, which can only be
    /// changed using the [`PixState::with_window`] method. It is the responsibility of the caller to
    /// manage created textures and call [`PixState::delete_texture`] when a texture resource is no
    /// longer needed and to ensure that texture methods are not called for a given window after it
    /// has been closed, otherwise an error will be returned.
    ///
    /// This constraint arises due to lifetime issues with SDL textures, See
    /// <https://github.com/Rust-SDL2/rust-sdl2/issues/1107> for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { texture_id: TextureId };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     self.texture_id = s.create_texture(
    ///         s.width()? / 2,
    ///         s.height()? / 2,
    ///         PixelFormat::Rgb,
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn create_texture<F>(&mut self, width: u32, height: u32, format: F) -> PixResult<TextureId>
    where
        F: Into<Option<PixelFormat>>,
    {
        self.renderer.create_texture(width, height, format.into())
    }

    /// Delete a `Texture`.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the texture has already been dropped,
    /// then an error is returned.
    ///
    /// # Note
    ///
    /// Currently, it is up to the caller to manage valid textures. Textures become invalid
    /// whenever the `window_target` they were created in has been closed. Calling any texture
    /// methods with an invalid `TextureId` will result in an error.
    ///
    /// This constraint arises due to lifetime issues with SDL textures, See
    /// <https://github.com/Rust-SDL2/rust-sdl2/issues/1107> for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     // A more polished implementation would manage state in `self` to avoid re-creating and
    ///     // destroying textures every frame
    ///     let texture_id = s.create_texture(500, 600, PixelFormat::Rgb)?;
    ///     // Render things
    ///     s.delete_texture(texture_id)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn delete_texture(&mut self, texture_id: TextureId) -> PixResult<()> {
        self.renderer.delete_texture(texture_id)
    }

    /// Update the `Texture` with a [u8] [slice] of pixel data. Passing `None` for `rect` updates
    /// the entire texture. `pitch` is the number of bytes in a row of pixels data including
    /// padding between lines.
    /// # Errors
    ///
    /// If the window in which the texture was created is closed, or the renderer fails
    /// to update to the texture, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { texture_id: TextureId };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     self.texture_id = s.create_texture(500, 600, None)?;
    ///     let image = Image::from_file("./some_image.png")?;
    ///     s.update_texture(self.texture_id, None, image.as_bytes(), image.pitch())?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn update_texture<R, P>(
        &mut self,
        texture_id: TextureId,
        rect: R,
        pixels: P,
        pitch: usize,
    ) -> PixResult<()>
    where
        R: Into<Option<Rect<i32>>>,
        P: AsRef<[u8]>,
    {
        let rect = rect.into();
        let pixels = pixels.as_ref();
        self.renderer
            .update_texture(texture_id, rect, pixels, pitch)
    }

    /// Target a `Texture` for drawing operations.
    ///
    /// # Errors
    ///
    /// If the window in which the texture was created is closed, or the renderer fails
    /// to update to the texture, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { texture_id: TextureId };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     self.texture_id = s.create_texture(500, 600, None)?;
    ///     s.with_texture(self.texture_id, |s: &mut PixState| -> PixResult<()> {
    ///         s.background(Color::random());
    ///         s.text("Rendered texture!")?;
    ///         Ok(())
    ///     })?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn with_texture<F>(&mut self, id: TextureId, f: F) -> PixResult<()>
    where
        F: FnOnce(&mut PixState) -> PixResult<()>,
    {
        trace!("Targeting TextureId: {} for rendering", id);

        self.push();
        self.ui.push_cursor();
        self.set_cursor_pos(self.theme.spacing.frame_pad);

        self.renderer.set_texture_target(id);
        let result = f(self);
        self.renderer.clear_texture_target();

        self.ui.pop_cursor();
        self.pop();

        trace!("Restoring render target");

        result
    }

    /// Save a portion `src` of a texture to a [png] file. Passing `None` for `src` saves the
    /// entire texture.
    ///
    /// # Errors
    ///
    /// Returns an error for any of the following:
    ///     - The window in which the texture was created is closed.
    ///     - The renderer fails to read pixels from the texture.
    ///     - An [`io::Error`] occurs attempting to create the [png] file.
    ///     - A [`png::EncodingError`] occurs attempting to write image bytes.
    ///
    /// [`io::Error`]: std::io::Error
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::S = event.key {
    ///         let texture_id = s.create_texture(200, 200, None)?;
    ///         s.with_texture(texture_id, |s: &mut PixState| -> PixResult<()> {
    ///             s.background(Color::random());
    ///             s.text("Rendered texture!")?;
    ///             Ok(())
    ///         })?;
    ///         s.save_texture(texture_id, None, "test_image.png")?;
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    pub fn save_texture<P, R>(&mut self, id: TextureId, src: R, path: P) -> PixResult<()>
    where
        P: AsRef<Path>,
        R: Into<Option<Rect<i32>>>,
    {
        info!("Saving TextureId: {} to {}", id, path.as_ref().display());
        self.with_texture(id, |s: &mut PixState| -> PixResult<()> {
            s.save_canvas(src, path)
        })
    }
}

/// Trait for texture operations on the underlying `Renderer`.
pub(crate) trait TextureRenderer {
    /// Create a `Texture` to draw to.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the texture dimensions are invalid,
    /// then an error is returned.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> PixResult<TextureId>;

    /// Delete a `Texture`.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the texture has already been dropped,
    /// then an error is returned.
    ///
    fn delete_texture(&mut self, texture_id: TextureId) -> PixResult<()>;

    /// Update texture with pixel data.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the renderer fails to update to the
    /// texture, then an error is returned.
    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> PixResult<()>;

    /// Draw texture to the curent canvas.
    ///
    /// # Errors
    ///
    /// Returns an error for any of the following:
    ///     - The current render target is closed or dropped.
    ///     - The texture being rendered has been dropped.
    ///     - The target texture is the same as the texture being rendered.
    ///     - The renderer fails to draw to the texture.
    ///
    #[allow(clippy::too_many_arguments)]
    fn texture(
        &mut self,
        texture_id: TextureId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> PixResult<()>;

    /// Returns texture used as the target for drawing operations, if set.
    fn texture_target(&self) -> Option<TextureId>;

    /// Set texture as the target for drawing operations.
    fn set_texture_target(&mut self, texture_id: TextureId);

    /// Clear texture as the target for drawing operations.
    fn clear_texture_target(&mut self);

    /// Clear internal texture cache.
    fn clear_texture_cache(&mut self);
}
