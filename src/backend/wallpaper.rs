// Responsible for all wallpaper configurations and "manifests"

use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WpManifest {
    pub name: String,
    pub description: String,
    pub author: Vec<String>,
    pub maintainer: String,
    pub source: Option<String>,
    pub thumbnail_url: String,
    pub download_url: String,
    pub sizes: Vec<WpSizes>,
}

impl WpManifest {
    pub fn as_lines_ui(&self) -> Vec<Line> {
        let mut base_vec = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::new().bold()),
                self.name.clone().into(),
            ]),
            Line::from(vec![
                Span::styled("Description: ", Style::new().bold()),
                self.description.clone().into(),
            ]),
            Line::from(vec![
                Span::styled("Author: ", Style::new().bold()),
                format!("[{}]", self.author.join(",")).into(),
            ]),
            Line::from(vec![
                Span::styled("Maintainer: ", Style::new().bold()),
                self.maintainer.clone().into(),
            ]),
            Line::from(vec![
                Span::styled("Sizes: ", Style::new().bold()),
                format!(
                    "[{}]",
                    self.sizes
                        .iter()
                        .map(|x| x.as_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
                .into(),
            ]),
        ];

        if let Some(x) = self.source.clone() {
            base_vec.push(Line::from(vec![
                Span::styled("Source: ", Style::new().bold()),
                x.into(),
            ]));
        }

        base_vec
    }
}

// Most common screen aspect ratios
#[derive(Debug, Deserialize, Clone)]
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
    X21x9,
}

impl WpSizes {
    pub fn as_string(&self) -> String {
        let as_str = match self {
            Self::X1x1 => "1:1",
            Self::X5x4 => "5:4",
            Self::X4x3 => "4:3",
            Self::X3x2 => "3:2",
            Self::X16x10 => "16:10",
            Self::X16x9 => "16:9",
            Self::X256x135 => "256:135",
            Self::X64x27 => "64:27",
            Self::X32x9 => "32:9",
            Self::X4x1 => "4:1",
            Self::X21x9 => "21:9",
        };
        return as_str.to_string();
    }
}
