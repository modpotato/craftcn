use anyhow::Result;
use colored::Colorize;

use crate::registry::client::RegistryClient;

pub async fn handle_theme(action: crate::cli::ThemeCommands) -> Result<()> {
    match action {
        crate::cli::ThemeCommands::List => handle_list_themes().await,
        crate::cli::ThemeCommands::Info { name } => handle_theme_info(name).await,
        crate::cli::ThemeCommands::Apply { name } => handle_apply_theme(name).await,
    }
}

async fn handle_list_themes() -> Result<()> {
    let themes = RegistryClient::get_available_themes().await?;

    println!("{}", "Available Themes".bold().cyan());
    println!("{}", "═".repeat(40).cyan());
    println!();

    for theme in &themes {
        println!(
            "{} {}",
            theme.name.cyan().bold(),
            theme.description.dimmed()
        );
        println!("  {}", format!("Author: {}", theme.author).dimmed());
        println!();
    }

    Ok(())
}

async fn handle_theme_info(name: String) -> Result<()> {
    let themes = RegistryClient::get_available_themes().await?;
    let theme = themes
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| anyhow::anyhow!("Theme '{}' not found", name))?;

    println!("{}", format!("Theme: {}", theme.name).bold().cyan());
    println!("{}", "─".repeat(40).cyan());
    println!();
    println!("{}", theme.description);
    println!();
    println!("Author: {}", theme.author);
    println!("Version: {}", theme.version);

    if let Some(preview) = &theme.preview {
        println!();
        println!("Preview:");
        for color in preview {
            println!("  {}", color);
        }
    }

    Ok(())
}

async fn handle_apply_theme(name: String) -> Result<()> {
    let project_root = crate::utils::project::find_project_root()?;
    let config_path = project_root.join("craftcn.json");

    if !config_path.exists() {
        anyhow::bail!("CraftCN not initialized. Run 'craftcn init' first.");
    }

    let mut config: crate::config::CraftCNConfig =
        serde_json::from_str(&std::fs::read_to_string(&config_path)?)?;

    let themes = RegistryClient::get_available_themes().await?;
    if !themes.iter().any(|t| t.name == name) {
        anyhow::bail!("Theme '{}' not found", name);
    }

    config.theme = name.clone();

    std::fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;

    println!();
    println!(
        "{} {} {}",
        "✓".green(),
        "Theme updated to".green(),
        name.cyan()
    );
    println!();
    println!(
        "{}",
        "Note: You need to regenerate your UITheme.java for this to take effect.".yellow()
    );
    println!(
        "  {}",
        "Delete and recreate your project to apply the new theme.".dimmed()
    );

    Ok(())
}
