//! Generates the CraftCN resource pack: GUI font, title plates, item icons and pack metadata.

pub mod art;
pub mod font;

use anyhow::{Context, Result};
use image::{ImageFormat, RgbaImage};
use serde_json::json;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use crate::minecraft;
use crate::registry::models::{Palette, Theme};

/// One file of the pack, addressed by its path inside the pack root.
#[derive(Debug, Clone)]
pub struct PackFile {
    pub path: String,
    pub bytes: Vec<u8>,
}

/// Palette colors used for the pack, with the same fallbacks as the theme defaults.
pub struct PackColors {
    pub primary: art::Color,
    pub accent: art::Color,
    pub danger: art::Color,
    pub foreground: art::Color,
    pub base: art::Color,
}

impl PackColors {
    pub fn from_palette(palette: &Palette) -> Self {
        let parse = |hex: &str, fallback: &str| {
            art::parse_hex(hex, art::parse_hex(fallback, art::TRANSPARENT))
        };
        Self {
            primary: parse(&palette.primary, "#FFD700"),
            accent: parse(&palette.accent, "#00CED1"),
            danger: parse(&palette.destructive, "#FF4444"),
            foreground: parse(&palette.foreground, "#FFFFFF"),
            base: parse(&palette.background, "#2C2C2C"),
        }
    }
}

/// Builds every file of the pack for a theme.
///
/// `min_minecraft` and `target_minecraft` set the `pack_format` range. The pack works on every
/// version in that range, so it can serve servers on several releases at once.
pub fn build_files(
    theme: &Theme,
    min_minecraft: &str,
    target_minecraft: &str,
) -> Result<Vec<PackFile>> {
    let min_format = minecraft::pack_format(min_minecraft).with_context(|| {
        format!("resource packs need Minecraft 1.21.4 or newer, got {min_minecraft}")
    })?;
    let max_format = minecraft::pack_format(target_minecraft)
        .with_context(|| format!("unknown pack format for Minecraft {target_minecraft}"))?;

    let colors = PackColors::from_palette(&theme.palette);
    let ns = font::NAMESPACE;
    let mut files = Vec::new();

    let mcmeta = json!({
        "pack": {
            "description": format!("CraftCN GUI pack ({})", theme.name),
            "min_format": min_format,
            "max_format": max_format,
        }
    });
    files.push(text_file(
        "pack.mcmeta",
        serde_json::to_string_pretty(&mcmeta)?,
    ));
    files.push(png_file(
        "pack.png",
        &art::pack_icon(colors.base, colors.primary, colors.foreground),
    )?);

    files.push(text_file(
        &format!("assets/{ns}/font/{}.json", font::FONT_FILE),
        serde_json::to_string_pretty(&font::font_json())?,
    ));

    for glyph in &font::GLYPHS {
        let color = match glyph.texture {
            "gui/plate_accent" => colors.accent,
            "gui/plate_danger" => colors.danger,
            "gui/plate_compact" => colors.accent,
            _ => colors.primary,
        };
        let img = art::plate(
            glyph.width,
            glyph.height,
            color,
            art::darken(colors.foreground, 0.1),
        );
        files.push(png_file(
            &format!("assets/{ns}/textures/{}.png", glyph.texture),
            &img,
        )?);
    }

    let icons: [(&str, RgbaImage); 7] = [
        ("icon_next", art::chevron(false, colors.primary)),
        ("icon_back", art::chevron(true, colors.primary)),
        ("icon_confirm", art::check(colors.accent)),
        ("icon_cancel", art::cross(colors.danger)),
        ("icon_info", art::info(colors.accent)),
        ("icon_close", art::close(colors.danger)),
        ("icon_filler", art::filler(16, colors.base, colors.primary)),
    ];
    debug_assert_eq!(icons.len(), font::ICONS.len());

    for (name, img) in &icons {
        files.push(png_file(
            &format!("assets/{ns}/textures/item/{name}.png"),
            img,
        )?);
        files.push(text_file(
            &format!("assets/{ns}/items/{name}.json"),
            serde_json::to_string_pretty(&json!({
                "model": { "type": "minecraft:model", "model": format!("{ns}:item/{name}") }
            }))?,
        ));
        files.push(text_file(
            &format!("assets/{ns}/models/item/{name}.json"),
            serde_json::to_string_pretty(&json!({
                "parent": "minecraft:item/generated",
                "textures": { "layer0": format!("{ns}:item/{name}") }
            }))?,
        ));
    }

    Ok(files)
}

fn text_file(path: &str, mut content: String) -> PackFile {
    if !content.ends_with('\n') {
        content.push('\n');
    }
    PackFile {
        path: path.to_string(),
        bytes: content.into_bytes(),
    }
}

fn png_file(path: &str, img: &RgbaImage) -> Result<PackFile> {
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png)
        .with_context(|| format!("failed to encode {path}"))?;
    Ok(PackFile {
        path: path.to_string(),
        bytes: buffer.into_inner(),
    })
}

/// Writes the pack as a directory tree. Returns the written paths.
pub fn write_tree(root: &Path, files: &[PackFile]) -> Result<Vec<PathBuf>> {
    let mut written = Vec::with_capacity(files.len());
    for file in files {
        let path = root.join(&file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&path, &file.bytes)
            .with_context(|| format!("failed to write {}", path.display()))?;
        written.push(path);
    }
    Ok(written)
}

/// Writes the pack as a zip archive, the form Paper and clients download.
pub fn write_zip(path: &Path, files: &[PackFile]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let file =
        fs::File::create(path).with_context(|| format!("failed to create {}", path.display()))?;
    let mut zip = zip::ZipWriter::new(file);
    // A fixed timestamp keeps the archive byte-identical between runs, so its SHA-1 only
    // changes when the pack content does.
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .last_modified_time(zip::DateTime::default());

    for entry in files {
        zip.start_file(entry.path.as_str(), options)?;
        zip.write_all(&entry.bytes)?;
    }
    zip.finish()?;
    Ok(())
}

/// Lowercase hex SHA-1, the hash Paper and clients verify resource packs against.
pub fn sha1_hex(bytes: &[u8]) -> String {
    Sha1::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::models::Assets;
    use std::collections::BTreeMap;

    fn theme() -> Theme {
        Theme {
            name: "default".to_string(),
            description: "test".to_string(),
            version: "1.0.0".to_string(),
            author: "tests".to_string(),
            palette: Palette {
                primary: "#FFD700".to_string(),
                secondary: "#A0A0A0".to_string(),
                accent: "#00CED1".to_string(),
                destructive: "#FF4444".to_string(),
                muted: "#808080".to_string(),
                background: "#2C2C2C".to_string(),
                foreground: "#FFFFFF".to_string(),
            },
            assets: Assets::default(),
            styles: BTreeMap::new(),
            sounds: BTreeMap::new(),
            preview: None,
        }
    }

    #[test]
    fn sha1_matches_known_vector() {
        assert_eq!(sha1_hex(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn builds_a_complete_pack() {
        let files = build_files(&theme(), "1.21.4", "26.2").unwrap();
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();

        assert!(paths.contains(&"pack.mcmeta"));
        assert!(paths.contains(&"pack.png"));
        assert!(paths.contains(&"assets/craftcn/font/gui.json"));
        assert!(paths.contains(&"assets/craftcn/textures/gui/plate_primary.png"));
        assert!(paths.contains(&"assets/craftcn/items/icon_next.json"));
        assert!(paths.contains(&"assets/craftcn/models/item/icon_next.json"));

        for icon in font::ICONS {
            assert!(
                paths
                    .iter()
                    .any(|p| p.ends_with(&format!("textures/item/{icon}.png"))),
                "{icon}"
            );
        }
    }

    #[test]
    fn pack_mcmeta_uses_format_range() {
        let files = build_files(&theme(), "1.21.4", "26.2").unwrap();
        let mcmeta = files.iter().find(|f| f.path == "pack.mcmeta").unwrap();
        let parsed: serde_json::Value = serde_json::from_slice(&mcmeta.bytes).unwrap();

        assert_eq!(parsed["pack"]["min_format"], 61);
        assert_eq!(parsed["pack"]["max_format"], 107);
    }

    #[test]
    fn rejects_versions_without_item_definitions() {
        assert!(build_files(&theme(), "1.20.6", "26.2").is_err());
    }

    #[test]
    fn zip_round_trips_every_file() {
        let temp = tempfile::tempdir().unwrap();
        let files = build_files(&theme(), "1.21.4", "26.2").unwrap();
        let zip_path = temp.path().join("craftcn-pack.zip");

        write_zip(&zip_path, &files).unwrap();

        let archive = zip::ZipArchive::new(fs::File::open(&zip_path).unwrap()).unwrap();
        assert_eq!(archive.len(), files.len());
    }
}
