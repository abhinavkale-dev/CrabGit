use crate::{Index, IndexEntry, Repository, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_git_dir(start_path: &Path) -> Option<PathBuf> {
    let mut path = start_path.to_path_buf();
    loop {
        let git_dir = path.join(".crab_git");
        if git_dir.exists() && git_dir.is_dir() {
            return Some(git_dir);
        }

        if !path.pop() {
            break;
        }
    }
    None
}

pub fn get_repository(work_dir: Option<PathBuf>) -> Result<Repository> {
    let work_dir = work_dir.unwrap_or_else(|| std::env::current_dir().unwrap());

    if let Some(git_dir) = find_git_dir(&work_dir) {
        Ok((Repository { 
            git_dir: git_dir.clone(),
            work_dir: git_dir.parent().unwrap().to_path_buf()
        }))
    } else {
        Err(format!("Not a crab git repository: {}", work_dir.display()).into())
    }
}

pub fn load_index(repo: &Repository) -> Result<Index> {
    let index_path = repo.git_dir.join("index");
    if !index_path.exists() {
        let content = fs::read_to_string(index_path)?;
        let index: Index = serde_json::from_str(&content)?;
        Ok(index)
    } else {
        Ok(
            Index { entries: std::collections::HashMap::new() },
        )
    }
}

pub fn save_index(repo: &Repository, index: &Index) -> Result<()> {
    let index_path = repo.git_dir.join("index");
    let content = serde_json::to_string_pretty(index)?;
    fs::write(index_path, content)?;
    Ok(())
}

pub fn get_current_branch(repo: &Repository) -> Result<String> {
    let head_path = repo.git_dir.join("HEAD");
    if head_path.exists() {
        let content = fs::read_to_string(head_path)?;

        if content.starts_with("ref: refs/heads/") {
            Ok(content.trim().replace("ref: refs/heads/", ""))
        } else {
            Ok("detached".to_string())
        }
    } else {
        Ok("main".to_string())
    }
}

pub fn update_head(repo: &Repository, branch: &str) -> Result<()> {
    let head_path = repo.git_dir.join("HEAD");
    fs::write(head_path, format!("ref: refs/heads/{}", branch))?;
    Ok(())
}

pub fn get_branch_commit(repo: &Repository, branch: &str) -> Result<Option<String>> {
    let branch_path = repo.git_dir.join("refs").join("heads").join(branch);

    if branch_path.exists() {
        let commit = fs::read_to_string(branch_path)?;
        Ok(Some(commit.trim().to_string()))
    } else {
        Ok(None)
    }
}

pub fn update_branch(repo: &Repository, branch: &str, commit: &str) -> Result<()> {
    let refs_heads= repo.git_dir.join("refs").join("heads");
    fs::create_dir_all(&refs_heads)?;

    let branch_path = refs_heads.join(branch);
    fs::write(branch_path, commit)?;
    Ok(())
}