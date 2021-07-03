//! Graphics renderer functions.

use crate::{prelude::*, state::Error as StateError, window, ASSET_DIR};
use num_traits::AsPrimitive;
use std::{borrow::Cow, error, ffi::NulError, fmt, io, path::PathBuf, result};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod sdl;
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
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) font: PathBuf,
    #[cfg(target_arch = "wasm32")]
    pub(crate) font: String,
    pub(crate) font_size: u16,
    #[cfg(not(target_arch = "wasm32"))]
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
            #[cfg(not(target_arch = "wasm32"))]
            font: PathBuf::from(ASSET_DIR).join("emulogic.ttf"),
            #[cfg(target_arch = "wasm32")]
            font: "Courier New",
            font_size: 16,
            #[cfg(not(target_arch = "wasm32"))]
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
    fn new(settings: &RendererSettings) -> Result<Self>;

    /// Width of the current canvas.
    fn width(&self) -> u32;

    /// Height of the current canvas.
    fn height(&self) -> u32;

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self);

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color<C>(&mut self, color: C)
    where
        C: Into<Color>;

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip<T, R>(&mut self, rect: R)
    where
        T: AsPrimitive<Scalar>,
        R: Into<Option<Rect<T>>>;

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode);

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Scale the current canvas.
    fn scale<T: AsPrimitive<f32>>(&mut self, x: T, y: T) -> Result<()>;

    /// Create a texture to draw to.
    fn create_texture<T, F>(&mut self, width: T, height: T, format: F) -> Result<TextureId>
    where
        T: Into<Scalar>,
        F: Into<Option<PixelFormat>>;

    /// Delete a texture.
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()>;

    /// Update texture with pixel data.
    fn update_texture<R, P>(
        &mut self,
        texture_id: TextureId,
        rect: R,
        pixels: P,
        pitch: usize,
    ) -> Result<()>
    where
        R: Into<Option<Rect<Scalar>>>,
        P: AsRef<[u8]>;

    /// Draw texture canvas.
    fn texture<R>(&mut self, texture_id: TextureId, src: R, dst: R) -> Result<()>
    where
        R: Into<Option<Rect<Scalar>>>;

    /// Set the font size for drawing to the current canvas.
    fn font_size(&mut self, size: u32) -> Result<()>;

    /// Set the font style for drawing to the current canvas.
    fn font_style(&mut self, style: FontStyle);

    /// Set the font family for drawing to the current canvas.
    fn font_family<S>(&mut self, family: S) -> Result<()>
    where
        S: Into<String>;

    /// Draw text to the current canvas.
    fn text<P, T, C>(&mut self, position: P, text: T, fill: C, stroke: C) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        T: AsRef<str>,
        C: Into<Option<Color>>;

    /// Draw a pixel to the current canvas.
    fn point<P, C>(&mut self, p: P, color: C) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        C: Into<Option<Color>>;

    /// Draw a line to the current canvas.
    fn line<L, C>(&mut self, line: L, color: C) -> Result<()>
    where
        L: Into<Line<Scalar>>,
        C: Into<Option<Color>>;

    /// Draw a triangle to the current canvas.
    fn triangle<T, C>(&mut self, tri: T, fill: C, stroke: C) -> Result<()>
    where
        T: Into<Triangle<Scalar>>,
        C: Into<Option<Color>>;

    /// Draw a rectangle to the current canvas.
    fn rect<R, C>(&mut self, rect: R, fill: C, stroke: C) -> Result<()>
    where
        R: Into<Rect<Scalar>>,
        C: Into<Option<Color>>;

    /// Draw a polygon to the current canvas.
    fn polygon<C, V>(&mut self, vx: V, vy: V, fill: C, stroke: C) -> Result<()>
    where
        C: Into<Option<Color>>,
        V: AsRef<[Scalar]>;

    /// Draw a ellipse to the current canvas.
    fn ellipse<E, C>(&mut self, ellipse: E, fill: C, stroke: C) -> Result<()>
    where
        E: Into<Ellipse<Scalar>>,
        C: Into<Option<Color>>;

    /// Draw an image to the current canvas.
    fn image<P>(&mut self, position: P, img: &Image) -> Result<()>
    where
        P: Into<Point<Scalar>>;

    /// Draw a resized image to the current canvas.
    fn image_resized<R>(&mut self, dst_rect: R, img: &Image) -> Result<()>
    where
        R: Into<Rect<Scalar>>;
}

/// The error type for `Renderer` operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Renderer initialization errors.
    InitError,
    /// Renderer I/O errors.
    IoError(io::Error),
    /// Window errors.
    WindowError(window::Error),
    /// Invalid text.
    InvalidText(&'static str, NulError),
    /// Invalid Texture.
    InvalidTexture(TextureId),
    /// An overflow occurred.
    Overflow(Cow<'static, str>, u32),
    /// Any other unknown error as a string.
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InitError => write!(f, "renderer initialization error"),
            InvalidText(msg, err) => write!(f, "invalid text: {}, {}", msg, err),
            InvalidTexture(id) => write!(f, "invalid texture_id: {}", id),
            Overflow(err, val) => write!(f, "overflow {}: {}", err, val),
            Other(err) => write!(f, "unknown renderer error: {}", err),
            _ => self.fmt(f),
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
