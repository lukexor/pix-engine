//! Image related operations and functionality.

use crate::common;
use std::{
    borrow::Cow,
    error,
    ffi::{OsStr, OsString},
    fmt,
    fs::File,
    io::{self, BufReader},
    path::Path,
    result,
};

/// `Image` Result
pub type Result<T> = result::Result<T, Error>;

/// Types of errors `Image` can return in a `Result`.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Invalid file type.
    InvalidFileType(Option<OsString>),
    /// Invalid color type.
    InvalidColorType(png::ColorType),
    /// Invalid bit depth.
    InvalidBitDepth(png::BitDepth),
    /// IO specific errors.
    IoError(io::Error),
    /// Decoding specific errors.
    DecodingError(png::DecodingError),
    /// Any other unknown error as a string.
    Other(Cow<'static, str>),
}

/// Represents a buffer of pixel color values.
#[derive(Debug, Default, Clone)]
pub struct Image {
    /// Width of the image
    width: u32,
    /// Height of the image
    height: u32,
    /// RGB values
    data: Vec<u8>,
    channels: usize, // TODO: pixel_format: PixelFormat
                     // TODO: tint, flip
}

// TODO: Texture { id, image, quad, uv, uv_scale, w }
// texture!()

impl Image {
    // TODO:
    // rgb(w: u32, h: u32), rgba(w, h)
    // from_bytes(w: u32, h: u32, pixel_format: PixelFormat, bytes: &[u8])
    // from_pixels(w: u32, h: u32, pixels: &[Color])
    // pixel(x i32, y: i32) -> Color
    // set_pixel(x i32, y: i32, pixel: Color)
    // pixel_format() -> PixelFormat
    // dimensions() -> (u32, u32)
    // pixels() -> &[Color]
    // pixels_mut() -> & mut[Color]
    // get_index(x: i32, y: i32) -> usize
    // filter(filter: ImageFilter)
    // sub_image(x: i32, y: i32, w: u32, h: u32) -> Image
    // set_sub_image(x: i32, y: i32, image: &Image)
    // resize(w: u32, h: u32)
    // blend(image: &Image, mode: BlendMode)
    // mask(image: &Image)
    // Image.filtered(image: Image, filter: ImageFilter) -> Image
    // Image.resized(image: Image, w: u32, h: u32) -> Image
    // Image.blended(image: &Image, mode: BlendMode) -> Image
    // Image.mask(image: &Image) -> Image
    // image!(w, h)

    /// Create a blank RGBA `Image` with a given width/height.
    pub fn new(width: u32, height: u32) -> Self {
        let channels = 4;
        Self {
            width,
            height,
            data: vec![0x00; channels * (width * height) as usize],
            channels,
        }
    }

    /// Create a blank RGB `Image` with a given width/height.
    pub fn rgb(width: u32, height: u32) -> Self {
        let channels = 3;
        Self {
            width,
            height,
            data: vec![0x00; channels * (width * height) as usize],
            channels,
        }
    }

    /// The image width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The image height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The image data as a u8 slice.
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    /// The image data as a mutable u8 slice.
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// The number of color channels.
    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Create a new `Image` from an array of u8 bytes representing RGB/A values.
    pub fn from_bytes(width: u32, height: u32, bytes: &[u8], channels: usize) -> Self {
        Self {
            width,
            height,
            data: bytes.to_vec(),
            channels,
        }
    }

    /// Update `Image` with an array of u8 bytes representing RGB/A values.
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        self.data.clone_from_slice(bytes);
    }

    /// Create a new `Image` by loading it from a `png` file.
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let ext = path.extension();
        if ext != Some(OsStr::new("png")) {
            return Err(Error::InvalidFileType(ext.map(|e| e.to_os_string())));
        }

        let png_file = BufReader::new(File::open(&path)?);
        let png = png::Decoder::new(png_file);
        let (info, mut reader) = png.read_info()?;

        let channels = match info.color_type {
            png::ColorType::Grayscale => 1,
            png::ColorType::GrayscaleAlpha => 2,
            png::ColorType::RGB => 3,
            png::ColorType::RGBA => 4,
            _ => return Err(Error::InvalidColorType(info.color_type)),
        };

        if info.bit_depth != png::BitDepth::Eight {
            return Err(Error::InvalidBitDepth(info.bit_depth));
        }

        let mut data = vec![0x00; info.buffer_size()];
        reader.next_frame(&mut data).unwrap();
        Ok(Self {
            width: info.width,
            height: info.height,
            data,
            channels,
        })
    }

    /// Save an `Image` to a `png` file.
    pub fn save<P>(&self, _path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        todo!("save image");
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidFileType(ext) => write!(f, "Invalid file type: {:?}", ext),
            InvalidColorType(color_type) => write!(f, "Invalid color type: {:?}", color_type),
            InvalidBitDepth(depth) => write!(f, "Invalid bit depth: {:?}", depth),
            IoError(err) => err.fmt(f),
            DecodingError(err) => err.fmt(f),
            Other(err) => write!(f, "Renderer Error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::*;
        match self {
            IoError(err) => err.source(),
            DecodingError(err) => err.source(),
            _ => None,
        }
    }
}

impl From<Error> for common::Error {
    fn from(err: Error) -> Self {
        Self::ImageError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<png::DecodingError> for Error {
    fn from(err: png::DecodingError) -> Self {
        Self::DecodingError(err)
    }
}
