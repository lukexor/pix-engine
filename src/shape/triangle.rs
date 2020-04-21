use super::Point;
use crate::state::{State, StateResult};

impl State {
    /// Draw a series of lines.
    pub fn triangle<P1, P2, P3>(&mut self, p1: P1, p2: P2, p3: P3) -> StateResult<()>
    where
        P1: Into<Point>,
        P2: Into<Point>,
        P3: Into<Point>,
    {
        let (p1, p2, p3) = (p1.into(), p2.into(), p3.into());
        if let Some(c) = self.get_stroke() {
            self.line((p1, p2))?;
            self.line((p2, p3))?;
            self.line((p3, p1))?;
        }
        if let Some(c) = self.get_fill() {
            // TODO fill triangle
        }
        Ok(())
    }
}
