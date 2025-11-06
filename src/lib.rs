pub mod commands;
pub mod utils;
pub mod object_store;

use Chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blob {
    pub hash: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree {
    pub hash: String,
    pub entries: HashMap<String, TreeEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeEntry {
    pub mode: String,
    pub hash: String,
    pub name: String,
    pub is_file: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
   pub hash: String,
   pub parent: Option<String>,
   pub tree: String,
   pub author: String,
   pub message: String,
   pub timestamp: DateTime<Utc>,
}

