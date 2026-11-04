# Flat paths, no tree objects

Changesets map full paths to blobs; there are no tree objects, and a directory is just a path prefix answered by derived data. Git needs trees to rebuild full snapshots cheaply on an offline client. Here the server indexes every path, so trees would add structure without benefit.
