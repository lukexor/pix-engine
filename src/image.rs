use crate::color::Color;
use std::{
    borrow::Cow,
    error,
    ffi::OsStr,
    fmt,
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

const RGB_CHANNELS: usize = 3;
const RGBA_CHANNELS: usize = 4;

/// Result type for Image Errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Types of errors the Image can return in a result.
#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    PngEncodingError(png::EncodingError),
    PngDecodingError(png::DecodingError),
    InvalidFormat(PixelFormat, u32, u32),
    InvalidFile(PathBuf),
    Other(Cow<'static, str>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            IoError(err) => write!(f, "io error: {}", err),
            PngEncodingError(err) => write!(f, "png encoding error: {}", err),
            PngDecodingError(err) => write!(f, "png decoding error: {}", err),
            InvalidFormat(format, w, h) => write!(
                f,
                "invalid pixel format {:?} for image ({}, {})",
                &format, w, h
            ),
            InvalidFile(path) => write!(f, "invalid file: {}", path.display()),
            Other(desc) => write!(f, "{}", &desc),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => Some(err),
            PngEncodingError(err) => Some(err),
            PngDecodingError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<png::DecodingError> for Error {
    fn from(err: png::DecodingError) -> Self {
        Self::PngDecodingError(err)
    }
}
impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Self {
        Self::PngEncodingError(err)
    }
}

/// Determines the way images are drawn by changing how the parameters given to
/// `State::draw_image()` are interpreted. The default is Corner.
///
/// Corner: Uses x and y as the upper-left corner of the image.
/// Center: Uses x and y as the center of the image.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImageMode {
    Corner,
    Center,
}

/// Represents the data format for an Image as either RGB or RGBA.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb,
    Rgba,
}

/// Represents an image with a width, height and list of pixel data.
///
/// Supported formats: RGB and RGBA.
#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    channels: usize,
    pixel_format: PixelFormat,
    data: Vec<u8>,
}

impl Image {
    /// Creates a new RGBA image with given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self::rgba(width, height)
    }

    /// Creates a new RGB image with given dimensions.
    pub fn rgb(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            channels: RGB_CHANNELS,
            pixel_format: PixelFormat::Rgb,
            data: vec![0; RGB_CHANNELS * (width * height) as usize],
        }
    }

    /// Creates a new RGBA image with given dimensions.
    pub fn rgba(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            channels: RGBA_CHANNELS,
            pixel_format: PixelFormat::Rgba,
            data: vec![0; RGBA_CHANNELS * (width * height) as usize],
        }
    }

    /// Creates a new image from a slice of bytes.
    pub fn from_bytes(
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        bytes: &[u8],
    ) -> Result<Self> {
        let channels = match pixel_format {
            PixelFormat::Rgb => RGB_CHANNELS,
            PixelFormat::Rgba => RGBA_CHANNELS,
        };
        if bytes.len() != channels * (width * height) as usize {
            Err(Error::InvalidFormat(pixel_format, width, height))
        } else {
            Ok(Self {
                width,
                height,
                channels,
                pixel_format,
                data: bytes.to_vec(),
            })
        }
    }

    /// Gets the pixel color at the given (x, y) coords.
    pub fn pixel(&self, x: u32, y: u32) -> Color {
        let idx = self.channels * (y * self.width + x) as usize;
        assert!(
            idx < self.data.len(),
            "pixel (x, y) within bounds ({}, {})",
            self.width,
            self.height
        );
        self.data[idx..self.channels].into()
    }

    /// Sets the pixel color at the given (x, y) coords.
    pub fn set_pixel<C: Into<Color>>(&mut self, x: u32, y: u32, color: C) {
        let idx = self.channels * (y * self.width + x) as usize;
        assert!(
            idx < self.data.len(),
            "pixel (x, y) within bounds ({}, {})",
            self.width,
            self.height
        );
        self.data[idx..self.channels].copy_from_slice(&color.into().as_list()[0..self.channels]);
    }

    /// PixelFormat of the image.
    pub fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }

    /// Width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a reference to the pixels within the image.
    pub fn bytes(&self) -> &Vec<u8> {
        &self.data
    }

    /// Returns a mutable reference to the pixels within the image.
    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Create a new image from a PNG file.
    /// Only 8-bit RGB and RGBA formats are supported currently.
    pub fn from_file<P: AsRef<Path>>(path: &P) -> Result<Self> {
        let path = path.as_ref();
        if path.extension() != Some(OsStr::new("png")) {
            return Err(Error::InvalidFile(path.into()));
        }

        let png_file = BufReader::new(std::fs::File::open(&path)?);
        let png = png::Decoder::new(png_file);
        let (info, mut reader) = png.read_info()?;

        assert_eq!(
            info.bit_depth,
            png::BitDepth::Eight,
            "Only 8-bit formats supported right now."
        );

        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data).unwrap();

        Image::from_bytes(info.width, info.height, info.color_type.into(), &data)
    }

    /// Saves a image out to a png file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: &P) -> Result<()> {
        let path = path.as_ref();
        let png_file = BufWriter::new(std::fs::File::create(&path)?);
        let mut png = png::Encoder::new(png_file, self.width, self.height);
        png.set_color(self.pixel_format.into());
        let mut writer = png.write_header()?;
        writer.write_image_data(self.bytes())?;
        Ok(())
    }
}

impl From<png::ColorType> for PixelFormat {
    fn from(color_type: png::ColorType) -> Self {
        use png::ColorType::*;
        match color_type {
            RGB => Self::Rgb,
            RGBA => Self::Rgba,
            _ => panic!("Only RGB and RGBA formats are supported right now."),
        }
    }
}

impl From<PixelFormat> for png::ColorType {
    fn from(format: PixelFormat) -> Self {
        use PixelFormat::*;
        match format {
            Rgb => Self::RGB,
            Rgba => Self::RGBA,
        }
    }
}

impl Default for ImageMode {
    fn default() -> Self {
        Self::Corner
    }
}
