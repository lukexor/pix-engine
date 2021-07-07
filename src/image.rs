//! [Image] and [PixelFormat] functions.

use crate::prelude::*;
use std::{
    borrow::Cow,
    error,
    ffi::{OsStr, OsString},
    fmt,
    fs::File,
    io::{self, BufReader, BufWriter},
    path::Path,
    result,
};

/// The result type for [Image] operations.
pub type Result<T> = result::Result<T, Error>;

/// The error type for [Image] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Invalid file type.
    InvalidFileType(Option<OsString>),
    /// Invalid color type.
    InvalidColorType(png::ColorType),
    /// Invalid bit depth.
    InvalidBitDepth(png::BitDepth),
    /// I/O errors.
    IoError(io::Error),
    /// [png] decoding errors.
    DecodingError(png::DecodingError),
    /// [png] encoding errors.
    EncodingError(png::EncodingError),
    /// Unknown error.
    Other(Cow<'static, str>),
}

/// Format for interpreting bytes when using textures.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// 8-bit Indexed color
    Indexed,
    /// 8-bit Gray
    Grayscale,
    /// u-bit Gray with Alpha
    GrayscaleAlpha,
    /// 8-bit Red, Green, Blue
    Rgb,
    /// 8-bit Red, Green, Blue, Alpha
    Rgba,
}

impl PixelFormat {
    /// Returns the number of channels associated with the format.
    pub fn channels(&self) -> usize {
        use PixelFormat::*;
        match self {
            Indexed | Grayscale => 1,
            GrayscaleAlpha => 2,
            Rgb => 3,
            Rgba => 4,
        }
    }
}

impl From<png::ColorType> for PixelFormat {
    fn from(color_type: png::ColorType) -> Self {
        use png::ColorType::*;
        match color_type {
            Indexed => Self::Indexed,
            Grayscale => Self::Grayscale,
            GrayscaleAlpha => Self::GrayscaleAlpha,
            RGB => Self::Rgb,
            RGBA => Self::Rgba,
        }
    }
}

impl Default for PixelFormat {
    fn default() -> Self {
        Self::Rgba
    }
}

/// An `Image` representing a buffer of pixel color values.
#[non_exhaustive]
#[derive(Debug, Default, Clone)]
pub struct Image {
    /// `Image` width.
    width: u32,
    /// `Image` height.
    height: u32,
    /// Raw pixel data.
    data: Vec<u8>,
    /// Pixel Format.
    format: PixelFormat,
    /// Texture Identifier.
    texture_id: usize,
}

impl Image {
    /// `Image` width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// `Image` height.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the `Image` dimensions as `(width, height)`.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the `Image` pixel data as a [u8] [slice].
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the `Image` pixel data as a mutable [u8] [slice].
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Update the `Image` with a  [u8] [slice] representing RGB/A values.
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        self.data.clone_from_slice(bytes);
    }

    /// Returns the `Image` pixel format.
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    /// Save the `Image` to a [png] file.
    pub fn save<P>(&self, path: P) -> PixResult<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let png_file = BufWriter::new(std::fs::File::create(&path)?);
        let mut png = png::Encoder::new(png_file, self.width, self.height);
        png.set_color(png::ColorType::RGBA);
        let mut writer = png.write_header()?;
        Ok(writer.write_image_data(self.bytes())?)
    }

    /// Returns the `Image` [TextureId].
    pub(crate) fn texture_id(&self) -> TextureId {
        self.texture_id
    }
}

impl PixState {
    /// Constructs an empty RGBA `Image` with given `width` and `height`.
    pub fn create_image(&mut self, width: u32, height: u32) -> PixResult<Image> {
        self.create_rgba_image(width, height)
    }

    /// Constructs an empty RGBA `Image` with given `width` and `height`.
    pub fn create_rgba_image(&mut self, width: u32, height: u32) -> PixResult<Image> {
        let format = PixelFormat::Rgba;
        Ok(Image {
            width,
            height,
            data: vec![0x00; format.channels() * (width * height) as usize],
            format,
            texture_id: self.create_texture(width, height, format)?,
        })
    }

    /// Constructs an empty RGB `Image` with given `width` and `height`.
    pub fn create_rgb_image(&mut self, width: u32, height: u32) -> PixResult<Image> {
        let format = PixelFormat::Rgb;
        Ok(Image {
            width,
            height,
            data: vec![0x00; format.channels() * (width * height) as usize],
            format,
            texture_id: self.create_texture(width, height, format)?,
        })
    }

    /// Constructs an `Image` from a [u8] [slice] representing RGB/A values.
    pub fn create_image_from_bytes(
        &mut self,
        width: u32,
        height: u32,
        bytes: &[u8],
        format: PixelFormat,
    ) -> PixResult<Image> {
        Ok(Image {
            width,
            height,
            data: bytes.to_vec(),
            format,
            texture_id: self.create_texture(width, height, format)?,
        })
    }

    /// Constructs an `Image` from a [png] file.
    pub fn create_image_from_file<P>(&mut self, path: P) -> PixResult<Image>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let ext = path.extension();
        if ext != Some(OsStr::new("png")) {
            return Err(Error::InvalidFileType(ext.map(|e| e.to_os_string())).into());
        }

        let png_file = BufReader::new(File::open(&path)?);
        let png = png::Decoder::new(png_file);
        let (info, mut reader) = png.read_info()?;

        let mut data = vec![0x00; info.buffer_size()];
        reader.next_frame(&mut data)?;
        let format = info.color_type.into();
        self.create_image_from_bytes(info.width, info.height, &data, format)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidFileType(ext) => write!(f, "invalid file type: {:?}", ext),
            InvalidColorType(color_type) => write!(f, "invalid color type: {:?}", color_type),
            InvalidBitDepth(depth) => write!(f, "invalid bit depth: {:?}", depth),
            Other(err) => write!(f, "renderer error: {}", err),
            err => err.fmt(f),
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

impl From<Error> for PixError {
    fn from(err: Error) -> Self {
        Self::ImageError(err)
    }
}

impl From<io::Error> for PixError {
    fn from(err: io::Error) -> Self {
        Self::ImageError(Error::IoError(err))
    }
}

impl From<png::DecodingError> for PixError {
    fn from(err: png::DecodingError) -> Self {
        Self::ImageError(Error::DecodingError(err))
    }
}

impl From<png::EncodingError> for PixError {
    fn from(err: png::EncodingError) -> Self {
        Self::ImageError(Error::EncodingError(err))
    }
}
