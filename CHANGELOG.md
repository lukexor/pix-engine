# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2021-08-31

### Added

#### Core

- `crate::prelude` for common imports.
- `Clone`, `Debug`, `Default` implementations for most structs.
- Several structs now implement `Serialize` and `Deserialize` with the `serde`
  feature enabled.
- Several new optional `AppState` event methods.
- `PixEngineBuilder` struct added with several settings for configuring the
  `PixEngine`.

#### State

- Several environment methods for interacting with the window and `PixEngine`
  state.
- Several `PixState` methods for controlling drawing and `PixEngine` loop.

#### Window

- `PixState::set_fullscreen()`.
- `PixState::set_vsync()`.
- New window cursor methods.
- `window::Result` and `window::Error` for window related failures.

#### Drawing

- `ColorMode` enum with `Rgb`, `Hsb`, and `Hsl` variants.
- `Color` struct with several methods for creating and converting between color
  modes.
- `Color` macros `rgb!()`, `hsb!()`, and `hsl!()`.
- Additional `Color` constants matching the [SVG 1.0 Color
  Keywords](https://www.w3.org/TR/SVG11/types.html#ColorKeywords).
- `color::Result` and `color::Error` types for conversion failures.
- `Draw` trait for types that can be drawn using `PixState`.
- Several new shape drawing methods on `PixState`.
- New Texture methods and `TextureId` type.
- `Light` and `LightSource` structs for doing basic 3D light rendering.

#### UI

- New Immediate-mode UI drawing methods for buttons.

#### Shapes

- Made shape structs generic.
- Conversion implementations.
- `Deref` and `DerefMut` into an array of values representing a shape.
- `Contains` and `Intersects` traits and implementations for defining collision
  detection.
- `Draw` implementations.
- `Rect` struct extended with several constructor and utility methods.
- Several new shape structs: `Circle`, `Ellipse`, `Line`, `Point`, `Quad`,
  `Sphere`, and `Triangle`.

#### Image

- New `Image` methods for converting and manipulating images.
- `Debug` and `Default` trait implementations.
- `image::Result` and `image::Error` for image failures.

#### Events

- `WindowEvent` struct.
- `KeyMod` struct for capturing key modifiers.

#### Wasm

- `build_wasm.sh` script for building Web-Assembly version.

#### Misc

- `math` module for noise and randomization utilities.
- `Number` trait for generic number handling.
- `Vector` type for doing 2D/3D vector math.
- Attribution for `Emulogic` font.
- Several new examples:
  - `colors`
  - `flocking`
  - `fluid_simulation`
  - `image`
  - `maze`
  - `raycasting`
  - `raytracing`.
- Extensive documentation additions.

### Changed

- `description`, `category` and `keywords` updated in `Cargo.toml`.
- Updated to [resolver 2](https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions)
  in `Cargo.toml`
- Updated `LICENSE.md` to `MIT/Apache-2.0`.
- Updated `README`, documentation and usage examples.
- Updated and refined examples.

### Breaking

#### Core

- Root imports have been removed in favor of `crate::prelude`.
- `PixEngineResult` renamed to `PixResult`.
- `PixEngineErr` renamed to `PixError` and `PixEngineErr::new()` removed.
- `PixEngine` `State` generic parameter removed in favor of passing application
  to `PixEngine::run()`.
- `PixEngine::new()` removed. Use `PixEngine::builder()` instead.
- `PixEngine::set_icon()` moved to `PixEngineBuilder::icon()` and changed to
  take `AsRef<Path>`.
- `PixEngine::fullscreen()` moved to `PixEngineBuilder::fullscreen()`.
- `PixEngine::vsync()` moved to `PixEngineBuilder::vsync()`.
- `PixEngine::set_audio_sample_rate()` moved to
  `PixEngineBuilder::audio_sample_rate()`.
- `PixEngine::run()` now takes a type that implements `AppState` and returns
  `PixResult` instead of `PixEngineResult`.

#### State

- `State` trait renamed to `AppState`.
- `StateData` struct renamed to `PixState`. Affects all methods from the `State`
  trait which was renamed to `PixState`.
- `AppState::on_start`, `AppState::on_update`, and `AppState::on_stop` changed
  to return `PixResult<()>`. Use `PixState::quit()` instead of returning `false`
  in order to
  terminate the application.
- `AppState::on_update` no longer takes `elapsed`. Use `PixState::delta_time()`
  instead.
- `StateData::enable_coord_wrapping()` removed.
- `StateData::wrap_coords()` removed in favor of `Point::wrap_*` and
  `Vector::wrap_*` methods.
- `StateData::create_texture()` changed to take `width`, `height` and optional `PixelFormat`.
- `StateData::copy_draw_target()` removed.
- `StateData::copy_texture()` renamed to `PixState::update_texture()` and
  parameters changed to `texture_id`, `Into<Option<Rect<i32>>>` and
  `AsRef<[u8]>`.
- `StateData::is_inside_circle()` removed. Use `Circle::contains_point()` instead.

#### Window

- `WindowId` type changed from `u32` to `usize`.
- `StateData::open_window()` renamed to `PixState::create_window()` which creates a `WindowBuilder`.
- `StateData::close_window()` renamed to `PixState::close_window()`.
- `StateData::main_window_id()` renamed to `PixState::window_id()`.
- `StateData::screen_width()` renamed to `PixState::width()`.
- `StateData::screen_height()` renamed to `PixState::height()`.
- `StateData::set_screen_size()` renamed to `PixState::set_dimensions()` and now
  takes a tuple `(u32, u32)`.
- `StateData::is_focused()` renamed to `PixState::focused()`.
- `StateData::fullscreen()` renamed to `PixState::fullscreen()` and changed to
  return the current fullscreen state. Use `PixState::set_fullscreen()` to change
  state.
- `StateData::vsync()` renamed to `PixState::vsync()` and changed to return the
  current vsync state. Use `PixState::set_vsync()` to change state.
- `StateData::create_window_texture()` removed.
- `StateData::copy_window_draw_target()` removed.
- `StateData::copy_window_texture()` removed.

#### Drawing

- `Pixel` renamed to `Color` with members made private.
- `AlphaMode` renamed to `BlendMode`. `Normal` renamed to `None`. `Mask`
  removed. `Add` and `Mod` added.
- `StateData::construct_font()` removed in favor of `sdl2::ttf`.
- `StateData::get_draw_target()` removed.
- `StateData::set_draw_target()` removed.
- `StateData::get_draw_target_dims()` removed.
- `StateData::clear_draw_target()` removed.
- `StateData::get_alpha_mode()` removed.
- `StateData::set_alpha_mode()` renamed to `PixState::blend_mode()`.
- `StateData::set_alpha_blend()` removed.
- `StateData::get_draw_color()` removed.
- `StateData::set_draw_color()` replaced with `PixState::background()`,
  `PixState::fill()` and `PixState::stroke()`.
- `StateData::reset_draw_color()` removed.
- `StateData::set_draw_scale()` removed.
- `StateData::fill()` renamed to `PixState::background()`.
- `StateData::clear()` renamed to `PixState::clear()` and updated to fill screen
  with current `background` color.
- `StateData::draw()` renamed to `PixState::point()` which now takes an
  `Into<Point>` to draw with the current `stroke` color.
- `StateData::draw_line()` renamed to `PixState::line()` which now takes an
  `Into<Line>` to draw with the current `stroke` color.
- `StateData::draw_line_i32()` removed.
- `StateData::draw_line_pattern()` removed.
- `StateData::draw_circle()` renamed to `PixState::circle()` which now takes an
  `Into<Circle>` to draw with the current `stroke` color.
- `StateData::draw_partial_circle()` renamed to `PixState::arc()` which now takes an
  `Into<Point>`, radius, start, and end to draw with the current `fill` and
  `stroke` colors.
- `StateData::fill_circle()` renamed to `PixState::circle()` which now takes an
  `Into<Circle>` to draw with the current `fill` color.
- `StateData::draw_elipse()` renamed to `PixState::ellipse()` which now takes an
  `Into<Ellipse>` to draw with the current `stroke` color.
- `StateData::fill_elipse()` renamed to `PixState::ellipse()` which now takes an
  `Into<Ellipse>` to draw with the current `fill` color.
- `StateData::draw_rect()` renamed to `PixState::rect()` which now takes an
  `Into<Rect>` to draw with the current `stroke` color.
- `StateData::fill_rect()` renamed to `PixState::rect()` which now takes an
  `Into<Rect>` to draw with the current `fill` color.
- `StateData::draw_triangle()` renamed to `PixState::triangle()` which now takes an
  `Into<Triangle>` to draw with the current `stroke` color.
- `StateData::fill_triangle()` renamed to `PixState::triangle()` which now takes an
  `Into<Triangle>` to draw with the current `fill` color.
- `StateData::draw_image()` renamed to `PixState::image()` which now takes an
  `Into<Point>`.
- `StateData::draw_partial_image()` removed.
- `StateData::draw_string()` renamed to `PixState::text()` which now takes a
  `Into<Point>` to draw with the current `fill` color.
- `StateData::draw_wireframe()` renamed to `PixState::wireframe()` which now takes a
  `&[Vector]` and an `Into<Vector>`in addition to angle and scale to draw a
  polygon with the current `stroke` and `fill` colors.
- `StateData::draw_transform()` removed.

#### Shapes

- `Rect` members made private. Use getter/setter methods to access `x`, `y`,
  `width`, and `height` instead.

#### Image

- `ImageRef` struct removed along with all related methods: `new_ref()`,
  `ref_from()`, `rgb_ref()`, and `rgba_ref()`.
- `rgb()` renamed to `with_rgb()`.
- `rgba()` renamed to `with_rgba()`.
- `from_bytes()` updated to take `PixelFormat` parameter and now returns
  `image::Result`.
- `put_pixel()` renamed to `set_pixel()`.
- `get_pixel()` and `put_pixel()` updated to return and accept `Color`.
- `get_pixel()` now returns a `color::Result`.
- `ColorType` renamed to `PixelFormat`.
- `color_type()` renamed to `format()` and returns `PixelFormat`.
- `bytes()` changed to return `&[u8]` instead of `Vec<u8>`.
- `bytes_mut()` changed to return `&mut [u8]` instead of `&mut Vec<u8>`.
- `from_file()` updated to take `AsRef<Path>` instead of `&str` and returns
  `image::Result`.
- `save_to_file()` renamed to `save()`, updated to take `AsRef<Path>` instead of
  `&str` and returns `image::Result`.

#### Events

- `Input` renamed to `KeyEvent`. `released` removed. `held` changed to
  `repeat`. `key` and `keymod` added.
- `Axis::Unknown` added and made `[non_exhaustive]`.
- `Button` renamed to `ControllerButton`. `Button::Unknown` added and made
  `[non_exhaustive]`.
- `Key` made `[non_exhaustive]`.
- `Mouse::X1` and `Mouse::X2` removed and made `[non_exhaustive]`.
- `PixEvent` renamed to `Event` and made `[non_exhaustive]`. Several new events
  added or changed.
- `State::get_key()` replaced with `PixState::keys()`,
  `PixState::key_pressed()`, and `PixState::key_down()`.
- `State::get_mouse()` replaced with `PixState::mouse_pos()`,
  `PixState::mouse_pressed()`, `PixState::mouse_down()` and
  `PixState::mouse_buttons()`.
- `State::get_mouse_x()` removed.
- `State::get_mouse_y()` removed.
- `State::get_mouse_wheel()` removed.
- `State::poll()` removed. Use the `AppState` methods to listen to events.
