# maestro-release-canary

A deliberately small Rust binary whose only job is to prove, on real tags, that
[rust-workflows](https://github.com/Orchestration-Maestro/rust-workflows)
releases what it claims: every pull request runs the organization's Rust CI,
and every `v*` tag runs the live release path.

| Workflow | What it proves |
| --- | --- |
| `ci.yml` | The organization's Rust CI template, as a new repository adopts it |
| `release.yml` | Binary upload, SLSA provenance attestation and release evidence on a protected tag |

## Cutting a release

1. Push a tag `vX.Y.Z` on a commit of `main`.
2. Create the GitHub Release for it: the publishers upload to an existing
   Release and never create one.
3. Approve the two `release` deployments the run waits for.

## Verifying a release

Download the assets of a release, then:

```bash
sha256sum --check SHA256SUMS
gh attestation verify payload.tar.gz --repo Orchestration-Maestro/maestro-release-canary \
  --signer-workflow Orchestration-Maestro/rust-workflows/.github/workflows/attest-binaries.yml
tar -xzf payload.tar.gz
cargo audit bin maestro-release-canary
```
