//! Shows the live state of every connected controller.
//!
//! Buttons light up while held and axes are drawn as bars, so a gamepad can be checked against
//! the engine's mapping without reading logs. Verifying stick direction needs a real pad, so this
//! example exists to make that check quick: push a stick up or left and the bar should fill to
//! the left of centre, which is the direction the engine reports as negative.

use pix_engine::prelude::*;
use std::collections::{BTreeMap, HashMap};

const WIDTH: u32 = 900;
const HEIGHT: u32 = 720;

/// Margin from the window edge.
const PAD: i32 = 12;

/// Left edge of the axis bars, leaving room for the label and value.
const BAR_X: i32 = 210;

/// Width of an axis bar.
const BAR_W: i32 = 300;

/// Height of an axis bar.
const BAR_H: i32 = 14;

/// Vertical step between axis rows.
const ROW_H: i32 = 20;

/// Face, shoulder and d-pad buttons, drawn in this order.
const BUTTONS: [(ControllerButton, &str); 15] = [
    (ControllerButton::A, "A"),
    (ControllerButton::B, "B"),
    (ControllerButton::X, "X"),
    (ControllerButton::Y, "Y"),
    (ControllerButton::Back, "Back"),
    (ControllerButton::Guide, "Guide"),
    (ControllerButton::Start, "Start"),
    (ControllerButton::LeftStick, "LStick"),
    (ControllerButton::RightStick, "RStick"),
    (ControllerButton::LeftShoulder, "LShldr"),
    (ControllerButton::RightShoulder, "RShldr"),
    (ControllerButton::DPadUp, "Up"),
    (ControllerButton::DPadDown, "Down"),
    (ControllerButton::DPadLeft, "Left"),
    (ControllerButton::DPadRight, "Right"),
];

/// Axes, drawn in this order.
///
/// The flag marks an axis that rests at centre and travels both ways. A trigger rests at zero and
/// only climbs, so it fills from the left instead.
const AXES: [(Axis, &str, bool); 6] = [
    (Axis::LeftX, "LeftX", true),
    (Axis::LeftY, "LeftY", true),
    (Axis::RightX, "RightX", true),
    (Axis::RightY, "RightY", true),
    (Axis::TriggerLeft, "TrigL", false),
    (Axis::TriggerRight, "TrigR", false),
];

/// What one controller is doing right now.
#[derive(Default)]
struct ControllerState {
    /// Buttons currently held.
    pressed: Vec<ControllerButton>,
    /// Latest position of each axis, in the range the event hook reports.
    axes: HashMap<&'static str, i32>,
}

struct ControllerDemo {
    /// State for each controller the engine has reported, ordered by id so rows stay put.
    controllers: BTreeMap<u32, ControllerState>,
}

impl ControllerDemo {
    fn new() -> Self {
        Self {
            controllers: BTreeMap::new(),
        }
    }

    /// Draws one controller starting at `top`, returning the next free y coordinate.
    fn draw_controller(
        s: &mut PixState,
        id: u32,
        state: &ControllerState,
        top: i32,
    ) -> PixResult<i32> {
        s.set_cursor_pos([PAD, top]);
        s.fill(Color::WHITE);
        s.text(format!("Controller {id}"))?;
        let mut y = s.cursor_pos().y() + 4;

        // Buttons wrap at the window edge. Sizing each one from its own label keeps the row tight
        // whatever font the theme is using.
        let (_, text_h) = s.size_of("A")?;
        let button_h = text_h as i32 + 6;
        let mut x = PAD;
        for (button, label) in BUTTONS {
            let (text_w, _) = s.size_of(label)?;
            let button_w = text_w as i32 + 10;
            if x + button_w > WIDTH as i32 - PAD {
                x = PAD;
                y += button_h + 4;
            }
            s.fill(if state.pressed.contains(&button) {
                Color::GREEN
            } else {
                Color::DARK_SLATE_GRAY
            });
            s.rect([x, y, button_w, button_h])?;
            s.fill(Color::WHITE);
            s.set_cursor_pos([x + 5, y + 3]);
            s.text(label)?;
            x += button_w + 4;
        }
        y += button_h + 10;

        for (_axis, label, bipolar) in AXES {
            let value = state.axes.get(label).copied().unwrap_or(0);
            s.fill(Color::WHITE);
            s.set_cursor_pos([PAD, y]);
            s.text(format!("{label:>6} {value:>7}"))?;

            s.fill(Color::DARK_SLATE_GRAY);
            s.rect([BAR_X, y, BAR_W, BAR_H])?;
            s.fill(Color::CYAN);
            if bipolar {
                let half = BAR_W / 2;
                let offset = (value * half) / 32767;
                let (fill_x, fill_w) = if offset >= 0 {
                    (BAR_X + half, offset)
                } else {
                    (BAR_X + half + offset, -offset)
                };
                s.rect([fill_x, y, fill_w, BAR_H])?;
                // A tick at rest position, so a drifting stick is obvious.
                s.fill(Color::GRAY);
                s.rect([BAR_X + half - 1, y, 2, BAR_H])?;
            } else {
                let fill_w = (value.max(0) * BAR_W) / 32767;
                s.rect([BAR_X, y, fill_w, BAR_H])?;
            }
            y += ROW_H;
        }
        Ok(y + 10)
    }
}

impl PixEngine for ControllerDemo {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.clear()?;
        // Settings persist across frames, so the fill is set rather than inherited from whatever
        // the last bar drew.
        s.fill(Color::WHITE);
        if self.controllers.is_empty() {
            s.set_cursor_pos([PAD, PAD]);
            s.text("Connect a controller.")?;
            return Ok(());
        }
        let mut y = PAD;
        for (id, state) in &self.controllers {
            if y > HEIGHT as i32 - 40 {
                s.set_cursor_pos([PAD, y]);
                s.text("More controllers than fit on screen.")?;
                break;
            }
            y = Self::draw_controller(s, *id, state, y)?;
        }
        Ok(())
    }

    fn on_controller_update(
        &mut self,
        _s: &mut PixState,
        controller_id: ControllerId,
        update: ControllerUpdate,
    ) -> PixResult<bool> {
        let id = *controller_id;
        match update {
            ControllerUpdate::Added | ControllerUpdate::Remapped => {
                log::info!("controller {id} attached");
                self.controllers.entry(id).or_default();
            }
            ControllerUpdate::Removed => {
                log::info!("controller {id} detached");
                self.controllers.remove(&id);
            }
        }
        // `false` lets the engine open or close the controller for us.
        Ok(false)
    }

    fn on_controller_pressed(
        &mut self,
        _s: &mut PixState,
        event: ControllerEvent,
    ) -> PixResult<bool> {
        let state = self.controllers.entry(*event.controller_id).or_default();
        if !state.pressed.contains(&event.button) {
            state.pressed.push(event.button);
        }
        Ok(true)
    }

    fn on_controller_released(
        &mut self,
        _s: &mut PixState,
        event: ControllerEvent,
    ) -> PixResult<bool> {
        let state = self.controllers.entry(*event.controller_id).or_default();
        state.pressed.retain(|button| *button != event.button);
        Ok(true)
    }

    fn on_controller_axis_motion(
        &mut self,
        _s: &mut PixState,
        controller_id: ControllerId,
        axis: Axis,
        value: i32,
    ) -> PixResult<bool> {
        if let Some((_, label, _)) = AXES.iter().find(|(known, _, _)| *known == axis) {
            let state = self.controllers.entry(*controller_id).or_default();
            state.axes.insert(label, value);
        }
        Ok(true)
    }
}

fn main() -> PixResult<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut engine = Engine::builder()
        .dimensions(WIDTH, HEIGHT)
        .title("Controller")
        .show_frame_rate()
        .target_frame_rate(60)
        .build()?;
    let mut app = ControllerDemo::new();
    engine.run(&mut app)
}
