//! Drawing functions.
//!
//! Provides a [Draw] trait as well standard draw methods.
//!
//! Provided [PixState] methods:
//!
//! - [PixState::clear]: Clear the render target to the current background [Color].
//! - [PixState::save_canvas]: Save the current render target out to a [png] file.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App;
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.background(ALICE_BLUE);
//!     s.clear();
//!     let rect = rect![0, 0, 100, 100];
//!     s.fill(RED);
//!     s.stroke(BLACK);
//!     s.rect(rect)?;
//!     Ok(())
//! }
//! # }
//! ```

use anyhow::Context;

use crate::{prelude::*, renderer::Rendering};
use std::{fs::File, io::BufWriter, path::Path};

/// Trait for objects that can be drawn to the screen.
pub trait Draw {
    /// Draw object to the current [PixState] canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let rect = rect![0, 0, 100, 100];
    ///     // The following two lines are equivalent.
    ///     s.rect(rect)?;
    ///     rect.draw(s)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn draw(&self, s: &mut PixState) -> PixResult<()>;
}

impl PixState {
    /// Clears the render target to the current background [Color] set by [PixState::background].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.background(CADET_BLUE);
    ///     s.clear();
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn clear(&mut self) -> PixResult<()> {
        self.renderer.set_draw_color(self.settings.background)?;
        self.renderer.clear()
    }

    /// Save currently rendered target to a [png] file.
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
    ///             s.background(s.accent_color())?;
    ///             s.text("Hello!")?;
    ///             s.save_canvas("test_image.png")?;
    ///             Ok(())
    ///         });
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    pub fn save_canvas<P>(&mut self, path: P) -> PixResult<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let png_file = BufWriter::new(File::create(&path)?);
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
