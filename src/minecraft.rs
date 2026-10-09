//! Minecraft version helpers: ordering, build-file detection and resource pack formats.

use regex::Regex;
use std::cmp::Ordering;

/// The Paper line craftcn's components are verified against by default.
pub const DEFAULT_TARGET: &str = "26.2";

/// Oldest Minecraft release the bundled components are written for.
pub const MIN_SUPPORTED: &str = "1.21.4";

/// `(first Minecraft version using the format, pack_format)`, from the Minecraft Wiki.
/// Versions between two entries use the earlier entry's format.
const PACK_FORMATS: &[(&str, u32)] = &[
    ("1.21.4", 61),
    ("1.21.5", 71),
    ("1.21.6", 80),
    ("1.21.7", 81),
    ("1.21.9", 88),
    ("1.21.11", 94),
    ("26.1", 101),
    ("26.2", 107),
    ("26.3", 121),
];

/// Parses `26.2`, `1.21.4` or `26.1.2` into numeric parts.
pub fn parse(version: &str) -> Option<Vec<u32>> {
    let parts = version
        .trim()
        .split('.')
        .map(|part| part.parse::<u32>().ok())
        .collect::<Option<Vec<_>>>()?;
    (!parts.is_empty()).then_some(parts)
}

fn compare(a: &[u32], b: &[u32]) -> Ordering {
    let len = a.len().max(b.len());
    for i in 0..len {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        match x.cmp(&y) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

/// True when `version` is `minimum` or newer. Unparseable versions are never "at least" anything.
pub fn is_at_least(version: &str, minimum: &str) -> bool {
    match (parse(version), parse(minimum)) {
        (Some(v), Some(m)) => compare(&v, &m) != Ordering::Less,
        _ => false,
    }
}

/// The `pack_format` a resource pack needs for a Minecraft version (1.21.4 and newer).
pub fn pack_format(version: &str) -> Option<u32> {
    let wanted = parse(version)?;
    PACK_FORMATS
        .iter()
        .rev()
        .find(|(first, _)| {
            let first = parse(first).unwrap_or_default();
            compare(&wanted, &first) != Ordering::Less
        })
        .map(|(_, format)| *format)
}

/// Extracts the Minecraft version from a `paper-api` version string.
/// `26.2.build.132-stable` becomes `26.2`, `1.21.4-R0.1-SNAPSHOT` becomes `1.21.4`.
pub fn from_paper_api_version(raw: &str) -> Option<String> {
    let re = Regex::new(r"^(\d+\.\d+(?:\.\d+)?)").ok()?;
    re.captures(raw.trim()).map(|caps| caps[1].to_string())
}

/// Finds the `paper-api` version in a Maven `pom.xml` or Gradle build script.
pub fn detect_from_build_file(content: &str) -> Option<String> {
    let maven = Regex::new(r"paper-api</artifactId>\s*<version>\s*([^<\s]+)\s*</version>").ok()?;
    let gradle = Regex::new(r#"io\.papermc\.paper:paper-api:([^"'\s)]+)"#).ok()?;

    maven
        .captures(content)
        .or_else(|| gradle.captures(content))
        .and_then(|caps| from_paper_api_version(&caps[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_versions_numerically() {
        assert!(is_at_least("26.2", "1.21.4"));
        assert!(is_at_least("1.21.11", "1.21.4"));
        assert!(!is_at_least("1.21.4", "1.21.6"));
        assert!(is_at_least("1.21.6", "1.21.6"));
        assert!(!is_at_least("garbage", "1.21.4"));
    }

    #[test]
    fn maps_versions_to_pack_formats() {
        assert_eq!(pack_format("1.21.4"), Some(61));
        assert_eq!(pack_format("1.21.8"), Some(81));
        assert_eq!(pack_format("1.21.10"), Some(88));
        assert_eq!(pack_format("26.1.2"), Some(101));
        assert_eq!(pack_format("26.2"), Some(107));
        assert_eq!(pack_format("1.20.6"), None);
    }

    #[test]
    fn reads_paper_api_versions() {
        assert_eq!(
            from_paper_api_version("26.2.build.132-stable").as_deref(),
            Some("26.2")
        );
        assert_eq!(
            from_paper_api_version("1.21.4-R0.1-SNAPSHOT").as_deref(),
            Some("1.21.4")
        );
        assert_eq!(from_paper_api_version("latest"), None);
    }

    #[test]
    fn detects_version_from_pom_and_gradle() {
        let pom = "<dependency>\n<artifactId>paper-api</artifactId>\n<version>1.21.11-R0.1-SNAPSHOT</version>\n</dependency>";
        assert_eq!(detect_from_build_file(pom).as_deref(), Some("1.21.11"));

        let gradle = r#"compileOnly("io.papermc.paper:paper-api:26.2.build.132-stable")"#;
        assert_eq!(detect_from_build_file(gradle).as_deref(), Some("26.2"));

        assert_eq!(detect_from_build_file("nothing here"), None);
    }
}
