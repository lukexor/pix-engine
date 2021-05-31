# PixEngine

## Summary

A simple, cross-platform graphics/UI engine framework with a minimal interface.

## Dependencies

* [Rust][rust]
* [SDL2][sdl2]

## Usage

In order to use the `PixEngine`, you need to implement the `AppState` interface
on your application struct. There are several methods, only one of which is
required:

* on_start - Setup functionality when the application begins.
* on_stop - Teardown functionality for when the application is closed.
* on_update - Update functionality. This is your draw loop. It's run roughly 60 times/second.

Here's an example app skeleton:

```rust
use pix_engine::prelude::*;

struct App;

impl App {
    fn new() -> Self {
        Self
    }
}

impl AppState for App {
    fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
        // Setup App state. State contains engine specific state and
        // functions like mouse coordinates, draw functions, etc.
        Ok(())
    }
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        // Update App state roughly every 16ms.
        Ok(())
    }
    fn on_stop(&mut self, s: &mut PixState) -> PixResult<()> {
        // Teardown any state or resources.
        Ok(())
    }
}

pub fn main() {
    let width = 800;
    let height = 600;
    let mut engine = PixEngine::create(width, height)
      .with_title("App Title")
      .position_centered()
      .vsync_enabled()
      .build()
      .expect("valid engine");
    let mut app = App::new();
    engine.run(&mut app).expect("engine run");
}
```

## Known Issues

See the github issue tracker.

## License

`PixEngine` is licensed under the GPLv3 license. See the `LICENSE.md` file in
the root for a copy.

## Contact

For issue reporting, please use the github issue tracker. You can contact me directly
[here](https://lukeworks.tech/contact/).

## Credits

Implementation heavily inspired by
[OneLoneCoder](https://github.com/OneLoneCoder/) and his amazing
[olcPixelGameEngine](https://github.com/OneLoneCoder/olcPixelGameEngine)
project.

Also heavily influenced by [p5js](https://p5js.org/).

[rust]: https://www.rust-lang.org/tools/install
[sdl2]: https://www.libsdl.org/
