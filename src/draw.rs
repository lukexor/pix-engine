//! Drawing functions.
//!
//! Provides a [Draw] trait as well standard draw methods like [PixState::clear].
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

    /// Save currently rendered target to a `png` file.
    ///
    /// # Example
    ///
    /// ```
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
