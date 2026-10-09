use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::minecraft::DEFAULT_TARGET;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftCNConfig {
    pub version: String,
    pub package: String,
    pub theme: String,
    /// Minecraft version the project targets, e.g. `26.2`. Used for compatibility checks
    /// and resource pack formats.
    #[serde(default = "default_minecraft")]
    pub minecraft: String,
    pub components: Vec<String>,
}

fn default_minecraft() -> String {
    DEFAULT_TARGET.to_string()
}

impl CraftCNConfig {
    pub fn load(project_root: &Path) -> Result<Self> {
        let config_path = project_root.join("craftcn.json");

        if !config_path.exists() {
            anyhow::bail!("craftcn.json not found. Run 'craftcn init' first.");
        }

        let content = fs::read_to_string(&config_path).context("Failed to read craftcn.json")?;

        let config: CraftCNConfig =
            serde_json::from_str(&content).context("Failed to parse craftcn.json")?;

        Ok(config)
    }

    pub fn save(&self, project_root: &Path) -> Result<()> {
        let config_path = project_root.join("craftcn.json");
        let content = serde_json::to_string_pretty(self)?;

        fs::write(&config_path, format!("{content}\n")).context("Failed to write craftcn.json")?;

        Ok(())
    }
}

impl Default for CraftCNConfig {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            package: "com.example.plugin".to_string(),
            theme: "default".to_string(),
            minecraft: default_minecraft(),
            components: Vec::new(),
        }
    }
}
