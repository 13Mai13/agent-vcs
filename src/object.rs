//! ObjectStore: blobs stored by blob ID (git blob format, SHA-256). See ADR 0007.
use sha2::{Digest, Sha256}; // brings what Cargo.toml has, no need to do sha2::SHA-256
// Digest is the trait (kind of an interface)
pub struct BlobId([u8; 32]); // 32 bytes, inline: no heap allocationlowlevel

impl BlobId {
    pub fn of(bytes: &[u8]) -> BlobId { // Hashes any number of bytes (borrowed, not copied) into their BlobId

        let mut hasher = Sha256::new(); // mut because it will be changed
        hasher.update(bytes);
        let hash = hasher.finalize(); // I'm guessin this is how the Sha256 API works
        BlobId(hash.into()) // .into() converts it whatever type we need here
    }
    pub fn to_hex(&self) -> String { // Reads this ID (borrowed) and returns it as 64 lowercase hex characters.
        todo!()
    }
}
