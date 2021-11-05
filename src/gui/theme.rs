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
//!     s.font_color(CADET_BLUE); // Change theme text color
//!     s.font_size(14)?; // Change theme body text size
//!     s.font_style(FontStyle::BOLD); // Change theme body font stylle
//!     s.font_family(fonts::INCONSOLATA)?;
//!     s.fill(s.background_color());
//!     Ok(())
//! }
//! # }
//! ```

use crate::{prelude::*, renderer::Rendering};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
use std::str::FromStr;

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
///     let dark_colors = ThemeColors::dark();
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
///         .with_color(ColorType::Text, BLACK)
///         .with_color(ColorType::Background, dark_colors.accent)
///         .with_style(ThemeStyle {
///             frame_pad: point!(10, 10),
///             item_pad: point!(5, 5),
///          })
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
    pub fn with_color(&mut self, color_type: ColorType, color: Color) -> &mut Self {
        use ColorType::*;
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

/// Represents a font family name along with the font glyph source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Font {
    /// Family name of the font.
    pub(crate) name: &'static str,
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
            name,
            source: FontSrc::from_bytes(bytes),
        }
    }

    /// Constructs a new `Font` instance from a file.
    pub fn from_file<P: Into<PathBuf>>(name: &'static str, path: P) -> Self {
        Self {
            name,
            source: FontSrc::from_file(path),
        }
    }
}

/// Represents a source of font glyph data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) enum FontSrc {
    /// A font from static byte data.
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
pub(crate) struct ThemeFonts {
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct ThemeFontSizes {
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

/// A set of [FontStyle]s for body, heading, and monospace text.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct ThemeFontStyles {
    pub(crate) body: FontStyle,
    pub(crate) heading: FontStyle,
    pub(crate) monospace: FontStyle,
}

/// A set of [Color]s for theming UI elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThemeColors {
    /// Body text foreground color.
    pub text: Color,
    /// Window/frame background color, used to clear the screen each frame.
    pub background: Color,
    /// Primary brand color for links, buttons, etc.
    pub primary: Color,
    /// Secondary brand color for alternative styling.
    pub secondary: Color,
    /// A contrast color for emphasis.
    pub accent: Color,
    /// A background color for highlighting text or focusing widgets.
    pub highlight: Color,
    /// A faint color for backgrounds, borders, and accents that do not require high contrast with
    /// the background.
    pub muted: Color,
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
///     s.fill(s.accent_color());
///     s.font_size(s.heading_font_size())?;
///     s.font_family(Font::from_file("Some font", "./some_font.ttf"))?;
///     Ok(())
/// }
/// # }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "'de: 'static")))]
pub struct Theme {
    pub(crate) fonts: ThemeFonts,
    pub(crate) font_sizes: ThemeFontSizes,
    pub(crate) font_styles: ThemeFontStyles,
    pub(crate) colors: ThemeColors,
    pub(crate) style: ThemeStyle,
}

impl Theme {
    /// Constructs a default [ThemeBuilder] which can build a `Theme` instance.
    ///
    /// See [ThemeBuilder] for examples.
    #[inline]
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }
}

impl PixState {
    /// Set the font color for drawing to the current canvas.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.font_color(ALICE_BLUE);
    ///     s.text("Some blue text")?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn font_color<C: Into<Color>>(&mut self, color: C) {
        self.theme.colors.text = color.into();
    }

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
        self.renderer.size_of(text.as_ref())
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

    /// Returns the current theme body font size.
    #[inline]
    pub fn body_font_size(&self) -> u32 {
        self.theme.font_sizes.body
    }

    /// Returns the current theme body font style.
    #[inline]
    pub fn body_font_style(&self) -> FontStyle {
        self.theme.font_styles.body
    }

    /// Returns the current theme body font family.
    #[inline]
    pub fn body_font(&self) -> &Font {
        &self.theme.fonts.body
    }

    /// Returns the current theme heading font size.
    #[inline]
    pub fn heading_font_size(&self) -> u32 {
        self.theme.font_sizes.heading
    }

    /// Returns the current theme heading font style.
    #[inline]
    pub fn heading_font_style(&self) -> FontStyle {
        self.theme.font_styles.heading
    }

    /// Returns the current theme heading font.
    #[inline]
    pub fn heading_font(&self) -> &Font {
        &self.theme.fonts.heading
    }

    /// Returns the current theme monospace font size.
    #[inline]
    pub fn monospace_font_size(&self) -> u32 {
        self.theme.font_sizes.monospace
    }

    /// Returns the current theme monospace font style.
    #[inline]
    pub fn monospace_font_style(&self) -> FontStyle {
        self.theme.font_styles.monospace
    }

    /// Returns the current theme fixed-width font.
    #[inline]
    pub fn monospace_font(&self) -> &Font {
        &self.theme.fonts.monospace
    }

    /// Returns the current theme text color.
    ///
    /// Used as the body text foreground color.
    #[inline]
    pub fn text_color(&self) -> Color {
        self.theme.colors.text
    }

    /// Returns the current theme background color.
    ///
    /// Used as the window/frame background color to clear the screen each frame.
    #[inline]
    pub fn background_color(&self) -> Color {
        self.theme.colors.background
    }

    /// Returns the current theme primary color.
    ///
    /// Used as the primary brand color for links, buttons, etc.
    #[inline]
    pub fn primary_color(&self) -> Color {
        self.theme.colors.primary
    }

    /// Returns the current theme secondary color.
    ///
    /// Used as the secondary brand color for alternative styling.
    #[inline]
    pub fn secondary_color(&self) -> Color {
        self.theme.colors.secondary
    }

    /// Returns the current theme accent color.
    ///
    /// Used as a contrast color for emphasis.
    #[inline]
    pub fn accent_color(&self) -> Color {
        self.theme.colors.accent
    }

    /// Returns the current theme highlight color.
    ///
    /// Used as a background color for highlighting text or focusing widgets.
    #[inline]
    pub fn highlight_color(&self) -> Color {
        self.theme.colors.highlight
    }

    /// Returns the current theme muted color.
    ///
    /// Used as a faint color for backgrounds, borders, and accents that do not require high
    /// contrast with the background.
    #[inline]
    pub fn muted_color(&self) -> Color {
        self.theme.colors.muted
    }
}
