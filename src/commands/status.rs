use crate::{Repository, Result, utils};
use std::fs;
use std::collections::HashSet;
use walkdir::WalkDir;

pub fn status(repo: &Repository) -> Result<()> {
    let current_branch = utils::get_current_branch(repo)?;
    println!("# On branch {} #", current_branch);

    let index = utils::load_index(repo)?;

    let mut working_tree_files = HashSet::new();
    for entry in WalkDir::new(&repo.work_dir)
         .into_iter()
         .filter_map(|e| e.ok())
         .filter(|e| e.file_type().is_file())
         {
            let path = entry.path();
            if path.starts_with(&repo.work_dir) {
                continue;
            }

            let relative_path = path
                 .strip_prefix(&repo.work_dir)?
                 .to_string_lossy()
                 .replace("\\", "/");

            working_tree_files.insert(relative_path);
        }

        let mut staged_files = Vec::new();
        let mut modified_files = Vec::new();

        for (path, entry) in &index.entries {
            staged_files.push(path.clone());

            let file_path = repo.work_dir.join(path);
            if file_path.exists() {
                let content = fs::read(&file_path)?;
                let current_hash = crate::object_store::ObjectStore::hash_content(&content);
                if current_hash != entry.hash {
                    modified_files.push(path.clone());
                }
            }
            working_tree_files.remove(path);
        }

        if !staged_files.is_empty() {
            println!("Changes to be committed:");
            for path in &staged_files {
                println!("  new file: {}", path);
            }
        }

        if !modified_files.is_empty() {
            println!("Changes not staged for commit:");
            for path in &modified_files {
                println!("  modified: {}", path);
            }
        }

        if !working_tree_files.is_empty() {
            println!("Untracked files:");
            for path in &working_tree_files {
                println!("  {}", path);
            }
        }

        if staged_files.is_empty() && modified_files.is_empty() && working_tree_files.is_empty() {
            println!("Nothing to commit, working tree clean");
        }

    Ok(())
}