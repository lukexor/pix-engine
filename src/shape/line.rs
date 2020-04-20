use super::Point;
use crate::{
    math::Vector,
    renderer::Renderer,
    state::{State, StateResult},
};
use std::fmt;

pub const DEFAULT_STROKE_WEIGHT: u32 = 1;

/// Sets the style for rendering line endings. More noticeable when stroke weight is set greater
/// than 1. The default is Round.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StrokeCap {
    Round,
    Square,
    Project,
}

impl Default for StrokeCap {
    fn default() -> Self {
        Self::Round
    }
}

/// Sets the style of the joints which connect line segments. More noticeable when stroke weight is
/// set greater than 1. The default is Miter.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StrokeJoin {
    Miter,
    Bevel,
    Round,
}

impl Default for StrokeJoin {
    fn default() -> Self {
        Self::Miter
    }
}

/// Represents a line in 2D or 3D space.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    /// Creates a new `Line` instance based on a start and end `Point`.
    pub fn new<P: Into<Point>>(start: P, end: P) -> Self {
        Line {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        let s = std::iter::once(self.start);
        let e = std::iter::once(self.end);
        s.chain(e)
    }

    pub fn intersects(&self, other: &Line) -> Option<(Point, f64)> {
        None
    }
}

impl State {
    /// Draw a line.
    pub fn draw_line<L: Into<Line>>(&mut self, line: L) -> StateResult<()> {
        let line = line.into();
        Ok(self.renderer.draw_line(line)?)
    }

    /// Draw a series of lines.
    pub fn draw_lines<'a, L: Into<&'a [Line]>>(&mut self, lines: L) -> StateResult<()> {
        // TODO change to an array of lines
        Ok(self.renderer.draw_lines(lines)?)
    }
}

/// From a `Point` tuple of (start, end) to `Line`.
impl From<(Point, Point)> for Line {
    fn from((start, end): (Point, Point)) -> Self {
        Self::new(start, end)
    }
}

/// From a `Vector` tuple of (start, end) to `Line`.
impl From<(Vector, Vector)> for Line {
    fn from((start, end): (Vector, Vector)) -> Self {
        Self::new::<Point>(start.into(), end.into())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}
