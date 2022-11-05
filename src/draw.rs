//! Drawing methods.
//!
//! Provides a [Draw] trait as well standard draw methods.
//!
//! Provided [`PixState`] methods:
//!
//! - [`PixState::clear`]: Clear the render target to the current background [Color].
//! - [`PixState::save_canvas`]: Save the current render target out to a [png] file.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App;
//! # impl PixEngine for App {
//! fn on_update(&mut self, s: &mut PixState) -> Result<()> {
//!     s.background(Color::ALICE_BLUE);
//!     s.clear();
//!     let rect = rect![0, 0, 100, 100];
//!     s.fill(Color::RED);
//!     s.stroke(Color::BLACK);
//!     s.rect(rect)?;
//!     Ok(())
//! }
//! # }
//! ```

use anyhow::Context;

use crate::{prelude::*, renderer::Rendering};
use log::info;
use std::{fs::File, io::BufWriter, path::Path};

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw object to the current [`PixState`] canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     let rect = rect![0, 0, 100, 100];
    ///     // The following two lines are equivalent.
    ///     s.rect(rect)?;
    ///     rect.draw(s)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn draw(&self, s: &mut PixState) -> Result<()>;
}

impl PixState {
    /// Clears the render target to the current background [Color] set by [`PixState::background`].
    ///
    /// # Errors
    ///
    /// If the current render target is closed or dropped, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> Result<()> {
    ///     s.background(Color::CADET_BLUE);
    ///     s.clear();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn clear(&mut self) -> Result<()> {
        self.renderer.set_draw_color(self.settings.background)?;
        self.renderer.clear()
    }

    /// Save a portion `src` of the currently rendered target to a [png] file. Passing `None` for
    /// `src` saves the entire target.
    ///
    /// # Errors
    ///
    /// Returns an error for any of the following:
    ///     - The current render target is closed or dropped.
    ///     - The renderer fails to read pixels from the current window target.
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
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> Result<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> Result<bool> {
    ///     if let Key::S = event.key {
    ///         s.save_canvas(None, "test_image.png")?;
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    pub fn save_canvas<P, R>(&mut self, src: R, path: P) -> Result<()>
    where
        P: AsRef<Path>,
        R: Into<Option<Rect<i32>>>,
    {
        info!("Saving canvas to {}", path.as_ref().display());
        if let Some(src) = src.into() {
            // Copy current texture target to a texture
            let bytes = self.renderer.to_bytes()?;
            let render_texture = self.create_texture(self.width()?, self.height()?, None)?;
            self.update_texture(render_texture, None, bytes, self.width()? as usize * 4)?;
            // Render the `src` rect from texture onto another texture, and save it
            let src_texture = self.create_texture(src.width() as u32, src.height() as u32, None)?;
            self.with_texture(src_texture, |s: &mut PixState| -> Result<()> {
                s.texture(render_texture, src, None)?;
                s.save_canvas(None, path)
            })?;
            self.delete_texture(render_texture)?;
            self.delete_texture(src_texture)?;
            Ok(())
        } else {
            let path = path.as_ref();
            let png_file = BufWriter::new(File::create(path)?);
            let mut png = png::Encoder::new(png_file, self.width()?, self.height()?);
            png.set_color(PixelFormat::Rgba.into());
            png.set_depth(png::BitDepth::Eight);
            let mut writer = png
                .write_header()
                .with_context(|| format!("failed to write png header: {:?}", path))?;

            writer
                .write_image_data(&self.renderer.to_bytes()?)
                .with_context(|| format!("failed to write png data: {:?}", path))
        }
    }
}
