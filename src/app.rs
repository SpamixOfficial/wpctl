/// Handles the app, UI is handled by the ui.rs file
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

use crate::{
    backend::{config::Config, repository::RepositoryManifest, wallpaper::WpManifest},
    ui::Page,
    utils::ehandle,
};

pub struct App {
    pub config: Option<Config>,
    pub config_path: PathBuf,
    pub approot: PathBuf,
    pub current_page: Page,
    pub is_setup: bool,
    pub wp_items: Vec<WpManifest>,
    pub repositories: Vec<RepositoryManifest>,
}

impl App {
    pub fn new() -> Self {
        // Creates the cursive root - required for every application.
        Self {
            config: Option::default(),
            config_path: dirs::config_dir().unwrap().join("wpctl"),
            approot: dirs::data_dir().unwrap().join("wpctl"),
            current_page: Page::default(),
            is_setup: false,
            wp_items: vec![],
            repositories: vec![], // Not loaded on startup because we don't know if set up yet
        }
    }

    /// All repositories are loaded/re-loaded on request through this function
    ///
    /// Doesn't just have to be used on reload, also used on first load
    pub fn reload_repositories(&mut self) {
        let repositories = ehandle(
            RepositoryManifest::load_all_repositories(&self.config_path),
            None,
            Some(|e| {
                eprintln!("[*] Error while loading repositories");
                e
            }),
        );

        if let Some(rp) = repositories {
            self.repositories = rp
        }
        // If not Some just return because something went wrong and mutating repositories at this
        // time is not good
    }

    /// Handles all of the needed setup functions, like checks and such!
    pub fn init(&mut self) {
        // Load configuration
        self.config = ehandle(
            Config::load(&self.config_path.join("config.toml")),
            None,
            Some(|e| {
                eprintln!("[*] Error while loading configuration");
                e
            }),
        );
        // Load repositories
        self.reload_repositories();

        self.is_setup = self.is_setup();
    }

    pub fn install(config_dir: PathBuf, approot: PathBuf) -> anyhow::Result<()> {
        if !config_dir.try_exists().unwrap_or(false) {
            create_dir_all(&config_dir)?;
        }

        // Create repositories
        if !config_dir
            .join("repositories")
            .try_exists()
            .unwrap_or(false)
        {
            create_dir_all(&config_dir.join("repositories"))?;
        }

        // Here we can assume that the config dir exists, so we can check for the config file
        // If it's just an error in the config file during loading we don't want to overwrite the
        // file, so we just check if it exists here
        if !config_dir.join("config.toml").try_exists().unwrap_or(false) {
            let mut out_file = File::create(config_dir.join("config.toml"))?;
            out_file.write_all(Config::default_config().as_bytes())?;
        }

        if !approot.try_exists().unwrap_or(false) {
            create_dir_all(approot)?;
        }

        Ok(())
    }

    /// Check if our program is set up/installed
    fn is_setup(&self) -> bool {
        if !(self.config_path.try_exists().unwrap_or(false)
            || self.approot.try_exists().unwrap_or(false))
        {
            return false;
        }

        if !self
            .config_path
            .join("repositories")
            .try_exists()
            .unwrap_or(false)
        {
            return false;
        }

        if self.config.is_none() {
            return false;
        }

        return true;
    }
}
