use super::Renderer;
use crate::renderer::TextureRenderer;

impl TextureRenderer for Renderer {
    fn create_texture(
        &mut self,
        width: u32,
        height: u32,
        format: Option<crate::prelude::PixelFormat>,
    ) -> crate::prelude::Result<crate::prelude::TextureId> {
        todo!()
    }

    fn delete_texture(
        &mut self,
        texture_id: crate::prelude::TextureId,
    ) -> crate::prelude::Result<()> {
        todo!()
    }

    fn update_texture<P: AsRef<[u8]>>(
        &mut self,
        texture_id: crate::prelude::TextureId,
        rect: Option<crate::prelude::Rect<i32>>,
        pixels: P,
        pitch: usize,
    ) -> crate::prelude::Result<()> {
        todo!()
    }

    fn texture(
        &mut self,
        texture_id: crate::prelude::TextureId,
        src: Option<crate::prelude::Rect<i32>>,
        dst: Option<crate::prelude::Rect<i32>>,
        angle: f64,
        center: Option<crate::prelude::Point<i32>>,
        flipped: Option<crate::prelude::Flipped>,
        tint: Option<crate::prelude::Color>,
    ) -> crate::prelude::Result<()> {
        todo!()
    }

    fn texture_target(&self) -> Option<crate::prelude::TextureId> {
        todo!()
    }

    fn set_texture_target(&mut self, texture_id: crate::prelude::TextureId) {
        todo!()
    }

    fn clear_texture_target(&mut self) {
        todo!()
    }

    fn has_texture_target(&self) -> bool {
        todo!()
    }

    fn clear_texture_cache(&mut self) {
        todo!()
    }
}
