# CL drift: `validate_descriptor` shipped as `pub`, locked CL says private

**Date:** 2026-05-25
**Scope:** workspace task #398 (`Reconcile round 202605090011 src CL wording with shipped validate_descriptor visibility`). Off-chain bounded slice; not part of any active arc.
**Status:** drift documentation. Names the discrepancy, proposes the reconciliation path. Future round runs the formal mockspace ceremony to close the task.

## The drift

The locked SRC CL for round 202605090011 (descriptor v1.1 ABI hardening) claims:

```
### CHANGE: fn `validate_descriptor` ADDED (private host-side helper)

File: `mock/crates/hilavitkutin-extensions/src/host.rs`
Verification: `grep "fn validate_descriptor" mock/crates/hilavitkutin-extensions/src/host.rs` returns at least one match.
```

(Source: `mock/design_rounds/202605090011/202605090011_changelist.src.lock.md:103-106`.)

Shipped reality at `mock/crates/hilavitkutin-extensions/src/host.rs:220-232`:

```rust
/// Returns `Outcome::Ok(())` on success. The first failed check
/// surfaces; subsequent fields are not read. Public so consumers can
/// validate a descriptor pointer they obtained outside the standard
/// load path (e.g. from a probe-only inspection or a custom loader),
/// and so the integration test suite can exercise the contract
/// directly.
pub fn validate_descriptor(
    descriptor: &ExtensionDescriptor,
) -> Outcome<(), ExtensionError> {
```

The function is `pub`, not private. The rustdoc explicitly names two reasons for the public visibility: (1) consumer-side validation of descriptor pointers obtained outside the standard load path, and (2) integration-test access to the contract.

The CL verification grep (`grep "fn validate_descriptor"`) passes against the shipped state, so the lint did not catch the drift. The visibility difference is in the prose annotation `(private host-side helper)`, not in the verification command.

## Why the drift matters

Per `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`:

> A locked changelist is a claim about source state. Claims about source state must match source reality at lock time. When they do not, the gap is a discipline violation, not a documentation slip.

The CL says the function is private. Source says it is public. A future agent reading the CL to understand what shipped would form a wrong picture of the contract surface. The visibility is part of the contract, not an implementation detail: `pub fn validate_descriptor` is a stable host-side contract that consumers (probe inspectors, custom loaders, the integration test suite) rely on.

## Why the drift exists

Reconstructing from the rustdoc rationale, the change happened during apply. The CL was authored against the original design that named the helper as private. During implementation, two real consumer needs surfaced (probe inspection, integration tests) that required public visibility. The rustdoc captures the rationale; the CL prose was not updated to match.

Per the discipline rule's preamble, this is the canonical failure mode the lint at #481 (superseding #318) is designed to catch. The cl-claim-vs-source-mismatch lint would have flagged this at lock time if it had been live then.

## Reconciliation path

Two options for closing #398.

**Option A: deprecate the original CL and write a replacement.** Open a new round (or use the existing #398 task as the round identifier). Author a deprecated-CL chain that:

- Renames `202605090011_changelist.src.lock.md` to `*.deprecated.md` per the mockspace deprecate flow.
- Writes a new active CL whose `## CHANGE: fn validate_descriptor` block reads `ADDED (public host-side helper)` with rustdoc rationale matching the shipped state.
- Includes a `## Comparison to deprecated changelist` section per the deprecation-comparison lint, naming the visibility discrepancy as the load-bearing change.

This preserves the audit trail (original CL is auditable as `*.deprecated.md`) and lands a CL that matches source reality.

**Option B: leave the locked CL as-is, write an addendum sketch under `mock/research/sketches/`.** The sketch documents the drift, references the locked CL, and points future readers to the rustdoc rationale on the shipped source.

Option A matches the discipline rule's framing precisely: claims about source state must match source reality, and when they do not, the fix is a corrected CL via the deprecate flow. Option B is the "addendum" pattern explicitly criticised in the discipline rule: "deprecation that postpones the discipline is the discipline failing".

**Recommendation: Option A.**

## What Option A requires

A round in hilavitkutin that touches no source code. The round's deliverable is the deprecated original CL plus a corrected replacement CL. Touch points:

1. Topic file: `mock/design_rounds/<new-timestamp>/<new-timestamp>_topic.reconcile-validate-descriptor-visibility.md`.
2. Run `cargo mock deprecate mock/design_rounds/202605090011/202605090011_changelist.src.lock.md` (or the v2 equivalent) to mark the original as deprecated.
3. Author a replacement CL at `mock/design_rounds/<new-timestamp>/<new-timestamp>_changelist.src.md` with the corrected CHANGE block plus the comparison section.
4. Lock the replacement CL.
5. Close the round.

No source changes. No new tests. The round is purely a CL-state correction.

## Why this slice ships as a research memo, not the ceremony itself

This memo is the off-chain bounded slice. The ceremony itself is the next slice; running it requires hilavitkutin's mockspace v2 phase machine, which is operational ergonomics not currently familiar enough to the agent to execute in a single firing window without risk of incomplete state. The memo:

- Documents the drift so a future firing (or op) can run the ceremony directly against this artefact as the design input.
- Names Option A as the recommended path with the rationale visible.
- Lists the touch points so the ceremony slice is mechanical.

The task #398 description framing ("Reconcile ... wording with shipped visibility") is unambiguous in pointing at this drift; this memo confirms the drift exists in the shape the task describes, and pins the resolution shape.

## What this slice does NOT do

- Does not run the mockspace ceremony itself. That belongs to the follow-up slice.
- Does not edit `host.rs` or any other source. The shipped public visibility is correct; the CL prose is what drifts from it.
- Does not change the locked CL file. Locked CLs are immutable per the discipline rule; the reconciliation is via deprecate-and-replace, not in-place edit.

## See also

- `mock/design_rounds/202605090011/202605090011_changelist.src.lock.md:103-106` (the locked claim).
- `mock/crates/hilavitkutin-extensions/src/host.rs:220-232` (the shipped reality with rustdoc rationale).
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md` (the discipline this memo applies).
- Workspace task #398 (the off-chain task this slice begins).
- Workspace task #481 (the cl-claim-vs-source-mismatch lint that would have caught this drift at lock time).
