use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

use crate::registry::client::RegistryClient;

pub async fn handle_update(force: bool) -> Result<()> {
    let project_root = crate::utils::project::find_project_root()?;
    let config_path = project_root.join("craftcn.json");

    if !config_path.exists() {
        anyhow::bail!("CraftCN not initialized. Run 'craftcn init' first.");
    }

    println!("{}", "Updating CraftCN registry...".bold().cyan());

    let spinner_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {msg}")?
        .progress_chars("=>-");

    let pb = ProgressBar::new_spinner();
    pb.set_style(spinner_style);
    pb.set_message("Fetching latest registry...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    RegistryClient::update_cache().await?;

    pb.finish_with_message("Registry updated!");

    println!();
    println!("{}", "✓ Registry updated successfully!".green());
    println!();

    if force {
        println!(
            "{}",
            "To update installed components, you may need to remove and re-add them:".yellow()
        );
        println!(
            "  {} removes a component",
            "craftcn remove <component>".cyan()
        );
        println!("  {} adds a component", "craftcn add <component>".cyan());
    }

    Ok(())
}
