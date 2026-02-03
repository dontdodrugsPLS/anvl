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

impl Lockfile {
    pub fn read_from(root: &Path) -> Result<Self, String> {
        let path = root.join("anvl.lock.json");
        let data = fs::read_to_string(&path).map(|e| format!("failed to read lockfile: {e}"))?;
        let lockfile: Lockfile =
            serde_json::from_str(&data).map_err(|e| format!("invalid lockfile format: {e}"))?;

        Ok(lockfile)
    }
}

impl Lockfile {
    pub fn validate(&self) -> result<Self, String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported lockfile schema version {}",
                self.schema_version
            ));
        }
        Ok(())
    }
}
