use super::{
    renderer::Renderer,
    rendering::{BlendMode, DEFAULT_BLEND_FACTOR},
    StateData, StateDataResult,
};
use crate::{
    color::{self, Color, ColorMaxes, ColorMode},
    image::ImageMode,
    math::AngleMode,
    shape::{self, ArcMode, EllipseMode, RectMode, StrokeCap, StrokeJoin},
    transform::Transform,
    typography::{Font, FontFamily, TextAlign, TextStyle},
};

/// Contains all style and transform settings for the engine state.
///
/// When `StateData::push()` is called, all settings here are pushed onto a stack to be later restored
/// with `StateData::pop()`.
#[derive(Default, Clone)]
pub(crate) struct Setting {
    show_frame_rate: bool,

    /// Colors
    color_mode: ColorMode,
    color_maxes: ColorMaxes,
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
            bg_color: color::transparent(),
            stroke_weight: shape::DEFAULT_STROKE_WEIGHT,
            blend_factor: DEFAULT_BLEND_FACTOR,
            ..Default::default()
        }
    }
}

impl StateData {
    /// Set title for the current window target.
    ///
    /// Errors if the title contains a nul byte.
    pub fn title(&mut self, title: &str) -> StateDataResult<()> {
        Ok(self.renderer.title(title)?)
    }

    /// Sets the audio sample rate for the audio playback in Hz.
    pub fn audio_sample_rate(&mut self, rate: i32) -> StateDataResult<()> {
        Ok(self.renderer.audio_sample_rate(rate)?)
    }

    /// Get whether to display the frame rate in the title bar.
    pub fn get_show_frame_rate(&mut self) -> bool {
        self.settings.show_frame_rate
    }

    /// Set whether to display the frame rate in the title bar.
    pub fn show_frame_rate(&mut self, val: bool) {
        self.settings.show_frame_rate = val;
    }

    /// Get current color mode.
    pub fn get_color_mode(&self) -> ColorMode {
        self.settings.color_mode
    }
    /// Set current color mode.
    pub fn color_mode(&mut self, mode: ColorMode) {
        self.settings.color_mode = mode;
    }

    /// Get color maxes for the current mode.
    pub fn get_color_maxes(&self) -> ColorMaxes {
        self.settings.color_maxes
    }
    /// Set color maxes for the current color mode.
    pub fn color_maxes(&mut self, max1: u16, max2: u16, max3: u8, max_alpha: u8) {
        match self.get_color_mode() {
            ColorMode::Rgb => {
                self.settings.color_maxes.rgb =
                    [max1 as f64, max2 as f64, max3 as f64, max_alpha as f64]
            }
            ColorMode::Hsb => {
                self.settings.color_maxes.hsb =
                    [max1 as f64, max2 as f64, max3 as f64, max_alpha as f64]
            }
            ColorMode::Hsl => {
                self.settings.color_maxes.hsl =
                    [max1 as f64, max2 as f64, max3 as f64, max_alpha as f64]
            }
        }
    }

    /// Get the current color used to clear the canvas.
    pub fn get_background(&self) -> Color {
        self.settings.bg_color
    }

    /// Set the color used for the background. The default is transparent. This is typically used
    /// in `StateData::on_update()` to clear the canvas at the start of each frame but it can also be
    /// used in `StateData::on_start()` to set the background for the first frame if using
    /// `StateData::no_loop()`. Or it can be used any time if the background needs to be set to a given
    /// `Color`. To return to a transparent background use `StateData::no_background()`
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// # let mut state = StateData::new("State", 100, 100).unwrap();
    ///
    /// state.background([128, 200, 0]);
    /// assert_eq!(state.get_background(), Color::from([128, 200, 0]));
    /// ```
    pub fn background<C: Into<Color>>(&mut self, color: C) {
        let c = color.into();
        self.settings.bg_color = c;
        self.renderer.background(c);
        self.renderer.clear();
    }

    /// Disables the background color by setting it to transparent.
    pub fn no_background(&mut self) {
        self.settings.bg_color = color::transparent();
    }

    /// Get the current color used to fill shapes.
    pub fn get_fill(&self) -> Option<Color> {
        self.settings.fill
    }

    /// Set the color used to fill shapes.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let mut state = StateData::new("State", 100, 100).unwrap();
    ///
    /// state.fill([128, 200, 0]);
    /// assert_eq!(state.get_fill(), Some(Color::from([128, 200, 0])));
    /// ```
    pub fn fill<C: Into<Color>>(&mut self, color: C) {
        let c = color.into();
        self.settings.fill = Some(c);
        self.renderer.fill(self.settings.fill);
    }

    /// Disable filling shapes.
    ///
    /// # Example
    ///
    /// ```
    /// use pix_engine::prelude::*;
    /// let mut state = StateData::new("State", 100, 100).unwrap();
    ///
    /// state.fill([128, 200, 0]);
    /// assert_eq!(state.get_fill(), Some(Color::from([128, 200, 0])));
    /// state.no_fill();
    /// assert_eq!(state.get_fill(), None);
    /// ```
    pub fn no_fill(&mut self) {
        self.settings.fill = None;
        self.renderer.fill(self.settings.fill);
    }

    /// Get the current color used to outline shapes.
    pub fn get_stroke(&self) -> Option<Color> {
        self.settings.stroke
    }
    /// Set the color used to outline shapes. Pass None to disable outlines.
    pub fn stroke<C: Into<Color>>(&mut self, color: C) {
        let c = color.into();
        self.settings.stroke = Some(c);
        self.renderer.stroke(self.settings.stroke);
    }
    /// Disable outlining shapes.
    /// Shortcut for `StateData::set_stroke(None)`.
    pub fn no_stroke(&mut self) {
        self.settings.stroke = None;
        self.renderer.stroke(self.settings.stroke);
    }

    /// Gets the current arc mode for filling arc segments.
    pub fn get_arc_mode(&self) -> ArcMode {
        self.settings.arc_mode
    }
    /// Sets the current arc mode for filling arc segments.
    pub fn arc_mode(&mut self, mode: ArcMode) {
        self.settings.arc_mode = mode;
    }

    /// Gets the current ellipse mode for drawing ellipses and circles.
    pub fn get_ellipse_mode(&self) -> EllipseMode {
        self.settings.ellipse_mode
    }
    /// Sets the current ellipse mode for drawing ellipses and circles.
    pub fn ellipse_mode(&mut self, mode: EllipseMode) {
        self.settings.ellipse_mode = mode;
    }

    /// Gets the current rect mode for drawing rects and squares.
    pub fn get_rect_mode(&self) -> RectMode {
        self.settings.rect_mode
    }
    /// Sets the current rect mode for drawing rects and squares.
    pub fn rect_mode(&mut self, mode: RectMode) {
        self.settings.rect_mode = mode;
    }

    /// Gets the current stroke weight for drawing lines.
    pub fn get_stroke_weight(&self) -> u32 {
        self.settings.stroke_weight
    }
    /// Sets the current stroke weight for drawing lines.
    pub fn stroke_weight(&mut self, weight: u32) {
        self.settings.stroke_weight = weight;
    }

    /// Gets the current stroke cap for drawing lines.
    pub fn get_stroke_cap(&self) -> StrokeCap {
        self.settings.stroke_cap
    }
    /// Sets the current stroke cap for drawing lines.
    pub fn stroke_cap(&mut self, mode: StrokeCap) {
        self.settings.stroke_cap = mode;
    }

    /// Gets the current stroke join for drawing adjoining lines.
    pub fn get_stroke_join(&self) -> StrokeJoin {
        self.settings.stroke_join
    }
    /// Sets the current stroke join for drawing adjoining lines.
    pub fn stroke_join(&mut self, mode: StrokeJoin) {
        self.settings.stroke_join = mode;
    }

    /// Gets the current angle mode used for interpreting angle parameters.
    pub fn get_angle_mode(&self) -> AngleMode {
        self.settings.angle_mode
    }
    /// Sets the current angle mode used for interpreting angle parameters.
    pub fn angle_mode(&mut self, mode: AngleMode) {
        self.settings.angle_mode = mode;
    }

    /// Gets the current image tint used for drawing images.
    pub fn get_image_tint(&self) -> Option<Color> {
        self.settings.image_tint
    }
    /// Sets the current image tint used for drawing images.
    pub fn image_tint<C: Into<Option<Color>>>(&mut self, color: C) {
        self.settings.image_tint = color.into();
    }
    /// Disable image tint
    /// Shortcut for `StateData::set_image_tint(None)`
    pub fn no_image_tint(&mut self) {
        self.image_tint(None)
    }

    /// Gets the current image mode used for drawing images.
    pub fn get_image_mode(&self) -> ImageMode {
        self.settings.image_mode
    }
    /// Sets the current image mode used for drawing images.
    pub fn image_mode(&mut self, mode: ImageMode) {
        self.settings.image_mode = mode;
    }

    /// Gets the current text alignment for drawing text.
    pub fn get_text_align(&self) -> TextAlign {
        self.settings.font.align
    }
    /// Sets the current text alignment for drawing text.
    pub fn text_align(&mut self, align: TextAlign) {
        self.settings.font.align = align;
    }

    /// Gets the current font leading for drawing text.
    pub fn get_font_leading(&self) -> u32 {
        self.settings.font.leading
    }
    /// Sets the current font leading for drawing text.
    pub fn font_leading(&mut self, leading: u32) {
        self.settings.font.leading = leading;
    }

    /// Gets the current font size for drawing text.
    pub fn get_font_size(&self) -> u32 {
        self.settings.font.size
    }
    /// Sets the current font size for drawing text.
    pub fn font_size(&mut self, size: u32) {
        self.settings.font.size = size;
    }

    /// Gets the current font style for drawing text.
    pub fn get_font_style(&self) -> TextStyle {
        self.settings.font.style
    }
    /// Sets the current font style for drawing text.
    pub fn font_style(&mut self, style: TextStyle) {
        self.settings.font.style = style;
    }

    /// Gets the current text font for drawing text.
    pub fn get_font_family(&self) -> FontFamily {
        self.settings.font.family.clone()
    }
    /// Sets the current text font for drawing text.
    pub fn font_family(&mut self, family: FontFamily) {
        self.settings.font.family = family;
    }

    /// Gets the current blend mode for rendering.
    pub fn get_blend_mode(&self) -> BlendMode {
        self.settings.blend_mode
    }
    /// Sets the current blend mode for rendering.
    pub fn blend_mode(&mut self, mode: BlendMode) {
        self.settings.blend_mode = mode;
        self.renderer.blend_mode(mode);
    }
}
