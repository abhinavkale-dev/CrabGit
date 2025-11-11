use crate::{Commit, Repository, Result, Tree, TreeEntry, object_store::ObjectStore, utils};
use chrono::Utc;
use std::collections::HashMap;

pub fn commit(repo: &Repository, message: String, author: Option<String>) -> Result<()> {
    let index = utils::load_index(repo)?;
    if index.entries.is_empty() {
        return Err("Nothing to commit, working tree clean".into());
    }

    let object_store = ObjectStore::new(repo);

    let mut tree_entries = HashMap::new();
    for (path, entry) in &index.entries {
        tree_entries.insert(
            path.clone(),
            TreeEntry {
                mode: entry.mode.clone(),
                hash: entry.hash.clone(),
                name: path.clone(),
                is_file: true,
            },
        );
    }

    let tree_content = serde_json::to_vec(&tree_entries)?;
    let tree_hash = ObjectStore::hash_content(&tree_content);
    let tree = Tree {
        hash: tree_hash.clone(),
        entries: tree_entries,
    };

    object_store.store_tree(&tree)?;

    let current_branch = utils::get_current_branch(repo)?;
    let parent = utils::get_branch_commit(repo, &current_branch)?;

    let author = author.unwrap_or_else(|| "Unknown email <unknown@example.com>".to_string());
    let commit_content = format!(
        "{}{}{}{}",
        tree_hash,
        parent.as_ref().unwrap_or(&String::new()),
        author,
        message
    );
    let commit_hash = ObjectStore::hash_content(commit_content.as_bytes());

    let commit = Commit {
        hash: commit_hash.clone(),
        parent,
        tree: tree_hash,
        author,
        message,
        timestamp: Utc::now(),
    };

    object_store.store_commit(&commit)?;

    utils::update_branch(repo, &current_branch, &commit_hash)?;

    println!("Created commit {}", &commit_hash[..8]);
    Ok(())
}