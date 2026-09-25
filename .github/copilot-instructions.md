# Copilot instructions for release-canary

## Start here

A deliberately small Rust binary whose only job is to prove, on real tags, that
[rust-workflows](https://github.com/Orchestration-Maestro/rust-workflows)
releases what it claims: every push runs the organization's Rust CI, and every
`v*` tag runs the live release path.

Paths below are relative to this repository. Before editing, read
[AGENTS.md](../AGENTS.md) for the rules that bind every change,
[CONTEXT.md](../CONTEXT.md) for the words it uses and
[CONTRIBUTING.md](https://github.com/Orchestration-Maestro/.github/blob/main/CONTRIBUTING.md)
for how a change is proposed. The organization's [golden
rules](https://github.com/Orchestration-Maestro/.github/blob/main/golden-rules/engineering.md)
come first: nothing in a specification, a plan or this repository weakens them.

For quality, engineering or security changes, read
[northstar.md](../docs/standards/northstar.md),
[engineering.md](../docs/standards/engineering.md) and
[security.md](../docs/standards/security.md): this repository's map of the
organization's golden rules.

Keep changes scoped to the request, and read historical plans and specifications
as records, not as instructions to start new work.

## Repository tree

Every tracked file, with what it is for. `rust-gate guide` writes this tree at
every commit and keeps each explanation already here, so improve an explanation
in place.

```text
.                                               # Repository root
├── .config/                                    # Tool settings that live in a directory
│   └── nextest.toml                            # TOML settings: nextest; rendered by rust-gate sync
├── .github/                                    # GitHub metadata, templates and workflows
│   ├── workflows/                              # GitHub Actions workflows
│   │   ├── ci.yml                              # CI: calls ci.yml, upload-coverage.yml, upload-sarif.yml; rendered by rust-gate sync
│   │   ├── dependabot-auto-merge.yml           # Dependabot auto-merge
│   │   ├── release.yml                         # Release: calls attest-binaries.yml, publish-binaries.yml, publish-evidence.yml
│   │   └── scorecard.yml                       # OpenSSF Scorecard
│   ├── CODEOWNERS                              # Who reviews each path
│   ├── copilot-instructions.md                 # This guide, written by rust-gate guide at every commit
│   └── dependabot.yml                          # The organization merges only conventional titles: "ci(deps): bump ..."; rendered by rust-gate sync
├── docs/                                       # Documentation
│   └── standards/                              # Standards
│       ├── engineering.md                      # Engineering rules in release-canary
│       ├── northstar.md                        # Northstar for release-canary
│       └── security.md                         # Security rules in release-canary
├── src/                                        # The crate's sources
│   ├── lib.rs                                  # Checked arithmetic for the binary consumer fixture
│   └── main.rs                                 # Binary consumer fixture that prints a checked arithmetic result
├── supply-chain/                               # cargo-vet audits, configuration and imports
│   ├── audits.toml                             # cargo-vet audits file
│   ├── config.toml                             # cargo-vet config file
│   └── imports.lock                            # The audits cargo-vet imports, locked
├── tests/                                      # Integration tests
│   └── cli.rs                                  # The released binary, run as a user runs it: it prints the checked sum
├── .editorconfig                               # Editor settings that survive the editor; rendered by rust-gate sync
├── .gitattributes                              # How Git should treat each kind of file; rendered by rust-gate sync
├── .gitignore                                  # Paths git never tracks
├── .pre-commit-config.yaml                     # The commit hooks prek runs locally and CI runs over every file; rendered by rust-gate sync
├── .rumdl.toml                                 # rumdl: the Markdown structure every repository holds to; rendered by rust-gate sync
├── .taplo.toml                                 # taplo: the TOML formatter just check and the commit hook run over every TOML file in the repository; rendered by rust-gate sync
├── .yamlfmt.yml                                # How yamlfmt formats every YAML file; rendered by rust-gate sync
├── AGENTS.md                                   # Rules for coding agents: what to read, what never to weaken, how to verify
├── CONTEXT.md                                  # The words this repository uses, and the ones it avoids
├── Cargo.lock                                  # Exact dependency versions, committed so every build resolves the same
├── Cargo.toml                                  # Crate manifest: Canary consumer that proves the live release path of rust-workflows
├── LICENSE                                     # The licence this repository is distributed under
├── README.md                                   # A deliberately small Rust binary whose only job is to prove, on real tags, that rust-workflows releases what it claims
├── clippy.toml                                 # TOML settings: clippy; rendered by rust-gate sync
├── deny.toml                                   # DEP-001: one version of each crate, no wildcard requirement, crates.io alone, and no yanked or unmaintained crate; rendered by rust-gate sync
├── maestro-quality.toml                        # This repository's quality settings; rust-gate sync reads them
├── rust-toolchain.toml                         # The pinned Rust toolchain; rendered by rust-gate sync
├── rustfmt.toml                                # TOML settings: rustfmt; rendered by rust-gate sync
└── typos.toml                                  # The words this repository means, from [typos] words in maestro-quality.toml; rendered by rust-gate sync
```

## Change and verification procedure

1. Read the rules in AGENTS.md that cover the files you change, and keep every
   gate intact: never weaken one to pass.
2. Add an executable regression check for a change in behaviour.
3. The commit hook `rust-gate guide` rewrites this guide when a file is added,
   moved or removed; commit it with the change. The organization's daily drift
   check reports a guide left stale.
4. Run `prek run --all-files`, and report the commands you actually ran.
5. Commits are signed, with a conventional title; the default branch takes only
   squash-merged pull requests.
