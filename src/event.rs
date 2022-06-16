//! User and system [Event]s.

use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

/// System or User `Event`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Event {
    /// System quit event for the application.
    Quit,
    /// System termination event for the application.
    AppTerminating,
    /// Window events.
    Window {
        /// Window identifer for this event.
        window_id: u32,
        /// Specific window event.
        win_event: WindowEvent,
    },
    /// User key press event.
    KeyDown {
        /// Specific key being pressed.
        key: Option<Key>,
        /// Key modifiers being held upon press, e.g. Shift or Ctrl, etc.
        keymod: KeyMod,
        /// Whether this is a key-repeat event.
        repeat: bool,
    },
    /// User key release event.
    KeyUp {
        /// Specific key being released.
        key: Option<Key>,
        /// Key modifiers being held upon release, e.g. Shift or Ctrl, etc.
        keymod: KeyMod,
        /// Whether this is a key-repeat event.
        repeat: bool,
    },
    /// User text entry event.
    TextInput {
        /// The user-entered text.
        text: String,
    },
    /// User mouse movement event.
    MouseMotion {
        /// Current horizontal mouse position after motion.
        x: i32,
        /// Current vertical mouse position after motion.
        y: i32,
        /// Relative horizontal screen movement since last event.
        xrel: i32,
        /// Relative vertical screen movement since last event.
        yrel: i32,
    },
    /// User mouse click event.
    MouseDown {
        /// Specific mouse button being clicked.
        button: Mouse,
        /// Current horizontal mouse position after click.
        x: i32,
        /// Current vertical mouse position after click.
        y: i32,
    },
    /// User mouse release event.
    MouseUp {
        /// Specific mouse button being released.
        button: Mouse,
        /// Current horizontal mouse position after release.
        x: i32,
        /// Current vertical mouse position after release.
        y: i32,
    },
    /// User mouse wheel event.
    MouseWheel {
        /// Relative horizontal wheel offset.
        x: i32,
        /// Relative vertical wheel offset.
        y: i32,
    },
    /// User joystick axis movement event.
    JoyAxisMotion {
        /// Specific attached joystick identifier.
        joy_id: u32,
        /// Specific joystick axis being moved.
        axis_idx: u8,
        /// Relative value of axis motion.
        value: i16,
    },
    /// User joystick hat movement event.
    JoyHatMotion {
        /// Specific attached joystick identifier.
        joy_id: u32,
        /// Specific joystick hat being moved.
        hat_idx: u8,
        /// Hat state.
        state: HatState,
    },
    /// User joystick ball movement event.
    JoyBallMotion {
        /// Specific attached joystick identifier.
        joy_id: u32,
        /// Specific joystick ball being moved.
        ball_idx: u8,
        /// Relative horizontal value of ball motion.
        xrel: i16,
        /// Relative vertical value of ball motion.
        yrel: i16,
    },
    /// User joystick button pressed event.
    JoyDown {
        /// Specific attached joystick identifier.
        joy_id: u32,
        /// Specific joystick button being pressed.
        button_idx: u8,
    },
    /// User joystick button released event.
    JoyUp {
        /// Specific attached joystick identifier.
        joy_id: u32,
        /// Specific joystick button being released.
        button_idx: u8,
    },
    /// User joystick connected event.
    JoyDeviceAdded {
        /// Specific attached joystick identifier.
        joy_id: u32,
    },
    /// User joystick disconnected event.
    JoyDeviceRemoved {
        /// Specific attached joystick identifier.
        joy_id: u32,
    },
    /// User controller axis movement event.
    ControllerAxisMotion {
        /// Specific attached controller identifier.
        controller_id: u32,
        /// Specific controller axis being moved.
        axis: Axis,
        /// Relative value of axis motion.
        value: i16,
    },
    /// User controller button pressed event.
    ControllerDown {
        /// Specific attached controller identifier.
        controller_id: u32,
        /// Specific controller button being pressed.
        button: ControllerButton,
    },
    /// User controller button released event.
    ControllerUp {
        /// Specific attached controller identifier.
        controller_id: u32,
        /// Specific controller button being released.
        button: ControllerButton,
    },
    /// User controller connected event.
    ControllerAdded {
        /// Specific attached controller identifier.
        controller_id: u32,
    },
    /// User controller disconnected event.
    ControllerRemoved {
        /// Specific attached controller identifier.
        controller_id: u32,
    },
    /// User controller remapped event.
    ControllerRemapped {
        /// Specific attached controller identifier.
        controller_id: u32,
    },
    /// User finger press event.
    FingerDown {
        /// Specific touch device identifier.
        touch_id: i64,
        /// Specific finger identifier.
        finger_id: i64,
        /// Current horizontal finger position after press.
        x: f32,
        /// Current vertical finger position after press.
        y: f32,
        /// Relative horizontal finger position since last event.
        dx: f32,
        /// Relative vertical finger position since last event.
        dy: f32,
        /// Amount of finger pressure being applied during press.
        pressure: f32,
    },
    /// User finger released event.
    FingerUp {
        /// Specific touch device identifier.
        touch_id: i64,
        /// Specific finger identifier.
        finger_id: i64,
        /// Current horizontal finger position after press.
        x: f32,
        /// Current vertical finger position after press.
        y: f32,
        /// Relative horizontal finger position since last event.
        dx: f32,
        /// Relative vertical finger position since last event.
        dy: f32,
        /// Amount of finger pressure being applied during press.
        pressure: f32,
    },
    /// User finger movement event.
    FingerMotion {
        /// Specific touch device identifier.
        touch_id: i64,
        /// Specific finger identifier.
        finger_id: i64,
        /// Current horizontal finger position after press.
        x: f32,
        /// Current vertical finger position after press.
        y: f32,
        /// Relative horizontal finger position since last event.
        dx: f32,
        /// Relative vertical finger position since last event.
        dy: f32,
        /// Amount of finger pressure being applied during press.
        pressure: f32,
    },
    /// Audio device connected event.
    AudioDeviceAdded {
        /// Specific audio device identifier.
        device_id: u32,
        /// Whether this device is a capture device or not.
        iscapture: bool,
    },
    /// Audio device disconnected event.
    AudioDeviceRemoved {
        /// Specific audio device identifier.
        device_id: u32,
        /// Whether this device is a capture device or not.
        iscapture: bool,
    },
    /// An unknown/unsupported event.
    Unsupported,
}

impl Default for Event {
    fn default() -> Self {
        Self::Unsupported
    }
}

/// A specific [Event] representing a keypress.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyEvent {
    /// Specific key for this event.
    pub key: Key,
    /// Key modifiers being held upon press, e.g. Shift or Ctrl, etc.
    pub keymod: KeyMod,
    /// Whether this is a key-repeat event.
    pub repeat: bool,
}

impl KeyEvent {
    pub(crate) const fn new(key: Key, keymod: KeyMod, repeat: bool) -> Self {
        Self {
            key,
            keymod,
            repeat,
        }
    }
}

/// Window [Event].
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WindowEvent {
    /// Window is being shown.
    Shown,
    /// Window is being exposed.
    Exposed,
    /// Window is being hidden.
    Hidden,
    /// Window moved to new position `(x, y)`
    Moved(i32, i32),
    /// Window resized to new dimensions `(width, height
    Resized(i32, i32),
    /// Window size changed to new dimensions `(width, height
    SizeChanged(i32, i32),
    /// Window minimized.
    Minimized,
    /// Window maximized.
    Maximized,
    /// Window restored.
    Restored,
    /// Users mouse entered the window.
    Enter,
    /// Users mouse left the window.
    Leave,
    /// Window gained user focus.
    FocusGained,
    /// Window lost user focus.
    FocusLost,
    /// Window closed.
    Close,
    /// An unknown/unsupported window event.
    Unsupported,
}

impl Default for WindowEvent {
    fn default() -> Self {
        Self::Unsupported
    }
}

/// Mouse Button type.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Mouse {
    /// Left mouse button.
    Left,
    /// Middle mouse wheel/button.
    Middle,
    /// Right mouse button.
    Right,
    /// An unknown/unsupported mouse button.
    Unsupported,
}

impl Default for Mouse {
    fn default() -> Self {
        Self::Unsupported
    }
}

bitflags! {
    /// Key Modifier.
    #[derive(Default)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[must_use]
    pub struct KeyMod: u16 {
        /// No key modifier.
        const NONE = 0x0000;
        /// Left Shift or Right Shift.
        const SHIFT = 0x0001;
        /// Left Control or Right Control.
        const CTRL = 0x0040;
        /// Left Alt/Option or Right Alt/Option.
        const ALT = 0x0100;
        /// Left GUI or Right GUI (e.g. Windows or Command keys).
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
    Right, Left, Down, Up, NumLock, KpDivide, KpMultiply, KpMinus, KpPlus, KpEnter, Kp1, Kp2, Kp3,
    Kp4, Kp5, Kp6, Kp7, Kp8, Kp9, Kp0, KpPeriod, KpEquals, KpComma, LCtrl, LShift, LAlt, LGui,
    RCtrl, RShift, RAlt, RGui, Unsupported
}

impl Default for Key {
    fn default() -> Self {
        Self::Unsupported
    }
}

/// A Joystick axis.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Axis {
    /// Left horizontal axis.
    LeftX,
    /// Left vertical axis.
    LeftY,
    /// Right horizontal axis.
    RightX,
    /// Left horizontal axis.
    RightY,
    /// Left trigger switch.
    TriggerLeft,
    /// Right trigger switch.
    TriggerRight,
    /// An unknown/unsupported axis.
    Unsupported,
}

impl Default for Axis {
    fn default() -> Self {
        Self::Unsupported
    }
}

/// A Joystick hat state.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HatState {
    /// Left+Up state.
    LeftUp,
    /// Left state.
    Left,
    /// Left+Down state.
    LeftDown,
    /// Up state.
    Up,
    /// Centered state.
    Centered,
    /// Down state.
    Down,
    /// Right+Up state.
    RightUp,
    /// Right state.
    Right,
    /// Right+Down state.
    RightDown,
}

impl Default for HatState {
    fn default() -> Self {
        Self::Centered
    }
}

/// A Controller button
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ControllerButton {
    /// A button.
    A,
    /// B button.
    B,
    /// X button.
    X,
    /// Y button.
    Y,
    /// Back button.
    Back,
    /// Guide button.
    Guide,
    /// Start button.
    Start,
    /// Left axis button.
    LeftStick,
    /// Right axis button.
    RightStick,
    /// Left shoulder button.
    LeftShoulder,
    /// Right shoulder button.
    RightShoulder,
    /// Directional pad up button.
    DPadUp,
    /// Directional pad down button.
    DPadDown,
    /// Directional pad left button.
    DPadLeft,
    /// Directional pad right button.
    DPadRight,
    /// Misc Controller button
    /// - Xbox Series X share button
    /// - PS5 microphone button
    /// - Nintendo Switch Pro capture button
    /// - Amazon Luna microphone button
    Misc1,
    /// Xbox Elite paddle P1
    Paddle1,
    /// Xbox Elite paddle P2
    Paddle2,
    /// Xbox Elite paddle P3
    Paddle3,
    /// Xbox Elite paddle P4
    Paddle4,
    /// PS4/PS5 touchpad button
    Touchpad,
    /// An unknown/unsupported button
    Unsupported,
}

impl Default for ControllerButton {
    fn default() -> Self {
        Self::Unsupported
    }
}

/// `Controller` identifier used to reference attached controllers.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ControllerId(pub(crate) u32);

impl fmt::Display for ControllerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ControllerId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ControllerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// `Controller` update event.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ControllerUpdate {
    /// A controller was attached.
    Added,
    /// A controller was unattached.
    Removed,
    /// A controller has been remapped.
    Remapped,
}

/// A specific [Event] representing a controller button press.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ControllerEvent {
    /// The Identifier for this controller.
    pub controller_id: ControllerId,
    /// Specific button for this event.
    pub button: ControllerButton,
}

impl ControllerEvent {
    pub(crate) const fn new(controller_id: u32, button: ControllerButton) -> Self {
        Self {
            controller_id: ControllerId(controller_id),
            button,
        }
    }
}
