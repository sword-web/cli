use crate::error::AppResult;
use crate::types::GenerationPlan;

use camino::{Utf8Path, Utf8PathBuf};
use std::process::Command;

/// Writes every file of the plan and returns the paths it touched.
pub fn apply(plan: &GenerationPlan) -> AppResult<Vec<Utf8PathBuf>> {
    let mut touched = Vec::with_capacity(plan.files.len());

    for file in &plan.files {
        let dest = plan.root.join(&file.path);

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(&dest, &file.contents)?;

        touched.push(dest);
    }

    Ok(touched)
}

/// Initializes a git repository and optionally links a remote.
pub fn init_git(dir: &Utf8Path, remote: Option<&str>) -> AppResult<()> {
    let status = Command::new("git")
        .arg("init")
        .current_dir(dir.as_str())
        .status()?;

    if !status.success() {
        eprintln!("warning: git init failed");
    }

    if let Some(url) = remote {
        let status = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(url)
            .current_dir(dir.as_str())
            .status()?;

        if !status.success() {
            eprintln!("warning: git remote add failed");
        }
    }

    Ok(())
}
