//! SDL Renderer implementation

use super::{
    state::{environment::WindowId, settings::BlendMode},
    Error, Position, RendererSettings, Rendering, Result,
};
use crate::{
    color::Color,
    event::{Axis, Button, Event, Key, Mouse, WindowEvent},
    image::Image,
    shape::Rect,
};
use sdl2::{
    audio::{AudioQueue, AudioSpecDesired},
    gfx::primitives::{DrawRenderer, ToColor},
    image::LoadSurface,
    pixels::PixelFormatEnum,
    render::{Canvas, TextureCreator, TextureQuery, TextureValueError, UpdateTextureError},
    surface::Surface,
    ttf::{self, FontError, InitError},
    video::{FullscreenType, Window, WindowBuildError, WindowContext},
    EventPump, IntegerOrSdlError, Sdl,
};
use std::{borrow::Cow, convert::TryFrom, ffi::NulError};

type SdlAxis = sdl2::controller::Axis;
type SdlButton = sdl2::controller::Button;
type SdlMouseButton = sdl2::mouse::MouseButton;
type SdlKeycode = sdl2::keyboard::Keycode;
type SdlWindowEvent = sdl2::event::WindowEvent;
type SdlEvent = sdl2::event::Event;
type SdlColor = sdl2::pixels::Color;
type SdlRect = sdl2::rect::Rect;
type SdlBlendMode = sdl2::render::BlendMode;

/// An SDL [`Renderer`] implementation.
pub struct Renderer {
    context: Sdl,
    ttf_context: ttf::Sdl2TtfContext,
    event_pump: EventPump,
    window_id: WindowId,
    canvas: Canvas<Window>,
    audio_device: AudioQueue<f32>,
    texture_creator: TextureCreator<WindowContext>,
    blend_mode: SdlBlendMode,
}

impl Rendering for Renderer {
    /// Initializes the Sdl2Renderer using the given settings and opens a new window.
    fn init(s: RendererSettings) -> Result<Self> {
        let context = sdl2::init()?;
        let ttf_context = ttf::init()?;
        let video_subsys = context.video()?;
        let event_pump = context.event_pump()?;

        // Set up window with options
        let win_width = (s.scale_x * s.width as f32).floor() as u32;
        let win_height = (s.scale_y * s.height as f32).floor() as u32;
        let mut window_builder = video_subsys.window(&s.title, win_width, win_height);
        match (s.x, s.y) {
            (Position::Centered, Position::Centered) => {
                window_builder.position_centered();
            }
            (Position::Positioned(x), Position::Positioned(y)) => {
                window_builder.position(x, y);
            }
            _ => return Err(Error::InvalidPosition(s.x, s.y)),
        };
        if s.fullscreen {
            window_builder.fullscreen();
        }
        if s.resizable {
            window_builder.resizable();
        }

        let window = window_builder.build()?;
        let window_id = window.id();
        let mut canvas_builder = window.into_canvas().accelerated().target_texture();
        if s.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let mut canvas = canvas_builder.build()?;
        canvas.set_logical_size(win_width, win_height)?;
        canvas.set_scale(s.scale_x, s.scale_y)?;

        if let Some(icon) = &s.icon {
            let surface = Surface::from_file(icon)?;
            canvas.window_mut().set_icon(surface);
        }

        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        // Set up Audio
        let audio_sub = context.audio()?;
        let desired_spec = AudioSpecDesired {
            freq: Some(s.audio_sample_rate),
            channels: Some(1), // TODO: Add stereo option
            samples: None,
        };
        let audio_device = audio_sub.open_queue(None, &desired_spec)?;
        audio_device.resume();

        Ok(Self {
            context,
            ttf_context,
            event_pump,
            window_id,
            canvas,
            audio_device,
            texture_creator,
            blend_mode: SdlBlendMode::None,
        })
    }

    /// Get the primary window id.
    fn window_id(&self) -> WindowId {
        self.window_id
    }

    /// Clears the canvas to the current clear color.
    fn clear(&mut self) {
        self.canvas.clear();
    }

    /// Set whether the cursor is shown or not.
    fn cursor(&mut self, show: bool) {
        self.context.mouse().show_cursor(show);
    }

    /// Sets the color used by the renderer to draw to the current canvas.
    fn draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    /// Sets the clip rect used by the renderer to draw to the current canvas.
    fn clip(&mut self, rect: Option<Rect>) {
        let rect = rect.map(|rect| rect.into());
        self.canvas.set_clip_rect(rect);
    }

    /// Sets the blend mode used by the renderer to draw textures.
    fn blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode.into();
    }

    /// Returns a single event or None if the event pump is empty.
    fn poll_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event().map(|evt| evt.into())
    }

    /// Returns a single event or None if the event pump is empty.
    fn poll_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().map(|evt| evt.into()).collect()
    }

    /// Updates the canvas from the current back buffer.
    fn present(&mut self) {
        self.canvas.present();
    }

    /// Get the current window title.
    fn title(&self) -> &str {
        self.canvas.window().title()
    }

    /// Set the current window title.
    fn set_title<S>(&mut self, title: S) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.canvas.window_mut().set_title(title.as_ref())?;
        Ok(())
    }

    /// Width of the current canvas.
    fn width(&self) -> u32 {
        let (width, _) = self.canvas.output_size().unwrap_or((0, 0));
        width
    }

    /// Height of the current canvas.
    fn height(&self) -> u32 {
        let (_, height) = self.canvas.output_size().unwrap_or((0, 0));
        height
    }

    /// Scale the current canvas.
    fn scale(&mut self, x: f32, y: f32) -> Result<()> {
        self.canvas.set_scale(x, y)?;
        Ok(())
    }

    /// Returns whether the application is fullscreen or not.
    fn is_fullscreen(&self) -> bool {
        use FullscreenType::*;
        matches!(self.canvas.window().fullscreen_state(), True | Desktop)
    }

    /// Set the application to fullscreen or not.
    fn fullscreen(&mut self, val: bool) {
        let fullscreen_type = if val {
            FullscreenType::True
        } else {
            FullscreenType::Off
        };
        // Don't care if this fails or not.
        let _ = self.canvas.window_mut().set_fullscreen(fullscreen_type);
    }

    // /// Create a texture to render to.
    // fn create_texture(&mut self, _width: u32, _height: u32) -> Result<usize> {
    //     // TODO: Handle textures
    //     // Ok(self
    //     //     .texture_creator
    //     //     .create_texture_streaming(None, width, height)?)
    //     todo!("create_texture")
    // }

    /// Draw text to the current canvas.
    fn text<S>(
        &mut self,
        text: S,
        x: i32,
        y: i32,
        size: u32,
        fill: Option<Color>,
        _stroke: Option<Color>,
    ) -> Result<()>
    where
        S: AsRef<str>,
    {
        // TODO: Figure out how to store this
        // TODO: This path only works locally
        let font = self
            .ttf_context
            .load_font("static/emulogic.ttf", size as u16)?;
        if let Some(fill) = fill {
            let surface = font.render(text.as_ref()).blended(fill)?;
            let texture = self.texture_creator.create_texture_from_surface(&surface)?;
            let TextureQuery { width, height, .. } = texture.query();
            self.canvas
                .copy(&texture, None, Some(SdlRect::new(x, y, width, height)))?;
        }
        // TODO: Explore more
        // if let Some(fill) = fill {
        //     let x = i16::try_from(x)?;
        //     let y = i16::try_from(y)?;
        //     self.canvas.string(x, y, text, fill)?;
        // }
        Ok(())
    }

    /// Draw a pixel to the current canvas.
    fn point(&mut self, x: i32, y: i32, stroke: Option<Color>) -> Result<()> {
        if let Some(stroke) = stroke {
            let x = i16::try_from(x)?;
            let y = i16::try_from(y)?;
            self.canvas.pixel(x, y, stroke)?;
        }
        Ok(())
    }

    /// Draw an array of pixels to the canvas.
    fn points(&mut self, _pixels: &[u8], _pitch: usize) -> Result<()> {
        // TODO: Handle drawing pixels to textures
        // self.textures[0].update(None, pixels, pitch)?;
        // self.canvas.copy(&self.textures[0], None, None)?;
        todo!("points")
    }

    /// Draw a line to the current canvas.
    fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, stroke: Option<Color>) -> Result<()> {
        let x1 = i16::try_from(x1)?;
        let y1 = i16::try_from(y1)?;
        let x2 = i16::try_from(x2)?;
        let y2 = i16::try_from(y2)?;
        if let Some(stroke) = stroke {
            self.canvas.line(x1, y1, x2, y2, stroke)?;
        }
        Ok(())
    }

    /// Draw a triangle to the current canvas.
    fn triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let x1 = i16::try_from(x1)?;
        let y1 = i16::try_from(y1)?;
        let x2 = i16::try_from(x2)?;
        let y2 = i16::try_from(y2)?;
        let x3 = i16::try_from(x3)?;
        let y3 = i16::try_from(y3)?;
        if let Some(stroke) = stroke {
            self.canvas.trigon(x1, y1, x2, y2, x3, y3, stroke)?;
        }
        if let Some(fill) = fill {
            self.canvas.filled_trigon(x1, y1, x2, y2, x3, y3, fill)?;
        }
        Ok(())
    }

    /// Draw a rectangle to the current canvas.
    fn rect(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let x = i16::try_from(x)?;
        let y = i16::try_from(y)?;
        let w = i16::try_from(width)?;
        let h = i16::try_from(height)?;
        if let Some(stroke) = stroke {
            self.canvas.rectangle(x, y, x + w - 1, y + h - 1, stroke)?;
        }
        if let Some(fill) = fill {
            self.canvas.box_(x, y, x + w - 1, y + h - 1, fill)?;
        }
        Ok(())
    }

    /// Draw a ellipse to the current canvas.
    fn ellipse(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        fill: Option<Color>,
        stroke: Option<Color>,
    ) -> Result<()> {
        let x = i16::try_from(x)?;
        let y = i16::try_from(y)?;
        let w = i16::try_from(width)?;
        let h = i16::try_from(height)?;
        if let Some(stroke) = stroke {
            self.canvas.ellipse(x, y, w, h, stroke)?;
        }
        if let Some(fill) = fill {
            self.canvas.filled_ellipse(x, y, w, h, fill)?;
        }
        Ok(())
    }

    // TODO: Move texture creation into image object?
    /// Draw an image to the current canvas.
    fn image(&mut self, x: i32, y: i32, img: &Image) -> Result<()> {
        let mut texture = self.texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24,
            img.width(),
            img.height(),
        )?;
        texture.update(None, img.bytes(), 3 * img.width() as usize)?;
        texture.set_blend_mode(self.blend_mode);
        let dst = SdlRect::new(x, y, img.width(), img.height());
        self.canvas.copy(&texture, None, dst)?;
        Ok(())
    }

    /// Draw an image to the current canvas.
    fn image_resized(&mut self, x: i32, y: i32, w: u32, h: u32, img: &Image) -> Result<()> {
        let mut texture = self.texture_creator.create_texture_streaming(
            PixelFormatEnum::RGB24,
            img.width(),
            img.height(),
        )?;
        texture.update(None, img.bytes(), img.channels() * img.width() as usize)?;
        texture.set_blend_mode(self.blend_mode);
        let dst = SdlRect::new(x, y, w, h);
        self.canvas.copy(&texture, None, dst)?;
        Ok(())
    }

    /// Add audio samples to the audio buffer queue.
    fn enqueue_audio(&mut self, samples: &[f32]) {
        // Don't let queue overflow
        let sample_rate = self.audio_device.spec().freq as u32;
        while self.audio_device.size() > sample_rate {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        self.audio_device.queue(samples);
    }
}

impl std::fmt::Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add some fields
        write!(f, "SdlRenderer {{}}")
    }
}

// impl AudioCallback for Audio {
//     type Channel = f32;

//     fn callback(&mut self, out: &mut [f32]) {
//         for x in out.iter_mut() {
//             if let Some(sample) = self.samples.pop_front() {
//                 *x = sample;
//             }
//         }
//     }
// }

/*
 * Type Conversions
 */

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
                keycode, repeat, ..
            } => KeyDown {
                key: keycode.map(|k| k.into()),
                repeat,
            },
            SdlEvent::KeyUp {
                keycode, repeat, ..
            } => KeyUp {
                key: keycode.map(|k| k.into()),
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

impl From<SdlButton> for Button {
    fn from(button: SdlButton) -> Self {
        use Button::*;
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

impl ToColor for Color {
    fn as_rgba(&self) -> (u8, u8, u8, u8) {
        use Color::*;
        match self {
            Rgb(rgb) => rgb.channels(),
            Hsv(hsv) => hsv.to_rgb().channels(),
        }
    }
}

impl From<Color> for SdlColor {
    fn from(color: Color) -> Self {
        use Color::*;
        let rgb = match color {
            Rgb(rgb) => rgb,
            Hsv(hsv) => hsv.to_rgb(),
        };
        Self::RGBA(rgb.red(), rgb.green(), rgb.blue(), rgb.alpha())
    }
}

impl From<Rect> for SdlRect {
    fn from(rect: Rect) -> Self {
        Self::new(rect.x, rect.y, rect.w, rect.h)
    }
}

impl From<BlendMode> for SdlBlendMode {
    fn from(mode: BlendMode) -> Self {
        use BlendMode::*;
        match mode {
            None => SdlBlendMode::None,
            Blend => SdlBlendMode::Blend,
            Add => SdlBlendMode::Add,
            Mod => SdlBlendMode::Mod,
        }
    }
}

/*
 * Error Conversions
 */

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Other(Cow::from(err))
    }
}

impl From<InitError> for Error {
    fn from(err: InitError) -> Self {
        use InitError::*;
        match err {
            InitializationError(err) => Self::IoError(err),
            AlreadyInitializedError => Self::InitError,
        }
    }
}

impl From<FontError> for Error {
    fn from(err: FontError) -> Self {
        use FontError::*;
        match err {
            InvalidLatin1Text(e) => Self::InvalidText("invalid latin1 text", e),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<WindowBuildError> for Error {
    fn from(err: WindowBuildError) -> Self {
        use WindowBuildError::*;
        match err {
            HeightOverflows(h) => Self::Overflow(Cow::from("window height"), h),
            WidthOverflows(w) => Self::Overflow(Cow::from("window width"), w),
            InvalidTitle(e) => Self::InvalidText("invalid title", e),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<IntegerOrSdlError> for Error {
    fn from(err: IntegerOrSdlError) -> Self {
        use IntegerOrSdlError::*;
        match err {
            IntegerOverflows(s, v) => Self::Overflow(Cow::from(s), v),
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<TextureValueError> for Error {
    fn from(err: TextureValueError) -> Self {
        use TextureValueError::*;
        match err {
            HeightOverflows(h) => Self::Overflow(Cow::from("texture height"), h),
            WidthOverflows(w) => Self::Overflow(Cow::from("texture width"), w),
            WidthMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("width must be multiple of 2"))
            }
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<UpdateTextureError> for Error {
    fn from(err: UpdateTextureError) -> Self {
        use UpdateTextureError::*;
        match err {
            PitchOverflows(p) => Self::Overflow(Cow::from("pitch"), p as u32),
            PitchMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("pitch must be multiple of 2"))
            }
            XMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("x must be multiple of 2"))
            }
            YMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("y must be multiple of 2"))
            }
            WidthMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("width must be multiple of 2"))
            }
            HeightMustBeMultipleOfTwoForFormat(_, _) => {
                Self::Other(Cow::from("height must be multiple of 2"))
            }
            SdlError(s) => Self::Other(Cow::from(s)),
        }
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Self::InvalidText("Unknown nul error", err)
    }
}
