use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use reqwest::Client;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::registry::models::{Registry, Theme};

static REGISTRY_CACHE: OnceCell<Mutex<Option<Registry>>> = OnceCell::new();
static THEMES_CACHE: OnceCell<Mutex<Option<Vec<Theme>>>> = OnceCell::new();

pub struct RegistryClient;

impl RegistryClient {
    fn get_registry_dir() -> Result<PathBuf> {
        let mut path = std::env::current_exe()?;
        path.pop();
        path.push("registry");
        Ok(path)
    }

    fn get_local_registry_path() -> Result<PathBuf> {
        let mut path = Self::get_registry_dir()?;
        path.push("index.json");
        Ok(path)
    }

    fn get_component_path(component_name: &str, file_path: &str) -> Result<PathBuf> {
        let mut path = Self::get_registry_dir()?;
        path.push("components");
        path.push(component_name);
        path.push(file_path);
        Ok(path)
    }

    async fn fetch_registry_from_github() -> Result<String> {
        let client = Client::new();
        let url = "https://raw.githubusercontent.com/modpotato/cli/main/registry/index.json";

        let response = client
            .get(url)
            .header("User-Agent", "CraftCN CLI")
            .send()
            .await
            .context("Failed to fetch registry from GitHub")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Failed to fetch registry from GitHub (HTTP {}). Please check your internet connection or try again later.",
                response.status()
            );
        }

        let content = response.text().await?;
        Ok(content)
    }

    async fn fetch_themes_from_github() -> Result<String> {
        let client = Client::new();
        let url = "https://raw.githubusercontent.com/modpotato/cli/main/registry/themes.json";

        let response = client
            .get(url)
            .header("User-Agent", "CraftCN CLI")
            .send()
            .await
            .context("Failed to fetch themes from GitHub")?;

        if !response.status().is_success() {
            anyhow::bail!("GitHub returned status: {}", response.status());
        }

        let content = response.text().await?;
        Ok(content)
    }

    pub async fn get_registry() -> Result<Registry> {
        let cache = REGISTRY_CACHE.get_or_init(|| Mutex::new(None));

        {
            let guard = cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Cache mutex poisoned: {}", e))?;
            if let Some(registry) = guard.as_ref() {
                return Ok(registry.clone());
            }
        }

        let registry: Registry = if let Ok(local_path) = Self::get_local_registry_path() {
            if local_path.exists() {
                let content = fs::read_to_string(&local_path)?;
                serde_json::from_str(&content)?
            } else {
                let content = Self::fetch_registry_from_github().await?;
                serde_json::from_str(&content)?
            }
        } else {
            let content = Self::fetch_registry_from_github().await?;
            serde_json::from_str(&content)?
        };

        {
            let mut guard = cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Cache mutex poisoned: {}", e))?;
            *guard = Some(registry.clone());
        }

        Ok(registry)
    }

    pub async fn get_available_themes() -> Result<Vec<Theme>> {
        let cache = THEMES_CACHE.get_or_init(|| Mutex::new(None));

        {
            let guard = cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Cache mutex poisoned: {}", e))?;
            if let Some(themes) = guard.as_ref() {
                return Ok(themes.clone());
            }
        }

        let themes_content = if let Ok(local_path) = Self::get_registry_dir() {
            let themes_path = local_path.join("themes.json");
            if themes_path.exists() {
                fs::read_to_string(&themes_path)?
            } else {
                Self::fetch_themes_from_github().await?
            }
        } else {
            Self::fetch_themes_from_github().await?
        };

        let themes: Vec<Theme> = serde_json::from_str(&themes_content)?;

        {
            let mut guard = cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Cache mutex poisoned: {}", e))?;
            *guard = Some(themes.clone());
        }

        Ok(themes)
    }

    pub async fn get_file_content(component_name: &str, file_path: &str) -> Result<String> {
        let local_path = Self::get_component_path(component_name, file_path)?;

        if local_path.exists() {
            return fs::read_to_string(&local_path)
                .context(format!("Failed to read local file: {:?}", local_path));
        }

        let client = Client::new();
        let url = format!(
            "https://raw.githubusercontent.com/modpotato/cli/main/registry/components/{}/{}",
            component_name, file_path
        );

        let response = client
            .get(&url)
            .header("User-Agent", "CraftCN CLI")
            .send()
            .await
            .context("Failed to fetch file from GitHub")?;

        if !response.status().is_success() {
            anyhow::bail!("GitHub returned status: {}", response.status());
        }

        let content = response.text().await?;
        Ok(content)
    }

    pub async fn update_cache() -> Result<()> {
        let registry_content = Self::fetch_registry_from_github().await?;
        let themes_content = Self::fetch_themes_from_github().await?;

        let registry_dir = Self::get_registry_dir()?;
        fs::create_dir_all(&registry_dir)?;

        let registry_path = registry_dir.join("index.json");
        fs::write(&registry_path, registry_content)?;

        let themes_path = registry_dir.join("themes.json");
        fs::write(&themes_path, themes_content)?;

        if let Some(cache) = REGISTRY_CACHE.get() {
            let mut guard = cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Cache mutex poisoned: {}", e))?;
            *guard = None;
        }

        if let Some(cache) = THEMES_CACHE.get() {
            let mut guard = cache
                .lock()
                .map_err(|e| anyhow::anyhow!("Cache mutex poisoned: {}", e))?;
            *guard = None;
        }

        Ok(())
    }
}
