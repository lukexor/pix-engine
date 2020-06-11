//! Image related operations and functionality.

use crate::common::Result;
use std::path::Path;

/// Represents a buffer of pixel color values.
#[derive(Debug, Default, Clone)]
pub struct Image {}

impl Image {
    /// Create a blank `Image` with a given width/height.
    pub fn new(_width: u32, _height: u32) -> Self {
        unimplemented!("TODO image new");
    }

    /// Create a new `Image` from an array of u8 bytes representing RGBA values.
    pub fn from_bytes(_width: u32, _height: u32, _bytes: &[u8]) -> Result<Self> {
        unimplemented!("TODO image from_bytes");
    }

    /// Create a new `Image` by loading it from a `png` file.
    pub fn load<P: AsRef<Path>>(_path: P) -> Result<Self> {
        unimplemented!("TODO load image");
    }

    /// Save an `Image` to a `png` file.
    pub fn save<P: AsRef<Path>>(&self, _path: P) -> Result<()> {
        unimplemented!("TODO save image");
    }
}
