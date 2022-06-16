//! Graphics renderer functions.

use crate::{image::Icon, prelude::*};

pub(crate) use crate::{texture::TextureRenderer, window::WindowRenderer};

#[cfg(not(target_arch = "wasm32"))]
pub mod sdl;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use sdl::Renderer;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::Renderer;

/// Settings used to set up the renderer.
#[derive(Debug, Clone)]
pub(crate) struct RendererSettings {
    /// Base window title.
    pub(crate) title: String,
    /// Application icon.
    pub(crate) icon: Option<Icon>,
    /// Starting window X coordinate.
    pub(crate) x: Position,
    /// Starting window Y coordinate.
    pub(crate) y: Position,
    /// Starting window width.
    pub(crate) width: u32,
    /// Starting window height.
    pub(crate) height: u32,
    /// Rendering scale for x-coordinates.
    pub(crate) scale_x: f32,
    /// Rendering scale for y-coordinates.
    pub(crate) scale_y: f32,
    /// Audio queue sample rate. `None` uses device default.
    pub(crate) audio_sample_rate: Option<i32>,
    /// Audio queue channel count. 1 for mono, 2 for stereo, etc. `None` uses device default.
    pub(crate) audio_channels: Option<u8>,
    /// Audio queue buffer size. `None` uses devide default.
    pub(crate) audio_buffer_size: Option<u16>,
    /// Window fullscreen mode.
    pub(crate) fullscreen: bool,
    /// Sync [`PixEngine::on_update`] rate with monitor refresh rate.
    pub(crate) vsync: bool,
    /// Enable window resizing.
    pub(crate) resizable: bool,
    /// Disable window borders.
    pub(crate) borderless: bool,
    /// Enable high resolution mode, if supported.
    pub(crate) allow_highdpi: bool,
    /// Hide window.
    pub(crate) hidden: bool,
    /// Show frame rate per second in title bar.
    pub(crate) show_frame_rate: bool,
    /// Limit [`PixEngine::on_update`] to target frame frate per second.
    pub(crate) target_frame_rate: Option<usize>,
    /// Size of allowed texture cache before least-used entries are evicted.
    pub(crate) texture_cache_size: usize,
    /// Size of allowed font cache before least-used entries are evicted.
    pub(crate) text_cache_size: usize,
}

impl Default for RendererSettings {
    fn default() -> Self {
        Self {
            title: String::new(),
            icon: None,
            x: Position::default(),
            y: Position::default(),
            width: 640,
            height: 480,
            scale_x: 1.0,
            scale_y: 1.0,
            audio_sample_rate: None,
            audio_channels: None,
            audio_buffer_size: None,
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

    /// Set the rendering scale of the current canvas. Drawing coordinates are scaled by x/y
    /// factors before being drawn to the canvas.
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
        position: Point<i32>,
        text: &str,
        wrap_width: Option<u32>,
        angle: Option<f64>,
        center: Option<Point<i32>>,
        flipped: Option<Flipped>,
        fill: Option<Color>,
        outline: u16,
    ) -> PixResult<(u32, u32)>;

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    fn size_of(&self, text: &str, wrap_width: Option<u32>) -> PixResult<(u32, u32)>;

    /// Draw a pixel to the current canvas.
    fn point(&mut self, p: Point<i32>, color: Color) -> PixResult<()>;

    /// Draw a line to the current canvas.
    fn line(&mut self, line: Line<i32>, smooth: bool, width: u8, color: Color) -> PixResult<()>;

    /// Draw a cubic Bezier curve to the current canvas.
    fn bezier<I>(&mut self, ps: I, detail: i32, stroke: Option<Color>) -> PixResult<()>
    where
        I: Iterator<Item = Point<i32>>;

    /// Draw a triangle to the current canvas.
    fn triangle(
        &mut self,
        tri: Tri<i32>,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()>;

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        rect: Rect<i32>,
        radius: Option<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()>;

    /// Draw a quadrilateral to the current canvas.
    fn quad(
        &mut self,
        quad: Quad<i32>,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()>;

    /// Draw a polygon to the current canvas.
    fn polygon<I>(
        &mut self,
        ps: I,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()>
    where
        I: Iterator<Item = Point<i32>>;

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: Ellipse<i32>,
        smooth: bool,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()>;

    /// Draw an arc to the current canvas.
    #[allow(clippy::too_many_arguments)]
    fn arc(
        &mut self,
        p: Point<i32>,
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
        angle: f64,
        center: Option<Point<i32>>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> PixResult<()>;

    /// Return the current rendered target pixels as an array of bytes.
    fn to_bytes(&mut self) -> PixResult<Vec<u8>>;

    /// Connect a controller with the given joystick index to start receiving events.
    fn open_controller(&mut self, controller_id: ControllerId) -> PixResult<()>;

    /// Disconnect a controller with the given joystick index to stop receiving events.
    fn close_controller(&mut self, controller_id: ControllerId);
}
