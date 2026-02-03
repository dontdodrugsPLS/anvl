use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    pub lock_version: u32,
    pub cache_commit: u32,
    pub installed_modules: Vec<String>,

    pub modules: std::collections::HashMap<String, ModulesEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModulesEntry {
    pub files: std::collections::HashMap<String, String>,
}
