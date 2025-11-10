use crate::{Repository, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn init(path: Option<PathBuf>) -> Result<()> {
    let work_dir = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    let git_dir = work_dir.join(".crab_git");

    if git_dir.exists() {
        return Err("Repository already initialized".into());
    }

    fs::create_dir_all(&git_dir)?;
    fs::create_dir_all(git_dir.join("objects"))?;
    fs::create_dir_all(git_dir.join("refs").join("heads"))?;
    fs::create_dir_all(git_dir.join("refs").join("remotes"))?;

    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main")?;

    fs::write(
        git_dir.join("config"), 
        "[\n  \"core\"\n]\n"
    )?;

    println!("Initialized empty crab git repository in {}", git_dir.display());

    Ok(())
}