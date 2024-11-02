use cursive::{
    views::{Dialog, TextView},
    CursiveRunnable,
};
use std::path::PathBuf;

pub struct App {
    pub config: PathBuf,
    pub approot: PathBuf,
    app: CursiveRunnable,
}

impl App {
    pub fn new() -> Self {
        // Creates the cursive root - required for every application.
        Self {
            app: cursive::default(),
            config: dirs::config_dir().unwrap().join("wpctl"),
            approot: dirs::data_dir().unwrap().join("wpctl")
        }
    }

    pub fn init(&mut self) {
        self.app.add_global_callback('q', |s| s.quit());    

        // Creates a dialog with a single "Quit" button
        self.app.add_layer(TextView::new("Hello Dialog!"));

        // Starts the event loop.
        self.app.run();
    }
}
