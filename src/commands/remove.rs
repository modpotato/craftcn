use anyhow::{bail, Result};
use colored::Colorize;

use crate::installer::{installed_dependents, remove_component, Project};
use crate::registry::client::RegistryClient;

pub async fn handle_remove(component_name: String, force: bool) -> Result<()> {
    let mut project = Project::load()?;

    if !project.is_installed(&component_name) {
        bail!("{component_name} is not installed in this project");
    }

    let client = RegistryClient::from_env();
    let registry = client.index().await?;

    let dependents = installed_dependents(&project, &registry, &component_name);
    if !dependents.is_empty() && !force {
        bail!(
            "{component_name} is required by {}. Remove those first, or pass --force.",
            dependents.join(", ")
        );
    }

    match registry.get_component(&component_name) {
        Some(component) => {
            let removed = remove_component(&mut project, component)?;
            for path in &removed {
                println!(
                    "  {} {}",
                    "✗".red(),
                    format!("Removed {}", path.display()).dimmed()
                );
            }
        }
        None => {
            // The component no longer exists in the registry, so there are no known files to delete.
            project.config.components.retain(|n| n != &component_name);
            project.save()?;
            println!(
                "{} Unknown to the registry, so only craftcn.json was updated. Delete its files manually.",
                "⚠".yellow()
            );
        }
    }

    println!();
    println!(
        "{} {}",
        "✓".green(),
        format!("Removed {component_name}").green()
    );

    Ok(())
}
