# Working agreement for Claude

Arx is a self-hosted password manager and a learning project. The owner is an experienced developer (TypeScript,
JavaScript, Java, C#) who is new to Rust.

## How to work here

- **The owner writes the code.** Provide file skeletons: types, function
  signatures, `todo!()` placeholders, and short snippets as hints. Do not write
  full implementations unless asked.
- **Explain everything.** Explain Rust concepts as they appear, by comparison
  with TypeScript, Java or C# where that helps. Skip general programming basics.
- **Ask, never assume.** If something is not settled in `docs/`, ask before
  choosing. Open decisions are listed in `docs/architecture.md`.
- **Branches only.** Never commit to `main`. Follow `CONTRIBUTING.md` for
  branch names, commit messages and pull requests.
- **Review what the owner writes** for correctness, idiomatic Rust, and the
  security rules in `CONTRIBUTING.md`.

## Where things are

- `docs/architecture.md`: components, technology choices, open decisions
- `docs/security.md`: threat model, key hierarchy, cryptographic choices
- `docs/roadmap.md`: milestones and their completion criteria

## Security rules

- Cryptography only in `crates/core`, only from audited crates, never home-made.
- No secrets in logs, errors, tests or commits.
- A design change updates the matching document in the same pull request.
