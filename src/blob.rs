//! BlobStore: blobs stored by blob ID (git blob format, SHA-256). See ADR 0007.
use sha2::{Digest, Sha256}; // brings what Cargo.toml has, no need to do sha2::SHA-256
// Digest is the trait (kind of an interface)
pub struct BlobId([u8; 32]); // 32 bytes, inline: no heap allocationlowlevel

impl BlobId {
    pub fn of(bytes: &[u8]) -> BlobId { // Hashes any number of bytes (borrowed, not copied) into their BlobId

        let mut hasher = Sha256::new(); // mut because it will be changed
        // Git hashes a header before the content:
        // blob 6\0hello\n
        // │    │ │
        // │    │ └─ a zero byte, separating the header from the content
        // │    └─── the length of the content in bytes ("hello\n" = 6)
        // └──────── the object type
        // We need the git compatibility
        hasher.update(format!("blob {}\0", bytes.len()).as_bytes()); // the git header
        hasher.update(bytes); // then the content itself
        let hash = hasher.finalize(); // I'm guessin this is how the Sha256 API works
        BlobId(hash.into()) // .into() converts it whatever type we need here
    }
    pub fn to_hex(&self) -> String { // Reads this ID (borrowed) and returns it as 64 lowercase hex characters.
        let mut out = String::new();
        for byte in self.0 { // because in position 0 is the array
            out.push_str(&format!("{:02x}", byte));
        }
        out
    }
}
