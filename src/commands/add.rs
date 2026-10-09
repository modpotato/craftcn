use anyhow::Result;
use colored::Colorize;

use crate::installer::{add_component, Project};
use crate::registry::client::RegistryClient;

pub async fn handle_add(component_name: String, force: bool) -> Result<()> {
    let mut project = Project::load()?;

    if project.is_installed(&component_name) && !force {
        println!(
            "{} {} is already installed. Use {} to reinstall it.",
            "✓".green(),
            component_name.bold(),
            "--force".cyan()
        );
        return Ok(());
    }

    let client = RegistryClient::from_env();
    let registry = client.index().await?;

    println!(
        "{}",
        format!("Adding component: {component_name}").bold().cyan()
    );
    println!();

    add_component(&mut project, &client, &registry, &component_name, force).await?;

    println!();
    println!("{}", "✓ Components installed successfully!".green());

    Ok(())
}
