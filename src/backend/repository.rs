use std::{
    fs::{self, remove_dir_all, remove_file, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use git2::{build::{CheckoutBuilder, RepoBuilder}, Direction, FetchOptions, MergeOptions, Remote as git2_remote, RemoteHead, Repository, RepositoryInitOptions};
// Responsible for all repository related stuff, such as networking, reading remote host, etc.
use serde::Deserialize;

use crate::{app::App, utils::areusure};

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

        for dir in repo_dir_iter {
            // unwrap or continue on loop
            if dir.is_err() {
                continue;
            }
            let file_path = dir.unwrap().path().join("manifest.toml");
            let content = fs::read_to_string(file_path)?;
            return_vec.push(toml::from_str(&content)?);
        }
        Ok(return_vec)
    }

    /// Removes manifest PERMANENTALY
    ///
    /// Repositories should be reloaded after this, please use frontend function for cleaner
    /// interface
    pub fn remove(&self, config_dir: &PathBuf) -> anyhow::Result<()> {
        let location: PathBuf = config_dir
            .join("repositories")
            .join(format!("{}.toml", self.identifier));
        remove_dir_all(location)?;
        return Ok(());
    }

    pub fn add(config_dir: &PathBuf, url: String) -> anyhow::Result<()> {
        let rp: String = reqwest::blocking::get(url)?.text()?;
        let manifest: Self = toml::from_str(&rp)?;

        // Check if identifier is already installed
        if Self::identifiers(config_dir)?.contains(&manifest.identifier) {
            return Err(anyhow!("A repository with this ID already exists!"));
        };

        /*match areusure(
            format!(
                "Are you sure you want to add this repository?\nName: {}\nRemote Url: {}\nID: {}",
                manifest.name, manifest.git_url, manifest.identifier
            ),
            vec!['y', 'n'],
            'y',
        ) {
            'n' => return Err(anyhow!("Cancelled")),
            _ => (),
        };*/

        let repo_path: PathBuf = config_dir
            .join("repositories")
            .join(format!("{}", manifest.identifier));
        /*let mut manifest_file: File = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(manifest_path)?;
        manifest_file.write(rp.as_bytes())?;*/

        /*let repo = Repository::init_opts(
            manifest_path,
            RepositoryInitOptions::new()
                .mkdir(true)
                .origin_url(&manifest.git_url)
                .no_reinit(true)
        )?;
        repo.find_remote("origin")?.fetch(&["main"], Some(FetchOptions::new().depth(1)), None)?;*/
        let mut fo: FetchOptions = FetchOptions::new();
        fo.depth(1);
        RepoBuilder::new().fetch_options(fo).clone(&manifest.git_url, repo_path.as_path())?;        
        Ok(())
    }

    pub fn to_remote(&self) -> Remote {
        return Remote {
            url: self.git_url.clone(),
            nitems: 0,
            manifest: self.clone(),
        };
    }
    
    pub fn load_packages(&self, app: &mut App) -> anyhow::Result<Vec<WpManifest>> {
        let package_dir = app.config_path.join("repositories").join(&self.identifier);
        let mut packages = vec![];
        for package in fs::read_dir(package_dir)? {
            if package.is_err() {
                continue;
            }
            let package = package.unwrap();
            if !package.file_name().into_string().unwrap().contains("toml") || package.file_name().into_string().unwrap().contains("manifest.toml") {
                continue;
            };
            let content = fs::read_to_string(package.path())?;
            packages.push(toml::from_str::<WpManifest>(&content)?);
        };

        Ok(packages)
    }

    // Needs to work without access to the app initialized, so needs to be a little longer
    /// Get identifiers of repositories
    pub fn identifiers(config_dir: &PathBuf) -> anyhow::Result<Vec<String>> {
        let repo_dir_iter = fs::read_dir(config_dir.join("repositories"))?;

        // Initialize our return vector
        let mut return_vec: Vec<String> = vec![];

        for file in repo_dir_iter {
            // unwrap or continue on loop
            if file.is_err() {
                continue;
            }
            let name = file.unwrap().file_name().into_string();
            if name.is_err() {
                continue;
            }
            return_vec.push(name.unwrap().replace(".toml", ""));
        }
        Ok(return_vec)
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

    /*pub fn get_packages(&self) -> anyhow::Result<Vec<WpManifest>> {
        
        let mut final_items: Vec<WpManifest> = items.iter().map(|f| f.);
        Ok(final_items)
    }*/
}
