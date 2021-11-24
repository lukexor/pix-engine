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

use crate::{prelude::*, renderer::Rendering};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::{
    borrow::Cow,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// A hashed  identifier for internal state management.
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
///     let mut style = Spacing::default();
///     style.frame_pad = point!(10, 10);
///     style.item_pad = point!(5, 5);
///     let theme = Theme::builder()
///         .with_font(
///             FontType::Body,
///             Font::from_file("Some font", "./some_font.ttf"),
///             16,
///             FontStyle::ITALIC,
///         )
///         .with_font(
///             FontType::Heading,
///             Font::NOTO,
///             22,
///             FontStyle::BOLD | FontStyle::UNDERLINE
///         )
///         .with_color(ColorType::OnBackground, Color::BLACK)
///         .with_color(ColorType::Background, Color::DARK_GRAY)
///         .with_style(style)
///         .build();
///     let mut engine = PixEngine::builder()
///         .with_theme(theme)
///         .build()?;
///     let mut app = MyApp;
///     engine.run(&mut app)
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
pub struct Builder {
    name: String,
    fonts: Fonts,
    sizes: FontSizes,
    styles: FontStyles,
    colors: Colors,
    spacing: Spacing,
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

    /// Set font theme values for a given [`FontType`].
    pub fn with_font(
        &mut self,
        font_type: FontType,
        font: Font,
        size: u32,
        style: FontStyle,
    ) -> &mut Self {
        match font_type {
            FontType::Body => {
                self.fonts.body = font;
                self.sizes.body = size;
                self.styles.body = style;
            }
            FontType::Heading => {
                self.fonts.heading = font;
                self.sizes.heading = size;
                self.styles.heading = style;
            }
            FontType::Monospace => {
                self.fonts.monospace = font;
                self.sizes.monospace = size;
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
    pub fn with_style(&mut self, style: Spacing) -> &mut Self {
        self.spacing = style;
        self
    }

    /// Convert `Builder` into a [Theme] instance.
    pub fn build(&self) -> Theme {
        Theme {
            name: self.name.clone(),
            fonts: self.fonts.clone(),
            sizes: self.sizes,
            styles: self.styles,
            colors: self.colors,
            spacing: self.spacing,
        }
    }
}

/// Represents a font family name along with the font glyph source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
pub struct Font {
    /// Family name of the font.
    pub(crate) name: Cow<'static, str>,
    /// Data source for the font.
    pub(crate) source: FontSrc,
}

impl Default for Font {
    fn default() -> Self {
        Self::EMULOGIC
    }
}

impl Font {
    const NOTO_TTF: &'static [u8] = include_bytes!("../../assets/noto_sans_regular.ttf");
    const EMULOGIC_TTF: &'static [u8] = include_bytes!("../../assets/emulogic.ttf");
    const INCONSOLATA_TTF: &'static [u8] = include_bytes!("../../assets/inconsolata_bold.ttf");

    /// [Noto Sans Regular](https://fonts.google.com/noto/specimen/Noto+Sans) - an open-source used
    /// by Linux and Google.
    pub const NOTO: Self = Self::from_bytes("Noto", Self::NOTO_TTF);

    /// Emulogic - a bold, retro gaming pixel font by Freaky Fonts.
    pub const EMULOGIC: Self = Self::from_bytes("Emulogic", Self::EMULOGIC_TTF);

    /// [Inconsolata](https://fonts.google.com/specimen/Inconsolata) - an open-source monospace
    /// font designed for source code and terminals.
    pub const INCONSOLATA: Self = Self::from_bytes("Inconsolata", Self::INCONSOLATA_TTF);

    /// Constructs a new `Font` instance from a static byte array.
    #[inline]
    pub const fn from_bytes(name: &'static str, bytes: &'static [u8]) -> Self {
        Self {
            name: Cow::Borrowed(name),
            source: FontSrc::from_bytes(bytes),
        }
    }

    /// Constructs a new `Font` instance from a file.
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) enum FontSrc {
    /// A font from byte data.
    Bytes(&'static [u8]),
    #[cfg(not(target_arch = "wasm32"))]
    /// A path to a `.ttf` font file.
    Path(PathBuf),
}

impl FontSrc {
    pub(crate) const fn from_bytes(bytes: &'static [u8]) -> Self {
        Self::Bytes(bytes)
    }

    pub(crate) fn from_file<P: Into<PathBuf>>(path: P) -> Self {
        Self::Path(path.into())
    }
}

/// A set of font families for body, heading, and monospace text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
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
            body: Font::EMULOGIC,
            heading: Font::EMULOGIC,
            monospace: Font::INCONSOLATA,
        }
    }
}

/// A set of font sizes for body, heading, and monospace text.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FontSizes {
    /// Body font size.
    pub body: u32,
    /// Heading font size.
    pub heading: u32,
    /// Monospace font size.
    pub monospace: u32,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            body: 12,
            heading: 20,
            monospace: 16,
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
    pub fn dark() -> Self {
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
    pub fn light() -> Self {
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

/// A set of styles for sizing, padding, borders, etc for theming UI elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Spacing {
    /// Padding between the edge of frames/windows and UI widgets.
    pub frame_pad: PointI2,
    /// Padding between UI widgets.
    pub item_pad: PointI2,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            frame_pad: point![8, 8],
            item_pad: point![8, 6],
        }
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
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
pub struct Theme {
    /// The name of this theme.
    pub name: String,
    /// The font families used in this theme.
    pub fonts: Fonts,
    /// The font sizes used in this theme.
    pub sizes: FontSizes,
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
            sizes: FontSizes::default(),
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
            sizes: FontSizes::default(),
            styles: FontStyles::default(),
            spacing: Spacing::default(),
        }
    }
}

impl PixState {
    /// Set the font size for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the given font size from the currently loaded font data, then
    /// an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.font_size(22);
    ///     s.text("Some big text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn font_size(&mut self, size: u32) -> PixResult<()> {
        self.theme.sizes.body = size;
        self.renderer.font_size(self.theme.sizes.body)
    }

    /// Return the dimensions of given text for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the current font, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let text = "Some text";
    ///     let (w, h) = s.size_of(text)?;
    ///     // Draw a box behind the text
    ///     s.rect(rect![s.cursor_pos() - 10, w as i32 + 20, h as i32 + 20]);
    ///     s.text(text)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn size_of<S: AsRef<str>>(&mut self, text: S) -> PixResult<(u32, u32)> {
        self.renderer
            .size_of(text.as_ref(), self.settings.wrap_width)
    }

    /// Set the font style for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the current font, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.font_style(FontStyle::BOLD);
    ///     s.text("Some bold text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn font_style(&mut self, style: FontStyle) {
        self.theme.styles.body = style;
        self.renderer.font_style(style);
    }

    /// Set the font family for drawing to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to load the given font size from the currently loaded font data, then
    /// an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.font_family(Font::NOTO)?;
    ///     s.text("Some NOTO family text")?;
    ///     s.font_family(Font::from_file("Custom font", "./custom_font.ttf"))?;
    ///     s.text("Some custom family text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    #[inline]
    pub fn font_family(&mut self, font: Font) -> PixResult<()> {
        self.theme.fonts.body = font;
        self.renderer.font_family(&self.theme.fonts.body)
    }

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
