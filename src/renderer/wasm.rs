use crate::{
    prelude::*,
    renderer::{RendererSettings, Rendering},
    shape::{LineI2, QuadI2, TriI2},
};

mod audio;
mod texture;
mod window;

/// A Web-Assembly [Renderer] implementation.
pub(crate) struct Renderer {}

impl Rendering for Renderer {
    /// Creates a new Renderer instance.
    #[inline]
    fn new(settings: RendererSettings) -> PixResult<Self> {
        todo!()
    }

    /// Clears the current canvas to the given clear color
    #[inline]
    fn clear(&mut self) -> PixResult<()> {
        todo!()
    }

    /// Sets the color used by the renderer to draw the current canvas.
    #[inline]
    fn set_draw_color(&mut self, color: Color) -> PixResult<()> {
        todo!()
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    #[inline]
    fn clip(&mut self, rect: Option<Rect<i32>>) -> PixResult<()> {
        todo!()
    }

    /// Sets the blend mode used by the renderer to drawing.
    #[inline]
    fn blend_mode(&mut self, mode: BlendMode) {
        todo!()
    }

    /// Updates the canvas from the current back buffer.
    #[inline]
    fn present(&mut self) {
        todo!()
    }

    /// Scale the current canvas.
    #[inline]
    fn scale(&mut self, x: f32, y: f32) -> PixResult<()> {
        todo!()
    }

    /// Set the font size for drawing text to the current canvas.
    #[inline]
    fn font_size(&mut self, size: u32) -> PixResult<()> {
        todo!()
    }

    /// Set the font style for drawing text to the current canvas.
    #[inline]
    fn font_style(&mut self, style: FontStyle) {
        todo!()
    }

    /// Set the font family for drawing text to the current canvas.
    #[inline]
    fn font_family(&mut self, font: &Font) -> PixResult<()> {
        todo!()
    }

    /// Get clipboard text from the system clipboard.
    #[inline]
    fn clipboard_text(&self) -> String {
        todo!()
    }

    /// Set clipboard text to the system clipboard.
    #[inline]
    fn set_clipboard_text(&self, value: &str) -> PixResult<()> {
        todo!()
    }

    /// Open a URL in the default system browser.
    #[inline]
    fn open_url(&self, url: &str) -> PixResult<()> {
        todo!()
    }

    /// Draw text to the current canvas. `angle` must be in degrees.
    #[allow(clippy::too_many_arguments)]
    #[inline]
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
    ) -> PixResult<(u32, u32)> {
        todo!()
    }

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    #[inline]
    fn size_of(&self, text: &str, wrap_width: Option<u32>) -> PixResult<(u32, u32)> {
        todo!()
    }

    /// Draw a pixel to the current canvas.
    #[inline]
    fn point(&mut self, p: PointI2, color: Color) -> PixResult<()> {
        todo!()
    }

    /// Draw a line to the current canvas.
    #[inline]
    fn line(&mut self, line: LineI2, width: u8, color: Color) -> PixResult<()> {
        todo!()
    }

    /// Draw a triangle to the current canvas.
    #[inline]
    fn triangle(
        &mut self,
        tri: TriI2,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        todo!()
    }

    /// Draw a rectangle to the current canvas.
    #[inline]
    fn rect(
        &mut self,
        rect: Rect<i32>,
        radius: Option<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        todo!()
    }

    /// Draw a quadrilateral to the current canvas.
    #[inline]
    fn quad(&mut self, quad: QuadI2, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()> {
        todo!()
    }

    /// Draw a polygon to the current canvas.
    #[inline]
    fn polygon<I>(&mut self, ps: I, fill: Option<Color>, stroke: Option<Color>) -> PixResult<()>
    where
        I: Iterator<Item = PointI2>,
    {
        todo!()
    }

    /// Draw a ellipse to the current canvas.
    #[inline]
    fn ellipse(
        &mut self,
        ellipse: Ellipse<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        todo!()
    }

    /// Draw an arc to the current canvas.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn arc(
        &mut self,
        p: PointI2,
        radius: i32,
        start: i32,
        end: i32,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> PixResult<()> {
        todo!()
    }

    /// Draw an image to the current canvas, optionally rotated about a `center`, flipped or
    /// tinted. `angle` must be in degrees.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn image(
        &mut self,
        img: &Image,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
        angle: Scalar,
        center: Option<PointI2>,
        flipped: Option<Flipped>,
        tint: Option<Color>,
    ) -> PixResult<()> {
        todo!()
    }

    /// Return the current rendered target pixels as an array of bytes.
    #[inline]
    fn to_bytes(&mut self) -> PixResult<Vec<u8>> {
        todo!()
    }
}

impl std::fmt::Debug for Renderer {
    #[doc(hidden)]
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("WasmRenderer {{}}")
    }
}
