/// Determines the way images are drawn by changing how the parameters given to State.draw_image
/// are interpreted. The default is Corner.
///
/// Corner: Uses x and y as the upper-left corner of the image
/// Center: Uses x and y as the center of the image
pub enum ImageMode {
    Corner,
    Center,
}

impl Default for ImageMode {
    fn default() -> Self {
        Self::Corner
    }
}
