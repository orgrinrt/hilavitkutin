# Sketch — DispatchCodegen TAIT with realistic state capture

**Round:** 202605101036
**Topic:** 4 (dispatch codegen), Axis A — audit-2 C3 follow-up
**Hypothesis:** The `trait DispatchCodegen<Cfg>` with `type CoreDispatch = impl Fn(...)` (TAIT) still lowers transparently when:

1. The trait has the full envelope of const generics the real codegen needs (ten parameters: WU count, morsel records, fiber shape kind, three EMA history depths, plus four derived ceiling-div constants).
2. The TAIT-bound closure captures realistic state: a tuple of WU bodies, an `AdaptSidecar` reference, a `MorselWindow` reference.
3. Multiple sealed impls (`StandardCodegen`, `BenchInstrumentedCodegen`) coexist and each produces independently optimised code.

If this hypothesis HOLDS, audit-2 C3 closes; the original `codegen-entrypoint-tait` sketch's "WORKS" result generalises to the full envelope.
If this hypothesis FAILS (LLVM stops constant-folding through the captured closure, or TAIT projection times out on the const-generic envelope, or the sealed family + min_specialization interact badly), the Axis A lock is at risk and we need to either narrow the envelope or fall back to A1 free-fn shape.

## Why this matters

The original `codegen-entrypoint-tait` sketch validated TAIT with three trivial `#[inline(always)]` WU bodies and zero captured state. Audit-2 C3 flagged: "the toy sketch may not cover what real codegen actually emits; LLVM can fold three inline-always closures into a polynomial but real WU bodies do real work and the closure captures real state."

This sketch raises the bar:

- Ten const-generic parameters (matching the real `Cfg` shape).
- WU bodies that do non-trivial work (column reads, fixed-point arithmetic, conditional branches).
- The TAIT closure captures both the WU tuple AND mutable state references.
- Two coexisting sealed impls.

The test is: does each `<Codegen as DispatchCodegen<Cfg>>::build(...)` call site emit code where the inner loop is fully visible to the optimiser (no `blr`, no struct-field indirection, constants from `Cfg` baked as immediates)?

## What goes in this directory

- `SKETCH.md` — this file.
- `Cargo.toml` — nightly toolchain, feature-gated.
- `src/lib.rs` — the realistic-envelope sketch.
- `FINDINGS.md` — outcome + decision implication.

## How to run

```bash
cd mock/research/sketches/202605101036-codegen-tait-capture
cargo +nightly rustc --release --lib -- --emit=asm
ls target/release/deps/*.s
```

Inspect each `.s` for:

1. **Distinct symbols per (Codegen, Cfg) instantiation.**
2. **Constants from Cfg baked into immediates in each body.**
3. **No `blr` to closure address.** Direct `bl` or fully-inlined body only.
4. **No struct-field loads of fn pointers inside the loop.**

## Outcome (to be filled)

WORKS | FAILS WITH ... | INCONCLUSIVE
