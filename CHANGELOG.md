<!-- markdownlint-disable-file no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0](https://github.com/lukexor/pix-engine/compare/0.8.0..0.9.0) - 2026-11-14

### ⛰️  Features

- *(audio)* Replace SDL audio with cpal - ([9496d98](https://github.com/lukexor/pix-engine/commit/9496d98aab39ca9d71d8fd71a7e9f9aea615479b))
- *(bench)* Add a frame-timing harness and record the baseline - ([ff2f050](https://github.com/lukexor/pix-engine/commit/ff2f050c639bc967f7c2d1927bb945c25f7cc76b))
- *(controller)* Replace SDL game controllers with gilrs - ([cc4d267](https://github.com/lukexor/pix-engine/commit/cc4d2675b1b3ac7b9d95ea1875ed1c20ef61adf9))
- *(renderer)* [**breaking**] Replace SDL rendering with wgpu and winit - ([b23fffd](https://github.com/lukexor/pix-engine/commit/b23fffde24ad3388194d26ba1d01dfc985baf255))
- *(text)* [**breaking**] Draw bold and italic from real faces - ([8008e75](https://github.com/lukexor/pix-engine/commit/8008e75f448a70e52d99509a78d3b3a5ecfb8a52))
- *(window)* [**breaking**] Return nothing from vsync and toggle_vsync - ([1adb7fe](https://github.com/lukexor/pix-engine/commit/1adb7fe499d559fe44a181471114ad08fc95ffd8))

- Add scancode support WIP 4 - Complete list of scancodes from SDL2 - ([0d7fe30](https://github.com/lukexor/pix-engine/commit/0d7fe305a629308f45dd7ef5993c4ebe36d0d65f))
- Add scancode support WIP 3 - ([4d912a0](https://github.com/lukexor/pix-engine/commit/4d912a095d600dc2330f0ab9106d2779cd8bf3f6))
- Add scancode support WIP 2 - ([bc208ba](https://github.com/lukexor/pix-engine/commit/bc208ba969c5e179c923c32fcc6b2809d17d7720))
- Add scancode support WIP 1 - ([c53cb2c](https://github.com/lukexor/pix-engine/commit/c53cb2c9e304d494321106d48072a1170299ac44))

### 🐛 Bug Fixes

- *(color)* [**breaking**] Do color arithmetic on Rgb channels in every mode - ([53f103c](https://github.com/lukexor/pix-engine/commit/53f103c413e37fc5cf60267d44d9d76af36c51df))
- *(examples)* Keep fluid drags inside the simulated rows - ([068977b](https://github.com/lukexor/pix-engine/commit/068977b7b8d1d382a9c14dbfe5b49ab0ad98b65d))
- *(examples)* Show the matrix rain sooner - ([be5cac1](https://github.com/lukexor/pix-engine/commit/be5cac1e4cf9dd44a8953977b6eb4f98d3abaf1e))
- *(renderer)* Stretch text when the drawing scale differs per axis - ([014a473](https://github.com/lukexor/pix-engine/commit/014a47370ba629440ce160926ebc1127e264eb90))
- *(renderer)* Satisfy clippy on Windows when toggling fullscreen - ([7bea0fc](https://github.com/lukexor/pix-engine/commit/7bea0fc57e532c4d0447e23685f610cce3fb7c49))
- *(renderer)* Apply the drawing scale to new windows, clips and the mouse - ([fd8644c](https://github.com/lukexor/pix-engine/commit/fd8644c8d02e7c2f9052e476591c9d778935d5b9))
- *(renderer)* Rotate images about a center relative to dst - ([36a496d](https://github.com/lukexor/pix-engine/commit/36a496dc49439e42c4a322dfd07298aacb786610))
- *(renderer)* Honor the text rotation center and flip - ([f54f4e0](https://github.com/lukexor/pix-engine/commit/f54f4e01862c7716dea882fbe3c2f37be858b505))
- *(renderer)* Size windows in logical points on scaled displays - ([7dd6da1](https://github.com/lukexor/pix-engine/commit/7dd6da102dd60f90e850a316d326763fe8cb4930))
- *(renderer)* Reuse textures for an image redrawn within a frame - ([410004e](https://github.com/lukexor/pix-engine/commit/410004e0d9d49275ac5fcdcf84ca12e7fe773ef7))
- *(renderer)* Keep presenting after a surface fails to reconfigure - ([39b30da](https://github.com/lukexor/pix-engine/commit/39b30dab192993779f72ae3fa738a673154f24d6))
- *(renderer)* Draw the default face until a new font is baked - ([a0b1b21](https://github.com/lukexor/pix-engine/commit/a0b1b21d33a1160ab8e05e6183015afcf29daff9))
- *(renderer)* Draw bold text by drawing the glyphs twice - ([02a6f63](https://github.com/lukexor/pix-engine/commit/02a6f632159f299de182bdf6cb9d0cd5c2b632b9))
- *(renderer)* Fingerprint cached images so two cannot share a texture - ([2c04886](https://github.com/lukexor/pix-engine/commit/2c04886a8e892f5b7082788b0b77347ac5e703f7))
- *(window)* Size the canvas to the window the compositor grants - ([5c96c2a](https://github.com/lukexor/pix-engine/commit/5c96c2ad4110702ee50f45d5fef937a72c95c446))
- *(window)* Make toggle_vsync flip the setting - ([d811d2a](https://github.com/lukexor/pix-engine/commit/d811d2aa98341abe69d2916d974fc74333f538a8))

- Import shape types by name instead of globbing the prelude - ([200415f](https://github.com/lukexor/pix-engine/commit/200415ffb16e695b549372b46fc8a9e51026457d))
- Correct caching, counter and lifecycle bugs - ([955c386](https://github.com/lukexor/pix-engine/commit/955c38648a658b71ec4d3291f2fd2debe3e928aa))
- Fixed color byte order pending updated rust-sdl2 release - ([8827bac](https://github.com/lukexor/pix-engine/commit/8827bacef75741a40bae1e7db16226b4fd1a3f87))
- Fixed unused warning - ([e88f9da](https://github.com/lukexor/pix-engine/commit/e88f9da3cb09cbedce999a440cfa5f80648ee29a))
- Fix removing joypad devices - ([539eb18](https://github.com/lukexor/pix-engine/commit/539eb18d93437c86ab3ec13ca052335277d00902))

### 🚜 Refactor


- Replace once_cell with std LazyLock - ([5497af7](https://github.com/lukexor/pix-engine/commit/5497af76d1998ea86acf9ba68e951b6e022a1112))

### 📚 Documentation

- *(bench)* Record the wgpu, color and hasher measurements - ([e0afec9](https://github.com/lukexor/pix-engine/commit/e0afec98ac4721a5d966fe050482cdfd840ce01e))
- *(text)* Note that a font's name identifies it - ([7e7139a](https://github.com/lukexor/pix-engine/commit/7e7139a421af2eb3851f135be1bd2f81434ffdb3))

- Explain the unpaced p99 spikes and drop a stale README version - ([dc05f8a](https://github.com/lukexor/pix-engine/commit/dc05f8ab8f046ad2e29118e8c06b0581e736fb62))
- Drop the removed backtrace feature from the README and CLAUDE.md - ([2ab3906](https://github.com/lukexor/pix-engine/commit/2ab39066c136b156fb7cb1e26eb3275f9011944f))
- Add a migration guide from 0.8 - ([2c7a875](https://github.com/lukexor/pix-engine/commit/2c7a8754d40b2cd72a740cb87fcfa96c79ed8d18))
- Correct the rustdoc and wasm notes in CLAUDE.md - ([e21e6a9](https://github.com/lukexor/pix-engine/commit/e21e6a907502bc2a49f3ae0a5987014dafa71705))
- Resolve the broken intra-doc links - ([10de1d6](https://github.com/lukexor/pix-engine/commit/10de1d6c93bac03975785ba6c64acb950a2e991c))
- Add CLAUDE.md for Claude Code - ([7de235c](https://github.com/lukexor/pix-engine/commit/7de235cf4a7b004b4063d65a45f54f0b68453ed2))
- Fixed docs - ([284b8dc](https://github.com/lukexor/pix-engine/commit/284b8dcdd27e5c25b778ee39f46ca149f1b60670))

### ⚡ Performance

- *(color)* Scale Rgb colors without the level round trip - ([bb41a14](https://github.com/lukexor/pix-engine/commit/bb41a1429b3c817c203836eb626659d3f9d2164e))
- *(renderer)* Hash the renderer maps with ahash - ([da6c543](https://github.com/lukexor/pix-engine/commit/da6c54397dbaaf64a9e272bbaf70fd0f775e95df))


### 🎨 Styling


- Fixed lints - ([839145c](https://github.com/lukexor/pix-engine/commit/839145ca6253a2950782da6d30c52445bd233a1b))
- Fix lints - ([e361694](https://github.com/lukexor/pix-engine/commit/e36169475bdb41926c4e5a8b2d1ed73da4424cd3))

### 🧪 Testing


- Updated tests - ([ebe6f47](https://github.com/lukexor/pix-engine/commit/ebe6f471c43395ad7bc5f32b13860f7ce0238b6f))

### ⚙️ Miscellaneous Tasks

- *(ci)* Fixed changelog - ([ea653e5](https://github.com/lukexor/pix-engine/commit/ea653e5c2bd1adc25c7d30d4ad226d3579c0b42b))

- Remove the SDL and wasm-pack scripts - ([701d61d](https://github.com/lukexor/pix-engine/commit/701d61d014fbdf299c5e63c9227fd65f28117cff))
- Drop dead modules and raise the MSRV - ([7c738fb](https://github.com/lukexor/pix-engine/commit/7c738fba7eac5ddaa37e0c4a4070694e64220f4a))
- Fix homebrew paths - ([4baff7c](https://github.com/lukexor/pix-engine/commit/4baff7c89d9fc005b931bbe6aa7bd170ec0cd002))
- Update actions - ([44520b2](https://github.com/lukexor/pix-engine/commit/44520b2e2b3b963df848c4d99718df88aa465a16))
- Reduce dependabot - ([b901fc6](https://github.com/lukexor/pix-engine/commit/b901fc66c16db3034be6428b2c113e9c1f01afe6))
- Updated deps - ([4a9c949](https://github.com/lukexor/pix-engine/commit/4a9c949ebb64cb8c44ee78f2a35ff146b734d1ae))
- Updated lints and set msrv - ([45fb86a](https://github.com/lukexor/pix-engine/commit/45fb86ad06daa7f79ba9a28c4896147f03455c70))
- Updated deps - ([12e684e](https://github.com/lukexor/pix-engine/commit/12e684ec06e161416f6bf47b015102a4aba27f56))
- Updated deps - ([780baf7](https://github.com/lukexor/pix-engine/commit/780baf71f2754f7ef9bf7718195150a61ea06434))
- Update dependencies - ([1128b2d](https://github.com/lukexor/pix-engine/commit/1128b2dc132cf064ce22a969927662b9f49222d8))
- Updated deps - ([d3e647e](https://github.com/lukexor/pix-engine/commit/d3e647e4167a884b385635837fbee3841ca11357))
- Updated deps - ([f9cd068](https://github.com/lukexor/pix-engine/commit/f9cd06868007d8c64c82363aae9cae7990f87874))
- Updated deps - ([2a6515e](https://github.com/lukexor/pix-engine/commit/2a6515e91166bec6e36987dec28ba86080697ce8))
- Updated deps - ([a6c49ba](https://github.com/lukexor/pix-engine/commit/a6c49ba538ff2a9caac82154368341e4dd7ca802))
- Updated packages - ([dba3424](https://github.com/lukexor/pix-engine/commit/dba34240dabf667c15970e2cb04dd7bc840ec885))
- Updated cargo deps - ([bccdb27](https://github.com/lukexor/pix-engine/commit/bccdb27720d2de9765c271709a2a7e0975fd6cd4))


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
