//! Plugin manifest parsing

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<Author>,
    pub license: Option<String>,
    pub min_forge_version: String,
    pub max_forge_version: Option<String>,
    pub permissions: Vec<String>,
    pub commands: Vec<Command>,
    pub hooks: Option<Hooks>,
    pub ui: Option<UiConfig>,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Option<String>,
    pub output_schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hooks {
    #[serde(rename = "onInstall")]
    pub on_install: Option<String>,
    #[serde(rename = "onEnable")]
    pub on_enable: Option<String>,
    #[serde(rename = "onDisable")]
    pub on_disable: Option<String>,
    #[serde(rename = "onUninstall")]
    pub on_uninstall: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub panels: Vec<Panel>,
    pub settings: Vec<Setting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: String,
    pub title: String,
    pub location: String,
    pub component: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    #[serde(rename = "type")]
    pub setting_type: String,
    pub default: serde_json::Value,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algo: String,
    pub public_key: String,
    pub signed_hash: String,
}
