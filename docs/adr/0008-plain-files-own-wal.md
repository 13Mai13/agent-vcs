# Plain files and a hand-written write-ahead log

The server stores blobs as files under `objects/` and each repo's log as one append-only file of checksummed records, fsynced on every commit. Derived data is rebuilt in memory on startup. SQLite or an embedded key-value store would be more robust, but writing the log yourself (durability, torn writes, replay) is the core of what Scion teaches, and homelab scale doesn't need more.
