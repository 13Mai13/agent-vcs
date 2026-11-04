# gRPC, with the .proto as the API spec

Client and server talk gRPC (`tonic`, TLS, streaming for blobs). The commented `proto/avcs.proto` is the single API spec; docs are generated from it, and both sides are generated code, so the spec can't drift.

## Considered Options

- HTTP + JSON: easier to poke with curl, but the spec would live in a separate document (reflection + grpcurl covers the poking).
- A custom pkt-line-style protocol: closer to git, but weeks of plumbing before any version control works. It's a Phase 3 idea.
- MCP as the spec: tool descriptions are for LLMs, not a contract. MCP will be an adapter whose descriptions come from the .proto.
