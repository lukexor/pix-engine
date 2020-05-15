# PixEngine

## Summary

A simple, cross-platform graphics/UI engine framework with a minimal interface.

## Dependencies

* [Rust][rust]
* [SDL2][sdl2]

## Usage

Update your `Cargo.toml` file to include pix-engine:

```
pix-engine = "0.3.4"
```

The default driver is sdl2. Future versions will support wasm. To use it:

```
pix-engine = {
  version = "0.3.4",
  default-features = false,
  features = ["wasm-driver"],
}
```

In order to use the PixEngine, you need to implement the `State` interface on your
application struct. There are three methods, only one of which is required:

* on_start - Setup functionality when the application begins.
* on_stop - Teardown functionality for when the application is closed.
* on_update - Update functionality. This is your draw loop. It's run roughly 60 times/second.

Here's an example app skeleton:

```
struct App {
    // Some data fields
}

impl App {
    fn new() -> Self {
        Self {
          // Data initialization
        }
    }
}

impl State for App {
    fn on_start(&mut self, data: &mut StateData) -> PixEngineResult<bool> {
        // Setup App state. StateData contains engine specific state and functions like mouse
        // coordinates and draw functions
        // Return true to continue. False errors the application.
        Ok(true)
    }
    fn on_update(&mut self, _elapsed: f32, _data: &mut StateData) -> PixEngineResult<bool> {
        // Update App state roughly every 16ms.
        // Return true to continue, or false to abort the update loop
        Ok(true)
    }
    fn on_stop(&mut self, _data: &mut StateData) -> PixEngineResult<bool> {
        // Teardown any state or resources
        // Returning false here prevents the app from closing
        Ok(true)
    }
}

pub fn main() {
    let app = App::new();
    let width = 800;
    let height = 600;
    let vsync = true;
    let mut engine = PixEngine::new(
      "App Title".to_string(),
      app,
      800,
      600,
      vsync
    ).expect("valid engine");
    engine.run().expect("engine run");
}
```

## Known Issues

See the github issue tracker.

## License

`PixEngine` is licensed under the GPLv3 license. See the `LICENSE.md` file in the root for a copy.

## Contact

For issue reporting, please use the github issue tracker. You can contact me directly
[here](https://lukeworks.tech/contact/).

## Contact

For issue reporting, please use the github issue tracker. You can contact me directly
[here](https://lukeworks.tech/contact/).

## Credits

Implementation heavily inspired by [OneLoneCoder](https://github.com/OneLoneCoder/) and his amazing
[olcPixelGameEngine](https://github.com/OneLoneCoder/olcPixelGameEngine) project.

Also heavily influenced by [p5js](https://p5js.org/).

[rust]: https://www.rust-lang.org/tools/install
[sdl2]: https://www.libsdl.org/
