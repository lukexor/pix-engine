use crate::{
    common::Scalar,
    renderer::Renderer,
    state_data::{StateData, StateDataResult},
};

impl StateData {
    /// Draw a series of lines.
    pub fn triangle(
        &mut self,
        x1: Scalar,
        y1: Scalar,
        x2: Scalar,
        y2: Scalar,
        x3: Scalar,
        y3: Scalar,
    ) -> StateDataResult<()> {
        if let Some(c) = self.get_fill() {
            self.renderer.triangle(x1, y1, x2, y2, x3, y3, c)?;
        }
        Ok(())
    }
}
