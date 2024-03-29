//! [SVG 1.0 Color Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
//!
//! Provides a set a default named colors matching the `SVG 1.0 Color Keywords` and are included in
//! the `prelude`.
//!
//! # Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//! let c: Color = Color::ALICE_BLUE;
//! assert_eq!(c.as_hex(), 0xF0F8FF);
//!
//! let c: Color = Color::PALE_TURQUOISE;
//! assert_eq!(c.as_hex(), 0xAFEEEE);
//! ```

use crate::prelude::Color;

#[allow(missing_docs)]
impl Color {
    pub const ALICE_BLUE: Self = Self::rgb(0xF0, 0xF8, 0xFF);
    pub const ANTIQUE_WHITE: Self = Self::rgb(0xFA, 0xEB, 0xD7);
    pub const AQUA: Self = Self::rgb(0x0, 0xFF, 0xFF);
    pub const AQUA_MARINE: Self = Self::rgb(0x7F, 0xFF, 0xD4);
    pub const AZURE: Self = Self::rgb(0xF0, 0xFF, 0xFF);
    pub const BEIGE: Self = Self::rgb(0xF5, 0xF5, 0xDC);
    pub const BISQUE: Self = Self::rgb(0xFF, 0xE4, 0xC4);
    pub const BLACK: Self = Self::rgb(0x0, 0x0, 0x0);
    pub const BLANCHE_DALMOND: Self = Self::rgb(0xFF, 0xEB, 0xCD);
    pub const BLUE: Self = Self::rgb(0x0, 0x0, 0xFF);
    pub const BLUE_VIOLET: Self = Self::rgb(0x8A, 0x2B, 0xE2);
    pub const BROWN: Self = Self::rgb(0xA5, 0x2A, 0x2A);
    pub const BURLY_WOOD: Self = Self::rgb(0xDE, 0xB8, 0x87);
    pub const CADET_BLUE: Self = Self::rgb(0x5F, 0x9E, 0xA0);
    pub const CHARTREUSE: Self = Self::rgb(0x7F, 0xFF, 0x0);
    pub const CHOCOLATE: Self = Self::rgb(0xD2, 0x69, 0x1E);
    pub const CORAL: Self = Self::rgb(0xFF, 0x7F, 0x50);
    pub const CORNFLOWER_BLUE: Self = Self::rgb(0x64, 0x95, 0xED);
    pub const CORN_SILK: Self = Self::rgb(0xFF, 0xF8, 0xDC);
    pub const CRIMSON: Self = Self::rgb(0xDC, 0x14, 0x3C);
    pub const CYAN: Self = Self::rgb(0x0, 0xFF, 0xFF);
    pub const DARK_BLUE: Self = Self::rgb(0x0, 0x0, 0x8B);
    pub const DARK_CYAN: Self = Self::rgb(0x0, 0x8B, 0x8B);
    pub const DARK_GOLDENROD: Self = Self::rgb(0xB8, 0x86, 0xB);
    pub const DARK_GRAY: Self = Self::rgb(0xA9, 0xA9, 0xA9);
    pub const DARK_GREEN: Self = Self::rgb(0x0, 0x64, 0x0);
    pub const DARK_GREY: Self = Self::rgb(0xA9, 0xA9, 0xA9);
    pub const DARK_KHAKI: Self = Self::rgb(0xBD, 0xB7, 0x6B);
    pub const DARK_MAGENTA: Self = Self::rgb(0x8B, 0x0, 0x8B);
    pub const DARK_OLIVE_GREEN: Self = Self::rgb(0x55, 0x6B, 0x2F);
    pub const DARK_ORANGE: Self = Self::rgb(0xFF, 0x8C, 0x0);
    pub const DARK_ORCHID: Self = Self::rgb(0x99, 0x32, 0xCC);
    pub const DARK_RED: Self = Self::rgb(0x8B, 0x0, 0x0);
    pub const DARK_SALMON: Self = Self::rgb(0xE9, 0x96, 0x7A);
    pub const DARK_SEA_GREEN: Self = Self::rgb(0x8F, 0xBC, 0x8F);
    pub const DARK_SLATE_BLUE: Self = Self::rgb(0x48, 0x3D, 0x8B);
    pub const DARK_SLATE_GRAY: Self = Self::rgb(0x2F, 0x4F, 0x4F);
    pub const DARK_SLATE_GREY: Self = Self::rgb(0x2F, 0x4F, 0x4F);
    pub const DARK_TURQUOISE: Self = Self::rgb(0x0, 0xCE, 0xD1);
    pub const DARK_VIOLET: Self = Self::rgb(0x94, 0x0, 0xD3);
    pub const DEEP_PINK: Self = Self::rgb(0xFF, 0x14, 0x93);
    pub const DEEP_SKY_BLUE: Self = Self::rgb(0x0, 0xBF, 0xFF);
    pub const DIM_GRAY: Self = Self::rgb(0x69, 0x69, 0x69);
    pub const DIM_GREY: Self = Self::rgb(0x69, 0x69, 0x69);
    pub const DODGER_BLUE: Self = Self::rgb(0x1E, 0x90, 0xFF);
    pub const FIRE_BRICK: Self = Self::rgb(0xB2, 0x22, 0x22);
    pub const FLORAL_WHITE: Self = Self::rgb(0xFF, 0xFA, 0xF0);
    pub const FOREST_GREEN: Self = Self::rgb(0x22, 0x8B, 0x22);
    pub const FUCHSIA: Self = Self::rgb(0xFF, 0x0, 0xFF);
    pub const GAINSBORO: Self = Self::rgb(0xDC, 0xDC, 0xDC);
    pub const GHOST_WHITE: Self = Self::rgb(0xF8, 0xF8, 0xFF);
    pub const GOLD: Self = Self::rgb(0xFF, 0xD7, 0x0);
    pub const GOLDENROD: Self = Self::rgb(0xDA, 0xA5, 0x20);
    pub const GRAY: Self = Self::rgb(0x80, 0x80, 0x80);
    pub const GREEN: Self = Self::rgb(0x0, 0x80, 0x0);
    pub const GREEN_YELLOW: Self = Self::rgb(0xAD, 0xFF, 0x2F);
    pub const GREY: Self = Self::rgb(0x80, 0x80, 0x80);
    pub const HONEYDEW: Self = Self::rgb(0xF0, 0xFF, 0xF0);
    pub const HOTOINK: Self = Self::rgb(0xFF, 0x69, 0xB4);
    pub const INDIAN_RED: Self = Self::rgb(0xCD, 0x5C, 0x5C);
    pub const INDIGO: Self = Self::rgb(0x4B, 0x0, 0x82);
    pub const IVORY: Self = Self::rgb(0xFF, 0xFF, 0xF0);
    pub const KHAKI: Self = Self::rgb(0xF0, 0xE6, 0x8C);
    pub const LAVENDER: Self = Self::rgb(0xE6, 0xE6, 0xFA);
    pub const LAVENDER_BLUSH: Self = Self::rgb(0xFF, 0xF0, 0xF5);
    pub const LAWN_GREEN: Self = Self::rgb(0x7C, 0xFC, 0x0);
    pub const LEMON_CHIFFON: Self = Self::rgb(0xFF, 0xFA, 0xCD);
    pub const LIGHT_BLUE: Self = Self::rgb(0xAD, 0xD8, 0xE6);
    pub const LIGHT_CORAL: Self = Self::rgb(0xF0, 0x80, 0x80);
    pub const LIGHT_CYAN: Self = Self::rgb(0xE0, 0xFF, 0xFF);
    pub const LIGHT_GOLDENROD_YELLOW: Self = Self::rgb(0xFA, 0xFA, 0xD2);
    pub const LIGHT_GRAY: Self = Self::rgb(0xD3, 0xD3, 0xD3);
    pub const LIGHT_GREEN: Self = Self::rgb(0x90, 0xEE, 0x90);
    pub const LIGHT_GREY: Self = Self::rgb(0xD3, 0xD3, 0xD3);
    pub const LIGHT_PINK: Self = Self::rgb(0xFF, 0xB6, 0xC1);
    pub const LIGHT_SALMON: Self = Self::rgb(0xFF, 0xA0, 0x7A);
    pub const LIGHT_SEA_GREEN: Self = Self::rgb(0x20, 0xB2, 0xAA);
    pub const LIGHT_SKY_BLUE: Self = Self::rgb(0x87, 0xCE, 0xFA);
    pub const LIGHT_SLATE_GRAY: Self = Self::rgb(0x77, 0x88, 0x99);
    pub const LIGHT_SLATE_GREY: Self = Self::rgb(0x77, 0x88, 0x99);
    pub const LIGHT_STEEL_BLUE: Self = Self::rgb(0xB0, 0xC4, 0xDE);
    pub const LIGHT_YELLOW: Self = Self::rgb(0xFF, 0xFF, 0xE0);
    pub const LIME: Self = Self::rgb(0x0, 0xFF, 0x0);
    pub const LIME_GREEN: Self = Self::rgb(0x32, 0xCD, 0x32);
    pub const LINEN: Self = Self::rgb(0xFA, 0xF0, 0xE6);
    pub const MAGENTA: Self = Self::rgb(0xFF, 0x0, 0xFF);
    pub const MAROON: Self = Self::rgb(0x80, 0x0, 0x0);
    pub const MEDIUMAQUA_MARINE: Self = Self::rgb(0x66, 0xCD, 0xAA);
    pub const MEDIUM_BLUE: Self = Self::rgb(0x0, 0x0, 0xCD);
    pub const MEDIUM_ORCHID: Self = Self::rgb(0xBA, 0x55, 0xD3);
    pub const MEDIUM_PURPLE: Self = Self::rgb(0x93, 0x70, 0xDB);
    pub const MEDIUM_SEA_GREEN: Self = Self::rgb(0x3C, 0xB3, 0x71);
    pub const MEDIUM_SLATE_BLUE: Self = Self::rgb(0x7B, 0x68, 0xEE);
    pub const MEDIUM_SPRING_GREEN: Self = Self::rgb(0x0, 0xFA, 0x9A);
    pub const MEDIUM_TURQUOISE: Self = Self::rgb(0x48, 0xD1, 0xCC);
    pub const MEDIUM_VIOLET_RED: Self = Self::rgb(0xC7, 0x15, 0x85);
    pub const MIDNIGHT_BLUE: Self = Self::rgb(0x19, 0x19, 0x70);
    pub const MINT_CREAM: Self = Self::rgb(0xF5, 0xFF, 0xFA);
    pub const MISTY_ROSE: Self = Self::rgb(0xFF, 0xE4, 0xE1);
    pub const MOCCASIN: Self = Self::rgb(0xFF, 0xE4, 0xB5);
    pub const NAVAJO_WHITE: Self = Self::rgb(0xFF, 0xDE, 0xAD);
    pub const NAVY: Self = Self::rgb(0x0, 0x0, 0x80);
    pub const OLD_LACE: Self = Self::rgb(0xFD, 0xF5, 0xE6);
    pub const OLIVE: Self = Self::rgb(0x80, 0x80, 0x0);
    pub const OLIVE_DRAB: Self = Self::rgb(0x6B, 0x8E, 0x23);
    pub const ORANGE: Self = Self::rgb(0xFF, 0xA5, 0x0);
    pub const ORANGE_RED: Self = Self::rgb(0xFF, 0x45, 0x0);
    pub const ORCHID: Self = Self::rgb(0xDA, 0x70, 0xD6);
    pub const PALE_GOLDENROD: Self = Self::rgb(0xEE, 0xE8, 0xAA);
    pub const PALE_GREEN: Self = Self::rgb(0x98, 0xFB, 0x98);
    pub const PALE_TURQUOISE: Self = Self::rgb(0xAF, 0xEE, 0xEE);
    pub const PALE_VIOLET_RED: Self = Self::rgb(0xDB, 0x70, 0x93);
    pub const PAPAYA_WHIP: Self = Self::rgb(0xFF, 0xEF, 0xD5);
    pub const PEACH_PUFF: Self = Self::rgb(0xFF, 0xDA, 0xB9);
    pub const PERU: Self = Self::rgb(0xCD, 0x85, 0x3F);
    pub const PINK: Self = Self::rgb(0xFF, 0xC0, 0xCB);
    pub const PLUM: Self = Self::rgb(0xDD, 0xA0, 0xDD);
    pub const POWDER_BLUE: Self = Self::rgb(0xB0, 0xE0, 0xE6);
    pub const PURPLE: Self = Self::rgb(0x80, 0x0, 0x80);
    pub const REBECCA_PURPLE: Self = Self::rgb(0x66, 0x33, 0x99);
    pub const RED: Self = Self::rgb(0xFF, 0x0, 0x0);
    pub const ROSY_BROWN: Self = Self::rgb(0xBC, 0x8F, 0x8F);
    pub const ROYAL_BLUE: Self = Self::rgb(0x41, 0x69, 0xE1);
    pub const SADDLE_BROWN: Self = Self::rgb(0x8B, 0x45, 0x13);
    pub const SALMON: Self = Self::rgb(0xFA, 0x80, 0x72);
    pub const SANDY_BROWN: Self = Self::rgb(0xF4, 0xA4, 0x60);
    pub const SEA_GREEN: Self = Self::rgb(0x2E, 0x8B, 0x57);
    pub const SEA_SHELL: Self = Self::rgb(0xFF, 0xF5, 0xEE);
    pub const SIENNA: Self = Self::rgb(0xA0, 0x52, 0x2D);
    pub const SILVER: Self = Self::rgb(0xC0, 0xC0, 0xC0);
    pub const SKY_BLUE: Self = Self::rgb(0x87, 0xCE, 0xEB);
    pub const SLATE_BLUE: Self = Self::rgb(0x6A, 0x5A, 0xCD);
    pub const SLATE_GRAY: Self = Self::rgb(0x70, 0x80, 0x90);
    pub const SLATE_GREY: Self = Self::rgb(0x70, 0x80, 0x90);
    pub const SNOW: Self = Self::rgb(0xFF, 0xFA, 0xFA);
    pub const SPRING_GREEN: Self = Self::rgb(0x0, 0xFF, 0x7F);
    pub const STEEL_BLUE: Self = Self::rgb(0x46, 0x82, 0xB4);
    pub const TAN: Self = Self::rgb(0xD2, 0xB4, 0x8C);
    pub const TEAL: Self = Self::rgb(0x0, 0x80, 0x80);
    pub const THISTLE: Self = Self::rgb(0xD8, 0xBF, 0xD8);
    pub const TOMATO: Self = Self::rgb(0xFF, 0x63, 0x47);
    pub const TRANSPARENT: Self = Self::rgba(0x0, 0x0, 0x0, 0x0);
    pub const TURQUOISE: Self = Self::rgb(0x40, 0xE0, 0xD0);
    pub const VIOLET: Self = Self::rgb(0xEE, 0x82, 0xEE);
    pub const WHEAT: Self = Self::rgb(0xF5, 0xDE, 0xB3);
    pub const WHITE: Self = Self::rgb(0xFF, 0xFF, 0xFF);
    pub const WHITE_SMOKE: Self = Self::rgb(0xF5, 0xF5, 0xF5);
    pub const YELLOW: Self = Self::rgb(0xFF, 0xFF, 0x0);
    pub const YELLOW_GREEN: Self = Self::rgb(0x9A, 0xCD, 0x32);
}
