# Clients are always online and keep no history

The client keeps only a sparse working copy and its base revision; all history lives on the server, and every operation needs it. This is the paradigm flip from git that the project exists to explore, and it's what makes server-side commits, rebases and derived data possible. The cost is accepted on purpose: no server, no work. Even the CLI has no local mode; the in-process adapter exists for tests only.
