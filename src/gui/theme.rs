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
//!     s.fill(CADET_BLUE); // Change text color
//!     s.font_size(14)?;
//!     s.font_style(FontStyle::BOLD);
//!     s.font_family(fonts::INCONSOLATA)?;
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
    str::FromStr,
};

/// A hashed  identifier for internal state management.
pub(crate) type FontId = u64;

pub mod fonts {
    //! A list of library-provided font families.

    use super::Font;

    const NOTO_TTF: &[u8] = include_bytes!("../../assets/noto_sans_regular.ttf");
    const EMULOGIC_TTF: &[u8] = include_bytes!("../../assets/emulogic.ttf");
    const INCONSOLATA_TTF: &[u8] = include_bytes!("../../assets/inconsolata_bold.ttf");

    /// [Noto Sans Regular](https://fonts.google.com/noto/specimen/Noto+Sans) - an open-source used
    /// by Linux and Google.
    pub const NOTO: Font = Font::from_bytes("Noto", NOTO_TTF);

    /// Emulogic - a bold, retro gaming pixel font by Freaky Fonts.
    pub const EMULOGIC: Font = Font::from_bytes("Emulogic", EMULOGIC_TTF);

    /// [Inconsolata](https://fonts.google.com/specimen/Inconsolata) - an open-source monospace
    /// font designed for source code and terminals.
    pub const INCONSOLATA: Font = Font::from_bytes("Inconsolata", INCONSOLATA_TTF);
}

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
///     let mut style = ThemeStyle::default();
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
///             fonts::NOTO,
///             22,
///             FontStyle::BOLD | FontStyle::UNDERLINE
///         )
///         .with_color(ColorType::OnBackground, BLACK)
///         .with_color(ColorType::Background, DARK_GRAY)
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
pub struct ThemeBuilder {
    name: String,
    fonts: ThemeFonts,
    font_sizes: ThemeFontSizes,
    font_styles: ThemeFontStyles,
    colors: ThemeColors,
    style: ThemeStyle,
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

impl ThemeBuilder {
    /// Constructs a default `ThemeBuilder`.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Set font theme values for a given [FontType].
    pub fn with_font(
        &mut self,
        font_type: FontType,
        font: Font,
        size: u32,
        style: FontStyle,
    ) -> &mut Self {
        use FontType::*;
        match font_type {
            Body => {
                self.fonts.body = font;
                self.font_sizes.body = size;
                self.font_styles.body = style;
            }
            Heading => {
                self.fonts.heading = font;
                self.font_sizes.heading = size;
                self.font_styles.heading = style;
            }
            Monospace => {
                self.fonts.monospace = font;
                self.font_sizes.monospace = size;
                self.font_styles.monospace = style;
            }
        }
        self
    }

    /// Set color theme for a given [ColorType].
    pub fn with_color<C: Into<Color>>(&mut self, color_type: ColorType, color: C) -> &mut Self {
        let color = color.into();
        use ColorType::*;
        let c = &mut self.colors;
        match color_type {
            Background => c.background = color,
            Surface => c.surface = color,
            Primary => c.primary = color,
            PrimaryVariant => c.primary_variant = color,
            Secondary => c.secondary = color,
            SecondaryVariant => c.secondary_variant = color,
            Error => c.error = color,
            OnBackground => c.on_background = color,
            OnSurface => c.on_surface = color,
            OnPrimary => c.on_primary = color,
            OnSecondary => c.on_secondary = color,
            OnError => c.on_error = color,
        }
        self
    }

    /// Set element padding space.
    pub fn with_style(&mut self, style: ThemeStyle) -> &mut Self {
        self.style = style;
        self
    }

    /// Convert [ThemeBuilder] to a [Theme] instance.
    pub fn build(&self) -> Theme {
        Theme {
            name: self.name.clone(),
            fonts: self.fonts.clone(),
            font_sizes: self.font_sizes,
            font_styles: self.font_styles,
            colors: self.colors,
            style: self.style,
        }
    }
}

/// Represents a font family name along with the font glyph source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
#[non_exhaustive]
pub struct Font {
    /// Family name of the font.
    pub(crate) name: Cow<'static, str>,
    /// Data source for the font.
    pub(crate) source: FontSrc,
}

impl Default for Font {
    fn default() -> Self {
        fonts::EMULOGIC
    }
}

impl Font {
    /// Constructs a new `Font` instance from a static byte array.
    pub const fn from_bytes(name: &'static str, bytes: &'static [u8]) -> Self {
        Self {
            name: Cow::Borrowed(name),
            source: FontSrc::from_bytes(bytes),
        }
    }

    /// Constructs a new `Font` instance from a file.
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
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the hashed identifier for this font family.
    #[inline]
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
pub struct ThemeFonts {
    /// Body font.
    pub body: Font,
    /// Heading font.
    pub heading: Font,
    /// Monospace font.
    pub monospace: Font,
}

impl Default for ThemeFonts {
    fn default() -> Self {
        Self {
            body: fonts::EMULOGIC,
            heading: fonts::EMULOGIC,
            monospace: fonts::INCONSOLATA,
        }
    }
}

/// A set of font sizes for body, heading, and monospace text.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ThemeFontSizes {
    /// Body font size.
    pub body: u32,
    /// Heading font size.
    pub heading: u32,
    /// Monospace font size.
    pub monospace: u32,
}

impl Default for ThemeFontSizes {
    fn default() -> Self {
        Self {
            body: 12,
            heading: 20,
            monospace: 14,
        }
    }
}

/// A set of [FontStyle]s for body, heading, and monospace text.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ThemeFontStyles {
    /// Body style.
    pub body: FontStyle,
    /// Heading style.
    pub heading: FontStyle,
    /// Monospace style.
    pub monospace: FontStyle,
}

/// A set of [Color]s for theming UI elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ThemeColors {
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

impl ThemeColors {
    /// A dark color theme.
    pub fn dark() -> Self {
        Self {
            background: Color::from_str("#121212").unwrap(),
            surface: Color::from_str("#121212").unwrap(),
            primary: Color::from_str("#bf360c").unwrap(),
            primary_variant: Color::from_str("#ff6f43").unwrap(),
            secondary: Color::from_str("#0c95bf").unwrap(),
            secondary_variant: Color::from_str("#43d3ff").unwrap(),
            error: Color::from_str("#cf6679").unwrap(),
            on_background: WHITE,
            on_surface: WHITE,
            on_primary: BLACK,
            on_secondary: BLACK,
            on_error: BLACK,
        }
    }

    /// A light color theme.
    pub fn light() -> Self {
        Self {
            background: Color::from_str("#fff").unwrap(),
            surface: Color::from_str("#fff").unwrap(),
            primary: Color::from_str("#00796b").unwrap(),
            primary_variant: Color::from_str("#4db6ac").unwrap(),
            secondary: Color::from_str("#79000e").unwrap(),
            secondary_variant: Color::from_str("#b64d58").unwrap(),
            error: Color::from_str("#b00020").unwrap(),
            on_background: BLACK,
            on_surface: BLACK,
            on_primary: WHITE,
            on_secondary: WHITE,
            on_error: WHITE,
        }
    }

    /// Return the OnBackground color.
    #[inline]
    pub fn on_background(&self) -> Color {
        self.on_background.blended(self.background, 0.87)
    }

    /// Return the OnSurface color.
    #[inline]
    pub fn on_surface(&self) -> Color {
        self.on_surface.blended(self.surface, 0.87)
    }

    /// Return the Disabled color.
    #[inline]
    pub fn disabled(&self) -> Color {
        self.on_background.blended(self.background, 0.38)
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

/// A set of styles for sizing, padding, borders, etc for theming UI elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ThemeStyle {
    /// Padding between the edge of frames/windows and UI widgets.
    pub frame_pad: PointI2,
    /// Padding between UI widgets.
    pub item_pad: PointI2,
}

impl Default for ThemeStyle {
    fn default() -> Self {
        Self {
            frame_pad: point![8, 8],
            item_pad: point![8, 6],
        }
    }
}

/// A UI `Theme` containing font families, sizes, styles, and colors.
///
/// See [ThemeBuilder] examples for building a custom theme.
///
/// # Example
///
/// ```
/// # use pix_engine::prelude::*;
/// # struct App { checkbox: bool, text_field: String };
/// # impl AppState for App {
/// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
///     s.fill(CADET_BLUE); // Change font color
///     s.font_size(16)?;
///     s.font_style(FontStyle::UNDERLINE);
///     s.font_family(Font::from_file("Some font", "./some_font.ttf"))?;
///     s.text("Blue, underlined, size 16 text in Some Font")?;
///     Ok(())
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
#[non_exhaustive]
pub struct Theme {
    /// The name of this theme.
    pub name: String,
    /// The font families used in this theme.
    pub fonts: ThemeFonts,
    /// The font sizes used in this theme.
    pub font_sizes: ThemeFontSizes,
    /// The font styles used in this theme.
    pub font_styles: ThemeFontStyles,
    /// The colors used in this theme.
    pub colors: ThemeColors,
    /// The padding, offsets, and other styles used in this theme.
    pub style: ThemeStyle,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Constructs a default [ThemeBuilder] which can build a `Theme` instance.
    ///
    /// See [ThemeBuilder] for examples.
    #[inline]
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }

    /// Constructs a default `Dark` Theme.
    #[inline]
    pub fn dark() -> Theme {
        Self {
            name: "Dark".into(),
            colors: ThemeColors::dark(),
            fonts: ThemeFonts::default(),
            font_sizes: ThemeFontSizes::default(),
            font_styles: ThemeFontStyles::default(),
            style: ThemeStyle::default(),
        }
    }

    /// Constructs a default `Light` Theme.
    #[inline]
    pub fn light() -> Theme {
        Self {
            name: "Light".into(),
            colors: ThemeColors::light(),
            fonts: ThemeFonts::default(),
            font_sizes: ThemeFontSizes::default(),
            font_styles: ThemeFontStyles::default(),
            style: ThemeStyle::default(),
        }
    }
}

impl PixState {
    /// Set the font size for drawing to the current canvas.
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
    pub fn font_size(&mut self, size: u32) -> PixResult<()> {
        self.theme.font_sizes.body = size;
        self.renderer.font_size(self.theme.font_sizes.body)
    }

    /// Return the dimensions of given text for drawing to the current canvas.
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
    pub fn size_of<S: AsRef<str>>(&mut self, text: S) -> PixResult<(u32, u32)> {
        self.renderer
            .size_of(text.as_ref(), self.settings.wrap_width)
    }

    /// Set the font style for drawing to the current canvas.
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
    pub fn font_style(&mut self, style: FontStyle) {
        self.theme.font_styles.body = style;
        self.renderer.font_style(style);
    }

    /// Set the font family for drawing to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.font_family(fonts::NOTO)?;
    ///     s.text("Some NOTO family text")?;
    ///     s.font_family(Font::from_file("Custom font", "./custom_font.ttf"))?;
    ///     s.text("Some custom family text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn font_family(&mut self, font: Font) -> PixResult<()> {
        self.theme.fonts.body = font;
        self.renderer.font_family(&self.theme.fonts.body)
    }

    /// Returns the reference to the current theme.
    #[inline]
    pub fn theme(&self) -> &Theme {
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
