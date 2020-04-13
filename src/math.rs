/// The mode used to determine the value of angle parameters either in Radians
/// or Degrees. The default is Radians.
pub enum AngleMode {
    Radians,
    Degrees,
}

impl Default for AngleMode {
    fn default() -> Self {
        Self::Radians
    }
}
