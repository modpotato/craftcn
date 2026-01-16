use anyhow::Result;
use regex::Regex;

pub struct JavaManipulator {
    package: String,
}

impl JavaManipulator {
    pub fn new(package: String) -> Self {
        Self { package }
    }

    pub fn process_file(&self, content: &str) -> Result<String> {
        let mut result = content.to_string();

        result = self.rewrite_package(&result)?;
        result = self.inject_theme_import(&result)?;

        Ok(result)
    }

    fn rewrite_package(&self, content: &str) -> Result<String> {
        let package_re = Regex::new(r"^package\s+([\w.]+);")
            .map_err(|e| anyhow::anyhow!("Failed to compile package regex: {}", e))?;

        if let Some(caps) = package_re.captures(content) {
            let old_package = &caps[1];

            let new_package = if let Some(relative) = old_package.strip_prefix("com.craftcn") {
                if relative.is_empty() {
                    self.package.clone()
                } else {
                    format!("{}.{}", self.package, relative.trim_start_matches('.'))
                }
            } else {
                old_package.to_string()
            };

            let new_content = package_re.replace(content, &format!("package {};", new_package));

            return Ok(new_content.to_string());
        }

        Ok(content.to_string())
    }

    fn inject_theme_import(&self, content: &str) -> Result<String> {
        let import_re = Regex::new(r"^import\s+([\w.]+);").unwrap();
        let theme_import = format!("import {}.ui.UITheme;", self.package);

        if import_re
            .find_iter(content)
            .any(|m| m.as_str().contains("UITheme"))
        {
            return Ok(content.to_string());
        }

        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut imports_end = 0;

        for (i, line) in lines.iter().enumerate() {
            if line.trim().starts_with("import ") {
                imports_end = i + 1;
            }
        }

        for (i, line) in lines.iter().enumerate() {
            result.push(line.to_string());

            if i == imports_end && imports_end > 0 {
                result.push(theme_import.clone());
            }
        }

        Ok(result.join("\n"))
    }

    pub fn extract_class_name(&self, content: &str) -> Option<String> {
        let class_re =
            Regex::new(r"(?:public\s+)?(?:abstract\s+)?(?:class|interface|enum)\s+(\w+)").unwrap();
        class_re.captures(content).map(|caps| caps[1].to_string())
    }

    pub fn extract_methods(&self, content: &str) -> Vec<(String, String, String)> {
        let method_re = Regex::new(r"(?:public|protected|private)?\s*(?:static\s+)?(?:abstract\s+)?(\w+(?:<[^>]+>)?)\s+(\w+)\s*\(([^)]*)\)").unwrap();

        method_re
            .captures_iter(content)
            .map(|caps| {
                (
                    caps[1].to_string(),
                    caps[2].to_string(),
                    caps[3].to_string(),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rewrite_package() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());

        let content = r#"package com.craftcn.ui.menus;

public class TestMenu {
}"#;

        let result = manipulator.rewrite_package(content).unwrap();
        assert!(result.contains("package com.example.plugin.ui.menus;"));
    }

    #[test]
    fn test_inject_theme_import() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());

        let content = r#"package com.example.ui.menus;

import org.bukkit.entity.Player;

public class TestMenu {
}"#;

        let result = manipulator.inject_theme_import(content).unwrap();
        assert!(result.contains("import com.example.plugin.ui.UITheme;"));
    }
}
