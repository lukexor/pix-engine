//! Translation from `winit` events into the engine's [`Event`].
//!
//! Two things have to be kept between events. Modifiers arrive in an event of their own rather
//! than on each key, and a mouse button press reports no position. Controller events do not pass
//! through here. They come from `gilrs`, which reports every pad whatever window has focus.

use crate::prelude::{Event, Key, KeyMod, Mouse, Point, Scan, WindowEvent};
use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent as WinitEvent},
    keyboard::{Key as WinitKey, KeyCode, ModifiersState, NamedKey, PhysicalKey},
};

/// Pixels a wheel notch scrolls, when a device reports pixels rather than lines.
///
/// The engine's wheel events count notches, so a touchpad that reports pixels is divided down to
/// the same unit.
const PIXELS_PER_LINE: f32 = 32.0;

/// Input state that a single `winit` event does not include.
#[derive(Debug, Default)]
pub(crate) struct Input {
    /// Modifiers held as of the last `ModifiersChanged`.
    keymod: KeyMod,
    /// Cursor position in window pixels, from the last `CursorMoved`.
    cursor: Point<i32>,
}

impl Input {
    /// Appends the engine events a `winit` window event produces.
    ///
    /// A canvas is the size of its window, so a position needs no scaling on the way through.
    pub(crate) fn translate(&mut self, window_id: u32, event: &WinitEvent, out: &mut Vec<Event>) {
        match event {
            WinitEvent::CloseRequested | WinitEvent::Destroyed => out.push(Event::Window {
                window_id,
                win_event: WindowEvent::Close,
            }),
            WinitEvent::Focused(gained) => out.push(Event::Window {
                window_id,
                win_event: if *gained {
                    WindowEvent::FocusGained
                } else {
                    WindowEvent::FocusLost
                },
            }),
            WinitEvent::Resized(size) => {
                #[allow(clippy::cast_possible_wrap)]
                let (width, height) = (size.width as i32, size.height as i32);
                out.push(Event::Window {
                    window_id,
                    win_event: WindowEvent::Resized(width, height),
                });
                out.push(Event::Window {
                    window_id,
                    win_event: WindowEvent::SizeChanged(width, height),
                });
            }
            WinitEvent::Moved(position) => out.push(Event::Window {
                window_id,
                win_event: WindowEvent::Moved(position.x, position.y),
            }),
            WinitEvent::Occluded(occluded) => out.push(Event::Window {
                window_id,
                win_event: if *occluded {
                    WindowEvent::Hidden
                } else {
                    WindowEvent::Shown
                },
            }),
            WinitEvent::CursorEntered { .. } => out.push(Event::Window {
                window_id,
                win_event: WindowEvent::Enter,
            }),
            WinitEvent::CursorLeft { .. } => out.push(Event::Window {
                window_id,
                win_event: WindowEvent::Leave,
            }),
            WinitEvent::ModifiersChanged(modifiers) => {
                self.keymod = keymod(modifiers.state());
            }
            WinitEvent::KeyboardInput { event, .. } => {
                let scan = match event.physical_key {
                    PhysicalKey::Code(code) => scan(code),
                    PhysicalKey::Unidentified(_) => Scan::Unhandled,
                };
                let key = key(&event.logical_key);
                // A modifier key's own event folds in the bit it stands for. `ModifiersChanged`
                // can arrive after it, and the engine reads the modifiers it last saw on a key
                // event when a later mouse click asks which are down.
                let mut keymod = self.keymod;
                if let Some(bit) = modifier(&event.logical_key) {
                    keymod.set(bit, event.state.is_pressed());
                }
                match event.state {
                    ElementState::Pressed => {
                        out.push(Event::KeyDown {
                            key,
                            keymod,
                            repeat: event.repeat,
                            scan: Some(scan),
                        });
                        // A key that produces text reports it here. A chord held with a control
                        // or command key is a shortcut, and its text stays out of a text field.
                        let chord = keymod.intersects(KeyMod::CTRL | KeyMod::GUI);
                        if let Some(text) = &event.text {
                            if !chord && !text.chars().any(char::is_control) {
                                out.push(Event::TextInput {
                                    text: text.to_string(),
                                });
                            }
                        }
                    }
                    ElementState::Released => out.push(Event::KeyUp {
                        key,
                        keymod,
                        repeat: event.repeat,
                        scan: Some(scan),
                    }),
                }
            }
            WinitEvent::Ime(winit::event::Ime::Commit(text)) => {
                out.push(Event::TextInput { text: text.clone() });
            }
            WinitEvent::CursorMoved { position, .. } => {
                #[allow(clippy::cast_possible_truncation)]
                let pos = Point::new([position.x as i32, position.y as i32]);
                let previous = self.cursor;
                self.cursor = pos;
                out.push(Event::MouseMotion {
                    x: pos.x(),
                    y: pos.y(),
                    xrel: pos.x() - previous.x(),
                    yrel: pos.y() - previous.y(),
                });
            }
            WinitEvent::MouseInput { state, button, .. } => {
                let button = mouse_button(*button);
                let (x, y) = (self.cursor.x(), self.cursor.y());
                out.push(match state {
                    ElementState::Pressed => Event::MouseDown { button, x, y },
                    ElementState::Released => Event::MouseUp { button, x, y },
                });
            }
            WinitEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (*x, *y),
                    MouseScrollDelta::PixelDelta(delta) =>
                    {
                        #[allow(clippy::cast_possible_truncation)]
                        (
                            delta.x as f32 / PIXELS_PER_LINE,
                            delta.y as f32 / PIXELS_PER_LINE,
                        )
                    }
                };
                #[allow(clippy::cast_possible_truncation)]
                out.push(Event::MouseWheel {
                    x: x.round() as i32,
                    y: y.round() as i32,
                });
            }
            _ => (),
        }
    }
}

/// Returns the modifiers the engine reports for a `winit` modifier state.
fn keymod(state: ModifiersState) -> KeyMod {
    let mut keymod = KeyMod::NONE;
    keymod.set(KeyMod::SHIFT, state.shift_key());
    keymod.set(KeyMod::CTRL, state.control_key());
    keymod.set(KeyMod::ALT, state.alt_key());
    keymod.set(KeyMod::GUI, state.super_key());
    keymod
}

/// Returns the modifier a key stands for, or `None` for a key that is not one.
fn modifier(key: &WinitKey) -> Option<KeyMod> {
    let WinitKey::Named(named) = key else {
        return None;
    };
    match named {
        NamedKey::Shift => Some(KeyMod::SHIFT),
        NamedKey::Control => Some(KeyMod::CTRL),
        NamedKey::Alt => Some(KeyMod::ALT),
        NamedKey::Super => Some(KeyMod::GUI),
        _ => None,
    }
}

/// Returns the mouse button the engine reports for a `winit` button.
fn mouse_button(button: MouseButton) -> Mouse {
    match button {
        MouseButton::Left => Mouse::Left,
        MouseButton::Middle => Mouse::Middle,
        MouseButton::Right => Mouse::Right,
        _ => Mouse::Unhandled,
    }
}

/// Returns the key a logical `winit` key stands for.
///
/// A logical key is the one the layout produced, so a shifted digit arrives as its symbol and maps
/// straight through the character table. `None` marks a key the engine does not model.
fn key(key: &WinitKey) -> Option<Key> {
    let key = match key {
        WinitKey::Character(text) => return text.chars().next().and_then(character),
        WinitKey::Named(named) => match named {
            NamedKey::Backspace => Key::Backspace,
            NamedKey::Tab => Key::Tab,
            NamedKey::Enter => Key::Return,
            NamedKey::Escape => Key::Escape,
            NamedKey::Space => Key::Space,
            NamedKey::Delete => Key::Delete,
            NamedKey::CapsLock => Key::CapsLock,
            NamedKey::F1 => Key::F1,
            NamedKey::F2 => Key::F2,
            NamedKey::F3 => Key::F3,
            NamedKey::F4 => Key::F4,
            NamedKey::F5 => Key::F5,
            NamedKey::F6 => Key::F6,
            NamedKey::F7 => Key::F7,
            NamedKey::F8 => Key::F8,
            NamedKey::F9 => Key::F9,
            NamedKey::F10 => Key::F10,
            NamedKey::F11 => Key::F11,
            NamedKey::F12 => Key::F12,
            NamedKey::PrintScreen => Key::PrintScreen,
            NamedKey::ScrollLock => Key::ScrollLock,
            NamedKey::Pause => Key::Pause,
            NamedKey::Insert => Key::Insert,
            NamedKey::Home => Key::Home,
            NamedKey::PageUp => Key::PageUp,
            NamedKey::End => Key::End,
            NamedKey::PageDown => Key::PageDown,
            NamedKey::ArrowRight => Key::Right,
            NamedKey::ArrowLeft => Key::Left,
            NamedKey::ArrowDown => Key::Down,
            NamedKey::ArrowUp => Key::Up,
            NamedKey::NumLock => Key::NumLock,
            NamedKey::Control => Key::LCtrl,
            NamedKey::Shift => Key::LShift,
            NamedKey::Alt => Key::LAlt,
            NamedKey::Super => Key::LGui,
            _ => return None,
        },
        _ => return None,
    };
    Some(key)
}

/// Returns the key a character stands for.
///
/// Letters fold to upper case, matching the single variant the engine has per letter.
fn character(character: char) -> Option<Key> {
    let key = match character.to_ascii_uppercase() {
        'A' => Key::A,
        'B' => Key::B,
        'C' => Key::C,
        'D' => Key::D,
        'E' => Key::E,
        'F' => Key::F,
        'G' => Key::G,
        'H' => Key::H,
        'I' => Key::I,
        'J' => Key::J,
        'K' => Key::K,
        'L' => Key::L,
        'M' => Key::M,
        'N' => Key::N,
        'O' => Key::O,
        'P' => Key::P,
        'Q' => Key::Q,
        'R' => Key::R,
        'S' => Key::S,
        'T' => Key::T,
        'U' => Key::U,
        'V' => Key::V,
        'W' => Key::W,
        'X' => Key::X,
        'Y' => Key::Y,
        'Z' => Key::Z,
        '0' => Key::Num0,
        '1' => Key::Num1,
        '2' => Key::Num2,
        '3' => Key::Num3,
        '4' => Key::Num4,
        '5' => Key::Num5,
        '6' => Key::Num6,
        '7' => Key::Num7,
        '8' => Key::Num8,
        '9' => Key::Num9,
        ' ' => Key::Space,
        '!' => Key::Exclaim,
        '"' => Key::Quotedbl,
        '#' => Key::Hash,
        '$' => Key::Dollar,
        '%' => Key::Percent,
        '&' => Key::Ampersand,
        '\'' => Key::Quote,
        '(' => Key::LeftParen,
        ')' => Key::RightParen,
        '*' => Key::Asterisk,
        '+' => Key::Plus,
        ',' => Key::Comma,
        '-' => Key::Minus,
        '.' => Key::Period,
        '/' => Key::Slash,
        ':' => Key::Colon,
        ';' => Key::Semicolon,
        '<' => Key::Less,
        '=' => Key::Equals,
        '>' => Key::Greater,
        '?' => Key::Question,
        '@' => Key::At,
        '[' => Key::LeftBracket,
        '\\' => Key::Backslash,
        ']' => Key::RightBracket,
        '^' => Key::Caret,
        '_' => Key::Underscore,
        '`' => Key::Backquote,
        _ => return None,
    };
    Some(key)
}

/// Returns the scancode the engine reports for a physical `winit` key.
///
/// A scancode names a position on the keyboard whatever the layout maps it to, so this table is
/// the one a game reads when it wants `WASD` to stay under the same fingers.
#[rustfmt::skip]
fn scan(code: KeyCode) -> Scan {
    match code {
        KeyCode::KeyA => Scan::A,
        KeyCode::KeyB => Scan::B,
        KeyCode::KeyC => Scan::C,
        KeyCode::KeyD => Scan::D,
        KeyCode::KeyE => Scan::E,
        KeyCode::KeyF => Scan::F,
        KeyCode::KeyG => Scan::G,
        KeyCode::KeyH => Scan::H,
        KeyCode::KeyI => Scan::I,
        KeyCode::KeyJ => Scan::J,
        KeyCode::KeyK => Scan::K,
        KeyCode::KeyL => Scan::L,
        KeyCode::KeyM => Scan::M,
        KeyCode::KeyN => Scan::N,
        KeyCode::KeyO => Scan::O,
        KeyCode::KeyP => Scan::P,
        KeyCode::KeyQ => Scan::Q,
        KeyCode::KeyR => Scan::R,
        KeyCode::KeyS => Scan::S,
        KeyCode::KeyT => Scan::T,
        KeyCode::KeyU => Scan::U,
        KeyCode::KeyV => Scan::V,
        KeyCode::KeyW => Scan::W,
        KeyCode::KeyX => Scan::X,
        KeyCode::KeyY => Scan::Y,
        KeyCode::KeyZ => Scan::Z,
        KeyCode::Digit1 => Scan::Num1,
        KeyCode::Digit2 => Scan::Num2,
        KeyCode::Digit3 => Scan::Num3,
        KeyCode::Digit4 => Scan::Num4,
        KeyCode::Digit5 => Scan::Num5,
        KeyCode::Digit6 => Scan::Num6,
        KeyCode::Digit7 => Scan::Num7,
        KeyCode::Digit8 => Scan::Num8,
        KeyCode::Digit9 => Scan::Num9,
        KeyCode::Digit0 => Scan::Num0,
        KeyCode::Enter => Scan::Return,
        KeyCode::Escape => Scan::Escape,
        KeyCode::Backspace => Scan::Backspace,
        KeyCode::Tab => Scan::Tab,
        KeyCode::Space => Scan::Space,
        KeyCode::Minus => Scan::Minus,
        KeyCode::Equal => Scan::Equals,
        KeyCode::BracketLeft => Scan::LeftBracket,
        KeyCode::BracketRight => Scan::RightBracket,
        KeyCode::Backslash => Scan::Backslash,
        KeyCode::Semicolon => Scan::Semicolon,
        KeyCode::Quote => Scan::Apostrophe,
        KeyCode::Backquote => Scan::Grave,
        KeyCode::Comma => Scan::Comma,
        KeyCode::Period => Scan::Period,
        KeyCode::Slash => Scan::Slash,
        KeyCode::CapsLock => Scan::CapsLock,
        KeyCode::F1 => Scan::F1,
        KeyCode::F2 => Scan::F2,
        KeyCode::F3 => Scan::F3,
        KeyCode::F4 => Scan::F4,
        KeyCode::F5 => Scan::F5,
        KeyCode::F6 => Scan::F6,
        KeyCode::F7 => Scan::F7,
        KeyCode::F8 => Scan::F8,
        KeyCode::F9 => Scan::F9,
        KeyCode::F10 => Scan::F10,
        KeyCode::F11 => Scan::F11,
        KeyCode::F12 => Scan::F12,
        KeyCode::F13 => Scan::F13,
        KeyCode::F14 => Scan::F14,
        KeyCode::F15 => Scan::F15,
        KeyCode::F16 => Scan::F16,
        KeyCode::F17 => Scan::F17,
        KeyCode::PrintScreen => Scan::PrintScreen,
        KeyCode::ScrollLock => Scan::ScrollLock,
        KeyCode::Pause => Scan::Pause,
        KeyCode::Insert => Scan::Insert,
        KeyCode::Home => Scan::Home,
        KeyCode::PageUp => Scan::PageUp,
        KeyCode::Delete => Scan::Delete,
        KeyCode::End => Scan::End,
        KeyCode::PageDown => Scan::PageDown,
        KeyCode::ArrowRight => Scan::Right,
        KeyCode::ArrowLeft => Scan::Left,
        KeyCode::ArrowDown => Scan::Down,
        KeyCode::ArrowUp => Scan::Up,
        KeyCode::NumLock => Scan::NumLockClear,
        KeyCode::NumpadDivide => Scan::KpDivide,
        KeyCode::NumpadMultiply => Scan::KpMultiply,
        KeyCode::NumpadSubtract => Scan::KpMinus,
        KeyCode::NumpadAdd => Scan::KpPlus,
        KeyCode::NumpadEnter => Scan::KpEnter,
        KeyCode::Numpad1 => Scan::Kp1,
        KeyCode::Numpad2 => Scan::Kp2,
        KeyCode::Numpad3 => Scan::Kp3,
        KeyCode::Numpad4 => Scan::Kp4,
        KeyCode::Numpad5 => Scan::Kp5,
        KeyCode::Numpad6 => Scan::Kp6,
        KeyCode::Numpad7 => Scan::Kp7,
        KeyCode::Numpad8 => Scan::Kp8,
        KeyCode::Numpad9 => Scan::Kp9,
        KeyCode::Numpad0 => Scan::Kp0,
        KeyCode::NumpadDecimal => Scan::KpPeriod,
        KeyCode::NumpadEqual => Scan::KpEquals,
        KeyCode::NumpadComma => Scan::KpComma,
        KeyCode::IntlBackslash => Scan::NonUsBackslash,
        KeyCode::ContextMenu => Scan::Application,
        KeyCode::Power => Scan::Power,
        KeyCode::Help => Scan::Help,
        KeyCode::Select => Scan::Select,
        KeyCode::Again => Scan::Again,
        KeyCode::Undo => Scan::Undo,
        KeyCode::Cut => Scan::Cut,
        KeyCode::Copy => Scan::Copy,
        KeyCode::Paste => Scan::Paste,
        KeyCode::Find => Scan::Find,
        KeyCode::AudioVolumeMute => Scan::Mute,
        KeyCode::AudioVolumeUp => Scan::VolumeUp,
        KeyCode::AudioVolumeDown => Scan::VolumeDown,
        KeyCode::ControlLeft => Scan::LCtrl,
        KeyCode::ShiftLeft => Scan::LShift,
        KeyCode::AltLeft => Scan::LAlt,
        KeyCode::SuperLeft => Scan::LGui,
        KeyCode::ControlRight => Scan::RCtrl,
        KeyCode::ShiftRight => Scan::RShift,
        KeyCode::AltRight => Scan::RAlt,
        KeyCode::SuperRight => Scan::RGui,
        KeyCode::Lang1 => Scan::Lang1,
        KeyCode::Lang2 => Scan::Lang2,
        KeyCode::Lang3 => Scan::Lang3,
        KeyCode::Lang4 => Scan::Lang4,
        KeyCode::Lang5 => Scan::Lang5,
        _ => Scan::Unhandled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shifted_digit_maps_to_its_symbol() {
        // The logical key reports what the layout produced, so `Shift+1` arrives as `!`.
        assert_eq!(character('!'), Some(Key::Exclaim));
        assert_eq!(character('1'), Some(Key::Num1));
    }

    #[test]
    fn a_letter_maps_to_one_variant_whatever_its_case() {
        assert_eq!(character('a'), Some(Key::A));
        assert_eq!(character('A'), Some(Key::A));
    }

    #[test]
    fn a_scancode_names_a_position_not_a_layout() {
        // `KeyA` is the key left of `S` on any layout, including one that types `Q` there.
        assert_eq!(scan(KeyCode::KeyA), Scan::A);
        assert_eq!(scan(KeyCode::Fn), Scan::Unhandled);
    }
}
