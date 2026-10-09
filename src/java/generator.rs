use anyhow::{Context, Result};
use colored::Colorize;
use minijinja::{context, Environment};
use std::fs;
use std::path::{Path, PathBuf};

use crate::registry::models::Theme;
use crate::utils::project::java_source_root;

const UITHEME_TEMPLATE: &str = include_str!("../../templates/UITheme.java.j2");

/// Renders `UITheme.java` for a theme. The output lives in `<package>.ui`.
pub fn render_theme(theme: &Theme, package: &str) -> Result<String> {
    let mut env = Environment::new();
    env.add_template("UITheme.java", UITHEME_TEMPLATE)
        .context("UITheme.java template is invalid")?;

    let template = env.get_template("UITheme.java")?;
    template
        .render(context! {
            package => format!("{package}.ui"),
            theme => theme,
        })
        .context("Failed to render UITheme.java")
}

/// Writes `UITheme.java` into the project and returns its path.
pub fn write_theme(project_root: &Path, package: &str, theme: &Theme) -> Result<PathBuf> {
    let rendered = render_theme(theme, package)?;
    let output_path = java_source_root(project_root, package)
        .join("ui")
        .join("UITheme.java");

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&output_path, rendered)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    println!(
        "{} {}",
        "✓".green(),
        format!("Created {}", output_path.display()).dimmed()
    );

    Ok(output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn sample_theme() -> Theme {
        let mut sounds = BTreeMap::new();
        sounds.insert("success".to_string(), "ENTITY_PLAYER_LEVELUP".to_string());
        sounds.insert("click".to_string(), "UI_BUTTON_CLICK".to_string());

        Theme {
            name: "test".to_string(),
            description: "test theme".to_string(),
            version: "1.0.0".to_string(),
            author: "tests".to_string(),
            palette: Default::default(),
            assets: crate::registry::models::Assets {
                filler_glass: "GRAY_STAINED_GLASS_PANE".to_string(),
                back_button: "ARROW".to_string(),
                next_button: "ARROW".to_string(),
                error_icon: "BARRIER".to_string(),
                success_icon: "EMERALD".to_string(),
            },
            styles: BTreeMap::new(),
            sounds,
            preview: None,
        }
    }

    #[test]
    fn renders_sound_constants_in_the_ui_package() {
        let rendered = render_theme(&sample_theme(), "com.example.plugin").unwrap();

        assert!(rendered.starts_with("package com.example.plugin.ui;"));
        assert!(
            rendered.contains("public static final Sound SUCCESS = Sound.ENTITY_PLAYER_LEVELUP;")
        );
        assert!(rendered.contains("public static final Sound CLICK = Sound.UI_BUTTON_CLICK;"));
        assert!(rendered.contains("Material.valueOf(\"GRAY_STAINED_GLASS_PANE\")"));
        // valueOrThrow throws NoSuchElementException, which the theme loader never caught, so
        // parsing a hex colour crashed UITheme on load.
        assert!(!rendered.contains("valueOrThrow"));
    }

    #[test]
    fn every_bundled_theme_renders() {
        let registry = Path::new(env!("CARGO_MANIFEST_DIR")).join("registry/themes.json");
        let themes: Vec<Theme> =
            serde_json::from_str(&fs::read_to_string(registry).unwrap()).unwrap();

        assert!(!themes.is_empty());
        for theme in &themes {
            let rendered = render_theme(theme, "com.example.plugin")
                .unwrap_or_else(|e| panic!("theme {} failed: {e:#}", theme.name));
            assert!(rendered.contains(&format!("Theme: {}", theme.name)));
        }
    }
}
