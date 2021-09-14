use crate::{
    audio::Audio,
    core::window::{Result as WindowResult, Window},
    prelude::*,
    renderer::{RendererSettings, Rendering, Result},
};

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
    fn cursor(&mut self, cursor: Option<&Cursor>) -> WindowResult<()> {
        let _ = cursor;
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
    fn set_title(&mut self, title: &str) -> WindowResult<()> {
        let _ = title;
        todo!("set_title")
    }

    /// Set the current window title with FPS appended.
    #[inline(always)]
    fn set_fps_title(&mut self, fps: usize) -> WindowResult<()> {
        let _ = fps;
        todo!("set_fps_title")
    }

    /// Dimensions of the primary window as `(width, height)`.
    fn dimensions(&self, id: WindowId) -> WindowResult<(u32, u32)> {
        let _ = id;
        todo!("dimensions")
    }

    /// Set dimensions of the primary window as `(width, height)`.
    fn set_dimensions(&mut self, id: WindowId, dimensions: (u32, u32)) -> WindowResult<()> {
        let _ = id;
        let _ = dimensions;
        todo!("set_dimensions")
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

    /// Returns whether the window synchronizes frame rate to the screens refresh rate.
    fn vsync(&self) -> bool {
        todo!("vsync")
    }

    /// Set the window to synchronize frame rate to the screens refresh rate.
    fn set_vsync(&mut self, val: bool) -> WindowResult<()> {
        let _ = val;
        todo!("set_vsync")
    }
}

impl Rendering for Renderer {
    /// Creates a new Renderer instance.
    fn new(s: RendererSettings) -> Result<Self> {
        let _ = s;
        Ok(Self {})
    }

    /// Clears the current canvas to the given clear color.
    fn clear(&mut self) {
        todo!("clear")
    }

    /// Sets the color used by the renderer to draw the current canvas.
    fn set_draw_color(&mut self, color: Color) {
        let _ = color;
        todo!("set_draw_color")
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect<i32>>) {
        let _ = rect;
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

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> Result<()> {
        let _ = x;
        let _ = y;
        todo!("set_scale")
    }

    /// Create a texture to draw to.
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<PixelFormat>,
    ) -> Result<TextureId> {
        let _ = width;
        let _ = height;
        let _ = format;
        todo!("create_teture")
    }

    /// Delete a texture.
    fn delete_texture(&mut self, texture_id: TextureId) -> Result<()> {
        let _ = texture_id;
        todo!("delete_texture");
    }

    /// Update texture with pixel data.
    fn update_texture(
        &mut self,
        texture_id: TextureId,
        rect: Option<Rect<i32>>,
        pixels: &[u8],
        pitch: usize,
    ) -> Result<()> {
        let _ = texture_id;
        let _ = rect;
        let _ = pixels;
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
    fn font_family(&mut self, family: &str) -> Result<()> {
        let _ = family;
        todo!("font_family")
    }

    /// Draw text to the current canvas.
    fn text(
        &mut self,
        position: &Point<i32>,
        text: &str,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let _ = position;
        let _ = text;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("text")
    }

    /// Returns the rendered dimensions of the given text using the current font
    /// as `(width, height)`.
    fn size_of(&self, text: &str) -> Result<(u32, u32)> {
        let _ = text;
        todo!("size_of")
    }

    /// Draw a pixel to the current canvas.
    fn point(&mut self, p: &Point<i32>, color: Color) -> Result<()> {
        let _ = p;
        let _ = color;
        todo!("pixels")
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, line: &Line<i32>, color: Color) -> Result<()> {
        let _ = line;
        let _ = color;
        todo!("line")
    }

    /// Draw a triangle to the current canvas.
    fn triangle(
        &mut self,
        tri: &Triangle<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let _ = tri;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("triangle")
    }

    /// Draw a rectangle to the current canvas.
    fn rect(&mut self, rect: &Rect<i32>, fill: Option<Color>, stroke: Option<Color>) -> Result<()> {
        let _ = rect;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("rect")
    }

    /// Draw a rounded rectangle to the current canvas.
    fn rounded_rect(
        &mut self,
        rect: &Rect<i32>,
        radius: i32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let _ = rect;
        let _ = radius;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("rounded_rect")
    }

    /// Draw a quadrilateral to the current canvas.
    fn quad(&mut self, quad: &Quad<i32>, fill: Option<Color>, stroke: Option<Color>) -> Result<()> {
        let _ = quad;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("quad")
    }

    /// Draw a polygon to the current canvas.
    fn polygon<P>(&mut self, ps: P, fill: Option<Color>, stroke: Option<Color>) -> Result<()>
    where
        P: IntoIterator<Item = Point<i32>>,
    {
        let _ = ps;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("polygon")
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        ellipse: &Ellipse<i32>,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let _ = ellipse;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("ellipse")
    }

    /// Draw an arc to the current canvas.
    #[allow(clippy::too_many_arguments)]
    fn arc(
        &mut self,
        p: &Point<i32>,
        radius: i32,
        start: i32,
        end: i32,
        mode: ArcMode,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let _ = p;
        let _ = radius;
        let _ = start;
        let _ = end;
        let _ = mode;
        if let Some(_) = fill {}
        if let Some(_) = stroke {}
        todo!("arc")
    }

    /// Draw an image to the current canvas.
    fn image(&mut self, position: &Point<i32>, img: &Image, tint: Option<Color>) -> Result<()> {
        let _ = position;
        let _ = img.texture_cache();
        img.set_texture_id(0);
        let _ = tint;
        todo!("image")
    }

    /// Draw a resized image to the current canvas.
    fn image_resized(&mut self, img: &Image, dst: &Rect<i32>, tint: Option<Color>) -> Result<()> {
        let _ = dst;
        let _ = img.texture_cache();
        let _ = tint;
        todo!("image_resized")
    }

    /// Draw a rotated image to the current canvas.
    fn image_rotated(
        &mut self,
        pos: &Point<i32>,
        img: &Image,
        angle: f64,
        tint: Option<Color>,
    ) -> Result<()> {
        let _ = pos;
        let _ = img.texture_cache();
        let _ = angle;
        let _ = tint;
        todo!("image_rotated")
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("WasmRenderer {{}}")
    }
}
