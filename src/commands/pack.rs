use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

use crate::config::CraftCNConfig;
use crate::minecraft::MIN_SUPPORTED;
use crate::pack;
use crate::registry::client::RegistryClient;
use crate::utils::project::find_project_root;

pub async fn handle_pack(out: Option<PathBuf>, zip: bool) -> Result<()> {
    let root = find_project_root()?;
    let config = CraftCNConfig::load(&root)?;

    let themes = RegistryClient::from_env().themes().await?;
    let theme = themes
        .iter()
        .find(|t| t.name == config.theme)
        .with_context(|| format!("theme '{}' is not in the registry", config.theme))?;

    let files = pack::build_files(theme, MIN_SUPPORTED, &config.minecraft)?;

    let dir = match out {
        Some(path) if path.is_absolute() => path,
        Some(path) => root.join(path),
        None => root.join("resourcepack"),
    };

    pack::write_tree(&dir, &files)?;
    println!(
        "{} {}",
        "✓".green(),
        format!("Wrote {} pack files to {}", files.len(), dir.display()).dimmed()
    );

    if zip {
        let archive = root.join("build").join("craftcn-pack.zip");
        pack::write_zip(&archive, &files)?;

        let bytes = fs::read(&archive)?;
        println!(
            "{} {}",
            "✓".green(),
            format!("Wrote {} ({} bytes)", archive.display(), bytes.len()).dimmed()
        );
        println!();
        println!("  {} {}", "SHA-1:".bold(), pack::sha1_hex(&bytes).cyan());
        println!();
        println!(
            "Upload the zip somewhere players can download it, then pass its URL and SHA-1 to ResourcePackService."
        );
    }

    println!();
    println!(
        "{}",
        format!(
            "Pack format range {} → {} (Minecraft {MIN_SUPPORTED} to {}).",
            crate::minecraft::pack_format(MIN_SUPPORTED).unwrap_or_default(),
            crate::minecraft::pack_format(&config.minecraft).unwrap_or_default(),
            config.minecraft
        )
        .dimmed()
    );

    Ok(())
}
