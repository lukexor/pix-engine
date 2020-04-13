use super::Sdl2Renderer;
use crate::event::{
    Axis, Button, Key, KeyMod, MouseButton, MouseWheelDirection, PixEvent, WindowEvent,
};
use sdl2::{
    controller,
    event::{self, Event},
    keyboard, mouse,
};

impl Sdl2Renderer {
    /// Poll all events from event pump
    pub(super) fn sdl_poll_events(&mut self) -> Vec<PixEvent> {
        self.event_pump.poll_iter().map(|e| e.into()).collect()
    }
}

impl From<Event> for PixEvent {
    fn from(item: Event) -> Self {
        use Event::*;
        match item {
            Quit { timestamp } => Self::Quit { timestamp },
            AppTerminating { timestamp } => Self::AppTerminating { timestamp },
            AppWillEnterBackground { timestamp } => Self::AppWillEnterBackground { timestamp },
            AppDidEnterBackground { timestamp } => Self::AppDidEnterBackground { timestamp },
            AppWillEnterForeground { timestamp } => Self::AppWillEnterForeground { timestamp },
            AppDidEnterForeground { timestamp } => Self::AppDidEnterForeground { timestamp },
            Window {
                timestamp,
                window_id,
                win_event,
            } => Self::Window {
                timestamp,
                window_id,
                win_event: win_event.into(),
            },
            KeyDown {
                timestamp,
                window_id,
                keycode: Some(key),
                keymod,
                repeat,
                ..
            } => Self::KeyDown {
                timestamp,
                window_id,
                key: key.into(),
                keymod: keymod.into(),
                repeat,
            },
            KeyUp {
                timestamp,
                window_id,
                keycode: Some(key),
                keymod,
                repeat,
                ..
            } => Self::KeyUp {
                timestamp,
                window_id,
                key: key.into(),
                keymod: keymod.into(),
                repeat,
            },
            TextEditing {
                timestamp,
                window_id,
                text,
                start,
                length,
            } => Self::TextEditing {
                timestamp,
                window_id,
                text,
                start,
                length,
            },
            TextInput {
                timestamp,
                window_id,
                text,
            } => Self::TextInput {
                timestamp,
                window_id,
                text,
            },
            MouseMotion {
                timestamp,
                window_id,
                which,
                x,
                y,
                xrel,
                yrel,
                ..
            } => Self::MouseMotion {
                timestamp,
                window_id,
                which,
                x,
                y,
                xrel,
                yrel,
            },
            MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => Self::MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn: mouse_btn.into(),
                clicks,
                x,
                y,
            },
            MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => Self::MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn: mouse_btn.into(),
                clicks,
                x,
                y,
            },
            MouseWheel {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
            } => Self::MouseWheel {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction: direction.into(),
            },
            JoyAxisMotion {
                timestamp,
                which,
                axis_idx,
                value,
            } => Self::JoyAxisMotion {
                timestamp,
                which,
                axis_idx,
                value,
            },
            JoyButtonDown {
                timestamp,
                which,
                button_idx,
            } => Self::JoyButtonDown {
                timestamp,
                which,
                button_idx,
            },
            JoyButtonUp {
                timestamp,
                which,
                button_idx,
            } => Self::JoyButtonUp {
                timestamp,
                which,
                button_idx,
            },
            JoyDeviceAdded { timestamp, which } => Self::JoyDeviceAdded { timestamp, which },
            JoyDeviceRemoved { timestamp, which } => Self::JoyDeviceRemoved { timestamp, which },
            ControllerAxisMotion {
                timestamp,
                which,
                axis,
                value,
            } => Self::ControllerAxisMotion {
                timestamp,
                which,
                axis: axis.into(),
                value,
            },
            ControllerButtonDown {
                timestamp,
                which,
                button,
            } => Self::ControllerButtonDown {
                timestamp,
                which,
                button: button.into(),
            },
            ControllerButtonUp {
                timestamp,
                which,
                button,
            } => Self::ControllerButtonUp {
                timestamp,
                which,
                button: button.into(),
            },
            ControllerDeviceAdded { timestamp, which } => {
                Self::ControllerDeviceAdded { timestamp, which }
            }
            ControllerDeviceRemoved { timestamp, which } => {
                Self::ControllerDeviceRemoved { timestamp, which }
            }
            ControllerDeviceRemapped { timestamp, which } => {
                Self::ControllerDeviceRemapped { timestamp, which }
            }
            Unknown { timestamp, type_ } => Self::Unknown { timestamp, type_ },
            _ => Self::Unknown {
                timestamp: 0,
                type_: 0,
            },
        }
    }
}

impl From<event::WindowEvent> for WindowEvent {
    fn from(item: event::WindowEvent) -> Self {
        use event::WindowEvent::*;
        match item {
            None => Self::None,
            Shown => Self::Shown,
            Hidden => Self::Hidden,
            Exposed => Self::Exposed,
            Moved(x, y) => Self::Moved(x, y),
            Resized(w, h) => Self::Resized(w, h),
            SizeChanged(w, h) => Self::SizeChanged(w, h),
            Minimized => Self::Minimized,
            Maximized => Self::Maximized,
            Restored => Self::Restored,
            Enter => Self::Enter,
            Leave => Self::Leave,
            FocusGained => Self::FocusGained,
            FocusLost => Self::FocusLost,
            Close => Self::Close,
            TakeFocus => Self::TakeFocus,
            HitTest => Self::HitTest,
        }
    }
}

macro_rules! map_keycode {
    ($item:ident, $($variant:ident),+ ) => {
        match $item {
            $(
                keyboard::Keycode::$variant => Self::$variant,
            )+
            _ => Self::Unknown,
        }
    };
}
macro_rules! map_button {
    ($item:ident, $($variant:ident),+ ) => {
        match $item {
            $(
                controller::Button::$variant => Self::$variant,
            )+
        }
    };
}

impl From<keyboard::Keycode> for Key {
    #[rustfmt::skip]
    fn from(item: keyboard::Keycode) -> Self {
        map_keycode!(
            item,
            A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
            Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
            Kp0, Kp1, Kp2, Kp3, Kp4, Kp5, Kp6, Kp7, Kp8, Kp9,
            F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
            Left, Up, Down, Right,
            Tab, Insert, Delete, Home, End, PageUp, PageDown,
            Escape, Backspace, Return, KpEnter, Pause, ScrollLock,
            Plus, Minus, Period, Underscore, Equals,
            KpMultiply, KpDivide, KpPlus, KpMinus, KpPeriod,
            Backquote, Exclaim, At, Hash, Dollar, Percent,
            Caret, Ampersand, Asterisk, LeftParen, RightParen,
            LeftBracket, RightBracket, Backslash,
            CapsLock, Semicolon, Colon, Quotedbl, Quote,
            Less, Comma, Greater, Question, Slash,
            LShift, RShift, Space, LCtrl, RCtrl, LAlt, RAlt, LGui, RGui
        )
    }
}

impl From<mouse::MouseButton> for MouseButton {
    fn from(item: mouse::MouseButton) -> Self {
        use mouse::MouseButton::*;
        match item {
            Left => Self::Left,
            Middle => Self::Middle,
            Right => Self::Right,
            X1 => Self::X1,
            X2 => Self::X2,
            _ => Self::Unknown,
        }
    }
}

impl From<mouse::MouseWheelDirection> for MouseWheelDirection {
    fn from(item: mouse::MouseWheelDirection) -> Self {
        use mouse::MouseWheelDirection::*;
        match item {
            Normal => Self::Normal,
            Flipped => Self::Flipped,
            Unknown(val) => Self::Unknown(val),
        }
    }
}

impl From<controller::Axis> for Axis {
    fn from(item: controller::Axis) -> Self {
        use controller::Axis::*;
        match item {
            LeftX => Self::LeftX,
            RightX => Self::RightX,
            LeftY => Self::LeftY,
            RightY => Self::RightY,
            TriggerLeft => Self::TriggerLeft,
            TriggerRight => Self::TriggerRight,
        }
    }
}

impl From<controller::Button> for Button {
    #[rustfmt::skip]
    fn from(item: controller::Button) -> Self {
        map_button!(
            item,
            A, B, X, Y, Back, Start, Guide, DPadUp, DPadDown, DPadLeft, DPadRight, LeftStick,
            RightStick, LeftShoulder, RightShoulder
        )
    }
}

impl From<keyboard::Mod> for KeyMod {
    fn from(item: keyboard::Mod) -> Self {
        Self::from_bits_truncate(item.bits())
    }
}
