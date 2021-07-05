use crate::{
    audio::Audio,
    prelude::*,
    renderer::{RendererSettings, Rendering, Result},
    window::{Result as WindowResult, Window},
};
use num_traits::AsPrimitive;

/// A Web-Assembly [Renderer] implementation.
pub(crate) struct Renderer {}

impl Audio for Renderer {
    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]) {
        let _ = samples;
        todo!("enqueue_audio")
    }
}

impl Window for Renderer {
    /// Get the primary window id.
    fn window_id(&self) -> WindowId {
        todo!("window_id")
    }

    /// Set whether the cursor is shown or not.
    fn cursor(&mut self, show: bool) {
        let _ = show;
        todo!("show_cursor")
    }

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event> {
        todo!("poll_event")
    }

    /// Get the current window title.
    fn title(&self) -> &str {
        todo!("title")
    }

    /// Set the current window title.
    fn set_title<S>(&mut self, title: S) -> WindowResult<()>
    where
        S: AsRef<str>,
    {
        let _ = title.as_ref();
        todo!("set_title")
    }

    /// Width of the window.
    fn window_width(&self) -> WindowResult<u32> {
        todo!("window_width")
    }

    /// Height of the window.
    fn window_height(&self) -> WindowResult<u32> {
        todo!("window_height")
    }

    /// Resize the window.
    fn resize<T>(&mut self, width: T, height: T) -> WindowResult<()>
    where
        T: AsPrimitive<u32>,
    {
        let _ = width.as_();
        let _ = height.as_();
        todo!("resize")
    }

    /// Returns whether the application is fullscreen or not.
    fn fullscreen(&self) -> bool {
        todo!("fullscreen")
    }

    /// Set the application to fullscreen or not.
    fn set_fullscreen(&mut self, val: bool) {
        let _ = val;
        todo!("set_fullscreen")
    }
}

impl Rendering for Renderer {
    /// Creates a new Renderer instance.
    fn new(s: &RendererSettings) -> Result<Self> {
        let _ = s;
        Ok(Self {})
    }

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self) {
        todo!("clear")
    }

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color<C>(&mut self, color: C)
    where
        C: Into<Color>,
    {
        let _ = color.into();
        todo!("set_draw_color")
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip<T, R>(&mut self, rect: R)
    where
        T: AsPrimitive<i32>,
        R: Into<Option<Rect<T>>>,
    {
        let _ = rect.into();
        todo!("set_clip_rect")
    }

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode) {
        let _ = mode;
        todo!("blend_mode")
    }

    /// Updates the canvas from the current back buffer.
    fn present(&mut self) {
        todo!("present")
    }

    /// Width of the current canvas.
    fn width(&self) -> u32 {
        todo!("width")
    }

    /// Height of the current canvas.
    fn height(&self) -> u32 {
        todo!("height")
    }

    /// Scale the current canvas.
    fn scale<T>(&mut self, x: T, y: T) -> Result<()>
    where
        T: AsPrimitive<f32>,
    {
        let _ = x.as_();
        let _ = y.as_();
        todo!("set_scale")
    }

    /// Create a texture to draw to.
    fn create_texture<F>(&mut self, width: u32, height: u32, format: F) -> Result<TextureId>
    where
        F: Into<Option<PixelFormat>>,
    {
        let _ = width;
        let _ = height;
        let _ = format.into();
        todo!("create_teture")
    }

    /// Delete a texture.
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
        let _ = texture_id;
        todo!("delete_texture");
    }

    /// Update texture with pixel data.
    fn update_texture<P>(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> Result<()>
    where
        P: AsRef<[u8]>,
    {
        let _ = texture_id;
        let _ = rect;
        let _ = pixels.as_ref();
        let _ = pitch;
        todo!("update_texture")
    }

    /// Draw texture canvas.
    fn texture(
        &mut self,
        texture_id: usize,
        src: Option<Rect<i32>>,
        dst: Option<Rect<i32>>,
    ) -> Result<()> {
        let _ = texture_id;
        let _ = src;
        let _ = dst;
        todo!("texture")
    }

    /// Set the font size for drawing to the current canvas.
    fn font_size(&mut self, size: u32) -> Result<()> {
        let _ = size;
        todo!("font_size")
    }

    /// Set the font style for drawing to the current canvas.
    fn font_style(&mut self, style: FontStyle) {
        let _ = style;
        todo!("font_style")
    }

    /// Set the font family for drawing to the current canvas.
    fn font_family<S>(&mut self, family: S) -> Result<()>
    where
        S: Into<String>,
    {
        let _ = family.into();
        todo!("font_family")
    }

    /// Draw text to the current canvas.
    fn text<P, T, C>(&mut self, position: P, text: T, fill: C, stroke: C) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        T: AsRef<str>,
        C: Into<Option<Color>>,
    {
        let _ = position.into();
        let _ = text.as_ref();
        if let Some(_) = fill.into() {}
        if let Some(_) = stroke.into() {}
        todo!("text")
    }

    /// Draw a pixel to the current canvas.
    fn point<P, C>(&mut self, p: P, color: C) -> Result<()>
    where
        P: Into<Point<Scalar>>,
        C: Into<Option<Color>>,
    {
        if let Some(_) = color.into() {
            let _ = p.into();
        }
        todo!("pixels")
    }

    /// Draw a line to the current canvas.
    fn line<L, C>(&mut self, line: L, color: C) -> Result<()>
    where
        L: Into<Line<Scalar>>,
        C: Into<Option<Color>>,
    {
        if let Some(_) = color.into() {
            let _ = line.into();
        }
        todo!("line")
    }

    /// Draw a triangle to the current canvas.
    fn triangle<T, C>(&mut self, tri: T, fill: C, stroke: C) -> Result<()>
    where
        T: Into<Triangle<Scalar>>,
        C: Into<Option<Color>>,
    {
        let _ = tri.into();
        if let Some(_) = fill.into() {}
        if let Some(_) = stroke.into() {}
        todo!("triangle")
    }

    /// Draw a rectangle to the current canvas.
    fn rect<R, C>(&mut self, rect: R, fill: C, stroke: C) -> Result<()>
    where
        R: Into<Rect<Scalar>>,
        C: Into<Option<Color>>,
    {
        let _ = rect.into();
        if let Some(_) = fill.into() {}
        if let Some(_) = stroke.into() {}
        todo!("rect")
    }

    /// Draw a polygon to the current canvas.
    fn polygon<C, V>(&mut self, vx: V, vy: V, fill: C, stroke: C) -> Result<()>
    where
        C: Into<Option<Color>>,
        V: AsRef<[Scalar]>,
    {
        let _: Vec<i16> = vx.as_ref().iter().map(|v| v.round() as i16).collect();
        let _: Vec<i16> = vy.as_ref().iter().map(|v| v.round() as i16).collect();
        if let Some(_) = fill.into() {}
        if let Some(_) = stroke.into() {}
        todo!("polygon")
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse<E, C>(&mut self, ellipse: E, fill: C, stroke: C) -> Result<()>
    where
        E: Into<Ellipse<Scalar>>,
        C: Into<Option<Color>>,
    {
        let _ = ellipse.into();
        if let Some(_) = fill.into() {}
        if let Some(_) = stroke.into() {}
        todo!("ellipse")
    }

    /// Draw an image to the current canvas.
    fn image<P>(&mut self, position: P, img: &Image) -> Result<()>
    where
        P: Into<Point<Scalar>>,
    {
        let _ = position.into();
        let _ = img.texture_id();
        todo!("image")
    }

    /// Draw a resized image to the current canvas.
    fn image_resized<R>(&mut self, dst_rect: R, img: &Image) -> Result<()>
    where
        R: Into<Rect<Scalar>>,
    {
        let _ = dst_rect.into();
        let _ = img.texture_id();
        todo!("image_resized")
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("WasmRenderer {{}}")
    }
}
