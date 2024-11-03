/// Handles the app, UI is handled by the ui.rs file
use cursive::{
    views::{Dialog, TextView},
    CursiveRunnable,
};
use std::{fs::create_dir_all, path::PathBuf, process::exit};

use crate::ui::AppViews;

pub struct App {
    pub config: PathBuf,
    pub approot: PathBuf,
    pub app: CursiveRunnable,
    pub is_setup: bool,
}

impl App {
    pub fn new() -> Self {
        // Creates the cursive root - required for every application.
        Self {
            app: cursive::default(),
            config: dirs::config_dir().unwrap().join("wpctl"),
            approot: dirs::data_dir().unwrap().join("wpctl"),
            is_setup: false,
        }
    }

    /// Handles all of the needed setup functions, like checks and such!
    pub fn init(&mut self) {
        self.is_setup = self.is_setup();
    }

    pub fn install(config_dir: PathBuf, approot: PathBuf) -> Result<(), std::io::Error> {
        if !config_dir.try_exists().unwrap_or(false) {
            create_dir_all(config_dir)?;
        }
        if !approot.try_exists().unwrap_or(false) {
            create_dir_all(approot)?;
        }
        Ok(())
    }

    pub fn config_dir() -> PathBuf {
        return dirs::config_dir().unwrap().join("wpctl")
    }

    pub fn approot() -> PathBuf {
        return dirs::data_dir().unwrap().join("wpctl")
    }

    /// Check if our program is set up/installed
    fn is_setup(&self) -> bool {
        if !(self.config.try_exists().unwrap_or(false)
            || self.approot.try_exists().unwrap_or(false))
        {
            return false;
        }

        return true;
    }
}
