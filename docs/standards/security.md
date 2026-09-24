# Security rules in `release-canary`

`release-canary` follows the organization's [security
rules](https://github.com/Orchestration-Maestro/.github/blob/main/golden-rules/security.md).
This page is its rule map (C-001): for every rule, what holds it here, or why it
does not apply. A row may name a stricter local rule; none weakens one.

The organization's `scripts/golden-rules.py` writes the rows from the golden
rules and keeps what each row says here. A rule added there arrives as "Not
mapped yet", and the drift check fails until it is mapped.

## What this repository protects

The trust in a release: the payload, its checksums, its provenance attestation
and its evidence must be what the tagged commit built. The binary itself takes
no input; it adds two fixed numbers.

## Rule map

| Rule | Held here by |
| --- | --- |
| SEC-001 Minimise sensitive data | Not applicable: the repository handles no sensitive data |
| SEC-002 Treat input as data | Not applicable: the binary takes no input |
| SEC-003 Validate boundaries | Not applicable: the binary opens no path, URL or argument |
| SEC-004 Use real authority | Organization: rulesets, workflow permissions and the organization bot's own App identity; automation borrows no person's credentials |
| SEC-005 Scope sensitive approvals | Review: a publication, release or settings change is approved in its own pull request |
| SEC-006 Inspect code safely | CI: pull requests run without secrets; release jobs run only on a `v*` tag, behind the `release` environment's approvals |
| SEC-007 Stop and escalate incidents | Review: a suspected exposure stops the work and goes to SECURITY.md's private channel; a leaked secret is revoked and rotated |
| SEC-008 Keep truthful evidence | Review: results are reported as run, with what was not checked |
| SEC-009 Preserve safe progress | Review: blocked work is reported as partial, never as done |
| SEC-010 Report vulnerabilities privately | Organization: private vulnerability reporting is on (`maestrolabs-baseline`), and SECURITY.md routes reports to it |
| SEC-011 Sign every release | Gate: `attest-binaries.yml` signs the payload's build provenance; each release carries `SHA256SUMS` and SBOMs, and the README says how to verify them |
