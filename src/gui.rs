//! Graphical User Interface elements, event handling and drawing routines.

/// Represents the current system cursor icon.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CursorType {
    Arrow,
    IBeam,
    Wait,
    Crosshair,
    WaitArrow,
    No,
    Hand,
}

impl Default for CursorType {
    fn default() -> Self {
        Self::Arrow
    }
}
