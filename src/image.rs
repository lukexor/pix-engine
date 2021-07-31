//! [Image] and [PixelFormat] functions.

use crate::{prelude::*, renderer::Rendering};
use std::{
    borrow::Cow,
    cell::Cell,
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

/// Format for interpreting bytes when using textures.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// 8-bit Red, Green, and Blue
    Rgb,
    /// 8-bit Red, Green, Blue, and Alpha
    Rgba,
}

impl PixelFormat {
    /// Returns the number of channels associated with the format.
    #[inline]
    pub fn channels(&self) -> usize {
        use PixelFormat::*;
        match self {
            Rgb => 3,
            Rgba => 4,
        }
    }
}

impl From<png::ColorType> for PixelFormat {
    fn from(color_type: png::ColorType) -> Self {
        use png::ColorType::*;
        match color_type {
            RGB => Self::Rgb,
            RGBA => Self::Rgba,
            _ => unimplemented!("{:?} is not supported.", color_type),
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
#[derive(Default, Clone)]
pub struct Image {
    /// `Image` width.
    width: Primitive,
    /// `Image` height.
    height: Primitive,
    /// Raw pixel data.
    data: Vec<u8>,
    /// Pixel Format.
    format: PixelFormat,
    /// Texture Identifier.
    texture_id: Cell<Option<usize>>,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            .field("texture_id", &self.texture_id)
            .field("size", &self.data.len())
            .finish()
    }
}

impl Image {
    /// Constructs an empty RGBA `Image` with given `width` and `height`. Alias for
    /// [Image::new_rgba].
    #[inline]
    pub fn new(width: Primitive, height: Primitive) -> Self {
        Self::new_rgba(width, height)
    }

    /// Constructs an empty RGBA `Image` with given `width` and `height`.
    pub fn new_rgba(width: Primitive, height: Primitive) -> Self {
        let format = PixelFormat::Rgba;
        let data = vec![0x00; format.channels() * (width * height) as usize];
        Self::from_vec(width, height, data, format)
    }

    /// Constructs an empty RGB `Image` with given `width` and `height`.
    pub fn new_rgb(width: Primitive, height: Primitive) -> Self {
        let format = PixelFormat::Rgb;
        let data = vec![0x00; format.channels() * (width * height) as usize];
        Self::from_vec(width, height, data, format)
    }

    /// Constructs an `Image` from a [u8] [slice] representing RGB/A values.
    pub fn from_bytes(
        width: Primitive,
        height: Primitive,
        bytes: &[u8],
        format: PixelFormat,
    ) -> Self {
        Self::from_vec(width, height, bytes.to_vec(), format)
    }

    /// Constructs an `Image` from a [Vec<u8>] representing RGB/A values.
    pub fn from_vec(
        width: Primitive,
        height: Primitive,
        data: Vec<u8>,
        format: PixelFormat,
    ) -> Self {
        Self {
            width,
            height,
            data,
            format,
            texture_id: Cell::new(None),
        }
    }

    /// Constructs an `Image` from a [png] file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let ext = path.extension();
        if ext != Some(OsStr::new("png")) {
            return Err(Error::InvalidFileType(ext.map(|e| e.to_os_string())));
        }

        let png_file = BufReader::new(File::open(&path)?);
        let png = png::Decoder::new(png_file);
        let (info, mut reader) = png.read_info()?;

        if info.bit_depth != png::BitDepth::Eight {
            return Err(Error::UnsupportedBitDepth(info.bit_depth));
        } else if !matches!(info.color_type, png::ColorType::RGB | png::ColorType::RGBA) {
            return Err(Error::UnsupportedColorType(info.color_type));
        }

        let mut data = vec![0x00; info.buffer_size()];
        reader.next_frame(&mut data)?;
        let format = info.color_type.into();
        Ok(Self::from_bytes(
            info.width as i32,
            info.height as i32,
            &data,
            format,
        ))
    }

    /// `Image` width.
    #[inline]
    pub fn width(&self) -> Primitive {
        self.width
    }

    /// `Image` height.
    #[inline]
    pub fn height(&self) -> Primitive {
        self.height
    }

    /// Returns the `Image` dimensions as `(width, height)`.
    #[inline]
    pub fn dimensions(&self) -> (Primitive, Primitive) {
        (self.width, self.height)
    }

    /// Returns the `Image` pixel data as a [u8] [slice].
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the `Image` pixel data as a mutable [u8] [slice].
    #[inline]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Update the `Image` with a  [u8] [slice] representing RGB/A values.
    #[inline]
    pub fn update_bytes(&mut self, bytes: &[u8]) {
        self.data.clone_from_slice(bytes);
    }

    /// Returns the `Image` pixel format.
    #[inline]
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
        let mut png = png::Encoder::new(png_file, self.width as u32, self.height as u32);
        png.set_color(png::ColorType::RGBA);
        let mut writer = png.write_header()?;
        Ok(writer.write_image_data(self.bytes())?)
    }

    /// Returns the `Image` [TextureId].
    #[inline]
    pub(crate) fn texture_id(&self) -> Option<TextureId> {
        self.texture_id.get()
    }

    /// Set the `Image` [TextureId].
    #[inline]
    pub(crate) fn set_texture_id(&self, texture_id: TextureId) {
        self.texture_id.set(Some(texture_id));
    }
}

impl PixState {
    /// Draw an [Image] to the current canvas.
    pub fn image<P>(&mut self, position: P, img: &Image) -> PixResult<()>
    where
        P: Into<Point<Primitive>>,
    {
        let p = position.into();
        let s = &self.settings;
        let position = match s.image_mode {
            DrawMode::Corner => p,
            DrawMode::Center => point!(p.x - img.width() / 2, p.y - img.height() / 2),
        };
        Ok(self.renderer.image(&position, img, s.image_tint)?)
    }

    /// Draw a resized [Image] to the current canvas.
    pub fn image_resized<R>(&mut self, rect: R, img: &Image) -> PixResult<()>
    where
        R: Into<Rect<Primitive>>,
    {
        let rect = rect.into();
        let s = &self.settings;
        let rect = match s.image_mode {
            DrawMode::Corner => rect,
            DrawMode::Center => rect!(
                rect.x - rect.width / 2,
                rect.y - rect.height / 2,
                rect.width,
                rect.height
            ),
        };
        Ok(self.renderer.image_resized(&rect, img, s.image_tint)?)
    }
}

/// The error type for [Image] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Invalid file type.
    InvalidFileType(Option<OsString>),
    /// Invalid color type.
    UnsupportedColorType(png::ColorType),
    /// Invalid bit depth.
    UnsupportedBitDepth(png::BitDepth),
    /// I/O errors.
    IoError(io::Error),
    /// [png] decoding errors.
    DecodingError(png::DecodingError),
    /// [png] encoding errors.
    EncodingError(png::EncodingError),
    /// Unknown error.
    Other(Cow<'static, str>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidFileType(ext) => write!(f, "invalid file type: {:?}", ext),
            UnsupportedColorType(color_type) => write!(f, "invalid color type: {:?}", color_type),
            UnsupportedBitDepth(depth) => write!(f, "invalid bit depth: {:?}", depth),
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<png::DecodingError> for Error {
    fn from(err: png::DecodingError) -> Self {
        Error::DecodingError(err)
    }
}

impl From<png::EncodingError> for Error {
    fn from(err: png::EncodingError) -> Self {
        Error::EncodingError(err)
    }
}

impl From<png::EncodingError> for PixError {
    fn from(err: png::EncodingError) -> Self {
        PixError::ImageError(Error::EncodingError(err))
    }
}
