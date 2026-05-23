# Sketch — FiberShape: Sealed as generic parameter

**Round:** 202605101036
**Topic:** 4 (dispatch codegen), Axis D + Topic 3 amendment
**Hypothesis:** Promoting `FiberShape` from a runtime enum to a sealed trait family lets the dispatch codegen monomorphise per shape: `fn dispatch<S: FiberShape>(ctx)` emits N specialised bodies (one per `impl FiberShape for ShapeKindX`), each with shape-specific constants (stride, prefetch distance, alignment) baked in at compile time. No runtime shape branch in the hot loop.

If this hypothesis HOLDS, Topic 4 Axis D locks at D2 + Topic 3 gains the `FiberShape: Sealed` amendment.
If this hypothesis FAILS (the trait machinery or const-method interaction prevents monomorphisation or hides shape-specific constants from constant-folding), D2 is shelved and dispatch reverts to a runtime shape match.

## Why this matters

Topic 4 Axis D's core question: where does the per-fiber shape discrimination cost get paid?

- **D1** (runtime enum match): shape branch in every dispatch call. 4-way switch lowers to either a jump table (cache miss + indirect branch) or chained compares.
- **D2** (compile-time generic): shape baked into the monomorphised dispatch fn pointer. Zero runtime cost; the entire fiber loop sees only its shape's constants.

The polka-dots Domain 17 bench (T6 L1540) measured 12.6x penalty for indirect dispatch over a struct field of fn pointers. Shape-match-then-call has the same shape opacity unless LLVM constant-folds the match. D2 sidesteps the question entirely: there is no shape match, only a direct call to the per-shape body.

The test is: does `dispatch::<Sequential>` compile to a body that uses Sequential's stride constant directly, *not* a load from a generic table indexed by shape kind? And do both `dispatch::<Sequential>` and `dispatch::<Strided>` compile to distinct, independently-optimised bodies?

## What goes in this directory

- `SKETCH.md` — this file.
- `Cargo.toml` — minimal one-crate sketch, nightly profile.
- `src/lib.rs` — sealed `FiberShape` trait + four impls + a dispatch fn generic over `S: FiberShape`.
- `FINDINGS.md` — written after running.

## How to run

```bash
cd mock/research/sketches/202605101036-fibershape-typing
cargo +nightly rustc --release --lib -- --emit=asm
ls target/release/deps/sketch_fibershape_typing-*.s
```

Inspect the `.s` for:

1. **Distinct symbols per shape.** Symbols like
   `dispatch_per_shape::<Sequential>` and
   `dispatch_per_shape::<Strided>` must both exist; if only one exists
   the monomorphisation collapsed.
2. **Shape constants inlined.** Sequential's `STRIDE = 1` must appear
   as `#1` immediate in its asm body; Strided's `STRIDE = 8` as `#8`.
   No `ldr` of a "shape table" constant.
3. **No shape-kind branch.** Each per-shape body has zero
   compare-then-branch on a runtime shape tag.

## Outcome (to be filled)

WORKS | FAILS WITH ... | INCONCLUSIVE
