use pix_engine::{
    draw::Rect,
    gui::{element::Text, selection::ListSelection, Drawable},
    pixel::Pixel,
    PixEngine, PixEngineResult, State, StateData,
};
use std::path::PathBuf;

struct Gui {
    selection: ListSelection,
    paths: Vec<PathBuf>,
    items: Vec<Text>,
}

impl Gui {
    fn new() -> Self {
        Self {
            selection: ListSelection::new(),
            paths: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl State for Gui {
    fn on_start(&mut self, _data: &mut StateData) -> PixEngineResult<bool> {
        self.paths = vec![
            PathBuf::from("/Users/caeledh/dir1"),
            PathBuf::from("/Users/caeledh/file.txt"),
        ];
        for i in 0..30 {
            self.paths
                .push(PathBuf::from(&format!("/Users/filename.{}", i)));
        }
        self.items = self
            .paths
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|s| s.to_str())
            .map(|s| Text::new(&s))
            .collect();
        self.items.insert(0, Text::new("../"));
        self.selection = ListSelection::with_items(Rect::new(100, 100, 200, 400), &self.items);
        Ok(true)
    }
    fn on_update(&mut self, _elapsed: f32, data: &mut StateData) -> PixEngineResult<bool> {
        data.set_draw_color(Pixel::very_dark_gray());
        data.fill_rect(Rect::new(0, 0, 800, 600));
        self.selection.update(data);
        let _ = data.poll_events();
        Ok(true)
    }
    fn on_stop(&mut self, _data: &mut StateData) -> PixEngineResult<bool> {
        Ok(true)
    }
}

pub fn main() {
    let gui = Gui::new();
    let mut engine =
        PixEngine::new("Gui Example".to_string(), gui, 800, 600).expect("valid engine");
    engine.run().expect("engine run");
}
