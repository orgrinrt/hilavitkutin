# Sketch: v2 phase machine field notes

**Date:** 2026-05-25
**Hypothesis:** Direct experimentation with `cargo mock` (the v2 CLI) reveals enough of the phase machine's contract to execute a deprecate-and-replace ceremony in a single firing window.
**Outcome:** INCONCLUSIVE. Direct experimentation in one firing window surfaces enough surface area to identify the ceremony's mechanical steps but not enough to execute them without rework risk. The v2 phase machine demands more deliberate investigation than a single cron-firing window provides.
**Parent context:** workspace task #398 (`Reconcile round 202605090011 src CL wording with shipped validate_descriptor visibility`). The 2026-05-25 firing attempted Option A (deprecate-and-replace via mockspace flow) per the drift memo at `mock/research/202605251200_validate-descriptor-cl-drift.md`. This sketch captures what was learned before the agent paused the attempt.

## What this sketch lands

A concrete checklist of CLI subcommands, error modes, and side-effects observed during the firing. The next ceremony attempt (any repo) starts from this rather than from blank knowledge.

## Subcommand inventory (per task #560)

The `cargo mock` CLI surface (v2 Phase 5) shipped: `status / check / lock / unlock / deprecate / close / phase / task`. These are the eight verbs the v2 phase machine exposes.

Observed behaviour:

- `cargo mock` (no args): regenerates the agent files. Produces 108 generated files (per the run's tail line `generated 108 agent files`). Touches `.claude/` (gitignored) and `docs/` (tracked). The `docs/` touches are pure timestamp churn against unchanged source templates.
- `cargo mock --help`: drowns out the actual help text with regen output. Effective output access via `cargo mock --help 2>&1 | grep -A N "^Commands"` doesn't work as expected; the help formatter does not emit a clear `Commands:` section.
- `cargo mock status`: returns one line, `mockspace hooks: active`. Does not enumerate the current phase, active changelist, or pending work. Less informative than the v1 status surface (which named the current phase + active CL).
- `cargo mock deprecate --help`: errors with `no changelist to deprecate (TOPIC phase)`. The error is emitted before help text renders. Implies the deprecate subcommand only runs when a non-TOPIC phase is active (i.e. an active DOC CL or SRC CL exists).

## The phase machine's TOPIC phase

The "TOPIC phase" error from `cargo mock deprecate --help` reveals that the v2 phase machine treats an empty-or-no-active-round state as the `TOPIC` phase. Phases observed or implied:

- `TOPIC`: no active CL; only a topic file may exist or be opened.
- `DOC`: a DOC CL is active (per the workspace `branch-pr-flow.md` framing).
- `SRC`: a SRC CL is active.
- `DONE` (implied): post-close state.

The deprecate verb wants a non-TOPIC active phase. So the v2 flow for deprecating a locked CL must be: open a new round (transition out of TOPIC by writing a topic file or running a phase transition), then run deprecate against the locked file's path.

Per task #577 (`CLI wiring: mock phase {plan|apply|finish|replan}`), the phase transition verbs are `plan / apply / finish / replan`. These are sub-subcommands of `cargo mock phase`. The shapes:

- `cargo mock phase plan`: presumably opens a new round / advances from TOPIC to DOC.
- `cargo mock phase apply`: presumably advances DOC to DRAFT or SRC.
- `cargo mock phase finish`: presumably advances to DONE.
- `cargo mock phase replan`: presumably restarts a round under deprecation.

Per task #574 (the Phase 5 IO executor task that adds `advance_phase()` for all four verbs), these verbs map to concrete IO operations on the design_rounds tree.

## Regen churn handling

Every `cargo mock` invocation (including the help-related ones above) regenerates the agent file tree and updates timestamps in `docs/*.md` files. Concretely 19 tracked files modified in `docs/` after one `cargo mock` invocation, all pure timestamp diffs against unchanged source templates.

The clean approach during ceremony work:

1. Run cargo mock invocations as needed for the ceremony.
2. Stage only the ceremony-related files (the new topic, the deprecated rename, the replacement CL).
3. Before commit, `git restore docs/` to discard timestamp churn.
4. Commit the ceremony files.

This avoids polluting #398's PR (or any feature PR) with regen churn that doesn't reflect any source change. Regen-bundled commits (e.g. `chore: bump mockspace pin + regen docs`) ship separately when a mockspace pin bump or template change makes the regen substantive.

## Why this firing paused the #398 ceremony

The #398 attempt hit a procedural gap: opening a deprecate-and-replace flow requires opening a new round (TOPIC -> DOC -> ...) first, then deprecating from within the new round. The exact phase transitions, the file-layout expectations, and the lock criteria for the replacement CL all need walking through with the v2 phase machine's commands, and each `cargo mock` call adds regen churn that needs cleanup.

Doing this confidently in one firing window requires more concrete knowledge of:

1. What `cargo mock phase plan` produces (file shape, manifest entries, naming).
2. What `cargo mock phase apply` requires (does it want certain files staged? does it lint inputs?).
3. How `cargo mock deprecate` names its target (path arg? round-id arg? interactive prompt?).
4. How a replacement CL must reference the deprecated original (per the deprecation-comparison lint at `mock-workspace.md`).
5. What `cargo mock close` requires to succeed (all locks present? clean working tree?).

Each of these is one experimentation step; collectively they exceed one firing's careful execution budget once regen churn cleanup is factored in.

## What ships when the ceremony eventually runs

The drift memo's Option A list stays canonical:

1. New topic file at `mock/design_rounds/<new-timestamp>/<new-timestamp>_topic.reconcile-validate-descriptor-visibility.md`.
2. Deprecate the original via `cargo mock deprecate mock/design_rounds/202605090011/202605090011_changelist.src.lock.md` (or whatever its v2 arg shape is).
3. Author replacement CL at `mock/design_rounds/<new-timestamp>/<new-timestamp>_changelist.src.md` with the CHANGE block corrected to `fn validate_descriptor ADDED (public host-side helper)` plus rustdoc rationale matching shipped source, plus the `## Comparison to deprecated changelist` section the deprecation-comparison lint requires.
4. Lock the replacement: `cargo mock lock` (or `cargo mock lock <path>`; arg shape unclear).
5. Close the round: `cargo mock close`.
6. Stage ceremony files only; `git restore docs/` to discard regen churn.
7. Commit and open PR against `dev`.

## Recommendation

The next firing that attempts this should:

1. Run `cargo mock phase --help` first to learn the plan/apply/finish/replan shapes.
2. Run `cargo mock phase plan` to open a new round; observe what files are created.
3. Inspect those files and run `cargo mock phase apply` to advance, observing what changes.
4. Read the deprecate subcommand's actual arg shape from `cargo mock deprecate --help` (which needs an active non-TOPIC phase to surface real help).
5. Proceed through deprecate / lock / close one verb at a time, restoring docs/ at the end.

Reserving an entire firing window for this learning loop (rather than trying to bundle it with substantive ceremony work) is the right shape. The autonomous rule's "make the call" framing favors this exploration; the previous firing's "appropriate caution rather than evasion" framing applies here too.

## See also

- `mock/research/202605251200_validate-descriptor-cl-drift.md` (the drift memo Option A documents).
- Task #560 (`cargo mock` CLI subcommand surface).
- Task #577 (CLI wiring: mock phase plan/apply/finish/replan).
- Task #574 (Phase 5 IO executor: advance_phase verbs).
- Workspace rule `cl-claim-sketch-discipline.md` (the discipline reconciliation closes).
- Workspace rule `mock-workspace.md` (the deprecation-comparison lint requirement).
