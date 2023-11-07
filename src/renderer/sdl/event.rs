use crate::prelude::*;
use log::warn;
use sdl2::{
    controller::{Axis as SdlAxis, Button as SdlButton},
    event::{Event as SdlEvent, WindowEvent as SdlWindowEvent},
    joystick::HatState as SdlHatState,
    keyboard::{Keycode as SdlKeycode, Mod as SdlMod, Scancode as SdlScancode},
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
                scancode,
                ..
            } => Self::KeyDown {
                key: keycode.map(Into::into),
                keymod: keymod.into(),
                repeat,
                scan: scancode.map(Into::into),
            },
            SdlEvent::KeyUp {
                keycode,
                keymod,
                repeat,
                scancode,
                ..
            } => Self::KeyUp {
                key: keycode.map(Into::into),
                keymod: keymod.into(),
                repeat,
                scan: scancode.map(Into::into),
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
                warn!("Unhandled SDL `Event`: {:?}", evt);
                Self::Unhandled
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
                warn!("Unhandled SDL `WindowEvent`: {:?}", evt);
                Self::Unhandled
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
                warn!("Unhandled SDL `Keycode`: {:?}", keycode);
                Self::Unhandled
            }
        }
    }
}

#[doc(hidden)]
impl From<SdlScancode> for Scan {
    fn from(scancode: SdlScancode) -> Self {
        match scancode {
            SdlScancode::A => Self::A,
            SdlScancode::B => Self::B,
            SdlScancode::C => Self::C,
            SdlScancode::D => Self::D,
            SdlScancode::E => Self::E,
            SdlScancode::F => Self::F,
            SdlScancode::G => Self::G,
            SdlScancode::H => Self::H,
            SdlScancode::I => Self::I,
            SdlScancode::J => Self::J,
            SdlScancode::K => Self::K,
            SdlScancode::L => Self::L,
            SdlScancode::M => Self::M,
            SdlScancode::N => Self::N,
            SdlScancode::O => Self::O,
            SdlScancode::P => Self::P,
            SdlScancode::Q => Self::Q,
            SdlScancode::R => Self::R,
            SdlScancode::S => Self::S,
            SdlScancode::T => Self::T,
            SdlScancode::U => Self::U,
            SdlScancode::V => Self::V,
            SdlScancode::W => Self::W,
            SdlScancode::X => Self::X,
            SdlScancode::Y => Self::Y,
            SdlScancode::Z => Self::Z,
            SdlScancode::Num1 => Self::Num1,
            SdlScancode::Num2 => Self::Num2,
            SdlScancode::Num3 => Self::Num3,
            SdlScancode::Num4 => Self::Num4,
            SdlScancode::Num5 => Self::Num5,
            SdlScancode::Num6 => Self::Num6,
            SdlScancode::Num7 => Self::Num7,
            SdlScancode::Num8 => Self::Num8,
            SdlScancode::Num9 => Self::Num9,
            SdlScancode::Num0 => Self::Num0,
            SdlScancode::Return => Self::Return,
            SdlScancode::Escape => Self::Escape,
            SdlScancode::Backspace => Self::Backspace,
            SdlScancode::Tab => Self::Tab,
            SdlScancode::Space => Self::Space,
            SdlScancode::Minus => Self::Minus,
            SdlScancode::Equals => Self::Equals,
            SdlScancode::LeftBracket => Self::LeftBracket,
            SdlScancode::RightBracket => Self::RightBracket,
            SdlScancode::Backslash => Self::Backslash,
            SdlScancode::NonUsHash => Self::NonUsHash,
            SdlScancode::Semicolon => Self::Semicolon,
            SdlScancode::Apostrophe => Self::Apostrophe,
            SdlScancode::Grave => Self::Grave,
            SdlScancode::Comma => Self::Comma,
            SdlScancode::Period => Self::Period,
            SdlScancode::Slash => Self::Slash,
            SdlScancode::CapsLock => Self::CapsLock,
            SdlScancode::F1 => Self::F1,
            SdlScancode::F2 => Self::F2,
            SdlScancode::F3 => Self::F3,
            SdlScancode::F4 => Self::F4,
            SdlScancode::F5 => Self::F5,
            SdlScancode::F6 => Self::F6,
            SdlScancode::F7 => Self::F7,
            SdlScancode::F8 => Self::F8,
            SdlScancode::F9 => Self::F9,
            SdlScancode::F10 => Self::F10,
            SdlScancode::F11 => Self::F11,
            SdlScancode::F12 => Self::F12,
            SdlScancode::PrintScreen => Self::PrintScreen,
            SdlScancode::ScrollLock => Self::ScrollLock,
            SdlScancode::Pause => Self::Pause,
            SdlScancode::Insert => Self::Insert,
            SdlScancode::Home => Self::Home,
            SdlScancode::PageUp => Self::PageUp,
            SdlScancode::Delete => Self::Delete,
            SdlScancode::End => Self::End,
            SdlScancode::PageDown => Self::PageDown,
            SdlScancode::Right => Self::Right,
            SdlScancode::Left => Self::Left,
            SdlScancode::Down => Self::Down,
            SdlScancode::Up => Self::Up,
            SdlScancode::NumLockClear => Self::NumLockClear,
            SdlScancode::KpDivide => Self::KpDivide,
            SdlScancode::KpMultiply => Self::KpMultiply,
            SdlScancode::KpMinus => Self::KpMinus,
            SdlScancode::KpPlus => Self::KpPlus,
            SdlScancode::KpEnter => Self::KpEnter,
            SdlScancode::Kp1 => Self::Kp1,
            SdlScancode::Kp2 => Self::Kp2,
            SdlScancode::Kp3 => Self::Kp3,
            SdlScancode::Kp4 => Self::Kp4,
            SdlScancode::Kp5 => Self::Kp5,
            SdlScancode::Kp6 => Self::Kp6,
            SdlScancode::Kp7 => Self::Kp7,
            SdlScancode::Kp8 => Self::Kp8,
            SdlScancode::Kp9 => Self::Kp9,
            SdlScancode::Kp0 => Self::Kp0,
            SdlScancode::KpPeriod => Self::KpPeriod,
            SdlScancode::NonUsBackslash => Self::NonUsBackslash,
            SdlScancode::Application => Self::Application,
            SdlScancode::Power => Self::Power,
            SdlScancode::KpEquals => Self::KpEquals,
            SdlScancode::F13 => Self::F13,
            SdlScancode::F14 => Self::F14,
            SdlScancode::F15 => Self::F15,
            SdlScancode::F16 => Self::F16,
            SdlScancode::F17 => Self::F17,
            SdlScancode::F18 => Self::F18,
            SdlScancode::F19 => Self::F19,
            SdlScancode::F20 => Self::F20,
            SdlScancode::F21 => Self::F21,
            SdlScancode::F22 => Self::F22,
            SdlScancode::F23 => Self::F23,
            SdlScancode::F24 => Self::F24,
            SdlScancode::Execute => Self::Execute,
            SdlScancode::Help => Self::Help,
            SdlScancode::Menu => Self::Menu,
            SdlScancode::Select => Self::Select,
            SdlScancode::Stop => Self::Stop,
            SdlScancode::Again => Self::Again,
            SdlScancode::Undo => Self::Undo,
            SdlScancode::Cut => Self::Cut,
            SdlScancode::Copy => Self::Copy,
            SdlScancode::Paste => Self::Paste,
            SdlScancode::Find => Self::Find,
            SdlScancode::Mute => Self::Mute,
            SdlScancode::VolumeUp => Self::VolumeUp,
            SdlScancode::VolumeDown => Self::VolumeDown,
            SdlScancode::KpComma => Self::KpComma,
            SdlScancode::KpEqualsAS400 => Self::KpEqualsAS400,
            SdlScancode::International1 => Self::International1,
            SdlScancode::International2 => Self::International2,
            SdlScancode::International3 => Self::International3,
            SdlScancode::International4 => Self::International4,
            SdlScancode::International5 => Self::International5,
            SdlScancode::International6 => Self::International6,
            SdlScancode::International7 => Self::International7,
            SdlScancode::International8 => Self::International8,
            SdlScancode::International9 => Self::International9,
            SdlScancode::Lang1 => Self::Lang1,
            SdlScancode::Lang2 => Self::Lang2,
            SdlScancode::Lang3 => Self::Lang3,
            SdlScancode::Lang4 => Self::Lang4,
            SdlScancode::Lang5 => Self::Lang5,
            SdlScancode::Lang6 => Self::Lang6,
            SdlScancode::Lang7 => Self::Lang7,
            SdlScancode::Lang8 => Self::Lang8,
            SdlScancode::Lang9 => Self::Lang9,
            SdlScancode::AltErase => Self::AltErase,
            SdlScancode::SysReq => Self::SysReq,
            SdlScancode::Cancel => Self::Cancel,
            SdlScancode::Clear => Self::Clear,
            SdlScancode::Prior => Self::Prior,
            SdlScancode::Return2 => Self::Return2,
            SdlScancode::Separator => Self::Separator,
            SdlScancode::Out => Self::Out,
            SdlScancode::Oper => Self::Oper,
            SdlScancode::ClearAgain => Self::ClearAgain,
            SdlScancode::CrSel => Self::CrSel,
            SdlScancode::ExSel => Self::ExSel,
            SdlScancode::Kp00 => Self::Kp00,
            SdlScancode::Kp000 => Self::Kp000,
            SdlScancode::ThousandsSeparator => Self::ThousandsSeparator,
            SdlScancode::DecimalSeparator => Self::DecimalSeparator,
            SdlScancode::CurrencyUnit => Self::CurrencyUnit,
            SdlScancode::CurrencySubUnit => Self::CurrencySubUnit,
            SdlScancode::KpLeftParen => Self::KpLeftParen,
            SdlScancode::KpRightParen => Self::KpRightParen,
            SdlScancode::KpLeftBrace => Self::KpLeftBrace,
            SdlScancode::KpRightBrace => Self::KpRightBrace,
            SdlScancode::KpTab => Self::KpTab,
            SdlScancode::KpBackspace => Self::KpBackspace,
            SdlScancode::KpA => Self::KpA,
            SdlScancode::KpB => Self::KpB,
            SdlScancode::KpC => Self::KpC,
            SdlScancode::KpD => Self::KpD,
            SdlScancode::KpE => Self::KpE,
            SdlScancode::KpF => Self::KpF,
            SdlScancode::KpXor => Self::KpXor,
            SdlScancode::KpPower => Self::KpPower,
            SdlScancode::KpPercent => Self::KpPercent,
            SdlScancode::KpLess => Self::KpLess,
            SdlScancode::KpGreater => Self::KpGreater,
            SdlScancode::KpAmpersand => Self::KpAmpersand,
            SdlScancode::KpDblAmpersand => Self::KpDblAmpersand,
            SdlScancode::KpVerticalBar => Self::KpVerticalBar,
            SdlScancode::KpDblVerticalBar => Self::KpDblVerticalBar,
            SdlScancode::KpColon => Self::KpColon,
            SdlScancode::KpHash => Self::KpHash,
            SdlScancode::KpSpace => Self::KpSpace,
            SdlScancode::KpAt => Self::KpAt,
            SdlScancode::KpExclam => Self::KpExclam,
            SdlScancode::KpMemStore => Self::KpMemStore,
            SdlScancode::KpMemRecall => Self::KpMemRecall,
            SdlScancode::KpMemClear => Self::KpMemClear,
            SdlScancode::KpMemAdd => Self::KpMemAdd,
            SdlScancode::KpMemSubtract => Self::KpMemSubtract,
            SdlScancode::KpMemMultiply => Self::KpMemMultiply,
            SdlScancode::KpMemDivide => Self::KpMemDivide,
            SdlScancode::KpPlusMinus => Self::KpPlusMinus,
            SdlScancode::KpClear => Self::KpClear,
            SdlScancode::KpClearEntry => Self::KpClearEntry,
            SdlScancode::KpBinary => Self::KpBinary,
            SdlScancode::KpOctal => Self::KpOctal,
            SdlScancode::KpDecimal => Self::KpDecimal,
            SdlScancode::KpHexadecimal => Self::KpHexadecimal,
            SdlScancode::LCtrl => Self::LCtrl,
            SdlScancode::LShift => Self::LShift,
            SdlScancode::LAlt => Self::LAlt,
            SdlScancode::LGui => Self::LGui,
            SdlScancode::RCtrl => Self::RCtrl,
            SdlScancode::RShift => Self::RShift,
            SdlScancode::RAlt => Self::RAlt,
            SdlScancode::RGui => Self::RGui,
            SdlScancode::Mode => Self::Mode,
            SdlScancode::AudioNext => Self::AudioNext,
            SdlScancode::AudioPrev => Self::AudioPrev,
            SdlScancode::AudioStop => Self::AudioStop,
            SdlScancode::AudioPlay => Self::AudioPlay,
            SdlScancode::AudioMute => Self::AudioMute,
            SdlScancode::MediaSelect => Self::MediaSelect,
            SdlScancode::Www => Self::Www,
            SdlScancode::Mail => Self::Mail,
            SdlScancode::Calculator => Self::Calculator,
            SdlScancode::Computer => Self::Computer,
            SdlScancode::AcSearch => Self::AcSearch,
            SdlScancode::AcHome => Self::AcHome,
            SdlScancode::AcBack => Self::AcBack,
            SdlScancode::AcForward => Self::AcForward,
            SdlScancode::AcStop => Self::AcStop,
            SdlScancode::AcRefresh => Self::AcRefresh,
            SdlScancode::AcBookmarks => Self::AcBookmarks,
            SdlScancode::BrightnessDown => Self::BrightnessDown,
            SdlScancode::BrightnessUp => Self::BrightnessUp,
            SdlScancode::DisplaySwitch => Self::DisplaySwitch,
            SdlScancode::KbdIllumToggle => Self::KbdIllumToggle,
            SdlScancode::KbdIllumDown => Self::KbdIllumDown,
            SdlScancode::KbdIllumUp => Self::KbdIllumUp,
            SdlScancode::Eject => Self::Eject,
            SdlScancode::Sleep => Self::Sleep,
            SdlScancode::App1 => Self::App1,
            SdlScancode::App2 => Self::App2,
            scancode @ SdlScancode::Num => {
                warn!("Unhandled SDL `Scancode`: {:?}", scancode);
                Self::Unhandled
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
                warn!("Unhandled SDL `MouseButton`: {:?}", btn);
                Self::Unhandled
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
