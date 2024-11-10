// Responsible for all wallpaper configurations and "manifests"

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WpManifest {
    pub name: String,
    pub description: String,
    pub author: Vec<String>,
    pub maintainer: String,
    pub thumbnail_url: String,
    pub download_url: String,
    pub sizes: Vec<WpSizes>
}

// Most common screen aspect ratios
#[derive(Debug, Deserialize)]
pub enum WpSizes {
    #[serde(rename = "1:1")]
    X1x1,
    #[serde(rename = "5:4")]
    X5x4,
    #[serde(rename = "4:3")]
    X4x3,
    #[serde(rename = "3:2")]
    X3x2,
    #[serde(rename = "16:10")]
    X16x10,
    #[serde(rename = "16:9")]
    X16x9,
    #[serde(rename = "256:135")]
    X256x135,
    #[serde(rename = "64:27")]
    X64x27,
    #[serde(rename = "32:9")]
    X32x9,
    #[serde(rename = "4:1")]
    X4x1,
    #[serde(rename = "21:9")]
    X21x9
}
