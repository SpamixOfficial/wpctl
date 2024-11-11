use std::{fs, io::Read, path::PathBuf};

use git2::{Direction, Remote as git2_remote};
// Responsible for all repository related stuff, such as networking, reading remote host, etc.
use serde::Deserialize;

use super::wallpaper::WpManifest;

/// Repository manifest file, found locally
#[derive(Deserialize, Debug, Clone)]
pub struct RepositoryManifest {
    pub name: String,
    pub pretty_url: String,
    pub git_url: String,
    pub identifier: String,
    pub version: String,
}

/// Repository data and access object
#[derive(Debug)]
pub struct Remote {
    pub url: String,
    pub nitems: usize,
    pub manifest: RepositoryManifest,
}

impl RepositoryManifest {
    pub fn load_all_repositories(config_dir: &PathBuf) -> anyhow::Result<Vec<Self>> {
        let repo_dir_iter = fs::read_dir(config_dir.join("repositories"))?;

        // Initialize our return vector
        let mut return_vec: Vec<Self> = vec![];

        for file in repo_dir_iter {
            // unwrap or continue on loop
            if file.is_err() {
                continue;
            }
            let file_path = file.unwrap().path();
            let content = fs::read_to_string(file_path)?;
            return_vec.push(toml::from_str(&content)?);
        }
        Ok(return_vec)
    }

    pub fn add_repository(config_dir: &PathBuf, url: String) -> anyhow::Result<()> {
        let rp = reqwest::blocking::get(url)?.text()?;
        // TODO: FINISH TOMORROW
        Ok(())
    }

    pub fn to_remote(&self) -> Remote {
        return Remote {
            url: self.git_url.clone(),
            nitems: 0,
            manifest: self.clone(),
        };
    }
}

impl Remote {
    /// Updates nitems
    pub fn len_packages(&mut self) -> anyhow::Result<()> {
        let mut remote = git2_remote::create_detached(self.url.as_str())?;
        remote.connect(Direction::Fetch)?;
        self.nitems = remote.list()?.len();
        remote.disconnect()?;
        Ok(())
    }

    pub fn get_package_names(&self) -> anyhow::Result<Vec<String>> {
        let mut remote = git2_remote::create_detached(self.url.as_str())?;
        remote.connect(Direction::Fetch)?;
        let items: Vec<String> = remote
            .list()?
            .iter()
            .map(|x| x.name().to_string())
            .filter(|x: &String| {
                x.ends_with("toml") && !x.contains("/") && x != &"manifest.toml".to_string()
                // Filter for toml files, in root, which isnt manifest.toml
            })
            .collect();
        remote.disconnect()?;
        Ok(items)
    }
}
