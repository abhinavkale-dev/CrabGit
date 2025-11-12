use crate::{Repository, Result, object_store::ObjectStore, utils};

pub fn log(repo: &Repository, max_count: Option<usize>) -> Result<()> {
    let current_branch = utils::get_current_branch(repo)?;
    let mut current_commit = utils::get_branch_commit(repo, &current_branch)?;

    if current_commit.is_none() {
        println!("No commits found");
        return Ok(());
    }

    let object_store = ObjectStore::new(repo);

    let mut count = 0;
    let max = max_count.unwrap_or(usize::MAX);

    while let Some(commit_hash) = current_commit {
        if count >= max {
            break;
        }

        let commit = object_store.load_commit(&commit_hash)?;

        println!("commit {}", commit.hash);
        println!("Author: {}", commit.author);
        println!("Date: {}", commit.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        println!();
        println!("    {}", commit.message);
        println!();

        current_commit = commit.parent;
        count += 1;
    }

    Ok(())
}