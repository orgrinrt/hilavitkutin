**Date:** 2026-05-08
**Phase:** TOPIC
**Scope:** hilavitkutin-api
**Source topics:** task #333 (HILA-AUDIT-A4), round 202605042200 follow-up

# AccessSet arity solve: delete legacy flat-tuple A, retain canonical HList B

## Background

Round 202605042200 locked the typestate substrate at `Empty` plus `Cons<H, T>` with `#[marker]` `Contains` and `ContainsAll`. The engine's `SchedulerBuilder<Wus, Stores>` typestate, `WorkUnitBundle::AccumRead/Write` projections, the `read!` / `write!` macros, and `StoreBundle` all build on the HList shape exclusively.

Source under `mock/crates/hilavitkutin-api/src/access.rs` ships a parallel legacy encoding from v0.1: macro-generated flat-tuple impls for arities 1..=12 (`impl AccessSet for (T0, ..., TN)`, per-position `Contains` impls), a unit-tuple impl `impl AccessSet for ()`, and a hybrid bridge impl `impl<H, R, T> Contains<T> for (H, R) where R: Contains<T>` that lets nested-pair tuples participate as cons-cell substitutes. Findings file `mock/research/sketches/202605050503_accessset_arity/FINDINGS.md` settled candidate B as the going-forward shape; the legacy A impls were left in place by round 202605042200 but no consumer uses them anymore.

Task #333 closes the loop: delete the legacy A impls now that B is canonical, fix one latent bug in `Cons<H, T>::LEN`, and update the one test file that exercised the legacy LEN values. The round is the second tier-A architectural cleanup behind the kit-trait split (#330) and is the named load-bearing dependency for finalising that earlier work.

## Workspace sweep

The workspace was searched for every concrete bare-tuple AccessSet usage outside the file under edit. Result: zero hits in `vehje`, `viola`, or `mockspace`. The only call sites referencing `<(A, B) as AccessSet>` literals live in `tests/access_set.rs` of the api crate itself. Tests in `tests/work_unit.rs` and `tests/providers.rs` either use the `read!` / `write!` macros or take generic `<R: AccessSet>` bounds that work uniformly across the canonical HList impls. Nothing crosses repos.

## Decisions

### Decision 1: scope is full deletion of the legacy A encoding

Lines 40 to 189 of `mock/crates/hilavitkutin-api/src/access.rs` go away in their entirety. That covers:

- `impl AccessSet for ()` and the matching `sealed::Sealed for ()` impl (the arity-0 entry).
- The `impl_access_set!` and `impl_contains!` declarative macros.
- All arity 1 through arity 12 `impl AccessSet for (T0, ..., TN)` invocations and per-position `Contains` impls those macros emit.
- The hybrid cons-cell bridge `impl<H: 'static, R: 'static, T: 'static> Contains<T> for (H, R) where R: Contains<T>`.

Rationale: `no-legacy-shims-pre-1.0.md` licenses unconditional deletion of pre-release encodings with no installed-base obligation; the workspace sweep confirms zero external consumers; round 202605042200 already ratified HList as the canonical shape and the macros already emit Cons directly. Keeping the legacy impls alongside HList costs dual type identity (a bare `(A, B)` and a `Cons<A, Cons<B, Empty>>` both impl AccessSet but are distinct Rust types the trait solver does not unify), which the engine builder cannot accumulate from cleanly. One canonical shape is correct.

### Decision 2: AccessSet::LEN computes recursively on the HList shape

The current `impl AccessSet for Cons<H, T>` body sets `const LEN: USize = USize(0)`, which is wrong. Replace with:

```rust
impl AccessSet for Empty {
    const LEN: USize = USize::ZERO;
}

impl<H: 'static, T: 'static + AccessSet> AccessSet for Cons<H, T> {
    const LEN: USize = USize(T::LEN.0 + 1);
}
```

`T::LEN.0` accesses the inner `usize` of the recursive position; the substrate's `arvo::USize` already supports this in const contexts (round 202605021200 layer A). LEN values for any cons-list compute automatically from the recursive impl. Empty is zero. Single-element `Cons<X, Empty>` is one. Arbitrary depths follow.

LEN survives the deletion of legacy A because the engine and downstream consumers may want the cardinality of a typestate set at monomorphisation time without spelling out the depth. It is cheap to keep and obviously correct under the recursive impl.

### Decision 3: rewrite tests/access_set.rs in macro form

Replace the existing flat-tuple LEN assertions with macro-form equivalents:

```rust
use hilavitkutin_api::{AccessSet, read};

#[test]
fn len_empty() {
    assert_eq!(<read![]>::LEN, 0);
}

#[test]
fn len_one() {
    assert_eq!(<read![A]>::LEN, 1);
}

#[test]
fn len_two() {
    assert_eq!(<read![A, B]>::LEN, 2);
}
// ... up through arity 6
```

The macro form covers the same arity range as the legacy test (0 through 6) and validates two pieces of substrate at once: the macro expansion to nested `Cons` and the recursive LEN computation. The legacy `<(A, B,) as AccessSet>::LEN` form will not compile after Decision 1 lands, so the file cannot stay as-is. The macro form is more compact than spelling out `Cons<A, Cons<B, Empty>>` chains and exercises the API surface consumers actually write.

## Out of scope

- Set-difference operators (`Concat<L>` already lives in source; `Diff<L>` and similar are research items, not round-4 closure).
- Bitset fallback for `marker_trait_attr` (documented in `mock/research/marker_trait_attr_bitset_fallback.md`; activated only if the unstable feature becomes unviable).
- Type-level dedup on `Concat` (deferred per round 202605042200; tracked in `mock/research/sketches/202605050800_dedup_concat/FINDINGS.md`).
- Renames or moves of the substrate types. `Empty`, `Cons`, `Contains`, `ContainsAll`, `Concat` keep their current names and their current home in `hilavitkutin-api/src/access.rs`.

## Lock criteria

Doc CL covers the api crate's DESIGN.md.tmpl section on AccessSet (drop the "arities 0..=12" framing, state HList as the only encoding, document LEN recursion). BACKLOG.md.tmpl loses any line that promised the legacy shape would stay.

Source CL deletes the named lines, fixes LEN, rewrites the test file. Cargo check passes on `hilavitkutin-api` and every crate that depends on it (engine, kit, providers). Cargo test on `hilavitkutin-api` passes. Provider and work_unit test files compile unchanged.
