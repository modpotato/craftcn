mod cli;
mod commands;
mod config;
mod installer;
mod java;
mod minecraft;
mod pack;
mod registry;
mod utils;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use colored::Colorize;
use commands::run_command;

#[tokio::main]
async fn main() -> Result<()> {
    // Warnings and errors only by default; command output is printed directly.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .without_time()
        .init();

    let cli = Cli::parse();

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
