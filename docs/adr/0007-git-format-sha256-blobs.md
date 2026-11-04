# Blob IDs are git's blob format with SHA-256

A blob ID is `sha256("blob <len>\0" + bytes)`, exactly git's SHA-256 object format, so IDs can be checked against `git hash-object` in a SHA-256 git repo. SHA-1 would match most real git repos but is broken for collisions; BLAKE3 is faster but loses the git link. Changing this later means rehashing every stored blob. Import from a SHA-1 git repo rehashes its blobs.
