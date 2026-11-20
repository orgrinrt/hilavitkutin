# Default Implementations Behind Opt-In Features: Op's Call

Branch `work/hila-consumer-surface` carried loru's asks for a column release that frees, a memory provider that takes blocks back (`StaticFreeListMemory`) and a zeroed block on the memory provider contract, in round 202611200201-yszg. Two independent canon reads of the branch disagreed, so the question went to op as ticket `may_the_providers_crate_free_a_released_column_and_ship_a_no` (clause-dev `.shared/tickets/closed/`), filed 2026-11-20T03:40Z. The full reads are in `mock/research/notes/202611200500_review-pass-2-hila-consumer-surface.md` on the same branch.

## The question and his answer

The question: may the providers crate free a released column and ship a no-os provider that takes blocks back.

The options put to him:

- a: supersede the release row so release frees; keep `StaticFreeListMemory` and `allocate_zeroed` in `hilavitkutin-providers` as defaults over the contracts.
- b: release frees, but `StaticFreeListMemory` and `StaticBumpMemory` move out of the hilavitkutin family into the consumer or a crate outside it; `allocate_zeroed` stays.
- c: release stays a no-op and reclamation waits for the columnar engine; loru's freeing bump lives in loru.
- d: something else, in his words.

His answer, closed 2026-11-20T04:58Z, verbatim:

> Option a/b but make the default impls an opt in feature on their respective crates. Default to contracts only. The default impls we already have for other things should probably move there too and the readmes should mention the different principled features for impls to opt in optionally (there are different principles on the impls now so each needs a unified name)

## The intent, named apart from the words

The intent is that release frees, so the provisional row saying release is a no-op is superseded, and that every default implementation of a contract, the new ones and the ones that existed before, sits in its own crate behind a cargo feature a consumer turns on, with a build on default features carrying the contracts alone. Each principle an implementation rests on has one feature name, the same in every crate, and the readmes name those features. `allocate_zeroed` stays, as both options a and b keep it.

"should probably move there too" is read as part of the intent, since "default to contracts only" cannot hold while an implementation outside a feature remains. The parenthesis gives the reason for the unified names and is kept for provenance.
