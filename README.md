# agent-vcs

Source control for a world where most clients are agents, built from scratch in Rust. Inspired by Anthropic's [Redesigning source control](https://www.anthropic.com/research/redesigning-source-control) (Scion).

Is a fun project to understand in depth how git works, by trying to implement the same functionalities in a completelly different paradigm.

## Where we are

Nothing works yet. There's a design: [PRD](docs/prd.md), [architecture](docs/architecture.md), [glossary](GLOSSARY.md) and [decisions](docs/adr/).

## Where we want to be

A server on my homelab that humans and agents commit to over gRPC. It has no local history, no full copies, and the server does the work. Done when I use it on one real repo of mine.

## How to start

```bash
brew install rustup && rustup default stable   # then add /opt/homebrew/opt/rustup/bin to PATH
cargo build
cargo run --bin avcs-server
cargo run --bin avcs
```

Then pick the next unchecked item in [Phase 0](docs/prd.md#phase-0-the-core-no-network).
