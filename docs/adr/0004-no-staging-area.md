# No staging area

There's no index and no `add`: `commit` takes the changed files in the working copy, or the paths given. It means fewer concepts for agents, and `status` just compares local files with the base. It's a deliberate deviation from git, so don't add an index without revisiting this.
