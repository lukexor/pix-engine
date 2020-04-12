const RED_SHIFT: u32 = 24;
const GREEN_SHIFT: u32 = 16;
const BLUE_SHIFT: u32 = 8;
const ALPHA_SHIFT: u32 = 0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorType {
    Rgb,
    Rgba,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_u32(color: u32) -> Self {
        Self {
            r: (color >> RED_SHIFT) as u8,
            g: (color >> GREEN_SHIFT) as u8,
            b: (color >> BLUE_SHIFT) as u8,
            a: (color >> ALPHA_SHIFT) as u8,
        }
    }

    pub fn r(self) -> u8 {
        self.r
    }
    pub fn set_r(mut self, r: u8) {
        self.r = r;
    }

    pub fn g(self) -> u8 {
        self.g
    }
    pub fn set_g(mut self, g: u8) {
        self.g = g;
    }

    pub fn b(self) -> u8 {
        self.b
    }
    pub fn set_b(mut self, b: u8) {
        self.b = b;
    }

    pub fn a(self) -> u8 {
        self.a
    }
    pub fn set_a(mut self, a: u8) {
        self.a = a;
    }

    /// Pixel Constants

    // White/Black/Blank
    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }
    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }
    pub fn transparent() -> Self {
        Self::rgba(0, 0, 0, 0)
    }

    // Gray
    pub fn gray() -> Self {
        Self::rgb(192, 192, 192)
    }
    pub fn dark_gray() -> Self {
        Self::rgb(128, 128, 128)
    }
    pub fn very_dark_gray() -> Self {
        Self::rgb(64, 64, 64)
    }

    // Red
    pub fn red() -> Self {
        Self::rgb(255, 0, 0)
    }
    pub fn dark_red() -> Self {
        Self::rgb(128, 0, 0)
    }
    pub fn very_dark_red() -> Self {
        Self::rgb(64, 0, 0)
    }

    // Orange
    pub fn orange() -> Self {
        Self::rgb(255, 128, 0)
    }

    // Yellow
    pub fn yellow() -> Self {
        Self::rgb(255, 255, 0)
    }
    pub fn dark_yellow() -> Self {
        Self::rgb(128, 128, 0)
    }
    pub fn very_dark_yellow() -> Self {
        Self::rgb(64, 64, 0)
    }

    // Green
    pub fn green() -> Self {
        Self::rgb(0, 255, 0)
    }
    pub fn dark_green() -> Self {
        Self::rgb(0, 128, 0)
    }
    pub fn very_dark_green() -> Self {
        Self::rgb(0, 64, 0)
    }

    // Cyan
    pub fn cyan() -> Self {
        Self::rgb(0, 255, 255)
    }
    pub fn dark_cyan() -> Self {
        Self::rgb(0, 128, 128)
    }
    pub fn very_dark_cyan() -> Self {
        Self::rgb(0, 64, 64)
    }

    // Blue
    pub fn blue() -> Self {
        Self::rgb(0, 0, 255)
    }
    pub fn dark_blue() -> Self {
        Self::rgb(0, 0, 128)
    }
    pub fn very_dark_blue() -> Self {
        Self::rgb(0, 0, 64)
    }

    // Magenta
    pub fn magenta() -> Self {
        Self::rgb(255, 0, 255)
    }
    pub fn dark_magenta() -> Self {
        Self::rgb(128, 0, 128)
    }
    pub fn very_dark_magenta() -> Self {
        Self::rgb(64, 0, 64)
    }
}
