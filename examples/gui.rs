use pix_engine::prelude::*;

struct Gui {
    disabled: bool,
    button_clicked: bool,
    text_clicked: bool,
    checkbox: bool,
    radio: usize,
    text_field: String,
    advanced_text_field: String,
    text_area: String,
    advanced_text_area: String,
    drag: i32,
    advanced_drag: f64,
    slider: i32,
    advanced_slider: f32,
    select_box: usize,
    select_list: usize,
}

impl Gui {
    fn new() -> Self {
        Self {
            disabled: false,
            button_clicked: false,
            text_clicked: false,
            checkbox: true,
            radio: 0,
            text_field: "Hello, world!".into(),
            advanced_text_field: String::new(),
            text_area: "Hello, world!".into(),
            advanced_text_area: String::new(),
            drag: 50,
            advanced_drag: 1.0,
            slider: 0,
            advanced_slider: 0.5,
            select_box: 0,
            select_list: 0,
        }
    }

    fn basic_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
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

        Ok(())
    }

    fn tooltip_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
        s.text("Hover me")?;
        if s.hovered() {
            s.tooltip("A hot tooltip")?;
        }
        s.same_line(None);
        s.text("- and me!")?;
        if s.hovered() {
            s.advanced_tooltip(
                "Advanced Tooltip",
                rect![s.mouse_pos(), 200, 100],
                |s: &mut PixState| {
                    s.background(s.accent_color())?;
                    s.text("Advanced tip")?;
                    s.separator()?;
                    s.text("Draw all the things!")?;
                    Ok(())
                },
            )?;
        }
        s.text("Click me")?;
        if s.clicked() {
            self.text_clicked = !self.text_clicked;
        }
        if self.text_clicked {
            s.same_line(None);
            s.text("Clicked!")?;
        }

        Ok(())
    }

    fn text_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
        s.text_transformed("Flipped text", None, None, Flipped::Both)?;
        if s.hovered() {
            s.tooltip("Flipped text")?;
        }
        s.spacing()?;
        s.angle_mode(AngleMode::Degrees);
        s.text_transformed("Rotated text", 90.0, None, None)?;
        s.spacing()?;

        s.indent()?;
        s.bullet("Bulleted text indented")?;

        s.push();
        s.stroke(s.accent_color());
        s.font_size(20)?;
        s.stroke_weight(2);
        s.font_style(FontStyle::BOLD | FontStyle::ITALIC);
        s.text("Outlined Bold Italicized Text!")?;
        s.pop();

        Ok(())
    }

    fn text_field_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
        s.next_width(200);
        s.text_field("Text Field", &mut self.text_field)?;
        s.same_line(None);
        s.help_marker(
            "CTRL-X, CTRL-C, CTRL-V to use the clipboard.\n\
            ALT-Backspace to delete word.\n\
            CTRL-Backspace to clear.\n\
            (CTRL and ALT are mapped to CMD and OPTION on macOs)",
        )?;

        s.next_width(200);
        s.advanced_text_field(
            "Filtered Text Field w/ hint",
            "type here",
            &mut self.advanced_text_field,
            Some(char::is_numeric),
        )?;
        s.same_line(None);
        s.help_marker("Filters any non-numeric characters")?;

        s.text_area("Text Area", 200, 100, &mut self.text_area)?;
        s.same_line(None);
        s.help_marker(
            "CTRL-X, CTRL-C, CTRL-V to use the clipboard.\n\
            ALT-Backspace to delete word.\n\
            CTRL-Backspace to clear.\n\
            RETURN to enter newline.\n\
            (CTRL and ALT are mapped to CMD and OPTION on macOs)",
        )?;
        s.same_line([8, 0]);
        s.advanced_text_area(
            "Filtered Text Area w/ hint",
            "type here",
            200,
            100,
            &mut self.advanced_text_area,
            Some(char::is_alphabetic),
        )?;
        s.same_line(None);
        s.help_marker("Filters any non-alphabetic characters")?;

        Ok(())
    }

    fn drag_and_slider_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
        // Drag bars
        s.next_width(200);
        s.drag("Drag", &mut self.drag, 1)?;
        s.same_line(None);
        s.help_marker(
            "Click and drag to edit value.\n\
            Hold SHIFT/ALT for faster/slower edit.\n\
            CTRL+click to input a value.\n\
            (CTRL is mapped to CMD on macOs)",
        )?;
        s.same_line(None);
        s.next_width(200);
        s.advanced_drag(
            "Advanced Drag",
            &mut self.advanced_drag,
            0.005,
            0.0,
            1.0,
            Some(|val| format!("{:.3}", val).into()),
        )?;

        // Sliders
        s.next_width(200);
        s.slider("Slider", &mut self.slider, -5, 5)?;
        s.same_line(None);
        s.help_marker(
            "CTRL+click to input a value.\n
            (CTRL is mapped to CMD on macOs)",
        )?;
        s.same_line(None);
        s.next_width(200);
        s.advanced_slider(
            "Advanced Slider",
            &mut self.advanced_slider,
            0.0,
            3.0,
            Some(|v| format!("{:.3}", v).into()),
        )?;

        Ok(())
    }

    fn select_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
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
        let displayed_count = 4;

        s.next_width(150);
        s.select_box("Select Box", &mut self.select_box, &items, displayed_count)?;

        s.next_width(300);
        s.select_list(
            "Select List",
            &mut self.select_list,
            &items,
            displayed_count,
        )?;

        // Scroll area
        s.scroll_area("Scroll Area", 300, 200, |s: &mut PixState| {
            for i in 0..10 {
                s.text(format!("{}: Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam", i))?;
            }
            Ok(())
        })?;

        Ok(())
    }
}

impl AppState for Gui {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        if self.disabled {
            s.disable();
        }

        s.push();
        s.font_style(FontStyle::BOLD);
        s.font_size(20)?;
        s.text("Widgets")?;
        s.pop();

        s.spacing()?;

        s.tab_bar(
            "Tab Bar",
            &["Basic", "Fields & Sliders", "Selectables"],
            |tab: &str, s: &mut PixState| {
                match tab {
                    "Basic" => {
                        self.basic_widgets(s)?;
                        self.tooltip_widgets(s)?;
                        self.text_widgets(s)?;
                    }
                    "Fields & Sliders" => {
                        self.text_field_widgets(s)?;
                        self.drag_and_slider_widgets(s)?;
                    }
                    "Selectables" => self.select_widgets(s)?,
                    _ => (),
                }
                Ok(())
            },
        )?;

        s.separator()?;

        s.no_disable();
        s.checkbox("Disable Elements", &mut self.disabled)?;
        if self.disabled {
            s.disable();
        }
        s.text(format!("Mouse: {}", s.mouse_pos()))?;

        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(1024, 768)
        .with_title("GUI Demo")
        .with_frame_rate()
        .with_font(fonts::NOTO, 14)
        .vsync_enabled()
        .build()?;
    let mut app = Gui::new();
    engine.run(&mut app)
}
