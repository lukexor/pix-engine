use pix_engine::prelude::*;

struct ImGui {
    edit_mode: bool,
    edit_target: usize,
    list_items: Vec<String>,
    selected_item: Option<usize>,
    disabled: bool,
    check: bool,
    radio: usize,
    text_input: String,
    rects: Vec<Rect<i32>>,
}

impl ImGui {
    fn new() -> Self {
        Self {
            edit_mode: false,
            edit_target: 0,
            list_items: vec![
                "1. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam".into(),
                "2. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam".into(),
            ],
            selected_item: None,
            disabled: false,
            check: true,
            radio: 0,
            text_input: String::new(),
            rects: vec![
                rect![10, 10, 100, 25],   // Add button
                rect![120, 10, 100, 25],  // Remove button
                rect![10, 45, 210, 160],  // Select List
                rect![10, 215, 210, 50],  // Text Field
                rect![10, 275, 210, 50],  // Output
                rect![240, 10, 100, 25],  // Checkbox 1
                rect![325, 10, 100, 25],  // Checkbox 2
                rect![240, 65, 100, 25],  // Radio 1
                rect![240, 95, 100, 25],  // Radio 2
                rect![240, 125, 100, 25], // Radio 3
                rect![395, 13, 14, 20],   // Tooltip Icon
                rect![10, 10, 100, 30],   // Tooltip
            ],
        }
    }
}

impl AppState for ImGui {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(s.background_color())?;
        s.font_family("arial")?;
        s.fill(s.text_color());
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.disable(self.disabled);

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
            "Items\nLine 2",
            &self.list_items,
            14,
            &mut self.selected_item,
        )?;

        // Text Field
        s.text_field(self.rects[3], "Input: ", &mut self.text_input)?;
        s.wrap_width(200);
        s.text(
            self.rects[4].top_left(),
            format!("Output: {}", self.text_input),
        )?;
        s.no_wrap();

        s.disable(false);
        s.checkbox(self.rects[5], "Disable", &mut self.disabled)?;
        s.disable(self.disabled);

        s.checkbox(self.rects[6], "Ipsum", &mut self.check)?;

        s.radio(self.rects[7], "Dolor", &mut self.radio, 0)?;
        s.radio(self.rects[8], "Sit", &mut self.radio, 1)?;
        s.radio(self.rects[9], "Amet", &mut self.radio, 2)?;

        s.push();
        let tooltip = " (?)";
        let (hw, hh) = s.size_of(tooltip)?;
        s.fill(s.muted_color());
        s.text(self.rects[10].top_left(), tooltip)?;
        let tooltip_text = "Some tooltip";
        let (w, h) = s.size_of(tooltip_text)?;
        s.tooltip(
            rect![0, 0, w, h],
            tooltip_text,
            [self.rects[10].x(), self.rects[10].y(), hw as i32, hh as i32],
        )?;

        if self.edit_mode {
            let rect = self.rects[self.edit_target];
            s.stroke(RED);
            s.no_fill();
            s.rect(rect)?;
        }
        s.pop();
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
