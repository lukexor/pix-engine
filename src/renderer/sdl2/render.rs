use crate::renderer::BlendMode;
use sdl2::render;

impl From<render::BlendMode> for BlendMode {
    fn from(mode: render::BlendMode) -> Self {
        use render::BlendMode::*;
        match mode {
            None => Self::None,
            Blend => Self::Blend,
            Add => Self::Add,
            Mod => Self::Mod,
            Invalid => Self::Invalid,
        }
    }
}

impl From<BlendMode> for render::BlendMode {
    fn from(mode: BlendMode) -> Self {
        use BlendMode::*;
        match mode {
            None => Self::None,
            Blend => Self::Blend,
            Add => Self::Add,
            Mod => Self::Mod,
            Invalid => Self::Invalid,
        }
    }
}
