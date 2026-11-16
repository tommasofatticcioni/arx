# Roadmap

Milestones are done in order. Each one ends with something that can be run and
tested. A milestone starts by settling the open decisions listed for it in
[architecture.md](architecture.md).

## Milestone 0: Setup

- [x] Design documents
- [x] GitHub repository, licence, contribution workflow
- [x] Cargo workspace with `core` and `cli` crates
- [x] CI: format check, lints, tests on Linux and Windows, and `cargo deny`
      (vulnerabilities, licences, bans, sources)

**Done when:** a pull request runs CI and passes.

## Milestone 1: Core cryptography

- [ ] Master password to master key (Argon2id)
- [ ] Authenticated encryption and decryption with versioned output
- [ ] Key hierarchy: account key, key pair, vault keys
- [ ] Recovery key and recovery passphrase
- [ ] Secrets wiped from memory
- [ ] Tests against published test vectors

**Done when:** the core can create an account's keys, lock them, and unlock
them with the password, the recovery key, or the recovery passphrase.

## Milestone 2: Vault and CLI

- [ ] Data model: vault, login (title, where it is used, username, password, notes), tags
- [ ] Local encrypted storage
- [ ] CLI: create vault, unlock, add, list, show, edit, delete
- [ ] Filter by tag and search by title

**Done when:** logins can be managed entirely from the command line and the
file on disk contains no plaintext.

## Milestone 3: Tools

- [ ] Password generator (length, character sets, passphrase mode)
- [ ] TOTP from a secret code or an `otpauth://` URI
- [ ] TOTP setup from a QR code image
- [ ] Breach check through the Have I Been Pwned range API

**Done when:** all three tools work from the CLI.

## Milestone 4: Backups

- [ ] Encrypted export file format
- [ ] Import, with handling of duplicates
- [ ] Restore onto a fresh installation

**Done when:** a vault can be exported, the local data deleted, and everything
restored from the file.

## Milestone 5: Server and sync

- [ ] Registration and login without sending the master password
- [ ] Per-device sessions and a device list
- [ ] Push and pull of encrypted changes; conflict handling
- [ ] Account handling: change password, regenerate recovery methods, delete account
- [ ] Rate limiting and lockout
- [ ] Docker image and server-side backups

**Done when:** two CLI installations on different machines stay in sync
through the server, and the server database contains no plaintext.

## Milestone 6: Windows app

- [ ] Tauri 2 project with React + TypeScript
- [ ] Unlock, list, search, tag filter, add, edit, delete
- [ ] Generator, TOTP and breach check screens
- [ ] Auto-lock and clipboard clearing
- [ ] Account, sync, backup and recovery screens

**Done when:** everything the CLI does can be done in the Windows app.

## Milestone 7: Android app

- [ ] The same app built for Android
- [ ] QR code scanning with the camera
- [ ] Android autofill service (Kotlin)
- [ ] Biometric unlock

**Done when:** a login saved on Windows can be autofilled in an Android app.

## Milestone 8: Firefox extension

- [ ] Detect login and registration forms
- [ ] Autofill on user action, matched to the site origin
- [ ] Offer to save new or changed logins
- [ ] Popup: search, copy, generate
- [ ] Firefox for Android support

**Done when:** logging in to a site in Firefox on both platforms needs no
typing.

## Milestone 9: Hardening and sharing

- [ ] Shared vaults between accounts
- [ ] Review of the whole design against [security.md](security.md)
- [ ] Fuzzing of parsers and the sync protocol
- [ ] External review, if possible

**Done when:** a vault can be shared with another account and the review
findings are resolved.
