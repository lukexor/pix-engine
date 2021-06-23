//! Graphics renderer functions.

use crate::{prelude::*, state::Error as StateError};
use std::{borrow::Cow, error, ffi::NulError, fmt, io, path::PathBuf, result};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod sdl;
use num_traits::ToPrimitive;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use sdl::Renderer;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::Renderer;

/// The result type for `Renderer` operations.
pub type Result<T> = result::Result<T, Error>;

/// Settings used to set up the renderer.
#[derive(Debug, Clone)]
pub(crate) struct RendererSettings {
    pub(crate) title: String,
    pub(crate) icon: Option<PathBuf>,
    pub(crate) x: Position,
    pub(crate) y: Position,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) scale_x: f32,
    pub(crate) scale_y: f32,
    pub(crate) audio_sample_rate: i32,
    pub(crate) fullscreen: bool,
    pub(crate) vsync: bool,
    pub(crate) resizable: bool,
    pub(crate) show_frame_rate: bool,
}

impl Default for RendererSettings {
    fn default() -> Self {
        Self {
            title: String::new(),
            icon: None,
            x: Position::default(),
            y: Position::default(),
            width: 400,
            height: 400,
            scale_x: 1.0,
            scale_y: 1.0,
            audio_sample_rate: 44_100,
            fullscreen: false,
            vsync: false,
            resizable: false,
            show_frame_rate: false,
        }
    }
}

/// Trait for operations on the underlying `Renderer`.
pub(crate) trait Rendering: Sized {
    /// Creates a new Renderer instance.
    fn new(settings: RendererSettings) -> Result<Self>;

    /// Width of the current canvas.
    fn width(&self) -> u32;

    /// Height of the current canvas.
    fn height(&self) -> u32;

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self);

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: impl Into<Color>);

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: impl Into<Option<Rect<f64>>>);

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode);

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> Result<()>;

    /// Create a texture to draw to.
    fn create_texture<T, F>(&mut self, width: T, height: T, format: F) -> Result<TextureId>
    where
        T: Into<f64>,
        F: Into<Option<PixelFormat>>;

    /// Delete a texture.
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()>;

    /// Update texture with pixel data.
    fn update_texture<R>(
        &mut self,
        texture_id: TextureId,
        rect: R,
        pixels: &[u8],
        pitch: usize,
    ) -> Result<()>
    where
        R: Into<Option<Rect<f64>>>;

    /// Draw texture canvas.
    fn texture<R>(&mut self, texture_id: TextureId, src: R, dst: R) -> Result<()>
    where
        R: Into<Option<Rect<f64>>>;

    /// Draw text to the current canvas.
    fn text<P, T, C>(&mut self, position: P, text: T, size: u32, fill: C, stroke: C) -> Result<()>
    where
        P: Into<Point<f64>>,
        T: AsRef<str>,
        C: Into<Option<Color>>;

    /// Draw a pixel to the current canvas.
    fn point<P, C>(&mut self, p: P, color: C) -> Result<()>
    where
        P: Into<Point<f64>>,
        C: Into<Option<Color>>;

    /// Draw a line to the current canvas.
    fn line<L, C>(&mut self, line: L, color: C) -> Result<()>
    where
        L: Into<Line<f64>>,
        C: Into<Option<Color>>;

    /// Draw a triangle to the current canvas.
    fn triangle<T, U, C>(&mut self, tri: T, fill: C, stroke: C) -> Result<()>
    where
        T: Into<Triangle<U>>,
        U: ToPrimitive,
        C: Into<Option<Color>>;

    /// Draw a rectangle to the current canvas.
    fn rect<R, C>(&mut self, rect: R, fill: C, stroke: C) -> Result<()>
    where
        R: Into<Rect<f64>>,
        C: Into<Option<Color>>;

    /// Draw a polygon to the current canvas.
    fn polygon<C>(&mut self, vx: &[f64], vy: &[f64], fill: C, stroke: C) -> Result<()>
    where
        C: Into<Option<Color>>;

    /// Draw a ellipse to the current canvas.
    fn ellipse<E, C>(&mut self, ellipse: E, fill: C, stroke: C) -> Result<()>
    where
        E: Into<Ellipse<f64>>,
        C: Into<Option<Color>>;

    /// Draw an image to the current canvas.
    fn image(&mut self, x: i32, y: i32, img: &Image) -> Result<()>;

    /// Draw a resized image to the current canvas.
    fn image_resized(&mut self, x: i32, y: i32, w: u32, h: u32, img: &Image) -> Result<()>;
}

/// The error type for `Renderer` operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Renderer initialization errors.
    InitError,
    /// Renderer I/O errors.
    IoError(io::Error),
    /// Invalid text.
    InvalidText(&'static str, NulError),
    /// Invalid Texture.
    InvalidTexture(TextureId),
    /// An overflow occurred
    Overflow(Cow<'static, str>, u32),
    /// Any other unknown error as a string.
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InitError => write!(f, "renderer initialization error"),
            IoError(err) => err.fmt(f),
            InvalidText(msg, err) => write!(f, "invalid text: {}, {}", msg, err),
            InvalidTexture(id) => write!(f, "invalid texture_id: {}", id),
            Overflow(err, val) => write!(f, "{}: {}", err, val),
            Other(err) => write!(f, "unknown renderer error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => err.source(),
            _ => None,
        }
    }
}

impl From<Error> for PixError {
    fn from(err: Error) -> Self {
        Self::RendererError(err)
    }
}

impl From<Error> for StateError {
    fn from(err: Error) -> Self {
        Self::RendererError(err)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::Other(Cow::from(err.to_string()))
    }
}
