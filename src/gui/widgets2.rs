//! UI widget rendering functions.

use super::Size;
use crate::prelude::*;

impl PixState {
    /// Frame
    pub fn frame<R, S>(&mut self, rect: R, label: S) -> PixResult<()>
    where
        R: Into<Rect<i32>>,
        S: AsRef<str>,
    {
        let s = self;
        Ok(())
    }

    ///// Draw a button to the current canvas that returns `true` when clicked.
    /////
    ///// # Example
    /////
    ///// ```
    ///// # use pix_engine::prelude::*;
    ///// # struct App;
    ///// # impl App {
    ///// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    /////     if s.button([0, 0, 100, 50], "Click Me")? {
    /////       println!("I was clicked!");
    /////     }
    /////     Ok(())
    ///// }
    ///// # }
    ///// ```
    //pub fn button2<L>(&mut self, label: L) -> PixResult<bool>
    //where
    //    L: AsRef<str>,
    //{
    //    self._button2(label.as_ref(), None)
    //}
}

impl PixState {
    // fn _button2(&mut self, label: &str, size: Option<Size>) -> PixResult<bool> {
    //     let s = self;
    //     let id = get_hash(&label);

    //     // Check hover/active/keyboard focus
    //     let disabled = s.ui.disabled;
    //     if !disabled && rect.contains_point(s.mouse_pos()) {
    //         s.ui.hover(id);
    //     }
    //     s.ui.try_capture(id);
    //     let focused = !disabled && s.ui.is_focused(id);
    //     let hovered = s.ui.is_hovered(id);
    //     let active = s.ui.is_active(id);

    //     s.push();

    //     // Render

    //     // Button
    //     s.rect_mode(RectMode::Corner);
    //     if focused {
    //         s.stroke(s.highlight_color());
    //     } else {
    //         s.stroke(s.muted_color());
    //     }
    //     if hovered {
    //         s.frame_cursor(&Cursor::hand())?;
    //         s.fill(s.highlight_color());
    //         if active {
    //             let [x, y, width, height] = rect.values();
    //             s.rect([x + 1, y + 1, width, height])?;
    //         } else {
    //             s.rect(rect)?;
    //         }
    //     } else if disabled {
    //         s.fill(s.background_color());
    //         s.rect(rect)?;
    //     } else {
    //         s.fill(s.primary_color());
    //         s.rect(rect)?;
    //     }

    //     // Button text
    //     s.rect_mode(RectMode::Center);
    //     if disabled {
    //         s.fill(s.muted_color());
    //     } else {
    //         s.fill(s.text_color());
    //     }
    //     s.clip(rect)?;
    //     s.text(rect.center(), label)?;
    //     s.no_clip()?;

    //     s.pop();

    //     // Process input
    //     s.ui.handle_input(id);
    //     if !disabled {
    //         Ok(s.ui.was_clicked(id))
    //     } else {
    //         Ok(false)
    //     }
    // }
}
