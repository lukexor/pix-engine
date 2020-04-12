use crate::{
    draw::{Point, Rect},
    event::PixEvent,
    pixel::{ColorType, Pixel},
    PixEngineResult, WindowId,
};

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(super) mod sdl2;
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(super) mod wasm;

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(super) fn load_renderer(opts: RendererOpts) -> PixEngineResult<sdl2::Sdl2Renderer> {
    sdl2::Sdl2Renderer::new(opts)
}
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(super) fn load_renderer(opts: RendererOpts) -> PixEngineResult<wasm::WasmRenderer> {
    wasm::WasmRenderer::new(opts)
}

// TODO Add RendererErr and RendererResult types
pub(super) trait Renderer {
    fn fullscreen(&mut self, _val: bool) -> PixEngineResult<()> {
        Ok(())
    }
    fn vsync(&mut self, _val: bool) -> PixEngineResult<()> {
        Ok(())
    }
    fn load_icon(&mut self, _path: &str) -> PixEngineResult<()> {
        Ok(())
    }
    fn window_id(&self) -> WindowId {
        0
    }
    fn set_title(&mut self, _window_id: WindowId, _title: &str) -> PixEngineResult<()> {
        Ok(())
    }
    fn set_size(&mut self, _window_id: WindowId, _width: u32, _height: u32) -> PixEngineResult<()> {
        Ok(())
    }
    fn set_audio_sample_rate(&mut self, _sample_rate: i32) -> PixEngineResult<()> {
        Ok(())
    }
    fn poll(&mut self) -> PixEngineResult<Vec<PixEvent>> {
        Ok(Vec::new())
    }
    fn clear(&mut self) -> PixEngineResult<()> {
        Ok(())
    }
    fn clear_window(&mut self, _window_id: WindowId) -> PixEngineResult<()> {
        Ok(())
    }
    fn present(&mut self) {}
    fn create_texture(
        &mut self,
        _window_id: WindowId,
        _name: &str,
        _color_type: ColorType,
        _src: Rect,
        _dst: Rect,
    ) -> PixEngineResult<()> {
        Ok(())
    }
    fn copy_texture(
        &mut self,
        _window_id: WindowId,
        _name: &str,
        _bytes: &[u8],
    ) -> PixEngineResult<()> {
        Ok(())
    }
    fn open_window(&mut self, _title: &str, _width: u32, _height: u32) -> PixEngineResult<u32> {
        Ok(1)
    }
    fn close_window(&mut self, _window_id: WindowId) {}
    fn enqueue_audio(&mut self, _samples: &[f32]) {}
    fn set_draw_color(&mut self, _p: Pixel) -> PixEngineResult<()> {
        Ok(())
    }
    fn fill_rect(&mut self, _rect: Rect) -> PixEngineResult<()> {
        Ok(())
    }
    fn draw_point(&mut self, _point: Point) -> PixEngineResult<()> {
        Ok(())
    }
    fn set_viewport(&mut self, _rect: Option<Rect>) -> PixEngineResult<()> {
        Ok(())
    }
}

pub(super) struct RendererOpts {
    title: String,
    width: u32,
    height: u32,
    audio_sample_rate: Option<i32>,
}

impl RendererOpts {
    pub(super) fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_owned(),
            width,
            height,
            audio_sample_rate: None,
        }
    }
}
