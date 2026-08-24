# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this
repository.

## Project

`pix-engine` is a single-crate, cross-platform graphics/UI library (a Processing-like drawing API
plus an immediate-mode GUI) backed by `wgpu` for drawing, `winit` for windowing and events, `cpal`
for audio and `gilrs` for gamepads. It is published on crates.io and used by the [TetaNES]
emulator.

[TetaNES]: https://github.com/lukexor/tetanes

## Commands

```sh
cargo build --all-targets --features serde   # what CI builds
cargo clippy                                  # CI runs with RUSTFLAGS="-Dwarnings"
cargo fmt --all --check
cargo doc --features serde                    # CI treats rustdoc warnings as errors too
cargo test --features serde                   # unit + doc tests
cargo run --example asteroids                 # any file in examples/ (gui, matrix, flocking, ...)
cargo run --profile dev-opt --example fluid_simulation  # opt-level 1, unoptimized examples crawl
```

Run a single test: `cargo test --features serde <name>`.

Setting `PIX_BENCH_FRAMES` puts any example into the frame-timing harness in `src/bench.rs`. It
discards a warmup, records that many frames, prints the distribution and exits.

```sh
PIX_BENCH_FRAMES=600 cargo run --release --features serde --example matrix
```

`benches/baseline.md` contains the recorded numbers and the protocol that produced them. Follow
that protocol when comparing, and rerun on the machine it names.

Integration tests in `tests/pix-engine.rs` construct a real `Engine`. A `winit` event loop has to
be built on the main thread and only once per process, so that target sets `harness = false` and
runs its scenarios in turn from `main`. `PIX_ENGINE_TESTS` gates them, so an ordinary `cargo test`
opens no window:

```sh
PIX_ENGINE_TESTS=1 cargo test --features serde --test pix-engine
```

`wgpu` picks a graphics backend at runtime and needs nothing installed. `Linux` builds need ALSA
and udev headers for audio and gamepads, listed per-OS in the README. `shell.nix` provides them
plus a toolchain for Nix users.

## Toolchain and MSRV

`rust-toolchain.toml` pins 1.93.1 for local development. `Cargo.toml` declares
`rust-version = "1.92.0"`, which `egui` 0.35 requires, and CI builds against that as well as
stable. Language or std features newer than the MSRV pass locally and fail in CI.

## Architecture

The whole engine is one crate. `src/lib.rs` includes `README.md` as the crate docs and enables a
long list of `warn` lints, notably `missing_docs`, `clippy::unwrap_used`, `clippy::expect_used`,
`clippy::must_use_candidate` and `rustdoc::broken_intra_doc_links`. New public items need doc
comments, and `unwrap`/`expect` need an `#[allow]` with a reason where genuinely unavoidable.

### The two central types

`PixEngine` (`src/engine.rs`) is the trait an application implements: `on_start`, `on_update` (the
only required method), `on_stop`, plus optional event hooks. `Engine::builder()` configures a
`RendererSettings`, and `Engine::run` owns the frame loop, pumping events, calling `on_update`, and
honouring `target_frame_rate`/vsync.

`PixState` (`src/state.rs`) is the single mutable context threaded through every callback. It owns
the `Renderer`, `Environment` (frame count, timing, window dimensions), `UiState` (immediate-mode
GUI bookkeeping), a `Settings` struct, a `setting_stack` for `push`/`pop`, and the `Theme`. Nearly
all user-facing API (drawing, text, textures, windows, audio, widgets) is `impl PixState` blocks
spread across `src/draw.rs`, `src/texture.rs`, `src/window.rs`, `src/audio.rs`, `src/image.rs` and
`src/gui/`. When adding a feature, the method almost always belongs on `PixState`.

### Renderer abstraction

`src/renderer.rs` defines the private `Rendering` trait, along with `TextureRenderer` in
`src/texture.rs` and `WindowRenderer` in `src/window.rs`. `src/renderer/backend.rs` implements all
three over `wgpu`. **wasm is unsupported.** `Gpu::new` picks an adapter before any window exists,
which a browser does not allow, and wasm is not built in CI.

Drawing records `epaint` shapes into a `RenderTarget` rather than issuing draw calls, and
`Rendering::present` tessellates and paints everything recorded since the last frame. A target is
an offscreen texture in every case, including a window canvas, so a canvas keeps its contents
between frames and `src/renderer/gpu.rs` copies it onto the surface. `src/renderer/shapes.rs`
translates drawing primitives into shapes with no state of its own, which makes it the one part
testable against values.

`winit` owns the frame loop. `src/app.rs` implements the `ApplicationHandler` it calls back into,
and `Engine::run` hands control to the loop. Windows are created in its callbacks, because only an
`ActiveEventLoop` can create one. `WindowRenderer::create_window` allocates the canvas and returns
an id straight away, and the window follows on the next pass.

### Drawing model

Drawing targets the current window canvas by default. `set_texture_target`/`clear_texture_target`
and `set_window_target`/`reset_window_target` redirect it. Shape types live in `src/shape/` with
`point!`, `rect!`, `circle!` and `tri!` macros. Most drawing methods accept anything `Into<Shape>`,
so `s.circle([x, y, r])` and `s.circle(c)` both work. `Settings` fields like `rect_mode`,
`ellipse_mode` and `angle_mode` change how coordinates are interpreted, so read
`src/state/settings.rs` before assuming what a coordinate tuple means.

### Immediate-mode GUI

Widgets in `src/gui/` are drawn and their state resolved in the same call during `on_update`.
Widget identity is the hash of the label combined with the current ID stack. The module docs in
`src/gui.rs` cover the `##` suffix and `push_id`/`pop_id` conventions. `UiState` tracks the hovered,
active, focused and editing element between frames. Layout is a cursor that advances downward
unless `same_line` is called.

### Features

`serde` derives across all public types, plus `backtrace` and `debug_ui`. CI builds and tests with
`serde` on, so `#[cfg_attr(feature = "serde", ...)]` attributes need to stay consistent when adding
public types.

Fonts in `assets/` are compiled in and exposed as `Font::EMULOGIC`, `Font::INCONSOLATA` and
`Font::NOTO`.

## Conventions

Commits are conventional commits. `cliff.toml` sets `filter_unconventional = true`, so anything
else is dropped from the generated changelog. Releases are automated by release-plz. Do not edit
`CHANGELOG.md` or bump versions by hand.

README is the crate documentation. Doc examples there and in `src/` are compiled by `cargo test`,
so keep them building, or mark them `no_run`/`ignore` as the existing ones do.
