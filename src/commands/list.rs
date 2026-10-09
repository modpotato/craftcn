use anyhow::Result;
use colored::Colorize;

use crate::config::CraftCNConfig;
use crate::registry::client::RegistryClient;
use crate::registry::models::category_title;
use crate::utils::project::find_project_root;

pub async fn handle_list(category_opt: Option<String>, installed_only: bool) -> Result<()> {
    let client = RegistryClient::from_env();
    let registry = client.index().await?;

    let installed: Vec<String> = if installed_only {
        let root = find_project_root()?;
        CraftCNConfig::load(&root)?.components
    } else {
        // Mark installed components whenever a project is nearby, without requiring one.
        find_project_root()
            .ok()
            .and_then(|root| CraftCNConfig::load(&root).ok())
            .map(|config| config.components)
            .unwrap_or_default()
    };

    println!("{}", "Available Components".bold().cyan());
    println!("{}", "═".repeat(50).cyan());
    println!();

    let wanted = category_opt.map(|c| c.to_uppercase());
    let mut shown = 0;

    for (category, components) in registry.by_category() {
        if wanted.as_deref().is_some_and(|w| w != category) {
            continue;
        }

        let components: Vec<_> = components
            .into_iter()
            .filter(|c| !installed_only || installed.contains(&c.name))
            .collect();
        if components.is_empty() {
            continue;
        }

        println!("{} {}", category.yellow().bold(), category_title(&category));
        println!("{}", "─".repeat(50).dimmed());

        for component in components {
            shown += 1;
            let marker = if installed.contains(&component.name) {
                "✓".green()
            } else {
                " ".dimmed()
            };

            let mut badges = Vec::new();
            if let Some(minimum) = &component.minecraft {
                badges.push(format!("MC {minimum}+"));
            }
            if component.resource_pack {
                badges.push("resource pack".to_string());
            }

            println!(
                "  {} {} {}",
                marker,
                component.name.cyan(),
                component.description.dimmed()
            );

            if !badges.is_empty() || !component.dependencies.is_empty() {
                let mut details = badges;
                if !component.dependencies.is_empty() {
                    details.push(format!("deps: {}", component.dependencies.join(", ")));
                }
                println!("    {}", details.join("  ·  ").dimmed());
            }
        }

        println!();
    }

    if installed_only && shown == 0 {
        println!(
            "{}",
            "No components installed. Run 'craftcn add <component>' to install components."
                .yellow()
        );
    }

    Ok(())
}
