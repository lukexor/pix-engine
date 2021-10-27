//! UI theme functions.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;
use std::str::FromStr;

pub mod fonts {
    //! A list of library-provided font families.

    use super::{Font, FontSrc};

    const ARIAL_TTF: &[u8] = include_bytes!("../../assets/arial.ttf");
    const EMULOGIC_TTF: &[u8] = include_bytes!("../../assets/emulogic.ttf");
    const INCONSOLATA_TTF: &[u8] = include_bytes!("../../assets/inconsolata.ttf");

    /// Arial
    pub const ARIAL: Font = Font::new("Arial", FontSrc::Bytes(ARIAL_TTF));

    /// Emulogic - bold, pixel font.
    pub const EMULOGIC: Font = Font::new("Emulogic", FontSrc::Bytes(EMULOGIC_TTF));

    /// Inconsolata - monospace font.
    pub const INCONSOLATA: Font = Font::new("Inconsolata", FontSrc::Bytes(INCONSOLATA_TTF));
}

/// A builder to generate custom [Theme]s.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeBuilder {
    fonts: ThemeFonts,
    font_sizes: ThemeFontSizes,
    font_styles: ThemeFontStyles,
    colors: ThemeColors,
    style: ThemeStyle,
}

/// Represents a given font-themed section in a UI.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ThemeFontType {
    /// For paragraphs, links, buttons, etc
    Body,
    /// For headings and sub-headings.
    Heading,
    /// For fixed-width text.
    Monospace,
}

/// Represents a given color-themed section in a UI.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ThemeColorType {
    /// Body text foreground.
    Text,
    /// Body background.
    Background,
    /// Primary brand for linsk, buttons, etc.
    Primary,
    /// Secondary brand for alternative styling.
    Secondary,
    /// Contrast for emphasis.
    Accent,
    /// Background highlighting of text.
    Highlight,
    /// Faint for backgrounds, borders, or accents not requiring contrast.
    Muted,
}

impl ThemeBuilder {
    /// Constructs a default `ThemeBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set font theme values for a given [ThemeFontType].
    pub fn with_font(
        &mut self,
        font_type: ThemeFontType,
        font: Font,
        size: u32,
        style: FontStyle,
    ) -> &mut Self {
        use ThemeFontType::*;
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

    /// Set color theme for a given [ThemeColorType].
    pub fn with_color(&mut self, color_type: ThemeColorType, color: Color) -> &mut Self {
        use ThemeColorType::*;
        match color_type {
            Text => self.colors.text = color,
            Background => self.colors.background = color,
            Primary => self.colors.background = color,
            Secondary => self.colors.secondary = color,
            Accent => self.colors.accent = color,
            Highlight => self.colors.highlight = color,
            Muted => self.colors.muted = color,
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
            fonts: self.fonts.clone(),
            font_sizes: self.font_sizes.clone(),
            font_styles: self.font_styles.clone(),
            colors: self.colors.clone(),
            style: self.style,
        }
    }
}

/// Represents a font family. (e.g. "helvetica").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Font {
    /// Family name of the font.
    pub name: &'static str,
    /// Data source for the font.
    pub source: FontSrc,
}

impl Font {
    /// Constructs a new `Font` instance.
    pub const fn new(name: &'static str, source: FontSrc) -> Self {
        Self { name, source }
    }
}

/// Represents source of font data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontSrc {
    /// A font from static byte data.
    Bytes(&'static [u8]),
    /// A custom string or path to a `.ttf` font file.
    Custom(String),
}

impl Default for Font {
    fn default() -> Self {
        fonts::EMULOGIC
    }
}

/// A set of font families for body, heading, and monospace text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeFonts {
    pub(crate) body: Font,
    pub(crate) heading: Font,
    pub(crate) monospace: Font,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeFontSizes {
    pub(crate) body: u32,
    pub(crate) heading: u32,
    pub(crate) monospace: u32,
}

impl Default for ThemeFontSizes {
    fn default() -> Self {
        Self {
            body: 14,
            heading: 20,
            monospace: 14,
        }
    }
}

/// A set of font styles for body, heading, and monospace text.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeFontStyles {
    pub(crate) body: FontStyle,
    pub(crate) heading: FontStyle,
    pub(crate) monospace: FontStyle,
}

/// A set of colors for theming UI elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeColors {
    pub(crate) text: Color,
    pub(crate) background: Color,
    pub(crate) primary: Color,
    pub(crate) secondary: Color,
    pub(crate) accent: Color,
    pub(crate) highlight: Color,
    pub(crate) muted: Color,
}

impl ThemeColors {
    /// A dark color theme.
    pub fn dark() -> Self {
        Self {
            text: color!(0xf4),
            background: Color::from_str("#151617").unwrap(),
            primary: Color::from_str("#0f2a3f").unwrap(),
            secondary: Color::from_str("#2b4455").unwrap(),
            accent: Color::from_str("#c78654").unwrap(),
            highlight: Color::from_str("#236d7a").unwrap(),
            muted: Color::from_str("#20394f").unwrap(),
        }
    }

    /// A light color theme.
    pub fn light() -> Self {
        Self {
            text: Color::from_str("#272736").unwrap(),
            background: Color::from_str("#fffefe").unwrap(),
            primary: Color::from_str("#d0e3e6").unwrap(),
            secondary: Color::from_str("#8bacc9").unwrap(),
            accent: Color::from_str("#d8c072").unwrap(),
            highlight: Color::from_str("#5c94b1").unwrap(),
            muted: Color::from_str("#7b7995").unwrap(),
        }
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::dark()
    }
}

/// A set of styles for sizing, padding, borders, etc for theming UI elements.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ThemeStyle {
    pub(crate) frame_pad: PointI2,
    pub(crate) item_pad: PointI2,
}

impl Default for ThemeStyle {
    fn default() -> Self {
        Self {
            frame_pad: point![8, 8],
            item_pad: point![8, 6],
        }
    }
}

/// A UI theme containing font families, sizes, styles, and colors.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Theme {
    pub(crate) fonts: ThemeFonts,
    pub(crate) font_sizes: ThemeFontSizes,
    pub(crate) font_styles: ThemeFontStyles,
    pub(crate) colors: ThemeColors,
    pub(crate) style: ThemeStyle,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            fonts: ThemeFonts::default(),
            font_sizes: ThemeFontSizes::default(),
            font_styles: ThemeFontStyles::default(),
            colors: ThemeColors::default(),
            style: ThemeStyle::default(),
        }
    }
}

impl Theme {
    /// Constructs a default [ThemeBuilder] which can build a `Theme`.
    #[inline]
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::new()
    }
}

impl PixState {
    /// Set the font color for drawing to the current canvas.
    pub fn font_color<C: Into<Color>>(&mut self, color: C) {
        self.theme.colors.text = color.into();
    }

    /// Set the font size for drawing to the current canvas.
    pub fn font_size<S: AsPrimitive<u32>>(&mut self, size: S) -> PixResult<()> {
        self.theme.font_sizes.body = size.as_();
        Ok(self.renderer.font_size(self.theme.font_sizes.body)?)
    }

    /// Return the dimensions of given text for drawing to the current canvas.
    pub fn size_of<S: AsRef<str>>(&mut self, text: S) -> PixResult<(u32, u32)> {
        Ok(self.renderer.size_of(text.as_ref())?)
    }

    /// Set the font style for drawing to the current canvas.
    pub fn font_style(&mut self, style: FontStyle) {
        self.theme.font_styles.body = style;
        self.renderer.font_style(style);
    }

    /// Set the font family for drawing to the current canvas.
    pub fn font_family(&mut self, font: Font) -> PixResult<()> {
        self.theme.fonts.body = font;
        Ok(self.renderer.font_family(&self.theme.fonts.body)?)
    }

    /// Returns the current theme body font.
    #[inline]
    pub fn body_font(&self) -> &Font {
        &self.theme.fonts.body
    }

    /// Returns the current theme heading font.
    #[inline]
    pub fn heading_font(&self) -> &Font {
        &self.theme.fonts.heading
    }

    /// Returns the current theme fixed-width font.
    #[inline]
    pub fn monospace_font(&self) -> &Font {
        &self.theme.fonts.monospace
    }

    /// Body text foreground color.
    ///
    /// Returns the current theme text color.
    #[inline]
    pub fn text_color(&self) -> Color {
        self.theme.colors.text
    }

    /// Body background color.
    ///
    /// Returns the current theme background color.
    #[inline]
    pub fn background_color(&self) -> Color {
        self.theme.colors.background
    }

    /// Primary brand color for links, buttons, etc.
    ///
    /// Returns the current theme primary color.
    #[inline]
    pub fn primary_color(&self) -> Color {
        self.theme.colors.primary
    }

    /// Secondary brand color for alternative styling.
    ///
    /// Returns the current theme secondary color.
    #[inline]
    pub fn secondary_color(&self) -> Color {
        self.theme.colors.secondary
    }

    /// A contrast color for emphasis.
    ///
    /// Returns the current theme accent color.
    #[inline]
    pub fn accent_color(&self) -> Color {
        self.theme.colors.accent
    }

    /// A background color for highlighting text.
    ///
    /// Returns the current theme highlight color.
    #[inline]
    pub fn highlight_color(&self) -> Color {
        self.theme.colors.highlight
    }

    /// A faint color for backgrounds, borders, and accents that do not require high contrast with
    /// the background.
    ///
    /// Returns the current theme muted color.
    #[inline]
    pub fn muted_color(&self) -> Color {
        self.theme.colors.muted
    }
}
