use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fs;

use crate::config::CraftCNConfig;
use crate::java::generator::JavaGenerator;
use crate::registry::client::RegistryClient;

pub async fn handle_init(package: Option<String>, theme_opt: Option<String>) -> Result<()> {
    println!("{}", "CraftCN Initialization".bold().cyan());
    println!("{}", "═".repeat(40).cyan());
    println!();

    let project_root = crate::utils::project::find_project_root()
        .context("Could not find project root. Please run this command in a Minecraft plugin project with pom.xml or build.gradle.")?;

    println!("{} {}", "Project root:".green(), project_root.display());
    println!();

    let package_name = if let Some(pkg) = package {
        pkg
    } else {
        let default = crate::utils::project::detect_package_name(&project_root);

        dialoguer::Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Root package name")
            .default(default.unwrap_or_else(|| "com.example.plugin".to_string()))
            .interact()?
    };

    println!();

    let themes = RegistryClient::get_available_themes().await?;

    let theme_name = if let Some(t) = theme_opt {
        t
    } else {
        let items: Vec<String> = themes.iter().map(|t| t.name.clone()).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a theme")
            .items(&items)
            .default(0)
            .interact()?;

        themes[selection].name.clone()
    };

    println!();

    let theme = themes
        .iter()
        .find(|t| t.name == theme_name)
        .ok_or_else(|| anyhow::anyhow!("Theme '{}' not found", theme_name))?;

    let config = CraftCNConfig {
        version: env!("CARGO_PKG_VERSION").to_string(),
        package: package_name.clone(),
        theme: theme_name.clone(),
        components: Vec::new(),
    };

    let config_path = project_root.join("craftcn.json");
    fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;

    println!("{} {}", "✓".green(), "Created craftcn.json".green());

    let generator = JavaGenerator::new(project_root.clone(), package_name, theme_name.clone());
    generator.generate_theme(theme).await?;
    generator.generate_base_menu().await?;

    println!();
    println!("{}", "✓ CraftCN initialized successfully!".green());
    println!();
    println!("{}", "Next steps:".bold());
    println!(
        "  Run {} to view available components",
        "craftcn list".cyan()
    );
    println!(
        "  Run {} to add a component",
        "craftcn add <component>".cyan()
    );

    Ok(())
}
