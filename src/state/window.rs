use super::setting::Setting;

pub(crate) struct Window {
    id: u32,
    title: String,
    pub(crate) setting_stack: Vec<Setting>,
    pub(crate) settings: Setting,
}

impl Window {
    /// Creates a new Window instance with default settings.
    pub(crate) fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: title.to_owned(),
            setting_stack: Vec::new(),
            settings: Setting::default(),
        }
    }

    /// Gets the id of the window.
    pub(crate) fn id(&self) -> u32 {
        self.id
    }

    /// Gets the title of the window.
    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    /// Gets a mutable reference to the title of the window.
    pub(crate) fn title_mut(&mut self) -> &mut String {
        &mut self.title
    }

    /// Sets the title of the window.
    pub(crate) fn set_title(&mut self, title: &str) {
        self.title = title.to_owned();
    }

    /// Pushes current window settings to be later retrieved with `Window::pop()`.
    pub(crate) fn push(&mut self) {
        self.setting_stack.push(self.settings.clone());
    }

    /// Pops previous window settings if there are any, otherwise has no effect.
    pub(crate) fn pop(&mut self) {
        if let Some(settings) = self.setting_stack.pop() {
            self.settings = settings;
        }
    }
}
