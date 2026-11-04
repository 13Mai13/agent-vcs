# agent-vcs

A fun side project: source control for a world where most clients are agents, built from scratch in Rust. Inspired by Anthropic's [Redesigning source control](https://www.anthropic.com/research/redesigning-source-control) (Scion).

Status: just started. Nothing works yet.

## The idea

Git assumes every client is offline and keeps a full copy. Agents and CI jobs are always online, so flip it:

- **No full copy anywhere.** Data is content-addressed: every blob is stored under its hash.
- **The server does the work.** Commits, merges, history and blame happen on the server. There's nothing to push.
- **The log is the source of truth.** Each change is a changeset appended to a write-ahead log. A changeset stores only what changed.
- **Derived data.** File history and blame are built from the log, eagerly or lazily.
- **Thin client.** `avcs` asks the server questions. The working copy is optional and sparse.

## Layout

```
src/lib.rs             object store, changeset, WAL, derived data
src/bin/avcs.rs        client CLI
src/bin/avcs-server.rs server
```

## Roadmap (v0)

- [ ] Object store: put and get blobs by hash, on disk
- [ ] Changeset: parent + changed paths -> blob hashes
- [ ] WAL: append changesets, replay on start
- [ ] Server: `put-blob`, `commit`, `log`, `cat <path>@<changeset>` over a socket
- [ ] Client: `avcs commit`, `avcs log`, `avcs cat`
- [ ] Derived data: file history for a path
- [ ] Sparse checkout: fetch only the paths you ask for

## Not goals

Offline work, git compatibility, scale beyond my homelab.

## Run

```bash
cargo build
cargo run --bin avcs-server
cargo run --bin avcs
```
