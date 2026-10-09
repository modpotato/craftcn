use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub version: String,
    pub components: Vec<Component>,
}

impl Registry {
    pub fn get_component(&self, name: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.name == name)
    }

    /// Components grouped by category letter. Categories are derived from the
    /// component list, so `index.json` has a single source of truth.
    pub fn by_category(&self) -> BTreeMap<String, Vec<&Component>> {
        let mut grouped: BTreeMap<String, Vec<&Component>> = BTreeMap::new();
        for component in &self.components {
            grouped
                .entry(component.category.clone())
                .or_default()
                .push(component);
        }
        for components in grouped.values_mut() {
            components.sort_by(|a, b| a.name.cmp(&b.name));
        }
        grouped
    }
}

/// Human readable title for a category letter.
pub fn category_title(category: &str) -> &'static str {
    match category {
        "A" => "Inventory GUIs",
        "B" => "Chat Widgets",
        "C" => "HUD & Visuals",
        "D" => "Utilities",
        "E" => "Resource Pack GUIs",
        _ => "Other",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub files: Vec<ComponentFile>,
    /// Lowest Minecraft version whose APIs the component needs, e.g. `1.21.6`.
    #[serde(default)]
    pub minecraft: Option<String>,
    /// True when the component draws its visuals from the generated resource pack
    /// (`craftcn pack`). `craftcn doctor` checks that the pack exists.
    #[serde(default)]
    pub resource_pack: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentFile {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    #[serde(default)]
    pub palette: Palette,
    #[serde(default)]
    pub assets: Assets,
    #[serde(default)]
    pub styles: BTreeMap<String, String>,
    #[serde(default)]
    pub sounds: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Palette {
    #[serde(default)]
    pub primary: String,
    #[serde(default)]
    pub secondary: String,
    #[serde(default)]
    pub accent: String,
    #[serde(default)]
    pub destructive: String,
    #[serde(default)]
    pub muted: String,
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    pub foreground: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Assets {
    #[serde(default)]
    pub filler_glass: String,
    #[serde(default)]
    pub back_button: String,
    #[serde(default)]
    pub next_button: String,
    #[serde(default)]
    pub error_icon: String,
    #[serde(default)]
    pub success_icon: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::minecraft;
    use crate::registry::client::safe_relative_path;
    use std::path::Path;

    fn bundled_registry() -> Registry {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("registry/index.json"),
        )
        .expect("registry/index.json should exist");
        serde_json::from_str(&text).expect("registry/index.json should parse")
    }

    #[test]
    fn registry_components_are_consistent() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("registry/components");
        let registry = bundled_registry();

        let mut names = std::collections::HashSet::new();
        for component in &registry.components {
            assert!(
                names.insert(component.name.clone()),
                "duplicate component {}",
                component.name
            );
            assert!(
                ["A", "B", "C", "D", "E"].contains(&component.category.as_str()),
                "{} has unknown category {}",
                component.name,
                component.category
            );
            if let Some(minimum) = &component.minecraft {
                assert!(
                    minecraft::parse(minimum).is_some(),
                    "{} has bad minecraft version {minimum}",
                    component.name
                );
            }
            assert!(
                !component.files.is_empty(),
                "{} has no files",
                component.name
            );

            for file in &component.files {
                let relative =
                    safe_relative_path(&file.path).expect("registry path should be safe");
                let path = root.join(&component.name).join(relative);
                let source = std::fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("missing source file {}", path.display()));
                assert!(
                    source.contains("package com.craftcn."),
                    "{} must be declared under com.craftcn",
                    path.display()
                );
            }
        }

        for component in &registry.components {
            for dependency in &component.dependencies {
                assert!(
                    registry.get_component(dependency).is_some(),
                    "{} depends on unknown component {dependency}",
                    component.name
                );
            }
        }
    }

    #[test]
    fn theme_assets_are_plain_constant_names() {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("registry/themes.json"),
        )
        .expect("themes.json should exist");
        let themes: Vec<Theme> = serde_json::from_str(&text).expect("themes.json should parse");

        let constant = regex::Regex::new(r"^[A-Z][A-Z0-9_]*$").unwrap();
        for theme in &themes {
            let assets = [
                &theme.assets.filler_glass,
                &theme.assets.back_button,
                &theme.assets.next_button,
                &theme.assets.error_icon,
                &theme.assets.success_icon,
            ];
            for value in assets.into_iter().chain(theme.sounds.values()) {
                assert!(
                    constant.is_match(value),
                    "theme {} has invalid constant name {value}",
                    theme.name
                );
            }
        }
    }
}
