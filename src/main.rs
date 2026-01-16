mod cli;
mod commands;
mod config;
mod java;
mod registry;
mod utils;

use anyhow::Result;
use clap::Parser;
use tracing::info;

use cli::{Cli, Commands};
use colored::Colorize;
use commands::run_command;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    info!("CraftCN v{} starting", env!("CARGO_PKG_VERSION"));

    if matches!(cli.command, Commands::Init { .. }) {
        println!();
        println!("{} Welcome to CraftCN!", "═".repeat(40).cyan());
        println!(
            "{} A \"shadcn/ui\" for Minecraft Plugins",
            "─".repeat(40).cyan()
        );
        println!();
        println!(
            "{} Run {} to initialize a project",
            "│".cyan(),
            "craftcn init".bold()
        );
        println!(
            "{} Run {} to list available components",
            "│".cyan(),
            "craftcn list".bold()
        );
        println!(
            "{} Run {} to get help and options",
            "│".cyan(),
            "craftcn --help".bold()
        );
        println!(
            "{} Run {} to see how to contribute",
            "│".cyan(),
            "craftcn contribute".bold()
        );
        println!(
            "{} Run {} to open contribution guide",
            "│".cyan(),
            "craftcn --help".bold()
        );
        println!("{}", "─".repeat(40).cyan());
        println!();
    }

    run_command(cli).await?;

    Ok(())
}
