use anyhow::Result;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;

use crate::config::CraftCNConfig;
use crate::registry::client::RegistryClient;

pub async fn handle_list(category_opt: Option<String>, installed_only: bool) -> Result<()> {
    let registry = RegistryClient::get_registry().await?;

    let mut installed_components: Vec<String> = Vec::new();

    if installed_only {
        let project_root = crate::utils::project::find_project_root()?;
        let config_path = project_root.join("craftcn.json");

        if config_path.exists() {
            let config: CraftCNConfig = serde_json::from_str(&fs::read_to_string(&config_path)?)?;
            installed_components = config.components;
        }
    }

    let category_descriptions: HashMap<String, &str> = HashMap::from([
        ("A".to_string(), "Inventory GUIs"),
        ("B".to_string(), "Chat Widgets"),
        ("C".to_string(), "HUD & Visuals"),
        ("D".to_string(), "Utilities"),
    ]);

    let mut categories: Vec<String> = registry.categories.keys().cloned().collect();
    categories.sort();

    println!("{}", "Available Components".bold().cyan());
    println!("{}", "═".repeat(50).cyan());
    println!();

    for category in categories {
        if let Some(cat_opt) = &category_opt {
            if *cat_opt != category {
                continue;
            }
        }

        let components = &registry.categories.get(&category).unwrap();

        if components.is_empty() {
            continue;
        }

        println!(
            "{} {}",
            category.yellow().bold(),
            category_descriptions.get(&category).unwrap_or(&"")
        );
        println!("{}", "─".repeat(50).dimmed());

        let mut sorted_components: Vec<_> = components.iter().collect();
        sorted_components.sort_by_key(|c| c.name.clone());

        for component in sorted_components {
            if installed_only && !installed_components.contains(&component.name) {
                continue;
            }

            let installed_indicator = if installed_components.contains(&component.name) {
                "✓".green()
            } else {
                " ".dimmed()
            };

            println!(
                "  {} {} {}",
                installed_indicator,
                component.name.cyan(),
                component.description.dimmed()
            );

            if !component.dependencies.is_empty() {
                println!(
                    "    {}",
                    format!("Deps: {}", component.dependencies.join(", ")).dimmed()
                );
            }
        }

        println!();
    }

    if installed_only && installed_components.is_empty() {
        println!(
            "{}",
            "No components installed. Run 'craftcn add <component>' to install components."
                .yellow()
        );
    }

    Ok(())
}
