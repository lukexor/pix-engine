use crate::{core::state::MouseState, prelude::*};

#[derive(Debug)]
pub struct Button<'a> {
    rect: Rect<i32>,
    label: &'a str,
    hovered: bool,
    clicked: bool,
}

impl Button<'_> {
    /// Returns the button `Rect`.
    pub fn rect(&self) -> Rect<i32> {
        self.rect
    }

    /// Returns the button label.
    pub fn label(&self) -> &str {
        self.label
    }

    /// Returns whether the button was hovered this frame.
    pub fn hovered(&self) -> bool {
        self.hovered
    }

    /// Returns whether the button was clicked this frame.
    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

impl<'a> Button<'a> {
    #[inline]
    fn new(rect: Rect<i32>, label: &'a str, mouse: &MouseState) -> Self {
        let hovered = rect.contains_point(mouse.pos);
        let clicked = hovered && mouse.was_clicked(&Mouse::Left);
        Self {
            rect,
            label,
            hovered,
            clicked,
        }
    }
}

impl Draw for Button<'_> {
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.push();

        if self.hovered {
            s.fill(NAVY);
            s.frame_cursor(&Cursor::hand())?;
        } else {
            s.fill(GRAY);
        }
        s.stroke(WHITE);
        s.rect(self.rect.as_())?;

        s.rect_mode(DrawMode::Center);
        s.fill(WHITE);
        s.text(self.rect.center(), self.label)?;

        s.pop();
        Ok(())
    }
}

impl PixState {
    /// Draw a [Button] to the current canvas.
    pub fn button<'a, R>(&mut self, rect: R, label: &'a str) -> PixResult<Button<'a>>
    where
        R: Into<Rect>,
    {
        let button = Button::new(rect.into().as_::<i32>(), label, &self.mouse);
        button.draw(self)?;
        Ok(button)
    }
}
