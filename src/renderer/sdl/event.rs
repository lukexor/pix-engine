use crate::prelude::*;
use log::warn;
use sdl2::{
    controller::{Axis as SdlAxis, Button as SdlButton},
    event::{Event as SdlEvent, WindowEvent as SdlWindowEvent},
    joystick::HatState as SdlHatState,
    keyboard::{Keycode as SdlKeycode, Mod as SdlMod},
    mouse::MouseButton as SdlMouseButton,
};

#[doc(hidden)]
impl From<SdlEvent> for Event {
    fn from(event: SdlEvent) -> Self {
        match event {
            SdlEvent::Quit { .. } => Self::Quit,
            SdlEvent::AppTerminating { .. } => Self::AppTerminating,
            SdlEvent::Window {
                window_id,
                win_event,
                ..
            } => Self::Window {
                window_id,
                win_event: win_event.into(),
            },
            SdlEvent::KeyDown {
                keycode,
                keymod,
                repeat,
                ..
            } => Self::KeyDown {
                key: keycode.map(|k| k.into()),
                keymod: keymod.into(),
                repeat,
            },
            SdlEvent::KeyUp {
                keycode,
                keymod,
                repeat,
                ..
            } => Self::KeyUp {
                key: keycode.map(|k| k.into()),
                keymod: keymod.into(),
                repeat,
            },
            SdlEvent::TextInput { text, .. } => Self::TextInput { text },
            SdlEvent::MouseMotion {
                x, y, xrel, yrel, ..
            } => Self::MouseMotion { x, y, xrel, yrel },
            SdlEvent::MouseButtonDown {
                mouse_btn, x, y, ..
            } => Self::MouseDown {
                button: mouse_btn.into(),
                x,
                y,
            },
            SdlEvent::MouseButtonUp {
                mouse_btn, x, y, ..
            } => Self::MouseUp {
                button: mouse_btn.into(),
                x,
                y,
            },
            SdlEvent::MouseWheel { x, y, .. } => Self::MouseWheel { x, y },
            SdlEvent::JoyAxisMotion {
                which,
                axis_idx,
                value,
                ..
            } => Self::JoyAxisMotion {
                joy_id: which,
                axis_idx,
                value,
            },
            SdlEvent::JoyHatMotion {
                which,
                hat_idx,
                state,
                ..
            } => Self::JoyHatMotion {
                joy_id: which,
                hat_idx,
                state: state.into(),
            },
            SdlEvent::JoyBallMotion {
                which,
                ball_idx,
                xrel,
                yrel,
                ..
            } => Self::JoyBallMotion {
                joy_id: which,
                ball_idx,
                xrel,
                yrel,
            },
            SdlEvent::JoyButtonDown {
                which, button_idx, ..
            } => Self::JoyDown {
                joy_id: which,
                button_idx,
            },
            SdlEvent::JoyButtonUp {
                which, button_idx, ..
            } => Self::JoyUp {
                joy_id: which,
                button_idx,
            },
            SdlEvent::JoyDeviceAdded { which, .. } => Self::JoyDeviceAdded { joy_id: which },
            SdlEvent::JoyDeviceRemoved { which, .. } => Self::JoyDeviceRemoved { joy_id: which },
            SdlEvent::ControllerAxisMotion {
                which, axis, value, ..
            } => Self::ControllerAxisMotion {
                controller_id: which,
                axis: axis.into(),
                value,
            },
            SdlEvent::ControllerButtonDown { which, button, .. } => Self::ControllerDown {
                controller_id: which,
                button: button.into(),
            },
            SdlEvent::ControllerButtonUp { which, button, .. } => Self::ControllerUp {
                controller_id: which,
                button: button.into(),
            },
            SdlEvent::ControllerDeviceAdded { which, .. } => Self::ControllerAdded {
                controller_id: which,
            },
            SdlEvent::ControllerDeviceRemoved { which, .. } => Self::ControllerRemoved {
                controller_id: which,
            },
            SdlEvent::ControllerDeviceRemapped { which, .. } => Self::ControllerRemapped {
                controller_id: which,
            },
            SdlEvent::FingerDown {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
                ..
            } => Self::FingerDown {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            },
            SdlEvent::FingerUp {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
                ..
            } => Self::FingerUp {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            },
            SdlEvent::FingerMotion {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
                ..
            } => Self::FingerMotion {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            },
            SdlEvent::AudioDeviceAdded {
                which, iscapture, ..
            } => Self::AudioDeviceAdded {
                device_id: which,
                iscapture,
            },
            SdlEvent::AudioDeviceRemoved {
                which, iscapture, ..
            } => Self::AudioDeviceRemoved {
                device_id: which,
                iscapture,
            },
            evt => {
                warn!("Unsupported SDL `Event`: {:?}", evt);
                Self::Unsupported
            }
        }
    }
}

#[doc(hidden)]
impl From<SdlWindowEvent> for WindowEvent {
    fn from(win_event: SdlWindowEvent) -> Self {
        match win_event {
            SdlWindowEvent::Shown => Self::Shown,
            SdlWindowEvent::Exposed => Self::Exposed,
            SdlWindowEvent::Hidden => Self::Hidden,
            SdlWindowEvent::Moved(x, y) => Self::Moved(x, y),
            SdlWindowEvent::Resized(w, h) => Self::Resized(w, h),
            SdlWindowEvent::SizeChanged(w, h) => Self::SizeChanged(w, h),
            SdlWindowEvent::Minimized => Self::Minimized,
            SdlWindowEvent::Maximized => Self::Maximized,
            SdlWindowEvent::Restored => Self::Restored,
            SdlWindowEvent::Enter => Self::Enter,
            SdlWindowEvent::Leave => Self::Leave,
            SdlWindowEvent::FocusGained => Self::FocusGained,
            SdlWindowEvent::FocusLost => Self::FocusLost,
            SdlWindowEvent::Close => Self::Close,
            evt => {
                warn!("Unsupported SDL `WindowEvent`: {:?}", evt);
                Self::Unsupported
            }
        }
    }
}

#[doc(hidden)]
impl From<SdlKeycode> for Key {
    fn from(keycode: SdlKeycode) -> Self {
        match keycode {
            SdlKeycode::Backspace => Self::Backspace,
            SdlKeycode::Tab => Self::Tab,
            SdlKeycode::Return => Self::Return,
            SdlKeycode::Escape => Self::Escape,
            SdlKeycode::Space => Self::Space,
            SdlKeycode::Exclaim => Self::Exclaim,
            SdlKeycode::Quotedbl => Self::Quotedbl,
            SdlKeycode::Hash => Self::Hash,
            SdlKeycode::Dollar => Self::Dollar,
            SdlKeycode::Percent => Self::Percent,
            SdlKeycode::Ampersand => Self::Ampersand,
            SdlKeycode::Quote => Self::Quote,
            SdlKeycode::LeftParen => Self::LeftParen,
            SdlKeycode::RightParen => Self::RightParen,
            SdlKeycode::Asterisk => Self::Asterisk,
            SdlKeycode::Plus => Self::Plus,
            SdlKeycode::Comma => Self::Comma,
            SdlKeycode::Minus => Self::Minus,
            SdlKeycode::Period => Self::Period,
            SdlKeycode::Slash => Self::Slash,
            SdlKeycode::Num0 => Self::Num0,
            SdlKeycode::Num1 => Self::Num1,
            SdlKeycode::Num2 => Self::Num2,
            SdlKeycode::Num3 => Self::Num3,
            SdlKeycode::Num4 => Self::Num4,
            SdlKeycode::Num5 => Self::Num5,
            SdlKeycode::Num6 => Self::Num6,
            SdlKeycode::Num7 => Self::Num7,
            SdlKeycode::Num8 => Self::Num8,
            SdlKeycode::Num9 => Self::Num9,
            SdlKeycode::Colon => Self::Colon,
            SdlKeycode::Semicolon => Self::Semicolon,
            SdlKeycode::Less => Self::Less,
            SdlKeycode::Equals => Self::Equals,
            SdlKeycode::Greater => Self::Greater,
            SdlKeycode::Question => Self::Question,
            SdlKeycode::At => Self::At,
            SdlKeycode::LeftBracket => Self::LeftBracket,
            SdlKeycode::Backslash => Self::Backslash,
            SdlKeycode::RightBracket => Self::RightBracket,
            SdlKeycode::Caret => Self::Caret,
            SdlKeycode::Underscore => Self::Underscore,
            SdlKeycode::Backquote => Self::Backquote,
            SdlKeycode::A => Self::A,
            SdlKeycode::B => Self::B,
            SdlKeycode::C => Self::C,
            SdlKeycode::D => Self::D,
            SdlKeycode::E => Self::E,
            SdlKeycode::F => Self::F,
            SdlKeycode::G => Self::G,
            SdlKeycode::H => Self::H,
            SdlKeycode::I => Self::I,
            SdlKeycode::J => Self::J,
            SdlKeycode::K => Self::K,
            SdlKeycode::L => Self::L,
            SdlKeycode::M => Self::M,
            SdlKeycode::N => Self::N,
            SdlKeycode::O => Self::O,
            SdlKeycode::P => Self::P,
            SdlKeycode::Q => Self::Q,
            SdlKeycode::R => Self::R,
            SdlKeycode::S => Self::S,
            SdlKeycode::T => Self::T,
            SdlKeycode::U => Self::U,
            SdlKeycode::V => Self::V,
            SdlKeycode::W => Self::W,
            SdlKeycode::X => Self::X,
            SdlKeycode::Y => Self::Y,
            SdlKeycode::Z => Self::Z,
            SdlKeycode::Delete => Self::Delete,
            SdlKeycode::CapsLock => Self::CapsLock,
            SdlKeycode::F1 => Self::F1,
            SdlKeycode::F2 => Self::F2,
            SdlKeycode::F3 => Self::F3,
            SdlKeycode::F4 => Self::F4,
            SdlKeycode::F5 => Self::F5,
            SdlKeycode::F6 => Self::F6,
            SdlKeycode::F7 => Self::F7,
            SdlKeycode::F8 => Self::F8,
            SdlKeycode::F9 => Self::F9,
            SdlKeycode::F10 => Self::F10,
            SdlKeycode::F11 => Self::F11,
            SdlKeycode::F12 => Self::F12,
            SdlKeycode::PrintScreen => Self::PrintScreen,
            SdlKeycode::ScrollLock => Self::ScrollLock,
            SdlKeycode::Pause => Self::Pause,
            SdlKeycode::Insert => Self::Insert,
            SdlKeycode::Home => Self::Home,
            SdlKeycode::PageUp => Self::PageUp,
            SdlKeycode::End => Self::End,
            SdlKeycode::PageDown => Self::PageDown,
            SdlKeycode::Right => Self::Right,
            SdlKeycode::Left => Self::Left,
            SdlKeycode::Down => Self::Down,
            SdlKeycode::Up => Self::Up,
            SdlKeycode::NumLockClear => Self::NumLock,
            SdlKeycode::KpDivide => Self::KpDivide,
            SdlKeycode::KpMultiply => Self::KpMultiply,
            SdlKeycode::KpMinus => Self::KpMinus,
            SdlKeycode::KpPlus => Self::KpPlus,
            SdlKeycode::KpEnter => Self::KpEnter,
            SdlKeycode::Kp1 => Self::Kp1,
            SdlKeycode::Kp2 => Self::Kp2,
            SdlKeycode::Kp3 => Self::Kp3,
            SdlKeycode::Kp4 => Self::Kp4,
            SdlKeycode::Kp5 => Self::Kp5,
            SdlKeycode::Kp6 => Self::Kp6,
            SdlKeycode::Kp7 => Self::Kp7,
            SdlKeycode::Kp8 => Self::Kp8,
            SdlKeycode::Kp9 => Self::Kp9,
            SdlKeycode::Kp0 => Self::Kp0,
            SdlKeycode::KpPeriod => Self::KpPeriod,
            SdlKeycode::KpEquals => Self::KpEquals,
            SdlKeycode::KpComma => Self::KpComma,
            SdlKeycode::LCtrl => Self::LCtrl,
            SdlKeycode::LShift => Self::LShift,
            SdlKeycode::LAlt => Self::LAlt,
            SdlKeycode::LGui => Self::LGui,
            SdlKeycode::RCtrl => Self::RCtrl,
            SdlKeycode::RShift => Self::RShift,
            SdlKeycode::RAlt => Self::RAlt,
            SdlKeycode::RGui => Self::RGui,
            keycode => {
                warn!("Unsupported SDL `Keycode`: {:?}", keycode);
                Self::Unsupported
            }
        }
    }
}

#[doc(hidden)]
impl From<SdlMod> for KeyMod {
    fn from(keymod: SdlMod) -> Self {
        let mut result = Self::NONE;
        if keymod.contains(SdlMod::LSHIFTMOD) || keymod.contains(SdlMod::RSHIFTMOD) {
            result |= Self::SHIFT;
        }
        if keymod.contains(SdlMod::LCTRLMOD) || keymod.contains(SdlMod::RCTRLMOD) {
            result |= Self::CTRL;
        }
        if keymod.contains(SdlMod::LALTMOD) || keymod.contains(SdlMod::RALTMOD) {
            result |= Self::ALT;
        }
        if keymod.contains(SdlMod::LGUIMOD) || keymod.contains(SdlMod::RGUIMOD) {
            result |= Self::GUI;
        }
        result
    }
}

#[doc(hidden)]
impl From<SdlMouseButton> for Mouse {
    fn from(button: SdlMouseButton) -> Self {
        match button {
            SdlMouseButton::Left => Self::Left,
            SdlMouseButton::Middle => Self::Middle,
            SdlMouseButton::Right => Self::Right,
            btn => {
                warn!("Unsupported SDL `MouseButton`: {:?}", btn);
                Self::Unsupported
            }
        }
    }
}

#[doc(hidden)]
impl From<SdlButton> for ControllerButton {
    fn from(button: SdlButton) -> Self {
        match button {
            SdlButton::A => Self::A,
            SdlButton::B => Self::B,
            SdlButton::X => Self::X,
            SdlButton::Y => Self::Y,
            SdlButton::Back => Self::Back,
            SdlButton::Guide => Self::Guide,
            SdlButton::Start => Self::Start,
            SdlButton::LeftStick => Self::LeftStick,
            SdlButton::RightStick => Self::RightStick,
            SdlButton::LeftShoulder => Self::LeftShoulder,
            SdlButton::RightShoulder => Self::RightShoulder,
            SdlButton::DPadUp => Self::DPadUp,
            SdlButton::DPadDown => Self::DPadDown,
            SdlButton::DPadLeft => Self::DPadLeft,
            SdlButton::DPadRight => Self::DPadRight,
            SdlButton::Misc1 => Self::Misc1,
            SdlButton::Paddle1 => Self::Paddle1,
            SdlButton::Paddle2 => Self::Paddle2,
            SdlButton::Paddle3 => Self::Paddle3,
            SdlButton::Paddle4 => Self::Paddle4,
            SdlButton::Touchpad => Self::Touchpad,
        }
    }
}

#[doc(hidden)]
impl From<SdlAxis> for Axis {
    fn from(axis: SdlAxis) -> Self {
        match axis {
            SdlAxis::LeftX => Self::LeftX,
            SdlAxis::LeftY => Self::LeftY,
            SdlAxis::RightX => Self::RightX,
            SdlAxis::RightY => Self::RightY,
            SdlAxis::TriggerLeft => Self::TriggerLeft,
            SdlAxis::TriggerRight => Self::TriggerRight,
        }
    }
}

#[doc(hidden)]
impl From<SdlHatState> for HatState {
    fn from(hat: SdlHatState) -> Self {
        match hat {
            SdlHatState::LeftUp => Self::LeftUp,
            SdlHatState::Left => Self::Left,
            SdlHatState::LeftDown => Self::LeftDown,
            SdlHatState::Up => Self::Up,
            SdlHatState::Centered => Self::Centered,
            SdlHatState::Down => Self::Down,
            SdlHatState::RightUp => Self::RightUp,
            SdlHatState::Right => Self::Right,
            SdlHatState::RightDown => Self::RightDown,
        }
    }
}
