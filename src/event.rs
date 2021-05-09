//! Handles User and System level interaction event.

/// Wrapper around a concrete System or User Event.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Quit,
    AppTerminating,
    Window {
        win_event: WindowEvent,
    },
    KeyDown {
        keycode: Option<Keycode>,
        repeat: bool,
    },
    KeyUp {
        keycode: Option<Keycode>,
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
    MouseButtonDown {
        mouse_btn: MouseButton,
        x: i32,
        y: i32,
    },
    MouseButtonUp {
        mouse_btn: MouseButton,
        x: i32,
        y: i32,
    },
    MouseWheel {
        x: i32,
        y: i32,
    },
    JoyAxisMotion {
        which: u32,
        axis_idx: u8,
        value: i16,
    },
    JoyBallMotion {
        which: u32,
        ball_idx: u8,
        xrel: i16,
        yrel: i16,
    },
    JoyButtonDown {
        which: u32,
        button_idx: u8,
    },
    JoyButtonUp {
        which: u32,
        button_idx: u8,
    },
    JoyDeviceAdded {
        which: u32,
    },
    JoyDeviceRemoved {
        which: u32,
    },
    ControllerAxisMotion {
        which: u32,
        axis: Axis,
        value: i16,
    },
    ControllerButtonDown {
        which: u32,
        button: Button,
    },
    ControllerButtonUp {
        which: u32,
        button: Button,
    },
    ControllerDeviceAdded {
        which: u32,
    },
    ControllerDeviceRemoved {
        which: u32,
    },
    ControllerDeviceRemapped {
        which: u32,
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

/// Wrapper around a concrete Window Event.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

/// Wrapper around a concrete Mouse Button type.
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unknown,
}

/// Wrapper around a concrete Keycode type.
#[allow(missing_docs)]
#[non_exhaustive]
#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Keycode {
    Backspace, Tab, Return, Escape, Space, Exclaim, Quotedbl, Hash, Dollar, Percent, Ampersand,
    Quote, LeftParen, RightParen, Asterisk, Plus, Comma, Minus, Period, Slash, Num0, Num1, Num2,
    Num3, Num4, Num5, Num6, Num7, Num8, Num9, Colon, Semicolon, Less, Equals, Greater, Question,
    At, LeftBracket, Backslash, RightBracket, Caret, Underscore, Backquote, A, B, C, D, E, F, G, H,
    I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Delete, CapsLock, F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12, PrintScreen, ScrollLock, Pause, Insert, Home, PageUp, End, PageDown,
    Right, Left, Down, Up, NumLock, LCtrl, LShift, LAlt, LGui, RCtrl, RShift, RAlt, RGui, Unknown
}

/// A Joystick axis
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
}

/// A Controller button
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
}
