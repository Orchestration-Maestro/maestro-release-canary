# maestro-release-canary

A canary consumer of `rust-workflows`: the smallest binary that runs its live
release path on real tags.

## Language

**Canary**:
This repository: a consumer whose releases exist to prove `rust-workflows`,
not to ship a product.
_Avoid_: test repository, sample

**Live release path**:
What a `v*` tag runs: `publish-binaries`, `attest-binaries` and
`publish-evidence` from `rust-workflows`, against a real GitHub Release.
_Avoid_: dry run, pipeline

**Payload**:
`payload.tar.gz`, the release archive of the built binary.
_Avoid_: bundle

**Provenance attestation**:
The build provenance GitHub signs for the payload, checked with
`gh attestation verify` against the `attest-binaries.yml` signer workflow.

**Release evidence**:
What lets anyone check a release without trusting its run: the checksums in
`SHA256SUMS`, the provenance attestation, and the dependency list that
`cargo audit bin` reads from the binary.
