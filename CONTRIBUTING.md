# Contributing

## Workflow

`main` is always in a working state. Nothing is committed to it directly.

1. Create a branch from `main`.
2. Make the change in small commits.
3. Open a pull request. CI must pass.
4. Review, then squash-merge into `main` and delete the branch.

## Branch names

`<type>/<short-description>`, lowercase, words separated by hyphens.

| Type | Use for | Example |
|---|---|---|
| `feat` | New functionality | `feat/argon2-key-derivation` |
| `fix` | Bug fixes | `fix/totp-time-step` |
| `docs` | Documentation only | `docs/project-foundation` |
| `chore` | Tooling, CI, dependencies | `chore/ci-cargo-audit` |
| `refactor` | Code changes with no behaviour change | `refactor/vault-storage-trait` |
| `test` | Tests only | `test/aead-vectors` |

## Commit messages

[Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <summary in the imperative>

<optional body explaining why>
```

The scope is the component: `core`, `cli`, `server`, `app`, `extension`, `docs`, `ci`.

Example: `feat(core): derive master key with Argon2id`

## Before opening a pull request

Once the Cargo workspace exists (milestone 0), these must pass locally:

```bash
cargo fmt --all -- --check
```

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

```bash
cargo test --workspace
```

## Rules for anything that touches secrets

- Cryptographic code lives only in `crates/core`.
- No new cryptographic algorithm or crate without updating
  [docs/security.md](docs/security.md) in the same pull request.
- Never log, print or include in an error message: passwords, keys, TOTP
  secrets, recovery material, or decrypted item contents.
- Never commit real credentials, vault files or backups, even for testing.
- Adding a dependency needs a reason in the pull request description.

## Design changes

A change to the architecture or the security design updates the matching
document in `docs/` in the same pull request. Open decisions are listed in
[docs/architecture.md](docs/architecture.md) and are closed there when made.
