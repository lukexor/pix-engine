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
    pub fn button<R, L>(&mut self, rect: R, label: L) -> PixResult<bool>
    where
        R: Into<Rect<i32>>,
        L: AsRef<str>,
    {
        let rect = self.get_rect(rect);
        self._button(rect, label.as_ref())
    }

    fn _button(&mut self, rect: Rect<i32>, label: &str) -> PixResult<bool> {
        let s = self;
        let id = get_hash(&rect);

        // Check hover/active/keyboard focus
        let disabled = s.ui_state.disabled;
        if !disabled && rect.contains_point(s.mouse_pos()) {
            s.ui_state.hover(id);
        }
        s.ui_state.try_capture(id);
        let focused = !disabled && s.ui_state.is_focused(id);
        let hovered = s.ui_state.is_hovered(id);
        let active = s.ui_state.is_active(id);

        s.push();

        // Render
        let radius = 3;

        // Button
        s.rect_mode(RectMode::Corner);
        if focused {
            s.stroke(s.secondary_color());
        } else {
            s.stroke(s.muted_color());
        }
        if hovered {
            s.frame_cursor(&Cursor::hand())?;
            s.fill(s.accent_color());
            if active {
                let [x, y, width, height] = rect.values();
                s.rounded_rect([x + 1, y + 1, width, height], radius)?;
            } else {
                s.rounded_rect(rect, radius)?;
            }
        } else {
            s.fill(s.primary_color());
            s.rounded_rect(rect, radius)?;
        }

        // Button text
        s.rect_mode(RectMode::Center);
        s.renderer.font_family(&s.theme.fonts.body)?;
        s.fill(s.text_color());
        s.clip(rect)?;
        s.text(rect.center(), label)?;
        s.no_clip()?;

        s.pop();

        // Process input
        s.ui_state.handle_input(id);
        if !disabled {
            Ok(s.ui_state.was_clicked(id))
        } else {
            Ok(false)
        }
    }
}
