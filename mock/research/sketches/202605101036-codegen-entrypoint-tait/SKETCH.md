# Sketch — codegen entrypoint via sealed trait with TAIT

**Round:** 202605101036
**Topic:** 4 (dispatch codegen)
**Axis:** A (codegen entrypoint surface)
**Hypothesis:** A sealed `trait DispatchCodegen<Cfg>` with `type CoreDispatch = impl Fn(...)` (TAIT via `feature(impl_trait_in_assoc_type)`) lowers to LLVM in a way that preserves bench-validated devirtualisation: call-sites through `<StandardCodegen as DispatchCodegen<Cfg>>::build(...)(ctx)` emit a *direct* `bl` (or inlined body), not an indirect `blr` through a struct field.

If this hypothesis HOLDS, Axis A locks at A3 + Rider 1 + Rider 2.
If this hypothesis FAILS (the trait+TAIT machinery introduces an opaque indirection LLVM can't prove through), Axis A relocks at A1 free-fn shape; the trait abstraction is shelved.

## Why this matters

Domain 17 L1540 (T6 bench): "Struct-field fn pointer arrays (12.6x penalty — LLVM can't prove contents through struct reference)". The trait abstraction risks introducing the same opacity if `type CoreDispatch` is realised as a fielded struct holding fn pointers. TAIT lets the impl emit a *singleton anonymous fn type* the trait system tracks at compile time, which the optimiser can prove through.

The test is: at the call site, does LLVM see the body of the emitted closure? If yes, devirtualisation holds. If no, the trait is opaque and we lose the bench-validated win.

## What goes in this directory

- `SKETCH.md` — this file (hypothesis, plan, outcome).
- `Cargo.toml` — minimal one-crate sketch with nightly toolchain pin.
- `src/lib.rs` — the sketch code itself.
- `FINDINGS.md` — written after running. WORKS / FAILS / INCONCLUSIVE per `cl-claim-sketch-discipline.md`.

## How to run

```bash
cd mock/research/sketches/202605101036-codegen-entrypoint-tait
cargo +nightly build --release
cargo +nightly asm --lib --release sketch::call_through_trait 2>/dev/null \
    | grep -E "bl |blr |jmp"
```

The `bl` / `jmp` count and target should match a direct call to the inner fiber body. A `blr` (or x86 `call *reg`) is a fail signal.

Alternative without `cargo asm`:

```bash
cargo +nightly rustc --release -- --emit=llvm-ir
# inspect target/release/deps/*.ll for the call_through_trait fn
```

Look for `call void @sketch::standard_codegen::...` (direct) vs `call void %1` (indirect through register).

## Outcome (to be filled)

WORKS | FAILS WITH ... | INCONCLUSIVE

The single deliverable is the `FINDINGS.md` file recording outcome + expected next step.
