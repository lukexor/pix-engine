use crate::prelude::*;

/// A Web-Assembly [Renderer] implementation.
pub(crate) struct Renderer {}

impl std::fmt::Debug for Renderer {
    #[doc(hidden)]
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("WasmRenderer {{}}")
    }
}
