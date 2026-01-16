use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "craftcn")]
#[command(about = "A shadcn/ui for Minecraft Plugins - CLI tool for scaffolding UI primitives", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(short, long)]
        package: Option<String>,

        #[arg(short, long)]
        theme: Option<String>,
    },

    Add {
        component: String,

        #[arg(short, long)]
        force: bool,
    },

    Context {
        component: String,

        #[arg(short, long)]
        verbose: bool,
    },

    List {
        #[arg(short, long)]
        category: Option<String>,

        #[arg(short, long)]
        installed: bool,
    },

    Update {
        #[arg(short, long)]
        force: bool,
    },

    Theme {
        #[command(subcommand)]
        action: ThemeCommands,
    },

    /// Show contribution guide and instructions
    Contribute,
}

#[derive(Subcommand)]
pub enum ThemeCommands {
    List,

    Info { name: String },

    Apply { name: String },
}

#[derive(Subcommand)]
pub enum Contribute {}

#[derive(Subcommand)]
pub enum ThemeCommands {
    List,

    Info { name: String },

    Apply { name: String },
}
