use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::cli::ThemeCommands;
use crate::installer::Project;
use crate::java::generator::write_theme;
use crate::registry::client::RegistryClient;

pub async fn handle_theme(action: ThemeCommands) -> Result<()> {
    match action {
        ThemeCommands::List => handle_list_themes().await,
        ThemeCommands::Info { name } => handle_theme_info(name).await,
        ThemeCommands::Apply { name } => handle_apply_theme(name).await,
    }
}

async fn handle_list_themes() -> Result<()> {
    let themes = RegistryClient::from_env().themes().await?;

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
    let themes = RegistryClient::from_env().themes().await?;
    let theme = themes
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| anyhow!("Theme '{}' not found", name))?;

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
    let mut project = Project::load()?;

    let themes = RegistryClient::from_env().themes().await?;
    let theme = themes
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| anyhow!("Theme '{}' not found", name))?;

    project.config.theme = name.clone();
    project.save()?;

    write_theme(&project.root, &project.config.package, theme)?;

    println!();
    println!("{} {} {}", "✓".green(), "Theme set to".green(), name.cyan());
    println!(
        "{}",
        "Regenerated UITheme.java. Run 'craftcn pack' to rebuild the matching resource pack."
            .dimmed()
    );

    Ok(())
}
