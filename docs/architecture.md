# Architecture

How agent-vcs is put together, in the vocabulary of deep modules: **module**, **interface**, **seam**, **adapter**. Terms are defined in the [glossary](../GLOSSARY.md), what we build and when is in the [PRD](prd.md), and why is in the [ADRs](adr/).

## Overview

```
 laptop / agent                          homelab
 ┌──────────────────┐                    ┌───────────────────────────────────┐
 │ avcs (CLI)       │                    │ avcs-server                       │
 │  working copy    │    gRPC over TLS   │  gRPC adapter (tonic)             │
 │  .avcs/base      │ ─────────────────► │        │                          │
 │        │         │    token, stream   │        ▼                          │
 │  RepoService ────┼──── (port) ───────►│  ┌─────────────────────────────┐  │
 │  (gRPC adapter)  │                    │  │ Repo  (the deep module)     │  │
 └──────────────────┘                    │  │  ┌──────┐ ┌─────────────┐   │  │
                                         │  │  │ Log  │ │ ObjectStore │   │  │
                                         │  │  └──────┘ └─────────────┘   │  │
                                         │  │  ┌───────────────────────┐  │  │
                                         │  │  │ derived data (memory) │  │  │
                                         │  │  └───────────────────────┘  │  │
                                         │  └─────────────────────────────┘  │
                                         └───────────────────────────────────┘
```

The server holds all history. The client holds only the files it checked out and the revision they came from.

## Modules

### `Repo`: the deep module

All source-control behaviour for one repo sits behind a small interface:

```rust
impl Repo {
    fn open(dir: &Path) -> Result<Repo>;
    fn missing(&self, ids: &[BlobId]) -> Vec<BlobId>;
    fn put_blob(&self, bytes: impl Read) -> Result<BlobId>;
    fn commit(&self, base: Rev, changes: Changes, meta: Meta) -> Result<Rev, CommitError>;
    fn log(&self, from: Option<Rev>, limit: usize) -> Vec<Changeset>;
    fn read(&self, path: &str, at: Rev) -> Result<Option<impl Read>>;
    fn list(&self, prefix: &str, at: Rev) -> Vec<(String, BlobId)>;
    fn history(&self, path: &str, limit: usize) -> Vec<Rev>;
    fn diff(&self, a: Rev, b: Rev, path: Option<&str>) -> Result<Diff>;
}
```

The interface also includes facts the signatures don't show:

- **Commit is a server-side rebase.** If no revision in `(base, head]` touched a path in `changes`, the commit becomes `head + 1`. If one did, it fails with `CommitError::Conflict { paths }`.
- **Commit is atomic and durable.** `Ok(rev)` means the changeset is fsynced. It's all or nothing across files.
- **Commit requires its blobs.** Every `Put` must already be stored, or it fails with `CommitError::MissingBlobs`. Empty `changes` fails with `CommitError::Empty`.
- **One writer.** Commits to a repo are serialised. Reads run concurrently.
- **Reads are immutable.** The same `(path, rev)` returns the same bytes forever.

Why it's deep: callers learn nine methods. Behind them sit conflict detection, rebase, durability, crash recovery, indexing and diffing. Run the deletion test: remove `Repo`, and the server, the tests and any future tool would each have to reassemble the store, the log and the indexes.

A `Server` holds `name → Repo` for many repos ([PRD](prd.md#features-v0): `init`).

### `ObjectStore`: blobs by content

`put(bytes) -> BlobId`, `get(id) -> bytes`, `has(id)`. The ID is git's blob hash with SHA-256 ([ADR 0007](adr/0007-git-format-sha256-blobs.md)). Writing the same bytes twice is a no-op. Files live at `objects/ab/cdef…`.

- **Dependency:** the filesystem. It's *local-substitutable*, so tests use a temp dir.
- **Seam:** internal to `Repo`. There's one adapter, so it's a plain struct, not a trait. A cache tier would be the second adapter that makes it a real seam.

### `Log`: the write-ahead log

`append(bytes) -> Rev`, `replay()`. Each record is `[len][checksum][bytes]`, and `append` fsyncs before returning. `replay` drops a torn last record instead of failing. `Log` only stores bytes; `Repo` decides what a changeset means ([ADR 0008](adr/0008-plain-files-own-wal.md)).

- **Dependency and seam:** the same as `ObjectStore`.

### Derived data: an internal seam with real variation

```rust
trait Derived {
    fn apply(&mut self, rev: Rev, cs: &Changeset);
}
```

In v0 there are two adapters, so the seam is real:

- **path index:** path → revisions that touched it, with their blob. Answers `history`, `read`, `list` and "touched since base?" for conflicts.
- **revision index:** rev → changeset. Answers `log`.

On open, `Repo` replays the log through every `Derived`. Blame (Phase 2) is one more adapter, with no interface change. The trait stays private: tests reach derived data through `history`, `read` and `list`.

### Diff

Myers' algorithm, implemented in-house as a pure function `diff_lines(a, b) -> Vec<Edit>`. `Repo::diff` calls it per changed path. It's *in-process* only, so there's no seam.

### `RepoService`: the port between client and server

Client and server are separate processes, a *remote but owned* dependency. So there's a port that mirrors `Repo`'s interface, scoped by repo name, with two adapters:

- **gRPC:** `tonic` over TLS. `proto/avcs.proto` is the spec ([ADR 0006](adr/0006-grpc-proto-as-spec.md)). Blob upload is client-streaming and `read` is server-streaming; everything else is unary. A token interceptor checks `authorization`. Reflection is on.
- **in-process:** wraps a `Server` directly. **Tests only**: the CLI always uses gRPC ([ADR 0001](adr/0001-always-online.md)).

The same behaviour suite runs against both adapters.

### `avcs`: the CLI

It parses args, calls `RepoService` and prints text or `--json`. Its own logic is the working copy:

- `.avcs/` holds the server address, the repo, the base revision and the checked-out paths.
- `status` hashes local files and compares them with the base blob IDs. No server call is needed beyond the base listing.
- `commit` hashes changed files, asks `missing`, streams those blobs, then calls `commit`.
- `update` lists head for the checked-out paths, rewrites unedited files, and writes `.avcs-theirs` for edited files that changed.

## Testing

The interface is the test surface:

- **The behaviour suite runs against `RepoService`**, both adapters:
  - commit then read
  - disjoint commits both land
  - an overlapping commit conflicts
  - missing blobs are rejected
  - empty commits are rejected
  - history, list and diff
- **Crash recovery:** truncate the log mid-record, reopen, and get the same state up to the last full record.
- **Diff:** property tests on `diff_lines`. Applying the edits to `a` gives `b`.
- **Blob IDs:** checked against `git hash-object` in a SHA-256 git repo.
- **CLI:** a few end-to-end tests against a real server on localhost.

There are no isolated tests for `ObjectStore`, `Log` or derived data unless a bug needs one.

## Source layout

```
proto/avcs.proto        API spec
src/
  lib.rs
  object.rs             ObjectStore, BlobId
  log.rs                Log
  changeset.rs          Changeset, Changes, Meta, Rev
  diff.rs               Myers
  repo/
    mod.rs              Repo, Server
    derived.rs          Derived + adapters
  service/
    mod.rs              RepoService port
    local.rs            in-process adapter (tests)
    grpc.rs             gRPC client + server adapter
  bin/
    avcs.rs
    avcs-server.rs
```
