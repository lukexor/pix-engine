use pix_engine::{
    gui::{element::Text, selection::ListSelection, Drawable},
    PixEngine, PixEngineResult, State, StateData,
};
use std::path::PathBuf;

struct Gui {
    selection: ListSelection,
    paths: Vec<PathBuf>,
}

impl Gui {
    fn new() -> Self {
        Self {
            selection: ListSelection::new(),
            paths: Vec::new(),
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
        let mut items: Vec<Text> = self
            .paths
            .iter()
            .filter_map(|p| p.file_name())
            .filter_map(|s| s.to_str())
            .map(|s| Text::new(&s))
            .collect();
        items.insert(0, Text::new("../"));
        self.selection = ListSelection::with_items(10, 10, 800, 600 - 10, items);
        Ok(true)
    }
    fn on_update(&mut self, _elapsed: f32, data: &mut StateData) -> PixEngineResult<bool> {
        self.selection.update(data);
        self.selection.draw(data);
        // let events = data.poll_events();
        // if !events.is_empty() {
        //     println!("{:?}", events);
        // }
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
