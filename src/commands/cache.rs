use std::fs;

use crate::cli::args::CacheAction;
use crate::core::config::Config;
use crate::core::paths;
use crate::utils::git;

pub fn dispatch(action: CacheAction, verbose: bool) -> Result<(), String> {
    match action {
        CacheAction::Update => cmd_update(verbose),
        CacheAction::Clean => cmd_clean(verbose),
    }
}

pub fn cmd_update(verbose: bool) -> Result<(), String> {
    let cfg = Config::get()?;
    let anvl_storage = paths::get_cache_paths()?;

    git::is_available()?;
    if !anvl_storage.repo_dir.join(".git").is_dir() {
        git::clone(&cfg.repo, &anvl_storage.repo_dir)?;
    } else {
        git::fetch_all(&anvl_storage.repo_dir)?;
        git::reset_hard(&anvl_storage.repo_dir, "origin/HEAD")?;
    }
    Ok(())
}

pub fn cmd_clean(verbose: bool) -> Result<(), String> {
    let anvl_storage = paths::get_cache_paths()?;

    if anvl_storage.repo_dir.exists() {
        fs::remove_dir_all(&anvl_storage.repo_dir).map_err(|e| {
            format!(
                "failed to remove cache repo '{}': {e}",
                anvl_storage.repo_dir.display()
            )
        })?;
    }
    Ok(())
}
