use crate::{Repository, Result, utils};
use std::fs;

pub fn branch(repo: &Repository, name: Option<String>, delete: bool) -> Result<()> {

    let refs_heads= repo.git_dir.join("refs").join("heads");



    if let Some(branch_name) = name {
        if delete {
            let branch_path = refs_heads.join(&branch_name);
            if branch_path.exists() {
                fs::remove_file(branch_path)?;
                println!("Deleted branch {}", branch_name);
            } else {
                return Err(format!("Branch {} does not exist", branch_name).into());
            }
        }

        else {
            let current_branch = utils::get_current_branch(repo)?;
            let current_commit = utils::get_branch_commit(repo, &current_branch)?;
            if let Some(commit) = current_commit {
                utils::update_branch(repo, &branch_name, &commit)?;
                println!("Created branch {}", branch_name);
            } else {
                return Err("No commits yet, cannot create branch".into());
            }
        }
    } 

    else {
        let current_branch = utils::get_current_branch(repo)?;

        if refs_heads.exists() {
            for entry in fs::read_dir(refs_heads)? {
                let entry = entry?;
                let branch_name = entry.file_name().to_string_lossy().to_string();

                if branch_name == current_branch {
                    println!("* {}", branch_name);
                } else {
                    println!("  {}", branch_name);
                }
            }
        }
    }
    Ok(())
}