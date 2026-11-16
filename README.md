# password-manager

A personal, self-hosted, zero-knowledge password manager written in Rust.

> **Status: planning.** There is no code yet. This repository currently holds
> the design documents and the roadmap.
>
> **This software is unaudited and is a learning project.** Do not rely on it
> as your only password manager.

## Planned features

- Logins with a title, the sites/apps where they are used, username, password and notes
- Tags on logins, with filtering
- Vaults, with a key design that allows sharing a vault between accounts later
- Password generator
- Two-factor (TOTP) code generator, set up from a secret code or a QR code
- Breached ("pwned") password check
- Accounts for a small group (family/friends) on one self-hosted server
- Sync between devices through that server, which only ever stores encrypted data
- Encrypted backups, a recovery key and a recovery passphrase
- Windows and Android apps
- Firefox extension (desktop and Android) that detects login and registration forms and autofills them
- Android system autofill for other apps

## Components

| Path | What it is | Status |
|---|---|---|
| `crates/core` | Rust library: crypto, key hierarchy, vault model, generator, TOTP, backup format | planned |
| `crates/cli` | Command-line client built on the core | planned |
| `crates/server` | Sync and account server | planned |
| `apps/app` | Tauri 2 app for Windows and Android (React + TypeScript on Vite) | planned |
| `apps/extension` | Firefox extension | planned |

## Documentation

- [Architecture](docs/architecture.md): components, how they talk to each other, technology choices
- [Security design](docs/security.md): threat model, key hierarchy, cryptography
- [Roadmap](docs/roadmap.md): milestones and what "done" means for each
- [Contributing](CONTRIBUTING.md): branch, commit and pull request workflow
- [Security policy](SECURITY.md): how to report a vulnerability

## License

[MIT](LICENSE)
