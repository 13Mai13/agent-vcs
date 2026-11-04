//! agent-vcs: source control where the server does the work and clients stay thin.

/// Content-addressed object store: every blob is stored under the hash of its bytes.
pub mod object {}

/// A changeset: what changed (paths -> blob hashes) and its parent, not a full tree.
pub mod changeset {}

/// Write-ahead log: changesets are appended in order; the log is the source of truth.
pub mod wal {}

/// Derived data built from the log (file history, blame), computed eagerly or lazily.
pub mod derived {}
