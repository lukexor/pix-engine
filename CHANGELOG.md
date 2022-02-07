# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

### Added

- Added `PixState::ui_width` and `PoixState::ui_height` methods take into
  account remaining screen space with regards to current UI rendering position
  and frame padding.
- Added configurable `scroll_size` to the UI theme for the width of rendered
  scrollbars.
- Added `PixState::mouse_dbl_clicked` method indicating mouse was double clicked
  last frame.
- Added `PixState::dbl_clicked` method indicating mouse was double clicked on
  the previous UI widget drawn.

### Changed

- Improved element focus and blur.
- Fixed widgets to properly render the label with the current `fill` color.
- Increased the relative font size of `PixState::monospace`.
- Fixed `PixState::bullet` to be more indented.
- Removed indent of children under `PixState::collapsing_header`.
- Fixed UI elements being able to take focus if disabled.
- Blur focus by clicking outside on UI elements and by pressing escape/enter in
  input fields.

### Breaking

- Altered `PixState::tab_bar` to take a `selected` parameter to control the
  initial selected tab and altered the callback to take a generic type instead
  of a `usize`.
- `PixState::enqueue_audio` now returns a `PixResult` in the event the audio device fails, or the
  max queue size is reached.

## [0.5.4] - 2022-01-26

### Added

- Added `PixState::smooth` and `PixState::no_smooth` to toggle anti-alias
  drawing of shapes.
- Added `PixState::day`, `PixState::month`, `PixState::year`, `PixState::hour`,
  `PixState::minute`, and `PixState::second` methods.
- Added `[[T; 2]; 2]`, `[[T; 3]; 2]`, `[T; 4]`, and `[T; 6]` array conversions
  to `Line`.
- Added `[[T; 2]; 3]`, `[[T; 3]; 3]`, `[T; 6]`, and `[T; 9]` array conversions
  to `Tri`.
- Added `shapes` example.
- Added `PixState::bezier` and `PixState::bezier_detail` methods.
- Added `IntoIterator` for array-like types for `&T` and `&mut T`.
- Added audio callback and capture support with new types: `AudioSpecDesired`,
  `AudioSpec`, `AudioDevice`, a new trait: `AudioCallback` and new methods:
  `PixState::open_playback` and `PixState::open_capture`.
- Added `PixState::audio_driver` method to return the driver for the Audio Queue.
- Added `audio_callback` and `audio_capture_and_replay` examples.

### Changed

- Removed sleeping when audio queue got too full in favor of a maximum buffer
  size with a warning indicating `resume_audio` was not called.
- Updated `README` with better installation steps and a Table of Contents.
- Moved `rayon` to a `dev-dependency`.
- Added a `logging` example.
- Fixed `Color::TRANSPARENT` to have `0` alpha channel.
- Removed allowing `clippy::return_self_not_must_use` (Issue #9).
- Changed audio queue to not sleep if too full and instead warn (and eventually
  panic) if queue gets too full to avoid system contention.
- Optimized `Color` addition and subtraction operations.
- Renamed `audio` example to `audio_queue`.
- Removed `wasm` checks and dependencies until future Web-Assembly implementation starts in
  earnest.
- Made `PixState::present` public so that the current canvas can be updated in the middle of, or
  outside of `AppState::on_update`.
- Fixed various documentation errors.

### Breaking

- Made `WindowBuilder::new` crate-visible only. Prefer `PixState::window`.

## [0.5.3] - 2021-12-21

### Added

- Fixed mapping of `WindowEvent::Exposed`.
- Raw audio sample example.
- `AudioStatus` enum for representing the playback status of the audio device.
- `PixState::audio_status` and `PixState::audio_sample_rate` methods.
- `PixState::resume_audio` and `PixState::pause_audio` methods.
- `PixEngineBuilder::audio_channels` to choose the number of audio channels to
  play to. Defaults to `1` for mono.
- `SpacingBuilder` struct to construct custom theme spacing easier.
- `PixState::menu` method that renders a clickable menu item with hover state.

### Changed

- Engine loop sleeps remainder of target frame rate to reduce CPU usage.
- Default audio sample rate to 48,000 Hz.
- Fixed `ThemeBuilder` to default to "dark" theme.
- Changed radio and checkboxes to scale based on `font_size`.

### Breaking

- Disabled audio playback by default on startup. To queue and play audio you
  must first call `PixState::resume_audio`.

## [0.5.2] - 2021-12-13

### Changed

- Updated MSRV in README.

## [0.5.1] - 2021-12-13

### Added

- Basic gamepad controller support and a new event: `JoyHatMotion`.
- `PixEngineBuider::with_deadzone` which alters the default gamepad axis
  deadzone.
- `AppState::on_controller_pressed`, `AppState::on_controller_released`,
  `AppState::on_controller_axis_motion`, `AppState::on_controller_update`.
- More supported events: `AudioDeviceAdded`, `AudioDeviceRemoved`,
  `WindowEvent::Exposed`, `Key::Kp*` events for Keypad support.
- Warning logs for unsupported events.

### Changed

#### Core

- `PixEngineBuilder::icon` and `WindowBuilder::icon` now take an
  `Into<Icon>` parameter that can converted into either a `PathBuf` or an
  `Image` which allows loading an icon from a file, or a static or dynamic
  image.

#### UI

- Various UI padding now use frame padding instead of item padding.

### Breaking

- Changed `Unknown` event variants to `Unsupported` to better reflect that some
  events are known, but are not supported by this library.

## [0.5.0] - 2021-11-27

### Added

#### Core

- `log` facade added for logging support.
- Added methods to `PixEngineBuilder` to control cache sizes.
- Added `PixState::elapsed` method which returns the total elasped time since
  application start.
- A lot of documentation, examples, and README images.

#### UI

- Added `Theme` and `ThemeBuiilder` structs to customize UI theming for colors,
  fonts, sizes, styles and spacing. `PixEngineBuilder` updated with theme
  customizing methods. Default is a dark theme.
- Added several new UI widget rendering methods and a new `gui` example demoing
  their usage.
- `Cursor::no` method added.

#### Window

- `PixState` methods for getting and setting window dimensions changed to return
  a result instead of panicking and will return the dimensions for the current
  window target instead of only the primary window.
- `PixState::save_canvas` and `PixState::save_texture` methods.

#### Drawing

- `Color::from_hex_alpha` and `Color::as_hex_alpha` added that take/return RGBA
   values.
- `Color::blended` method added.
- `PixState::set_viewport` and `PixState::clear_viewport` methods added to
  control the rendering viewport.

#### Shapes

- `Rect::resized` and `Rect::resize_by` methods added.
- Added various `offset` methods to shapes.
- `Ellipse::diameter` method for circular ellipses.
- `Ellipse::bounding_rect` method.
- `Point::dist` method.
- `Serialize` and `Deserialize` added for shapes with const generics when the
  `serde` feature is enabled.

### Changed

#### Core

- Several types changed to `must_use`.
- Several optimizations and performance improvements regarding caching and
  memory management.

#### UI

- `PixEngineBuilder::with_font` updated to take anything that can be turned into
  a `Font` struct which includes a path as before, but can now also take static
  font data loaded from `include_bytes!` for example.
- `PixState::text` updated to return the bounding box of rendered text.
- Added `PixState::wrap_width` and `PixState::no_wrap` methods to control text
  wrap width.

#### Drawing

- Many `Color` methods made `const`.
- `Color::set_levels` method added.
- Changed shapes to use anti-aliasing where possible by default.
- Added `PixState::stroke_weight` method to draw thick lines.

#### Shapes

- Fixed radius handling in a circle `Ellipse`.
- `Line::new`, `Tri::new`, and `Quad::new` updated to support different types
  for each point parameter.
- `Line`, `Tri`, and `Quad` macros updated to have better type inference.

### Breaking

#### Core

- `core` module removed and all included modules moved up a level.
- `PixResult` changed to return `anyhow::Error`, which can include a backtrace
  on nightly. Many other types of errors returned now all return the same
  `PixResult` struct.
- `AppState` methods that handle events changed to return a `bool` indicating
  whether the event is to be consumed or not, and thus skipping any additional
  handling the engine may have for said event.
- `pix_engine::prelude::*` cleaned up by removing several type aliases which can
  be imported from `pix_engine::shape`.
- Removed `PixEngineBuilder::asset_dir` method in favor of including assets
  required by the library in the binary.
- `PixEngineBuilder::build` now returns a `PixResult` if any of the build
  settings are invalid.
- `math::constants::*` moved into `math`.

#### UI

- `PixEngineBuilder::with_font` changed to not take `size` as a parameter. Added
  an additional `PixEngineBuilder::with_font_size` method.
- `PixState::primary_window_id` removed.

#### Drawing

- `Color::new`, `Color::new_alpha`, `Color::rgb`, and `Color::rgba` changed to
  take `u8` RGB/A values. Affects `rgb!` and `color!` macros. RGBA setter
  methods also updated to take `u8`.
- `Color::levels` made non-const instead computing levels at run-time as needed.
- `Color::from_raw` renamed to `Color::from_levels` and removed `unsafe`.
- `Color::from_hex` and `Color::as_hex` changed to take/return RGB
  values with the top `u32` bits being `0x00`. `Color::from_hex_alpha` and
  `Color::as_hex_alpha` added that take/return RGBA values.
- `Color::rgb_channels` and `Color::rgba_channels` removed in favor of
  `Color::channels`.
- Color constants moved from the prelude to constants on
  `Color`. e.g. `Color::RED`.
- `PixState::polygon` and `PixState::wireframe` changed to take a type that can
  be converted into `IntoIterator<Item = Into<PointI2>>`.
- All shape drawing methods have more strict requirements that types can be
  converted into `i32`.
- All shape drawing methods with floating point representations have had
  `floor`, `ceil`, `round` and `trunc` methods added.

#### Textures

- `Texture` struct removed in favor of `TextureId`. All methods for getting or
  updating textures now take a `TextureId` instead.
- Removed `unsafe` from `PixState::delete_texture`. Now it will simply return a
  `PixResult` if the `TextureId` is invalid.

#### Shapes

- All shapes had their `values` method changed to `as_array` and `set_values`
  method removed in favor of `as_bytes_mut`.
- All shapes now have `as_bytes` and `as_bytes_mut` methods.
- Removed `Button` struct and changed `PixState::button` API to just return a
  `bool` if the button was clicked or not instead of having a `clicked`
  method. Use `PixState::hovered` method to check if the previously rendered
  item was hovered.

## [0.4.2] - 2021-09-02

### Added

#### Core

- `crate::prelude` and `crate::prelude_3d` for common imports.
- `Copy`, `Clone`, `Debug`, `Default`, `PartialEq`, `Eq`, and `Hash`
  implementations for many/most structs where appropriate.
- `Serialize` and `Deserialize` implemented for most structs with with the `serde`
  feature enabled.
- New optional `AppState` methods:
  - `on_key_pressed()`: Called on key press and key repeat.
  - `on_key_released()`: Called on key release.
  - `on_key_typed()`: Called on character typed (ignores special keys like Ctrl and Backspace).
  - `on_mouse_dragged()`: Called on mouse motion while button is being heled.
  - `on_mouse_pressed()`: Called on mouse button press.
  - `on_mouse_released()`: Called on mouse button release.
  - `on_mouse_clicked()`: Called on mouse button released followed by a press.
  - `on_mouse_dbl_clicked()`: Called on mouse button double click.
  - `on_mouse_motion()`: Called on mouse motion.
  - `on_mouse_wheel()`: Called on mouse wheel scroll.
  - `on_window_event()`: Called on a window event (e.g. closed, resized, moved).
  - `on_event()`: Called for every user or system event as a catch-all.
- `PixEngineBuilder` struct added with several settings for configuring the
  `PixEngine` initialization.

#### State

- `PixState` methods for interacting with the window and `PixEngine` state:
  - `delta_time()`: Time elapsed since last frame.
  - `frame_count()` Total number of frame since application start.
  - `frame_rate()`: Average frames per second rendered.
  - `target_frame_rate()`: Target frame rate.
  - `set_frame_rate()`: Set target frame rate.
  - `clear_frame_rate()`: Clear target frame rate.
  - `quit()`: Quit the application.
  - `abort_quit()`: Abort quitting the application (useful in
  `AppState::on_stop()` as a confirmation).
- `PixState` methods for controlling drawing and the `PixEngine` render loop:
  - `background()`: Set the color for clearing the screen and clear to it immediately.
  - `fill()`: Set the fill color for drawing operations.
  - `no_fill()`: Clear the fill color to transparent.
  - `stroke()`: Set the stroke color for drawing operations.
  - `no_stroke()`: Clear the stroke color to transparent.
  - `clip()`: Set a clipping rectangle for drawing to the canvas.
  - `no_clip()`: Clear the clipping rectangle.
  - `running()`: Whether the engine loop is running and calling
    `AppState::on_update()` or not.
  - `run()`: Start the engine loop if it's not already running.
  - `no_run()`: Disable the engine loop if it's currently running.
  - `redraw()`: Run render loop one time if it's not currently running.
  - `runtimes()`: Run render loop N times if it's not currently running.
  - `show_frame_rate()`: Set whether to show the average frame rate in the title
    bar or not.
  - `scale()`: Set the X, Y rendering scale for the canvas. Note this is
    different than `PixEngineBuilder::scale` which scales the window dimensions.
  - `font_size()`: Set the font size.
  - `size_of()`: Get the dimensions of a given string with the current `font_size`.
  - `font_style()`: Set the font style (e.g. NORMAL, BOLD, ITALIC, UNDERLINE,
    STRIKETHROUGH).
  - `font_family()`: Set the font family. (e.g. "courier_new.ttf").
  - `rect_mode()`: Set how (x, y) coordinates are treated for drawing squares
    and rectangles.
  - `ellipse_mode()`: Set how (x, y) coordinates are treated
    for drawing circles and ellipses.
  - `image_mode()`: Set how (x, y) coordinates are treated for drawing images.
  - `angle_mode()`: Set whether angles are interpreted as `Radians` or `Degrees`.
  - `blend_mode()`: Set drawing blend mode for images and textures.
  - `push()`: Save the current drawing settings to a stack.
  - `pop()`: Restore previous drawing settings from the stack.

#### Window

- `WindowBuilder` struct for opening windows:
  - `new()`: Create a new `WindowBuilder` instance.
  - `with_dimensions()`: Set window (width, height).
  - `with_title()`: Set window title.
  - `position()`: Set window (x, y) position.
  - `position_centered()`: Center window in the display.
  - `fullscreen()`: Make the window fullscreen.
  - `resizable()`: Allow window resizing.
  - `borderless()`: Disable window border.
  - `scale()`: Set window dimension scale.
  - `icon()`: Set window icon.
  - `build()`: Build window from `WindowBuilder` and open it.
- `PixState` window methods:
  - `focused()`: Wether the current window target has focus.
  - `primary_window_id()`: The primary window ID.
  - `window_id()`: The current window target ID.
  - `window()`: Create a new `WindowBuilder`.
  - `close_window()`: Close a given window.
  - `dimensions()`: The current window target (width, height) dimensions.
  - `set_dimensions()`: Set the current window target (width, height) dimensions.
  - `width()`: The current window target width.
  - `set_width()`: Set the current window target width.
  - `height()`: The current window target height.
  - `set_height()`: Set the current window target height.
  - `display_dimensionsc()`: The current display (width, height) dimensions.
  - `display_width()`: The current display width.
  - `display_height()`: The current display height.
  - `show_window()`: Show the current window target if hidden.
  - `hide_window()`: Hide the current window target if shown.
  - `fullscreen()`: Whether the application is currently in fullscreen mode.
  - `set_fullscreen()`: Set fullscreen to true or false.
  - `vsync()`: Whether vertical sync is enabled.
  - `set_vsync()`: Set vertical sync to true or false.
  - `cursor()`: Set the mouse cursor icon to either a system icon or custom image.
  - `no_cursor()`: Clear mouse cursor back to default.
  - `with_window()`: Target a window for drawing operations with a closure.
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
- New `Texture` struct and methods for hardware-accelerated rendering.
- `Light` and `LightSource` structs for doing basic 3D light rendering.

#### UI

- New Immediate-mode UI drawing methods for buttons. More to come in future
  versions.

#### Shapes

- Made shape structs generic over their type and number of dimensions using new
  const generics.
- Conversion implementations to convert shapes between units for better
  ergonomics.
- `Deref` and `DerefMut` into an array of values representing a shape.
- `IntoIterator` for structs where applicable.
- `Contains` and `Intersects` traits and implementations for defining collision
  detection.
- `Draw` implementations.
- `Rect` struct extended with several constructor and utility methods.
- Several new shape structs: `Ellipse`, `Line`, `Point`, `Quad`, `Sphere`, and
  `Tri` with convenience macros.

#### Image

- New `Image` methods for converting and manipulating images.
- `image::Result` and `image::Error` for image failures.

#### Events

- `WindowEvent` struct.
- `KeyMod` struct for detecting key modifiers on key press and release events.

#### Misc

- `math` module for noise and randomization utilities.
- `Num` trait for generic number handling.
- `Vector` type for doing N-dimensional vector math.
- Attribution for `Emulogic` font.
- New `katakana` provided font along with default unicode fonts for fallbacks.
- Several new examples:
  - `2d_raycasting`
  - `3d_raytracing`
  - `colors`
  - `flocking`
  - `fluid_simulation`
  - `image`
  - `matrix`
  - `maze`
  - `textures`
- Extensive documentation additions and README improvements.

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
  in order to terminate the application.
- `AppState::on_update` no longer takes `elapsed`. Use `PixState::delta_time()`
  instead.
- `StateData::enable_coord_wrapping()` removed. Use `Point::wrap()` and
  `Vector::wrap()` methods instead.
- `StateData::wrap_coords()` removed in favor of `Point::wrap()` and
  `Vector::wrap()` methods.
- `StateData::create_texture()` changed to take `width`, `height` and optional `PixelFormat`.
- `StateData::copy_draw_target()` removed.
- `StateData::copy_texture()` renamed to `PixState::update_texture()` and
  parameters changed to take `&mut Texture`, `Into<Option<Rect<i32>>>`,
  `AsRef<[u8]>`, and `pitch`.
- `StateData::is_inside_circle()` removed. Use `Ellipse::contains_point()` instead.

#### Window

- `WindowId` type changed from `u32` to `usize`.
- `StateData::open_window()` renamed to `PixState::create_window()` which creates a `WindowBuilder`.
- `StateData::close_window()` renamed to `PixState::close_window()`.
- `StateData::main_window_id()` renamed to `PixState::primary_window_id()`.
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
- `StateData::set_draw_target()` removed. Use `PixState::with_texture()` instead.
- `StateData::get_draw_target_dims()` removed. Use `Texture::dimensions()` instead.
- `StateData::clear_draw_target()` removed. Draw target is only set within
  `PixState::with_texture()` callback.
- `StateData::get_alpha_mode()` removed.
- `StateData::set_alpha_mode()` renamed to `PixState::blend_mode()`.
- `StateData::set_alpha_blend()` removed.
- `StateData::get_draw_color()` removed.
- `StateData::set_draw_color()` replaced with `PixState::background()`,
  `PixState::fill()` and `PixState::stroke()`.
- `StateData::reset_draw_color()` removed.
- `StateData::set_draw_scale()` removed. Use `PixEngineBuilder::scale()` or
  `PixState::scale()` methods instead.
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
  `Into<Ellipse>` to draw with the current `stroke` color.
- `StateData::draw_partial_circle()` renamed to `PixState::arc()` which now takes an
  `Into<Point>`, radius, start, and end to draw with the current `fill` and
  `stroke` colors.
- `StateData::fill_circle()` renamed to `PixState::circle()` which now takes an
  `Into<Ellipse>` to draw with the current `fill` color.
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
- `ColorType` renamed to `PixelFormat`.
- `color_type()` renamed to `format()` and returns `PixelFormat`.
- `bytes()` renamed to `as_bytes()`. `bytes()` now returns an `Iterator` of `u8`
  instead of `Vec<u8>`.
- `bytes_mut()` renamed to `as_mut_bytes()` and changed to return `&mut [u8]` instead of `&mut Vec<u8>`.
- `from_file()` updated to take `AsRef<Path>` instead of `&str` and returns
  `image::Result`.
- `save_to_file()` renamed to `save()`, updated to take `AsRef<Path>` instead of
  `&str` and returns `image::Result`.

#### Events

- `Input` renamed to `KeyEvent`. `released` removed. `held` changed to
  `repeat`. `key` and `keymod` added.
- `Axis::Unknown` added and `Axis` made `[non_exhaustive]`.
- `Button` renamed to `ControllerButton`. `ControllerButton::Unknown` added and
  `ControllerButton` made `[non_exhaustive]`.
- `Key` made `[non_exhaustive]`.
- `Mouse::X1` and `Mouse::X2` removed and `Mouse` made `[non_exhaustive]`.
- `PixEvent` renamed to `Event` and made `[non_exhaustive]`. Several new events
  added or changed.
- `State::get_key()` removed in favor of `PixState::keys()`,
  `PixState::key_pressed()`, `PixState::key_down()` and `AppState::on_key_*`
  methods.
- `State::get_mouse()` removed in favor of `PixState::mouse_pos()`,
  `PixState::mouse_pressed()`, `PixState::mouse_down()`,
  `PixState::mouse_buttons()` and `AppState::on_mouse_*` methods.
- `State::get_mouse_x()` removed.
- `State::get_mouse_y()` removed.
- `State::get_mouse_wheel()` removed in favor of `AppState::on_mouse_wheel()`.
- `State::poll()` removed. Use the `AppState::on_*` methods to respond to events.
