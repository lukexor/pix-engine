//! System and user [`Event`]s.

use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// System or User `Event`.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Event {
    Quit,
    AppTerminating,
    Window {
        window_id: u32,
        win_event: WindowEvent,
    },
    KeyDown {
        key: Option<Key>,
        keymod: KeyMod,
        repeat: bool,
    },
    KeyUp {
        key: Option<Key>,
        keymod: KeyMod,
        repeat: bool,
    },
    TextInput {
        text: String,
    },
    MouseMotion {
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    },
    MouseDown {
        button: Mouse,
        x: i32,
        y: i32,
    },
    MouseUp {
        button: Mouse,
        x: i32,
        y: i32,
    },
    MouseWheel {
        x: i32,
        y: i32,
    },
    JoyAxisMotion {
        joy_id: u32,
        axis_idx: u8,
        value: i16,
    },
    JoyBallMotion {
        joy_id: u32,
        ball_idx: u8,
        xrel: i16,
        yrel: i16,
    },
    JoyDown {
        joy_id: u32,
        button_idx: u8,
    },
    JoyUp {
        joy_id: u32,
        button_idx: u8,
    },
    JoyDeviceAdded {
        joy_id: u32,
    },
    JoyDeviceRemoved {
        joy_id: u32,
    },
    ControllerAxisMotion {
        controller_id: u32,
        axis: Axis,
        value: i16,
    },
    ControllerDown {
        controller_id: u32,
        button: Button,
    },
    ControllerUp {
        controller_id: u32,
        button: Button,
    },
    ControllerAdded {
        controller_id: u32,
    },
    ControllerRemoved {
        controller_id: u32,
    },
    ControllerRemapped {
        controller_id: u32,
    },
    FingerDown {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerUp {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    FingerMotion {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32,
    },
    Unknown,
}

/// A specific [`Event`] representing a keypress.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyEvent {
    pub key: Key,
    pub keymod: KeyMod,
    pub pressed: bool,
    pub repeat: bool,
}

impl Default for Event {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Window [`Event`].
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WindowEvent {
    Shown,
    Hidden,
    Moved(i32, i32),
    Resized(i32, i32),
    SizeChanged(i32, i32),
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
    Unknown,
}

impl Default for WindowEvent {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Mouse Button type.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mouse {
    Left,
    Middle,
    Right,
    Unknown,
}

impl Default for Mouse {
    fn default() -> Self {
        Self::Unknown
    }
}

bitflags! {
    /// Key Modifier.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct KeyMod: u16 {
        /// No modifier
        const NONE = 0x0000;
        /// LShift or RShift
        const SHIFT = 0x0001;
        /// LCtrl or RCtrl
        const CTRL = 0x0040;
        /// LAlt or RAlt
        const ALT = 0x0100;
        /// LGui or RGui
        const GUI = 0x0400;
    }
}

/// Keyboard key.
#[allow(missing_docs)]
#[non_exhaustive]
#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key {
    Backspace, Tab, Return, Escape, Space, Exclaim, Quotedbl, Hash, Dollar, Percent, Ampersand,
    Quote, LeftParen, RightParen, Asterisk, Plus, Comma, Minus, Period, Slash, Num0, Num1, Num2,
    Num3, Num4, Num5, Num6, Num7, Num8, Num9, Colon, Semicolon, Less, Equals, Greater, Question,
    At, LeftBracket, Backslash, RightBracket, Caret, Underscore, Backquote, A, B, C, D, E, F, G, H,
    I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Delete, CapsLock, F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12, PrintScreen, ScrollLock, Pause, Insert, Home, PageUp, End, PageDown,
    Right, Left, Down, Up, NumLock, LCtrl, LShift, LAlt, LGui, RCtrl, RShift, RAlt, RGui, Unknown
}

impl Default for Key {
    fn default() -> Self {
        Self::Unknown
    }
}

/// A Joystick axis
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Axis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
    Unknown,
}

impl Default for Axis {
    fn default() -> Self {
        Self::Unknown
    }
}

/// A Controller button
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Button {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Unknown,
}

impl Default for Button {
    fn default() -> Self {
        Self::Unknown
    }
}
