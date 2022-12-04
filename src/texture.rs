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
//! - [`PixState::set_texture_target`]: Target a texture for rendering.
//! - [`PixState::clear_texture_target`]: Clear texture target back to primary canvas for rendering.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App;
//! # impl PixEngine for App {
//! fn on_update(&mut self, s: &mut PixState) -> Result<()> {
//!     let texture_id1 = s.create_texture(500, 600, PixelFormat::Rgb)?;
//!     // Does not actually render to the current canvas
//!     s.set_texture_target(texture_id1)?;
//!     s.background(Color::random());
//!     s.text("Rendered texture!")?;
//!     s.clear_texture_target();
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

use crate::{error::Result, prelude::*};
use std::{
    fmt,
    ops::{Deref, DerefMut},
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
    /// another window will result in a [`Error::InvalidTexture`]. This restriction may be
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
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.set_texture_target(texture_id1)?;
    ///     s.background(Color::random());
    ///     s.text("Rendered texture!")?;
    ///     s.clear_texture_target();
    ///
    ///     let src = rect![10, 10, 100, 100]; // Render a sub-section of the texture
    ///     // translate and scale texture
    ///     let dst = rect![200, 200, 200, 200];
    ///     s.texture(self.texture_id, src, dst)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn texture<R1, R2>(&mut self, texture_id: TextureId, src: R1, dst: R2) -> Result<()>
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
    /// another window will result in a [`Error::InvalidTexture`]. This restriction may be
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
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.set_texture_target(texture_id1)?;
    ///     s.background(Color::random());
    ///     s.text("Rendered texture!")?;
    ///     s.clear_texture_target();
    ///
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
    pub fn texture_transformed<R1, R2, C, F>(
        &mut self,
        texture_id: TextureId,
        src: R1,
        dst: R2,
        mut angle: f64,
        center: C,
        flipped: F,
    ) -> Result<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
        C: Into<Option<Point<i32>>>,
        F: Into<Option<Flipped>>,
    {
        let s = &self.settings;
        if s.angle_mode == AngleMode::Radians {
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
    /// [`PixelFormat::default`]. The texture will be created and tied to the current window
    /// target. To create a texture for a window other than the primary window, call
    /// [`PixState::set_window`].
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
    /// this method will create a texture for the current `window_target`, which can only be
    /// changed using the [`PixState::set_window_target`] method. It is the responsibility of the
    /// caller to manage created textures and call [`PixState::delete_texture`] when a texture
    /// resource is no longer needed and to ensure that texture methods are not called for a given
    /// window after it has been closed, otherwise an error will be returned.
    ///
    /// This constraint arises due to lifetime issues with SDL textures, See
    /// <https://github.com/Rust-SDL2/rust-sdl2/issues/1107> for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { texture_id: TextureId };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> Result<()> {
    ///     self.texture_id = s.create_texture(
    ///         s.width()? / 2,
    ///         s.height()? / 2,
    ///         PixelFormat::Rgb,
    ///     )?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn create_texture<F>(&mut self, width: u32, height: u32, format: F) -> Result<TextureId>
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
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     // A more polished implementation would manage state in `self` to avoid re-creating and
    ///     // destroying textures every frame
    ///     let texture_id = s.create_texture(500, 600, PixelFormat::Rgb)?;
    ///     // Render things
    ///     s.delete_texture(texture_id)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
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
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> Result<()> {
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
    ) -> Result<()>
    where
        R: Into<Option<Rect<i32>>>,
        P: AsRef<[u8]>,
    {
        let rect = rect.into();
        let pixels = pixels.as_ref();
        self.renderer
            .update_texture(texture_id, rect, pixels, pitch)
    }

    /// Set a `Texture` as the priamry target for drawing operations. Pushes current settings and UI
    /// cursor to the stack, so any changes made while a texture target is set will be in effect
    /// until [`PixState::reset_texture_target`] is called.
    ///
    /// # Errors
    ///
    /// If the target has been dropped or is invalid, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { texture_id: TextureId };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> Result<()> {
    ///     self.texture_id = s.create_texture(500, 600, None)?;
    ///     s.set_texture_target(self.texture_id)?;
    ///     s.background(Color::random());
    ///     s.text("Rendered texture!")?;
    ///     s.clear_texture_target();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn set_texture_target(&mut self, id: TextureId) -> Result<()> {
        if self.renderer.texture_target().is_none() {
            self.push();
            self.ui.push_cursor();
            self.set_cursor_pos(self.theme.spacing.frame_pad);
            self.renderer.set_texture_target(id)
        } else {
            Ok(())
        }
    }

    /// Clears `Texture` target back to the primary canvas for drawing operations. Pops previous
    /// settings and UI cursor off the stack, so that changes made while texture target was set are
    /// reverted.
    pub fn clear_texture_target(&mut self) {
        if self.renderer.texture_target().is_some() {
            self.renderer.clear_texture_target();
            self.ui.pop_cursor();
            self.pop();
        }
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
    ) -> Result<TextureId>;

    /// Delete a `Texture`.
    ///
    /// # Errors
    ///
    /// If the current window target is closed or invalid, or the texture has already been dropped,
    /// then an error is returned.
    ///
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()>;

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
    ) -> Result<()>;

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
        angle: f64,
        center: Option<Point<i32>>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> Result<()>;

    /// Returns texture used as the target for drawing operations, if set.
    fn texture_target(&self) -> Option<TextureId>;

    /// Set a `Texture` as the primary target for drawing operations instead of the window target
    /// canvas.
    ///
    /// # Errors
    ///
    /// If the texture has been dropped or is invalid, then an error is returned.
    fn set_texture_target(&mut self, texture_id: TextureId) -> Result<()>;

    /// Clear `Texture` target back to the window target canvas for drawing operations.
    fn clear_texture_target(&mut self);

    /// Returns whether a texture is set as the target for drawing operations.
    fn has_texture_target(&self) -> bool;

    /// Clear internal texture cache.
    fn clear_texture_cache(&mut self);
}
