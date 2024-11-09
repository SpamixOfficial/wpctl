// Responsible for all repository related stuff, such as networking, reading remote host, etc.
use serde::Deserialize;


// Repository manifest file, found locally
#[derive(Deserialize, Debug)]
pub struct RepositoryManifest {
    pub name: String,
    pub pretty_url: String,
    pub git_url: String,
    pub identifier: String,
}

// Repository data, dynamic, not read from manifest but rather from remote
#[derive(Debug)]
pub struct Repository {
    pub remote_url: String,
    pub nitems: usize,
    pub manifest: RepositoryManifest
}
