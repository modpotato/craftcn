use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    /// Set up CraftCN in a Paper plugin project
    Init {
        /// Root package, e.g. com.example.plugin (detected from the build file when omitted)
        #[arg(short, long)]
        package: Option<String>,

        /// Theme name (see `craftcn theme list`)
        #[arg(short, long)]
        theme: Option<String>,

        /// Target Minecraft version, e.g. 26.2 (detected from paper-api when omitted)
        #[arg(short, long)]
        minecraft: Option<String>,

        /// Accept detected defaults without prompting
        #[arg(short, long)]
        yes: bool,
    },

    /// Add a component and its dependencies to the project
    Add {
        component: String,

        /// Overwrite files that already exist and reinstall the component
        #[arg(short, long)]
        force: bool,
    },

    /// Remove an installed component
    Remove {
        component: String,

        /// Remove even if other installed components depend on it
        #[arg(short, long)]
        force: bool,
    },

    /// Print a token-efficient API summary of a component for LLM context
    Context {
        component: String,

        /// Include private members and usage hints
        #[arg(short, long)]
        verbose: bool,
    },

    /// List available components
    List {
        /// Show only one category: A, B, C, D or E
        #[arg(short, long)]
        category: Option<String>,

        /// Show only components installed in this project
        #[arg(short, long)]
        installed: bool,
    },

    /// Refresh the cached registry index and themes
    Update,

    /// Browse and apply themes
    Theme {
        #[command(subcommand)]
        action: ThemeCommands,
    },

    /// Generate the resource pack for textured GUIs (title plates, icons, filler)
    Pack {
        /// Output directory (default: resourcepack/)
        #[arg(short, long)]
        out: Option<PathBuf>,

        /// Also write build/craftcn-pack.zip and print its SHA-1 for ResourcePackService
        #[arg(short, long)]
        zip: bool,
    },

    /// Check the project for missing files, version mismatches and missing dependencies
    Doctor,

    /// Show contribution guide and instructions
    Contribute,
}

#[derive(Subcommand)]
pub enum ThemeCommands {
    /// List available themes
    List,

    /// Show details of a theme
    Info { name: String },

    /// Switch the project to a theme and regenerate UITheme.java
    Apply { name: String },
}
