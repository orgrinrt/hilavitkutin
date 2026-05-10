# Findings — DispatchCodegen TAIT with realistic state capture

**Outcome:** **WORKS**. TAIT remains transparent under the full envelope: ten const-generic parameters on Cfg, captured-state closures with `&mut` borrows, multiple sealed `DispatchCodegen` impls coexisting. Per-Cfg constants bake as immediates. Zero indirect calls in any dispatch body.

## Test setup

- Toolchain: nightly with `feature(impl_trait_in_assoc_type)`.
- Profile: `release`, `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`.
- Target: aarch64-apple-darwin.
- Build: `cargo +nightly rustc --release --lib -- --emit=asm`.
- Envelope: 10 const generics (WU_COUNT, MORSEL_RECORDS, FIBER_SHAPE_KIND, three EMA depths, four derived ceiling-div constants); WU bodies do real work (column reads, fixed-point mixes, conditional branches); closure captures `&mut DispatchCtx` with two column borrows and two mutable accumulators; two sealed impls (`StandardCodegen` + `BenchInstrumentedCodegen`).

## Result A — three distinct entry-point symbols

```
__RNvCs..._27sketch_codegen_tait_capture18call_standard_test:
__RNvCs..._27sketch_codegen_tait_capture17call_standard_alt:
__RNvCs..._27sketch_codegen_tait_capture15call_bench_test:
```

Each `<Codegen, Cfg>` combination produces its own symbol.

## Result B — per-Cfg constants baked, distinct per instantiation

`call_standard_test` (TestCfg, `MORSEL_RECORDS = 256`):

- Two `#256` immediates in the body.
- Zero `#128` immediates.

`call_standard_alt` (AltCfg, `MORSEL_RECORDS = 128`):

- Two `#128` immediates in the body.
- Two `#256` immediates appear elsewhere as derived bounds-check constants.

The asymmetric appearance confirms the const-generic value `MORSEL_RECORDS` propagates into each monomorphisation independently. LLVM does not collapse the two instantiations into shared code.

## Result C — zero indirect calls in dispatch bodies

Across all three call-site bodies:

- 0 `blr` instructions.
- 0 `bl __` calls to user fns.

The closure body, all three `#[inline(always)]` WU bodies, and the per-Cfg constant propagation all dissolve into straight-line / branch-and-loop code with no abstraction boundaries surviving. The realistic envelope does not erode the transparency demonstrated by the toy sketch.

## Result D — captured `&mut` state composes cleanly

The closures capture `&mut DispatchCtx<'a>` with four borrows (`column_a`, `column_b`, `accumulator`, `branch_counter`). The asm shows these compose to plain pointer arithmetic in the optimised body. No vtable, no indirection, no escape-analysis-defeating pattern. The audit-2 C1 concern about `NonNull<AtomicUsize>` was about a different (cross-thread) pattern; for in-fiber `&mut` borrows the standard reference-as-pointer lowering applies and remains fully transparent.

## Result E — two sealed impls coexist without interference

`StandardCodegen` and `BenchInstrumentedCodegen` both implement `DispatchCodegen<TestCfg>`. The sealed family + min-specialization-friendly shape compiles cleanly, and each impl's `Self::CoreDispatch` resolves to its own anonymous TAIT type. The bench impl's extra branch-count snapshot at the loop tail is visible as additional asm after the morsel loop; the standard impl's body ends with just the accumulator write.

## Decision implication

**Audit-2 C3 closes WORKS.** The original `codegen-entrypoint-tait` sketch generalises to the realistic envelope. Topic 4 Axis A's lock (A3 + Rider 1 TAIT + Rider 2 sealed) holds without modification.

The combination locked: `feature(impl_trait_in_assoc_type)` + `pub trait DispatchCodegen<Cfg>: Sealed` + `type CoreDispatch = impl Fn(...)` + multiple coexisting sealed impls + ten-const-generic Cfg envelope. All transparent at release+LTO.

## What this sketch does NOT prove

- Compile-time scaling. The ten-const-generic envelope compiles fast in this sketch (~0.7s release build) because the instantiation count is small (three call sites). Real codegen will produce many more instantiations (one per (Codegen, Cfg) × per FiberShape × per WU tuple shape). Compile-time scaling under the full matrix is a separate concern; tracked under the megaround test bar, not this sketch.
- Rustc version pin. The TAIT shape compiled under the nightly toolchain available on 2026-05-10. The `feature(impl_trait_in_assoc_type)` gate is still nightly-only as of writing. The SRC CL must pin the rustc-toolchain.toml accordingly. Tracked in Topic 4 Axis A.

## Sketch retention

Stays committed forever per `cl-claim-sketch-discipline.md`. Together with the original `codegen-entrypoint-tait` sketch, this forms the two-step audit trail: simple sketch validates the mechanism, capture sketch validates the realistic envelope.
