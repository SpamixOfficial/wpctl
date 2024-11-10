use std::{fs, io::Read, path::PathBuf};

// Responsible for all repository related stuff, such as networking, reading remote host, etc.
use serde::Deserialize;

/// Repository manifest file, found locally
#[derive(Deserialize, Debug, Clone)]
pub struct RepositoryManifest {
    pub name: String,
    pub pretty_url: String,
    pub git_url: String,
    pub identifier: String,
    pub version: String,
}

/// Repository data, dynamic, not read from manifest but rather from remote
#[derive(Debug)]
pub struct Remote {
    pub remote_url: String,
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

    pub fn to_remote(&self) -> Remote {
        // TODO: Get nitems here from Remote
        return Remote {
            remote_url: self.pretty_url.clone(),
            nitems: 0,
            manifest: self.clone(),
        };
    }
}
