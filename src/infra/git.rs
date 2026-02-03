use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: Option<&Path>) -> Result<String, String> {
    let mut cmd = Command::new("git");

    cmd.args(args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let output = cmd
        .output()
        .map_err(|e| format!("failed to run git {:?}: {e}", args))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let msg = if !stderr.is_empty() { stderr } else { stdout };
        return Err(format!("git {:?} failed: {}", args, msg));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn is_available() -> Result<(), String> {
    run(&["--version"], None).map(|_| ())
}

pub fn clone(repo_url: &str, dest: &Path) -> Result<(), String> {
    run(
        &[
            "clone",
            repo_url,
            dest.to_str().ok_or("invalid destionation path")?,
        ],
        None,
    )
    .map(|_| ())
}

pub fn fetch(repo_dir: &Path) -> Result<(), String> {
    run(&["fetch", "--all", "--prune"], Some(repo_dir)).map(|_| ())
}
