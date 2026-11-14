//! The `winit` application that drives the frame loop.
//!
//! `winit` owns the loop and calls back, so [`Engine::run`] hands control to it and the frame body
//! lives in [`ApplicationHandler::about_to_wait`], which fires once every pass of the loop. Windows
//! are created in the callbacks too, because only an [`ActiveEventLoop`] can create one.

use crate::{
    bench::Bench,
    engine::{Engine, PixEngine},
    error::Result,
    time::Instant,
};
use anyhow::Context;
use log::{debug, error, info};
use std::cell::RefCell;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    platform::run_on_demand::EventLoopExtRunOnDemand,
};

thread_local! {
    /// The one event loop a process gets.
    ///
    /// `winit` refuses to build a second one, and [`Engine::run`] can be called again after it
    /// returns, so the loop is built on the first run and handed back on every one after.
    static EVENT_LOOP: RefCell<Option<EventLoop<()>>> = const { RefCell::new(None) };
}

/// Runs `app` until it quits.
///
/// # Errors
///
/// Returns the first error an engine callback reported, or an error from the event loop itself.
pub(crate) fn run<A>(engine: &mut Engine, app: &mut A) -> Result<()>
where
    A: PixEngine,
{
    info!("Starting `Engine`...");
    let mut handler = Handler {
        engine,
        app,
        bench: Bench::from_env(),
        started: false,
        stopping: false,
        frame_start: Instant::now(),
        next_frame: Instant::now(),
        result: Ok(()),
    };
    EVENT_LOOP.with_borrow_mut(|slot| {
        let event_loop = match slot.as_mut() {
            Some(event_loop) => event_loop,
            None => slot.insert(EventLoop::new().context("failed to start the event loop")?),
        };
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop
            .run_app_on_demand(&mut handler)
            .context("event loop failed")
    })?;
    info!("Quitting `Engine`...");
    handler.result
}

/// Engine state and the application, driven by the event loop.
struct Handler<'a, A> {
    /// Engine the frame loop runs against.
    engine: &'a mut Engine,
    /// Application whose callbacks the loop calls.
    app: &'a mut A,
    /// Frame timing harness, present when `PIX_BENCH_FRAMES` asked for one.
    bench: Option<Bench>,
    /// Whether [`PixEngine::on_start`] has run.
    started: bool,
    /// Whether the loop has been told to exit, so [`PixEngine::on_stop`] runs once.
    stopping: bool,
    /// When the last frame began. The target frame rate is measured from here.
    frame_start: Instant,
    /// Earliest the next frame may run.
    next_frame: Instant,
    /// First error a callback reported, returned once the loop exits.
    result: Result<()>,
}

impl<A> Handler<'_, A>
where
    A: PixEngine,
{
    /// Runs one frame.
    ///
    /// Errors from an event hook take the same path as errors from [`PixEngine::on_update`], so
    /// [`PixEngine::on_stop`] still runs and can release resources.
    fn update(&mut self) {
        let start_time = Instant::now();
        self.frame_start = start_time;
        let time_since_last = start_time - self.engine.state.last_frame_time();

        if let Err(err) = self.engine.handle_events(self.app) {
            self.engine.state.quit();
            self.result = Err(err);
            return;
        }
        if self.engine.state.should_quit() || !self.engine.state.is_running() {
            return;
        }

        self.engine.state.pre_update();
        if let Err(err) = self.app.on_update(&mut self.engine.state) {
            self.engine.state.quit();
            self.result = Err(err);
            return;
        }
        if let Err(err) = self.engine.state.on_update().and_then(|()| {
            self.engine.state.post_update();
            self.engine.state.present();
            self.engine
                .state
                .set_delta_time(start_time, time_since_last);
            self.engine.state.increment_frame(time_since_last)
        }) {
            self.engine.state.quit();
            self.result = Err(err);
            return;
        }

        // Sampled before the pacing below. See `crate::bench`.
        if let Some(active) = self.bench.as_mut() {
            if active.record(start_time.elapsed()) {
                active.report(self.engine.state.vsync_enabled());
                self.bench = None;
                self.engine.state.quit();
            }
        }
    }

    /// Asks the application to stop, and reports whether it agreed.
    ///
    /// [`PixEngine::on_stop`] can call [`PixState::abort_quit`] to keep running, which is why the
    /// answer is read back from the state rather than assumed.
    ///
    /// [`PixState::abort_quit`]: crate::prelude::PixState::abort_quit
    fn stop(&mut self) -> bool {
        // The loop can pass through `about_to_wait` again after it is told to exit, and the
        // application is asked to stop once.
        if self.stopping {
            return true;
        }
        debug!("Quitting with `PixEngine::on_stop`");
        if let Err(err) = self.app.on_stop(&mut self.engine.state) {
            if self.result.is_ok() {
                self.result = Err(err);
            }
            self.stopping = true;
            return true;
        }
        self.stopping = self.engine.state.should_quit();
        self.stopping
    }

    /// Reports whether the next frame is due.
    ///
    /// The loop wakes for reasons of its own, a compositor frame callback among them, so a target
    /// frame rate is enforced here rather than by the wake-up time alone.
    fn frame_due(&self) -> bool {
        // With vsync the present call blocks until the display is ready, so the loop paces itself.
        self.engine.state.vsync_enabled()
            || self.engine.state.target_delta_time().is_none()
            || Instant::now() >= self.next_frame
    }

    /// Chooses when the loop should wake for the next frame.
    fn pace(&mut self, event_loop: &ActiveEventLoop) {
        if self.engine.state.vsync_enabled() {
            event_loop.set_control_flow(ControlFlow::Poll);
            return;
        }
        let Some(delta) = self.engine.state.target_delta_time() else {
            event_loop.set_control_flow(ControlFlow::Poll);
            return;
        };
        // Measured from the start of the frame, so the work the frame did counts towards the
        // period rather than being added to it. A frame that overran leaves the deadline in the
        // past, and the next one runs at once rather than trying to catch up.
        self.next_frame = self.frame_start + delta;
        let now = Instant::now();
        if self.next_frame <= now {
            self.next_frame = now;
            event_loop.set_control_flow(ControlFlow::Poll);
        } else {
            event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
        }
    }
}

impl<A> ApplicationHandler for Handler<'_, A>
where
    A: PixEngine,
{
    /// Creates the windows and runs [`PixEngine::on_start`] the first time through.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(err) = self
            .engine
            .state
            .renderer
            .create_pending_windows(event_loop)
        {
            error!("Error: {err}");
            self.result = Err(err);
            event_loop.exit();
            return;
        }
        if self.started {
            return;
        }
        self.started = true;

        debug!("Starting with `PixEngine::on_start`");
        let started = self
            .engine
            .state
            .clear()
            .and_then(|()| self.app.on_start(&mut self.engine.state));
        if let Err(err) = started {
            error!("Error: {err}");
            self.result = Err(err);
            self.engine.state.quit();
        }
        if self.engine.state.should_quit() {
            debug!("Quitting during startup with `PixEngine::on_stop`");
            if self.stop() {
                event_loop.exit();
                return;
            }
        }
        self.engine.state.present();
    }

    /// Releases what the event loop owns, before it tears itself down.
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.engine.state.renderer.shut_down();
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        self.engine
            .state
            .renderer
            .handle_window_event(window_id, &event);
    }

    /// Runs one frame, after creating any window opened during the last one.
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if !self.started {
            return;
        }
        if !self.frame_due() {
            self.pace(event_loop);
            return;
        }
        if let Err(err) = self
            .engine
            .state
            .renderer
            .create_pending_windows(event_loop)
        {
            error!("Error: {err}");
            if self.result.is_ok() {
                self.result = Err(err);
            }
            self.engine.state.quit();
        } else {
            self.engine.state.renderer.apply_cursor(event_loop);
            self.update();
        }
        if self.engine.state.should_quit() && self.stop() {
            event_loop.exit();
            return;
        }
        self.pace(event_loop);
    }
}
