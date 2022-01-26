//! Trait for allowing [`PixEngine`] to drive your application.
//!
//! # Example
//!
//! ```no_run
//! use pix_engine::prelude::*;
//!
//! struct MyApp;
//!
//! impl AppState for MyApp {
//!     fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Setup App state. `PixState` contains engine specific state and
//!         // utility functions for things like getting mouse coordinates,
//!         // drawing shapes, etc.
//!         s.background(220);
//!         s.font_family(Font::NOTO)?;
//!         s.font_size(16);
//!         Ok(())
//!     }
//!
//!     fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Main render loop. Called as often as possible, or based on `target frame rate`.
//!         if s.mouse_pressed() {
//!             s.fill(0);
//!         } else {
//!             s.fill(255);
//!         }
//!         let m = s.mouse_pos();
//!         s.circle([m.x(), m.y(), 80])?;
//!         Ok(())
//!     }
//!
//!     fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
//!         // Teardown any state or resources before exiting.
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> PixResult<()> {
//!     let mut engine = PixEngine::builder()
//!       .with_dimensions(800, 600)
//!       .with_title("MyApp")
//!       .build()?;
//!     let mut app = MyApp;
//!     engine.run(&mut app)
//! }
//! ```

use crate::prelude::*;

/// Trait for allowing the [`PixEngine`] to drive your application and send notification of events,
/// passing along a [`&mut PixState`](PixState) to allow interacting with the [`PixEngine`].
///
/// Please see the [module-level documentation] for more examples.
///
/// [module-level documentation]: crate::appstate
#[allow(unused_variables)]
pub trait AppState {
    /// Called once upon engine start when [`PixEngine::run`] is called.
    ///
    /// This can be used to set up initial state like creating objects, loading files or [Image]s, or
    /// any additional application state that's either dynamic or relies on runtime values from
    /// [`PixState`].
    ///
    /// # Errors
    ///
    /// Returning an error will immediately exit the application and call [`AppState::on_stop`].
    /// [`PixEngine::run`] will return the original error or any error returned from
    /// [`AppState::on_stop`]. Calling [`PixState::abort_quit`] during [`AppState::on_stop`] has no
    /// effect.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     s.background(220);
    ///     s.font_family(Font::NOTO)?;
    ///     s.font_size(16);
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called after [`AppState::on_start`], every frame based on the [target frame rate].
    ///
    /// By default, this is called as often as possible but can be controlled by changing the
    /// [target frame rate]. It will continue to be executed until the application is terminated,
    /// or [`PixState::no_run`] is called.
    ///
    /// After [`PixState::no_run`] is called, you can call [`PixState::redraw`] or
    /// [`PixState::run_times`] to control the execution.
    ///
    /// [target frame rate]: PixState::frame_rate
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`].
    /// [`PixEngine::run`] will return the original error or any error returned from
    /// [`AppState::on_stop`]. Calling [`PixState::abort_quit`] during [`AppState::on_stop`] will
    /// allow program execution to resume. Care should be taken as this could result in a loop if
    /// the cause of the original error is not resolved.
    ///
    /// Any errors encountered from methods on [`PixState`] can either be handled by the
    /// implementor, or propagated with the [?] operator.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     if s.mouse_pressed() {
    ///         s.fill(0);
    ///     } else {
    ///         s.fill(255);
    ///     }
    ///     let m = s.mouse_pos();
    ///     s.circle([m.x(), m.y(), 80])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()>;

    /// Called when the engine detects a close/exit event such as calling [`PixState::quit`] or if an
    /// error is returned during program execution by any [`AppState`] methods.
    ///
    /// This can be used to clean up files or resources on appliation quit.
    ///
    /// # Errors
    ///
    /// Returning an error will immediately exit the application by propagating the error and returning from
    /// [`PixEngine::run`]. Calling [`PixState::abort_quit`] will allow program execution to
    /// resume. Care should be taken as this could result in a loop if the cause of the original
    /// error is not resolved.
    ///
    /// Any errors encountered from methods on [`PixState`] can either be handled by the
    /// implementor, or propagated with the [?] operator.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { resources: std::path::PathBuf }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     std::fs::remove_file(&self.resources)?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }

    /// Called each time a [`Key`] is pressed with the [`KeyEvent`] indicating which key and modifiers
    /// are pressed as well as whether this is a repeat event where the key is being held down.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Return if event.keymod == KeyMod::CTRL => {
    ///             s.fullscreen(true);
    ///             Ok(true)
    ///         },
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    fn on_key_pressed(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Key`] is pressed with the [`KeyEvent`] indicating which key and modifiers
    /// are released.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App { fn fire_bullet(&mut self, s: &mut PixState) {} }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_released(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
    ///     match event.key {
    ///         Key::Space => {
    ///             self.fire_bullet(s);
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    fn on_key_released(&mut self, s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time text input is received.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text: String };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_key_typed(&mut self, s: &mut PixState, text: &str) -> PixResult<bool> {
    ///     self.text.push_str(text);
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_key_typed(&mut self, s: &mut PixState, text: &str) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time the [`Mouse`] is moved while any mouse button is being held.
    ///
    /// You can inspect which button is being held by calling [`PixState::mouse_down`] with the desired
    /// [Mouse] button. See also: [`AppState::on_mouse_motion`].
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { pos: PointI2 };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_dragged(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     pos: PointI2,
    ///     rel_pos: PointI2,
    /// ) -> PixResult<bool> {
    ///     self.pos = pos;
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_dragged(
        &mut self,
        s: &mut PixState,
        pos: PointI2,
        rel_pos: PointI2,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`ControllerButton`] is pressed with the [`ControllerEvent`] indicating
    /// which button is pressed.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App { fn pause(&mut self) {} }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_controller_pressed(&mut self, s: &mut PixState, event: ControllerEvent) -> PixResult<bool> {
    ///     match event.button {
    ///         ControllerButton::Start => {
    ///             self.pause();
    ///             Ok(true)
    ///         },
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    fn on_controller_pressed(
        &mut self,
        s: &mut PixState,
        event: ControllerEvent,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`ControllerButton`] is pressed with the [`ControllerEvent`] indicating
    /// which key and modifiers are released.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App { fn fire_bullet(&mut self, s: &mut PixState) {} }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_controller_released(&mut self, s: &mut PixState, event: ControllerEvent) -> PixResult<bool> {
    ///     match event.button {
    ///         ControllerButton::X => {
    ///             self.fire_bullet(s);
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    fn on_controller_released(
        &mut self,
        s: &mut PixState,
        event: ControllerEvent,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a `Controller` `Axis` is moved with the delta since last frame.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { player1: ControllerId }
    /// # impl App {
    /// #   fn move_right(&self) {}
    /// #   fn move_left(&self) {}
    /// #   fn move_up(&self) {}
    /// #   fn move_down(&self) {}
    /// # }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_controller_axis_motion(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     controller_id: ControllerId,
    ///     axis: Axis,
    ///     value: i32,
    /// ) -> PixResult<bool> {
    ///     if controller_id == self.player1 {
    ///         match axis {
    ///             Axis::LeftX => {
    ///                 if value > 0 {
    ///                     self.move_right();
    ///                 } else if value < 0 {
    ///                     self.move_left();
    ///                 }
    ///                 Ok(true)
    ///             }
    ///             Axis::LeftY => {
    ///                 if value > 0 {
    ///                     self.move_up();
    ///                 } else if value < 0 {
    ///                     self.move_down();
    ///                 }
    ///                 Ok(true)
    ///             }
    ///             _ => Ok(false)
    ///         }
    ///     } else {
    ///         Ok(false)
    ///     }
    /// }
    /// # }
    /// ```
    fn on_controller_axis_motion(
        &mut self,
        s: &mut PixState,
        controller_id: ControllerId,
        axis: Axis,
        value: i32,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a `Controller` is added, removed, or remapped.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App {
    /// # fn add_gamepad(&mut self, _: ControllerId) {}
    /// # fn remove_gamepad(&mut self, _: ControllerId) {}
    /// # }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_controller_update(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     controller_id: ControllerId,
    ///     update: ControllerUpdate,
    /// ) -> PixResult<bool> {
    ///     match update {
    ///         ControllerUpdate::Added => {
    ///             self.add_gamepad(controller_id);
    ///             Ok(true)
    ///         }
    ///         ControllerUpdate::Removed => {
    ///             self.remove_gamepad(controller_id);
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    fn on_controller_update(
        &mut self,
        s: &mut PixState,
        controller_id: ControllerId,
        update: ControllerUpdate,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Mouse`] button is pressed.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { canvas: Rect<i32>, drawing: bool };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_pressed(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: PointI2,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.canvas.contains_point(pos) {
    ///             self.drawing = true;
    ///         }
    ///     }
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_pressed(&mut self, s: &mut PixState, btn: Mouse, pos: PointI2) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Mouse`] button is released.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { drawing: bool, canvas: Rect<i32> };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_released(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: PointI2,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.canvas.contains_point(pos) {
    ///             self.drawing = false;
    ///         }
    ///     }
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_released(&mut self, s: &mut PixState, btn: Mouse, pos: PointI2) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Mouse`] button is clicked (a press followed by a release).
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { item: Rect<i32>, selected: bool };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_clicked(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: PointI2,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.item.contains_point(pos) {
    ///             self.selected = true;
    ///         }
    ///     }
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_clicked(&mut self, s: &mut PixState, btn: Mouse, pos: PointI2) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Mouse`] button is clicked twice within 500ms.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { item: Rect<i32> };
    /// # impl App { fn execute_item(&mut self) {} }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_dbl_clicked(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: PointI2,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.item.contains_point(pos) {
    ///             self.execute_item()
    ///         }
    ///     }
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_dbl_clicked(
        &mut self,
        s: &mut PixState,
        btn: Mouse,
        pos: PointI2,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time the [`Mouse`] is moved with the `(x, y)` screen coordinates and relative
    /// `(xrel, yrel)` positions since last frame.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { pos: PointI2 };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_motion(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     pos: PointI2,
    ///     rel_pos: PointI2,
    /// ) -> PixResult<bool> {
    ///     self.pos = pos;
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_motion(
        &mut self,
        s: &mut PixState,
        pos: PointI2,
        rel_pos: PointI2,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time the [`Mouse`] wheel is scrolled with the `(x, y)` delta since last frame.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { scroll: PointI2 };
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_wheel(&mut self, s: &mut PixState, pos: PointI2) -> PixResult<bool> {
    ///     self.scroll += pos;
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_wheel(&mut self, s: &mut PixState, pos: PointI2) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a window event occurs.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { window_id: WindowId };
    /// # impl App { fn pause(&mut self) {} fn unpause(&mut self) {} }
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_window_event(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     window_id: WindowId,
    ///     event: WindowEvent,
    /// ) -> PixResult<()> {
    ///     if window_id == self.window_id {
    ///         match event {
    ///             WindowEvent::Minimized => self.pause(),
    ///             WindowEvent::Restored => self.unpause(),
    ///             _ => (),
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn on_window_event(
        &mut self,
        s: &mut PixState,
        window_id: WindowId,
        event: WindowEvent,
    ) -> PixResult<()> {
        Ok(())
    }

    /// Called for any system or user event. This is a catch-all for handling any events not
    /// covered by other [`AppState`] methods. Prefer using the more specialized methods where
    /// possible, as it allows you to consume them, preventing the engine from encountering
    /// event-handling collisions.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`AppState::on_stop`]. See
    /// the `Errors` section in [`AppState::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl AppState for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_event(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     event: &Event,
    /// ) -> PixResult<()> {
    ///     match event {
    ///         Event::ControllerDown { controller_id, button } => {
    ///             // Handle controller down event
    ///         }
    ///         Event::ControllerUp { controller_id, button } => {
    ///             // Handle controller up event
    ///         }
    ///         _ => (),
    ///     }
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn on_event(&mut self, s: &mut PixState, event: &Event) -> PixResult<()> {
        Ok(())
    }
}
