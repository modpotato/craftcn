pub mod add;
pub mod context;
pub mod contribute;
pub mod doctor;
pub mod init;
pub mod list;
pub mod pack;
pub mod remove;
pub mod theme;
pub mod update;

use anyhow::Result;

use crate::cli::{Cli, Commands};
use crate::commands::add::handle_add;
use crate::commands::context::handle_context;
use crate::commands::contribute::handle_contribute;
use crate::commands::doctor::handle_doctor;
use crate::commands::init::handle_init;
use crate::commands::list::handle_list;
use crate::commands::pack::handle_pack;
use crate::commands::remove::handle_remove;
use crate::commands::theme::handle_theme;
use crate::commands::update::handle_update;

pub async fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init {
            package,
            theme,
            minecraft,
            yes,
        } => handle_init(package, theme, minecraft, yes).await,
        Commands::Add { component, force } => handle_add(component, force).await,
        Commands::Remove { component, force } => handle_remove(component, force).await,
        Commands::Context { component, verbose } => handle_context(component, verbose).await,
        Commands::List {
            category,
            installed,
        } => handle_list(category, installed).await,
        Commands::Update => handle_update().await,
        Commands::Theme { action } => handle_theme(action).await,
        Commands::Pack { out, zip } => handle_pack(out, zip).await,
        Commands::Doctor => handle_doctor().await,
        Commands::Contribute => handle_contribute().await,
    }
}
