use anyhow::{bail, Context, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::io::IsTerminal;

use crate::config::CraftCNConfig;
use crate::installer::{add_component, Project};
use crate::java::generator::write_theme;
use crate::minecraft;
use crate::registry::client::RegistryClient;
use crate::utils::project::{detect_minecraft_version, detect_package_name, find_project_root};

/// Components every project starts with: the menu base class everything else builds on.
const FOUNDATION_COMPONENT: &str = "base-menu";

pub async fn handle_init(
    package: Option<String>,
    theme_opt: Option<String>,
    minecraft_opt: Option<String>,
    yes: bool,
) -> Result<()> {
    println!("{}", "CraftCN Initialization".bold().cyan());
    println!("{}", "═".repeat(40).cyan());
    println!();

    let project_root = find_project_root().context(
        "Could not find project root. Run this command in a Paper plugin project with pom.xml or build.gradle[.kts].",
    )?;

    if project_root.join("craftcn.json").exists() {
        bail!("CraftCN is already initialized here. Use 'craftcn theme apply <theme>' to change the theme.");
    }

    println!("{} {}", "Project root:".green(), project_root.display());
    println!();

    let interactive = !yes && std::io::stdin().is_terminal();
    let theme = ColorfulTheme::default();

    let detected_package =
        detect_package_name(&project_root).unwrap_or_else(|| "com.example.plugin".to_string());
    let package_name = match package {
        Some(pkg) => pkg,
        None if interactive => Input::with_theme(&theme)
            .with_prompt("Root package name")
            .default(detected_package)
            .interact_text()?,
        None => detected_package,
    };
    validate_package(&package_name)?;

    let detected_minecraft = detect_minecraft_version(&project_root)
        .unwrap_or_else(|| minecraft::DEFAULT_TARGET.to_string());
    let minecraft_version = match minecraft_opt {
        Some(version) => version,
        None if interactive => Input::with_theme(&theme)
            .with_prompt("Target Minecraft version")
            .default(detected_minecraft)
            .interact_text()?,
        None => detected_minecraft,
    };
    if minecraft::parse(&minecraft_version).is_none() {
        bail!("'{minecraft_version}' is not a Minecraft version (expected e.g. 26.2 or 1.21.4)");
    }

    println!();

    let client = RegistryClient::from_env();
    let themes = client.themes().await?;

    let theme_name = match theme_opt {
        Some(name) => name,
        None if interactive => {
            let items: Vec<String> = themes
                .iter()
                .map(|t| format!("{} — {}", t.name, t.description))
                .collect();
            let selection = Select::with_theme(&theme)
                .with_prompt("Select a theme")
                .items(&items)
                .default(0)
                .interact()?;
            themes[selection].name.clone()
        }
        None => themes
            .first()
            .map(|t| t.name.clone())
            .context("the registry has no themes")?,
    };

    let theme_data = themes
        .iter()
        .find(|t| t.name == theme_name)
        .ok_or_else(|| anyhow::anyhow!("Theme '{theme_name}' not found"))?;

    let config = CraftCNConfig {
        version: env!("CARGO_PKG_VERSION").to_string(),
        package: package_name.clone(),
        theme: theme_name.clone(),
        minecraft: minecraft_version.clone(),
        components: Vec::new(),
    };
    config.save(&project_root)?;
    println!("{} {}", "✓".green(), "Created craftcn.json".green());

    write_theme(&project_root, &package_name, theme_data)?;

    let mut project = Project {
        root: project_root,
        config,
    };
    let registry = client.index().await?;
    add_component(
        &mut project,
        &client,
        &registry,
        FOUNDATION_COMPONENT,
        false,
    )
    .await?;

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
    println!("  Run {} to check the project", "craftcn doctor".cyan());

    Ok(())
}

fn validate_package(package: &str) -> Result<()> {
    let valid = package.split('.').all(|segment| {
        let mut chars = segment.chars();
        matches!(chars.next(), Some(c) if c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    });

    if !valid {
        bail!("'{package}' is not a valid Java package (use lowercase segments such as com.example.plugin)");
    }
    Ok(())
}
