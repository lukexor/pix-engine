use pix_engine::prelude::*;

struct Gui {
    items: Vec<String>,
    selected: Option<usize>,
    disabled: bool,
    check: bool,
    radio: usize,
    input: String,
}

impl Gui {
    fn new() -> Self {
        Self {
            items: vec![
                "1. Lorem ipsum dolor".into(),
                "2. Lorem ipsum dolor sit".into(),
            ],
            selected: None,
            disabled: false,
            check: true,
            radio: 0,
            input: String::new(),
        }
    }
}

impl AppState for Gui {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        s.background(s.background_color())?;
        s.font_family("arial")?;
        s.fill(s.text_color());
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.disable(self.disabled);

        if s.button("Add Item")? {
            self.items.push(format!(
                "{}. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam",
                self.items.len() + 1
            ));
        }
        if s.button("Remove Item")? {
            self.items.pop();
        }

        s.select_list("Items", &self.items, 14, &mut self.selected)?;

        s.text_field("Input: ", &mut self.input)?;
        s.wrap_width(200);
        s.text(format!("Output: {}", self.input))?;
        s.no_wrap();

        s.disable(false);
        s.checkbox("Disabled", &mut self.disabled)?;
        s.disable(self.disabled);

        s.same_line(None);
        s.checkbox("Ipsum", &mut self.check)?;
        s.same_line(None);
        s.tooltip("A hot tooltip")?;

        s.radio("Red", &mut self.radio, 0)?;
        s.radio("Green", &mut self.radio, 1)?;
        s.radio("Blue", &mut self.radio, 2)?;

        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(800, 600)
        .with_title("GUI Demo")
        .position_centered()
        .with_frame_rate()
        .build();
    let mut app = Gui::new();
    engine.run(&mut app)
}
