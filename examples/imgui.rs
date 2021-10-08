use pix_engine::prelude::*;

struct ImGui {
    edit_mode: bool,
    edit_target: usize,
    list_items: Vec<String>,
    selected_item: Option<usize>,
    text_input: String,
    rects: Vec<Rect<i32>>,
}

impl ImGui {
    fn new() -> Self {
        Self {
            edit_mode: false,
            edit_target: 0,
            list_items: Vec::new(),
            selected_item: None,
            text_input: String::new(),
            rects: vec![
                rect![10, 10, 100, 25],  // Add button
                rect![120, 10, 100, 25], // Remove button
                rect![10, 45, 210, 145], // Select List
                rect![10, 200, 210, 30], // Text Field
                rect![10, 240, 210, 50], // Output
            ],
        }
    }
}

impl AppState for ImGui {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(s.background_color())?;
        s.font_family("arial")?;
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        // Buttons
        if s.button(self.rects[0], "Add Item")? {
            self.list_items.push(format!(
                "{}. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam",
                self.list_items.len() + 1
            ));
        }
        if s.button(self.rects[1], "Remove Item")? {
            self.list_items.pop();
        }

        // Select List
        s.select_list(
            self.rects[2],
            "Items",
            &self.list_items,
            14,
            &mut self.selected_item,
        )?;

        // Text Field
        s.same_line(true);
        s.text_field(self.rects[3], "Input:", &mut self.text_input)?;
        s.same_line(false);

        s.disabled(true);
        s.text_field(self.rects[4], "Output:", &mut self.text_input)?;
        s.disabled(false);

        if self.edit_mode {
            let rect = self.rects[self.edit_target];
            s.stroke(RED);
            s.no_fill();
            s.rect(rect)?;
        }
        Ok(())
    }

    // Testing shortcuts for visual editing of element sizes and positions.
    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        if event.key == Key::Escape {
            self.edit_mode = !self.edit_mode;
        }
        if !self.edit_mode {
            return Ok(false);
        }

        let rect = &mut self.rects[self.edit_target];
        let handled = match event.key {
            Key::Tab if event.keymod == KeyMod::NONE => {
                self.edit_target = (self.edit_target + 1) % self.rects.len();
                true
            }
            Key::Tab if event.keymod == KeyMod::SHIFT => {
                if self.edit_target > 0 {
                    self.edit_target -= 1;
                } else {
                    self.edit_target = self.rects.len() - 1;
                }
                true
            }
            Key::Up if event.keymod == KeyMod::NONE => {
                rect.set_y(rect.y() - 1);
                true
            }
            Key::Down if event.keymod == KeyMod::NONE => {
                rect.set_y(rect.y() + 1);
                true
            }
            Key::Left if event.keymod == KeyMod::NONE => {
                rect.set_x(rect.x() - 1);
                true
            }
            Key::Right if event.keymod == KeyMod::NONE => {
                rect.set_x(rect.x() + 1);
                true
            }
            Key::Up if event.keymod == KeyMod::SHIFT => {
                rect.set_height(rect.height() + 1);
                true
            }
            Key::Down if event.keymod == KeyMod::SHIFT => {
                rect.set_height(rect.height() - 1);
                true
            }
            Key::Left if event.keymod == KeyMod::SHIFT => {
                rect.set_width(rect.width() - 1);
                true
            }
            Key::Right if event.keymod == KeyMod::SHIFT => {
                rect.set_width(rect.width() + 1);
                true
            }
            Key::Return => {
                dbg!(&rect);
                true
            }
            _ => false,
        };
        Ok(handled)
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(800, 600)
        .with_title("Immediate GUI")
        .position_centered()
        .with_frame_rate()
        .build();
    let mut app = ImGui::new();
    engine.run(&mut app)
}
