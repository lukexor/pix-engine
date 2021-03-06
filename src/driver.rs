use crate::{draw::Rect, event::PixEvent, pixel::ColorType, PixEngineResult, WindowId};

#[cfg(all(feature = "sdl2-driver", not(feature = "wasm-driver")))]
pub(super) mod sdl2;
#[cfg(all(feature = "wasm-driver", not(feature = "sdl2-driver")))]
pub(super) mod wasm;

#[cfg(all(feature = "sdl2-driver", not(feature = "wasm-driver")))]
pub(super) fn load_driver(opts: DriverOpts) -> PixEngineResult<sdl2::Sdl2Driver> {
    sdl2::Sdl2Driver::new(opts)
}
#[cfg(all(feature = "wasm-driver", not(feature = "sdl2-driver")))]
pub(super) fn load_driver(opts: DriverOpts) -> PixEngineResult<wasm::WasmDriver> {
    wasm::WasmDriver::new(opts)
}

// TODO Add DriverErr and DriverResult types
pub(super) trait Driver {
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
    fn clear(&mut self, _window_id: WindowId) -> PixEngineResult<()> {
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
}

pub(super) struct DriverOpts {
    title: String,
    width: u32,
    height: u32,
    audio_sample_rate: Option<i32>,
    vsync: bool,
}

impl DriverOpts {
    pub(super) fn new(title: &str, width: u32, height: u32, vsync: bool) -> Self {
        Self {
            title: title.to_owned(),
            width,
            height,
            audio_sample_rate: None,
            vsync,
        }
    }
}
