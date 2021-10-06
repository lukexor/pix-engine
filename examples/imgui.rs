use pix_engine::prelude::*;

struct ImGui {
    selected: Option<usize>,
    background: Color,
}

impl ImGui {
    fn new() -> Self {
        Self {
            selected: None,
            background: BLACK,
        }
    }
}

impl AppState for ImGui {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        self.background = s.background_color();
        s.font_family("helvetica")?;
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(self.background)?;
        let (_, h) = s.size_of("Item 1")?;
        s.select_list(
            [10, 10, 200, 125],
            "Select",
            &[
                "Item 1", "Item 2", "Item 3", "Item 4", "Item 5", "Item 6", "Item 7", "Item 8",
            ],
            h,
            &mut self.selected,
        )?;
        if s.button([10, 160, 120, 25], "Click me")? {
            self.background = Color::random();
        }
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
