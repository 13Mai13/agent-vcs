# PRD: agent-vcs

## Problem

Git assumes every client works offline and keeps a full copy of the repository. When most clients are agents and jobs that are always online, that assumption costs more than it buys. Scion flips it: the server holds everything and does the work, and clients stay thin.

## Goal

Learn git in depth by rebuilding its features in that other paradigm. Every feature should teach one git concept by contrast.

**Done when** I use agent-vcs on one real repo of mine (imported from git), with me and an agent both committing to it.

## Users

- **Me**, through the `avcs` CLI.
- **Agents** (e.g. Claude Code), through the same CLI with `--json` output.

Both are first-class. Every changeset records whether its author is a human or an agent.

## Principles

1. **Always online.** No server, no work. The client keeps no history, only a sparse working copy and its base. ([ADR 0001](adr/0001-always-online.md))
2. **The server does the work.** Commits, rebases, diffs and history all happen on the server.
3. **The log is the source of truth.** Everything else is derived data and can be rebuilt from it.
4. **Learning over completeness.** When a feature exists mostly to teach a git concept, build it from scratch (the write-ahead log, Myers diff). When it doesn't (TLS, gRPC plumbing), use a library.

## Features (v0)

| Command | What it does | Git concept it teaches |
|---|---|---|
| `avcs init <repo>` | creates a repo on the server | repository and object storage |
| `avcs checkout <paths> [--at rN]` | writes only those files locally and records the base | working copy vs repository |
| `avcs status` | local files vs the base | the index and working-tree comparison |
| `avcs commit -m <msg> [paths]` | sends changed files; the server rebases or rejects | blobs, commits, hashing |
| `avcs update` | moves the base to head | fetch + merge, without the merge |
| `avcs log [--limit n]` | lists revisions | history and parent links |
| `avcs show <path>@rN` | prints a file at a revision | reconstructing state from history |
| `avcs diff rA rB [path]` | line diff between revisions | the Myers diff algorithm |
| `avcs history <path>` | revisions that touched a path | derived data vs walking full history |

Every command takes `--json`.

### Behaviour

- **Commit** is a server-side rebase. If no revision after your base touched your paths, it lands on head. If one did, it's rejected and lists the conflicting paths. ([ADR 0005](adr/0005-path-level-conflicts.md))
- **A commit is atomic** across its files, and acknowledged only once it's durable on disk. Empty commits are rejected.
- **Update**: a file without local edits moves to head. A file with local edits that also changed on the server keeps my version, and the server version is written next to it as `<file>.avcs-theirs`. I merge by hand, then commit.
- **Diff** is text only. Binary files report "binary files differ".
- **Renames** are a delete plus a put. `history` doesn't follow them.
- **Orphaned blobs** (uploaded for a rejected commit) are kept.

### Non-goals

Offline work, git compatibility on the wire, branches and merges, multiple servers, real per-user auth, scale beyond my homelab.

## Phases

### Phase 0: the core, no network

Everything behind `Repo`, tested in-process.

- [ ] `ObjectStore`: put and get blobs by git-format SHA-256 ID, on disk
- [ ] `Log`: append checksummed records with fsync, replay on open, drop a torn last record
- [ ] `Repo::commit` and `read` on a single trunk, with revision numbers
- [ ] Conflict detection and server-side rebase
- [ ] Derived data: `history(path)` and "files under a prefix at rN"
- [ ] Myers diff

**Exit:** the behaviour suite passes against the in-process adapter, including a crash-recovery test.

### Phase 1: v0, server and CLI

- [ ] `proto/avcs.proto`, commented, as the API spec; generated `docs/api.md`
- [ ] gRPC server (`tonic`), TLS with a self-signed cert, shared token, reflection on
- [ ] Streaming blob upload and read
- [ ] The same behaviour suite passing against the gRPC adapter
- [ ] `avcs` CLI: all v0 commands, with `--json`
- [ ] Sparse working copy, `status`, `update` with `.avcs-theirs`

**Exit:** I run the server on the homelab, check out a few files from my laptop, and commit from two clients at once: one human, one agent.

### Phase 2: v1, the done-test

- [ ] `avcs import <git-repo>`: one-way, reading git's loose objects, trees and commits (packfiles if needed), rehashing SHA-1 blobs as SHA-256
- [ ] `avcs blame <path>` as a new kind of derived data
- [ ] Move one real repo of mine onto agent-vcs and use it for a while

**Exit:** the goal's done-test.

### Phase 3: later ideas

Each one is a git lesson of its own:

- Line-level 3-way merge, replacing `.avcs-theirs`
- `Move(from, to)` as a change type, so history follows renames
- `gc` for orphaned blobs (git's reachability)
- Persisted derived data instead of rebuilding on startup
- An MCP server as an agent adapter, with tool descriptions generated from the `.proto`
- A `pkt-line`-style protocol adapter, to learn git's wire protocol

## Open questions

None blocking Phase 0.
