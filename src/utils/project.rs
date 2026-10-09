use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::minecraft;

const BUILD_FILES: [&str; 3] = ["pom.xml", "build.gradle", "build.gradle.kts"];

pub fn find_project_root() -> Result<PathBuf> {
    let current = std::env::current_dir()?;

    for path in current.ancestors() {
        if has_build_file(path) {
            return Ok(path.to_path_buf());
        }
    }

    anyhow::bail!("Could not find project root (pom.xml or build.gradle[.kts] not found)")
}

fn has_build_file(path: &Path) -> bool {
    BUILD_FILES.iter().any(|file| path.join(file).exists())
}

/// `src/main/java/<package as directories>`, where generated and installed sources go.
pub fn java_source_root(project_root: &Path, package: &str) -> PathBuf {
    project_root
        .join("src")
        .join("main")
        .join("java")
        .join(package.replace('.', "/"))
}

/// Reads the first build file that exists, for detection helpers.
fn read_build_files(project_root: &Path) -> Vec<String> {
    BUILD_FILES
        .iter()
        .filter_map(|file| fs::read_to_string(project_root.join(file)).ok())
        .collect()
}

/// Guesses the root package from Maven `groupId` + `artifactId` or Gradle `group` + name.
/// The artifact is sanitized so `my-plugin` becomes the valid package segment `my_plugin`.
pub fn detect_package_name(project_root: &Path) -> Option<String> {
    let pom = fs::read_to_string(project_root.join("pom.xml")).ok();
    if let Some(content) = pom {
        let group = Regex::new(r"<groupId>([^<]+)</groupId>")
            .ok()?
            .captures(&content);
        let artifact = Regex::new(r"<artifactId>([^<]+)</artifactId>")
            .ok()?
            .captures(&content);
        if let (Some(group), Some(artifact)) = (group, artifact) {
            return Some(join_package(&group[1], &artifact[1]));
        }
    }

    let gradle = read_build_files(project_root)
        .into_iter()
        .find(|content| content.contains("group"));
    if let Some(content) = gradle {
        let group = Regex::new(r#"group\s*=?\s*['"]([^'"]+)['"]"#)
            .ok()?
            .captures(&content);
        let artifact =
            Regex::new(r#"(?:archivesBaseName|rootProject\.name)\s*=\s*['"]([^'"]+)['"]"#)
                .ok()?
                .captures(&content);
        if let (Some(group), Some(artifact)) = (group, artifact) {
            return Some(join_package(&group[1], &artifact[1]));
        }
    }

    None
}

fn join_package(group: &str, artifact: &str) -> String {
    let artifact: String = artifact
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    format!("{}.{}", group.trim(), artifact)
}

/// The Minecraft version the project's `paper-api` dependency points at, if any.
pub fn detect_minecraft_version(project_root: &Path) -> Option<String> {
    read_build_files(project_root)
        .iter()
        .find_map(|content| minecraft::detect_from_build_file(content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_build_file_presence() {
        let temp = tempfile::tempdir().unwrap();
        assert!(!has_build_file(temp.path()));

        fs::write(temp.path().join("build.gradle.kts"), "").unwrap();
        assert!(has_build_file(temp.path()));
    }

    #[test]
    fn sanitizes_artifact_into_package_segment() {
        assert_eq!(
            join_package("com.example", "My-Plugin"),
            "com.example.my_plugin"
        );
        assert_eq!(join_package(" io.test ", "tools"), "io.test.tools");
    }

    #[test]
    fn detects_package_from_pom() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(
            temp.path().join("pom.xml"),
            "<project><groupId>dev.acme</groupId><artifactId>ember-hud</artifactId></project>",
        )
        .unwrap();
        assert_eq!(
            detect_package_name(temp.path()).as_deref(),
            Some("dev.acme.ember_hud")
        );
    }

    #[test]
    fn java_source_root_uses_package_directories() {
        let root = java_source_root(Path::new("/proj"), "com.example.plugin");
        assert!(root.ends_with(Path::new("src/main/java/com/example/plugin")));
    }
}
