use crate::{Index, IndexEntry, Repository, Result, object_store::ObjectStore, utils};
use std::fs;
use clap::builder::Str;
use walkdir::WalkDir;

pub fn add(repo: &Repository, paths: Vec<String>) -> Result<()> {
    let mut index = utils::load_index(repo)?;
    let object_store = ObjectStore::new(repo);

    for path_str in paths {
        if path_str == "." {
            for entry in WalkDir::new(&repo.work_dir)
                 .into_iter()
                 .filter_map(|e| e.ok())
                 .filter(|e| e.file_type().is_file())
                 {
                    let path = entry.path();
                    if path.starts_with(&repo.work_dir) {
                        let relative_path = path
                            .strip_prefix(&repo.work_dir)?
                            .to_string_lossy()
                            .replace("\\", "/");

                        add_file_to_index(&mut index, &object_store, path, &relative_path)?;
                    }
                 }
        } else {
            let full_path = repo.work_dir.join(&path_str);
            if full_path.is_file() {
                add_file_to_index(&mut index, &object_store, &full_path, &path_str)?;
            } else if full_path.is_dir() {
                for entry in WalkDir::new(&full_path) 
                     .into_iter()
                     .filter_map(|e| e.ok())
                     .filter(|e| e.file_type().is_file())
                {
                    let path = entry.path();
                    let relative_path = path
                         .strip_prefix(&full_path)?
                         .to_string_lossy()
                         .replace("\\", "/");

                    add_file_to_index(&mut index, &object_store, &path, &relative_path)?;
                }
            }
        }
    }

    utils::save_index(repo, &index)?;
    println!("Added files to staging area");

    Ok(())
}


pub fn add_file_to_index(
    index: &mut Index,
    object_store: &ObjectStore,
    file_path: &std::path::Path,
    relative_path: &str,
) -> Result<()> {

    let content = fs::read(file_path)?;
    let hash = object_store.store_blob(&content)?;

    index.entries.insert(
        relative_path.to_string(),
        IndexEntry { 
        hash,
        mode: "100644".to_string(),
        path: relative_path.to_string(), 
        }
    );
    Ok(())
}