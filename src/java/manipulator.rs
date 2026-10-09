use regex::{NoExpand, Regex};

/// Registry sources are written under `com.craftcn`. Installed files are rewritten from
/// that namespace to the project's root package.
pub struct JavaManipulator {
    package: String,
    namespace_re: Regex,
    package_line_re: Regex,
    theme_ref_re: Regex,
}

impl JavaManipulator {
    pub fn new(package: String) -> Self {
        Self {
            package,
            namespace_re: Regex::new(r"\bcom\.craftcn\b").expect("static regex"),
            package_line_re: Regex::new(r"(?m)^\s*package\s+([\w.]+)\s*;").expect("static regex"),
            theme_ref_re: Regex::new(r"\bUITheme\b").expect("static regex"),
        }
    }

    /// Rewrites a registry source file for the project: every `com.craftcn` reference
    /// (package line, imports, qualified names) moves to the project's package, and the
    /// generated `UITheme` import is added when the file needs it.
    pub fn process_file(&self, content: &str) -> String {
        let rewritten = self
            .namespace_re
            .replace_all(content, NoExpand(&self.package))
            .into_owned();

        self.inject_theme_import(&rewritten)
    }

    fn theme_import(&self) -> String {
        format!("import {}.ui.UITheme;", self.package)
    }

    fn file_package<'a>(&self, content: &'a str) -> Option<&'a str> {
        self.package_line_re
            .captures(content)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
    }

    fn inject_theme_import(&self, content: &str) -> String {
        let ui_package = format!("{}.ui", self.package);

        // Files in the UI package itself see UITheme without an import.
        if self.file_package(content) == Some(ui_package.as_str()) {
            return content.to_string();
        }

        if !self.theme_ref_re.is_match(content) {
            return content.to_string();
        }

        let import_line = self.theme_import();
        if content.lines().any(|line| line.trim() == import_line) {
            return content.to_string();
        }

        let lines: Vec<&str> = content.lines().collect();

        // Prefer placing the import after the last existing import, keeping imports together.
        if let Some(last_import) = lines
            .iter()
            .rposition(|line| line.trim_start().starts_with("import "))
        {
            let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
            out.insert(last_import + 1, import_line);
            return join_lines(&out, content);
        }

        // No imports yet: place it directly after the package declaration.
        if let Some(package_index) = lines
            .iter()
            .position(|line| line.trim_start().starts_with("package "))
        {
            let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
            out.insert(package_index + 1, String::new());
            out.insert(package_index + 2, import_line);
            return join_lines(&out, content);
        }

        content.to_string()
    }
}

fn join_lines(lines: &[String], original: &str) -> String {
    let mut joined = lines.join("\n");
    if original.ends_with('\n') {
        joined.push('\n');
    }
    joined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_package_imports_and_qualified_names() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());

        let content = "package com.craftcn.ui.menus;\n\nimport com.craftcn.ui.core.BaseMenu;\n\nclass X extends com.craftcn.ui.core.BaseMenu {}\n";
        let result = manipulator.process_file(content);

        assert!(result.starts_with("package com.example.plugin.ui.menus;"));
        assert!(result.contains("import com.example.plugin.ui.core.BaseMenu;"));
        assert!(result.contains("extends com.example.plugin.ui.core.BaseMenu"));
        assert!(!result.contains("com.craftcn"));
    }

    #[test]
    fn injects_theme_import_after_existing_imports() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());

        let content = "package com.example.plugin.ui.menus;\n\nimport org.bukkit.entity.Player;\n\nclass Menu {\n    Material m = UITheme.FILLER_GLASS;\n}\n";
        let result = manipulator.process_file(content);

        assert!(result
            .contains("import org.bukkit.entity.Player;\nimport com.example.plugin.ui.UITheme;\n"));
    }

    #[test]
    fn skips_theme_import_inside_ui_package() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());

        let content = "package com.craftcn.ui;\n\nclass Helper {\n    UITheme t;\n}\n";
        let result = manipulator.process_file(content);

        assert!(!result.contains("import com.example.plugin.ui.UITheme;"));
    }

    #[test]
    fn does_not_duplicate_existing_theme_import() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());

        let content =
            "package com.craftcn.ui.hud;\n\nimport com.craftcn.ui.UITheme;\n\nclass Hud {}\n";
        let result = manipulator.process_file(content);

        assert_eq!(
            result
                .matches("import com.example.plugin.ui.UITheme;")
                .count(),
            1
        );
    }

    #[test]
    fn injects_theme_import_when_no_imports_exist() {
        let manipulator = JavaManipulator::new("com.example.plugin".to_string());

        let content =
            "package com.craftcn.ui.core;\n\nclass Base {\n    Object t = UITheme.BACKGROUND;\n}\n";
        let result = manipulator.process_file(content);

        assert!(result.contains(
            "package com.example.plugin.ui.core;\n\nimport com.example.plugin.ui.UITheme;"
        ));
    }
}
