use super::{Renderer, Result};
use crate::{color::Color, event::PixEvent, state::rendering::BlendMode};

pub(crate) struct WasmRenderer {}

impl WasmRenderer {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        Ok(Self {})
    }
}

impl Renderer for WasmRenderer {
    // TODO WasmRenderer
}
