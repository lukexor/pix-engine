use pix_engine::prelude::*;

struct ImGui {
    selected: Option<usize>,
    items: Vec<String>,
    text_input: String,
    rect: Rect<i32>,
}

impl ImGui {
    fn new() -> Self {
        Self {
            selected: None,
            items: Vec::new(),
            text_input: String::new(),
            rect: rect![10, 45, 120, 103],
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
        if s.button([10, 10, 100, 25], "Add Item")? {
            self.items.push(format!("Item {}", self.items.len() + 1));
        }
        if s.button([120, 10, 100, 25], "Remove Item")? {
            self.items.pop();
        }

        // Select List
        s.select_list(
            [10, 50, 120, 103],
            "Items",
            &self.items,
            14,
            &mut self.selected,
        )?;

        // Text Field
        s.same_line(true);
        s.text_field([10, 195, 130, 30], "Input:", &mut self.text_input)?;
        s.text([10, 225], &format!("Output: {}", self.text_input))?;
        Ok(())
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
