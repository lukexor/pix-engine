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
use log::warn;
use std::{cell::RefCell, collections::HashMap, sync::Arc};

/// Scale between a laid-out point and a pixel.
///
/// The canvas is the size of the window in pixels, so a point is a pixel and text comes out at
/// the size the drawing API asked for.
const PIXELS_PER_POINT: f32 = 1.0;

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
    /// Family name each registered [`Font`] is laid out under.
    families: HashMap<FontId, Arc<str>>,
    /// Family text is currently laid out in.
    current: FontId,
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
            families,
            current: default.id(),
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
            // the rebuild waits for the frame boundary and this frame draws a fallback face.
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
    ///
    /// `epaint` picks bold from a bold face rather than thickening a regular one, and the engine
    /// ships one face per family, so [`FontStyle::BOLD`] is reported and dropped.
    pub(crate) fn set_style(&mut self, style: FontStyle) {
        if style.contains(FontStyle::BOLD) && !self.style.contains(FontStyle::BOLD) {
            warn!("bold text needs a bold font face; drawing regular weight instead");
        }
        self.style = style;
    }

    /// Prepares the fonts for a new frame, loading any face named during the last one.
    pub(crate) fn begin_frame(&mut self) {
        if self.stale {
            self.stale = false;
            self.fonts = RefCell::new(Fonts::new(self.options, self.definitions.clone()));
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
                italics: self.style.contains(FontStyle::ITALIC),
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
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        (size.x.ceil() as u32, size.y.ceil() as u32)
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
        let text = |at: egui::Pos2, color: Color32| {
            Shape::Text(TextShape {
                angle,
                override_text_color: Some(color),
                ..TextShape::new(at, Arc::clone(galley), color)
            })
        };
        if outline == 0 {
            return text(origin, shapes::color32(fill));
        }
        let width = f32::from(outline);
        let halo = shapes::color32(stroke);
        let mut shapes: Vec<Shape> = OUTLINE_OFFSETS
            .iter()
            .map(|(x, y)| text(origin + egui::Vec2::new(x * width, y * width), halo))
            .collect();
        shapes.push(text(origin, shapes::color32(fill)));
        Shape::Vec(shapes)
    }

    /// Returns the font the current family, size and style resolve to.
    fn font_id(&self) -> egui::FontId {
        let family = self.families.get(&self.current).map_or_else(
            || FontFamily::Proportional,
            |name| FontFamily::Name(Arc::clone(name)),
        );
        egui::FontId::new(self.size, family)
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

/// Adds a font to `definitions` under a family of its own, returning the family name.
///
/// Each engine font becomes its own family rather than a fallback within one, because the drawing
/// API selects a family by name and expects exactly the glyphs of that face.
fn register(definitions: &mut FontDefinitions, font: &Font) -> Result<Arc<str>> {
    let data = match font.source() {
        FontSrc::None => return Err(anyhow!("font `{}` has no data source", font.name())),
        FontSrc::Bytes(bytes) => egui::FontData::from_static(bytes),
        FontSrc::Path(path) => egui::FontData::from_owned(
            std::fs::read(path)
                .with_context(|| format!("failed to read font file {}", path.display()))?,
        ),
    };
    let name: Arc<str> = Arc::from(font.name());
    definitions
        .font_data
        .insert(font.name().to_owned(), Arc::new(data));
    definitions.families.insert(
        FontFamily::Name(Arc::clone(&name)),
        vec![font.name().to_owned()],
    );
    Ok(name)
}
