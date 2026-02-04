use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleMeta {
    pub name: String,

    #[serde(default)]
    pub desc: String,

    #[serde(default)]
    pub provides: Vec<ProvidesItem>,

    #[serde(default)]
    pub includes: String,

    #[serde(default)]
    pub deps: Vec<String>,

    #[serde(default)]
    pub public_headers: Vec<String>,

    #[serde(default)]
    pub private_headers: Vec<String>,

    #[serde(default)]
    pub src: Vec<String>,

    #[serde(default)]
    pub src_tests: Vec<String>,

    #[serde(default)]
    pub ld_flags: Vec<String>,

    #[serde(default)]
    pub extern_fn_calls: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProvidesItem {
    #[serde(default)]
    pub func: String,

    #[serde(default)]
    pub desc: String,
}

pub fn load(path: &Path) -> Result<ModuleMeta, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let meta: ModuleMeta = toml::from_str(&text)
        .map_err(|e| format!("invalid module TOML metadata {}: {e}", path.display()))?;

    Ok(meta)
}
