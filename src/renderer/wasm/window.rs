use super::Renderer;
use crate::renderer::WindowRenderer;

impl WindowRenderer for Renderer {
    fn window_count(&self) -> usize {
        todo!()
    }

    fn window_id(&self) -> crate::prelude::WindowId {
        todo!()
    }

    fn create_window(
        &mut self,
        s: &mut crate::renderer::RendererSettings,
    ) -> crate::prelude::PixResult<crate::prelude::WindowId> {
        todo!()
    }

    fn close_window(&mut self, id: crate::prelude::WindowId) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn cursor(&mut self, cursor: Option<&crate::prelude::Cursor>) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn poll_event(&mut self) -> Option<crate::event::Event> {
        todo!()
    }

    fn title(&self) -> &str {
        todo!()
    }

    fn set_title(&mut self, title: &str) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn set_fps(&mut self, fps: f32) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn dimensions(&self) -> crate::prelude::PixResult<(u32, u32)> {
        todo!()
    }

    fn window_dimensions(&self) -> crate::prelude::PixResult<(u32, u32)> {
        todo!()
    }

    fn set_window_dimensions(&mut self, dimensions: (u32, u32)) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn viewport(&self) -> crate::prelude::PixResult<crate::prelude::Rect<i32>> {
        todo!()
    }

    fn set_viewport(
        &mut self,
        rect: Option<crate::prelude::Rect<i32>>,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn display_dimensions(&self) -> crate::prelude::PixResult<(u32, u32)> {
        todo!()
    }

    fn fullscreen(&self) -> crate::prelude::PixResult<bool> {
        todo!()
    }

    fn set_fullscreen(&mut self, val: bool) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn vsync(&self) -> bool {
        todo!()
    }

    fn set_vsync(&mut self, val: bool) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn set_window_target(&mut self, id: crate::prelude::WindowId) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn reset_window_target(&mut self) {
        todo!()
    }

    fn show(&mut self) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn hide(&mut self) -> crate::prelude::PixResult<()> {
        todo!()
    }
}
