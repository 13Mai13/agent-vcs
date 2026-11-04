# Path-level conflicts with server-side rebase

A commit against base B lands on head if no revision in (B, head] touched any of its paths; otherwise it's rejected with the conflicting paths. There's no merging on the server or client in v0: `update` writes a theirs file and the human resolves it.

## Considered Options

- Line-level 3-way merge: what git does. More work, and planned for Phase 3.
- Locks: simple, but the opposite of lock-free commits from many agents.
