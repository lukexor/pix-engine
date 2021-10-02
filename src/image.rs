//! [Image] and [PixelFormat] functions.

use crate::{prelude::*, renderer::Rendering};
use num_traits::AsPrimitive;
use png::{BitDepth, ColorType, Decoder};
use std::{
    borrow::Cow,
    error,
    ffi::{OsStr, OsString},
    fmt,
    fs::File,
    io::{self, BufReader, BufWriter},
    iter::Copied,
    path::Path,
    result, slice,
};

/// The result type for [Image] operations.
pub type Result<T> = result::Result<T, Error>;

/// Format for interpreting bytes when using textures.
#[non_exhaustive]
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
            Rgb => Self::Rgb,
            Rgba => Self::Rgba,
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
#[derive(Default, Clone)]
pub struct Image {
    /// `Image` width.
    width: u32,
    /// `Image` height.
    height: u32,
    /// Raw pixel data.
    data: Vec<u8>,
    /// Pixel Format.
    format: PixelFormat,
}

impl Image {
    /// Constructs an empty RGBA `Image` with given `width` and `height`. Alias for
    /// [Image::with_rgba].
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_rgba(width, height)
    }

    /// Constructs an empty RGBA `Image` with given `width` and `height`.
    #[inline]
    pub fn with_rgba(width: u32, height: u32) -> Self {
        let format = PixelFormat::Rgba;
        let data = vec![0x00; format.channels() * (width * height) as usize];
        Self::from_vec(width, height, data, format)
    }

    /// Constructs an empty RGB `Image` with given `width` and `height`.
    #[inline]
    pub fn with_rgb(width: u32, height: u32) -> Self {
        let format = PixelFormat::Rgb;
        let data = vec![0x00; format.channels() * (width * height) as usize];
        Self::from_vec(width, height, data, format)
    }

    /// Constructs an `Image` from a [u8] [prim@slice] representing RGB/A values.
    #[inline]
    pub fn from_bytes<B: AsRef<[u8]>>(
        width: u32,
        height: u32,
        bytes: B,
        format: PixelFormat,
    ) -> Result<Self> {
        let bytes = bytes.as_ref();
        if bytes.len() != (format.channels() * width as usize * height as usize) {
            return Err(Error::InvalidImage((width, height), bytes.len(), format));
        }
        Ok(Self::from_vec(width, height, bytes.to_vec(), format))
    }

    /// Constructs an `Image` from a [Color] [prim@slice] representing RGBA values.
    #[inline]
    pub fn from_pixels<P: AsRef<[Color]>>(
        width: u32,
        height: u32,
        pixels: P,
        format: PixelFormat,
    ) -> Result<Self> {
        let pixels = pixels.as_ref();
        if pixels.len() != (width as usize * height as usize) {
            return Err(Error::InvalidImage((width, height), pixels.len(), format));
        }
        let bytes: Vec<u8> = match format {
            PixelFormat::Rgb => pixels.iter().map(|c| c.rgb_channels()).flatten().collect(),
            PixelFormat::Rgba => pixels.iter().map(|c| c.rgba_channels()).flatten().collect(),
        };
        Ok(Self::from_vec(width, height, bytes, format))
    }

    /// Constructs an `Image` from a [Vec<u8>] representing RGB/A values.
    #[inline]
    pub fn from_vec(width: u32, height: u32, data: Vec<u8>, format: PixelFormat) -> Self {
        Self {
            width,
            height,
            data,
            format,
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
        let png = Decoder::new(png_file);

        // TODO: Make this machine-dependent to best match display capabilities for texture
        // performance
        // EXPL: Switch RGBA32 (RGBA8888) format to ARGB8888 by swapping alpha
        // EXPL: Expand paletted to RGB and non-8-bit grayscale to 8-bits
        // png.set_transformations(Transformations::SWAP_ALPHA | Transformations::EXPAND);

        let mut reader = png.read_info()?;
        let mut buf = vec![0x00; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;
        if info.bit_depth != BitDepth::Eight {
            return Err(Error::UnsupportedBitDepth(info.bit_depth));
        } else if !matches!(info.color_type, ColorType::Rgb | ColorType::Rgba) {
            return Err(Error::UnsupportedColorType(info.color_type));
        }

        let data = &buf[..info.buffer_size()];
        let format = info.color_type.into();
        Self::from_bytes(info.width, info.height, &data, format)
    }

    /// Returns the `Image` width.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the `Image` height.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the `Image` dimensions as `(width, height)`.
    #[inline]
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the center position as [Point].
    #[inline]
    pub fn center(&self) -> PointI2 {
        point!(self.width() as i32 / 2, self.height() as i32 / 2)
    }

    /// Returns the `Image` pixel data as an iterator of [u8].
    #[inline]
    pub fn bytes(&self) -> Bytes<'_> {
        Bytes(self.as_bytes().iter().copied())
    }

    /// Returns the `Image` pixel data as a [u8] [prim@slice].
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the `Image` pixel data as a mutable [u8] [prim@slice].
    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Returns the `Image` pixel data as a [`Vec<u8>`].
    ///
    /// This consumes the `Image`, so we do not need to copy its contents.
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    /// Returns the `Image` pixel data as an iterator of [Color]s.
    #[inline]
    pub fn pixels(&self) -> Pixels<'_> {
        Pixels(self.format.channels(), self.as_bytes().iter().copied())
    }

    /// Returns the `Image` pixel data as a [`Vec<Color>`].
    #[inline]
    pub fn into_pixels(self) -> Vec<Color> {
        // SAFETY: Converting to Vec<Color> from data should be valid since the image could not have
        // been constructed with an invalid set of bytes due to bounds checks in constructors.
        self.data
            .chunks(self.format.channels())
            .map(|slice| Color::from_slice(ColorMode::Rgb, slice).expect("valid image"))
            .collect()
    }

    /// Returns the color value at the given `(x, y)` position.
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let idx = self.idx(x, y);
        let channels = self.format.channels();
        // SAFETY: Converting to Color from data should be valid since the image could not have
        // been constructed with an invalid set of bytes due to bounds checks in constructors.
        Color::from_slice(ColorMode::Rgb, &self.data[idx..idx + channels]).expect("valid image")
    }

    /// Sets the color value at the given `(x, y)` position.
    #[inline]
    pub fn set_pixel<C: Into<Color>>(&mut self, x: u32, y: u32, color: C) {
        let color = color.into();
        let idx = self.idx(x, y);
        let channels = self.format.channels();
        self.data[idx..(idx + channels)].clone_from_slice(&color.channels()[..channels]);
    }

    /// Update the `Image` with a  [u8] [prim@slice] representing RGB/A values.
    #[inline]
    pub fn update_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) {
        self.data.clone_from_slice(bytes.as_ref());
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
        let png_file = BufWriter::new(File::create(&path)?);
        let mut png = png::Encoder::new(png_file, self.width, self.height);
        png.set_color(png::ColorType::Rgba);
        png.set_depth(png::BitDepth::Eight);
        let mut writer = png.write_header()?;
        Ok(writer.write_image_data(self.as_bytes())?)
    }
}

impl Image {
    #[inline]
    fn idx(&self, x: u32, y: u32) -> usize {
        self.format.channels() * (y * self.width + x) as usize
    }
}

impl PixState {
    /// Draw an [Image] to the current canvas.
    pub fn image<P>(&mut self, position: P, img: &Image) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        let pos = position.into();
        self.image_transformed(
            img,
            None,
            rect![pos.x(), pos.y(), img.width() as i32, img.height() as i32],
            0.0,
            None,
            None,
        )
    }

    /// Draw a transformed [Image] to the current canvas resized to the target `rect`, optionally
    /// rotated by an `angle` about the `center` point or `flipped`. `angle` can be in either
    /// radians or degrees based on [AngleMode].
    pub fn image_transformed<R1, R2, T, C, F>(
        &mut self,
        img: &Image,
        src: R1,
        dst: R2,
        angle: T,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
        T: AsPrimitive<Scalar>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
    {
        let s = &self.settings;
        let mut dst = dst.into();
        if let ImageMode::Center = s.image_mode {
            dst = dst.map(|dst| Rect::from_center(dst.top_left(), dst.width(), dst.height()));
        };
        let mut angle: Scalar = angle.as_();
        if let AngleMode::Radians = s.angle_mode {
            angle = angle.to_degrees();
        };
        Ok(self.renderer.image(
            img,
            src.into(),
            dst,
            angle,
            center.into(),
            flipped.into(),
            s.image_tint,
        )?)
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            .field("size", &self.data.len())
            .finish()
    }
}

/// An iterator over the bytes of an [Image].
///
/// This struct is created by the [Image::bytes] method.
/// See its documentation for more.
#[derive(Debug, Clone)]
pub struct Bytes<'a>(Copied<slice::Iter<'a, u8>>);

impl Iterator for Bytes<'_> {
    type Item = u8;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// An iterator over the [Color] pixels of an [Image].
///
/// This struct is created by the [Image::pixels] method.
/// See its documentation for more.
#[derive(Debug, Clone)]
pub struct Pixels<'a>(usize, Copied<slice::Iter<'a, u8>>);

impl Iterator for Pixels<'_> {
    type Item = Color;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.1.next()?;
        let g = self.1.next()?;
        let b = self.1.next()?;
        let channels = self.0;
        match channels {
            3 => Some(Color::from_slice(ColorMode::Rgb, [r, g, b]).expect("valid pixel")),
            4 => {
                let a = self.1.next()?;
                Some(Color::from_slice(ColorMode::Rgb, [r, g, b, a]).expect("valid pixel"))
            }
            _ => unreachable!("invalid number of color channels"),
        }
    }
}

/// The error type for [Image] operations.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Invalid image.
    InvalidImage((u32, u32), usize, PixelFormat),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidImage(dimensions, len, format) => write!(
                f,
                "invalid image. dimensions: {:?}, bytes: {}, format: {:?}",
                dimensions, len, format
            ),
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
