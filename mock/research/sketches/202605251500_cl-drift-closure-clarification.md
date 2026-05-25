# Sketch: CL drift closure clarification

**Date:** 2026-05-25
**Hypothesis:** Workspace task #398 (`Reconcile round 202605090011 src CL wording with shipped validate_descriptor visibility`) is already satisfied by the drift memo landed in PR #85; further ceremony work is not required by the discipline rule.
**Outcome:** WORKS. The discipline rule explicitly says retroactive reconciliation of past rounds is not required.
**Parent context:** PR #85 (drift memo at `mock/research/202605251200_validate-descriptor-cl-drift.md`), PR #87 (v2 phase machine field notes at `mock/research/sketches/202605251400_v2-phase-machine-field-notes.md`).

## What this sketch closes

The previous two artefacts on #398 (the drift memo and the field-notes sketch) both treated "Option A: deprecate-and-replace via mockspace flow" as the intended reconciliation. Investigating the mockspace CLI source at `~/.cargo/git/checkouts/mockspace-d2db2c8fb6d9e932/0d2f5cf/src/design_round.rs` revealed two clarifying facts.

First, the `cargo mock deprecate` command operates on the *current round's* active CLs, not on arbitrary historical rounds. The implementation at `cmd_deprecate` (lines 143 to 222) matches against `Phase::Doc` / `Phase::Src` / `Phase::Topic` / `Phase::SrcPlan` / `Phase::Done` and dispatches to operations on whatever CL is currently active. There is no path argument; there is no mechanism for picking a specific historical round's CL to deprecate.

Second, the closed round 202605090011 (the one carrying the drift) has both its DOC and SRC CLs locked. The `current_phase` helper computes a phase from file-presence patterns in the design rounds directory; with a closed round and no active CL anywhere else, the phase is `Topic`. The `deprecate` command in `Phase::Topic` errors with `no changelist to deprecate (TOPIC phase)` (line 207 to 209 in the source).

Combined: the cargo mock CLI provides no path for retroactively deprecating a past round's locked CL. The "Option A" framing in PR #85's drift memo was based on a misreading.

## The discipline rule's actual framing

Reading `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md` more carefully:

> "Locked CLs are immutable."

> "Sketches retroactively: Round 1 and Round 2 did real sketching during apply... Going forward: [rules apply prospectively]. Retroactive sketches for past rounds are not required. The rule applies forward."

The rule names two artefact types:

1. Locked CLs (immutable; the audit trail keeps them as-is).
2. New sketches and CLs (the discipline applies prospectively).

The rule explicitly does NOT require retroactive editing of past rounds. The drift documented by PR #85 is itself the audit trail entry that satisfies the rule's spirit: the locked CL stays as it was at lock time, the shipped source is what shipped, and a sketch documents the gap so future readers see both.

## What this means for task #398

Task #398's literal description ("Reconcile round 202605090011 src CL wording with shipped validate_descriptor visibility") could be read three ways:

1. Literally update the CL prose (impossible per the immutability rule).
2. Document the drift so the audit trail is intact (PR #85 did this).
3. Run some specific reconciliation procedure that involves the mockspace CLI.

Reading (1) is ruled out by the immutability rule. Reading (3) is ruled out by the cargo mock CLI not providing a retroactive-deprecate path. Reading (2) is what PR #85 accomplishes and matches the discipline rule's prospective framing.

**Recommended resolution: close task #398 with PR #85 as the resolving artefact.**

The drift memo (PR #85) names the discrepancy, points at both the locked CL and the shipped source with line citations, and proposes a path that is now (per this sketch) understood to be unnecessary. The drift memo's existence is itself the audit-trail entry the discipline rule wants.

If op disagrees with this re-reading and intends a specific ceremony procedure for past-round reconciliation, op can redirect; the current state preserves all options. Closing #398 as resolved-by-PR-#85 is the agent's call given the source investigation.

## What this sketch does NOT do

- Does not edit the locked CL at `mock/design_rounds/202605090011/202605090011_changelist.src.lock.md`. The immutability rule forbids it.
- Does not change the shipped source at `mock/crates/hilavitkutin-extensions/src/host.rs:226`. The shipped visibility is correct.
- Does not run any `cargo mock` ceremony. None is available for this case.
- Does not modify PR #85's drift memo. That memo's Option A framing was wrong but the memo's primary artefact (documenting the drift) is correct and load-bearing.

## Future drift discipline

When a future round's source application drifts from its lock-time CL claim, the right pattern (per this investigation and the discipline rule):

1. The original CL stays locked and immutable.
2. A sketch under `mock/research/sketches/` documents the drift at the time it's discovered, naming the original CL by path and the shipped reality by source line.
3. If a future round genuinely supersedes the original (substantive design change), that future round's CL carries a `## Comparison to deprecated changelist` section per the deprecation-comparison lint, and the original CL is renamed to `.deprecated.md` via manual `git mv` + the new round's commit. This is the case the cargo mock CLI does NOT automate; it's a manual operation when the design genuinely warrants it.
4. If the drift is purely a wording mismatch with no design change behind it (as in #398), a documentation sketch is sufficient; no rename or new round is needed.

The drift documented by #398 is case 4: the visibility changed during apply for documented reasons, the rustdoc rationale is clear in source, and no design supersession is warranted. PR #85 is the right artefact.

## See also

- PR #85 (the drift memo) and `mock/research/202605251200_validate-descriptor-cl-drift.md`.
- PR #87 (the v2 phase machine field notes) and `mock/research/sketches/202605251400_v2-phase-machine-field-notes.md`.
- Workspace rule `cl-claim-sketch-discipline.md` (the discipline this sketch applies).
- `~/.cargo/git/checkouts/mockspace-d2db2c8fb6d9e932/0d2f5cf/src/design_round.rs` lines 143 to 222 (the `cmd_deprecate` impl this investigation read).
- Workspace task #398 (the task this closure recommends resolving).
