use pix_engine::prelude::*;

const FONTS: [&str; 3] = ["Emulogic", "Noto", "Inconsolata"];
const THEMES: [&str; 2] = ["Dark", "Light"];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Category {
    Basic,
    FieldsSliders,
    SelectsCollapsables,
    Settings,
}

impl AsRef<str> for Category {
    fn as_ref(&self) -> &str {
        match self {
            Self::Basic => "Basic",
            Self::FieldsSliders => "Fields/Sliders",
            Self::SelectsCollapsables => "Selects/Collapsables",
            Self::Settings => "Settings",
        }
    }
}

struct Gui {
    disabled: bool,
    selected_category: Category,
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
    collapsing_header_list: usize,
    font_size: u32,
    font_family: usize,
    theme: usize,
    frame_padx: i32,
    frame_pady: i32,
    item_padx: i32,
    item_pady: i32,
}

impl Gui {
    fn new() -> Self {
        Self {
            disabled: false,
            selected_category: Category::Basic,
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
            collapsing_header_list: 0,
            font_size: 12,
            font_family: 0,
            theme: 0,
            frame_padx: 8,
            frame_pady: 8,
            item_padx: 8,
            item_pady: 6,
        }
    }

    fn basic_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
        if s.button("Button")? {
            self.button_clicked = !self.button_clicked;
        }
        if self.button_clicked {
            s.same_line([0, s.theme().spacing.item_pad.y()]);
            s.text("Clicked!")?;
        }

        s.text("A")?;
        s.same_line(None);
        if s.link("URL Link")? {
            s.open_url("https://github.com/lukexor/pix-engine")?;
        }
        s.same_line(None);
        s.text("in a sentence.")?;

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
                rect![s.mouse_pos(), 300, 100],
                |s: &mut PixState| {
                    let colors = s.theme().colors;
                    s.background(colors.secondary);
                    s.fill(colors.on_secondary);
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
        s.text_transformed("Flipped", None, None, Flipped::Both)?;
        if s.hovered() {
            s.tooltip("Flipped")?;
        }
        s.angle_mode(AngleMode::Degrees);
        s.text_transformed("Rotated", 30.0, None, None)?;

        s.font_style(FontStyle::BOLD);
        s.text("Bolded")?;
        s.font_style(FontStyle::ITALIC);
        s.text("Italicized")?;
        s.font_style(FontStyle::UNDERLINE);
        s.text("Underlined")?;
        s.font_style(FontStyle::STRIKETHROUGH);
        s.text("Strikethrough")?;

        s.font_style(FontStyle::NORMAL);
        s.push();
        let colors = s.theme().colors;
        s.stroke(colors.secondary_variant);
        s.stroke_weight(2);
        s.text("Outlined")?;
        s.pop();

        s.heading("Heading")?;
        s.monospace("Monospace")?;

        s.indent()?;
        s.text("Indented")?;
        s.bullet("Bullet")?;

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
        s.same_line(None);
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
        s.same_line([8, 0]);
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
        s.same_line([8, 0]);
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

    fn select_and_tree_widgets(&mut self, s: &mut PixState) -> PixResult<()> {
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

        // Scroll area
        s.scroll_area("Scroll Area", 300, 200, |s: &mut PixState| {
            for i in 0..10 {
                s.text(format!("{}: Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam", i))?;
            }
            Ok(())
        })?;

        s.same_line(None);
        s.next_width(300);
        s.select_list(
            "Select List",
            &mut self.select_list,
            &items,
            displayed_count,
        )?;

        s.same_line(None);
        s.next_width(200);
        s.select_box("Select Box", &mut self.select_box, &items, displayed_count)?;

        s.collapsing_tree("Basic tree", |s: &mut PixState| {
            for i in 0..3 {
                s.collapsing_tree(format!("Child {}", i), |s: &mut PixState| {
                    s.text("Some text")?;
                    Ok(())
                })?;
            }
            Ok(())
        })?;
        s.collapsing_header("Collapsing Header", |s: &mut PixState| {
            s.next_width(300);
            let items = ["Item 1", "Item 2", "Item 3", "Item 4"];
            s.select_list("Some list", &mut self.collapsing_header_list, &items, 3)?;
            Ok(())
        })?;

        Ok(())
    }

    fn settings(&mut self, s: &mut PixState) -> PixResult<()> {
        s.next_width(200);
        if s.select_box("Theme", &mut self.theme, &THEMES, THEMES.len())? {
            match THEMES[self.theme] {
                "Dark" => s.set_theme(Theme::dark()),
                "Light" => s.set_theme(Theme::light()),
                _ => unreachable!("unavailable theme"),
            }
        }

        s.next_width(200);
        if s.select_box("Font Family", &mut self.font_family, &FONTS, FONTS.len())? {
            let font = match FONTS[self.font_family] {
                "Emulogic" => Font::EMULOGIC,
                "Noto" => Font::NOTO,
                "Inconsolata" => Font::INCONSOLATA,
                _ => unreachable!("unavailable font family"),
            };
            s.font_family(font)?;
        }

        s.same_line(None);
        s.next_width(200);
        s.slider("Font Size", &mut self.font_size, 8, 25)?;
        s.same_line(None);
        if s.button("Apply")? {
            s.font_size(self.font_size)?;
        }

        s.next_width(200);
        if s.slider("Frame Padding X", &mut self.frame_padx, 0, 50)? {
            s.theme_mut().spacing.frame_pad.set_x(self.frame_padx);
        }
        s.same_line(None);
        s.next_width(200);
        if s.slider("Frame Padding Y", &mut self.frame_pady, 0, 50)? {
            s.theme_mut().spacing.frame_pad.set_y(self.frame_pady);
        }

        s.next_width(200);
        if s.slider("Item Padding X", &mut self.item_padx, 0, 50)? {
            s.theme_mut().spacing.item_pad.set_x(self.item_padx);
        }
        s.same_line(None);
        s.next_width(200);
        if s.slider("Item Padding Y", &mut self.item_pady, 0, 50)? {
            s.theme_mut().spacing.item_pad.set_y(self.item_pady);
        }

        Ok(())
    }
}

impl AppState for Gui {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;

        if self.disabled {
            s.disable();
        }

        s.heading("Widgets")?;
        s.spacing()?;

        let mut selected = self.selected_category;
        s.tab_bar(
            "Tab Bar",
            &[
                Category::Basic,
                Category::FieldsSliders,
                Category::SelectsCollapsables,
                Category::Settings,
            ],
            &mut selected,
            |tab: &Category, s: &mut PixState| {
                match tab {
                    Category::Basic => {
                        self.basic_widgets(s)?;
                        self.tooltip_widgets(s)?;
                        self.text_widgets(s)?;
                    }
                    Category::FieldsSliders => {
                        self.text_field_widgets(s)?;
                        self.drag_and_slider_widgets(s)?;
                    }
                    Category::SelectsCollapsables => self.select_and_tree_widgets(s)?,
                    Category::Settings => self.settings(s)?,
                }
                Ok(())
            },
        )?;
        self.selected_category = selected;

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
        .target_frame_rate(60)
        .build()?;
    let mut app = Gui::new();
    engine.run(&mut app)
}
