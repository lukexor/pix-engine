use crate::{renderer::Renderer, state::StateData};

impl StateData {
    pub fn enqueue_audio(&mut self, samples: &[f32]) {
        self.renderer.enqueue_audio(samples);
    }
}
