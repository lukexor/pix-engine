//! [Image] and [`PixelFormat`] functions.

use crate::{ops::clamp_dimensions, prelude::*, renderer::Rendering};
use anyhow::Context;
use png::{BitDepth, ColorType, Decoder};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    fmt,
    fs::File,
    io::{self, BufReader, BufWriter},
    iter::Copied,
    path::{Path, PathBuf},
    slice,
};

/// Format for interpreting image data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[must_use]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PixelFormat {
    /// 8-bit Red, Green, and Blue
    Rgb,
    /// 8-bit Red, Green, Blue, and Alpha
    Rgba,
}

impl PixelFormat {
    /// Returns the number of channels associated with the format.
    #[inline]
    #[must_use]
    pub const fn channels(&self) -> usize {
        match self {
            PixelFormat::Rgb => 3,
            PixelFormat::Rgba => 4,
        }
    }
}

/// The error type returned when a checked conversion from [png::ColorType] fails.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[doc(hidden)]
pub struct TryFromColorTypeError(pub(crate) ());

impl TryFrom<png::ColorType> for PixelFormat {
    type Error = TryFromColorTypeError;
    #[doc(hidden)]
    fn try_from(color_type: png::ColorType) -> Result<Self, Self::Error> {
        match color_type {
            png::ColorType::Rgb => Ok(Self::Rgb),
            png::ColorType::Rgba => Ok(Self::Rgba),
            _ => Err(TryFromColorTypeError(())),
        }
    }
}

impl From<PixelFormat> for png::ColorType {
    #[doc(hidden)]
    fn from(format: PixelFormat) -> Self {
        match format {
            PixelFormat::Rgb => Self::Rgb,
            PixelFormat::Rgba => Self::Rgba,
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
#[must_use]
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
    /// Constructs an empty RGBA `Image` with given `width` and `height`.
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_rgba(width, height)
    }

    /// Constructs an empty RGBA `Image` with given `width` and `height`.
    ///
    /// Alias for [Image::new].
    #[doc(alias = "new")]
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
    ///
    /// # Errors
    ///
    /// If the bytes length doesn't match the image dimensions and [`PixelFormat`] provided, then
    /// an error is returned.
    #[inline]
    pub fn from_bytes<B: AsRef<[u8]>>(
        width: u32,
        height: u32,
        bytes: B,
        format: PixelFormat,
    ) -> PixResult<Self> {
        let bytes = bytes.as_ref();
        if bytes.len() != (format.channels() * width as usize * height as usize) {
            return Err(PixError::InvalidImage {
                width,
                height,
                size: bytes.len(),
                format,
            }
            .into());
        }
        Ok(Self::from_vec(width, height, bytes.to_vec(), format))
    }

    /// Constructs an `Image` from a [Color] [prim@slice] representing RGBA values.
    ///
    /// # Errors
    ///
    /// If the pixels length doesn't match the image dimensions and [`PixelFormat`] provided, then
    /// an error is returned.
    #[inline]
    pub fn from_pixels<P: AsRef<[Color]>>(
        width: u32,
        height: u32,
        pixels: P,
        format: PixelFormat,
    ) -> PixResult<Self> {
        let pixels = pixels.as_ref();
        if pixels.len() != (width as usize * height as usize) {
            return Err(PixError::InvalidImage {
                width,
                height,
                size: pixels.len() * format.channels(),
                format,
            }
            .into());
        }
        let bytes: Vec<u8> = match format {
            PixelFormat::Rgb => pixels
                .iter()
                .flat_map(|p| [p.red(), p.green(), p.blue()])
                .collect(),
            PixelFormat::Rgba => pixels.iter().flat_map(Color::channels).collect(),
        };
        Ok(Self::from_vec(width, height, bytes, format))
    }

    /// Constructs an `Image` from a [`Vec<u8>`] representing RGB/A values.
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
    ///
    /// # Errors
    ///
    /// If the file format is not supported or extension is not `.png`, then an error is returned.
    pub fn from_file<P: AsRef<Path>>(path: P) -> PixResult<Self> {
        let path = path.as_ref();
        let ext = path.extension();
        if ext != Some(OsStr::new("png")) {
            return Err(PixError::UnsupportedFileType(ext.map(OsStr::to_os_string)).into());
        }
        Self::from_read(File::open(&path)?)
    }

    /// Constructs an `Image` from a [png] reader.
    ///
    /// # Errors
    ///
    /// If the file format is not supported or there is an [`io::Error`] reading the file then an
    /// error is returned.
    pub fn from_read<R: io::Read>(read: R) -> PixResult<Self> {
        let png_file = BufReader::new(read);
        let png = Decoder::new(png_file);

        // TODO: Make this machine-dependent to best match display capabilities for performance
        // EXPL: Switch RGBA32 (RGBA8888) format to ARGB8888 by swapping alpha
        // EXPL: Expand paletted to RGB and non-8-bit grayscale to 8-bits
        // png.set_transformations(Transformations::SWAP_ALPHA | Transformations::EXPAND);

        let mut reader = png.read_info().context("failed to read png data")?;
        let mut buf = vec![0x00; reader.output_buffer_size()];
        let info = reader
            .next_frame(&mut buf)
            .context("failed to read png data frame")?;
        let bit_depth = info.bit_depth;
        let color_type = info.color_type;
        if bit_depth != BitDepth::Eight || !matches!(color_type, ColorType::Rgb | ColorType::Rgba) {
            return Err(PixError::UnsupportedImageFormat {
                bit_depth,
                color_type,
            }
            .into());
        }

        let data = &buf[..info.buffer_size()];
        let format = info
            .color_type
            .try_into()
            .map_err(|_| PixError::UnsupportedImageFormat {
                bit_depth,
                color_type,
            })?;
        Self::from_bytes(info.width, info.height, &data, format)
    }

    /// Returns the `Image` width.
    #[inline]
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the `Image` height.
    #[inline]
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the `Image` dimensions as `(width, height)`.
    #[inline]
    #[must_use]
    pub const fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the `pitch` of the image data which is the number of bytes in a row of pixel data,
    /// including padding between lines.
    #[inline]
    #[must_use]
    pub const fn pitch(&self) -> usize {
        self.width() as usize * self.format.channels()
    }

    /// Returns the `Image` bounding [Rect] positioned at `(0, 0)`.
    ///
    /// The width and height of the returned rectangle are clamped to ensure that size does not
    /// exceed [`i32::MAX`]. This could result in unexpected behavior with drawing routines if the
    /// image size is larger than this.
    #[inline]
    pub fn bounding_rect(&self) -> Rect<i32> {
        let (width, height) = clamp_dimensions(self.width, self.height);
        rect![0, 0, width, height]
    }

    /// Returns the `Image` bounding [Rect] positioned at `offset`.
    #[inline]
    pub fn bounding_rect_offset<P>(&self, offset: P) -> Rect<i32>
    where
        P: Into<PointI2>,
    {
        let (width, height) = clamp_dimensions(self.width, self.height);
        rect![offset.into(), width, height]
    }

    /// Returns the center position as [Point].
    #[inline]
    pub fn center(&self) -> PointI2 {
        let (width, height) = clamp_dimensions(self.width, self.height);
        point!(width / 2, height / 2)
    }

    /// Returns the `Image` pixel data as an iterator of [u8].
    #[inline]
    pub fn bytes(&self) -> Bytes<'_> {
        Bytes(self.as_bytes().iter().copied())
    }

    /// Returns the `Image` pixel data as a [u8] [prim@slice].
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the `Image` pixel data as a mutable [u8] [prim@slice].
    #[inline]
    #[must_use]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Returns the `Image` pixel data as a [`Vec<u8>`].
    ///
    /// This consumes the `Image`, so we do not need to copy its contents.
    #[inline]
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    /// Returns the `Image` pixel data as an iterator of [Color]s.
    #[inline]
    pub fn pixels(&self) -> Pixels<'_> {
        Pixels(self.format.channels(), self.as_bytes().iter().copied())
    }

    /// Returns the `Image` pixel data as a [`Vec<Color>`].
    ///
    /// # Panics
    ///
    /// Panics if the image has an invalid sequence of bytes given it's [`PixelFormat`].
    #[inline]
    #[must_use]
    pub fn into_pixels(self) -> Vec<Color> {
        self.data
            .chunks(self.format.channels())
            .map(|slice| match *slice {
                [red, green, blue] => Color::rgb(red, green, blue),
                [red, green, blue, alpha] => Color::rgba(red, green, blue, alpha),
                _ => panic!("invalid number of color channels"),
            })
            .collect()
    }

    /// Returns the color value at the given `(x, y)` position.
    ///
    /// # Panics
    ///
    /// Panics if the image has an invalid sequence of bytes given it's [`PixelFormat`], or the `(x,
    /// y`) index is out of range.
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let idx = self.idx(x, y);
        let channels = self.format.channels();
        match self.data.get(idx..idx + channels) {
            Some([red, green, blue]) => Color::rgb(*red, *green, *blue),
            Some([red, green, blue, alpha]) => Color::rgba(*red, *green, *blue, *alpha),
            _ => panic!("invalid number of color channels"),
        }
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
    pub const fn format(&self) -> PixelFormat {
        self.format
    }

    /// Save the `Image` to a [png] file.
    ///
    /// # Errors
    ///
    /// Returns an error for any of the following:
    ///     - An [`io::Error`] occurs attempting to create the [png] file.
    ///     - A [`png::EncodingError`] occurs attempting to write image bytes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { image: Image };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     if let Key::S = event.key {
    ///         self.image.save("test_image.png")?;
    ///     }
    ///     Ok(false)
    /// }
    /// # }
    /// ```
    pub fn save<P>(&self, path: P) -> PixResult<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let png_file = BufWriter::new(File::create(&path)?);
        let mut png = png::Encoder::new(png_file, self.width, self.height);
        png.set_color(self.format.into());
        png.set_depth(png::BitDepth::Eight);
        let mut writer = png
            .write_header()
            .with_context(|| format!("failed to write png header: {:?}", path))?;
        writer
            .write_image_data(self.as_bytes())
            .with_context(|| format!("failed to write png data: {:?}", path))
    }
}

impl Image {
    /// Helper function to get the byte array index based on `(x, y)`.
    #[inline]
    const fn idx(&self, x: u32, y: u32) -> usize {
        self.format.channels() * (y * self.width + x) as usize
    }
}

impl PixState {
    /// Draw an [Image] to the current canvas.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     let image = Image::from_file("./some_image.png")?;
    ///     s.image(&image, [10, 10])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn image<P>(&mut self, img: &Image, position: P) -> PixResult<()>
    where
        P: Into<PointI2>,
    {
        let pos = position.into();
        let dst = img.bounding_rect_offset(pos);
        self.image_transformed(img, None, dst, 0.0, None, None)
    }

    /// Draw a transformed [Image] to the current canvas resized to the target `rect`, optionally
    /// rotated by an `angle` about the `center` point or `flipped`. `angle` can be in either
    /// radians or degrees based on [`AngleMode`]. [`PixState::image_tint`] can optionally add a tint
    /// color to the rendered image.
    ///
    /// # Errors
    ///
    /// If the renderer fails to draw to the current render target, then an error is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text_field: String, text_area: String};
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.angle_mode(AngleMode::Degrees);
    ///     let image = Image::from_file("./some_image.png")?;
    ///     let src = None; // Draw entire image instead of a sub-image
    ///     let dst = image.bounding_rect_offset([10, 10]); // Draw image at `(10, 10)`.
    ///     let angle = 10.0;
    ///     let center = point!(10, 10);
    ///     s.image_transformed(&image, src, dst, angle, center, Flipped::Horizontal)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub fn image_transformed<R1, R2, A, C, F>(
        &mut self,
        img: &Image,
        src: R1,
        dst: R2,
        angle: A,
        center: C,
        flipped: F,
    ) -> PixResult<()>
    where
        R1: Into<Option<Rect<i32>>>,
        R2: Into<Option<Rect<i32>>>,
        A: Into<Option<Scalar>>,
        C: Into<Option<PointI2>>,
        F: Into<Option<Flipped>>,
    {
        let s = &self.settings;
        let mut dst = dst.into();
        if let ImageMode::Center = s.image_mode {
            dst = dst.map(|dst| Rect::from_center(dst.top_left(), dst.width(), dst.height()));
        };
        let mut angle = angle.into().unwrap_or(0.0);
        if let AngleMode::Radians = s.angle_mode {
            angle = angle.to_degrees();
        };
        self.renderer.image(
            img,
            src.into(),
            dst,
            angle,
            center.into(),
            flipped.into(),
            s.image_tint,
        )
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("size", &self.data.len())
            .field("format", &self.format)
            .finish()
    }
}

/// An iterator over the bytes of an [Image].
///
/// This struct is created by the [`Image::bytes`] method.
/// See its documentation for more.
#[derive(Debug, Clone)]
#[must_use]
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
/// This struct is created by the [`Image::pixels`] method.
/// See its documentation for more.
#[derive(Debug, Clone)]
#[must_use]
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
            3 => Some(Color::rgb(r, g, b)),
            4 => {
                let a = self.1.next()?;
                Some(Color::rgba(r, g, b, a))
            }
            _ => panic!("invalid number of color channels"),
        }
    }
}

/// Represents an image icon source.
#[derive(Debug, Clone)]
pub enum Icon {
    /// An icon image.
    Image(Image),
    #[cfg(not(target_arch = "wasm32"))]
    /// A path to an icon image.
    Path(PathBuf),
}

impl<T: Into<PathBuf>> From<T> for Icon {
    fn from(value: T) -> Self {
        Self::Path(value.into())
    }
}

impl From<Image> for Icon {
    fn from(img: Image) -> Self {
        Self::Image(img)
    }
}
