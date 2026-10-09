use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::registry::models::{Registry, Theme};

/// Where the registry is fetched from when no local checkout is configured.
pub const DEFAULT_REMOTE: &str =
    "https://raw.githubusercontent.com/modpotato/craftcn/develop/registry";

/// Point craftcn at a local registry checkout, e.g. `CRAFTCN_REGISTRY=$PWD/registry`.
const ENV_LOCAL: &str = "CRAFTCN_REGISTRY";
/// Override the remote registry base URL (for forks or staging registries).
const ENV_REMOTE: &str = "CRAFTCN_REGISTRY_URL";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrySource {
    /// A registry directory on disk containing `index.json`, `themes.json` and `components/`.
    Directory(PathBuf),
    /// A remote registry. Files are cached under `cache` after the first successful fetch.
    Remote {
        base_url: String,
        cache: Option<PathBuf>,
    },
}

pub struct RegistryClient {
    source: RegistrySource,
    http: reqwest::Client,
}

impl RegistryClient {
    /// Resolves the source from the environment: `CRAFTCN_REGISTRY` first, otherwise the
    /// remote registry with the default on-disk cache.
    pub fn from_env() -> Self {
        let source = match std::env::var_os(ENV_LOCAL) {
            Some(dir) => RegistrySource::Directory(PathBuf::from(dir)),
            None => RegistrySource::Remote {
                base_url: std::env::var(ENV_REMOTE).unwrap_or_else(|_| DEFAULT_REMOTE.to_string()),
                cache: default_cache_dir(),
            },
        };
        Self::with_source(source)
    }

    pub fn with_source(source: RegistrySource) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(format!("craftcn/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { source, http }
    }

    pub fn source(&self) -> &RegistrySource {
        &self.source
    }

    pub async fn index(&self) -> Result<Registry> {
        let text = self.read_root_file("index.json").await?;
        serde_json::from_str(&text).context("registry index.json is not valid")
    }

    pub async fn themes(&self) -> Result<Vec<Theme>> {
        let text = self.read_root_file("themes.json").await?;
        serde_json::from_str(&text).context("registry themes.json is not valid")
    }

    /// Reads a component source file, e.g. `ui/core/BaseMenu.java`.
    pub async fn component_file(&self, component: &str, file: &str) -> Result<String> {
        validate_component_name(component)?;
        let relative = safe_relative_path(file)?;
        let path = format!(
            "components/{component}/{}",
            relative.to_string_lossy().replace('\\', "/")
        );
        self.read_root_file(&path).await
    }

    /// Re-downloads `index.json` and `themes.json` and drops cached component files.
    /// A local directory source has nothing to refresh.
    pub async fn refresh(&self) -> Result<()> {
        let RegistrySource::Remote { base_url, cache } = &self.source else {
            return Ok(());
        };

        for name in ["index.json", "themes.json"] {
            let text = self
                .fetch(&format!("{}/{name}", base_url.trim_end_matches('/')))
                .await?;
            if let Some(cache) = cache {
                write_cache(cache, name, &text)?;
            }
        }

        if let Some(cache) = cache {
            let components = cache.join("components");
            if components.exists() {
                fs::remove_dir_all(&components)
                    .with_context(|| format!("failed to clear {}", components.display()))?;
            }
        }

        Ok(())
    }

    async fn read_root_file(&self, relative: &str) -> Result<String> {
        match &self.source {
            RegistrySource::Directory(dir) => {
                let path = dir.join(relative);
                fs::read_to_string(&path)
                    .with_context(|| format!("failed to read {}", path.display()))
            }
            RegistrySource::Remote { base_url, cache } => {
                if let Some(cache) = cache {
                    let cached = cache.join(relative);
                    if cached.exists() {
                        return fs::read_to_string(&cached)
                            .with_context(|| format!("failed to read {}", cached.display()));
                    }
                }

                let url = format!("{}/{relative}", base_url.trim_end_matches('/'));
                let text = self.fetch(&url).await?;

                if let Some(cache) = cache {
                    write_cache(cache, relative, &text)?;
                }

                Ok(text)
            }
        }
    }

    async fn fetch(&self, url: &str) -> Result<String> {
        let response = self
            .http
            .get(url)
            .send()
            .await
            .with_context(|| format!("failed to reach {url}"))?;

        if !response.status().is_success() {
            bail!(
                "registry request to {url} failed (HTTP {}). Check your connection, or set {ENV_LOCAL} to a local registry checkout.",
                response.status()
            );
        }

        response
            .text()
            .await
            .with_context(|| format!("failed to read response from {url}"))
    }
}

fn default_cache_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("dev", "modpotato", "craftcn")
        .map(|dirs| dirs.cache_dir().join("registry"))
}

fn write_cache(cache: &Path, relative: &str, text: &str) -> Result<()> {
    let path = cache.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&path, text).with_context(|| format!("failed to write {}", path.display()))
}

/// Component names are lowercase kebab-case, e.g. `paginated-menu`.
pub fn validate_component_name(name: &str) -> Result<()> {
    let valid = !name.is_empty()
        && name.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        });

    if !valid {
        bail!("invalid component name '{name}'");
    }
    Ok(())
}

/// Registry file paths must stay inside the component directory.
pub fn safe_relative_path(path: &str) -> Result<PathBuf> {
    let candidate = Path::new(path);
    let mut clean = PathBuf::new();

    for component in candidate.components() {
        match component {
            Component::Normal(part) => clean.push(part),
            _ => bail!("registry path '{path}' must be relative and must not contain '..'"),
        }
    }

    if clean.as_os_str().is_empty() {
        bail!("registry path is empty");
    }

    Ok(clean)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsafe_registry_paths() {
        assert!(safe_relative_path("../../etc/passwd").is_err());
        assert!(safe_relative_path("/etc/passwd").is_err());
        assert!(safe_relative_path("").is_err());
        assert_eq!(
            safe_relative_path("ui/core/BaseMenu.java").unwrap(),
            PathBuf::from("ui").join("core").join("BaseMenu.java")
        );
    }

    #[test]
    fn validates_component_names() {
        assert!(validate_component_name("paginated-menu").is_ok());
        assert!(validate_component_name("text-fx").is_ok());
        assert!(validate_component_name("../evil").is_err());
        assert!(validate_component_name("Upper").is_err());
        assert!(validate_component_name("double--dash").is_err());
    }

    #[tokio::test]
    async fn reads_index_and_components_from_directory_source() {
        let registry = Path::new(env!("CARGO_MANIFEST_DIR")).join("registry");
        let client = RegistryClient::with_source(RegistrySource::Directory(registry));

        let index = client.index().await.expect("index.json should parse");
        assert!(index.get_component("base-menu").is_some());

        let themes = client.themes().await.expect("themes.json should parse");
        assert!(themes.iter().any(|t| t.name == "default"));

        let source = client
            .component_file("base-menu", "ui/core/BaseMenu.java")
            .await
            .expect("base-menu source should exist");
        assert!(source.contains("class BaseMenu"));
    }
}
