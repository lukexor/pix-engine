//! Font handling and text layout.
//!
//! `epaint` lays text out into a [`Galley`] and packs the glyphs it needed into an atlas texture.
//! The atlas is shared by every window and by the canvas, so this type owns one set of fonts for
//! the whole renderer and hands the painter a delta whenever new glyphs are packed.
//!
//! Layout mutates the galley cache, so [`Fonts`] sits behind a [`RefCell`]. The drawing API
//! measures text through `&self`, and the borrow never escapes a method.

use crate::{
    error::Result,
    gui::theme::{FontId, FontSrc},
    prelude::{Color, Font, FontStyle, Point},
    renderer::shapes,
};
use anyhow::{anyhow, Context};
use egui::epaint::{
    text::{FontDefinitions, Fonts, LayoutJob, TextFormat, TextOptions},
    Color32, FontFamily, Galley, ImageDelta, Shape, Stroke, TextShape,
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    sync::Arc,
};

/// Scale between a laid-out point and a pixel.
///
/// The canvas is the size of the window in pixels, so a point is a pixel and text comes out at
/// the size the drawing API asked for.
const PIXELS_PER_POINT: f32 = 1.0;

/// Horizontal offset a bold draws its second copy at, as a fraction of the point size.
///
/// `epaint` selects a bold face rather than thickening a regular one, and each engine font is a
/// single face. Drawing the galley twice, offset horizontally, widens every stem by that much.
/// This fraction puts the widening near the stem difference between a regular and a bold face,
/// and it applies to any font the caller supplies.
///
/// The offset is rounded to whole pixels. [`PIXELS_PER_POINT`] is 1, so a fractional offset puts
/// the second copy across a pixel boundary and the stem gains a grey fringe instead of a column.
/// That reads as blur on a pixel font such as [`Font::EMULOGIC`].
const BOLD_OFFSET: f32 = 0.045;

/// Smallest offset a bold draws its second copy at.
///
/// Under one pixel the second copy lands on the pixels of the first and nothing thickens.
const MIN_BOLD_OFFSET: f32 = 1.0;

/// Offsets an outline is drawn at, as fractions of the outline width.
///
/// `epaint` has no glyph outline, so the halo is eight copies of the text ringed around the
/// original. Eight rather than four, because a diagonal gap is visible at the corners of glyphs.
const OUTLINE_OFFSETS: [(f32, f32); 8] = [
    (-1.0, -1.0),
    (0.0, -1.0),
    (1.0, -1.0),
    (-1.0, 0.0),
    (1.0, 0.0),
    (-1.0, 1.0),
    (0.0, 1.0),
    (1.0, 1.0),
];

/// Fonts the renderer draws text with.
pub(crate) struct TextRenderer {
    /// Loaded faces, their atlas and the galley cache.
    fonts: RefCell<Fonts>,
    /// Face data and the families built from it, kept so a new font can be added.
    definitions: FontDefinitions,
    /// Family names each registered [`Font`] is laid out under, one per face it supplied.
    families: HashMap<FontId, Families>,
    /// Family text is currently laid out in.
    current: FontId,
    /// Families the current [`TextRenderer::fonts`] was built with.
    ///
    /// `epaint` panics on a family its [`Fonts`] does not hold, so a font named this frame is
    /// laid out in [`TextRenderer::default_family`] until the rebuild binds it.
    baked: HashSet<FontId>,
    /// Family every other one falls back to, registered before any frame runs.
    default_family: FontId,
    /// Point size text is currently laid out at.
    size: f32,
    /// Style applied to laid-out text.
    style: FontStyle,
    /// Layout options, fixed for the life of the renderer.
    options: TextOptions,
    /// Set when [`TextRenderer::definitions`] names a face the atlas does not hold yet.
    stale: bool,
}

impl TextRenderer {
    /// Loads the default font and prepares an atlas that fits the device.
    ///
    /// # Errors
    ///
    /// Returns an error if the default font cannot be read.
    pub(crate) fn new(max_texture_side: usize) -> Result<Self> {
        let options = TextOptions {
            max_texture_side,
            ..TextOptions::default()
        };
        let default = Font::default();
        let mut definitions = FontDefinitions::empty();
        let mut families = HashMap::new();
        let name = register(&mut definitions, &default)?;
        families.insert(default.id(), name);
        Ok(Self {
            fonts: RefCell::new(Fonts::new(options, definitions.clone())),
            definitions,
            baked: families.keys().copied().collect(),
            families,
            current: default.id(),
            default_family: default.id(),
            size: 14.0,
            style: FontStyle::NORMAL,
            options,
            stale: false,
        })
    }

    /// Lays subsequent text out in `font`, loading it the first time it is named.
    ///
    /// # Errors
    ///
    /// Returns an error if the font has no data source, or if its file cannot be read.
    pub(crate) fn set_family(&mut self, font: &Font) -> Result<()> {
        let id = font.id();
        if !self.families.contains_key(&id) {
            let name = register(&mut self.definitions, font)?;
            self.families.insert(id, name);
            // Faces are baked into `Fonts` when it is built, so adding one rebuilds it and repacks
            // the atlas. Text already laid out this frame points into the atlas as it stands, so
            // the rebuild waits for the frame boundary, and `font_id` falls back until then.
            self.stale = true;
        }
        self.current = id;
        Ok(())
    }

    /// Sets the point size subsequent text is laid out at.
    pub(crate) fn set_size(&mut self, size: u32) {
        #[allow(clippy::cast_precision_loss)]
        let size = size.max(1) as f32;
        self.size = size;
    }

    /// Sets the style applied to subsequent text.
    pub(crate) fn set_style(&mut self, style: FontStyle) {
        self.style = style;
    }

    /// Prepares the fonts for a new frame, loading any face named during the last one.
    pub(crate) fn begin_frame(&mut self) {
        if self.stale {
            self.stale = false;
            self.fonts = RefCell::new(Fonts::new(self.options, self.definitions.clone()));
            self.baked = self.families.keys().copied().collect();
            return;
        }
        // Repacks the atlas when it is filling up.
        self.fonts.borrow_mut().begin_pass(self.options);
    }

    /// Returns the glyphs packed since the last call, for upload to the painter.
    pub(crate) fn take_image_delta(&mut self) -> Option<ImageDelta> {
        self.fonts.borrow_mut().font_image_delta()
    }

    /// Returns the atlas size the tessellator maps glyph coordinates against.
    pub(crate) fn image_size(&self) -> [usize; 2] {
        self.fonts
            .borrow_mut()
            .with_pixels_per_point(PIXELS_PER_POINT)
            .font_image_size()
    }

    /// Returns the scale between a laid-out point and a pixel.
    pub(crate) const fn pixels_per_point(&self) -> f32 {
        PIXELS_PER_POINT
    }

    /// Lays `text` out in the current font, wrapping at `wrap_width` when given.
    pub(crate) fn layout(&self, text: &str, wrap_width: Option<u32>, color: Color) -> Arc<Galley> {
        let mut job = LayoutJob::single_section(
            text.to_owned(),
            TextFormat {
                font_id: self.font_id(),
                color: shapes::color32(color),
                italics: self.synthetic().contains(FontStyle::ITALIC),
                underline: self.decoration(FontStyle::UNDERLINE, color),
                strikethrough: self.decoration(FontStyle::STRIKETHROUGH, color),
                ..TextFormat::default()
            },
        );
        #[allow(clippy::cast_precision_loss)]
        {
            job.wrap.max_width = wrap_width.map_or(f32::INFINITY, |width| width as f32);
        }
        self.fonts
            .borrow_mut()
            .with_pixels_per_point(PIXELS_PER_POINT)
            .layout_job(job)
    }

    /// Returns the size `text` lays out to as `(width, height)`.
    ///
    /// Empty text still occupies a line, so a caller advancing a cursor by the returned height
    /// moves down one row.
    pub(crate) fn size_of(&self, text: &str, wrap_width: Option<u32>) -> (u32, u32) {
        if text.is_empty() {
            let height = self
                .fonts
                .borrow_mut()
                .with_pixels_per_point(PIXELS_PER_POINT)
                .row_height(&self.font_id());
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            return (0, height.ceil() as u32);
        }
        let galley = self.layout(text, wrap_width, Color::WHITE);
        let size = galley.rect.size();
        // Layout knows nothing of the second copy a bold draws, so the width it reports is short
        // by the offset between them.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        (
            (size.x + self.bold_offset()).ceil() as u32,
            size.y.ceil() as u32,
        )
    }

    /// Builds the shape that draws `galley` at `pos`.
    ///
    /// `angle` is clockwise radians about `pos`. An `outline` wider than zero rings the text in
    /// `stroke`, which is how stroked text is drawn.
    pub(crate) fn shape(
        &self,
        pos: Point<i32>,
        galley: &Arc<Galley>,
        angle: f32,
        fill: Color,
        outline: u16,
        stroke: Color,
    ) -> Shape {
        let origin = shapes::pos2(pos);
        let bold = self.bold_offset();
        let text = |at: egui::Pos2, color: Color32| {
            Shape::Text(TextShape {
                angle,
                override_text_color: Some(color),
                ..TextShape::new(at, Arc::clone(galley), color)
            })
        };
        // A bold is the same galley drawn again alongside itself, so every stem widens. The step
        // turns with `angle`, or it would run along the stems of rotated text instead of across
        // them. The halo below repeats the pair, or the outline would ring the regular weight.
        // Where the copies overlap a translucent fill composites twice and comes out denser.
        let step = egui::Vec2::angled(angle) * bold;
        let copies = |at: egui::Pos2, color: Color32, out: &mut Vec<Shape>| {
            out.push(text(at, color));
            if bold > 0.0 {
                out.push(text(at + step, color));
            }
        };

        if outline == 0 && bold == 0.0 {
            return text(origin, shapes::color32(fill));
        }

        let mut shapes = Vec::new();
        if outline > 0 {
            let width = f32::from(outline);
            let halo = shapes::color32(stroke);
            for (x, y) in OUTLINE_OFFSETS {
                copies(
                    origin + egui::Vec2::new(x * width, y * width),
                    halo,
                    &mut shapes,
                );
            }
        }
        copies(origin, shapes::color32(fill), &mut shapes);
        Shape::Vec(shapes)
    }

    /// Returns the offset a bold draws its second copy at, or zero when text is not bold.
    fn bold_offset(&self) -> f32 {
        if self.synthetic().contains(FontStyle::BOLD) {
            (self.size * BOLD_OFFSET).round().max(MIN_BOLD_OFFSET)
        } else {
            0.0
        }
    }

    /// Returns the font the current family, size and style resolve to.
    ///
    /// A family falls back to [`TextRenderer::default_family`] until a rebuild bakes it.
    fn font_id(&self) -> egui::FontId {
        let family = self
            .resolve()
            .map_or(FontFamily::Proportional, |(family, _)| {
                FontFamily::Name(Arc::clone(family))
            });
        egui::FontId::new(self.size, family)
    }

    /// Returns the family the current font and style draw from, with the bits it cannot cover.
    fn resolve(&self) -> Option<(&Arc<str>, FontStyle)> {
        let current = if self.baked.contains(&self.current) {
            self.current
        } else {
            self.default_family
        };
        Some(self.families.get(&current)?.resolve(self.style))
    }

    /// Returns the style bits no registered face covers, which the renderer synthesizes.
    fn synthetic(&self) -> FontStyle {
        self.resolve().map_or(self.style, |(_, style)| style)
    }

    /// Returns the stroke a text decoration is drawn with, or none when it is not set.
    fn decoration(&self, style: FontStyle, color: Color) -> Stroke {
        if self.style.contains(style) {
            Stroke::new(1.0, shapes::color32(color))
        } else {
            Stroke::NONE
        }
    }
}

/// Family names one [`Font`] is laid out under, one per face it supplied.
struct Families {
    /// Family holding the regular face.
    regular: Arc<str>,
    /// Family holding the bold face, where the font supplied one.
    bold: Option<Arc<str>>,
    /// Family holding the italic face, where the font supplied one.
    italic: Option<Arc<str>>,
    /// Family holding the bold italic face, where the font supplied one.
    bold_italic: Option<Arc<str>>,
}

impl Families {
    /// Returns the family `style` is drawn from, and the bits no face covers.
    ///
    /// The renderer synthesizes the returned style on top, by thickening or shearing. A font with
    /// every face returns [`FontStyle::NORMAL`], and nothing is synthesized.
    fn resolve(&self, style: FontStyle) -> (&Arc<str>, FontStyle) {
        let wants_bold = style.contains(FontStyle::BOLD);
        let wants_italic = style.contains(FontStyle::ITALIC);
        match (wants_bold, wants_italic) {
            (false, false) => (&self.regular, FontStyle::NORMAL),
            (true, false) => self
                .bold
                .as_ref()
                .map_or((&self.regular, FontStyle::BOLD), |f| (f, FontStyle::NORMAL)),
            (false, true) => self
                .italic
                .as_ref()
                .map_or((&self.regular, FontStyle::ITALIC), |f| {
                    (f, FontStyle::NORMAL)
                }),
            // Half a match beats none, so a font with only one of the two faces draws from it and
            // synthesizes the other.
            (true, true) => {
                if let Some(face) = &self.bold_italic {
                    (face, FontStyle::NORMAL)
                } else if let Some(face) = &self.bold {
                    (face, FontStyle::ITALIC)
                } else if let Some(face) = &self.italic {
                    (face, FontStyle::BOLD)
                } else {
                    (&self.regular, FontStyle::BOLD | FontStyle::ITALIC)
                }
            }
        }
    }
}

/// Adds a font's faces to `definitions`, returning the family name of each.
fn register(definitions: &mut FontDefinitions, font: &Font) -> Result<Families> {
    // `epaint` picks a face per family, not per style, so each face is registered as a family of
    // its own instead of as a fallback within one.
    let variant = |definitions: &mut FontDefinitions, suffix: &str, src: Option<&FontSrc>| {
        src.map(|src| insert(definitions, &format!("{}{suffix}", font.name()), src))
            .transpose()
    };
    let regular = insert(definitions, font.name(), font.source())?;
    Ok(Families {
        regular,
        bold: variant(definitions, "-Bold", font.variants.bold.as_ref())?,
        italic: variant(definitions, "-Italic", font.variants.italic.as_ref())?,
        bold_italic: variant(
            definitions,
            "-BoldItalic",
            font.variants.bold_italic.as_ref(),
        )?,
    })
}

/// Adds one face to `definitions` under `name`, returning the family name.
fn insert(definitions: &mut FontDefinitions, name: &str, src: &FontSrc) -> Result<Arc<str>> {
    let data = match src {
        FontSrc::None => return Err(anyhow!("font `{name}` has no data source")),
        FontSrc::Bytes(bytes) => egui::FontData::from_static(bytes),
        FontSrc::Path(path) => egui::FontData::from_owned(
            std::fs::read(path)
                .with_context(|| format!("failed to read font file {}", path.display()))?,
        ),
    };
    let family: Arc<str> = Arc::from(name);
    definitions
        .font_data
        .insert(name.to_owned(), Arc::new(data));
    definitions
        .families
        .insert(FontFamily::Name(Arc::clone(&family)), vec![name.to_owned()]);
    Ok(family)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a renderer with `font` registered and baked into the atlas.
    // Setup that cannot run leaves nothing to assert, so it fails the test where it breaks.
    #[allow(clippy::expect_used)]
    fn renderer(font: &Font) -> TextRenderer {
        let mut text = TextRenderer::new(2048).expect("default font loads");
        text.set_family(font).expect("font registers");
        text.begin_frame();
        text
    }

    /// Returns the family name text lays out under.
    fn family(text: &TextRenderer) -> String {
        match text.font_id().family {
            FontFamily::Name(name) => name.to_string(),
            other => panic!("expected a named family, got {other:?}"),
        }
    }

    #[test]
    fn a_font_with_faces_draws_each_style_from_its_own() {
        let mut text = renderer(&Font::NOTO);
        for (style, expected) in [
            (FontStyle::NORMAL, "Noto"),
            (FontStyle::BOLD, "Noto-Bold"),
            (FontStyle::ITALIC, "Noto-Italic"),
            (FontStyle::BOLD | FontStyle::ITALIC, "Noto-BoldItalic"),
        ] {
            text.set_style(style);
            assert_eq!(family(&text), expected, "{style:?}");
            assert_eq!(
                text.synthetic(),
                FontStyle::NORMAL,
                "{style:?} is drawn, not synthesized"
            );
        }
    }

    #[test]
    fn a_font_without_faces_synthesizes_every_style() {
        let mut text = renderer(&Font::EMULOGIC);
        for style in [
            FontStyle::BOLD,
            FontStyle::ITALIC,
            FontStyle::BOLD | FontStyle::ITALIC,
        ] {
            text.set_style(style);
            assert_eq!(family(&text), "Emulogic", "{style:?} keeps the one face");
            assert_eq!(text.synthetic(), style, "{style:?} is synthesized");
        }
    }

    #[test]
    fn only_a_synthesized_bold_widens_the_glyphs() {
        let mut noto = renderer(&Font::NOTO);
        noto.set_style(FontStyle::BOLD);
        assert_eq!(
            noto.bold_offset(),
            0.0,
            "a drawn bold face needs no widening"
        );

        let mut emulogic = renderer(&Font::EMULOGIC);
        emulogic.set_style(FontStyle::BOLD);
        assert!(emulogic.bold_offset() >= MIN_BOLD_OFFSET);
    }

    #[test]
    fn half_a_match_draws_from_the_face_it_has() {
        let font = Font::from_bytes("Half", Font::NOTO_TTF).bold(Font::NOTO_BOLD_TTF);
        let mut text = renderer(&font);
        text.set_style(FontStyle::BOLD | FontStyle::ITALIC);
        assert_eq!(family(&text), "Half-Bold");
        assert_eq!(
            text.synthetic(),
            FontStyle::ITALIC,
            "italic is sheared on top"
        );
    }
}
