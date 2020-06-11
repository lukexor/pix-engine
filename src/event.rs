//! Handles User and System level interaction event.

use crate::renderer::sdl::*;

/// Wrapper around a concrete EventIterator.
pub type EventIterator<'a> = SdlEventIterator<'a>;

/// Wrapper around a concrete System or User Event.
pub type Event = SdlEvent;

/// Wrapper around a concrete Window Event.
pub type WindowEvent = SdlWindowEvent;

/// Wrapper around a concrete Mouse Button type.
pub type MouseButton = SdlMouseButton;

/// Wrapper around a concrete Keycode type.
pub type Keycode = SdlKeycode;
