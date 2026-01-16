use anyhow::{Context, Result};
use colored::Colorize;
use minijinja::Environment;
use std::fs;
use std::path::PathBuf;

use crate::registry::models::Theme;

pub struct JavaGenerator {
    project_root: PathBuf,
    package: String,
    theme_name: String,
    env: Environment<'static>,
}

impl JavaGenerator {
    pub fn new(project_root: PathBuf, package: String, theme_name: String) -> Self {
        let mut env = Environment::new();
        env.add_function("to_uppercase", |s: String| s.to_uppercase());
        env.add_function("to_lowercase", |s: String| s.to_lowercase());

        Self {
            project_root,
            package,
            theme_name,
            env,
        }
    }

    fn get_package_path(&self, package: &str) -> PathBuf {
        let package_path = package.replace('.', "/");
        self.project_root
            .join("src")
            .join("main")
            .join("java")
            .join(package_path)
    }

    pub async fn generate_theme(&self, theme: &Theme) -> Result<()> {
        let template = include_str!("../../templates/UITheme.java.j2");
        let ui_package = format!("{}.ui", self.package);
        let output_path = self.get_package_path(&ui_package).join("UITheme.java");

        let ctx = minijinja::context! {
            package => ui_package,
            theme => theme,
        };

        let rendered = self
            .env
            .render_str(template, ctx)
            .context("Failed to render UITheme.java")?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, rendered)?;

        println!(
            "{} {}",
            "✓".green(),
            format!("Created {}", output_path.display()).dimmed()
        );

        Ok(())
    }

    pub async fn generate_base_menu(&self) -> Result<()> {
        let template = include_str!("../../templates/BaseMenu.java.j2");
        let ui_core_package = format!("{}.ui.core", self.package);
        let output_path = self
            .get_package_path(&ui_core_package)
            .join("BaseMenu.java");

        let ctx = minijinja::context! {
            package => ui_core_package,
        };

        let rendered = self
            .env
            .render_str(template, ctx)
            .context("Failed to render BaseMenu.java")?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, rendered)?;

        println!(
            "{} {}",
            "✓".green(),
            format!("Created {}", output_path.display()).dimmed()
        );

        Ok(())
    }
}
