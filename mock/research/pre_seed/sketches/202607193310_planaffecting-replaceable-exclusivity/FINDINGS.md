# Sketch: PlanAffecting/Replaceable mutual exclusivity via negative_impls

**Date:** 2026-07-19
**Hypothesis:** a blanket negative impl in the trait-owning crate,
`impl<T: PlanAffecting> !Replaceable for T`, compiles on the pinned nightly
(nightly-2026-05-28) and structurally rejects any type implementing both marker
traits, closing the silent-skip hole panel memo 01 names (a type with both
traits could take the `replace_value` path and skip plan recompute).

**Outcome: WORKS**, both arms.

- Same-crate (`single/`): the blanket negative impl compiles; a type with both
  impls is rejected with E0751 ("found both positive and negative
  implementation"); single-trait types compile clean.
- Cross-crate (`api/` + `consumer/`), the real shape (traits in
  hilavitkutin-api, consumer implements on its own types): the downstream
  double impl is rejected with E0751 naming the upstream negative impl
  ("negative implementation in crate `sketch_api`"); legitimate downstream
  single impls compile clean. The #133556 coherence gap (negative impls not
  yet disarming coherence for ALLOWING overlap) does not bite: this use wants
  negative impls to FORBID, which E0751 delivers.

**Direction of the wall:** this shape makes `PlanAffecting` dominant: a
plan-affecting type can never opt into the cheap `replace_value` path. That is
the safe direction; the reverse wall (Replaceable dominant) would forbid
plan-affecting on app state instead, which is not the hazard.

**Cost of adoption:** one `#![feature(negative_impls)]` gate in
hilavitkutin-api's crate root (WATCH tier per `unstable-features.md`, allowed)
plus the one-line blanket impl. Alternatives if the gate is unwanted: a
supertrait bound (changes the traits' meaning) or a mockspace lint (advisory,
not structural).

**Unblocks:** the swap-semantics spec (#697, panel `202607193300`); the fork in
memo 01 section 2 now has empirical evidence for the structural option.
