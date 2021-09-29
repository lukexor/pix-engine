//! Trait for allowing [PixEngine] to drive your application.

use crate::prelude::*;

/// Trait for allowing the [PixEngine] to drive your application and send notification of events,
/// passing along a [`&mut PixState`](PixState) to allow interacting with the [PixEngine].
///
/// [PixEngine]: crate::prelude::PixEngine
#[allow(unused_variables)]
pub trait AppState {
    /// Called once upon engine start when [PixEngine::run] is called.
    ///
    /// This can be used to set up initial state like creating objects, loading files or [Image]s, or
    /// any additional application state that's either dynamic or relies on runtime values from
    /// [PixState].
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called after [AppState::on_start], every frame based on the [target frame rate].
    ///
    /// By default, this is called as often as possible but can be controlled by changing the
    /// [target frame rate]. It will continue to be executed until the application is terminated,
    /// or [PixState::no_run] is called.
    ///
    /// After [PixState::no_run] is called, you can call [PixState::redraw] or
    /// [PixState::run_times] to control the execution.
    ///
    /// [target frame rate]: PixState::set_frame_rate
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()>;

    /// Called once when the engine detects a close/exit event such as calling [PixState::quit].
    ///
    /// This can be used to clean up resources, or quit confirmation messages.
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Key] is pressed with the [KeyEvent] indicating which key and modifiers
    /// are pressed as well as whether this is a repeat event where the key is being held down.
    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Key] is pressed with the [KeyEvent] indicating which key and modifiers
    /// are released.
    fn on_key_released(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Key] is typed with the text that was typed. Ignores special keys like [Key::Backspace].
    fn on_key_typed(&mut self, s: &mut PixState, text: &str) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the [Mouse] is moved while any mouse button is being held.
    ///
    /// You can inspect which button is being held by calling [PixState::mouse_down] with the desired
    /// [Mouse] button. See also: [AppState::on_mouse_motion].
    fn on_mouse_dragged(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Mouse] button is pressed.
    fn on_mouse_pressed(&mut self, s: &mut PixState, btn: Mouse, pos: PointI2) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Mouse] button is released.
    fn on_mouse_released(&mut self, s: &mut PixState, btn: Mouse, pos: PointI2) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Mouse] button is clicked (a press followed by a release).
    fn on_mouse_clicked(&mut self, s: &mut PixState, btn: Mouse, pos: PointI2) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [Mouse] button is clicked twice within 500ms.
    fn on_mouse_dbl_clicked(
        &mut self,
        s: &mut PixState,
        btn: Mouse,
        pos: PointI2,
    ) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the [Mouse] is moved with the `(x, y)` screen coordinates and relative
    /// `(xrel, yrel)` positions since last frame.
    fn on_mouse_motion(
        &mut self,
        s: &mut PixState,
        pos: PointI2,
        xrel: i32,
        yrel: i32,
    ) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the [Mouse] wheel is scrolled with the `(x, y)` delta since last frame.
    fn on_mouse_wheel(&mut self, s: &mut PixState, pos: PointI2) -> PixResult<()> {
        Ok(())
    }

    /// Called each time the window is resized with the new `(width, height)`.
    fn on_window_resized(
        &mut self,
        s: &mut PixState,
        window_id: WindowId,
        width: i32,
        height: i32,
    ) -> PixResult<()> {
        Ok(())
    }

    /// Called for any system or user event.
    fn on_event(&mut self, s: &mut PixState, event: Event) -> PixResult<()> {
        Ok(())
    }
}
