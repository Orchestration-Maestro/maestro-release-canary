# Northstar for `maestro-release-canary`

> Automate the guardrails to deliver faster, with higher quality, and more
> securely.

`maestro-release-canary` steers by the organization's
[Northstar](https://github.com/Orchestration-Maestro/.github/blob/864d85597a833864cd8506c3925830503b3c2163/golden-rules/northstar.md):
one KPI per pillar, each with its measurement. Unmeasured is written `not
measured`, never estimated; a value read by hand carries the date it was read.

## The point

Every consumer of rust-workflows trusts its release path. This repository is
where that trust is tested for real: each `v*` tag must publish binaries whose
checksums, provenance and evidence verify.

## KPIs

| Pillar | KPI | Current | Target | Measured by |
| --- | --- | --- | --- | --- |
| Speed | Time from a `v*` tag to its published assets | not measured | set from the first baseline | The Release run's duration |
| Quality | Releases that verify end to end | not measured | every release | README's verification steps on the latest release |
| Maintainability | Managed files out of sync | 0 | 0 | `rust-gate sync --check` in the daily drift check |
| Security | Release assets without a verified attestation | not measured | 0 | `gh attestation verify` on each release's payload |
