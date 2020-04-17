// use super::{setting::Setting, Error, Result, State};
// use crate::renderer::Renderer;

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum WindowPos {
//     Centered,
//     Positioned(i32),
// }

// impl Default for WindowPos {
//     fn default() -> Self {
//         Self::Centered
//     }
// }

// #[derive(Default, Clone)]
// pub(crate) struct WindowBuilder {
//     title: String,
//     width: u32,
//     height: u32,
//     scale: u32,
//     x: WindowPos,
//     y: WindowPos,
//     fullscreen: bool,
//     no_vsync: bool,
//     hidden: bool,
//     borderless: bool,
//     resizable: bool,
//     minimized: bool,
//     maximized: bool,
//     input_grabbed: bool,
// }

// impl WindowBuilder {
//     /// Initializes a new `WindowBuilder`.
//     pub fn new(title: &str, width: u32, height: u32) -> Self {
//         Self {
//             title: title.to_owned(),
//             width,
//             height,
//             scale: 1,
//             ..Default::default()
//         }
//     }

//     /// Sets the window position.
//     pub fn position(&mut self, x: i32, y: i32) -> &mut Self {
//         self.x = WindowPos::Positioned(x);
//         self.y = WindowPos::Positioned(y);
//         self
//     }

//     /// Centers the window.
//     pub fn position_centered(&mut self) -> &mut Self {
//         self.x = WindowPos::Centered;
//         self.y = WindowPos::Centered;
//         self
//     }

//     /// Sets the window to fullscreen.
//     pub fn fullscreen(&mut self) -> &mut Self {
//         self.fullscreen = true;
//         self
//     }

//     /// Disables vsync.
//     pub fn no_vsync(&mut self) -> &mut Self {
//         self.no_vsync = true;
//         self
//     }

//     /// Hides the window.
//     pub fn hidden(&mut self) -> &mut Self {
//         self.hidden = true;
//         self
//     }

//     /// Removes the window decoration.
//     pub fn borderless(&mut self) -> &mut Self {
//         self.borderless = true;
//         self
//     }

//     /// Sets the window to be resizable.
//     pub fn resizable(&mut self) -> &mut Self {
//         self.resizable = true;
//         self
//     }

//     /// Minimizes the window.
//     pub fn minimized(&mut self) -> &mut Self {
//         self.minimized = true;
//         self
//     }

//     /// Maximizes the window.
//     pub fn maximized(&mut self) -> &mut Self {
//         self.maximized = true;
//         self
//     }

//     /// Sets the window to have grabbed input focus.
//     pub fn input_grabbed(&mut self) -> &mut Self {
//         self.input_grabbed = true;
//         self
//     }

//     /// Builds a `Window` instance using the settings from the `WindowBuilder` consuming it in the
//     /// process.
//     pub fn build(self) -> Result<Window> {
//         // Create window in renderer
//         Ok(Window::new(0, &self.title))
//     }
// }

//#[derive(Default, Clone)]
//pub(crate) struct Window {
//    id: u32,
//    title: String,
//    focused: bool,
//    pub(crate) settings: Setting,
//    settings_stack: Vec<Setting>,
//}

//impl Window {
//    /// Creates a new Window instance with default settings.
//    pub(crate) fn new(id: u32, title: &str) -> Self {
//        Self {
//            id,
//            title: title.to_owned(),
//            settings_stack: Vec::new(),
//            ..Default::default()
//        }
//    }

//    /// Gets the id of the window.
//    pub fn id(&self) -> u32 {
//        self.id
//    }

//    /// Gets the title of the window.
//    pub fn title(&self) -> &str {
//        &self.title
//    }

//    /// Gets a mutable reference to the title of the window.
//    pub fn title_mut(&mut self) -> &mut String {
//        &mut self.title
//    }

//    /// Sets the title of the window.
//    pub fn set_title(&mut self, title: &str) {
//        self.title = title.to_owned();
//    }

//    /// Pushes current window settings to be later retrieved with `Window::pop()`.
//    pub fn push(&mut self) {
//        self.settings_stack.push(self.settings.clone());
//    }

//    /// Pops previous window settings if there are any, otherwise has no effect.
//    pub fn pop(&mut self) {
//        if let Some(settings) = self.settings_stack.pop() {
//            self.settings = settings;
//        }
//    }
//}

//impl State {
//    /// Window Management

//    /// Get a window based on the current window target.
//    pub(crate) fn get_window(&self) -> &Window {
//        let target = self.window_target();
//        self.windows
//            .iter()
//            .find(|w| target == w.id())
//            .expect("valid window target")
//    }
//    /// Get a mutable window based on the current window target.
//    pub(crate) fn get_window_mut(&mut self) -> &mut Window {
//        let target = self.window_target();
//        self.windows
//            .iter_mut()
//            .find(|w| target == w.id())
//            .expect("valid window target")
//    }

//    /// Get the default window_id.
//    pub fn default_window(&self) -> u32 {
//        self.default_window_target
//    }
//    /// Get the window_id of the current window target.
//    pub fn window_target(&self) -> u32 {
//        self.window_target.unwrap_or(self.default_window_target)
//    }

//    /// Set a temporary window target for setting and drawing operations. Passing `None` will unset
//    /// the temporary target and use the default window target.
//    ///
//    /// Errors if the window_id is not a valid window_id.
//    pub fn set_window_target<W: Into<Option<u32>>>(&mut self, target: W) -> Result<()> {
//        match target.into() {
//            Some(window_id) => {
//                if self.windows.iter().any(|w| window_id == w.id()) {
//                    self.window_target = window_id;
//                    self.renderer.set_window_target(window_id);
//                    Ok(())
//                } else {
//                    Err(Error::InvalidWindowTarget(window_id))
//                }
//            }
//            None => self.window_target = None,
//        }
//    }

//    /// Create and open a new window.
//    ///
//    /// Errors if the window can't be created for any reason.
//    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> Result<u32> {
//        let id = self.renderer.create_window(title, width, height)?;
//        Ok(id)
//    }

//    /// Close the current window target.
//    ///
//    /// Sets `should_close` to true when all windows are closed.
//    pub fn close_window(&mut self) {
//        let target = self.window_target();
//        if let Some(target) = self.window_target {
//            self.window_target = None;
//        }
//        self.windows.retain(|w| target != w.id());
//        self.renderer.close_window();
//        self.should_close = self.windows.is_empty();
//    }

//    /// Adds a pre-constructed window to the current engine State.
//    pub(crate) fn add_window(&mut self, window: Window) {
//        self.windows.push(window);
//    }
//}
