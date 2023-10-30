<!-- markdownlint-disable-file no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2023-10-30

### ⛰️  Features

- Add getters for window position -
  ([d488091](https://github.com/lukexor/pix-engine/commit/d48809116b1d20690cf6ae7e2d2599949a92af09))
  and
  ([bc29778](https://github.com/lukexor/pix-engine/commit/bc29778528ad26532c2168b7686182cc8a66d010))

### 🐛 Bug Fixes

- Problem around chrono, time, and an CVE - ([fb4a22f](https://github.com/lukexor/pix-engine/commit/fb4a22fab9a2c8b4d6d9b3707fce9edddbaf9386))
- Fixed build badge - ([0a01708](https://github.com/lukexor/pix-engine/commit/0a0170866ba4fe16b1d01a88a46ce0970a2a53b2))
- Fix setting cursor without support -
  ([c4baf4d](https://github.com/lukexor/pix-engine/commit/c4baf4dd2974732fd8e704e33691a49b7b476312))
  and
  ([fc3b038](https://github.com/lukexor/pix-engine/commit/fc3b0380f56a5e93d18375e7e218164a4b03a545))

### 📚 Documentation

- Updated MSRV - ([032f7b1](https://github.com/lukexor/pix-engine/commit/032f7b10d4356859b456544d3ba88d85c2356ad1))

## [0.7.0] - 2023-01-20

### Added

### Changed

- Added `Features` section to `README`.
- Avoid computing `target_delta_time` each frame
- Increased MSRV to `1.61.0`

### Fixed

### Breaking

- Changed `Unsupported` `Event` variants to `Unhandled` to better reflect
  intention.
- Renamed `AppState` to `PixEngine` and `PixEngine` to `Engine`.
- Renamed `engine::Builder` to `engine::EngineBuilder`.
- Updated `PixEngine::on_event` to return a `bool`, which consumes the event.
- Removed `with_` prefix from builder methods.
- `EngineBuilder::texture_cache` and `EngineBuilder::text_cache` now take a
  `NonZeroUsize` instead of `usize`.
- Changed `PixState::with_texture` to `PixState::set_texture_target` and
  `PixState::clear_texture_target`.
- Changed `PixState::with_window` to `PixState::set_window_target` and
  `PixState::clear_window_target`.
- Changed `PixState::focused` to return whether the current window target is
  focused instead of any window.

## [0.6.0] - 2022-06-20

### Added

- Added `PixState::ui_width` and `PoixState::ui_height` methods which take into
  account remaining screen space with regards to current UI cursor position and
  frame padding.
- Added configurable `scroll_size` to the UI theme for the width of rendered
  scrollbars.
- Added `PixState::mouse_dbl_clicked` method indicating mouse was double clicked
  last frame.
- Added `PixState::dbl_clicked` method indicating mouse was double clicked on
  the previous UI widget drawn.
- Added `PixState::set_column_offset` and `PixState::reset_column_offset` to
  allow controlling the `x-offset` cursor position when rendering UI elements.
- Added `ThemeBuilder` to `pix_engine::prelude`.
- Added `PixState::focused_window` that takes a `WindowId` to check for focus.
- Added `PixState::audio_size` to query the current size of the audio queue
  device.
- Added `Triangle` `contains` `Point` implementation.
- Added arrow keyboard navigation to `PixState::select_box` while focused.

### Changed

- Improved element focus and blur.
- Increased the relative font size of `PixState::monospace`.
- Removed indent of children under `PixState::collapsing_header`.
- Fixed UI elements being able to take focus if disabled.
- Blur focus by clicking outside on UI elements and by pressing escape/enter in
  input fields.
- Default types and dimensions for `Point`, `Vector`, `Line`, `Triangle`,
  `Quad`, `Light` and `LightSource` are now defined.
- Changed how `PixState::on_update` handles frame rates and no longer sleeps
  when `vsync` is enabled.
- Changed vertical scroll direction to be natural.
- Set default audio buffer size to 4096.
- Updated audio buffer doucmentation.
- Changed default audio settings to use device defaults.
- Swapped `lazy_static` for `once_cell`.

### Fixed

- Fixed widgets to properly render the label with the current `fill` color.
- Fixed `PixState::bullet` to be more indented.
- Fixed `PixState::tab` size
- Fixed `PixState::select_list` padding
- Fixed `PixState::mod_down` to correctly return `true` when multiple modifiers
  are pressed.
- Fixed off-by-one error in `Ellipse::bounding_rect`.
- Fixed `target_frame_rate` epsilon.
- Fixed update rate limiting when `vsync` is disabled and no `target_frame_rate`
- set.
- Fixed `PixState::select_box` expanding on focus and unexpanding when focus
  is lost.
- Fixed `PixState::font_size` affecting `Theme` font size.

### Breaking

- Changed `PixState::tab_bar` to take a `selected` parameter to control the
  initial selected tab and changed the callback to take a generic type instead
  of a `usize`.
- `PixState::enqueue_audio` now returns a `PixResult` in the event the audio
  device fails, or the max queue size is reached.
- Removed clearing the screen by default in `AppState::on_update`, leaving it to
  the application to choose when to clear the screen.
- Changed `PixState::focused` to return `true` whether any active windows have
  focus.
- Renamed `PixState::keymods` to `PixState::keymod` which now returns a single
  `&KeyMod` value instead of a `HashSet<KeyMod>` since `KeyMod` is already a
  set of bitflags.
- Changed `PixState::delta_time` and `PixState::elapsed` to return a `Duration`
  instead of milliseconds.
- Changed `PixState::avg_frame_rate` to return an `f32` instead of `usize`.
- Changed `PixState::stroke_weight` to accept a `u16` instead of a `u8`.
- Removed generic type aliases for all types that now have reasonable defaults.
- Removed `PixState::no_*` (e.g. `no_stroke`, `no_fill`, etc) methods in favor
  of the main setter method accepting `false` or `None`.
- Added `PartialEq` to `Num` trait.
- Modified `Contains` and `Intersects` traits to be more generic and have a
  single `contains` or `intersects` method to allow for future implementations
  of other shapes.
- Added `PixState::audio_queued_size` and changed `PixState::audio_size` to
  return the buffer capacity instead of the queued size.
- Added support for multiple concrete channel types for `AudioCallback` via
  an associated trait type.
- Removed `vcpkg` feature support due to flaky error rates. `Windows` builds now
  can utilize a build script with static linking. `macOS` and `Linux` can
  continue using `homebrew` or their distros package manager.
- Changed `PixEngineBuilder::scale` to only set rendering scale and to not
  affect window size, to mirror `PixState::scale`. Removed
  `WindowBuilder::scale`.
- Renamed all primitive `as_bytes` and `as_array` methods to `points`
  and `coords`.

## [0.5.4] - 2022-01-26

### Added

- Added `PixState::smooth` and `PixState::no_smooth` to toggle anti-alias
  drawing of shapes.
- Added `PixState::day`, `PixState::month`, `PixState::year`, `PixState::hour`,
  `PixState::minute`, and `PixState::second` met
