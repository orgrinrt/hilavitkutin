# Sketch — EMA vectorisation, cfg-gated SIMD per target_feature

**Round:** 202605101036
**Topic:** 5 (adapt subsystem), Axis H (vectorised EMA update)
**Hypothesis:** Three cfg-gated paths (NEON on aarch64+neon, SSE2 on x86_64+sse2, scalar fallback) compile to the expected ISA instructions, with no autovec dependency. Each path uses `cfg(target_feature = "...")` per audit-2 m5 (not `cfg(target_arch)` alone), so the scalar fallback is reachable even on aarch64/x86_64 targets where the feature happens to be disabled.

If this hypothesis HOLDS, Axis H locks at the cfg-gated-from-day-one shape per workspace rule `arvo-always-optimal-internals.md` Kind 1.
If this hypothesis FAILS (cfg gating doesn't lower as expected, intrinsic types don't compose with `arvo::Norm`'s repr-transparent shape, or there's a target_feature interaction with the build profile we missed), the SIMD work moves to BACKLOG and we ship scalar-only for v1.

## Why this matters

Topic 5's EMA update is the per-fiber metric write that runs per-morsel or per-phase-boundary. It's batchable across fibers (the same EMA decay applies to N independent accumulators in parallel). Without vectorisation, this is N scalar mul-add operations per update; with NEON 4-way u32 lanes, it's N/4 vector ops. The win is real, but only if the cfg-gated dispatch works.

The audit-2 m5 correction is load-bearing: `cfg(target_arch = "aarch64")` is true on every aarch64 build, but if the toolchain target tuple has `-neon` (rare but possible), the NEON intrinsics aren't available and the build breaks. `cfg(target_feature = "neon")` is the correct gate. Per audit-2 the workspace rule is explicit about this.

## What goes in this directory

- `SKETCH.md` — this file.
- `Cargo.toml` — minimal one-crate sketch.
- `src/lib.rs` — three paths under three cfg gates + a dispatcher fn picking the active one.
- `FINDINGS.md` — outcome, with per-path asm evidence on the validated target.

## EMA shape

Fixed-point `Norm = UFixed<0, 32, S>` (Q0.32). Per-fiber EMA recurrence with alpha = 1/8:

```
new = old * (7/8) + sample * (1/8)
    = (old * 7 + sample) / 8
```

Storing as `u32` (the underlying primitive that `Norm` wraps as repr-transparent). The vector op is: shift-mul-add-shift over 4 lanes.

## How to run

```bash
cd mock/research/sketches/202605101036-ema-vectorisation
RUSTFLAGS='-C target-feature=+neon' cargo +nightly rustc --release --lib -- --emit=asm
# inspect target/release/deps/*.s for ema_update_batch body
```

Inspect for:

1. **NEON path on aarch64+neon:** body contains `umlal2`, `umlal`, `ushll`, `ushr`, or similar NEON arithmetic.
2. **Scalar fallback (build with `cfg(not(target_feature = "neon"))` overridden):** body contains plain `mul`, `add`, `lsr` scalar ops, no vector regs.
3. **No mixing.** Each compilation produces exactly one path's body.

## Outcome (to be filled)

WORKS | FAILS WITH ... | INCONCLUSIVE
