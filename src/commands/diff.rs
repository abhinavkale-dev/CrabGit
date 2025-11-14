use crate::{Repository, Result, object_store::{self, ObjectStore}, utils};
use std::fs;


pub fn diff(repo: &Repository, files: Vec<String>) -> Result<()> {

    let index = utils::load_index(repo)?;
    let object_store = ObjectStore::new(repo);

    if files.is_empty() {
        for (path, _index_entry) in &index.entries {
            let _ = show_file_diff(repo, &object_store, path)?;
        }
    }

    else {
        for file in files{
            if index.entries.contains_key(&file) {
                let _ = show_file_diff(repo, &object_store, &file)?;
            } else {
                println!("File '{}' not tracked", file);
            }
        }
    }


    Ok(())
}

fn show_file_diff(repo: &Repository, object_store: &ObjectStore, path: &str) -> Result<()> {

    let file_path = repo.work_dir.join(path);
    let index = utils::load_index(repo)?;

    let index_entry = match index.entries.get(path) {
        Some(entry) => entry,
        None => return Ok(()),
    };

    if !file_path.exists() {
        println!("diff --git a/{} b/{}", path, path);
        println!("deleted file");
        println!("--- a/{}", path);
        println!("+++ /dev/null");


        let blob = object_store.load_blob(&index_entry.hash)?;
        let staged_content = String::from_utf8_lossy(&blob.content);
        for line in staged_content.lines() {
            println!("-{}", line);
        }
        return Ok(());
    }

    let current_content = fs::read(&file_path)?;
    let current_hash = ObjectStore::hash_content(&current_content);

    if current_hash == index_entry.hash {
        return Ok(());
    }

    let blob = object_store.load_blob(&index_entry.hash)?;
    let staged_content = String::from_utf8_lossy(&blob.content);
    let current_content_str = String::from_utf8_lossy(&current_content);

    println!("diff --git a/{} b/{}", path, path);
    println!("--- a/{}", path);
    println!("+++ b/{}", path);

    let staged_lines: Vec<&str> = staged_content.lines().collect();
    let current_lines: Vec<&str> = current_content_str.lines().collect();

    let mut common_start = 0;

    while common_start < staged_lines.len()
    && common_start < current_lines.len()
    && staged_lines[common_start] == current_lines[common_start]
    {
        common_start += 1;
    }

    let mut common_end = 0;

    while common_end < (staged_lines.len() - common_start)
        && common_end < (current_lines.len() - common_start)
        && staged_lines[staged_lines.len() - 1 - common_end]
            == current_lines[current_lines.len() - 1 - common_end]
    {
        common_end += 1;
    }

    for line in &staged_lines[common_start..staged_lines.len() - common_end] {
        println!("-{}", line);
    }

    for line in &current_lines[common_start..current_lines.len() - common_end] {
        println!("+{}", line);
    }


    Ok(())
}