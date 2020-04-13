use super::{
    rendering::{BlendMode, DEFAULT_BLEND_FACTOR},
    State,
};
use crate::{
    color::{Color, ColorMode},
    image::ImageMode,
    math::AngleMode,
    shape::{self, ArcMode, EllipseMode, RectMode, StrokeCap, StrokeJoin},
    transform::Transform,
    typography::{self, Font, TextAlignHori, TextAlignVert, TextStyle},
};

/// Contains all style and transform settings for the engine state
///
/// When `State::push()` is called, all settings here are pushed onto a stack to be later
/// restored with `State::pop()`
pub(crate) struct StateSetting {
    show_frame_rate: bool,

    /// Colors
    pub(super) color_mode: ColorMode,
    pub(super) bg_color: Color,
    pub(super) fill: Option<Color>,
    pub(super) stroke: Option<Color>,

    /// Shapes
    pub(super) arc_mode: ArcMode,
    pub(super) ellipse_mode: EllipseMode,
    pub(super) rect_mode: RectMode,
    pub(super) stroke_weight: u32,
    pub(super) stroke_cap: StrokeCap,
    pub(super) stroke_join: StrokeJoin,

    /// Math
    pub(super) angle_mode: AngleMode,

    /// Images
    pub(super) image_tint: Option<Color>,
    pub(super) image_mode: ImageMode,

    /// Typography
    pub(super) text_align_hori: TextAlignHori,
    pub(super) text_align_vert: TextAlignVert,
    pub(super) text_leading: u32,
    pub(super) text_size: u32,
    pub(super) text_style: TextStyle,
    pub(super) text_font: Font,

    /// Remndering
    pub(super) blend_mode: BlendMode,
    pub(super) blend_factor: f32,

    /// Transformation
    pub(super) transform: Transform,
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

            text_align_hori: TextAlignHori::default(),
            text_align_vert: TextAlignVert::default(),
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
    /// Whether to display the frame rate in the title bar
    pub fn show_frame_rate(&self) -> bool {
        self.settings.show_frame_rate
    }

    /// Set whether to display the frame rate in the title bar
    pub fn set_show_frame_rate(&mut self, val: bool) {
        self.settings.show_frame_rate = val;
    }
}

impl Default for StateSetting {
    fn default() -> Self {
        Self::new()
    }
}
