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
    typography::{self, Font, TextAlign, TextStyle},
};

/// Contains all style and transform settings for the engine state.
///
/// When `State::push()` is called, all settings here are pushed onto a stack to be later restored
/// with `State::pop()`.
pub(crate) struct StateSetting {
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
    text_align: TextAlign,
    text_leading: u32,
    text_size: u32,
    text_style: TextStyle,
    text_font: Font,

    /// Rendering
    blend_mode: BlendMode,
    blend_factor: f32,

    /// Transformation
    transform: Transform,
}

impl StateSetting {
    pub(crate) fn new() -> Self {
        Self {
            show_frame_rate: true,

            color_mode: ColorMode::default(),
            bg_color: Color::default(),
            fill: None,
            stroke: None,

            arc_mode: ArcMode::default(),
            ellipse_mode: EllipseMode::default(),
            rect_mode: RectMode::default(),
            stroke_weight: shape::DEFAULT_STROKE_WEIGHT,
            stroke_cap: StrokeCap::default(),
            stroke_join: StrokeJoin::default(),

            angle_mode: AngleMode::default(),

            image_tint: None,
            image_mode: ImageMode::default(),

            text_align: TextAlign::default(),
            text_leading: typography::DEFAULT_TEXT_LEADING,
            text_size: typography::DEFAULT_TEXT_SIZE,
            text_style: TextStyle::default(),
            text_font: Font::default(),

            blend_mode: BlendMode::default(),
            blend_factor: DEFAULT_BLEND_FACTOR,

            transform: Transform::new(),
        }
    }
}

impl State {
    /// Whether to display the frame rate in the title bar.
    pub fn show_frame_rate(&self) -> bool {
        self.settings.show_frame_rate
    }
    /// Set whether to display the frame rate in the title bar.
    pub fn set_show_frame_rate(&mut self, val: bool) {
        self.settings.show_frame_rate = val;
    }

    /// Get current color mode.
    pub fn color_mode(&self) -> ColorMode {
        self.settings.color_mode
    }
    /// Set current color mode.
    pub fn set_color_mode(&mut self, mode: ColorMode) {
        self.settings.color_mode = mode;
        unimplemented!("only RGB color mode is currently supported.");
    }

    /// Get the current color used to clear the canvas.
    pub fn bg_color(&self) -> Color {
        self.settings.bg_color
    }
    /// Set the color used to clear the canvas.
    pub fn set_bg_color<C: Into<Color>>(&mut self, color: C) {
        self.settings.bg_color = color.into();
    }

    /// Get the current color used to fill shapes.
    pub fn fill(&self) -> Option<Color> {
        self.settings.fill
    }
    /// Set the color used to fill shapes. Pass None to disable filling.
    pub fn set_fill<C: Into<Option<Color>>>(&mut self, color: C) {
        self.settings.fill = color.into();
    }

    /// Get the current color used to outline shapes.
    pub fn stroke(&self) -> Option<Color> {
        self.settings.stroke
    }
    /// Set the color used to outline shapes. Pass None to disable outlines.
    pub fn set_stroke<C: Into<Option<Color>>>(&mut self, color: C) {
        self.settings.stroke = color.into();
    }

    /// Gets the current arc mode for filling arc segments.
    pub fn arc_mode(&self) -> ArcMode {
        self.settings.arc_mode
    }
    /// Sets the current arc mode for filling arc segments.
    pub fn set_arc_mode(&mut self, mode: ArcMode) {
        self.settings.arc_mode = mode;
    }

    /// Gets the current ellipse mode for drawing ellipses and circles.
    pub fn ellipse_mode(&self) -> EllipseMode {
        self.settings.ellipse_mode
    }
    /// Sets the current ellipse mode for drawing ellipses and circles.
    pub fn set_ellipse_mode(&mut self, mode: EllipseMode) {
        self.settings.ellipse_mode = mode;
    }

    /// Gets the current rect mode for drawing rects and squares.
    pub fn rect_mode(&self) -> RectMode {
        self.settings.rect_mode
    }
    /// Sets the current rect mode for drawing rects and squares.
    pub fn set_rect_mode(&mut self, mode: RectMode) {
        self.settings.rect_mode = mode;
    }

    /// Gets the current stroke weight for drawing lines.
    pub fn stroke_weight(&self) -> u32 {
        self.settings.stroke_weight
    }
    /// Sets the current stroke weight for drawing lines.
    pub fn set_stroke_weight(&mut self, weight: u32) {
        self.settings.stroke_weight = weight;
    }

    /// Gets the current stroke cap for drawing lines.
    pub fn stroke_cap(&self) -> StrokeCap {
        self.settings.stroke_cap
    }
    /// Sets the current stroke cap for drawing lines.
    pub fn set_stroke_cap(&mut self, mode: StrokeCap) {
        self.settings.stroke_cap = mode;
    }

    /// Gets the current stroke join for drawing adjoining lines.
    pub fn stroke_join(&self) -> StrokeJoin {
        self.settings.stroke_join
    }
    /// Sets the current stroke join for drawing adjoining lines.
    pub fn set_stroke_join(&mut self, mode: StrokeJoin) {
        self.settings.stroke_join = mode;
    }

    /// Gets the current angle mode used for interpreting angle parameters.
    pub fn angle_mode(&self) -> AngleMode {
        self.settings.angle_mode
    }
    /// Sets the current angle mode used for interpreting angle parameters.
    pub fn set_angle_mode(&mut self, mode: AngleMode) {
        self.settings.angle_mode = mode;
    }

    /// Gets the current image tint used for drawing images.
    pub fn image_tint(&self) -> Option<Color> {
        self.settings.image_tint
    }
    /// Sets the current image tint used for drawing images.
    pub fn set_image_tint<C: Into<Option<Color>>>(&mut self, color: C) {
        self.settings.image_tint = color.into();
    }

    /// Gets the current image mode used for drawing images.
    pub fn image_mode(&self) -> ImageMode {
        self.settings.image_mode
    }
    /// Sets the current image mode used for drawing images.
    pub fn set_image_mode(&mut self, mode: ImageMode) {
        self.settings.image_mode = mode;
    }

    /// Gets the current text alignment for drawing text.
    pub fn text_align(&self) -> TextAlign {
        self.settings.text_align
    }
    /// Sets the current text alignment for drawing text.
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.settings.text_align = align;
    }

    /// Gets the current text leading for drawing text.
    pub fn text_leading(&self) -> u32 {
        self.settings.text_leading
    }
    /// Sets the current text leading for drawing text.
    pub fn set_text_leading(&mut self, leading: u32) {
        self.settings.text_leading = leading;
    }

    /// Gets the current text size for drawing text.
    pub fn text_size(&self) -> u32 {
        self.settings.text_size
    }
    /// Sets the current text size for drawing text.
    pub fn set_text_size(&mut self, size: u32) {
        self.settings.text_size = size;
    }

    /// Gets the current text style for drawing text.
    pub fn text_style(&self) -> TextStyle {
        self.settings.text_style
    }
    /// Sets the current text style for drawing text.
    pub fn set_text_style(&mut self, style: TextStyle) {
        self.settings.text_style = style;
    }

    /// Gets the current text font for drawing text.
    pub fn text_font(&self) -> Font {
        self.settings.text_font
    }
    /// Sets the current text font for drawing text.
    pub fn set_text_font(&mut self, font: Font) {
        self.settings.text_font = font;
    }

    /// Gets the current blend mode for rendering.
    pub fn blend_mode(&self) -> BlendMode {
        self.settings.blend_mode
    }
    /// Sets the current blend mode for rendering.
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.settings.blend_mode = mode;
        self.renderer.set_blend_mode(mode);
    }
}

impl Default for StateSetting {
    fn default() -> Self {
        Self::new()
    }
}
