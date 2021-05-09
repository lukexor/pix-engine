//! Image related operations and functionality.

use crate::common::PixError;
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
pub type ImageResult<T> = result::Result<T, ImageError>;

/// Types of errors `Image` can return in a `Result`.
#[non_exhaustive]
#[derive(Debug)]
pub enum ImageError {
    /// Invalid file type.
    InvalidFileType(Option<OsString>),
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
    // TODO: tint, flip, rgb
}

impl Image {
    /// Create a blank `Image` with a given width/height.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0x00; (4 * width * height) as usize],
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

    /// Create a new `Image` from an array of u8 bytes representing RGBA values.
    pub fn from_bytes(width: u32, height: u32, bytes: &[u8]) -> Self {
        Self {
            width,
            height,
            data: bytes.to_vec(),
        }
    }

    /// Create a new `Image` by loading it from a `png` file.
    pub fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let ext = path.extension();
        if ext != Some(OsStr::new("png")) {
            return Err(ImageError::InvalidFileType(ext.map(|e| e.to_os_string())));
        }

        let png_file = BufReader::new(File::open(&path)?);
        let png = png::Decoder::new(png_file);
        let (info, mut reader) = png.read_info()?;

        if info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::InvalidBitDepth(info.bit_depth));
        }

        let mut data = vec![0x00; info.buffer_size()];
        reader.next_frame(&mut data).unwrap();
        Ok(Self {
            width: info.width,
            height: info.height,
            data,
        })
    }

    /// Save an `Image` to a `png` file.
    pub fn save<P>(&self, _path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        todo!("save image");
    }
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ImageError::*;
        match self {
            InvalidFileType(ext) => write!(f, "Invalid file type: {:?}", ext),
            InvalidBitDepth(depth) => write!(f, "Invalid bit depth: {:?}", depth),
            IoError(err) => err.fmt(f),
            DecodingError(err) => err.fmt(f),
            Other(err) => write!(f, "Renderer Error: {}", err),
        }
    }
}

impl error::Error for ImageError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<ImageError> for PixError {
    fn from(err: ImageError) -> Self {
        Self::ImageError(err)
    }
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<png::DecodingError> for ImageError {
    fn from(err: png::DecodingError) -> Self {
        Self::DecodingError(err)
    }
}
