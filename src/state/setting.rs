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
        let c = color.into();
        self.settings.bg_color = c;
        self.renderer.set_bg_color(c);
    }

    /// Get the current color used to fill shapes.
    pub fn fill(&self) -> Option<Color> {
        self.settings.fill
    }
    /// Set the color used to fill shapes. Pass None to disable filling.
    pub fn set_fill<C: Into<Option<Color>>>(&mut self, color: C) {
        let c = color.into();
        self.settings.fill = c;
        self.renderer.set_fill(c);
    }
    /// Disable filling shapes.
    /// Shortcut for `State::set_fill(None)`.
    pub fn no_fill(&mut self) {
        self.set_fill(None);
        self.renderer.set_fill(None);
    }

    /// Get the current color used to outline shapes.
    pub fn stroke(&self) -> Option<Color> {
        self.settings.stroke
    }
    /// Set the color used to outline shapes. Pass None to disable outlines.
    pub fn set_stroke<C: Into<Option<Color>>>(&mut self, color: C) {
        let c = color.into();
        self.settings.stroke = c;
        self.renderer.set_stroke(c);
    }
    /// Disable outlining shapes.
    /// Shortcut for `State::set_stroke(None)`.
    pub fn no_stroke(&mut self) {
        self.set_stroke(None);
        self.renderer.set_stroke(None);
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
    /// Disable image tint
    /// Shortcut for `State::set_image_tint(None)`
    pub fn no_image_tint(&mut self) {
        self.set_image_tint(None)
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
        self.settings.font.align
    }
    /// Sets the current text alignment for drawing text.
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.settings.font.align = align;
    }

    /// Gets the current font leading for drawing text.
    pub fn font_leading(&self) -> u32 {
        self.settings.font.leading
    }
    /// Sets the current font leading for drawing text.
    pub fn set_font_leading(&mut self, leading: u32) {
        self.settings.font.leading = leading;
    }

    /// Gets the current font size for drawing text.
    pub fn font_size(&self) -> u32 {
        self.settings.font.size
    }
    /// Sets the current font size for drawing text.
    pub fn set_font_size(&mut self, size: u32) {
        self.settings.font.size = size;
    }

    /// Gets the current font style for drawing text.
    pub fn font_style(&self) -> TextStyle {
        self.settings.font.style
    }
    /// Sets the current font style for drawing text.
    pub fn set_font_style(&mut self, style: TextStyle) {
        self.settings.font.style = style;
    }

    /// Gets the current text font for drawing text.
    pub fn font_family(&self) -> FontFamily {
        self.settings.font.family.clone()
    }
    /// Sets the current text font for drawing text.
    pub fn set_font_family(&mut self, family: FontFamily) {
        self.settings.font.family = family;
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
