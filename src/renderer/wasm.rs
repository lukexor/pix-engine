use crate::{
    renderer::{Renderer, RendererOpts},
    PixEngineResult,
};
use wasm_bindgen::prelude::*;

pub(crate) struct WasmRenderer {}

impl WasmRenderer {
    pub(crate) fn new(_opts: RendererOpts) -> PixEngineResult<Self> {
        Ok(Self {})
    }
}

impl Renderer for WasmRenderer {}
