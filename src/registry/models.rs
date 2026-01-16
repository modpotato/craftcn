use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub version: String,
    pub components: Vec<Component>,
    #[serde(default)]
    pub categories: HashMap<String, Vec<Component>>,
}

impl Registry {
    pub fn get_component(&self, name: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.name == name)
    }

    pub fn get_components_by_category(&self, category: &str) -> Vec<Component> {
        self.categories.get(category).cloned().unwrap_or_default()
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
    pub styles: HashMap<String, String>,
    #[serde(default)]
    pub sounds: HashMap<String, String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentContext {
    pub name: String,
    pub category: String,
    pub dependencies: Vec<String>,
    pub classes: Vec<ClassContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassContext {
    pub name: String,
    pub generics: Option<String>,
    pub methods: Vec<MethodContext>,
    pub fields: Vec<FieldContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodContext {
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<ParameterContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterContext {
    pub r#type: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldContext {
    pub r#type: String,
    pub name: String,
}
