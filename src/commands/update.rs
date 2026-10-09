use anyhow::Result;
use colored::Colorize;

use crate::registry::client::{RegistryClient, RegistrySource};

pub async fn handle_update() -> Result<()> {
    let client = RegistryClient::from_env();

    match client.source() {
        RegistrySource::Directory(path) => {
            println!(
                "{}",
                format!(
                    "Using local registry at {} (nothing to refresh).",
                    path.display()
                )
                .yellow()
            );
        }
        RegistrySource::Remote { cache, .. } => {
            println!("{}", "Refreshing the CraftCN registry...".bold().cyan());
            client.refresh().await?;

            println!();
            println!("{}", "✓ Registry refreshed".green());
            if let Some(cache) = cache {
                println!("  {}", format!("Cache: {}", cache.display()).dimmed());
            }
        }
    }

    println!();
    println!(
        "{}",
        "Installed components are not changed. Run 'craftcn add <component> --force' to reinstall one.".dimmed()
    );

    Ok(())
}
