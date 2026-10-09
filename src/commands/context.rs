use anyhow::Result;
use regex::Regex;

use crate::installer::resolve_dependencies;
use crate::registry::client::RegistryClient;
use crate::registry::models::{Component, Registry};

pub async fn handle_context(component_name: String, verbose: bool) -> Result<()> {
    let client = RegistryClient::from_env();
    let registry: Registry = client.index().await?;
    let component = registry
        .get_component(&component_name)
        .ok_or_else(|| anyhow::anyhow!("Component '{}' not found in registry", component_name))?;

    println!();

    let context = generate_component_context(&client, &registry, component, verbose).await?;

    println!("{}", context);
    println!();

    Ok(())
}

async fn generate_component_context(
    client: &RegistryClient,
    registry: &Registry,
    component: &Component,
    verbose: bool,
) -> Result<String> {
    let mut output = String::new();

    output.push_str(&format!("// Context: {}\n", component.name));
    output.push_str(&format!("// Category: {}\n", component.category));

    if let Some(minimum) = &component.minecraft {
        output.push_str(&format!("// Minecraft: {minimum}+\n"));
    }

    if !component.dependencies.is_empty() {
        let mut all = resolve_dependencies(registry, &component.name, &[])?;
        all.retain(|c| c.name != component.name);
        let names: Vec<&str> = all.iter().map(|c| c.name.as_str()).collect();
        output.push_str(&format!("// Dependencies: {}\n", names.join(", ")));
    }

    output.push_str("//\n");

    for file_spec in &component.files {
        let content = client
            .component_file(&component.name, &file_spec.path)
            .await?;

        output.push_str(&parse_java_context(&content, verbose));
        output.push('\n');
    }

    Ok(output)
}

#[derive(Debug)]
struct ClassHeader {
    kind: String,
    name: String,
    generics: Option<String>,
    rest: String,
}

fn parse_class_header(line: &str) -> Option<ClassHeader> {
    let re = Regex::new(
        r"^(?:(?:public|protected|private|abstract|final|sealed|static|non-sealed)\s+)*(class|interface|enum|record)\s+(\w+)\s*(<[^{(]*>)?\s*(\([^{]*\))?(.*)$",
    )
    .ok()?;

    let caps = re.captures(line)?;
    let rest = caps
        .get(5)
        .map(|m| m.as_str().trim().trim_end_matches('{').trim().to_string())
        .unwrap_or_default();

    Some(ClassHeader {
        kind: caps[1].to_string(),
        name: caps[2].to_string(),
        generics: caps.get(3).map(|m| m.as_str().to_string()),
        rest,
    })
}

#[derive(PartialEq, Clone, Copy)]
enum Visibility {
    Public,
    Protected,
    Package,
    Private,
}

fn visibility_of(modifiers: &str, in_interface: bool) -> Visibility {
    if in_interface || modifiers.contains("public") {
        Visibility::Public
    } else if modifiers.contains("protected") {
        Visibility::Protected
    } else if modifiers.contains("private") {
        Visibility::Private
    } else {
        Visibility::Package
    }
}

fn is_visible(visibility: Visibility, verbose: bool) -> bool {
    verbose || matches!(visibility, Visibility::Public | Visibility::Protected)
}

/// Removes comments and string literals so braces and keywords inside them are ignored.
fn clean_line(raw: &str, in_block_comment: &mut bool) -> String {
    let mut line = raw.to_string();

    if *in_block_comment {
        match line.find("*/") {
            Some(end) => {
                line = line[end + 2..].to_string();
                *in_block_comment = false;
            }
            None => return String::new(),
        }
    }

    while let Some(start) = line.find("/*") {
        match line[start..].find("*/") {
            Some(end) => line.replace_range(start..start + end + 2, " "),
            None => {
                line.truncate(start);
                *in_block_comment = true;
                break;
            }
        }
    }

    if let Some(index) = line.find("//") {
        line.truncate(index);
    }

    let strings = Regex::new(r#""(?:\\.|[^"\\])*""#).expect("static regex");
    strings.replace_all(&line, "\"\"").into_owned()
}

fn strip_annotations(line: &str) -> String {
    let annotation = Regex::new(r"^(?:@\w+(?:\([^)]*\))?\s*)+").expect("static regex");
    annotation.replace(line, "").trim().to_string()
}

/// The declaration part of a statement: everything before a body (`{`) or terminator (`;`).
fn normalize_signature(statement: &str) -> String {
    let head = statement.split('{').next().unwrap_or_default();
    head.trim()
        .trim_end_matches(';')
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Renders one class-level statement as a context line, or `None` when it is not a member.
fn member_context(statement: &str, in_interface: bool, verbose: bool) -> Option<String> {
    if statement.is_empty()
        || statement.contains(" class ")
        || statement.contains(" interface ")
        || statement.contains(" enum ")
        || statement.contains(" record ")
        || statement.starts_with("class ")
        || statement.starts_with("interface ")
        || statement.starts_with("enum ")
    {
        return None;
    }

    let signature = normalize_signature(statement);
    let modifier_words = r"(?:public|protected|private|static|final|abstract|synchronized|default|native|transient|volatile|sealed|non-sealed)";

    let method = Regex::new(&format!(
        r"^((?:{modifier_words}\s+)*)(?:<[^>]+>\s+)?(?:([\w<>\[\],.? ]+?)\s+)?(\w+)\s*\(([^)]*)\)(?:\s*throws\s+[\w.,\s]+)?$"
    ))
    .ok()?;

    if let Some(caps) = method.captures(&signature) {
        let modifiers = &caps[1];
        let visibility = visibility_of(modifiers, in_interface);
        if !is_visible(visibility, verbose) {
            return None;
        }

        let params = caps[4].split_whitespace().collect::<Vec<_>>().join(" ");
        let mut flags = String::new();
        if modifiers.contains("static") {
            flags.push_str("[static] ");
        }
        if modifiers.contains("abstract") {
            flags.push_str("[abstract] ");
        }

        return Some(match caps.get(2) {
            Some(ret) => format!("// - {flags}{} {}({params})", ret.as_str().trim(), &caps[3]),
            None => format!("// + {flags}{}({params})", &caps[3]),
        });
    }

    let field = Regex::new(&format!(
        r"^((?:{modifier_words}\s+)*)([\w<>\[\],.? ]+?)\s+(\w+)\s*(?:=.*)?$"
    ))
    .ok()?;

    let caps = field.captures(&signature)?;
    let modifiers = &caps[1];
    let visibility = visibility_of(modifiers, in_interface);
    if !is_visible(visibility, verbose) {
        return None;
    }

    Some(format!("//   {}: {}", caps[2].trim(), &caps[3]))
}

pub fn parse_java_context(content: &str, verbose: bool) -> String {
    let mut class: Option<ClassHeader> = None;
    let mut body: Vec<String> = Vec::new();
    let mut depth: i32 = 0;
    let mut in_block_comment = false;
    let mut pending = String::new();
    let mut in_interface = false;

    for raw in content.lines() {
        let cleaned = clean_line(raw, &mut in_block_comment);
        let trimmed = strip_annotations(cleaned.trim());

        if depth == 0 && class.is_none() {
            if let Some(header) = parse_class_header(&trimmed) {
                in_interface = header.kind == "interface";
                class = Some(header);
            }
        } else if depth == 1 && class.is_some() && !trimmed.is_empty() {
            if !pending.is_empty() {
                pending.push(' ');
            }
            pending.push_str(&trimmed);

            let complete =
                pending.ends_with(';') || pending.ends_with('{') || pending.ends_with('}');
            if complete {
                if let Some(line) = member_context(&pending, in_interface, verbose) {
                    body.push(line);
                }
                pending.clear();
            }
        }

        depth += cleaned.matches('{').count() as i32;
        depth -= cleaned.matches('}').count() as i32;
    }

    let Some(class) = class else {
        return String::new();
    };

    let generics = class
        .generics
        .as_ref()
        .map(|g| g.to_string())
        .unwrap_or_default();

    let mut context = String::new();
    let header_rest = if class.rest.is_empty() {
        String::new()
    } else {
        format!(" {}", class.rest)
    };
    context.push_str(&format!(
        "// Class: {}{}{}\n",
        class.name, generics, header_rest
    ));
    if !generics.is_empty() {
        let inner = generics.trim_start_matches('<').trim_end_matches('>');
        context.push_str(&format!("//   Type Parameters: {}\n", inner));
    }
    context.push_str(&format!("//   Kind: {}\n", class.kind));

    for line in &body {
        context.push_str(line);
        context.push('\n');
    }

    if verbose {
        context.push_str("\n// Usage:\n");
        context.push_str(&format!(
            "//   Extend/implement {} to use this component.\n",
            class.name
        ));
        if !generics.is_empty() {
            let inner = generics.trim_start_matches('<').trim_end_matches('>');
            context.push_str(&format!("//   Specify type parameter: <{}>\n", inner));
        }
    }

    context
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGINATED: &str = r#"
package com.example.ui;

public abstract class PaginatedMenu<T> extends BaseMenu {
    private List<T> items;
    protected int currentPage;

    public PaginatedMenu(Player player, String title) {
        super(player, title);
    }

    /** Sets the source. */
    public void setSource(List<T> items) {
        this.items = items;
    }

    public void setRenderer(BiConsumer<T, Integer> renderer) {
        this.renderer = renderer;
    }

    protected abstract ItemStack render(T item, int slot);

    private void helper() {
        if (items != null) { items.clear(); }
    }
}
"#;

    #[test]
    fn parses_class_header_with_generics_and_superclass() {
        let context = parse_java_context(PAGINATED, false);

        assert!(context.contains("// Class: PaginatedMenu<T> extends BaseMenu"));
        assert!(context.contains("Type Parameters: T"));
    }

    #[test]
    fn lists_public_and_protected_members_only() {
        let context = parse_java_context(PAGINATED, false);

        assert!(context.contains("// + PaginatedMenu(Player player, String title)"));
        assert!(context.contains("// - void setSource(List<T> items)"));
        assert!(context.contains("// - void setRenderer(BiConsumer<T, Integer> renderer)"));
        assert!(context.contains("[abstract] ItemStack render(T item, int slot)"));
        assert!(context.contains("//   int: currentPage"));

        assert!(!context.contains("helper"));
        assert!(!context.contains("List<T>: items"));
    }

    #[test]
    fn verbose_includes_private_members_and_usage() {
        let context = parse_java_context(PAGINATED, true);

        assert!(context.contains("helper()"));
        assert!(context.contains("Extend/implement PaginatedMenu"));
    }

    #[test]
    fn ignores_braces_inside_strings_and_comments() {
        let source = r#"
public class Tricky {
    public String brace() { return "}{"; } // }
    /* { */
    public void after() {}
}
"#;
        let context = parse_java_context(source, false);
        assert!(context.contains("// - String brace()"));
        assert!(context.contains("// - void after()"));
    }
}
