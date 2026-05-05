# Findings: S2, AccessSet arity, three candidates

**Date:** 2026-05-05.
**Round:** 202605042200.
**Maps to:** Topic 4 sketch S2, open item O2 (#333).
**Outcome:** WORKS, both A (macro flat with `#[marker]`) and B (recursive HList with `#[marker]`) compile and produce usable error messages. C (const-generic length) deferred. The structural choice between A and B is qualitative.

## Setup

The current v0.1 hilavitkutin-api uses candidate A: macro-generated flat-tuple impls capped at arity 12. #333 names the cap as a band-aid. Round-4's typestate builder accumulates `Stores` and accumulated WU access through nested `.add_kit()` calls, so the cap will be exceeded at depth 2-3 of nested kits.

Three candidates were named in topic 4:

- **A. Macro flat impls.** `impl AccessSet for (T0, T1, ..., TN-1)`, `impl Contains<Ti> for (T0, T1, ..., TN-1)` per (arity, position). What v0.1 already ships.
- **B. Recursive HList AccessSet.** `Empty` terminator and `Cons<H, T>`. `Contains<X>` is recursive: head match, or tail recurses.
- **C. Const-generic length over a typed array.** `AccessSet<const N: usize>`.

## What got built

Two evolution paths:

`a_macro_flat/sketch.rs`. A natural shape, no position witness. Hits coherence overlap when two type parameters could substitute equal types (`Contains<T0> for (T0, T1)` overlaps `Contains<T1> for (T0, T1)` when T0 = T1). Required adding `#[marker]` to make the trait permit overlapping impls.

`a_macro_flat/sketch_v2_position.rs`. A with explicit position witness `Pi`. Compiles cleanly without `#[marker]`, but the user must thread the position through every call site (`require_contains_at::<Stores, M9, P9>()` instead of `require_contains_at::<Stores, M9>()`). The wrapper `ContainsAny<X>` that hides position is `where exists P. ContainsAt<X, P>`, which Rust does not express directly.

`b_hlist/sketch.rs`. Naive HList Contains, hits overlap (head match vs tail recurse).
`b_hlist/sketch_v2_assoc.rs`. Moves position to an associated type, still overlaps.
`b_hlist/sketch_v3_specialization.rs`. `feature(min_specialization)` does not break overlap (impls aren't structurally specialisable; `Cons<X, T>` and `Cons<H, T>` are syntactically equivalent shapes after trait-solver substitution).
`b_hlist/sketch_v4_marker.rs`. Adds `#[marker]`, natural shape compiles cleanly.

C (const-generic length) was not built. With both A and B working under `#[marker]`, the marginal value of exploring C is low; const-generic-length over a typed array is significantly more exotic, would require encoding type identity through const generics (which Rust does not support without `TypeId` or a registered-id table), and offers no obvious advantage over A or B at the arity scale that round-4 needs. Deferred.

## Critical finding: `#[marker]` is the unlock

Both v1 of A and the natural shape of B fail with E0119 coherence overlap. The fix in both cases is the same: declare `Contains` as `#[marker]` (gated by `feature(marker_trait_attr)`). Marker traits permit overlapping impls because they have no methods, so coherence has nothing to choose between.

This unblocks both candidates and is consistent with what v0.1 hilavitkutin-api already does:

```rust
// crates/hilavitkutin-api/src/access.rs
#[marker]
#[diagnostic::on_unimplemented(/* ... */)]
pub trait Contains<S>: AccessSet {}
```

`feature(marker_trait_attr)` is nightly. The substrate already accepts nightly per `arvo-compile-time-last.md` (compile-time paid once); marker_trait_attr is in the same bucket as `const_trait_impl`, `adt_const_params`, `generic_const_exprs`, etc., that the substrate adopts.

## Compile-time measurements (arity 12)

| Candidate | rustc emit-metadata time |
|---|---|
| A (flat with `#[marker]`) | ~0.05 s |
| B (HList with `#[marker]`) | ~0.04 s |

Both are effectively instant at arity 12. The measurements are not load-bearing at this scale; the structural difference matters for larger N.

A's macro generation produces O(N²) impls per arity (one AccessSet impl per arity, plus N Contains impls per arity). At N=64 this is ~64*64/2 = 2048 impls. The macro work and rustc's per-impl overhead grows with that count.

B's recursive structure produces O(1) impls regardless of arity (one head impl, one tail impl). The cost moves from impl-count to trait-solver search depth at the call site. For a list of length N, proving `Cons<...>: Contains<X>` walks the recursion N times in the worst case.

A scales by macro-generation cost; B scales by trait-solver search depth. Neither shows pathology at N=12. Empirical scaling at N=24, 48, 64 is left as a follow-up if the round-4 doc CL asks for it; the binary outcome (works / does not work) is settled.

## Error-message comparison (missing element on a 4-element set)

A v1 (flat with `#[marker]`):

```
error[E0277]: the trait bound `(M0, M1, M2, M3): Contains<M9>` is not satisfied
   --> sketch.rs:150:24
    = help: the following other types implement trait `Contains<X>`:
              `(T0, T1)` implements `Contains<T0>`
              `(T0, T1)` implements `Contains<T1>`
              ... and 22 others
note: required by a bound in `require_contains`
```

B v4 (HList with `#[marker]`):

```
error[E0277]: the trait bound `Empty: Contains<M9>` is not satisfied
   --> sketch_v4_marker.rs:71:24
help: the following other types implement trait `Contains<X>`:
   | impl<X, T: AccessSet> Contains<X> for Cons<X, T> {} `Cons<X, T>`
   | impl<X, H, T: AccessSet> Contains<X> for Cons<H, T> where T: Contains<X> {} `Cons<H, T>`
note: required for `Cons<M3, Empty>` to implement `Contains<M9>`
   = note: 3 redundant requirements hidden
   = note: required for `Cons<M0, Cons<M1, Cons<M2, Cons<M3, Empty>>>>` to implement `Contains<M9>`
```

A names the exact tuple shape that failed and the position-impls available, but the "and 22 others" is noisy.

B walks the recursion chain: it identifies `Empty: Contains<M9>` as the leaf failure and shows the recursion path through each Cons. The recursion-chain message is more verbose but more diagnostic, the user can see exactly why each layer failed. A `#[diagnostic::on_unimplemented]` annotation can mask this if desired.

Both are usable. A is shorter; B is more diagnostic.

## Structural comparison

| Axis | A (flat) | B (HList) |
|---|---|---|
| Arity cap | macro-generation cost; explicit cap | none structural; trait-solver search depth |
| Set construction | tuple literal `(M0, M1, M2, ...)` | nested `Cons<M0, Cons<M1, Cons<M2, ...>>>` |
| Set ops (union, diff) | per-(arity, op) macro generation; O(N²) ops impls per pair of arities | recursive impl per op; O(1) impls each |
| User ergonomics | tuple is short; `(M0, M1, M2)` reads naturally | Cons-chain is verbose; type alias is mandatory |
| Library complexity | large macro tables; mechanical to extend | recursive impls; structurally simpler |
| Trait-solver work per call | flat lookup over 2048 impls (at N=64) | recursive walk of length N |
| `#[marker]` requirement | yes | yes |

## Recommendation

Adopt **B (recursive HList) with `#[marker]`** for round-4. Reasons:

1. **No structural arity cap.** B's two impls cover any depth, eliminating the band-aid framing of #333.
2. **Set operations scale cleanly.** Union, intersection, and difference are O(1) impls each (recursive), instead of A's O(N²)-impls-per-operator macro tables.
3. **Code volume is much smaller.** v0.1's access.rs hand-codes ~1500 lines of macro-generated impls. B's recursive impl is dozens of lines.
4. **User ergonomics gap is mitigatable.** Cons-chain verbosity disappears behind a `set!` or similar macro that emits the type alias from a flat list.
5. **Error messages are comparable.** B's recursion-chain message is more verbose but more informative; A's "and 22 others" suppression is noisier without being cleaner.

Open follow-ups for the doc CL, not blockers:

- A friendly `set![M0, M1, ..., MN]` macro that emits the `Cons<M0, Cons<M1, ..., Empty>>` type alias.
- `#[diagnostic::on_unimplemented]` on B's Contains to keep the missing-element error friendly when the diagnostic chain matters.
- Set operators (union, intersection, difference) as recursive associated-type traits, plus the macro for the friendly surface.
- Naming: rename `Cons` to something domain-clearer (`AccessCons`, `Has`, etc.) if Cons reads like FP scaffolding.

## Pivot if B turns out empirically infeasible at deeper scales

Topic 4's failure-modes section names the pivot: AccessSet becomes a compile-time bitset over a registered store-table indexed at compile time. This is not the round-4 path; it is the round-5 fallback if depth-5+ kit nesting or set-op composition exposes pathology that does not surface at the depth-4 scale this sketch examined.

## Sketch-discipline note on candidate C

C (const-generic length over a typed array) was named in topic 4 but not built. The decision to skip it is recorded here, not in the topic file (which is frozen). The reasoning: A and B both pass the binary outcome at the arity round-4 needs, and exploring a structurally more exotic third option offers no leverage over the recommendation. If a future round needs to revisit AccessSet shape (the topic-4 pivot scenario), C should be one of the alternatives explored at that point.

## Path forward

S2 settled, recommendation B. Moving to S1 (deep-stacking typestate-builder) which uses S2's chosen B-shape as the substrate.

## Correction (post-audit)

**Date:** 2026-05-05.
**Audit reference:** `mock/design_rounds/202605042200_topic_round_4_audit.md` finding M1.

The S2 FINDINGS as originally written contained a factual error and an under-precise framing. Both are corrected below; the recommendation (adopt B, recursive HList with `#[marker]`) stands.

### Factual correction: line count

Line 114 above reads: "v0.1's access.rs hand-codes ~1500 lines of macro-generated impls."

This is wrong. The verified line count of `mock/crates/hilavitkutin-api/src/access.rs` at audit time (2026-05-05) is **188 lines**. The "1500 lines" figure was extrapolated from a misread of the macro-expanded surface; the actual source file is 188 lines and contains both the macro definition and the macro invocations together.

The corrected statement: B's recursive impl (the two `impl Contains<X> for Cons<...>` lines plus their `where` clauses) is roughly the same scale as the existing A-shape macro tables, not orders of magnitude smaller. The code-volume argument was overstated.

### Framing correction: v0.1 already ships A + B together

Line 114 also implied a binary "pick A or pick B" choice. The actual state of v0.1's `access.rs` is more nuanced:

- `#[marker] pub trait Contains<X>` is declared at line 32 of `access.rs`.
- The flat-arity Contains impls cover arities 1 through 12, generated by macro.
- The recursive `(H, R)` Cons-style impl on `(T, R)` tuple shape lives at line 188.
- All impls coexist under the `#[marker]` attribute, which lets the trait solver pick whichever resolution applies without coherence overlap.

Round-4's actual change is therefore not "switch from A to B" but **delete the flat-arity impls and rename the existing recursive primitives**. The "candidate A vs candidate B" sketch framing was misleading because v0.1 was already A + B in coexistence with the `#[marker]` attribute in place.

### What stands

The recommendation B (recursive HList with `#[marker]`) is correct; the typestate substrate adopts B as its single shape. Reasons 1, 2, 4, and 5 from the original recommendation block stand. Reason 3 (code volume) does not: the recursive impl is comparable in size to the flat-arity impls it replaces, not dozens vs. thousands.

The deprecated framing is preserved above (lines 108 to 116) as audit trail. This `## Correction (post-audit)` section is the new ground truth for any reader citing S2's findings.

### Cross-references

- `mock/crates/hilavitkutin-api/src/access.rs`. Verified 188 lines, both flat and recursive Contains impls coexist.
- `mock/design_rounds/202605042200_topic_round_4_audit.md`. Audit finding M1.
- `mock/research/sketches/202605050530_deep_stacking/sketch.rs`. S1 reuses access.rs's pattern.
- `mock/research/sketches/202605050700_deep_stacking_d5/sketch.rs`. S1b extends to depth 5; same pattern.
