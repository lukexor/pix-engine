use super::{
    renderer::Renderer,
    rendering::{BlendMode, DEFAULT_BLEND_FACTOR},
    State,
};
use crate::{
    color::{Color, ColorMode},
    image::ImageMode,
    math::AngleMode,
    shape::{self, ArcMode, EllipseMode, RectMode, StrokeCap, StrokeJoin},
    transform::Transform,
    typography::{Font, FontFamily, TextAlign, TextStyle},
};

/// Contains all style and transform settings for the engine state.
///
/// When `State::push()` is called, all settings here are pushed onto a stack to be later restored
/// with `State::pop()`.
#[derive(Default, Clone)]
pub(crate) struct Setting {
    show_frame_rate: bool,

    /// Colors
    color_mode: ColorMode,
    bg_color: Color,
    fill: Option<Color>,
    stroke: Option<Color>,

    /// Shapes
    arc_mode: ArcMode,
    ellipse_mode: EllipseMode,
    rect_mode: RectMode,
    stroke_weight: u32,
    stroke_cap: StrokeCap,
    stroke_join: StrokeJoin,

    /// Math
    angle_mode: AngleMode,

    /// Images
    image_tint: Option<Color>,
    image_mode: ImageMode,

    /// Typography
    font: Font,

    /// Rendering
    blend_mode: BlendMode,
    blend_factor: f32,

    /// Transformation
    transform: Transform,
}

impl Setting {
    pub(crate) fn new() -> Self {
        Self {
            show_frame_rate: true,
            stroke_weight: shape::DEFAULT_STROKE_WEIGHT,
            blend_factor: DEFAULT_BLEND_FACTOR,
            ..Default::default()
        }
    }
}

impl State {
    /// Whether to display the frame rate in the title bar.
    pub fn show_frame_rate(&self) -> bool {
        self.get_window()
            .map(|w| w.settings.show_frame_rate)
            .unwrap_or_default()
    }
    /// Set whether to display the frame rate in the title bar.
    pub fn set_show_frame_rate(&mut self, val: bool) {
        if let Some(window) = self.get_window_mut() {
            window.settings.show_frame_rate = val;
        }
    }

    /// Get current color mode.
    pub fn color_mode(&self) -> ColorMode {
        self.get_window()
            .map(|w| w.settings.color_mode)
            .unwrap_or_default()
    }
    /// Set current color mode.
    pub fn set_color_mode(&mut self, mode: ColorMode) {
        if let Some(window) = self.get_window_mut() {
            window.settings.color_mode = mode;
        }
        unimplemented!("only RGB color mode is currently supported.");
    }

    /// Get the current color used to clear the canvas.
    pub fn bg_color(&self) -> Color {
        self.get_window()
            .map(|w| w.settings.bg_color)
            .unwrap_or_default()
    }
    /// Set the color used to clear the canvas.
    pub fn set_bg_color<C: Into<Color>>(&mut self, color: C) {
        if let Some(window) = self.get_window_mut() {
            window.settings.bg_color = color.into();
        }
    }

    /// Get the current color used to fill shapes.
    pub fn fill(&self) -> Option<Color> {
        self.get_window()
            .map(|w| w.settings.fill)
            .unwrap_or_default()
    }
    /// Set the color used to fill shapes. Pass None to disable filling.
    pub fn set_fill<C: Into<Option<Color>>>(&mut self, color: C) {
        if let Some(window) = self.get_window_mut() {
            window.settings.fill = color.into();
        }
    }
    /// Disable filling shapes.
    /// Shortcut for `State::set_fill(None)`.
    pub fn no_fill(&mut self) {
        self.set_fill(None);
    }

    /// Get the current color used to outline shapes.
    pub fn stroke(&self) -> Option<Color> {
        self.get_window()
            .map(|w| w.settings.stroke)
            .unwrap_or_default()
    }
    /// Set the color used to outline shapes. Pass None to disable outlines.
    pub fn set_stroke<C: Into<Option<Color>>>(&mut self, color: C) {
        if let Some(window) = self.get_window_mut() {
            window.settings.stroke = color.into();
        }
    }
    /// Disable outlining shapes.
    /// Shortcut for `State::set_stroke(None)`.
    pub fn no_stroke(&mut self) {
        self.set_stroke(None);
    }

    /// Gets the current arc mode for filling arc segments.
    pub fn arc_mode(&self) -> ArcMode {
        self.get_window()
            .map(|w| w.settings.arc_mode)
            .unwrap_or_default()
    }
    /// Sets the current arc mode for filling arc segments.
    pub fn set_arc_mode(&mut self, mode: ArcMode) {
        if let Some(window) = self.get_window_mut() {
            window.settings.arc_mode = mode;
        }
    }

    /// Gets the current ellipse mode for drawing ellipses and circles.
    pub fn ellipse_mode(&self) -> EllipseMode {
        self.get_window()
            .map(|w| w.settings.ellipse_mode)
            .unwrap_or_default()
    }
    /// Sets the current ellipse mode for drawing ellipses and circles.
    pub fn set_ellipse_mode(&mut self, mode: EllipseMode) {
        if let Some(window) = self.get_window_mut() {
            window.settings.ellipse_mode = mode;
        }
    }

    /// Gets the current rect mode for drawing rects and squares.
    pub fn rect_mode(&self) -> RectMode {
        self.get_window()
            .map(|w| w.settings.rect_mode)
            .unwrap_or_default()
    }
    /// Sets the current rect mode for drawing rects and squares.
    pub fn set_rect_mode(&mut self, mode: RectMode) {
        if let Some(window) = self.get_window_mut() {
            window.settings.rect_mode = mode;
        }
    }

    /// Gets the current stroke weight for drawing lines.
    pub fn stroke_weight(&self) -> u32 {
        self.get_window()
            .map(|w| w.settings.stroke_weight)
            .unwrap_or(shape::DEFAULT_STROKE_WEIGHT)
    }
    /// Sets the current stroke weight for drawing lines.
    pub fn set_stroke_weight(&mut self, weight: u32) {
        if let Some(window) = self.get_window_mut() {
            window.settings.stroke_weight = weight;
        }
    }

    /// Gets the current stroke cap for drawing lines.
    pub fn stroke_cap(&self) -> StrokeCap {
        self.get_window()
            .map(|w| w.settings.stroke_cap)
            .unwrap_or_default()
    }
    /// Sets the current stroke cap for drawing lines.
    pub fn set_stroke_cap(&mut self, mode: StrokeCap) {
        if let Some(window) = self.get_window_mut() {
            window.settings.stroke_cap = mode;
        }
    }

    /// Gets the current stroke join for drawing adjoining lines.
    pub fn stroke_join(&self) -> StrokeJoin {
        self.get_window()
            .map(|w| w.settings.stroke_join)
            .unwrap_or_default()
    }
    /// Sets the current stroke join for drawing adjoining lines.
    pub fn set_stroke_join(&mut self, mode: StrokeJoin) {
        if let Some(window) = self.get_window_mut() {
            window.settings.stroke_join = mode;
        }
    }

    /// Gets the current angle mode used for interpreting angle parameters.
    pub fn angle_mode(&self) -> AngleMode {
        self.get_window()
            .map(|w| w.settings.angle_mode)
            .unwrap_or_default()
    }
    /// Sets the current angle mode used for interpreting angle parameters.
    pub fn set_angle_mode(&mut self, mode: AngleMode) {
        if let Some(window) = self.get_window_mut() {
            window.settings.angle_mode = mode;
        }
    }

    /// Gets the current image tint used for drawing images.
    pub fn image_tint(&self) -> Option<Color> {
        self.get_window()
            .map(|w| w.settings.image_tint)
            .unwrap_or_default()
    }
    /// Sets the current image tint used for drawing images.
    pub fn set_image_tint<C: Into<Option<Color>>>(&mut self, color: C) {
        if let Some(window) = self.get_window_mut() {
            window.settings.image_tint = color.into();
        }
    }
    /// Disable image tint
    /// Shortcut for `State::set_image_tint(None)`
    pub fn no_image_tint(&mut self) {
        self.set_image_tint(None)
    }

    /// Gets the current image mode used for drawing images.
    pub fn image_mode(&self) -> ImageMode {
        self.get_window()
            .map(|w| w.settings.image_mode)
            .unwrap_or_default()
    }
    /// Sets the current image mode used for drawing images.
    pub fn set_image_mode(&mut self, mode: ImageMode) {
        if let Some(window) = self.get_window_mut() {
            window.settings.image_mode = mode;
        }
    }

    /// Gets the current text alignment for drawing text.
    pub fn text_align(&self) -> TextAlign {
        self.get_window()
            .map(|w| w.settings.font.align)
            .unwrap_or_default()
    }
    /// Sets the current text alignment for drawing text.
    pub fn set_text_align(&mut self, align: TextAlign) {
        if let Some(window) = self.get_window_mut() {
            window.settings.font.align = align;
        }
    }

    /// Gets the current font leading for drawing text.
    pub fn font_leading(&self) -> u32 {
        self.get_window()
            .map(|w| w.settings.font.leading)
            .unwrap_or_default()
    }
    /// Sets the current font leading for drawing text.
    pub fn set_font_leading(&mut self, leading: u32) {
        if let Some(window) = self.get_window_mut() {
            window.settings.font.leading = leading;
        }
    }

    /// Gets the current font size for drawing text.
    pub fn font_size(&self) -> u32 {
        self.get_window()
            .map(|w| w.settings.font.size)
            .unwrap_or_default()
    }
    /// Sets the current font size for drawing text.
    pub fn set_font_size(&mut self, size: u32) {
        if let Some(window) = self.get_window_mut() {
            window.settings.font.size = size;
        }
    }

    /// Gets the current font style for drawing text.
    pub fn font_style(&self) -> TextStyle {
        self.get_window()
            .map(|w| w.settings.font.style)
            .unwrap_or_default()
    }
    /// Sets the current font style for drawing text.
    pub fn set_font_style(&mut self, style: TextStyle) {
        if let Some(window) = self.get_window_mut() {
            window.settings.font.style = style;
        }
    }

    /// Gets the current text font for drawing text.
    pub fn font_family(&self) -> FontFamily {
        self.get_window()
            .map(|w| w.settings.font.family.clone())
            .unwrap_or_default()
    }
    /// Sets the current text font for drawing text.
    pub fn set_font_family(&mut self, family: FontFamily) {
        if let Some(window) = self.get_window_mut() {
            window.settings.font.family = family;
        }
    }

    /// Gets the current blend mode for rendering.
    pub fn blend_mode(&self) -> BlendMode {
        self.get_window()
            .map(|w| w.settings.blend_mode)
            .unwrap_or_default()
    }
    /// Sets the current blend mode for rendering.
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        if let Some(window) = self.get_window_mut() {
            window.settings.blend_mode = mode;
        }
        self.renderer.set_blend_mode(mode);
    }
}
