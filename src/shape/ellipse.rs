pub enum ArcMode {
    Pie,
    Open,
    Chord,
}

impl Default for ArcMode {
    fn default() -> Self {
        Self::Pie
    }
}
