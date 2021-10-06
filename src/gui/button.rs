//! Immediate-GUI functions related to rendering and interacting with buttons.
//!
//! # Examples
//!
//! ```
//! # use pix_engine::prelude::*;
//! # struct App;
//! # impl App {
//! fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!     if s.button([0, 0, 100, 50], "Click Me")? {
//!       println!("I was clicked!");
//!     }
//!     Ok(())
//! }
//! # }
//! ```

use super::get_hash;
use crate::{prelude::*, renderer::Rendering};

impl PixState {
    /// Draw a button to the current canvas that returns `true` when clicked.
    pub fn button<R>(&mut self, rect: R, label: &str) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
    {
        let s = self;
        let rect = s.get_rect(rect);
        let id = get_hash(&rect);

        s.push();

        // Check hover/active/keyboard focus
        if rect.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);

        // Render

        // Button
        s.rect_mode(RectMode::Corner);
        if s.ui_state.is_focused(id) {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        let hovered = s.ui_state.is_hovered(id);
        let active = s.ui_state.is_active(id);
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(s.accent_color());
            if active {
                let mut rect = rect;
                rect.set_x(rect.x() + 2);
                rect.set_y(rect.y() + 2);
                s.rounded_rect(rect, 3.0)?;
            } else {
                s.rounded_rect(rect, 3.0)?;
            }
        } else {
            s.fill(s.primary_color());
            s.rounded_rect(rect, 3.0)?;
        }

        // Button text
        s.rect_mode(RectMode::Center);
        s.renderer.font_family(&s.theme.fonts.body)?;
        s.fill(s.text_color());
        s.text(rect.center(), label)?;

        s.pop();

        // Process input
        s.ui_state.handle_tab(id);
        s.ui_state.set_last(id);
        Ok(s.ui_state.was_clicked(id))
    }
}
