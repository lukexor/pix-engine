use crate::prelude::{Axis, ControllerButton, Event, Key, KeyMod, Mouse, WindowEvent};
use sdl2::{
    controller::{Axis as SdlAxis, Button as SdlButton},
    event::{Event as SdlEvent, WindowEvent as SdlWindowEvent},
    keyboard::{Keycode as SdlKeycode, Mod as SdlMod},
    mouse::MouseButton as SdlMouseButton,
};

impl From<SdlEvent> for Event {
    fn from(event: SdlEvent) -> Self {
        use Event::*;
        match event {
            SdlEvent::Quit { .. } => Quit,
            SdlEvent::AppTerminating { .. } => AppTerminating,
            SdlEvent::Window {
                window_id,
                win_event,
                ..
            } => Window {
                window_id,
                win_event: win_event.into(),
            },
            SdlEvent::KeyDown {
                keycode,
                keymod,
                repeat,
                ..
            } => KeyDown {
                key: keycode.map(|k| k.into()),
                keymod: keymod.into(),
                repeat,
            },
            SdlEvent::KeyUp {
                keycode,
                keymod,
                repeat,
                ..
            } => KeyUp {
                key: keycode.map(|k| k.into()),
                keymod: keymod.into(),
                repeat,
            },
            SdlEvent::TextInput { text, .. } => TextInput { text },
            SdlEvent::MouseMotion {
                x, y, xrel, yrel, ..
            } => MouseMotion { x, y, xrel, yrel },
            SdlEvent::MouseButtonDown {
                mouse_btn, x, y, ..
            } => MouseDown {
                button: mouse_btn.into(),
                x,
                y,
            },
            SdlEvent::MouseButtonUp {
                mouse_btn, x, y, ..
            } => MouseUp {
                button: mouse_btn.into(),
                x,
                y,
            },
            SdlEvent::MouseWheel { x, y, .. } => MouseWheel { x, y },
            SdlEvent::JoyAxisMotion {
                which,
                axis_idx,
                value,
                ..
            } => JoyAxisMotion {
                joy_id: which,
                axis_idx,
                value,
            },
            SdlEvent::JoyBallMotion {
                which,
                ball_idx,
                xrel,
                yrel,
                ..
            } => JoyBallMotion {
                joy_id: which,
                ball_idx,
                xrel,
                yrel,
            },
            SdlEvent::JoyButtonDown {
                which, button_idx, ..
            } => JoyDown {
                joy_id: which,
                button_idx,
            },
            SdlEvent::JoyButtonUp {
                which, button_idx, ..
            } => JoyUp {
                joy_id: which,
                button_idx,
            },
            SdlEvent::JoyDeviceAdded { which, .. } => JoyDeviceAdded { joy_id: which },
            SdlEvent::JoyDeviceRemoved { which, .. } => JoyDeviceRemoved { joy_id: which },
            SdlEvent::ControllerAxisMotion {
                which, axis, value, ..
            } => ControllerAxisMotion {
                controller_id: which,
                axis: axis.into(),
                value,
            },
            SdlEvent::ControllerButtonDown { which, button, .. } => ControllerDown {
                controller_id: which,
                button: button.into(),
            },
            SdlEvent::ControllerButtonUp { which, button, .. } => ControllerUp {
                controller_id: which,
                button: button.into(),
            },
            SdlEvent::ControllerDeviceAdded { which, .. } => ControllerAdded {
                controller_id: which,
            },
            SdlEvent::ControllerDeviceRemoved { which, .. } => ControllerRemoved {
                controller_id: which,
            },
            SdlEvent::ControllerDeviceRemapped { which, .. } => ControllerRemapped {
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
            } => FingerDown {
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
            } => FingerUp {
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
            } => FingerMotion {
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure,
            },
            _ => Self::Unknown,
        }
    }
}

impl From<SdlWindowEvent> for WindowEvent {
    fn from(win_event: SdlWindowEvent) -> Self {
        use WindowEvent::*;
        match win_event {
            SdlWindowEvent::Shown => Shown,
            SdlWindowEvent::Hidden => Hidden,
            SdlWindowEvent::Moved(x, y) => Moved(x, y),
            SdlWindowEvent::Resized(w, h) | SdlWindowEvent::SizeChanged(w, h) => Resized(w, h),
            SdlWindowEvent::Minimized => Minimized,
            SdlWindowEvent::Maximized => Maximized,
            SdlWindowEvent::Restored => Restored,
            SdlWindowEvent::Enter => Enter,
            SdlWindowEvent::Leave => Leave,
            SdlWindowEvent::FocusGained => FocusGained,
            SdlWindowEvent::FocusLost => FocusLost,
            SdlWindowEvent::Close => Close,
            _ => Unknown,
        }
    }
}

impl From<SdlKeycode> for Key {
    fn from(keycode: SdlKeycode) -> Self {
        use Key::*;
        match keycode {
            SdlKeycode::Backspace => Backspace,
            SdlKeycode::Tab => Tab,
            SdlKeycode::Return => Return,
            SdlKeycode::Escape => Escape,
            SdlKeycode::Space => Space,
            SdlKeycode::Exclaim => Exclaim,
            SdlKeycode::Quotedbl => Quotedbl,
            SdlKeycode::Hash => Hash,
            SdlKeycode::Dollar => Dollar,
            SdlKeycode::Percent => Percent,
            SdlKeycode::Ampersand => Ampersand,
            SdlKeycode::Quote => Quote,
            SdlKeycode::LeftParen => LeftParen,
            SdlKeycode::RightParen => RightParen,
            SdlKeycode::Asterisk => Asterisk,
            SdlKeycode::Plus => Plus,
            SdlKeycode::Comma => Comma,
            SdlKeycode::Minus => Minus,
            SdlKeycode::Period => Period,
            SdlKeycode::Slash => Slash,
            SdlKeycode::Num0 => Num0,
            SdlKeycode::Num1 => Num1,
            SdlKeycode::Num2 => Num2,
            SdlKeycode::Num3 => Num3,
            SdlKeycode::Num4 => Num4,
            SdlKeycode::Num5 => Num5,
            SdlKeycode::Num6 => Num6,
            SdlKeycode::Num7 => Num7,
            SdlKeycode::Num8 => Num8,
            SdlKeycode::Num9 => Num9,
            SdlKeycode::Colon => Colon,
            SdlKeycode::Semicolon => Semicolon,
            SdlKeycode::Less => Less,
            SdlKeycode::Equals => Equals,
            SdlKeycode::Greater => Greater,
            SdlKeycode::Question => Question,
            SdlKeycode::At => At,
            SdlKeycode::LeftBracket => LeftBracket,
            SdlKeycode::Backslash => Backslash,
            SdlKeycode::RightBracket => RightBracket,
            SdlKeycode::Caret => Caret,
            SdlKeycode::Underscore => Underscore,
            SdlKeycode::Backquote => Backquote,
            SdlKeycode::A => A,
            SdlKeycode::B => B,
            SdlKeycode::C => C,
            SdlKeycode::D => D,
            SdlKeycode::E => E,
            SdlKeycode::F => F,
            SdlKeycode::G => G,
            SdlKeycode::H => H,
            SdlKeycode::I => I,
            SdlKeycode::J => J,
            SdlKeycode::K => K,
            SdlKeycode::L => L,
            SdlKeycode::M => M,
            SdlKeycode::N => N,
            SdlKeycode::O => O,
            SdlKeycode::P => P,
            SdlKeycode::Q => Q,
            SdlKeycode::R => R,
            SdlKeycode::S => S,
            SdlKeycode::T => T,
            SdlKeycode::U => U,
            SdlKeycode::V => V,
            SdlKeycode::W => W,
            SdlKeycode::X => X,
            SdlKeycode::Y => Y,
            SdlKeycode::Z => Z,
            SdlKeycode::Delete => Delete,
            SdlKeycode::CapsLock => CapsLock,
            SdlKeycode::F1 => F1,
            SdlKeycode::F2 => F2,
            SdlKeycode::F3 => F3,
            SdlKeycode::F4 => F4,
            SdlKeycode::F5 => F5,
            SdlKeycode::F6 => F6,
            SdlKeycode::F7 => F7,
            SdlKeycode::F8 => F8,
            SdlKeycode::F9 => F9,
            SdlKeycode::F10 => F10,
            SdlKeycode::F11 => F11,
            SdlKeycode::F12 => F12,
            SdlKeycode::PrintScreen => PrintScreen,
            SdlKeycode::ScrollLock => ScrollLock,
            SdlKeycode::Pause => Pause,
            SdlKeycode::Insert => Insert,
            SdlKeycode::Home => Home,
            SdlKeycode::PageUp => PageUp,
            SdlKeycode::End => End,
            SdlKeycode::PageDown => PageDown,
            SdlKeycode::Right => Right,
            SdlKeycode::Left => Left,
            SdlKeycode::Down => Down,
            SdlKeycode::Up => Up,
            SdlKeycode::NumLockClear => NumLock,
            SdlKeycode::LCtrl => LCtrl,
            SdlKeycode::LShift => LShift,
            SdlKeycode::LAlt => LAlt,
            SdlKeycode::LGui => LGui,
            SdlKeycode::RCtrl => RCtrl,
            SdlKeycode::RShift => RShift,
            SdlKeycode::RAlt => RAlt,
            SdlKeycode::RGui => RGui,
            _ => Unknown,
        }
    }
}

impl From<SdlMod> for KeyMod {
    fn from(keymod: SdlMod) -> Self {
        let mut result = KeyMod::NONE;
        if keymod.contains(SdlMod::LSHIFTMOD) || keymod.contains(SdlMod::RSHIFTMOD) {
            result |= KeyMod::SHIFT;
        }
        if keymod.contains(SdlMod::LCTRLMOD) || keymod.contains(SdlMod::RCTRLMOD) {
            result |= KeyMod::CTRL;
        }
        if keymod.contains(SdlMod::LALTMOD) || keymod.contains(SdlMod::RALTMOD) {
            result |= KeyMod::ALT;
        }
        if keymod.contains(SdlMod::LGUIMOD) || keymod.contains(SdlMod::RGUIMOD) {
            result |= KeyMod::GUI;
        }
        result
    }
}

impl From<SdlMouseButton> for Mouse {
    fn from(button: SdlMouseButton) -> Self {
        use Mouse::*;
        match button {
            SdlMouseButton::Left => Left,
            SdlMouseButton::Middle => Middle,
            SdlMouseButton::Right => Right,
            _ => Unknown,
        }
    }
}

impl From<SdlButton> for ControllerButton {
    fn from(button: SdlButton) -> Self {
        use ControllerButton::*;
        match button {
            SdlButton::A => A,
            SdlButton::B => B,
            SdlButton::X => X,
            SdlButton::Y => Y,
            SdlButton::Back => Back,
            SdlButton::Guide => Guide,
            SdlButton::Start => Start,
            SdlButton::LeftStick => LeftStick,
            SdlButton::RightStick => RightStick,
            SdlButton::LeftShoulder => LeftShoulder,
            SdlButton::RightShoulder => RightShoulder,
            SdlButton::DPadUp => DPadUp,
            SdlButton::DPadDown => DPadDown,
            SdlButton::DPadLeft => DPadLeft,
            SdlButton::DPadRight => DPadRight,
            SdlButton::Misc1 => Misc1,
            SdlButton::Paddle1 => Paddle1,
            SdlButton::Paddle2 => Paddle2,
            SdlButton::Paddle3 => Paddle3,
            SdlButton::Paddle4 => Paddle4,
            SdlButton::Touchpad => Touchpad,
        }
    }
}

impl From<SdlAxis> for Axis {
    fn from(axis: SdlAxis) -> Self {
        use Axis::*;
        match axis {
            SdlAxis::LeftX => LeftX,
            SdlAxis::LeftY => LeftY,
            SdlAxis::RightX => RightX,
            SdlAxis::RightY => RightY,
            SdlAxis::TriggerLeft => TriggerLeft,
            SdlAxis::TriggerRight => TriggerRight,
        }
    }
}
