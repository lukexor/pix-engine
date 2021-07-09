//! Trait for implementing [PixEngine] methods in your application.

use crate::prelude::*;

/// Trait for implementing methods the [PixEngine] will call while running and handling events,
/// passing along a [`&mut PixState`](PixState) to allow interacting with the [PixEngine].
///
/// [PixEngine]: crate::prelude::PixEngine
#[allow(unused_variables)]
pub trait AppState {
    /// Called once upon engine start when [PixEngine::run] is called.
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called every frame based on the [target frame rate](PixState::set_frame_rate). By
    /// default this is as often as possible.
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()>;

    /// Called once when the engine detects a close/exit event.
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Key] is pressed.
    fn on_key_pressed(&mut self, s: &mut PixState, _event: KeyEvent) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Key] is released.
    fn on_key_released(&mut self, s: &mut PixState, _event: KeyEvent) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Key] is typed. Ignores special keys like [Key::Backspace].
    fn on_key_typed(&mut self, s: &mut PixState, _text: &str) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Mouse] button is pressed.
    fn on_mouse_dragged(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Mouse] button is pressed.
    fn on_mouse_pressed(&mut self, s: &mut PixState, _btn: Mouse) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Mouse] button is released.
    fn on_mouse_released(&mut self, s: &mut PixState, _btn: Mouse) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the mouse wheel is scrolled.
    fn on_mouse_wheel(&mut self, s: &mut PixState, _x_delta: i32, _y_delta: i32) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the window is resized.
    fn on_window_resized(&mut self, s: &mut PixState, _width: i32, height: i32) -> PixResult<()> {
        Ok(())
    }
}
