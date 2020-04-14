use bitflags::bitflags;
use std::fmt;

// Represents an input event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PixEvent {
    Quit {
        timestamp: u32,
    },
    AppTerminating {
        timestamp: u32,
    },
    AppWillEnterBackground {
        timestamp: u32,
    },
    AppDidEnterBackground {
        timestamp: u32,
    },
    AppWillEnterForeground {
        timestamp: u32,
    },
    AppDidEnterForeground {
        timestamp: u32,
    },
    Window {
        timestamp: u32,
        window_id: u32,
        win_event: WindowEvent,
    },
    KeyDown {
        timestamp: u32,
        window_id: u32,
        key: Key,
        keymod: KeyMod,
        repeat: bool,
    },
    KeyUp {
        timestamp: u32,
        window_id: u32,
        key: Key,
        keymod: KeyMod,
        repeat: bool,
    },
    TextEditing {
        timestamp: u32,
        window_id: u32,
        text: String,
        start: i32,
        length: i32,
    },
    TextInput {
        timestamp: u32,
        window_id: u32,
        text: String,
    },
    MouseMotion {
        timestamp: u32,
        window_id: u32,
        which: u32,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    },
    MouseButtonDown {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32,
    },
    MouseButtonUp {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32,
    },
    MouseWheel {
        timestamp: u32,
        window_id: u32,
        which: u32,
        x: i32,
        y: i32,
        direction: MouseWheelDirection,
    },
    JoyAxisMotion {
        timestamp: u32,
        which: u32,
        axis_idx: u8,
        value: i16,
    },
    JoyButtonDown {
        timestamp: u32,
        which: u32,
        button_idx: u8,
    },
    JoyButtonUp {
        timestamp: u32,
        which: u32,
        button_idx: u8,
    },
    JoyDeviceAdded {
        timestamp: u32,
        which: u32,
    },
    JoyDeviceRemoved {
        timestamp: u32,
        which: u32,
    },
    ControllerAxisMotion {
        timestamp: u32,
        which: u32,
        axis: Axis,
        value: i16,
    },
    ControllerButtonDown {
        timestamp: u32,
        which: u32,
        button: Button,
    },
    ControllerButtonUp {
        timestamp: u32,
        which: u32,
        button: Button,
    },
    ControllerDeviceAdded {
        timestamp: u32,
        which: u32,
    },
    ControllerDeviceRemoved {
        timestamp: u32,
        which: u32,
    },
    ControllerDeviceRemapped {
        timestamp: u32,
        which: u32,
    },
    Unknown {
        timestamp: u32,
        type_: u32,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WindowEvent {
    None,
    Shown,
    Hidden,
    Exposed,
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
    TakeFocus,
    HitTest,
}

bitflags! {
    pub struct KeyMod: u16 {
        const NOMOD = 0x0000;
        const LSHIFTMOD = 0x0001;
        const RSHIFTMOD = 0x0002;
        const LCTRLMOD = 0x0040;
        const RCTRLMOD = 0x0080;
        const LALTMOD = 0x0100;
        const RALTMOD = 0x0200;
        const LGUIMOD = 0x0400;
        const RGUIMOD = 0x0800;
        const NUMMOD = 0x1000;
        const CAPSMOD = 0x2000;
        const MODEMOD = 0x4000;
        const RESERVEDMOD = 0x8000;
    }
}

impl fmt::Display for KeyMod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04x}", *self)
    }
}

/// Represents a mouse button
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
    Unknown,
}

/// Represents a mouse wheel direction
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseWheelDirection {
    Normal,
    Flipped,
    Unknown(u32),
}

/// A non-exhaustive list of useful keys to detect
#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
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
    LShift, RShift, Space, LCtrl, RCtrl, LAlt, RAlt, LGui, RGui,
    Unknown,
}

/// Controller buttons
#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Button {
    A, B, X, Y, Back, Start, Guide, DPadUp, DPadDown, DPadLeft, DPadRight,
    LeftStick, RightStick, LeftShoulder, RightShoulder,
}

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    LeftX, RightX, LeftY, RightY, TriggerLeft, TriggerRight,
}

impl Default for PixEvent {
    fn default() -> Self {
        Self::Unknown {
            timestamp: 0,
            type_: 0,
        }
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::Left
    }
}

impl Default for MouseWheelDirection {
    fn default() -> Self {
        Self::Normal
    }
}

impl Default for Key {
    fn default() -> Self {
        Self::A
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::A
    }
}

impl Default for Axis {
    fn default() -> Self {
        Self::LeftX
    }
}
