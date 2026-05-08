**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** 6 grammatical polish fixes to clause-hinge em-dash sites that round 202605050400's mechanical bare-em-dash-to-comma rule mishandled. Senior-review-driven mini-round on the same branch as 202605050400 per `branch-pr-flow.md` multi-rounds-per-branch pattern.

# Topic: em-dash polish (round-202605050400 follow-up)

PR #57 senior reviewer of round 202605050400 flagged 6 sites where the mechanical bare-em-dash-to-comma rule produced stranded `, \n<continuation>` patterns. End-of-line em-dashes act as clause hinges, not asides; the comma substitution leaves the sentence reading mid-thought.

## Sites

- `mock/crates/hilavitkutin-build/src/axis.rs:34`
- `mock/crates/hilavitkutin-build/src/guards.rs:10`
- `mock/crates/hilavitkutin-ctx/src/lib.rs:123`
- `mock/crates/hilavitkutin-str/src/interner.rs:12`
- `mock/crates/hilavitkutin/src/thread/class.rs:5`
- `mock/crates/hilavitkutin/src/thread/mod.rs:57`

## Decision

Replace each stranded `, ` (introduced by round 202605050400) with the appropriate clause separator: `;` for tightly coupled clauses, `.` for clean clause break. Capitalise the continuation where a period was inserted.

## Per-rule compliance

- `writing-style.md`: zero em-dashes restored, period/semicolon/colon used per clause-hinge meaning.
- `cl-claim-sketch-discipline.md`: 6 concrete `## CHANGE:` blocks in the SRC CL.
