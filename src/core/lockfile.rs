use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    pub lock_version: u32,
    pub cache_commit: String,
    pub installed_modules: Vec<String>,

    pub modules: std::collections::HashMap<String, ModulesEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesEntry {
    pub files: std::collections::HashMap<String, String>,
}

impl Lockfile {
    pub fn new() -> Self {
        Self {
            lock_version: 1,
            cache_commit: String::new(),
            installed_modules: Vec::new(),
            modules: std::collections::HashMap::new(),
        }
    }
}

impl Lockfile {
    pub fn write_to(&self, root: &Path) -> Result<(), String> {
        let path = root.join("anvl.lock.json");
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("failed to serialize lockfile: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("failed to write lockfile: {e}"))?;
        Ok(())
    }
}
