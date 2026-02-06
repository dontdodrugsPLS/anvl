use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectPaths {
    pub root: PathBuf,
    pub lockfile_path: PathBuf,
}

pub fn find_project_root() -> Result<ProjectPaths, String> {
    let mut current_path =
        std::env::current_dir().map_err(|e| format!("failed to get current directory: {e}"))?;

    Err("not inside an Anvl project (anvl.lock.json not found in parent directory)".to_string())
}
