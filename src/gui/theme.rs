//! UI theme methods.
//!
//! Provides various methods for changing and querying the current UI theme used to render text and
//! UI widgets.
//!
//! # Example
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App { checkbox: bool, text_field: String };
//! # impl AppState for App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     s.fill(Color::CADET_BLUE); // Change text color
//!     s.font_size(14)?;
//!     s.font_style(FontStyle::BOLD);
//!     s.font_family(Font::INCONSOLATA)?;
//!     s.text("Blue, bold, size 14 text in Inconsolata font")?;
//!     Ok(())
//! }
//! # }
//! ```

use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::fmt;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::{
    borrow::Cow,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// A hashed identifier for internal state management.
pub(crate) type FontId = u64;

/// A builder to generate custom [Theme]s.
///
/// # Example
///
/// ```no_run
/// # use pix_engine::prelude::*;
/// use pix_engine::gui::theme::*;
/// # struct MyApp;
/// # impl AppState for MyApp {
/// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
/// # }
/// fn main() -> PixResult<()> {
///     let theme = Theme::builder()
///         .with_font_size(16)
///         .with_font(
///             FontType::Body,
///             Font::from_file("Some font", "./some_font.ttf"),
///             FontStyle::ITALIC,
///         )
///         .with_font(
///             FontType::Heading,
///             Font::NOTO,
///             FontStyle::BOLD | FontStyle::UNDERLINE
///         )
///         .with_color(ColorType::OnBackground, Color::BLACK)
///         .with_color(ColorType::Background, Color::DARK_GRAY)
///         .with_spacing(
///             Spacing::builder()
///                 .frame_pad(10, 10)
///                 .item_pad(5, 5)
///                 .build()
///         )
///         .build();
///     let mut engine = PixEngine::builder()
///         .with_theme(theme)
///         .build()?;
///     let mut app = MyApp;
///     engine.run(&mut app)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Builder {
    name: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    fonts: Fonts,
    size: u32,
    styles: FontStyles,
    colors: Colors,
    spacing: Spacing,
}

impl Default for Builder {
    fn default() -> Self {
        let theme = Theme::default();
        Self {
            name: theme.name,
            fonts: theme.fonts,
            size: theme.font_size,
            styles: theme.styles,
            colors: theme.colors,
            spacing: theme.spacing,
        }
    }
}

/// Represents a given font-themed section in a UI.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FontType {
    /// For paragraphs, links, buttons, etc
    Body,
    /// For headings and sub-headings.
    Heading,
    /// For fixed-width text.
    Monospace,
}

/// Represents a given color-themed section in a UI.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColorType {
    /// Background color, used to clear the screen each frame and appears behind scrollable
    /// content.
    Background,
    /// Surface color, used to render surfaces of widgets, cards, sheets, and menus.
    Surface,
    /// Primary color displayed most often across widgets.
    Primary,
    /// Primary variant color, optional.
    PrimaryVariant,
    /// Secondary color for accents and distinguishing content, optional.
    Secondary,
    /// Secondary variant color, optional.
    SecondaryVariant,
    /// Error highlighting of text and outlines.
    Error,
    /// Text and icon color when rendered over the background color.
    OnBackground,
    /// Text and icon color when rendered over the surface color.
    OnSurface,
    /// Text and icon color when rendered over a primary color.
    OnPrimary,
    /// Text and icon color when rendered over a secondary color.
    OnSecondary,
    /// Text and icon color when rendered over the error color.
    OnError,
}

impl Builder {
    /// Constructs a default [Theme] `Builder`.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Set font size.
    pub fn with_font_size(&mut self, size: u32) -> &mut Self {
        self.size = size;
        self
    }

    /// Set font theme values for a given [`FontType`].
    pub fn with_font(&mut self, font_type: FontType, font: Font, style: FontStyle) -> &mut Self {
        match font_type {
            FontType::Body => {
                self.fonts.body = font;
                self.styles.body = style;
            }
            FontType::Heading => {
                self.fonts.heading = font;
                self.styles.heading = style;
            }
            FontType::Monospace => {
                self.fonts.monospace = font;
                self.styles.monospace = style;
            }
        }
        self
    }

    /// Set color theme for a given [`ColorType`].
    pub fn with_color<C: Into<Color>>(&mut self, color_type: ColorType, color: C) -> &mut Self {
        let color = color.into();
        let c = &mut self.colors;
        match color_type {
            ColorType::Background => c.background = color,
            ColorType::Surface => c.surface = color,
            ColorType::Primary => c.primary = color,
            ColorType::PrimaryVariant => c.primary_variant = color,
            ColorType::Secondary => c.secondary = color,
            ColorType::SecondaryVariant => c.secondary_variant = color,
            ColorType::Error => c.error = color,
            ColorType::OnBackground => c.on_background = color,
            ColorType::OnSurface => c.on_surface = color,
            ColorType::OnPrimary => c.on_primary = color,
            ColorType::OnSecondary => c.on_secondary = color,
            ColorType::OnError => c.on_error = color,
        }
        self
    }

    /// Set element padding space.
    pub fn with_spacing(&mut self, spacing: Spacing) -> &mut Self {
        self.spacing = spacing;
        self
    }

    /// Convert `Builder` into a [Theme] instance.
    pub fn build(&self) -> Theme {
        Theme {
            name: self.name.clone(),
            fonts: self.fonts.clone(),
            font_size: self.size,
            styles: self.styles,
            colors: self.colors,
            spacing: self.spacing,
        }
    }
}

/// Represents a font family name along with the font glyph source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
pub struct Font {
    /// Family name of the font.
    pub(crate) name: Cow<'static, str>,
    #[cfg(not(target_arch = "wasm32"))]
    /// Data source for the font.
    pub(crate) source: FontSrc,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for Font {
    fn default() -> Self {
        Self::EMULOGIC
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for Font {
    fn default() -> Self {
        Self::named("Arial")
    }
}

impl Font {
    #[cfg(not(target_arch = "wasm32"))]
    const NOTO_TTF: &'static [u8] = include_bytes!("../../assets/noto_sans_regular.ttf");
    #[cfg(not(target_arch = "wasm32"))]
    const EMULOGIC_TTF: &'static [u8] = include_bytes!("../../assets/emulogic.ttf");
    #[cfg(not(target_arch = "wasm32"))]
    const INCONSOLATA_TTF: &'static [u8] = include_bytes!("../../assets/inconsolata_bold.ttf");

    /// [Noto Sans Regular](https://fonts.google.com/noto/specimen/Noto+Sans) - an open-source used
    /// by Linux and Google.
    #[cfg(not(target_arch = "wasm32"))]
    pub const NOTO: Self = Self::from_bytes("Noto", Self::NOTO_TTF);

    /// Emulogic - a bold, retro gaming pixel font by Freaky Fonts.
    #[cfg(not(target_arch = "wasm32"))]
    pub const EMULOGIC: Self = Self::from_bytes("Emulogic", Self::EMULOGIC_TTF);

    /// [Inconsolata](https://fonts.google.com/specimen/Inconsolata) - an open-source monospace
    /// font designed for source code and terminals.
    #[cfg(not(target_arch = "wasm32"))]
    pub const INCONSOLATA: Self = Self::from_bytes("Inconsolata", Self::INCONSOLATA_TTF);

    /// Constructs a new `Font` instance with a given name.
    #[inline]
    pub const fn named(name: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            #[cfg(not(target_arch = "wasm32"))]
            source: FontSrc::None,
        }
    }

    /// Constructs a new `Font` instance from a static byte array.
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub const fn from_bytes(name: &'static str, bytes: &'static [u8]) -> Self {
        Self {
            name: Cow::Borrowed(name),
            source: FontSrc::from_bytes(bytes),
        }
    }

    /// Constructs a new `Font` instance from a file.
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub fn from_file<S, P>(name: S, path: P) -> Self
    where
        S: Into<Cow<'static, str>>,
        P: Into<PathBuf>,
    {
        Self {
            name: name.into(),
            source: FontSrc::from_file(path),
        }
    }

    /// Returns the name of the font family.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the source data of the font family.
    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    #[must_use]
    pub(crate) const fn source(&self) -> &FontSrc {
        &self.source
    }

    /// Returns the hashed identifier for this font family.
    #[inline]
    #[must_use]
    pub fn id(&self) -> FontId {
        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        hasher.finish()
    }
}

/// Represents a source of font glyph data.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg(not(target_arch = "wasm32"))]
pub(crate) enum FontSrc {
    /// No source provided.
    None,
    /// A font from byte data.
    Bytes(&'static [u8]),
    /// A path to a `.ttf` font file.
    Path(PathBuf),
}

#[cfg(not(target_arch = "wasm32"))]
impl fmt::Debug for FontSrc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Bytes(bytes) => write!(f, "Bytes([u8; {}])", bytes.len()),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Path(path) => write!(f, "Path({})", path.display()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl FontSrc {
    pub(crate) const fn from_bytes(bytes: &'static [u8]) -> Self {
        Self::Bytes(bytes)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn from_file<P: Into<PathBuf>>(path: P) -> Self {
        Self::Path(path.into())
    }
}

/// A set of font families for body, heading, and monospace text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Fonts {
    /// Body font.
    pub body: Font,
    /// Heading font.
    pub heading: Font,
    /// Monospace font.
    pub monospace: Font,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            body: Font::default(),
            heading: Font::default(),
            #[cfg(not(target_arch = "wasm32"))]
            monospace: Font::INCONSOLATA,
            #[cfg(target_arch = "wasm32")]
            monospace: Font::named("Courier"),
        }
    }
}

/// A set of [`FontStyle`]s for body, heading, and monospace text.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FontStyles {
    /// Body style.
    pub body: FontStyle,
    /// Heading style.
    pub heading: FontStyle,
    /// Monospace style.
    pub monospace: FontStyle,
}

/// A set of [Color]s for theming UI elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Colors {
    /// Background color, used to clear the screen each frame and appears behind scrollable
    /// content.
    pub background: Color,
    /// Surface color, used to render surfaces of widgets, cards, sheets, and menus.
    pub surface: Color,
    /// Primary color displayed most often across widgets.
    pub primary: Color,
    /// Primary variant color.
    pub primary_variant: Color,
    /// Secondary color for accents and distinguishing content, optional.
    pub secondary: Color,
    /// Secondary variant color, optional.
    pub secondary_variant: Color,
    /// Error highlighting of text and outlines.
    pub error: Color,
    /// Text and icon color when rendered over the background color.
    pub on_background: Color,
    /// Text and icon color when rendered over the surface color.
    pub on_surface: Color,
    /// Text and icon color when rendered over a primary color.
    pub on_primary: Color,
    /// Text and icon color when rendered over a secondary color.
    pub on_secondary: Color,
    /// Text and icon color when rendered over the error color.
    pub on_error: Color,
}

impl Colors {
    /// A dark color theme.
    #[allow(clippy::unreadable_literal)]
    pub const fn dark() -> Self {
        Self {
            background: Color::from_hex(0x121212),
            surface: Color::from_hex(0x121212),
            primary: Color::from_hex(0xbf360c),
            primary_variant: Color::from_hex(0xff6f43),
            secondary: Color::from_hex(0x0c95bf),
            secondary_variant: Color::from_hex(0x43d3ff),
            error: Color::from_hex(0xcf6679),
            on_background: Color::WHITE,
            on_surface: Color::WHITE,
            on_primary: Color::BLACK,
            on_secondary: Color::BLACK,
            on_error: Color::BLACK,
        }
    }

    /// A light color theme.
    #[allow(clippy::unreadable_literal)]
    pub const fn light() -> Self {
        Self {
            background: Color::from_hex(0xffffff),
            surface: Color::from_hex(0xffffff),
            primary: Color::from_hex(0x00796b),
            primary_variant: Color::from_hex(0x4db6ac),
            secondary: Color::from_hex(0x79000e),
            secondary_variant: Color::from_hex(0xb64d58),
            error: Color::from_hex(0xb00020),
            on_background: Color::BLACK,
            on_surface: Color::BLACK,
            on_primary: Color::WHITE,
            on_secondary: Color::WHITE,
            on_error: Color::WHITE,
        }
    }

    /// Return the on background overlay color.
    #[inline]
    pub fn on_background(&self) -> Color {
        self.on_background.blended(self.background, 0.87)
    }

    /// Return the on surface overlay color.
    #[inline]
    pub fn on_surface(&self) -> Color {
        self.on_surface.blended(self.surface, 0.87)
    }

    /// Return the disabled color.
    #[inline]
    pub fn disabled(&self) -> Color {
        self.on_background.blended(self.background, 0.38)
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self::dark()
    }
}

/// Builds a [Spacing] instance by customizing various space and padding settings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpacingBuilder {
    frame_pad: Point<i32>,
    item_pad: Point<i32>,
    scroll_size: i32,
}

impl Default for SpacingBuilder {
    fn default() -> Self {
        let spacing = Spacing::default();
        Self {
            frame_pad: spacing.frame_pad,
            item_pad: spacing.item_pad,
            scroll_size: spacing.scroll_size,
        }
    }
}

impl SpacingBuilder {
    /// Set padding between the edge of frames/windows and UI widgets.
    pub fn frame_pad(&mut self, x: i32, y: i32) -> &mut Self {
        self.frame_pad = point!(x, y);
        self
    }

    /// Set padding between UI widgets.
    pub fn item_pad(&mut self, x: i32, y: i32) -> &mut Self {
        self.item_pad = point!(x, y);
        self
    }

    /// Set scroll bar size in UI widgets.
    pub fn scroll_size(&mut self, size: i32) -> &mut Self {
        self.scroll_size = size;
        self
    }

    /// Convert `SpacingBuilder` into a [Spacing] instance.
    pub const fn build(&self) -> Spacing {
        Spacing {
            frame_pad: self.frame_pad,
            item_pad: self.item_pad,
            scroll_size: self.scroll_size,
        }
    }
}

/// A set of styles for sizing, padding, borders, etc for theming UI elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Spacing {
    /// Padding between the edge of frames/windows and UI widgets.
    pub frame_pad: Point<i32>,
    /// Padding between UI widgets.
    pub item_pad: Point<i32>,
    /// Scroll bar size in UI widgets.
    pub scroll_size: i32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            frame_pad: point![8, 8],
            item_pad: point![8, 6],
            scroll_size: 12,
        }
    }
}

impl Spacing {
    /// Constructs a default [`SpacingBuilder`] which can build a `Spacing` instance.
    ///
    /// See [`SpacingBuilder`] for examples.
    pub fn builder() -> SpacingBuilder {
        SpacingBuilder::default()
    }
}

/// A UI `Theme` containing font families, sizes, styles, and colors.
///
/// See the [Builder] examples for building a custom theme.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// # struct App { checkbox: bool, text_field: String };
/// # impl AppState for App {
/// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
///     s.fill(Color::CADET_BLUE); // Change font color
///     s.font_size(16)?;
///     s.font_style(FontStyle::UNDERLINE);
///     s.font_family(Font::from_file("Some font", "./some_font.ttf"))?;
///     s.text("Blue, underlined, size 16 text in Some Font")?;
///     Ok(())
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Theme {
    /// The name of this theme.
    pub name: String,
    /// The font families used in this theme.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub fonts: Fonts,
    /// The body font size used in this theme.
    pub font_size: u32,
    /// The font styles used in this theme.
    pub styles: FontStyles,
    /// The colors used in this theme.
    pub colors: Colors,
    /// The padding, offsets, and other styles used in this theme.
    pub spacing: Spacing,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Constructs a default [Builder] which can build a `Theme` instance.
    ///
    /// See [Builder] for examples.
    #[inline]
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Constructs a default dark `Theme`.
    #[inline]
    pub fn dark() -> Self {
        Self {
            name: "Dark".into(),
            colors: Colors::dark(),
            fonts: Fonts::default(),
            font_size: 12,
            styles: FontStyles::default(),
            spacing: Spacing::default(),
        }
    }

    /// Constructs a default light `Theme`.
    #[inline]
    pub fn light() -> Self {
        Self {
            name: "Light".into(),
            colors: Colors::light(),
            fonts: Fonts::default(),
            font_size: 12,
            styles: FontStyles::default(),
            spacing: Spacing::default(),
        }
    }
}

impl PixState {
    /// Returns the reference to the current theme.
    #[inline]
    pub const fn theme(&self) -> &Theme {
        &self.theme
    }

    /// Returns the a mutable reference to the current theme.
    #[inline]
    pub fn theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    /// Sets a new theme.
    #[inline]
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        let colors = self.theme.colors;
        self.background(colors.background);
        self.fill(colors.on_background());
    }
}
