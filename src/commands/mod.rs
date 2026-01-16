pub mod add;
pub mod context;
pub mod contribute;
pub mod init;
pub mod list;
pub mod theme;
pub mod update;

use anyhow::Result;

use crate::cli::{Cli, Commands};
use crate::commands::add::handle_add;
use crate::commands::context::handle_context;
use crate::commands::contribute::handle_contribute;
use crate::commands::init::handle_init;
use crate::commands::list::handle_list;
use crate::commands::theme::handle_theme;
use crate::commands::update::handle_update;

pub async fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { package, theme } => handle_init(package, theme).await,
        Commands::Add { component, force } => handle_add(component, force).await,
        Commands::Context { component, verbose } => handle_context(component, verbose).await,
        Commands::List {
            category,
            installed,
        } => handle_list(category, installed).await,
        Commands::Update { force } => handle_update(force).await,
        Commands::Theme { action } => handle_theme(action).await,
        Commands::Contribute => handle_contribute().await,
    }
}
