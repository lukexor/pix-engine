//! gilrs controller backend.
//!
//! winit has no gamepad support, so controllers are pumped separately from window events and
//! translated into the same [`Event`] type. [`PixState::poll_event`] drains this pump after the
//! window's, which keeps one event source for applications.

use crate::prelude::*;
use gilrs::{Axis as GilrsAxis, Button as GilrsButton, EventType, Gilrs};
use log::{debug, warn};
use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

/// Scale from the normalized range gilrs reports to the range the event API uses.
///
/// The public hook takes an `i32` in `-32767..=32767`, so applications comparing against a
/// deadzone keep working against the same numbers.
const AXIS_SCALE: f32 = 32767.0;

/// Pumps controller events and tracks which controllers an application opened.
pub(crate) struct Controllers {
    /// The gilrs context, absent when no backend could start.
    gilrs: Option<Gilrs>,
    /// Controllers an application has opened.
    ///
    /// Events arrive for every connected gamepad, and only opened ones are reported, which is the
    /// contract [`PixState::open_controller`] describes.
    opened: HashSet<ControllerId>,
    /// Events to deliver before the ones gilrs reports.
    ///
    /// gilrs raises `Connected` only for a gamepad attached after it starts, so pads already
    /// plugged in are announced from here. Without it an application that opens controllers on
    /// [`ControllerUpdate::Added`] never sees the pad someone plugged in before launching.
    pending: VecDeque<Event>,
}

impl Controllers {
    /// Starts the controller backend, falling back to one that reports nothing.
    ///
    /// A machine with no gamepad support still runs everything else, so a failure here is logged
    /// rather than propagated.
    pub(crate) fn new() -> Self {
        let gilrs = match Gilrs::new() {
            Ok(gilrs) => Some(gilrs),
            Err(err) => {
                warn!("controller support is disabled: {err}");
                None
            }
        };
        let pending = gilrs
            .iter()
            .flat_map(|gilrs| gilrs.gamepads())
            .map(|(id, _)| Event::ControllerAdded {
                controller_id: gamepad_id(id),
            })
            .collect();
        Self {
            gilrs,
            opened: HashSet::new(),
            pending,
        }
    }

    /// Starts reporting events for a controller.
    pub(crate) fn open(&mut self, id: ControllerId) {
        self.opened.insert(id);
    }

    /// Stops reporting events for a controller.
    pub(crate) fn close(&mut self, id: ControllerId) {
        self.opened.remove(&id);
    }

    /// Returns the next controller event, translated into the engine's [`Event`].
    ///
    /// Returns `None` once the pump is drained for this frame.
    pub(crate) fn poll(&mut self) -> Option<Event> {
        if let Some(event) = self.pending.pop_front() {
            return Some(event);
        }
        let gilrs = self.gilrs.as_mut()?;
        loop {
            let event = gilrs.next_event()?;
            let id = ControllerId(gamepad_id(event.id));
            match event.event {
                EventType::Connected => {
                    return Some(Event::ControllerAdded {
                        controller_id: id.0,
                    })
                }
                EventType::Disconnected | EventType::Dropped => {
                    self.opened.remove(&id);
                    return Some(Event::ControllerRemoved {
                        controller_id: id.0,
                    });
                }
                _ if !self.opened.contains(&id) => continue,
                EventType::ButtonPressed(button, _) => {
                    if let Some(button) = map_button(button) {
                        return Some(Event::ControllerDown {
                            controller_id: id.0,
                            button,
                        });
                    }
                }
                EventType::ButtonReleased(button, _) => {
                    if let Some(button) = map_button(button) {
                        return Some(Event::ControllerUp {
                            controller_id: id.0,
                            button,
                        });
                    }
                }
                // The analog triggers arrive as buttons with a position. The event API models
                // them as axes, matching how an application reads a trigger.
                EventType::ButtonChanged(GilrsButton::LeftTrigger2, value, _) => {
                    return Some(axis_event(id, Axis::TriggerLeft, value));
                }
                EventType::ButtonChanged(GilrsButton::RightTrigger2, value, _) => {
                    return Some(axis_event(id, Axis::TriggerRight, value));
                }
                EventType::AxisChanged(axis, value, _) => {
                    if let Some(mapped) = map_axis(axis) {
                        return Some(axis_event(id, mapped, normalize(axis, value)));
                    }
                }
                _ => continue,
            }
        }
    }
}

impl fmt::Debug for Controllers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Controllers")
            .field("available", &self.gilrs.is_some())
            .field("opened", &self.opened)
            .finish()
    }
}

/// Narrows a gilrs gamepad id into the width the event API reports.
fn gamepad_id(id: gilrs::GamepadId) -> u32 {
    u32::try_from(usize::from(id)).unwrap_or(u32::MAX)
}

/// Converts a gilrs axis position into the convention the event API reports.
///
/// Two corrections are needed. gilrs normalizes a stick pushed up to a positive value, negating
/// the raw reading wherever `gilrs_core::IS_Y_AXIS_REVERSED` is set, which covers every platform
/// except XInput. The event API reports a stick pushed up as negative, so vertical sticks are
/// flipped back.
///
/// A trigger rests at zero and only climbs. gilrs reports one as a button position in `0.0..=1.0`
/// for a pad it has a mapping for, and as a full `-1.0..=1.0` axis for one it does not. Left
/// alone, an unmapped pad reports a resting trigger as fully pressed, so the axis form is
/// rescaled to match.
fn normalize(axis: GilrsAxis, value: f32) -> f32 {
    match axis {
        GilrsAxis::LeftStickY | GilrsAxis::RightStickY => -value,
        GilrsAxis::LeftZ | GilrsAxis::RightZ => (value + 1.0) / 2.0,
        _ => value,
    }
}

/// Builds an axis event, scaling the normalized position gilrs reports.
fn axis_event(controller_id: ControllerId, axis: Axis, value: f32) -> Event {
    #[allow(clippy::cast_possible_truncation)]
    let value = (value.clamp(-1.0, 1.0) * AXIS_SCALE) as i16;
    Event::ControllerAxisMotion {
        controller_id: controller_id.0,
        axis,
        value,
    }
}

/// Maps a gilrs button, returning `None` for one the event API does not model.
///
/// gilrs names buttons by position, so `South` is the lower face button whatever a given pad
/// prints on it.
fn map_button(button: GilrsButton) -> Option<ControllerButton> {
    let button = match button {
        GilrsButton::South => ControllerButton::A,
        GilrsButton::East => ControllerButton::B,
        GilrsButton::West => ControllerButton::X,
        GilrsButton::North => ControllerButton::Y,
        GilrsButton::Select => ControllerButton::Back,
        GilrsButton::Mode => ControllerButton::Guide,
        GilrsButton::Start => ControllerButton::Start,
        GilrsButton::LeftThumb => ControllerButton::LeftStick,
        GilrsButton::RightThumb => ControllerButton::RightStick,
        GilrsButton::LeftTrigger => ControllerButton::LeftShoulder,
        GilrsButton::RightTrigger => ControllerButton::RightShoulder,
        GilrsButton::DPadUp => ControllerButton::DPadUp,
        GilrsButton::DPadDown => ControllerButton::DPadDown,
        GilrsButton::DPadLeft => ControllerButton::DPadLeft,
        GilrsButton::DPadRight => ControllerButton::DPadRight,
        // The analog triggers are reported as axes instead. gilrs raises a press and release
        // for them alongside every position change, so logging here would be one line per pull.
        GilrsButton::LeftTrigger2 | GilrsButton::RightTrigger2 => return None,
        GilrsButton::C | GilrsButton::Z | GilrsButton::Unknown => {
            debug!("unmapped controller button: {button:?}");
            return None;
        }
    };
    Some(button)
}

/// Maps a gilrs axis, returning `None` for one the event API does not model.
fn map_axis(axis: GilrsAxis) -> Option<Axis> {
    let axis = match axis {
        GilrsAxis::LeftStickX => Axis::LeftX,
        GilrsAxis::LeftStickY => Axis::LeftY,
        GilrsAxis::RightStickX => Axis::RightX,
        GilrsAxis::RightStickY => Axis::RightY,
        GilrsAxis::LeftZ => Axis::TriggerLeft,
        GilrsAxis::RightZ => Axis::TriggerRight,
        // The d-pad also arrives as buttons, and reporting both would fire each press twice.
        GilrsAxis::DPadX | GilrsAxis::DPadY | GilrsAxis::Unknown => {
            debug!("unmapped controller axis: {axis:?}");
            return None;
        }
    };
    Some(axis)
}
