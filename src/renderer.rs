//! Graphics renderer functions.

use crate::{
    prelude::*,
    shape::{LineI2, PointI2, QuadI2, TriI2},
};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

pub(crate) use crate::{texture::TextureRenderer, window::WindowRenderer};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod sdl;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use sdl::Renderer;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::Renderer;

/// Default audio sample rate.
const DEFAULT_SAMPLE_RATE: i32 = 44_100; // in Hz

/// Settings used to set up the renderer.
#[derive(Debug, Clone)]
pub(crate) struct RendererSettings {
    pub(crate) title: String,
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
            #[cfg(not(target_arch = "wasm32"))]
            icon: None,
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
            texture_cache_size: 256,
            text_cache_size: 512,
        }
    }
}

/// Trait for operations on the underlying `Renderer`.
pub(crate) trait Rendering: Sized {
    /// Creates a new Renderer instance.
    fn new(settings: RendererSettings) -> PixResult<Self>;

    /// Clears the current canvas to the given clear color
    fn clear(&mut self) -> PixResult<()>;

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: Color) -> PixResult<()>;

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect<i32>>) -> PixResult<()>;

    /// Sets the blend mode used by the renderer to drawing.
    fn blend_mode(&mut self, mode: BlendMode);

    /// Updates the canvas from the current back buffer.
    fn present(&mut self);

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> PixResult<()>;

    /// Set the font size for drawing text to the current canvas.
    fn font_size(&mut self, size: u32) -> PixResult<()>;

    /// Set the font style for drawing text to the current canvas.
    fn font_style(&mut self, style: FontStyle);

    /// Set the font family for drawing text to the current canvas.
    fn font_family(&mut self, font: &Font) -> PixResult<()>;

    /// Get clipboard text from the system clipboard.
    fn clipboard_text(&self) -> String;

    /// Set clipboard text to the system clipboard.
    fn set_clipboard_text(&self, value: &str) -> PixResult<()>;

    /// Open a URL in the default system browser.
    fn open_url(&self, url: &str) -> PixResult<()>;

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
        outline: u8,
    ) -> PixResult<(u32, u32)>;

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    fn size_of(&self, text: &str, wrap_width: Option<u32>) -> PixResult<(u32, u32)>;

    /// Draw a pixel to the current canvas.
    fn point(&mut self, p: PointI2, color: Color) -> PixResult<()>;

    /// Draw a line to the current canvas.
    fn line(&mut self, line: LineI2, width: u8, color: Color) -> PixResult<()>;

    /// Draw a triangle to the current canvas.
    fn triangle(&mut self, tri: TriI2, fill: Option<Color>, stroke: Option<Color>)
        -> PixResult<()>;

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        rect: Rect<i32>,
        radius: Option<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()>;

    /// Draw a quadrilateral to the current canvas.
    fn quad(&mut self, quad: QuadI2, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()>;

    /// Draw a polygon to the current canvas.
    fn polygon<I>(&mut self, ps: I, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()>
    where
        I: Iterator<Item = PointI2>;

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: Ellipse<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()>;

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
    ) -> PixResult<()>;

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
    ) -> PixResult<()>;

    /// Return the current rendered target pixels as an array of bytes.
    fn to_bytes(&mut self) -> PixResult<Vec<u8>>;
}
