# PixEngine

[![Build Status]][build] [![Latest Version]][crates.io] [![Doc Status]][docs] [![Downloads]][crates.io] [![License]][license]

[Build Status]: https://img.shields.io/github/workflow/status/lukexor/pix-engine/CI?style=plastic
[build]: https://github.com/lukexor/pix-engine/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/pix-engine?style=plastic
[crates.io]: https://crates.io/crates/pix-engine
[Doc Status]: https://img.shields.io/docsrs/pix-engine?style=plastic
[docs]: https://docs.rs/pix-engine/
[Downloads]: https://img.shields.io/crates/d/pix-engine?style=plastic
[License]: https://img.shields.io/crates/l/pix-engine?style=plastic
[license]: https://github.com/lukexor/pix-engine/blob/main/LICENSE-MIT

## Table of Contents

 - [Summary](#summary)
 - [Minimum Supported Rust Version](#minimum-supported-rust-version)
 - [Screenshots](#screenshots)
 - [Getting Started](#getting-started)
    - [Installing Dependencies](#installing-dependencies)
    - [Creating Your Application](#creating-your-application)
    - [Build Features](#build-features)
 - [Features](#features)
    - [PixState](#pixstate)
    - [Drawing](#drawing)
    - [Audio](#audio)
    - [UI](#ui)
    - [Logging](#logging)
 - [Known Issues](#known-issues)
 - [License](#license)
 - [Contribution](#contribution)
 - [Contact](#contact)
 - [Credits](#credits)

## Summary

`pix_engine` is a cross-platform graphics & UI library for simple games,
visualizations, digital art, and graphics applications written in [Rust][],
supporting [SDL2][] (and soon [Web-Assembly][WASM]!) renderers.

The primary goal of this library is to be simple to setup and use for graphics
or algorithm exploration and is not meant to be as fully-featured as other,
larger graphics libraries.

It is intended to be more than just a toy library, however, and can be used to
drive real applications. The [Tetanes][] [NES][] emulator, for example uses
`pix_engine` for rendering, window and event handling.

Some examples of things you can create with `pix-engine`:

- 2D ray casting of scenes or objects.
- 2D games like Asteroids, Tetris, Pong, or platformers including sound effects,
  music and UI elements.
- Simple 3D scenes or games.
- Audio playback and capture applications.
- User interfaces for basic applications or configuration.
- Algorithm visualizations for sorting, searching, or particle simulations.
- Image viewing and editing.
- Visual art.

## Minimum Supported Rust Version (MSRV)

The current minimum Rust version is `1.59.0`.

## Screenshots

<img width="48%" alt="Asteroids" src="https://raw.githubusercontent.com/lukexor/pix-engine/main/images/asteroids.png">&nbsp;&nbsp;<img width="48%" alt="Maze Generation & A* Search" src="https://raw.githubusercontent.com/lukexor/pix-engine/main/images/maze.png">
<img width="48%" alt="2D Raycasting" src="https://raw.githubusercontent.com/lukexor/pix-engine/main/images/2d_raycasting.png">&nbsp;&nbsp;<img width="48%" alt="UI Widgets" src="https://raw.githubusercontent.com/lukexor/pix-engine/main/images/gui.png">
<img width="48%" alt="Fluid Simulation" src="https://raw.githubusercontent.com/lukexor/pix-engine/main/images/fluid_simulation.png">&nbsp;&nbsp;<img width="48%" alt="Matrix Rain" src="https://raw.githubusercontent.com/lukexor/pix-engine/main/images/matrix.png">

## Getting Started

### Installing Dependencies

First and foremost you'll need [Rust][] installed! Follow the latest directions
at <https://www.rust-lang.org/learn/get-started>.

When building or running applications for a desktop target such as `macOS`,
`Linux`, or `Windows` and not a [Web-Assembly][WASM] target, you must install
[SDL2][] libraries. Note for windows: You may need to install
[Visual Studio C++ Build Tools][vc++].

There are several options for installing `SDL2`, but these are the most common:

- Install via [homebrew][] for `macOS`, a package management tool like `apt` for
  `Linux` or `MSVC` for `Windows`.

For more details and installation options see the [rust-sdl2][] documentation.

#### macOS, Linux, or Windows 10 Subsystem for Linux (homebrew)

```sh
brew install sdl2 sdl2_gfx sdl2_image sdl2_mixer sdl2_ttf
```

#### Linux (package manager)

Note: The minimum `SDL2` version is `2.0.20`. Some package managers may not have
the latest versions available.

*Ubuntu*:

```sh
sudo apt-get install libsdl2-dev libsdl2-gfx-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev
```

*Fedora*:

```sh
sudo dnf install SDL2-devel SDL2_gfx-devel SDL2_image-devel SDL2_mixer-devel SDL2_ttf-devel
```

*Arch*:

```sh
sudo pacman -S sdl2 sdl2_gfx sdl2_image sdl2_mixer sdl2_ttf
```

#### Windows (MSVC)

1. Download the latest `SDL2` `MSVC` development libraries from
   <https://www.libsdl.org/download-2.0.php> e.g. (`SDL2-devel-2.0.20-VC.zip`).
2. Download the latest `SDL2_image`, `SDL2_mixer`, and `SDL2_ttf` `MSVC`
   development libraries from
   <https://www.libsdl.org/projects/>. e.g. (`SDL2_image-devel-2.0.5-VC.zip`).
3. Unzip each `.zip` file into a folder.
3. Copy library files:
   * from: `lib\x64\`
   * to: `C:\Users\{Username}\.rustup\toolchains\{current toolchain}\lib\rustlib\{current toolchain}\lib`
     where `{current toolchain}` is likely `stable-x86_64-pc-windows-msvc`.
   - *Note*: If you don't use `rustup`, See [rust-sdl2][] for more info on
     Windows installation.
4. Copy all `dll` files:
   * from: `lib\x64\`
   * to: your `cargo` project next to `Cargo.toml`.

MSVC binaries for SDL2 are also present in this repository under the `lib`
folder.

### Creating Your Application

Creating a visual or interactive application using `pix-engine` requires
implementing only a single method of the [`AppState`][AppState] trait for your
application: [`AppState::on_update`][AppState::on_update] which gets executed as
often as possible. Within that function you'll have access to a mutable
[`PixState`][PixState] object which provides several methods for modifying
settings and drawing to the screen.

[`AppState`][AppState] provides additional methods that can be implemented to
respond to user events and handle application startup and teardown.

Here's an example application which simply draws a circle following the mouse
and renders it white or black depending if the mouse is held down or not:

```rust no_run
use pix_engine::prelude::*;

struct MyApp;

impl AppState for MyApp {
    // Set up application state and initial settings. `PixState` contains
    // engine specific state and utility methods for actions like getting mouse
    // coordinates, drawing shapes, etc. (Optional)
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Set the background to GRAY and clear the screen.
        s.background(Color::GRAY);

        // Change the font family to NOTO and size to 16 instead of using the
        // defaults.
        s.font_family(Font::NOTO)?;
        s.font_size(16);

        // Returning `Err` instead of `Ok` would indicate initialization failed,
        // and that the application should terminate immediately.
        Ok(())
    }

    // Main update/render loop. Called as often as possible unless
    // `target_frame_rate` was set with a value. (Required)
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        // Set fill color to black if mouse is pressed, otherwise wite.
        if s.mouse_pressed() {
            s.fill(color!(0));
        } else {
            s.fill(color!(255));
        }

        // Draw a circle with fill color at the mouse position with a radius of
        // 80.
        let m = s.mouse_pos();
        s.circle([m.x(), m.y(), 80])?;

        Ok(())
    }

    // Clean up any state or resources before exiting such as deleting temporary
    // files or saving game state. (Optional)
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
      .with_dimensions(800, 600)
      .with_title("MyApp")
      .with_frame_rate()
      .resizable()
      .build()?;
    let mut app = MyApp;
    engine.run(&mut app)
}
```

### Build Features

The following features can be added to your `Cargo.toml` depending on your needs. e.g.:

```toml
[dependencies.pix-engine]
version = "0.6.0"
default-features = false
features = ["serde"]
```

* **serde** - Adds [serde][] `Serialize`/`Deserialize` implementations for all
  enums/structs.

* **backtrace** - Enables the `backtrace` feature for [anyhow][], which allows printing
  backtraces based on environment variables outlined in [std::backtrace][]. Useful for debugging.

* **opengl** - Forces `sdl2` to use `opengl` as its renderer. This feature is disabled by
  default, allowing `sdl2` to use whichever renderer it defaults to on the target system. For
  example, macOS defaults to `metal`.

## Features

### PixState

[`PixState`][PixState] is the global application context for the entire
`pix-engine` lifecycle from setup to teardown. It contains all of the settings
and methods required to draw pixels to the screen, manage windows, textures,
rendering settings, etc. See [Creating Your
Application](#creating-your-application) for a brief introduction to the engine
lifecycle methods and examples of using [`PixState`][PixState].

### Drawing

All of the drawing primitives for drawing shapes, text, or UI widgets are all
available on the [`PixState`][PixState] instance. Some methods are only available when the
corresponding traits are in scope. Many traits are included by default in the
[`prelude`](crate::prelude).

Some examples:

```rust ignore
// Draw a circle at `(x, y)` coordinates`(0, 0)` with a radius of `80`.
s.circle([0, 0, 80])?;

// Draw a rectangle at `(x, y)` coordinates `(10, 20)` with a width `80` and a
// height of `100`.
s.rect([10, 20, 80, 100])?;
```

There are also several convenience macros for creating shapes that can be used
for drawing, or storing inside a `struct`:

```rust ignore
// Create a triangle with points at `(x, y)` coordinates `(10, 20)`, `(30, 10)`, `(20, 25)`.
let t = tri!([10, 20], [30, 10], [20, 25]);

// Create a 3D point at `(x, y, z)` coordinates `(10, 20, 10)`.
let p = point!(10, 20, 10);

// Create a square at point `p` with a width/height of `100`.
let r = square!(p, 100);
```

#### Textures

Textures are simple a representation of pixels but have some extra flexibility:

- They can be drawn to separately from the primary canvas being rendered.
- They can be transformed, clipped, or modified.
- They can be blended together and overlayed on each other.

By default, all drawing operations target the primary window canvas. Once drawn,
the pixels are static and can only be drawn over. Using textures allows you to
create things like draggable elements, popups, animation sprites, etc.

To create a texture:

```rust ignore
// Create a texture with a width and height of 256, formatting as RGB with no
// alpha channel. You can also provide `None` as the format which will inherit
// the format of the current window.
let texture_id = s.create_texture(256, 256, PixelFormat::Rgb);

// Draw to the texture. These changes are not visible in the window.
s.with_texture(texture_id, |s: &mut PixState| -> PixResult<()> {
  s.background(Color::BLACK);
  s.text("Hello World!");
  Ok(())
})?;

// Now draw the texture to the current canvas. Specifying `None` as the `src`
// argument means use the entire texture size. The `dst` here is double the
// original texture which has the effect of scaling the texture by 2.
s.texture(texture_id, None, rect!(0, 0, 512, 512))?;

// To clean up unused textures, simply delete them.
s.delete_texture(texture_id)?;
```

### Audio

A limited form of audio support is available, with wider support coming soon. By
default, an audio queue is available that you can push samples to:

```rust ignore
s.resume_audio(); // Audio queue starts in a `Paused` state.

// Some method generating `f32` samples between 0.0 and 1.0
let samples = generate_audio();
s.enqueue_audio(&samples);
```

There is also an [`AudioCallback`](crate::prelude::AudioCallback) trait you can
implement for doing callback-based audio generation. See the `examples/` folder
for details. Using this callback you can also do limited audio recording and
playback with a microphone.

### UI

#### Summary

`pix-engine` offers an immediate mode graphical user interface ([IMGUI][]) library which allows for
rapid UI development that is performant and simple to setup/iterate on. Some limitations:

- Styling is limited to simple color themes and spacing.
- No animations or graphical effects.
- Limited layout constructs - more complicated layouts require carefully crafted code.
- Limited responsiveness for changes in viewport.

Much of the API design is inspired by [Dear ImGui][], but note the following differences:

- There are no window rendering utilities or features. Instead, separate native windows can be
  opened with UI elements rendered within. This approach simplifies window management by leveraging
  the native window features like minimize, maximize, resizing, etc.

#### End-User Guide

- `Tab`/`Shift-Tab` cycles focus through interactable elements.
- `Enter`/`Return` on an active element simulates clicking on it.
- `Ctrl+Click` (`Cmd+Click` on macOS) on a slider or drag box to edit the value as text.
  - Pressing `Tab`/`Escape`/`Return` exits editing mode.
- The mouse wheel can scroll elements that are out of view.
- Text Fields:
  - `Ctrl+Backspace` (`Cmd+Backspace` on macOS) to delete all content.
  - `Alt+Backspace` (`Option+Backspace` on macOS) to delete a single word.
  - `Ctrl+X` (`Cmd+X` on macOS) to cut contents to the system clipboard.
  - `Ctrl+C` (`Cmd+C` on macOS) to copy contents to the system clipboard.
  - `Ctrl+V` (`Cmd+V` on macOS) to paste contents from the system clipboard.

#### Programmer Notes

- See `gui` in the `examples/` folder to get started.
- The UI is generated in code with UI method calls executed on the `PixState` instance in the
  `PixState::on_update` render loop which is called every frame.
- Elements are rendered in-order from top-left, to bottom-right.
- Unless explicitly changed, each element will position itself below the previous element. Calling
  `PixState::same_line` will shift position to the right of the previous element.

#### Windows

As your application grows, you may find the need to have different views open
simultaneously. This can be done by opening up additional windows to render
into. Each window has it's own canvas, while sharing the global `PixState`
context settings. The API is very similar to working with textures.

```rust ignore
// Create a window with size of 800x600.
let window_id = s
  .window()
  .with_dimensions(800, 600)
  .with_title("My Window")
  .position_centered()
  .build()?;

// Draw to the window. These changes are immediately visible in the window.
s.with_window(window_id, |s: &mut PixState| -> PixResult<()> {
  s.background(Color::BLACK);
  s.fill(Color::RED);
  s.text("Hello World!");
  Ok(())
})?;

// A user can either close the window with the `X` button, `Ctrl-W`, `Alt-F4`,
// etc. or you can close it programatically.
s.close_window(window_id)?;
```

**Note:** One thing to consider when creating and managing widnows is that when
a window gets closed, its ID becomes invalid. Attempting to draw in an invalid
window will return an error. Thus, most window creation will also require
removing invalid window IDs from their application:

```rust ignore
fn on_window_event(
    &mut self,
    _s: &mut PixState,
    window_id: WindowId,
    event: WindowEvent,
) -> PixResult<()> {
    if event == WindowEvent::Close && self.popup_window == Some(window_id) {
      self.popup_window = None;
    }
    Ok(())
}
```

### Logging

This library uses the [log][] crate. To leverage
logging in your application, choose one of the supported logger implementations
and initialize it in your `main` function.

Example using [env_logger][]:

```rust ignore
fn main() -> PixResult<()> {
    env_logger::init();

    let mut engine = PixEngine::builder()
      .with_dimensions(800, 600)
      .with_title("MyApp")
      .build()?;
    let mut app = MyApp;
    engine.run(&mut app)
}
```

## Known Issues

See the [github issue tracker][].

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE][])
 * MIT license ([LICENSE-MIT][])

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Contact

For issue reporting, please use the [github issue tracker][]. You can also
contact me directly at <https://lukeworks.tech/contact/>.

## Credits

This has been a true passion project for several years and I can't thank the
open source community enough for the all the amazing content and support.

A special shout out to the following projects which heavily inspired the
implementation and evolution of this crate:

  * [OneLoneCoder][] and the [olcPixelGameEngine][].
  * [The Coding Train][] and [p5js][].
  * [Dear ImGui][]

[Rust]: https://www.rust-lang.org/
[SDL2]: https://crates.io/crates/sdl2/
[vc++]: https://visualstudio.microsoft.com/visual-cpp-build-tools/
[homebrew]: https://brew.sh/
[rust-sdl2]: https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries
[log]: https://crates.io/crates/log
[env_logger]: https://crates.io/crates/env_logger
[WASM]: https://www.rust-lang.org/what/wasm
[Tetanes]: https://crates.io/crates/tetanes
[NES]: https://en.wikipedia.org/wiki/Nintendo_Entertainment_System
[AppState]: crate::prelude::AppState
[AppState::on_update]: crate::prelude::AppState::on_update
[PixState]: crate::prelude::PixState
[serde]: https://crates.io/crates/serde
[anyhow]: https://crates.io/crates/anyhow
[std::backtrace]: https://doc.rust-lang.org/std/backtrace/index.html#environment-variables
[github issue tracker]: https://github.com/lukexor/pix-engine/issues
[LICENSE-APACHE]: http://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: http://opensource.org/licenses/MIT
[OneLoneCoder]: https://github.com/OneLoneCoder/
[olcPixelGameEngine]: https://github.com/OneLoneCoder/olcPixelGameEngine
[The Coding Train]: https://www.youtube.com/channel/UCvjgXvBlbQiydffZU7m1_aw
[p5js]: https://p5js.org/
[IMGUI]: http://www.johno.se/book/imgui.html
[Dear ImGui]: https://github.com/ocornut/imgui
