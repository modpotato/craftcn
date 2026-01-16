use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_project_root() -> Result<PathBuf> {
    let current = std::env::current_dir()?;

    for path in current.ancestors() {
        if has_build_file(path) {
            return Ok(path.to_path_buf());
        }
    }

    anyhow::bail!("Could not find project root (pom.xml or build.gradle not found)")
}

fn has_build_file(path: &Path) -> bool {
    path.join("pom.xml").exists() || path.join("build.gradle").exists()
}

pub fn detect_package_name(project_root: &Path) -> Option<String> {
    let pom_path = project_root.join("pom.xml");

    if pom_path.exists() {
        if let Ok(content) = fs::read_to_string(&pom_path) {
            if let Some(caps) = Regex::new(r"<groupId>([^<]+)</groupId>")
                .unwrap()
                .captures(&content)
            {
                let group = caps[1].trim();
                if let Some(artifact_caps) = Regex::new(r"<artifactId>([^<]+)</artifactId>")
                    .unwrap()
                    .captures(&content)
                {
                    let artifact = artifact_caps[1].trim();
                    return Some(format!("{}.{}", group, artifact));
                }
            }
        }
    }

    let gradle_path = project_root.join("build.gradle");

    if gradle_path.exists() {
        if let Ok(content) = fs::read_to_string(&gradle_path) {
            let group_re = match Regex::new(r#"group\s+['"]([^'"]+)['"]"#) {
                Ok(re) => re,
                Err(e) => return None,
            };

            if let Some(caps) = group_re.captures(&content) {
                let group = caps[1].trim();

                let artifact_re = match Regex::new(r#"archivesBaseName\s*=\s*['"]([^'"]+)['"]"#)
                    .or_else(|_| Regex::new(r#"rootProject\.name\s*=\s*['"]([^'"]+)['"]"#))
                {
                    Ok(re) => re,
                    Err(_) => return None,
                };

                if let Some(artifact_caps) = artifact_re.captures(&content) {
                    let artifact = artifact_caps[1].trim();
                    return Some(format!("{}.{}", group, artifact));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_build_file() {
        let temp = std::env::temp_dir();
        assert!(!has_build_file(&temp));
    }
}
