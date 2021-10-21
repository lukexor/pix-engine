//! Graphics renderer functions.

use crate::{core::state::Error as StateError, prelude::*};
use lazy_static::lazy_static;
use std::{borrow::Cow, error, ffi::NulError, fmt, io, path::PathBuf, result};

pub(crate) use crate::core::{
    texture::TextureRenderer, window::Error as WindowError, window::WindowRenderer,
};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod sdl;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use sdl::{Renderer, RendererTexture};

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::{Renderer, RendererTexture};

lazy_static! {
    /// Default directory to extract static library assets into.
    pub static ref DEFAULT_ASSET_DIR: PathBuf = PathBuf::from("/tmp/pix-engine");
}

/// The result type for `Renderer` operations.
pub type Result<T> = result::Result<T, Error>;

/// Default audio sample rate.
const DEFAULT_SAMPLE_RATE: i32 = 44_100; // in Hz

/// Settings used to set up the renderer.
#[derive(Debug, Clone)]
pub(crate) struct RendererSettings {
    pub(crate) title: String,
    pub(crate) theme: Theme,
    pub(crate) icon: Option<PathBuf>,
    pub(crate) asset_dir: PathBuf,
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
    pub(crate) borderless: bool,
    pub(crate) allow_highdpi: bool,
    pub(crate) hidden: bool,
    pub(crate) show_frame_rate: bool,
    pub(crate) target_frame_rate: Option<usize>,
    pub(crate) texture_cache_size: usize,
    pub(crate) text_cache_size: usize,
}

impl Default for RendererSettings {
    fn default() -> Self {
        Self {
            title: String::new(),
            theme: Theme::default(),
            icon: None,
            asset_dir: DEFAULT_ASSET_DIR.clone(),
            x: Position::default(),
            y: Position::default(),
            width: 640,
            height: 480,
            scale_x: 1.0,
            scale_y: 1.0,
            audio_sample_rate: DEFAULT_SAMPLE_RATE,
            fullscreen: false,
            vsync: false,
            resizable: false,
            borderless: false,
            allow_highdpi: false,
            hidden: false,
            show_frame_rate: false,
            target_frame_rate: None,
            texture_cache_size: 20,
            text_cache_size: 500,
        }
    }
}

/// Trait for operations on the underlying `Renderer`.
pub(crate) trait Rendering: Sized {
    /// Creates a new Renderer instance.
    fn new(settings: RendererSettings) -> Result<Self>;

    /// Clears the current canvas to the given clear color
    fn clear(&mut self) -> Result<()>;

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: Color) -> Result<()>;

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect<i32>>) -> Result<()>;

    /// Sets the blend mode used by the renderer to drawing.
    fn blend_mode(&mut self, mode: BlendMode);

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> Result<()>;

    /// Set the font size for drawing text to the current canvas.
    fn font_size(&mut self, size: u32) -> Result<()>;

    /// Set the font style for drawing text to the current canvas.
    fn font_style(&mut self, style: FontStyle);

    /// Set the font family for drawing text to the current canvas.
    fn font_family(&mut self, family: &str) -> Result<()>;

    /// Get clipboard text from the system clipboard.
    fn clipboard_text(&self) -> String;

    /// Set clipboard text to the system clipboard.
    fn set_clipboard_text(&self, value: &str) -> Result<()>;

    /// Draw text to the current canvas. `angle` must be in degrees.
    #[allow(clippy::too_many_arguments)]
    fn text(
        &mut self,
        position: PointI2,
        text: &str,
        wrap_width: Option<u32>,
        angle: Option<Scalar>,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        fill: Option<Color>,
    ) -> Result<()>;

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    fn size_of(&mut self, text: &str) -> Result<(u32, u32)>;

    /// Draw a pixel to the current canvas.
    fn point(&mut self, p: PointI2, color: Color) -> Result<()>;

    /// Draw a line to the current canvas.
    fn line(&mut self, line: LineI2, stroke: u8, color: Color) -> Result<()>;

    /// Draw a triangle to the current canvas.
    fn triangle(&mut self, tri: TriI2, fill: Option<Color>, stroke: Option<Color>) -> Result<()>;

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        rect: Rect<i32>,
        radius: Option<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw a quadrilateral to the current canvas.
    fn quad(&mut self, quad: QuadI2, fill: Option<Color>, stroke: Option<Color>) -> Result<()>;

    /// Draw a polygon to the current canvas.
    fn polygon(&mut self, ps: &[PointI2], fill: Option<Color>, stroke: Option<Color>)
        -> Result<()>;

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: Ellipse<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw an arc to the current canvas.
    #[allow(clippy::too_many_arguments)]
    fn arc(
        &mut self,
        p: PointI2,
        radius: i32,
        start: i32,
        end: i32,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw an image to the current canvas, optionally rotated about a `center`, flipped or
    /// tinted. `angle` must be in degrees.
    #[allow(clippy::too_many_arguments)]
    fn image(
        &mut self,
        img: &Image,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> Result<()>;
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
    WindowError(WindowError),
    /// Invalid text.
    InvalidText(&'static str, NulError),
    /// Invalid font.
    #[cfg(not(target_arch = "wasm32"))]
    InvalidFont(PathBuf),
    /// Invalid Texture.
    InvalidTexture(TextureId),
    /// An error from invalid type conversions.
    Conversion(Cow<'static, str>),
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
            Conversion(err) => write!(f, "conversion error: {}", err),
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
            WindowError(err) => err.source(),
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

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(err.into())
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::Conversion(err.to_string().into())
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Self::InvalidText("unknown nul error", err)
    }
}

impl From<PixError> for Error {
    fn from(err: PixError) -> Self {
        use PixError::*;
        match err {
            RendererError(err) => err,
            WindowError(err) => Error::WindowError(err),
            Conversion(err) => Error::Conversion(err),
            IoError(err) => Error::IoError(err),
            StateError(err) => Error::Other(err.to_string().into()),
            ImageError(err) => Error::Other(err.to_string().into()),
            Other(err) => Error::Other(err),
        }
    }
}
