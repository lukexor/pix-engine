//! UI Layout functions.

use crate::prelude::*;

impl PixState {
    /// Reset current UI rendering position back to previous line with item padding, and continue
    /// with horizontal layout.
    ///
    /// You can optionally remove the item padding, or set a different horizontal or vertical
    /// position by passing in an `offset`.
    #[inline]
    pub fn same_line<O>(&mut self, offset: O)
    where
        O: Into<Option<[i32; 2]>>,
    {
        let [x, y] = self.ui.pcursor.values();
        let offset = offset.into().unwrap_or([0; 2]);
        let item_pad = self.theme.style.item_pad;
        self.ui
            .set_cursor([x + item_pad.x() + offset[0], y + offset[1]]);
        self.ui.line_height = self.ui.pline_height - offset[1];
    }

    /// Change the default width of the next rendered element for elements that typically take up
    /// the remaining width.
    #[inline]
    pub fn next_width(&mut self, width: u32) {
        self.ui.next_width = Some(width);
    }
}

impl PixState {
    /// Draw a newline worth of spacing to the current canvas.
    pub fn spacing(&mut self) {
        let s = self;
        let height = s.theme.font_sizes.body as i32;
        s.advance_cursor([0, 0, 0, height]);
    }

    /// Draw an indent worth of spacing to the current canvas.
    pub fn indent(&mut self) -> PixResult<()> {
        let s = self;
        let (width, height) = s.size_of("    ")?;
        s.advance_cursor([0, 0, width, height]);
        s.same_line(None);
        Ok(())
    }

    /// Draw a horizontal or vertical separator to the current canvas.
    pub fn separator(&mut self) -> PixResult<()> {
        // TODO: Add s.layout(Direction) method
        let s = self;
        let pos = s.cursor_pos();
        let pad = s.theme.style.frame_pad;
        let height = s.theme.font_sizes.body as i32;
        let y = pos.y() + height / 2;

        s.push();

        s.stroke(s.primary_color());
        s.line([[pad.x(), y], [s.width()? as i32 - pad.x(), y]])?;

        s.pop();
        s.advance_cursor([0, 0, 0, height]);

        Ok(())
    }
}
