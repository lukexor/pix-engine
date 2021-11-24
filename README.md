# PixEngine

[![Build Status]][build] [![Latest Version]][crates.io] [![Doc Status]][docs] [![Downloads]][crates.io] ![License]

[Build Status]: https://img.shields.io/github/workflow/status/lukexor/pix-engine/CI?style=plastic
[build]: https://github.com/lukexor/pix-engine/actions/workflows/ci.yml
[Latest Version]: https://img.shields.io/crates/v/pix-engine?style=plastic
[crates.io]: https://crates.io/crates/pix-engine
[Doc Status]: https://img.shields.io/docsrs/pix-engine?style=plastic
[docs]: https://docs.rs/pix-engine/
[Downloads]: https://img.shields.io/crates/d/pix-engine?style=plastic
[License]: https://img.shields.io/crates/l/pix-engine?style=plastic

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

## Minimum Supported Rust Version

The current minimum Rust version is `1.56.1`.

## Getting Started

Creating an application is as simple as implementing the only required method of
the `AppState` trait for your application: `AppState::on_update` which gets
executed as often as possible by default. Within that function you'll have
access to a mutable `PixState` object which provides several methods for
modifying settings and drawing to the screen.

`AppState` has several additional methods that can be implemented to respond to
user and system events.

Here's an example application:

```rust no_run
use pix_engine::prelude::*;

struct MyApp;

impl AppState for MyApp {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Setup App state. `PixState` contains engine specific state and
        // utility functions for things like getting mouse coordinates,
        // drawing shapes, etc.
        s.background(Color::GRAY);
        s.font_family(Font::NOTO)?;
        s.font_size(16);
        Ok(())
    }

    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        // Main render loop. Called as often as possible, or based on `target frame rate`.
        if s.mouse_pressed() {
            s.fill(0);
        } else {
            s.fill(255);
        }
        let m = s.mouse_pos();
        s.circle([m.x(), m.y(), 80])?;
        Ok(())
    }

    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        // Teardown any state or resources before exiting.
        Ok(())
    }
}

fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
      .with_dimensions(800, 600)
      .with_title("MyApp")
      .build()?;
    let mut app = MyApp;
    engine.run(&mut app)
}
```

## Screenshots

<img src="https://github.com/lukexor/pix-engine/blob/main/images/asteroids.png?raw=true" width="400">&nbsp;&nbsp;<img src="https://github.com/lukexor/pix-engine/blob/main/images/fluid_simulation.png?raw=true" width="400">
<img src="https://github.com/lukexor/pix-engine/blob/main/images/2d_raycasting.png?raw=true" width="400">&nbsp;&nbsp;<img src="https://github.com/lukexor/pix-engine/blob/main/images/gui.png?raw=true" width="400">

## Dependencies

When using the default targets for macOS, Linux, or Windows, SDL2 libraries are
a required dependency. You can either install them manually using one of the
methods outlined in the [rust-sdl2][] crate, or you can use the `use-vcpkg`
feature flag to statically link them.

## Crate features

### Logging

This library uses the [log](https://crates.io/crates/log) crate to provide
various levels of logging. To leverage logging in your application, choose one
of the supported logger implementations and initialize it in your `main`
function.

Example using [env_logger](https://crates.io/crates/env_logger):

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

### Utility features

* **serde** -
  Enables Serialize/Deserialize implementations for most enums/structs. `serde`
  support for const generics is still pending, so many structs are not
  serializable just yet.

* **backtrace** -
  Enables the `backtrace` feature for `anyhow`, which allows printing backtraces
  based on environment variables outlined in [std::backtrace][]. Useful for
  debugging.

* **use-vcpkg** -
  Enables static linking of the SDL2 libraries which are dependencies for macOS,
  Linux, and Windows targets. Using this feature is the easiest way to get up
  and running unless you already have SDL2 installed on your system.

### Renderer features

* **opengl** -
  Forces `sdl2` to use `opengl` as its renderer. This feature is disabled by
  default, allowing `sdl2` to use whichever renderer it defaults to on the
  target system. For example, macOS defaults to `metal`.

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
[rust-sdl2]: https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries
[WASM]: https://www.rust-lang.org/what/wasm
[Tetanes]: https://crates.io/crates/tetanes
[NES]: https://en.wikipedia.org/wiki/Nintendo_Entertainment_System
[AppState]: crate::prelude::AppState
[AppState::on_update]: crate::prelude::AppState::on_update
[PixState]: crate::prelude::PixState
[std::backtrace]: https://doc.rust-lang.org/std/backtrace/index.html#environment-variables
[github issue tracker]: https://github.com/lukexor/pix-engine/issues
[LICENSE-APACHE]: http://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: http://opensource.org/licenses/MIT
[OneLoneCoder]: https://github.com/OneLoneCoder/
[olcPixelGameEngine]: https://github.com/OneLoneCoder/olcPixelGameEngine
[The Coding Train]: https://www.youtube.com/channel/UCvjgXvBlbQiydffZU7m1_aw
[p5js]: https://p5js.org/
[Dear ImGui]: https://github.com/ocornut/imgui
