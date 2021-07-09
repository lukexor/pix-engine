//! Graphics renderer functions.

use crate::{prelude::*, state::Error as StateError, window};
use std::{borrow::Cow, error, ffi::NulError, fmt, result};

#[cfg(not(target_arch = "wasm32"))]
use crate::ASSET_DIR;
#[cfg(not(target_arch = "wasm32"))]
use std::{io, path::PathBuf};
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
            font: "Courier New".to_string(),
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

    /// Clears the current canvas to the given clear color
    fn clear(&mut self);

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: Color);

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect<i32>>);

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode);

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> Result<()>;

    /// Create a texture to draw to.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> Result<TextureId>;

    /// Delete a texture.
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()>;

    /// Update texture with pixel data.
    fn update_texture(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: &[u8],
        pitch: usize,
    ) -> Result<()>;

    /// Draw texture canvas.
    fn texture(
        &mut self,
        texture_id: TextureId,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
    ) -> Result<()>;

    /// Set the font size for drawing to the current canvas.
    fn font_size(&mut self, size: u32) -> Result<()>;

    /// Set the font style for drawing to the current canvas.
    fn font_style(&mut self, style: FontStyle);

    /// Set the font family for drawing to the current canvas.
    fn font_family(&mut self, family: &str) -> Result<()>;

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        position: Point<i32>,
        text: &str,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    fn size_of(&self, text: &str) -> Result<(u32, u32)>;

    /// Draw a pixel to the current canvas.
    fn point(&mut self, p: Point<i16>, color: Color) -> Result<()>;

    /// Draw a line to the current canvas.
    fn line(&mut self, line: Line<i16>, color: Color) -> Result<()>;

    /// Draw a triangle to the current canvas.
    fn triangle(
        &mut self,
        tri: Triangle<i16>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw a rectangle to the current canvas.
    fn rect(&mut self, rect: Rect<i16>, fill: Option<Color>, stroke: Option<Color>) -> Result<()>;

    /// Draw a rounded rectangle to the current canvas.
    fn rounded_rect(
        &mut self,
        rect: Rect<i16>,
        radius: i16,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw a quadrilateral to the current canvas.
    fn quad(&mut self, quad: Quad<i16>, fill: Option<Color>, stroke: Option<Color>) -> Result<()>;

    /// Draw a polygon to the current canvas.
    fn polygon(
        &mut self,
        vx: &[i16],
        vy: &[i16],
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: Ellipse<i16>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw an arc to the current canvas.
    #[allow(clippy::too_many_arguments)]
    fn arc(
        &mut self,
        p: Point<i16>,
        radius: i16,
        start: i16,
        end: i16,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()>;

    /// Draw an image to the current canvas.
    fn image(&mut self, position: Point<i32>, img: &Image, tint: Option<Color>) -> Result<()>;

    /// Draw a resized image to the current canvas.
    fn image_resized(
        &mut self,
        dst_rect: Rect<i32>,
        img: &Image,
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
    #[cfg(not(target_arch = "wasm32"))]
    IoError(io::Error),
    /// Window errors.
    WindowError(window::Error),
    /// Invalid text.
    InvalidText(&'static str, NulError),
    /// Invalid font.
    #[cfg(not(target_arch = "wasm32"))]
    InvalidFont(PathBuf),
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
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            Error::IoError(err) => err.source(),
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
