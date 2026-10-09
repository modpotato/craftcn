//! Shared install/remove logic used by `init`, `add` and `remove`.

use anyhow::{anyhow, bail, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::CraftCNConfig;
use crate::java::manipulator::JavaManipulator;
use crate::minecraft;
use crate::registry::client::{safe_relative_path, RegistryClient};
use crate::registry::models::{Component, Registry};
use crate::utils::project::{find_project_root, java_source_root};

/// A CraftCN project: its root directory and parsed `craftcn.json`.
pub struct Project {
    pub root: PathBuf,
    pub config: CraftCNConfig,
}

impl Project {
    pub fn load() -> Result<Self> {
        let root = find_project_root()?;
        let config = CraftCNConfig::load(&root)?;
        Ok(Self { root, config })
    }

    /// The package directory in `src/main/java`, where installed sources live.
    pub fn source_root(&self) -> PathBuf {
        java_source_root(&self.root, &self.config.package)
    }

    pub fn save(&self) -> Result<()> {
        self.config.save(&self.root)
    }

    pub fn is_installed(&self, component: &str) -> bool {
        self.config.components.iter().any(|name| name == component)
    }
}

#[derive(Debug, Default)]
pub struct InstallReport {
    pub written: Vec<String>,
    pub skipped: Vec<String>,
}

/// Components that must be installed, dependencies first, skipping anything already present.
/// `installed` lists component names to treat as present.
pub fn resolve_dependencies(
    registry: &Registry,
    requested: &str,
    installed: &[String],
) -> Result<Vec<Component>> {
    fn visit(
        registry: &Registry,
        name: &str,
        installed: &[String],
        stack: &mut Vec<String>,
        order: &mut Vec<Component>,
    ) -> Result<()> {
        if installed.iter().any(|n| n == name) || order.iter().any(|c| c.name == name) {
            return Ok(());
        }
        if stack.iter().any(|n| n == name) {
            bail!("dependency cycle: {} -> {name}", stack.join(" -> "));
        }

        let component = registry
            .get_component(name)
            .ok_or_else(|| anyhow!("Component '{name}' not found in registry"))?;

        stack.push(name.to_string());
        for dependency in &component.dependencies {
            visit(registry, dependency, installed, stack, order)?;
        }
        stack.pop();

        order.push(component.clone());
        Ok(())
    }

    let mut order = Vec::new();
    visit(registry, requested, installed, &mut Vec::new(), &mut order)?;
    Ok(order)
}

/// Writes a component's source files into the project. Existing files are left alone
/// unless `force` is set, so local edits are never silently lost.
pub async fn install_component(
    project: &Project,
    client: &RegistryClient,
    component: &Component,
    force: bool,
) -> Result<InstallReport> {
    let manipulator = JavaManipulator::new(project.config.package.clone());
    let source_root = project.source_root();
    let mut report = InstallReport::default();

    for file in &component.files {
        let relative = safe_relative_path(&file.path)?;
        let target = source_root.join(&relative);
        let display = relative.to_string_lossy().replace('\\', "/");

        if target.exists() && !force {
            report.skipped.push(display);
            continue;
        }

        let source = client.component_file(&component.name, &file.path).await?;
        let content = manipulator.process_file(&source);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, content)?;

        report.written.push(display);
    }

    Ok(report)
}

/// Adds a component and its missing dependencies to the project. Returns the names of the
/// components whose files were written.
pub async fn add_component(
    project: &mut Project,
    client: &RegistryClient,
    registry: &Registry,
    name: &str,
    force: bool,
) -> Result<Vec<String>> {
    registry.get_component(name).ok_or_else(|| {
        anyhow!("Component '{name}' not found in registry. Run 'craftcn update' if it was added recently.")
    })?;

    // With --force the requested component is reinstalled even when present.
    let installed: Vec<String> = project
        .config
        .components
        .iter()
        .filter(|n| !(force && n.as_str() == name))
        .cloned()
        .collect();

    let plan = resolve_dependencies(registry, name, &installed)?;

    println!("{} {}", "Components to install:".yellow(), plan.len());
    for component in &plan {
        println!("  • {}", component.name);
        warn_if_unsupported(&project.config.minecraft, component);
    }
    println!();

    let mut written = Vec::new();
    for component in &plan {
        println!("{} {}", "Installing".cyan(), component.name.bold());
        let report = install_component(project, client, component, force).await?;

        for path in &report.written {
            println!("  {} {}", "✓".green(), format!("Created {path}").dimmed());
        }
        for path in &report.skipped {
            println!(
                "  {} {}",
                "–".yellow(),
                format!("Kept existing {path} (use --force to overwrite)").dimmed()
            );
        }

        if !project.is_installed(&component.name) {
            project.config.components.push(component.name.clone());
        }
        written.push(component.name.clone());
    }

    project.save()?;
    Ok(written)
}

/// Removes a component's files and its entry in `craftcn.json`.
pub fn remove_component(project: &mut Project, component: &Component) -> Result<Vec<PathBuf>> {
    let source_root = project.source_root();
    let mut removed = Vec::new();

    for file in &component.files {
        let target = source_root.join(safe_relative_path(&file.path)?);
        if target.exists() {
            fs::remove_file(&target)?;
            removed.push(target.clone());
            prune_empty_dirs(target.parent(), &source_root);
        }
    }

    project.config.components.retain(|n| n != &component.name);
    project.save()?;
    Ok(removed)
}

/// Removes now-empty directories between `dir` and `stop` (exclusive).
fn prune_empty_dirs(dir: Option<&Path>, stop: &Path) {
    let mut current = dir.map(Path::to_path_buf);
    while let Some(path) = current {
        if path == stop || !path.starts_with(stop) {
            break;
        }
        if fs::remove_dir(&path).is_err() {
            break;
        }
        current = path.parent().map(Path::to_path_buf);
    }
}

/// Prints a warning when a component needs a newer Minecraft than the project targets.
pub fn warn_if_unsupported(target: &str, component: &Component) {
    if let Some(minimum) = &component.minecraft {
        if !minecraft::is_at_least(target, minimum) {
            println!(
                "  {} {} needs Minecraft {minimum}+, but this project targets {target}",
                "⚠".yellow(),
                component.name
            );
        }
    }
}

/// Components that depend on `name` and are installed in the project.
pub fn installed_dependents(project: &Project, registry: &Registry, name: &str) -> Vec<String> {
    project
        .config
        .components
        .iter()
        .filter(|installed| {
            registry
                .get_component(installed)
                .map(|c| c.dependencies.iter().any(|d| d == name))
                .unwrap_or(false)
        })
        .cloned()
        .collect()
}
