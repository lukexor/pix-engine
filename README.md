# PixEngine

## Summary

`pix_engine` is a cross-platform graphics & UI library for simple games,
visualizations, digital art, and graphics applications written in [Rust][],
supporting [SDL2][] and [Web-Assembly][WASM] renderers.

The primary goal of this library is to be simple to setup and use for graphics
or algorithm exploration and is not meant to be as fully-featured as other,
larger graphics libraries.

It is intended to be more than just a toy library, however, and can be used to
drive real applications. The [Tetanes][] [NES][] emulator, for example uses
`pix_engine` for rendering, window and event handling.

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
        s.background(220);
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

## Known Issues

See the [github issue tracker][].

## License

See the `LICENSE.md` file in the root.

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
