use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Timer {
    start: Option<Instant>,
    end: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.end = Some(Instant::now());
    }

    pub fn elapsed(&self) -> f32 {
        match (self.start, self.end) {
            (Some(start), Some(end)) if end > start => (end - start).as_secs_f32(),
            (Some(start), _) => (Instant::now() - start).as_secs_f32(),
            _ => 0.0,
        }
    }
}
