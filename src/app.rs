/// Handles the app, UI is handled by the ui.rs file
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

use anyhow::anyhow;
use ratatui_image::{
    picker::{self, Picker},
    protocol::StatefulProtocol,
};

use crate::{
    backend::{
        config::Config,
        repository::{Remote, RepositoryManifest},
        wallpaper::WpManifest,
    },
    ui::Page,
    utils::ehandle,
};

pub struct App {
    pub config: Option<Config>,
    pub config_path: PathBuf,
    pub approot: PathBuf,
    pub current_page: Page,
    pub is_setup: bool,
    pub wp_items: Vec<(WpManifest, RepositoryManifest)>, // This contains all packages, fetched on startup. Might redo?
    pub ui_list_items: Vec<(bool, WpManifest)>,          // Current chosen item
    //pub ui_current_thumbnail: StatefulProtocol, // Current thumbnail_url
    pub repositories: Vec<RepositoryManifest>,
}

impl App {
    pub fn new() -> Self {
        //let default_img = image::load_from_memory(include_bytes!("assets/default.png")).unwrap();
        /*let picker = Picker::from_query_stdio();
        let thumbnail = match picker {
            Ok(mut x) => Some(x.new_resize_protocol(default_img)),
            Err(e) => {
                eprintln!(
                    "Warning: you are using an unsupported terminal, no images will be displayed: {}", e
                );
                None
            }
        };*/
        // Creates the ratatui root - required for every application.
        Self {
            config: Option::default(),
            config_path: dirs::config_dir().unwrap().join("wctl"),
            approot: dirs::data_dir().unwrap().join("wctl"),
            current_page: Page::default(),
            is_setup: false,
            wp_items: vec![],
            ui_list_items: vec![],
            //ui_current_thumbnail: thumbnail.unwrap(), // This is initialized here because there's no other
            // way to do it
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
            //let remotes: Vec<Remote> = rp.iter().map(|f| f.to_remote()).collect();
            self.repositories = rp
        }
        // If not Some just return because something went wrong and mutating repositories at this
        // time is not good
    }

    /// Load packages
    pub fn load_packages(&mut self) -> anyhow::Result<()> {
        // First clear so we don't get duplicates!
        self.wp_items = vec![];
        for repo in self.repositories.clone() {
            let mut repo_items = repo.load_packages(self)?;
            self.wp_items.append(&mut repo_items);
        }
        Ok(())
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

        // Load packages
        ehandle(
            self.load_packages(),
            None,
            Some(|x| {
                eprintln!("Failed to load packages");
                x
            }),
        );

        for (i, package) in self.wp_items.iter().enumerate() {
            if i == 0 {
                self.ui_list_items.push((true, package.0.clone()))
            } else {
                self.ui_list_items.push((false, package.0.clone()))
            }
        }

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
            create_dir_all(&approot)?;
        }

        if !approot.join("packages").try_exists().unwrap_or(false) {
            create_dir_all(&approot.join("packages"))?;
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

    /// Basically frontend for backend function, backend functions should never be directly be used
    /// by frontend so this function is Basically a layer
    pub fn add_repo(&mut self, url: String) -> anyhow::Result<()> {
        RepositoryManifest::add(&self.config_path, url)?;
        self.reload_repositories();
        Ok(())
    }

    /// Update and reload ALL repositories
    pub fn all_update_repo(&mut self) -> anyhow::Result<()> {
        for mut repo in self.repositories.clone() {
            if !repo.is_updated(self)? {
                repo.update(self)?;
            }
        }
        // Reload repositories after update
        self.reload_repositories();
        Ok(())
    }

    /// Update and reload ONE repository
    pub fn update_repo(&mut self, mut manifest: RepositoryManifest) -> anyhow::Result<()> {
        if !manifest.is_updated(self)? {
            manifest.update(self)?;
        }
        // Reload repositories after update
        self.reload_repositories();
        Ok(())
    }

    pub fn remove_repo(&mut self, manifest: RepositoryManifest) -> anyhow::Result<()> {
        manifest.remove(&self.config_path)?;
        self.reload_repositories();
        Ok(())
    }

    /// Like remove_repo but uses identifier to get the manifest
    pub fn remove_repo_id(&mut self, id: String) -> anyhow::Result<()> {
        let mut manifest: Option<RepositoryManifest> = None;
        for m in &self.repositories {
            if m.identifier == id {
                manifest = Some(m.clone());
            }
        }

        if let Some(m) = manifest {
            self.remove_repo(m)?;
            return Ok(());
        } else {
            return Err(anyhow!("No such identifier"));
        };
    }

    pub fn identifiers(config_dir: &PathBuf) -> anyhow::Result<Vec<String>> {
        RepositoryManifest::identifiers(config_dir)
    }
}
