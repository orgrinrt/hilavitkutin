**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-providers tests/smoke.rs (one new test)
**Source topic:** PR #49 pr-reviewer-senior advisory finding F3 (#329)

# Topic: PR #49 review follow-up — add_kit composition test

The pr-reviewer-senior pass on PR #49 returned `CLEAR TO MERGE`
with four advisory findings. Three are discipline / informational
notes that do not warrant action. F3 names a small ergonomic gap:

> The new smoke test calls `InternerKit::<128, 8>.install(builder)`
> directly. The actual idiomatic surface this round delivers is
> `Scheduler::builder().add_kit(InternerKit::<...>)`. `add_kit` is
> exercised in `hilavitkutin-kit/tests/kit.rs` so end-to-end
> coverage exists, but no test in this PR composes the two
> together.

Closing the gap costs one line. The test compositions across the
two surfaces (api-level `BuilderResource<T>` bridging trait + the
engine's `add_kit` method which carries `K::Output:
BuilderExtending<Self>`) are nontrivial enough that a smoke test
exercising them together has positive maintenance value: the next
breakage hits at type-check time inside this file rather than at a
downstream call site.

This mini-round adds one new test to the existing smoke.rs and
nothing else.

## Decisions

1. **Add new test, do not replace the existing one.** The existing
   `internerkit_installs_via_scheduler_builder` test exercises the
   Kit's `install` method directly. The new test exercises
   `add_kit` calling `install` indirectly. Both are useful; both
   stay.
2. **Test name**: `internerkit_installs_via_add_kit`. Mirrors the
   shape of the existing `internerkit_installs_via_scheduler_builder`
   name.

## What this round does NOT do

- Does not address F1 (CL grep wording was too tight; this round's
  src CL leaves the grammar unchanged).
- Does not address F2 (the prior round's CL described a stub-builder
  fallback shape; that history is preserved).
- Does not address F4 (seal-strength is a workspace-wide question
  about all four sealed traits, deferred).

## Out of scope

- The four other findings stay deferred per local-pr-review-flow's
  "Reviewer report is advisory, not authoritative" framing.
