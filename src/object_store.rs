use crate::{Blob, Tree, Commit, Result, Repository};
use sha2::{Sha256, Digest};
use std::fs;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::io::{Read, Write};

pub struct  ObjectStore {
    objects_dir: std::path::PathBuf,
}

impl ObjectStore {
    pub fn new(repo: &Repository) -> Self {
        Self {
            objects_dir: repo.git_dir.join("objects"),
        }
    }

    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.objects_dir)?;
        Ok(())
    }

    pub fn hash_content(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    pub fn store_blob(&self, content: &[u8]) -> Result<String> {
        let hash = Self::hash_content(content);
        let blob = Blob {
            hash: hash.clone(),
            content: content.to_vec(),
        };

        let serialized = serde_json::to_vec(&blob)?;
        self.store_object(&hash, &serialized)?;
        Ok(hash)
    }

    pub fn store_tree(&self, tree: &Tree) -> Result<String> {
        let serialized = serde_json::to_vec(tree)?;
        self.store_object(&tree.hash, &serialized)?;
        Ok(tree.hash.clone())
    }

    pub fn store_commit(&self, commit: &Commit) -> Result<String> {
        let serialized = serde_json::to_vec(commit)?;
        self.store_object(&commit.hash, &serialized)?;
        Ok(commit.hash.clone())
    }

    pub fn store_object(&self, hash: &str, content: &[u8]) -> Result<()> {
        let (dir_name, file_name) = hash.split_at(2);
        let obj_dir = self.objects_dir.join(dir_name);
        fs::create_dir_all(&obj_dir)?;

        let obj_path = obj_dir.join(file_name);
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(content)?;
        let compressed = encoder.finish()?;

        fs::write(obj_path, compressed)?;
        Ok(())
    }

    pub fn load_blob(&self, hash: &str) -> Result<Blob> {
        let content = self.load_object(hash)?;
        let blob: Blob = serde_json::from_slice(&content)?;
        Ok(blob)
    }

    pub fn load_tree(&self, hash: &str) -> Result<Tree> {
        let content = self.load_object(hash)?;
        let tree: Tree = serde_json::from_slice(&content)?;
        Ok(tree)
    }

    pub fn load_commit(&self, hash: &str) -> Result<Commit> {
        let content = self.load_object(hash)?;
        let commit: Commit = serde_json::from_slice(&content)?;
        Ok(commit)
    }

    fn load_object(&self, hash: &str) -> Result<Vec<u8>> {
        let (dir_name, file_name) = hash.split_at(2);
        let obj_path = self.objects_dir.join(dir_name).join(file_name);

        let compressed = fs::read(obj_path)?;
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut content = Vec::new();
        decoder.read_to_end(&mut content)?;

        Ok(content)
    }

    pub fn object_exists(&self, hash: &str) -> bool {
        let (dir_name, file_name) = hash.split_at(2);
        let obj_path = self.objects_dir.join(dir_name).join(file_name);
        obj_path.exists()
    }

}