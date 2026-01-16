use anyhow::Result;
use regex::Regex;

use crate::registry::client::RegistryClient;

pub async fn handle_context(component_name: String, verbose: bool) -> Result<()> {
    let registry = RegistryClient::get_registry().await?;
    let component = registry
        .get_component(&component_name)
        .ok_or_else(|| anyhow::anyhow!("Component '{}' not found in registry", component_name))?;

    println!();

    let context = generate_component_context(&component_name, component, verbose).await?;

    println!("{}", context);
    println!();

    Ok(())
}

async fn generate_component_context(
    component_name: &str,
    component: &crate::registry::models::Component,
    verbose: bool,
) -> Result<String> {
    let mut output = String::new();

    output.push_str(&format!("// Context: {}\n", component_name));
    output.push_str(&format!("// Category: {}\n", component.category));

    if !component.dependencies.is_empty() {
        output.push_str(&format!(
            "// Dependencies: {}\n",
            component.dependencies.join(", ")
        ));
    }

    output.push_str("//\n");

    for file_spec in &component.files {
        let content = RegistryClient::get_file_content(component_name, &file_spec.path).await?;

        let file_context = parse_java_context(&content, verbose);

        output.push_str(&file_context);
        output.push('\n');
    }

    Ok(output)
}

fn parse_java_context(content: &str, verbose: bool) -> String {
    let mut context = String::new();

    let class_re = Regex::new(
        r"(?:public\s+)?(?:abstract\s+)?(?:class|interface|enum)\s+(\w+)(?:<([^>]+)>)?\s*$",
    )
    .unwrap();
    let method_re = Regex::new(r"(?:public|protected|private)?\s*(?:static\s+)?(?:abstract\s+)?(\w+(?:<[^>]+>)?)\s+(\w+)\s*\(([^)]*)\)").unwrap();
    let field_re = Regex::new(
        r"(?:public|protected|private)?\s*(?:static\s+)?(?:final\s+)?(\w+(?:<[^>]+>)?)\s+(\w+)",
    )
    .unwrap();

    let lines: Vec<&str> = content.lines().collect();
    let mut in_class = false;
    let mut class_name = String::new();
    let mut class_generics = String::new();
    let mut indent = 0;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("package") || trimmed.starts_with("import") {
            continue;
        }

        if let Some(caps) = class_re.captures(trimmed) {
            class_name = caps[1].to_string();
            class_generics = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
            in_class = true;
            indent = trimmed.len() - trimmed.trim_start().len();

            let generics_suffix = if !class_generics.is_empty() {
                format!("<{}>", class_generics)
            } else {
                String::new()
            };
            context.push_str(&format!("// Class: {}{}\n", class_name, generics_suffix));

            if !class_generics.is_empty() {
                context.push_str(&format!("//   Type Parameters: {}\n", class_generics));
            }

            continue;
        }

        if in_class {
            let current_indent = trimmed.len() - trimmed.trim_start().len();

            if current_indent <= indent && !trimmed.is_empty() && !trimmed.starts_with("//") {
                break;
            }

            continue;
        }

        if in_class {
            let current_indent = trimmed.len() - trimmed.trim_start().len();

            if current_indent <= indent && !trimmed.is_empty() && !trimmed.starts_with("//") {
                break;
            }

            if let Some(caps) = method_re.captures(trimmed) {
                let return_type = &caps[1];
                let method_name = &caps[2];
                let params = &caps[3];

                context.push_str(&format!(
                    "// - {} {}({})\n",
                    return_type, method_name, params
                ));
            }

            if let Some(caps) = field_re.captures(trimmed) {
                let field_type = &caps[1];
                let field_name = &caps[2];

                context.push_str(&format!("//   {}: {}\n", field_type, field_name));
            }
        }
    }

    if verbose {
        context.push_str("\n// Usage:\n");
        context.push_str(&format!(
            "//   Extend/implement {} to use this component.\n",
            class_name
        ));

        if !class_generics.is_empty() {
            let generics = class_generics.trim_start_matches('<').trim_end_matches('>');
            context.push_str(&format!("//   Specify type parameter: <{}>\n", generics));
        }
    }

    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_java_context() {
        let code = r#"
package com.example.ui;

public abstract class PaginatedMenu<T> extends BaseMenu {
    private List<T> items;
    protected int currentPage;
    
    public PaginatedMenu(Player player, String title) {
        super(player, title);
    }
    
    public void setSource(List<T> items) {
        this.items = items;
    }
    
    public void setRenderer(BiConsumer<T, Integer> renderer) {
        this.renderer = renderer;
    }
    
    protected abstract ItemStack render(T item, int slot);
}
"#;

        let context = parse_java_context(code, false);

        assert!(context.contains("Class: PaginatedMenu<T>"));
        assert!(context.contains("Type Parameters: T"));
        assert!(context.contains("setSource"));
        assert!(context.contains("setRenderer"));
    }
}
