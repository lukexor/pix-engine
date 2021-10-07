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

The current minimum Rust version is `1.55.0`.

## Getting Started

Creating an application is as simple as implementing the only required method of
the [AppState] trait for your application: [AppState::on_update] which gets
executed as often as possible by default. Within that function you'll have
access to a mutable [PixState] object which provides several methods for
modifying settings and drawing to the screen.

[AppState] has several additional methods that can be implemented to respond to
user and system events.

Here's an example application:

```rust no_run
use pix_engine::prelude::*;

struct MyApp;

impl AppState for MyApp {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Setup App state. PixState contains engine specific state and
        // utility functions for things like getting mouse coordinates,
        // drawing shapes, etc.
        s.background(220)?;
        s.circle([10, 10, 100])?;
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
      .position_centered()
      .build();
    let mut app = MyApp;
    engine.run(&mut app)
}
```

## Crate features

### Utility features

* **serde** -
  Enables Serialize/Deserialize implementations for most enums/structs. `serde`
  support for const generics is still pending, so many structs are not
  serializable just yet.

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

For issue reporting, please use the [github issue tracker][]. You can contact me directly
[here](https://lukeworks.tech/contact/).

## Credits

Implementation heavily inspired by
[OneLoneCoder](https://github.com/OneLoneCoder/) and his amazing
[olcPixelGameEngine](https://github.com/OneLoneCoder/olcPixelGameEngine)
project.

Also heavily influenced by [p5js](https://p5js.org/).

[Rust]: https://www.rust-lang.org/
[SDL2]: https://crates.io/crates/sdl2/
[WASM]: https://www.rust-lang.org/what/wasm
[Tetanes]: https://crates.io/crates/tetanes
[NES]: https://en.wikipedia.org/wiki/Nintendo_Entertainment_System
[AppState]: crate::prelude::AppState
[AppState::on_update]: crate::prelude::AppState::on_update
[PixState]: crate::prelude::PixState
[github issue tracker]: https://github.com/lukexor/pix-engine/issues
[LICENSE-APACHE]: http://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: http://opensource.org/licenses/MIT
