# Sketch: call-site Cap turbofish from usize-typed engine wrapper

**Date:** 2026-05-12
**Round:** 202605101036 (hilavitkutin runtime megaround, sub-round to wire arvo)
**Author context:** Attempts 1 to 3 of cap-typing the engine were ruled out
(see `project_round_202605111719_resume.md`, attempt 3: rustc 26 CPU-min
trait-solver hang on `cargo check -p hilavitkutin` after engine-wide cap
typing). Option 1 is the agreed alternative: engine keeps `<const MAX_X:
usize>` everywhere; only the three arvo-stub call sites in `plan/steps.rs`
wrap into `Cap` via turbofish.

## Hypothesis

A `usize`-typed engine wrapper

```rust
pub fn engine_wrapper<W, const MAX_UNITS: usize>(
    adjacency: &BitMatrix<W, { Cap(USize(MAX_UNITS)) }>,
) -> [NodeId; cap_size(Cap(USize(MAX_UNITS)))]
where
    W: BitSequence + BitAccess + BitLogic + Identity + Copy + Default,
    [(); cap_size(Cap(USize(MAX_UNITS)))]:,
{
    arvo_sparse::rcm_reorder::<W, { Cap(USize(MAX_UNITS)) }>(adjacency)
}
```

is well-formed under `feature(adt_const_params, generic_const_exprs,
const_trait_impl)`, given that

- `Cap` and `USize` both derive `ConstParamTy` and are `repr(transparent)`,
- `cap_size: Cap -> usize` is `const fn`,
- arvo's `rcm_reorder<W, const N: Cap>` is the real shipping signature
  (`mock/crates/arvo-sparse/src/rcm.rs:35`).

Three arvo entrypoints exercise three return-shape families:

| arvo fn | Signature core | Return |
|---|---|---|
| `rcm_reorder<W, const N: Cap>` | `&BitMatrix<W, N>` | `[NodeId; cap_size(N)]` |
| `block_diagonal<W, const N: Cap>` | `&BitMatrix<W, N>` | `(USize, [USize; cap_size(N)])` |
| `spectral_bisection<const N: Cap, F>` | `&[F; cap_size(N)]` | `(USize, [USize; cap_size(N)])` |

All three pass `Cap(USize(MAX_UNITS))` as the const argument.

## Acceptance criteria

The sketch passes when:

1. `cargo check` on the sketch crate produces zero errors.
2. Compile time is dominated by the cold arvo build, not the sketch's own
   trait-solver work. Specifically: the second-and-subsequent invocations
   of `cargo check` after a no-op edit finish in single-digit seconds.
3. The witness propagation count per wrapper fn is bounded by 1 (a single
   `[(); cap_size(Cap(USize(MAX_UNITS)))]:` clause), not the 4-to-14-witness
   blowup that attempt 3 ran into.

If acceptance fails, capture the failure mode in OUTCOME.md. Anticipated
risks documented inline.

## What this sketch does NOT prove

- Whether `[T; MAX_UNITS]` (engine-side) and `[T; cap_size(Cap(USize(MAX_UNITS)))]`
  (arvo-side) unify directly. They are numerically equal but not syntactically
  identical to rustc; the sketch returns the arvo-shape array and accepts that
  the caller (the real engine integration) will index into it through the
  same shape, or unsafe-transmute through the layout (both arrays are
  contiguous `T`-sized; the layout is identical by spec). The integration
  decision lives in the src CL, not the sketch.
- Whether the witness clause on the wrapper has to be re-stated by callers
  of the wrapper. The sketch tests the wrapper in isolation; the integration
  reality is that the wrapper IS the call site (one of three in `plan/steps.rs`),
  so this question does not apply at the option-1 boundary.
- Whether arvo's other algorithm crates (`arvo-comb`, etc.) work the same
  way. Only the three megaround-blocking stubs are sketched here.

## Outcome

See `OUTCOME.md` after `cargo check`.
