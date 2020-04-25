use crate::{
    math,
    state::{State, StateResult},
};

impl State {
    /// Draw a series of lines.
    pub fn triangle(
        &mut self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> StateResult<()> {
        if let Some(c) = self.get_stroke() {
            self.line(x0, y0, x1, y1)?;
            self.line(x1, y1, x2, y2)?;
            self.line(x2, y2, x0, y0)?;
        }
        if let Some(c) = self.get_fill() {
            // Sort the points so that y0 <= y1 <= y2
            let (x0, y0, x1, y1) = if y1 < y0 {
                (x1, y1, x0, y0)
            } else {
                (x0, y0, x1, y1)
            };
            let (x0, y0, x2, y2) = if y2 < y0 {
                (x2, y2, x0, y0)
            } else {
                (x0, y0, x2, y2)
            };
            let (x1, y1, x2, y2) = if y2 < y1 {
                (x2, y2, x1, y1)
            } else {
                (x1, y1, x2, y2)
            };

            // Compute the x-coordinates of the triangle edges
            let x01 = math::interpolate(y0, x0, y1, x1);
            let x12 = math::interpolate(y1, x1, y2, x2);
            let x02 = math::interpolate(y0, x0, y2, x2);

            // Remove duplicate point y1 which is both the last value of x01 and the first value
            // of x12
            // if x01.last() == x12.last() {
            //     x01.pop();
            // }
            // Concatenate the short sides
            let mut x012 = x01;
            x012.extend(x12);

            // Determine which is left and which is right
            let m = x02.len() / 2;
            let (x_left, x_right) = if x02.get(m).expect("x02 mid in range")
                < x012.get(m).expect("x012 mid in range")
            {
                (x02, x012)
            } else {
                (x012, x02)
            };

            // Draw the horizontal segments
            self.stroke(c);
            for y in y0..y2 {
                let x0 = x_left
                    .get((y - y0) as usize)
                    .expect("x_left in range")
                    .round() as i32;
                let x1 = x_right
                    .get((y - y0) as usize)
                    .expect("x_right in range")
                    .round() as i32;
                self.line(x0, y, x1, y)?;
            }
            if let Some(c) = self.get_stroke() {
                self.stroke(c);
            }
        }
        Ok(())
    }
}
