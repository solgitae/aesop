use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

use crate::error::AesopError;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
    pub index: IndexConfig,
    pub ui: UiConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SourceConfig {
    pub path: String,
    pub enalbed: Option<bool>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IndexConfig {
    pub max_days: Option<u32>,
    pub max_size_mb: Option<u64>,
    pub max_memory_lines: Option<usize>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UiConfig {
    pub theme: Option<String>,
    pub nerd_fonts: Option<bool>,
    pub batch_ms: Option<u64>,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| AesopError::ConfigNotFound { path: path.clone() })?;

        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default_path() -> PathBuf {
        let local = PathBuf::from("aesop.toml");
        if local.exists() {
            return local;
        }

        dirs::home_dir()
            .unwrap_or_default()
            .join(".aesop/config.toml")
    }

    pub fn default_content() -> &'static str {
        r#"[[sources]]
        path    = "./logs/**/*.log"
        enabled = true
        color   = "blue"

        [index]
        max_days         = 3
        max_size_mb      = 512
        max_memory_lines = 100000

        [ui]
        theme      = "dark"
        nerd_fonts = true
        batch_ms   = 16
        "#
    }
}

impl SourceConfig {
    pub fn is_enabled(&self) -> bool {
        self.enalbed.unwrap_or(true)
    }
}

impl IndexConfig {
    pub fn max_days(&self) -> u32 {
        self.max_days.unwrap_or(3)
    }

    pub fn max_size_bytes(&self) -> u64 {
        self.max_size_mb.unwrap_or(512) * 1024 * 1024
    }

    pub fn max_memory_lines(&self) -> usize {
        self.max_memory_lines.unwrap_or(100_000)
    }
}

impl UiConfig {
    pub fn theme(&self) -> &str {
        self.theme.as_deref().unwrap_or("dark")
    }

    pub fn nerd_fonts(&self) -> bool {
        self.nerd_fonts.unwrap_or(true)
    }

    pub fn batch_ms(&self) -> u64 {
        self.batch_ms.unwrap_or(16)
    }
}
