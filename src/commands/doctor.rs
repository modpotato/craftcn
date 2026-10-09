use anyhow::{bail, Result};
use colored::Colorize;

use crate::config::CraftCNConfig;
use crate::installer::Project;
use crate::minecraft::{self, MIN_SUPPORTED};
use crate::registry::client::RegistryClient;
use crate::utils::project::{detect_minecraft_version, find_project_root, java_source_root};

#[derive(Default)]
struct Report {
    errors: usize,
    warnings: usize,
}

impl Report {
    fn ok(&self, message: impl AsRef<str>) {
        println!("  {} {}", "✓".green(), message.as_ref());
    }

    fn warn(&mut self, message: impl AsRef<str>) {
        self.warnings += 1;
        println!("  {} {}", "⚠".yellow(), message.as_ref());
    }

    fn error(&mut self, message: impl AsRef<str>) {
        self.errors += 1;
        println!("  {} {}", "✗".red(), message.as_ref());
    }
}

pub async fn handle_doctor() -> Result<()> {
    let root = find_project_root()?;
    let config = CraftCNConfig::load(&root)?;
    let project = Project {
        root: root.clone(),
        config: config.clone(),
    };

    let client = RegistryClient::from_env();
    let registry = client.index().await?;
    let themes = client.themes().await?;

    let mut report = Report::default();

    println!("{}", "Project".bold());
    report.ok(format!(
        "craftcn.json: package {}, theme {}, Minecraft {}",
        config.package, config.theme, config.minecraft
    ));

    if themes.iter().any(|t| t.name == config.theme) {
        report.ok(format!("theme '{}' is in the registry", config.theme));
    } else {
        report.error(format!("theme '{}' is not in the registry", config.theme));
    }

    let ui_theme = java_source_root(&root, &config.package)
        .join("ui")
        .join("UITheme.java");
    if ui_theme.exists() {
        report.ok("ui/UITheme.java is present");
    } else {
        report.error(
            "ui/UITheme.java is missing. Run 'craftcn theme apply <theme>' to regenerate it.",
        );
    }

    match detect_minecraft_version(&root) {
        Some(detected) if detected == config.minecraft => {
            report.ok(format!("build file targets paper-api {detected}"));
        }
        Some(detected) => report.warn(format!(
            "build file targets paper-api {detected}, but craftcn.json says {}. Update the \"minecraft\" field to match.",
            config.minecraft
        )),
        None => report.warn("no paper-api version found in the build file, so the Minecraft target could not be checked"),
    }

    if !minecraft::is_at_least(&config.minecraft, MIN_SUPPORTED) {
        report.warn(format!(
            "the bundled components need Minecraft {MIN_SUPPORTED}+, but the project targets {}",
            config.minecraft
        ));
    }

    println!();
    println!("{}", "Components".bold());

    if config.components.is_empty() {
        report.ok("no components installed");
    }

    for name in &config.components {
        let Some(component) = registry.get_component(name) else {
            report.warn(format!(
                "{name} is not in the registry. Run 'craftcn update', or remove it with 'craftcn remove {name}'."
            ));
            continue;
        };

        let missing: Vec<&str> = component
            .files
            .iter()
            .filter(|file| !project.source_root().join(&file.path).exists())
            .map(|file| file.path.as_str())
            .collect();

        if missing.is_empty() {
            report.ok(format!("{name}: {} file(s) present", component.files.len()));
        } else {
            report.error(format!(
                "{name}: missing {}. Reinstall with 'craftcn add {name} --force'.",
                missing.join(", ")
            ));
        }

        for dependency in &component.dependencies {
            if !config.components.contains(dependency) {
                report.error(format!("{name} needs {dependency}, which is not installed"));
            }
        }

        if let Some(minimum) = &component.minecraft {
            if !minecraft::is_at_least(&config.minecraft, minimum) {
                report.warn(format!(
                    "{name} needs Minecraft {minimum}+, but the project targets {}",
                    config.minecraft
                ));
            }
        }

        if component.resource_pack && !root.join("resourcepack").join("pack.mcmeta").exists() {
            report.warn(format!(
                "{name} draws from the resource pack, which has not been generated. Run 'craftcn pack'."
            ));
        }
    }

    println!();
    if report.errors == 0 && report.warnings == 0 {
        println!("{}", "No problems found.".green().bold());
        return Ok(());
    }

    println!(
        "{} error(s), {} warning(s)",
        report.errors.to_string().red().bold(),
        report.warnings.to_string().yellow().bold()
    );

    if report.errors > 0 {
        bail!("craftcn doctor found {} error(s)", report.errors);
    }

    Ok(())
}
