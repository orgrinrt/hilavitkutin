**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Sweep em-dashes (`—`) and en-dashes (`–`) from doc comments and inline comments across all hilavitkutin source files. Replace with grammatical alternatives (period+colon for clause separators; comma for asides; "to" for ranges).
**Source topics:** Senior PR reviewer flagged pre-existing em-dashes in `plan/dirty.rs` and `plan/phase.rs` doc comments during PR #56 review. Discovery widened to all hilavitkutin crates. Task #352.

# Topic: em-dash sweep across hilavitkutin source

Workspace `writing-style.md` is hard: zero em-dashes in any written content (prose, doc comments, inline comments, commit messages). Lint gate at commit/push level. The mock/agent/ generated docs are gated; in-repo source is not yet but should be.

109 em-dash and en-dash occurrences existed across hilavitkutin crates pre-sweep. Mostly clause separators in doc comments. Mechanical replacement.

## Replacement strategy

- ` — ` (space + em-dash + space): replaced with `: ` (colon + space). Most em-dashes are clause separators introducing explanation; colon reads naturally.
- Bare `—` without surrounding spaces (rare, mostly in `lint:allow` reasons): replaced with `, ` (comma + space).
- ` – ` (space + en-dash + space): replaced with ` to ` (range word).
- Bare `–`: replaced with `-` (hyphen).

## Decision

Mechanical replacement. No semantic changes. Verified `cargo +nightly check` clean post-sweep.

## Per-rule compliance

- `writing-style.md`: enforced. Zero em-dashes remain.
- `cl-claim-sketch-discipline.md`: bulk text replacement counts as a "trivial mechanical sweep" per the rule's exception for typo/formatting fixes. SRC CL records the file list with verification command.
