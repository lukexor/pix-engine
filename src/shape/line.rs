use super::Point;
use crate::{
    renderer::Renderer,
    state_data::{StateData, StateDataResult},
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
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    /// Creates a new `Line` instance based on a start and end `Point`.
    pub fn new<P0, P1>(start: P0, end: P1) -> Self
    where
        P0: Into<Point>,
        P1: Into<Point>,
    {
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
}

impl StateData {
    /// Draw a line.
    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) -> StateDataResult<()> {
        Ok(self.renderer.line(x0, y0, x1, y1)?)
        // if let Some(c) = self.get_stroke() {
        //     if (x1 - x0).abs() > (y1 - y0).abs() {
        //         // Line is horizontal-ish
        //         // Sort x0 < x1
        //         let (x0, y0, x1, y1) = if x0 > x1 {
        //             (x1, y1, x0, y0)
        //         } else {
        //             (x0, y0, x1, y1)
        //         };
        //         let ys = math::interpolate(x0, y0, x1, y1);
        //         for x in x0..x1 {
        //             let y = ys.get((x - x0) as usize).expect("y in range").round() as i32;
        //             self.point((x, y))?;
        //         }
        //     } else {
        //         // Line is vertical-ish
        //         // Sort xy < y1
        //         let (x0, y0, x1, y1) = if y0 > y1 {
        //             (x1, y1, x0, y0)
        //         } else {
        //             (x0, y0, x1, y1)
        //         };
        //         let xs = math::interpolate(y0, x0, y1, x1);
        //         for y in y0..y1 {
        //             let x = xs.get((y - y0) as usize).expect("y in range").round() as i32;
        //             self.point((x, y))?;
        //         }
        //     }
        // }
        // Ok(())
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}
