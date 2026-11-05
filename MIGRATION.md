# Migrating from 0.8

This release replaces SDL2 with pure Rust crates. `wgpu` draws, `winit`
provides windows and events, `cpal` plays audio and `gilrs` reads gamepads.
SDL2 no longer needs to be installed to build, run or develop an application.

The change brings:

- Pure Rust dependencies, with no C libraries to install or link.
- Faster primitive and texture drawing than SDL2_gfx.
- A path to the browser, which SDL2 ruled out. WebAssembly is not supported in
  this release (see [Not yet supported](#not-yet-supported)).

Most applications need three changes: new system packages, a newer Rust and,
if they use audio, the renamed audio types. The drawing and GUI APIs are
unchanged. Shapes rasterize differently, as covered under
[Silent changes](#silent-changes).

## Build requirements

SDL2 is no longer a dependency, so its install steps, its bundled MSVC DLLs and
the `lib/` directory are gone. `wgpu` picks a graphics backend at runtime and
needs nothing installed.

Linux builds need ALSA and udev headers, for audio and gamepads:

```sh
sudo apt-get install libasound2-dev libudev-dev   # Debian and Ubuntu
sudo pacman -S alsa-lib systemd-libs              # Arch
```

macOS and Windows need nothing. See the README for the full per-OS list.

## Minimum Supported Rust Version

The MSRV is now **1.92.0**, up from 1.67. `egui` 0.35 sets the floor.

## Compile errors

### The `opengl` and `backtrace` features are removed

`wgpu` picks the graphics backend, so `opengl` has nothing to select. `anyhow`
captures a backtrace when `RUST_BACKTRACE=1` is set, with no feature needed.
Remove both from the `features` list in `Cargo.toml`.

### `vsync` and `toggle_vsync` return `PixResult<()>`

They returned a `WindowId` because SDL2 recreated the window to change vsync.
The window is now kept, so there is no new id. Drop any code that stored the
returned id.

### Audio types use `cpal` names and units

<!-- markdownlint-disable line-length -->

| 0.8                                            | Now                                                       |
| ---------------------------------------------- | --------------------------------------------------------- |
| `AudioSpecDesired { freq, channels, samples }` | `AudioSpecDesired { sample_rate, channels, buffer_size }` |
| `AudioSpec { freq, channels, samples, size }`  | `AudioSpec { sample_rate, channels, buffer_size }`        |
| `AudioFormat::S8`                              | `AudioFormat::I8`                                         |
| `AudioFormat::S16LSB`, `S16MSB`, `s16_sys()`   | `AudioFormat::I16`                                        |
| `AudioFormat::U16LSB`, `U16MSB`, `u16_sys()`   | `AudioFormat::U16`                                        |
| `AudioFormat::S32LSB`, `S32MSB`, `s32_sys()`   | `AudioFormat::I32`                                        |
| `AudioFormat::F32LSB`, `F32MSB`, `f32_sys()`   | `AudioFormat::F32`                                        |
| `PixState::audio_queued_size() -> u32`         | `PixState::audio_queued_samples() -> usize`               |
| `PixState::audio_size() -> u32`                | `PixState::audio_buffer_size() -> usize`                  |
| `PixState::audio_sample_rate() -> i32`         | `PixState::audio_sample_rate() -> u32`                    |
| `EngineBuilder::audio_sample_rate(i32)`        | `EngineBuilder::audio_sample_rate(u32)`                   |
| `EngineBuilder::audio_channels(u8)`            | `EngineBuilder::audio_channels(u16)`                      |
| `EngineBuilder::audio_buffer_size(u16)`        | `EngineBuilder::audio_buffer_size(u32)`                   |

<!-- markdownlint-enable line-length -->

Samples are native-endian, so the formats no longer name a byte order.
`AudioFormat` adds `U32`, `I64`, `U64` and `F64`.

`audio_queued_samples` and `audio_buffer_size` count samples, where the 0.8
methods counted bytes. Code that paces output by comparing the two still works.
Code that compared either against a byte count needs to divide that count by
the sample size.

### `AudioFormatNum` is a pix-engine trait

It was a re-export of `sdl2::audio::AudioFormatNum`. It is now a sealed trait
defined by this crate, implemented for `i8`, `u8`, `i16`, `u16`, `i32`, `u32`,
`f32` and `f64`. Code that only names it in a bound compiles unchanged:

```rust
fn play<T: AudioFormatNum>(samples: &[T]) { /* unchanged */ }
```

Code that imported it from `sdl2` needs to import it from
`pix_engine::prelude::*`. Code outside this crate can't implement it.

### `ControllerId` values are different

The type is unchanged, but the values are not. Under SDL2 a `ControllerId` was a
joystick index. It is now derived from a `gilrs::GamepadId`.

A `ControllerId` saved to disk, such as in input bindings, reads back as the
wrong controller. Re-record the bindings, or discard the stored ids when loading
them.

`on_controller_axis_motion` still reports `i32` over the same range, so deadzone
handling and axis scaling need no changes.

### `audio_driver()` returns different strings

It returned an SDL2 driver name. It now returns a `cpal` host name. Code that
matched on the string needs new arms.

## Silent changes

These compile unchanged but change what an application draws.

### `stroke_weight` applies to every primitive

Under SDL2, `stroke_weight` only affected `line`. Every other primitive drew a
one-pixel outline.

It now sets the outline width for `rect`, `ellipse`, `triangle`, `quad`,
`polygon`, `arc` and `bezier` as well. An application that sets a weight above 1
and draws outlined shapes gets thicker outlines than in 0.8.

To keep the old appearance, call `stroke_weight(1)` before drawing shapes and
restore the wider weight before drawing lines.

### Antialiasing looks different everywhere

SDL2_gfx had a separate antialiased function per primitive. `epaint` feathers
the edge of tessellated geometry instead. The two don't match, and
`smooth(false)` doesn't reproduce SDL2's hard edges.

Edges differ by up to a pixel and diagonal lines look softer, so screenshots
won't match 0.8 exactly. Layout and positioning are unaffected.

### Concave polygons are triangulated

`polygon` filled by scanline under SDL2. It now ear-clips the outline into a
triangle mesh.

A simple concave polygon fills the same. A self-intersecting one doesn't,
because the two methods disagree about which regions are inside. Split a
self-intersecting outline into simple pieces to get the old result.

### A font draws its styles from real faces where it has them

A `Font` now has a face per style. `Font::NOTO` ships all four, and
`FontStyle::BOLD` or `ITALIC` on it draws the designed letterforms.

A font with no face for the requested style synthesizes it. Bold draws the text
twice a pixel apart, and italic shears the glyphs. `Font::EMULOGIC` and any
single-face font take that path, so their weight and slant differ from a
designed face. Synthetic bold is a pixel wider, and `size_of` includes the extra
width.

Supply your own faces with the builder:

```rust
let font = Font::from_bytes("MyFont", REGULAR)
    .bold(BOLD)
    .italic(ITALIC)
    .bold_italic(BOLD_ITALIC);
```

Any style not supplied is synthesized from the nearest face, so a font with only
a bold face shears it for bold italic.

`Font::INCONSOLATA` is a bold face in the regular slot, so it renders heavier
than its name suggests and `FontStyle::BOLD` widens it further.

`UNDERLINE` and `STRIKETHROUGH` are unaffected.

### Text and image caches are keyed differently

Cached text is keyed on the font style, outline and wrap width. Cached images
are fingerprinted by their pixels and dimensions, so two images sharing an
address each get their own texture.

Both fix cases where 0.8 drew the wrong thing: styled text drawn from the
regular texture, and an image drawn with another image's pixels. Workarounds
for either can be removed.

### Color arithmetic ignores the color mode

`+`, `-`, `*` and `/` and their assigning forms work on the red, green and blue
channels, whatever mode a color was created in. Alpha is unchanged.

In 0.8, `*` and `/` on an Hsb or Hsl color stored hue, saturation and
brightness as red, green and blue, so halving white gave blue. `*=` and `/=`
scaled the hue along with the other levels, and `+` or `-` between colors in
different modes added levels in the first color's mode. Multiplying an Hsb color
now scales its brightness and keeps its hue and saturation. Rgb colors give the
same results as before.

## What has not changed

- **`Engine::run`.** It is still `run(&mut self, app: &mut A)`.
- **The GUI API.** All widget methods, `Theme`, `Colors`, `Spacing`, `Fonts`,
  `push_id`/`pop_id` and the `##` convention work the same.
- **`on_update`, `on_start`, `on_stop` and the event hooks.** `on_update` is
  still called once per frame.
- **`BlendMode`.** All four modes are implemented as separate render pipelines.
- **`Image`, `PixelFormat` and `TextureId`.**

## Fixed

- `on_stop` runs when an event hook returns an error.
- `frame_count()` no longer resets every second when `show_frame_rate` is on.
- `vsync` no longer leaves the primary window id dangling, which made every
  later draw fail.
- `toggle_vsync` toggles. It previously set vsync to its current value.
- The font cache no longer evicts a font in use and then panics on it.
- Textures are allocated from the graphics device, not per window, so a texture
  and its render target can belong to different windows.

## New

- **Audio and gamepads need no per-OS install step** beyond the Linux headers
  above.
- **Bundled Noto Sans ships bold, italic and bold italic**, and `Font` takes a
  face per style through `bold`, `italic` and `bold_italic`.
- **Drawing is batched.** Shapes are recorded and tessellated once per frame
  instead of issuing one draw call each. Every benchmark example is faster,
  several by more than 70%. The numbers and protocol are in
  `benches/baseline.md`.

## Not yet supported

**WebAssembly.** `Gpu::new` requests a graphics adapter before any window
exists, which a browser rejects, and it blocks on the result, which deadlocks
the browser main thread.

**Direct egui access.** Drawing goes through `egui` and `epaint`, but the crate
exposes no handle to them. `PixState::egui_ctx()` and a `pub use egui` are
planned.

**Full-resolution HiDPI.** `EngineBuilder::allow_highdpi` has no effect. Window
sizes, positions and mouse coordinates are logical points, so an 800x600 window
opens at the same size on any display. On a 2x display the canvas is drawn at
800x600 and stretched to fill the window, so edges look blockier than the
display can show.
