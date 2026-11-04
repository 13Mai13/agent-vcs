# agent-vcs

Source control where the server holds all history and does the work, and clients (humans and agents) stay thin and always online.

## Language

### History

**Repo**:
A named history on a server. One server holds many repos.
_Avoid_: repository (fine in prose, but the code says repo), project

**Changeset**:
One entry in a repo's log: a set of changes, plus its author, kind and message. It records only what changed, never a full snapshot.
_Avoid_: commit (the noun), patch

**Change**:
What happens to one path in a changeset: either a put of a blob or a delete.
_Avoid_: edit, modification

**Revision**:
A changeset's position in the log, assigned by the server (`r42`). It's how everyone refers to a changeset.
_Avoid_: commit ID, version, hash

**Log**:
The ordered sequence of a repo's changesets. The source of truth for everything else.
_Avoid_: history (that's derived per path), journal

**Trunk**:
The single line of revisions in a repo. There are no branches.
_Avoid_: main, master, branch

**Head**:
The latest revision on the trunk.

### Content

**Blob**:
The bytes of one file version, identified by its blob ID.
_Avoid_: object, file (a file is a path plus a blob)

**Blob ID**:
The hash of a blob in git's blob format.
_Avoid_: SHA, object ID

**Path**:
A file's full name within a repo, e.g. `src/a.rs`. Directories don't exist on their own; they are path prefixes.
_Avoid_: tree, directory entry

### Clients

**Author**:
Who made a changeset, as they declare it (e.g. `mai`, `claude@laptop`).

**Author kind**:
Whether the author is a human or an agent.

**Working copy**:
The files a client has checked out locally, and nothing else. Always partial.
_Avoid_: clone, checkout (the noun)

**Base**:
The revision a working copy came from. Commits are made against it.
_Avoid_: parent, upstream

**Conflict**:
When a commit touches a path that some revision after its base also touched. The commit is rejected.
_Avoid_: merge conflict (no merging happens)

**Theirs file**:
The server's version of a conflicting file, written next to the local one during update.

### Server

**Derived data**:
Indexes the server builds from the log to answer questions (per-path history, files at a revision). Always rebuildable from the log.
_Avoid_: cache, metadata
