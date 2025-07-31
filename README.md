# vaultkit

**VaultKit** is a backend-agnostic, pluggable password interface in Rust.

It defines a trait-based abstraction (`PasswordSource`) for working with password storage systems like:

- [`pass`](https://www.passwordstore.org/)
- 1Password (via CLI)
- Bitwarden (via API or CLI)
- ....

The goal is to provide a unified interface for querying, searching, syncing, and modifying secrets, regardless of backend.

## Goals

- Swappable backends via feature flags
- Clear security model per backend
- Frontends (CLI, TUI, GUI) can use the same trait
- Easily testable with mock sources
- Backend logic isolated from presentation

## Status

It doesn't exist (beyond this file and the trait) :-)

## License

MIT
