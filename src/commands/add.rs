use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;

use crate::config::CraftCNConfig;
use crate::java::manipulator::JavaManipulator;
use crate::registry::client::RegistryClient;
use crate::registry::models::{Component, Registry};

pub async fn handle_add(component_name: String, force: bool) -> Result<()> {
    let project_root = crate::utils::project::find_project_root()?;
    let config_path = project_root.join("craftcn.json");

    if !config_path.exists() {
        anyhow::bail!("CraftCN not initialized. Run 'craftcn init' first.");
    }

    let config: CraftCNConfig = serde_json::from_str(&fs::read_to_string(&config_path)?)?;

    let registry = RegistryClient::get_registry().await?;
    let component = registry
        .get_component(&component_name)
        .ok_or_else(|| anyhow::anyhow!("Component '{}' not found in registry", component_name))?;

    println!(
        "{}",
        format!("Adding component: {}", component.name)
            .bold()
            .cyan()
    );
    println!();

    let installed_components = config.components.clone();

    let components_to_install =
        resolve_dependencies(&registry, &component_name, &installed_components)?;

    println!(
        "{} {}",
        "Components to install:".yellow(),
        components_to_install.len()
    );
    for comp in &components_to_install {
        println!("  • {}", comp.name);
    }
    println!();

    if !force {
        let spinner_style = ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {msg}")?
            .progress_chars("=>-");

        let pb = ProgressBar::new(components_to_install.len() as u64);
        pb.set_style(spinner_style);
        pb.set_message("Installing components...");

        for (i, comp) in components_to_install.iter().enumerate() {
            pb.set_message(format!("Installing {}...", comp.name));
            install_component(&project_root, &config.package, comp, &registry).await?;
            pb.inc(1);

            if i < components_to_install.len() - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }

        pb.finish_with_message("Installation complete!");
    } else {
        for comp in &components_to_install {
            install_component(&project_root, &config.package, comp, &registry).await?;
        }
        println!("{} {}", "✓".green(), "Installation complete!".green());
    }

    let mut updated_config = config;
    for comp in &components_to_install {
        if !updated_config.components.contains(&comp.name) {
            updated_config.components.push(comp.name.clone());
        }
    }

    fs::write(&config_path, serde_json::to_string_pretty(&updated_config)?)?;

    println!();
    println!("{}", "✓ Components installed successfully!".green());

    Ok(())
}

fn resolve_dependencies(
    registry: &Registry,
    component_name: &str,
    installed: &[String],
) -> Result<Vec<Component>> {
    let mut to_install = Vec::new();
    let mut visited = std::collections::HashSet::new();

    fn visit(
        registry: &Registry,
        name: &str,
        installed: &[String],
        to_install: &mut Vec<Component>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        if visited.contains(name) {
            return Ok(());
        }
        visited.insert(name.to_string());

        if installed.contains(&name.to_string()) {
            return Ok(());
        }

        let component = registry
            .get_component(name)
            .ok_or_else(|| anyhow::anyhow!("Component '{}' not found", name))?;

        for dep in &component.dependencies {
            visit(registry, dep, installed, to_install, visited)?;
        }

        if !to_install.iter().any(|c| c.name == name) {
            to_install.push(component.clone());
        }

        Ok(())
    }

    visit(
        registry,
        component_name,
        installed,
        &mut to_install,
        &mut visited,
    )?;

    Ok(to_install)
}

async fn install_component(
    project_root: &Path,
    package: &str,
    component: &Component,
    _registry: &Registry,
) -> Result<()> {
    let component_path = project_root.join("src").join("main").join("java");
    let package_path = package.replace('.', "/");

    let manipulator = JavaManipulator::new(package.to_string());

    for file_spec in &component.files {
        let content = RegistryClient::get_file_content(&component.name, &file_spec.path).await?;

        let modified_content = manipulator.process_file(&content)?;

        let full_path = component_path.join(&package_path).join(&file_spec.path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&full_path, modified_content)?;

        println!(
            "{} {}",
            "  ✓".green(),
            format!("Created {}", file_spec.path).dimmed()
        );
    }

    Ok(())
}
