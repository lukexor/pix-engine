//! Primary [`PixEngine`] trait and functions which drive your application.
//!
//! This is the core module of the `pix-engine` crate and is responsible for building and running
//! any application using it.
//!
//! [`EngineBuilder`] allows you to customize various engine features and, once built, can
//! [run][`Engine::run`] your application which must implement [`PixEngine::on_update`].
//!
//!
//!
//! # Example
//!
//! ```no_run
//! use pix_engine::prelude::*;
//!
//! struct MyApp;
//!
//! impl PixEngine for MyApp {
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
//!             s.fill(color!(0));
//!         } else {
//!             s.fill(color!(255));
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
//!     let mut app = MyApp;
//!     let mut engine = Engine::builder()
//!       .dimensions(800, 600)
//!       .title("MyApp")
//!       .build()?;
//!     engine.run(&mut app)
//! }
//! ```

use crate::{image::Icon, prelude::*, renderer::RendererSettings};
use log::{debug, error, info};
use std::{
    num::NonZeroUsize,
    thread,
    time::{Duration, Instant},
};

/// Trait for allowing the [`Engine`] to drive your application and send notification of events,
/// passing along a [`&mut PixState`](PixState) to allow interacting with the [`Engine`].
///
/// Please see the [module-level documentation] for more examples.
///
/// [module-level documentation]: crate::engine
#[allow(unused_variables)]
pub trait PixEngine {
    /// Called once upon engine start when [`Engine::run`] is called.
    ///
    /// This can be used to set up initial state like creating objects, loading files or [Image]s, or
    /// any additional application state that's either dynamic or relies on runtime values from
    /// [`PixState`].
    ///
    /// # Errors
    ///
    /// Returning an error will immediately exit the application and call [`PixEngine::on_stop`].
    /// [`Engine::run`] will return the original error or any error returned from
    /// [`PixEngine::on_stop`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
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

    /// Called after [`PixEngine::on_start`], every frame based on the [target frame rate].
    ///
    /// By default, this is called as often as possible but can be controlled by changing the
    /// [target frame rate]. It will continue to be executed until the application is terminated,
    /// or [`PixState::run(false)`] is called.
    ///
    /// After [`PixState::run(false)`] is called, you can call [`PixState::redraw`] or
    /// [`PixState::run_times`] to control the execution.
    ///
    /// [target frame rate]: PixState::frame_rate
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`].
    /// [`Engine::run`] will return the original error or any error returned from
    /// [`PixEngine::on_stop`]. Calling [`PixState::abort_quit`] during [`PixEngine::on_stop`] will
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
    /// # impl PixEngine for App {
    /// fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
    ///     if s.mouse_pressed() {
    ///         s.fill(color!(0));
    ///     } else {
    ///         s.fill(color!(255));
    ///     }
    ///     let m = s.mouse_pos();
    ///     s.circle([m.x(), m.y(), 80])?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()>;

    /// Called when the engine detects a close/exit event such as calling [`PixState::quit`] or if an
    /// error is returned during program execution by any [`PixEngine`] methods.
    ///
    /// This can be used to clean up files or resources on appliation quit.
    ///
    /// # Errors
    ///
    /// Returning an error will immediately exit the application by propagating the error and returning from
    /// [`Engine::run`]. Calling [`PixState::abort_quit`] will allow program execution to
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
    /// # impl PixEngine for App {
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App { fn fire_bullet(&mut self, s: &mut PixState) {} }
    /// # impl PixEngine for App {
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { text: String };
    /// # impl PixEngine for App {
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
    /// [Mouse] button. See also: [`PixEngine::on_mouse_motion`].
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { pos: Point<i32> };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_dragged(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     pos: Point<i32>,
    ///     rel_pos: Point<i32>,
    /// ) -> PixResult<bool> {
    ///     self.pos = pos;
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_dragged(
        &mut self,
        s: &mut PixState,
        pos: Point<i32>,
        rel_pos: Point<i32>,
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App { fn pause(&mut self) {} }
    /// # impl PixEngine for App {
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl App { fn fire_bullet(&mut self, s: &mut PixState) {} }
    /// # impl PixEngine for App {
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
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
    /// # impl PixEngine for App {
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
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
    /// # impl PixEngine for App {
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { canvas: Rect<i32>, drawing: bool };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_pressed(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: Point<i32>,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.canvas.contains(pos) {
    ///             self.drawing = true;
    ///         }
    ///     }
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_pressed(
        &mut self,
        s: &mut PixState,
        btn: Mouse,
        pos: Point<i32>,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Mouse`] button is released.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { drawing: bool, canvas: Rect<i32> };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_released(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: Point<i32>,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.canvas.contains(pos) {
    ///             self.drawing = false;
    ///         }
    ///     }
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_released(
        &mut self,
        s: &mut PixState,
        btn: Mouse,
        pos: Point<i32>,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Mouse`] button is clicked (a press followed by a release).
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { item: Rect<i32>, selected: bool };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_clicked(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: Point<i32>,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.item.contains(pos) {
    ///             self.selected = true;
    ///         }
    ///     }
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_clicked(
        &mut self,
        s: &mut PixState,
        btn: Mouse,
        pos: Point<i32>,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a [`Mouse`] button is clicked twice within 500ms.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { item: Rect<i32> };
    /// # impl App { fn execute_item(&mut self) {} }
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_dbl_clicked(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     btn: Mouse,
    ///     pos: Point<i32>,
    /// ) -> PixResult<bool> {
    ///     if let Mouse::Left = btn {
    ///         if self.item.contains(pos) {
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
        pos: Point<i32>,
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
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { pos: Point<i32> };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_motion(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     pos: Point<i32>,
    ///     rel_pos: Point<i32>,
    /// ) -> PixResult<bool> {
    ///     self.pos = pos;
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_motion(
        &mut self,
        s: &mut PixState,
        pos: Point<i32>,
        rel_pos: Point<i32>,
    ) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time the [`Mouse`] wheel is scrolled with the `(x, y)` delta since last frame.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { scroll: Point<i32> };
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_mouse_wheel(&mut self, s: &mut PixState, pos: Point<i32>) -> PixResult<bool> {
    ///     self.scroll += pos;
    ///     Ok(true)
    /// }
    /// # }
    /// ```
    fn on_mouse_wheel(&mut self, s: &mut PixState, pos: Point<i32>) -> PixResult<bool> {
        Ok(false)
    }

    /// Called each time a window event occurs.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App { window_id: WindowId };
    /// # impl App { fn pause(&mut self) {} fn unpause(&mut self) {} }
    /// # impl PixEngine for App {
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
    /// covered by other [`PixEngine`] methods.
    ///
    /// Returning `true` consumes this event, preventing any further event triggering.
    ///
    /// # Errors
    ///
    /// Returning an error will start exiting the application and call [`PixEngine::on_stop`]. See
    /// the `Errors` section in [`PixEngine::on_update`] for more details.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// # struct App;
    /// # impl PixEngine for App {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// fn on_event(
    ///     &mut self,
    ///     s: &mut PixState,
    ///     event: &Event,
    /// ) -> PixResult<bool> {
    ///     match event {
    ///         Event::ControllerDown { controller_id, button } => {
    ///             // Handle controller down event
    ///             Ok(true)
    ///         }
    ///         Event::ControllerUp { controller_id, button } => {
    ///             // Handle controller up event
    ///             Ok(true)
    ///         }
    ///         _ => Ok(false),
    ///     }
    /// }
    /// # }
    /// ```
    fn on_event(&mut self, s: &mut PixState, event: &Event) -> PixResult<bool> {
        Ok(false)
    }
}

/// Builds a [`Engine`] instance by providing several configration functions.
///
/// # Example
///
/// ```no_run
/// # use pix_engine::prelude::*;
/// # struct MyApp;
/// # impl PixEngine for MyApp {
/// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
/// # }
/// fn main() -> PixResult<()> {
///     let mut engine = Engine::builder()
///         .title("My App")
///         .position(10, 10)
///         .resizable()
///         .show_frame_rate()
///         .icon("myapp.png")
///         .build()?;
///     let mut app = MyApp;
///     engine.run(&mut app)
/// }
/// ```
#[must_use]
#[derive(Debug)]
pub struct EngineBuilder {
    settings: RendererSettings,
    theme: Theme,
    joystick_deadzone: i32,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            settings: RendererSettings::default(),
            theme: Theme::default(),
            joystick_deadzone: 8000,
        }
    }
}

impl EngineBuilder {
    /// Constructs a `EngineBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a window title.
    pub fn title<S>(&mut self, title: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.settings.title = title.into();
        self
    }

    /// Set font for text rendering.
    pub fn font(&mut self, font: Font) -> &mut Self {
        self.theme.fonts.body = font;
        self
    }

    /// Set font size for text rendering.
    pub fn font_size(&mut self, size: u32) -> &mut Self {
        self.theme.font_size = size;
        self
    }

    /// Set theme for UI rendering.
    pub fn theme(&mut self, theme: Theme) -> &mut Self {
        self.theme = theme;
        self
    }

    /// Set a window icon.
    pub fn icon<I>(&mut self, icon: I) -> &mut Self
    where
        I: Into<Icon>,
    {
        self.settings.icon = Some(icon.into());
        self
    }

    /// Position the window at the given `(x, y)` coordinates of the display.
    pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
        self.settings.x = Position::Positioned(x);
        self.settings.y = Position::Positioned(y);
        self
    }

    /// Position the window in the center of the display. This is the default.
    pub fn position_centered(&mut self) -> &mut Self {
        self.settings.x = Position::Centered;
        self.settings.y = Position::Centered;
        self
    }

    /// Set window dimensions.
    pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    /// Set the rendering scale of the current canvas. Drawing coordinates are scaled by x/y
    /// factors before being drawn to the canvas.
    pub fn scale(&mut self, x: f32, y: f32) -> &mut Self {
        self.settings.scale_x = x;
        self.settings.scale_y = y;
        self
    }

    /// Set audio sample rate in Hz (samples per second). Defaults to device fallback sample rate.
    pub fn audio_sample_rate(&mut self, sample_rate: i32) -> &mut Self {
        self.settings.audio_sample_rate = Some(sample_rate);
        self
    }

    /// Set number of audio channels (1 for Mono, 2 for Stereo, etc). Defaults to device fallback
    /// number of channels.
    pub fn audio_channels(&mut self, channels: u8) -> &mut Self {
        self.settings.audio_channels = Some(channels);
        self
    }

    /// Set audio buffer size in samples. Defaults to device fallback sample size.
    pub fn audio_buffer_size(&mut self, buffer_size: u16) -> &mut Self {
        self.settings.audio_buffer_size = Some(buffer_size);
        self
    }

    /// Start window in fullscreen mode.
    pub fn fullscreen(&mut self) -> &mut Self {
        self.settings.fullscreen = true;
        self
    }

    /// Set the window to synchronize frame rate to the screens refresh rate ([`VSync`]).
    ///
    /// [`VSync`]: https://en.wikipedia.org/wiki/Screen_tearing#Vertical_synchronization
    pub fn vsync_enabled(&mut self) -> &mut Self {
        self.settings.vsync = true;
        self
    }

    /// Allow window resizing.
    pub fn resizable(&mut self) -> &mut Self {
        self.settings.resizable = true;
        self
    }

    /// Removes the window decoration.
    pub fn borderless(&mut self) -> &mut Self {
        self.settings.borderless = true;
        self
    }

    /// Alter the joystick axis deadzone.
    pub fn deadzone(&mut self, value: i32) -> &mut Self {
        self.joystick_deadzone = value;
        self
    }

    /// Enables high-DPI on displays that support it.
    pub fn allow_highdpi(&mut self) -> &mut Self {
        self.settings.allow_highdpi = true;
        self
    }

    /// Starts engine with window hidden.
    pub fn hidden(&mut self) -> &mut Self {
        self.settings.hidden = true;
        self
    }

    /// Enable average frame rate (FPS) in title.
    pub fn show_frame_rate(&mut self) -> &mut Self {
        self.settings.show_frame_rate = true;
        self
    }

    /// Set a target frame rate to render at, controls how often
    /// [`PixEngine::on_update`] is called.
    pub fn target_frame_rate(&mut self, rate: usize) -> &mut Self {
        self.settings.target_frame_rate = Some(rate);
        self
    }

    /// Set a custom texture cache size other than the default of `20`.
    /// Affects font family and image rendering caching operations.
    pub fn texture_cache(&mut self, size: NonZeroUsize) -> &mut Self {
        self.settings.texture_cache_size = size;
        self
    }

    /// Set a custom text cache size other than the default of `500`.
    /// Affects text rendering caching operations.
    pub fn text_cache(&mut self, size: NonZeroUsize) -> &mut Self {
        self.settings.text_cache_size = size;
        self
    }

    /// Convert [EngineBuilder] to a [`Engine`] instance.
    ///
    /// # Errors
    ///
    /// If the engine fails to create a new renderer, then an error is returned.
    ///
    /// Possible errors include the title containing a `nul` character, the position or dimensions
    /// being invalid values or overlowing and an internal renderer error such as running out of
    /// memory or a software driver issue.
    pub fn build(&self) -> PixResult<Engine> {
        Ok(Engine {
            state: PixState::new(self.settings.clone(), self.theme.clone())?,
            joystick_deadzone: self.joystick_deadzone,
        })
    }
}

/// The core engine that maintains the render loop, state, drawing functions, event handling, etc.
#[must_use]
#[derive(Debug)]
pub struct Engine {
    state: PixState,
    joystick_deadzone: i32,
}

impl Engine {
    /// Constructs a default [EngineBuilder] which can build a `Engine` instance.
    ///
    /// See [EngineBuilder] for examples.
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    /// Starts the `Engine` application and begins executing the frame loop on a given
    /// application which must implement [`Engine`]. The only required method of which is
    /// [`PixEngine::on_update`].
    ///
    /// # Errors
    ///
    /// Any error in the entire library can propagate here and terminate the program. See the
    /// [error](crate::error) module for details. Also see [`PixEngine::on_stop`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pix_engine::prelude::*;
    /// # struct MyApp;
    /// # impl MyApp { fn new() -> Self { Self } }
    /// # impl PixEngine for MyApp {
    /// # fn on_update(&mut self, s: &mut PixState) -> PixResult<()> { Ok(()) }
    /// # }
    /// fn main() -> PixResult<()> {
    ///     let mut engine = Engine::builder().build()?;
    ///     let mut app = MyApp::new(); // MyApp implements `Engine`
    ///     engine.run(&mut app)
    /// }
    /// ```
    pub fn run<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: PixEngine,
    {
        info!("Starting `Engine`...");

        // Handle events before on_start to initialize window
        self.handle_events(app)?;

        debug!("Starting with `PixEngine::on_start`");
        self.state.clear()?;
        let on_start = app.on_start(&mut self.state);
        if on_start.is_err() || self.state.should_quit() {
            debug!("Quitting during startup with `PixEngine::on_stop`");
            if let Err(ref err) = on_start {
                error!("Error: {}", err);
            }
            return app.on_stop(&mut self.state).and(on_start);
        }
        self.state.present();

        // on_stop loop enables on_stop to prevent application close if necessary
        'on_stop: loop {
            debug!("Starting `PixEngine::on_update` loop.");
            // running loop continues until an event or on_update returns false or errors
            let result = 'running: loop {
                let start_time = Instant::now();
                let time_since_last = start_time - self.state.last_frame_time();

                self.handle_events(app)?;
                if self.state.should_quit() {
                    break 'running Ok(());
                }

                if self.state.is_running() {
                    self.state.pre_update();
                    let on_update = app.on_update(&mut self.state);
                    if on_update.is_err() {
                        self.state.quit();
                        break 'running on_update;
                    }
                    self.state.on_update()?;
                    self.state.post_update();
                    self.state.present();
                    self.state.set_delta_time(start_time, time_since_last);
                    self.state.increment_frame(time_since_last)?;
                }

                if !self.state.vsync_enabled() {
                    if let Some(target_delta_time) = self.state.target_delta_time() {
                        let time_to_next_frame = start_time + target_delta_time;
                        let now = Instant::now();
                        if time_to_next_frame > now {
                            thread::sleep(time_to_next_frame - now);
                        }
                    }
                }
            };

            debug!("Quitting with `PixEngine::on_stop`");
            let on_stop = app.on_stop(&mut self.state);
            if self.state.should_quit() {
                info!("Quitting `Engine`...");
                break 'on_stop on_stop.and(result);
            }
        }
    }
}

impl Engine {
    /// Handle user and system events.
    #[inline]
    fn handle_events<A>(&mut self, app: &mut A) -> PixResult<()>
    where
        A: PixEngine,
    {
        let state = &mut self.state;
        while let Some(event) = state.poll_event() {
            if let Event::ControllerAxisMotion { .. }
            | Event::JoyAxisMotion { .. }
            | Event::MouseMotion { .. }
            | Event::MouseWheel { .. }
            | Event::KeyDown { repeat: true, .. }
            | Event::KeyUp { repeat: true, .. } = event
            {
                // Ignore noisy events
            } else {
                debug!("Polling event {:?}", event);
            }
            let handled = app.on_event(state, &event)?;
            if !handled {
                match event {
                    Event::Quit | Event::AppTerminating => state.quit(),
                    Event::Window {
                        window_id,
                        win_event,
                    } => {
                        let window_id = WindowId(window_id);
                        match win_event {
                            WindowEvent::FocusGained => state.focus_window(Some(window_id)),
                            WindowEvent::FocusLost => state.focus_window(None),
                            WindowEvent::Close => state.close_window(window_id)?,
                            _ => (),
                        }
                        app.on_window_event(state, window_id, win_event)?;
                    }
                    Event::KeyDown {
                        key: Some(key),
                        keymod,
                        repeat,
                        scan: Some(scan),
                    } => {
                        let evt = KeyEvent::new(key, keymod, repeat, scan);
                        if !app.on_key_pressed(state, evt)? {
                            state.ui.keys.press(key, keymod);
                        }
                    }
                    Event::KeyUp {
                        key: Some(key),
                        keymod,
                        repeat,
                        scan: Some(scan),
                    } => {
                        let evt = KeyEvent::new(key, keymod, repeat, scan);
                        if !app.on_key_released(state, evt)? {
                            state.ui.keys.release(key, keymod);
                        }
                    }
                    Event::ControllerDown {
                        controller_id,
                        button,
                    } => {
                        let evt = ControllerEvent::new(controller_id, button);
                        app.on_controller_pressed(state, evt)?;
                    }
                    Event::ControllerUp {
                        controller_id,
                        button,
                    } => {
                        let evt = ControllerEvent::new(controller_id, button);
                        app.on_controller_released(state, evt)?;
                    }
                    Event::ControllerAxisMotion {
                        controller_id,
                        axis,
                        value,
                    } => {
                        let value = i32::from(value);
                        let value =
                            if (-self.joystick_deadzone..self.joystick_deadzone).contains(&value) {
                                0
                            } else {
                                value
                            };
                        let id = ControllerId(controller_id);
                        app.on_controller_axis_motion(state, id, axis, value)?;
                    }
                    Event::ControllerAdded { controller_id } => {
                        let id = ControllerId(controller_id);
                        if !app.on_controller_update(state, id, ControllerUpdate::Added)? {
                            state.open_controller(id)?;
                        }
                    }
                    Event::JoyDeviceAdded { joy_id } => {
                        let id = ControllerId(joy_id);
                        if !app.on_controller_update(state, id, ControllerUpdate::Added)? {
                            state.open_controller(id)?;
                        }
                    }
                    Event::ControllerRemoved { controller_id } => {
                        let id = ControllerId(controller_id);
                        if !app.on_controller_update(state, id, ControllerUpdate::Removed)? {
                            state.close_controller(id);
                        }
                    }
                    Event::JoyDeviceRemoved { joy_id } => {
                        let id = ControllerId(joy_id);
                        if !app.on_controller_update(state, id, ControllerUpdate::Removed)? {
                            state.close_controller(id);
                        }
                    }
                    Event::ControllerRemapped { controller_id } => {
                        let id = ControllerId(controller_id);
                        app.on_controller_update(state, id, ControllerUpdate::Remapped)?;
                    }
                    Event::TextInput { text, .. } => {
                        if !app.on_key_typed(state, &text)? {
                            state.ui.keys.typed(text);
                        }
                    }
                    Event::MouseMotion { x, y, xrel, yrel } => {
                        let pos = point!(x, y);
                        let rel_pos = point!(xrel, yrel);
                        if state.ui.mouse.is_pressed() {
                            app.on_mouse_dragged(state, pos, rel_pos)?;
                        }
                        if !app.on_mouse_motion(state, pos, rel_pos)? {
                            state.on_mouse_motion(pos);
                        }
                    }
                    Event::MouseDown { button, x, y } => {
                        if !app.on_mouse_pressed(state, button, point!(x, y))? {
                            state.on_mouse_pressed(button);
                        }
                    }
                    Event::MouseUp { button, x, y } => {
                        if state.ui.mouse.is_down(button) {
                            let now = Instant::now();
                            if let Some(clicked) = state.ui.mouse.last_clicked(button) {
                                if now - *clicked < Duration::from_millis(500)
                                    && !app.on_mouse_dbl_clicked(state, button, point!(x, y))?
                                {
                                    state.on_mouse_dbl_click(button, now);
                                }
                            }
                            if !app.on_mouse_clicked(state, button, point!(x, y))? {
                                state.on_mouse_click(button, now);
                            }
                        }
                        if !app.on_mouse_released(state, button, point!(x, y))? {
                            state.on_mouse_released(button);
                        }
                    }
                    Event::MouseWheel { x, y, .. } => {
                        if !app.on_mouse_wheel(state, point!(x, y))? {
                            state.on_mouse_wheel(x, y);
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
}
