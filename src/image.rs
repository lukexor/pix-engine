//! Image processing.

use crate::{
    color::Color,
    shape::{Point, Rect},
    state_data::rendering::BlendMode,
    StateData,
};
use rayon::prelude::*;
use std::{
    borrow::Cow,
    error,
    ffi::OsStr,
    fmt,
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

pub mod prelude {
    pub use super::{Image, ImageFilter, ImageMode, PixelFormat};
}

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
/// `StateData::image()` are interpreted. The default is Corner.
///
/// Corner: Uses x and y as the upper-left corner of the image.
/// Center: Uses x and y as the center of the image.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImageMode {
    Corner,
    Center,
}

impl Default for ImageMode {
    fn default() -> Self {
        Self::Corner
    }
}

/// Represents the data format for an Image as either RGB or RGBA.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb,
    Rgba,
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

/// A filter type that can be applied to an image.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImageFilter {
    Threshold,
    Gray,
    Opaque,
    Invert,
    Posterize,
    Blur,
    Erode,
    Dilate,
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
    pixels: Vec<Color>,
}

impl Image {
    /// Creates a new RGBA image with given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self::rgba(width, height)
    }

    /// Creates a new RGB image with given dimensions.
    pub fn rgb(width: u32, height: u32) -> Self {
        Self::with_format(width, height, PixelFormat::Rgb)
    }

    /// Creates a new RGBA image with given dimensions.
    pub fn rgba(width: u32, height: u32) -> Self {
        Self::with_format(width, height, PixelFormat::Rgba)
    }

    fn with_format(width: u32, height: u32, pixel_format: PixelFormat) -> Self {
        let channels = match pixel_format {
            PixelFormat::Rgb => RGB_CHANNELS,
            PixelFormat::Rgba => RGBA_CHANNELS,
        };
        Self {
            width,
            height,
            channels,
            pixel_format,
            data: vec![0; channels * (width * height) as usize],
            pixels: Vec::new(),
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
                pixels: Vec::new(),
            })
        }
    }

    /// Creates a new image from a list of pixels.
    pub fn from_pixels(width: u32, height: u32, pixels: &[Color]) -> Result<Self> {
        if pixels.len() != (width * height) as usize {
            Err(Error::InvalidFormat(PixelFormat::Rgba, width, height))
        } else {
            Ok(Self {
                width,
                height,
                channels: RGBA_CHANNELS,
                pixel_format: PixelFormat::Rgba,
                data: pixels
                    .par_iter()
                    .map(|c| &c[..])
                    .flatten()
                    .copied()
                    .collect(),
                pixels: Vec::new(),
            })
        }
    }

    /// Gets the pixel color at the given (x, y) coords.
    pub fn pixel<P: Into<Point>>(&self, p: P) -> Color {
        let idx = self.get_index(p);
        self.data[idx..idx + self.channels].into()
    }

    /// Sets the pixel color at the given (x, y) coords.
    pub fn set_pixel<C: Into<Color>, P: Into<Point>>(&mut self, p: P, color: C) {
        let idx = self.get_index(p);
        self.data[idx..idx + self.channels].copy_from_slice(&color.into()[0..self.channels]);
    }

    /// Loads the pixel data of the image into a pixels array that can be iterated over with
    /// `Image::pixels()` or `Image::pixels_mut()`. This function must be called before operating
    /// on pixels. For operating on raw bytes without calling this function, call `Image::bytes()`.
    pub fn load_pixels(&mut self) {
        self.pixels = self
            .data
            .par_chunks(self.channels)
            .map(|c| c.into())
            .collect();
    }

    /// Updates the pixel data of the image that was loaded with `Image::load_pixels()` and
    /// modified with `Image::pixels_mut()`. This function must be called after updating pixels for
    /// changes to take effect. For updating raw bytes without calling this function, call
    /// `Image::bytes_mut()`.
    ///
    /// # Remarks
    /// This is a slow operation and should not be called often.
    pub fn update_pixels(&mut self) {
        self.data = self
            .pixels
            .par_iter()
            .flat_map(|c| &c[..self.channels])
            .copied()
            .collect();
    }

    /// PixelFormat of the image.
    pub fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }

    /// Dimensions of the image.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a reference to the pixels of the image.
    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    /// Returns a mutable reference to the pixels of the image.
    pub fn pixels_mut(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }

    /// Returns a reference to the pixel data of the image.
    pub fn bytes(&self) -> &Vec<u8> {
        &self.data
    }

    /// Returns a mutable reference to the pixel data of the image.
    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Applies a filter to the image.
    ///
    /// Threshold: Converts to black and white based on the threshold passed into value.
    /// Gray: Converts to grayscale.
    /// Opaque: Sets the alpha channel to entirely opaque.
    /// Invert: Sets each pixel to its inverse value.
    /// Posterize: Limits each channel to the number of colors passed into value.
    /// Blur: Gaussian blur image with value. Defaults to a blur of 1.
    /// Erode: Reduces light areas.
    /// Dilate: Increases light areas.
    pub fn filter(&mut self, filter: ImageFilter, value: Option<f32>) {
        // TODO Image::filter()
        unimplemented!();
    }

    /// Applies a filter to the target image, returning a new Image.
    ///
    /// Threshold: Converts to black and white based on the threshold passed into value.
    /// Gray: Converts to grayscale.
    /// Opaque: Sets the alpha channel to entirely opaque.
    /// Invert: Sets each pixel to its inverse value.
    /// Posterize: Limits each channel to the number of colors passed into value.
    /// Blur: Gaussian blur image with value. Defaults to a blur of 1.
    /// Erode: Reduces light areas.
    /// Dilate: Increases light areas.
    pub fn filtered(img: &Image, filter: ImageFilter, value: Option<f32>) -> Self {
        // TODO Image::filtered_image()
        unimplemented!();
    }

    /// Blends one image onto another using a given blend mode.
    pub fn blend(&mut self, img: &Image, mode: BlendMode) {
        // TODO Image::blend()
        unimplemented!();
    }

    /// Blends one image onto another using a given blend mode, returning a new Image
    pub fn blended(img1: &Image, img2: &Image, mode: BlendMode) -> Self {
        // TODO Image::blended()
        unimplemented!();
    }

    /// Masks one image using the alpha channel from another image
    pub fn mask(&mut self, img: &Image) {
        // TODO Image::mask()
        unimplemented!();
    }

    /// Masks one image using the alpha channel from another image, returning a new Image
    pub fn masked(img1: &Image, img2: &Image) -> Self {
        // TODO Image::masked()
        unimplemented!();
    }

    /// Resizes the image to the target width/height.
    pub fn resize(&mut self, width: u32, height: u32) {
        // TODO Image::resize()
        unimplemented!();
    }

    /// Resizes the target image to the target width/height, returning a new Image.
    pub fn resized(img: &Image, width: u32, height: u32) -> Self {
        // TODO Image::resized_image()
        unimplemented!();
    }

    /// Returns a sub-image from the current image.
    pub fn sub_image<R: Into<Rect>>(&self, rect: R) -> Self {
        let rect: Rect = rect.into();
        // TODO Image::sub_image()
        unimplemented!();
    }

    /// Sets a sub-image within the current image.
    pub fn set_sub_image<P: Into<Point>>(&self, p: P, img: &Image) {
        // TODO Image::set_sub_image()
        unimplemented!();
    }

    /// Create a new image from a PNG file.
    /// Only 8-bit RGB and RGBA formats are supported currently.
    pub fn load<P: AsRef<Path>>(path: &P) -> Result<Self> {
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
    pub fn save<P: AsRef<Path>>(&self, path: &P) -> Result<()> {
        let path = path.as_ref();
        let png_file = BufWriter::new(std::fs::File::create(&path)?);
        let mut png = png::Encoder::new(png_file, self.width, self.height);
        png.set_color(self.pixel_format.into());
        let mut writer = png.write_header()?;
        writer.write_image_data(self.bytes())?;
        Ok(())
    }

    /// Used internally to get the index into the data array by (x, y).
    fn get_index<P: Into<Point>>(&self, p: P) -> usize {
        let p: Point = p.into();
        assert!(
            p.x >= 0 && p.y >= 0,
            "pixel index {:?} is negative",
            (p.x, p.y)
        );
        let idx = self.channels * (p.y as u32 * self.width + p.x as u32) as usize;
        assert!(
            idx < self.data.len(),
            "pixel {:?} out of bounds {:?}",
            (p.x, p.y),
            (self.width, self.height)
        );
        idx
    }
}

impl StateData {
    /// Creates a new RGBA image with given dimensions.
    pub fn create_image(&self, width: u32, height: u32) -> Image {
        Image::new(width, height)
    }

    /// Draws a given image to the current window target.
    pub fn image<P: Into<Point>>(&mut self, img: Image, p: P) {
        // TODO StateData::image()
        unimplemented!();
    }

    /// Saves the current window target to a PNG image given by path.
    pub fn save_canvas<P: AsRef<Path>>(&self, path: P) {
        // TODO StateData::save_canvas()
        unimplemented!();
    }

    /// Draws a given image to the current window target, resized to target dimensions.
    pub fn image_resized<R: Into<Rect>>(&mut self, img: Image, rect: R) {
        // TODO StateData::image_resized()
        unimplemented!();
    }

    /// Draws a given image to the current window target with top-left at p1 and bottom-right at
    /// p2.
    pub fn image_corners<P: Into<Point>>(&mut self, img: Image, p1: P, p2: P) {
        // TODO StateData::image_corners()
        unimplemented!();
    }

    /// Draws a portion of the given image given by dest rectangle to the current window target at
    /// the given src rectangle.
    pub fn image_projected<R: Into<Option<Rect>>>(&mut self, img: Image, dest: R, src: R) {
        // TODO StateData::image_projected()
        unimplemented!();
    }
}
