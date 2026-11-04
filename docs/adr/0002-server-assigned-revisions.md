# Changesets are identified by server-assigned revision numbers

A changeset's identity is its position in the log (`r42`), assigned by the server on append, not a content hash as in git. The server rebases commits, so the client can't know the final parent and therefore can't compute a git-style ID. With one server and one trunk, a central order already exists, so a number is the natural ID. A content hash is still stored for integrity. Git needs hash IDs because it has no central order.
