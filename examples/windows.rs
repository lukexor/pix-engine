use pix_engine::prelude::*;

struct WindowDemo {
    windows: Vec<(WindowId, Color)>,
}

impl WindowDemo {
    fn new() -> Self {
        Self { windows: vec![] }
    }
}

impl AppState for WindowDemo {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.fill(Color::WHITE);
        s.text("Window 1")?;

        if s.button("Open Window")? {
            let (w, h) = s.display_dimensions()?;
            let window_id = s
                .window()
                .with_dimensions(random!(200, 500), random!(200, 500))
                .with_title(format!("Window {}", self.windows.len() + 1))
                .position(random!(w) as i32, random!(h) as i32)
                .build()?;
            self.windows.push((window_id, Color::random()));
        }

        if s.button("Close Windows")? {
            for &(window_id, _) in &self.windows {
                s.close_window(window_id)?;
            }
            self.windows.clear();
        }

        for &(window_id, color) in &self.windows {
            s.with_window(window_id, |s: &mut PixState| {
                s.background(color);
                s.fill(color.inverted());
                s.text(format!("Window {}", window_id))?;
                Ok(())
            })?;
        }
        Ok(())
    }

    fn on_window_event(
        &mut self,
        _s: &mut PixState,
        window_id: WindowId,
        evt: WindowEvent,
    ) -> PixResult<()> {
        if let WindowEvent::Close = evt {
            self.windows.retain(|&(id, _)| id != window_id);
        }
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(400, 400)
        .with_title("Window 1")
        .position(10, 10)
        .build()?;
    let mut app = WindowDemo::new();
    engine.run(&mut app)
}
