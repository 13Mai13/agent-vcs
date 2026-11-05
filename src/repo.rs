//! Repo: the deep module. All source-control behaviour for one repo. See docs/architecture.md.
mod derived;
// mod fisrt use after
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use crate::blob::BlobId; // crate:: = this crate's root (lib.rs); tests/ say agent_vcs:: because they're outside
pub struct Repo {
    path: PathBuf, // private: no pub. Owned: PathBuf (a &Path would be borrowed)
}

impl Repo {
    pub fn open(path: &Path) -> io::Result<Repo> {
        fs::create_dir_all(path.join("blobs"))?; // blobs because is the name of the folder containing the blobs
        Ok(Repo { path: path.to_path_buf() })
    }

    pub fn put_blob(&self, mut bytes: impl Read) -> io::Result<BlobId> {

        let mut byte_buffer = Vec::new();
        bytes.read_to_end(&mut byte_buffer)?;
        let id = BlobId::of(&mut byte_buffer); // & is enough: of only reads. &mut only when the callee changes it
        let path = self.path.join("blobs").join(id.to_hex());
        fs::write(path, &byte_buffer)?; // no mut: write only reads the buffer
        Ok(id)
    }

    pub fn missing(&self, ids: &[BlobId]) -> Vec<BlobId> {
        let mut missing_ids = Vec::new();
        for id in ids{
            let path = self.path.join("blobs").join(id.to_hex());
            if !path.exists(){
                missing_ids.push(*id); // push adds one item to a Vec; push_str is only for String (adds text) --> We have * because we want to own the result
            }
        }
        missing_ids
    }
}

