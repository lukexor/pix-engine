use pix_engine::prelude::*;

struct Gui {
    disabled: bool,
    button_clicked: bool,
    checkbox: bool,
    radio: usize,
    text_field: String,
    text_field_hint: String,
    text_field_filtered: String,
    text_area: String,
    text_area_hint: String,
    text_area_filtered: String,
    select_box: usize,
    select_list: usize,
}

impl Gui {
    fn new() -> Self {
        Self {
            disabled: false,
            button_clicked: false,
            checkbox: true,
            radio: 0,
            text_field: "Hello, world!".into(),
            text_field_hint: String::new(),
            text_field_filtered: String::new(),
            text_area: "Hello, world!".into(),
            text_area_hint: String::new(),
            text_area_filtered: String::new(),
            select_box: 0,
            select_list: 0,
        }
    }
}

impl AppState for Gui {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        if self.disabled {
            s.disable();
        }

        s.text(format!("Mouse: {}", s.mouse_pos()))?;
        s.spacing();
        s.text("Widgets:")?;
        s.separator()?;

        if s.button("Button")? {
            self.button_clicked = !self.button_clicked;
        }
        if self.button_clicked {
            s.same_line([0, 8]);
            s.text("Clicked!")?;
        }

        s.checkbox("Checkbox", &mut self.checkbox)?;

        s.radio("Radio 1", &mut self.radio, 0)?;
        s.same_line(None);
        s.radio("Radio 2", &mut self.radio, 1)?;
        s.same_line(None);
        s.radio("Radio 3", &mut self.radio, 2)?;

        // Tooltips
        s.text("Hover me")?;
        if s.hovered() {
            s.tooltip("A hot tooltip")?;
        }
        s.same_line(None);
        s.text("- and me!")?;
        if s.hovered() {
            s.advanced_tooltip(200, 100, |s: &mut PixState| {
                s.background(s.accent_color())?;
                s.text("Advanced tip")?;
                s.separator()?;
                s.text("Draw all the things!")?;
                Ok(())
            })?;
        }

        s.indent()?;
        s.bullet("Bulleted text indented")?;

        s.separator()?;

        s.push();
        s.stroke(s.accent_color());
        s.font_size(20)?;
        s.stroke_weight(2);
        s.font_style(FontStyle::BOLD | FontStyle::ITALIC);
        s.text("Outlined Bold Italicized Text!")?;
        s.spacing();
        s.pop();

        // Text Fields
        s.next_width(200);
        s.text_field("Text Field", &mut self.text_field)?;
        s.same_line([0, 4]);
        s.help_marker(
            "CTRL-X, CTRL-C, CTRL-V to use the clipboard.\n\
            ALT-Backspace to delete word.\n\
            CTRL-Backspace to clear.\n\
            (CTRL and ALT are mapped to CMD and OPTION on macOs)",
        )?;

        s.next_width(200);
        s.text_field_hint(
            "Text Field w/ hint",
            "placeholder text",
            &mut self.text_field_hint,
        )?;

        s.next_width(200);
        s.text_field_filtered(
            "Filtered Text Field",
            &mut self.text_field_filtered,
            char::is_numeric,
        )?;
        s.same_line([0, 4]);
        s.help_marker("Filters any non-numeric characters")?;

        // Text Areas
        s.text_area("Text Area", 200, 100, &mut self.text_area)?;
        s.same_line(None);
        s.help_marker(
            "CTRL-X, CTRL-C, CTRL-V to use the clipboard.\n\
            ALT-Backspace to delete word.\n\
            CTRL-Backspace to clear.\n\
            RETURN to enter newline.\n\
            (CTRL and ALT are mapped to CMD and OPTION on macOs)",
        )?;
        s.same_line(None);
        s.text_area_hint(
            "Text Area w/ hint",
            "placeholder text",
            200,
            100,
            &mut self.text_area_hint,
        )?;
        s.same_line(None);
        s.text_area_filtered(
            "Text Area Filtered",
            200,
            100,
            &mut self.text_area_filtered,
            char::is_alphabetic,
        )?;
        s.same_line(None);
        s.help_marker("Filters any non-alphabetic characters")?;

        s.separator()?;

        // Selectables
        let items = [
            "Bulbasaur",
            "Charmander",
            "Squirtle",
            "Caterpie",
            "Weedle",
            "Pidgey",
            "Pikachu",
            "Rattata",
        ];
        s.next_width(150);
        s.select_box("Select Box", &mut self.select_box, &items)?;

        s.next_width(300);
        s.select_list("Select List", &mut self.select_list, &items, 4)?;

        s.no_disable();
        s.checkbox("Disable Elements", &mut self.disabled)?;
        if self.disabled {
            s.disable();
        }

        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(1000, 800)
        .with_title("GUI Demo")
        .with_frame_rate()
        .with_font(ARIAL, 14)
        .build()?;
    let mut app = Gui::new();
    engine.run(&mut app)
}
