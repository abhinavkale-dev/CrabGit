use crate::{Repository, Result, object_store::ObjectStore, utils};
use std::fs;

pub fn checkout(repo: &Repository, branch_or_commit: String) -> Result<()> {
    let refs_heads = repo.git_dir.join("refs").join("heads");
    let branch_path = refs_heads.join(&branch_or_commit);

    if branch_path.exists() {
        utils::update_head(repo, &branch_or_commit)?;

        let commit_hash = utils::get_branch_commit(repo, &branch_or_commit)?;
        if let Some(commit) = commit_hash {
            restore_working_directory(repo, &commit)?;
        }

        println!("Switched to branch '{}'", branch_or_commit);
    } else {
        let object_store = ObjectStore::new(repo);
        if object_store.object_exists(&branch_or_commit) {
            // Update HEAD to point directly to commit (detached HEAD)
            fs::write(repo.git_dir.join("HEAD"), &branch_or_commit)?;
            restore_working_directory(repo, &branch_or_commit)?;

            println!("HEAD is now at {} (detached HEAD)", &branch_or_commit[..8]);
        } else {
            return Err(format!("Branch or commit '{}' not found", branch_or_commit).into());
        }
    }

    Ok(())
}

fn restore_working_directory(repo: &Repository, commit_hash: &str) -> Result<()> {
    let object_store = ObjectStore::new(repo);
    let commit = object_store.load_commit(commit_hash)?;
    let tree = object_store.load_tree(&commit.tree)?;

    for entry in fs::read_dir(&repo.work_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.file_name().unwrap() == ".mini_git" {
            continue;
        }

        if path.is_file() {
            fs::remove_file(&path)?;
        } else if path.is_dir() {
            fs::remove_dir_all(&path)?;
        }
    }

    for (path, tree_entry) in &tree.entries {
        if tree_entry.is_file {
            let blob = object_store.load_blob(&tree_entry.hash)?;
            let file_path = repo.work_dir.join(path);

            // Create parent directories if needed
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(file_path, &blob.content)?;
        }
    }

    Ok(())
}