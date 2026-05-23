# Running MIRI against hilavitkutin

The engine's atomic-ordering guarantees (Topic 3 S7 of design round
`202605101036`) are verified with MIRI under strict-provenance and
tree-borrows. The canonical invocation:

```bash
MIRIFLAGS="-Zmiri-strict-provenance -Zmiri-tree-borrows" \
  cargo +nightly miri test -p hilavitkutin
```

## What MIRI exercises

The atomic-ordering pairs from the S7 ordering table are covered by
`tests/loom_atomics.rs` (loom-gated) and the integration test
scaffolds (`tests/scheduler_run.rs`, `tests/adapt_subsystem.rs`).
Each test that touches cross-thread atomics is in scope.

MIRI's strict-provenance flag catches int-to-pointer casts that
break provenance. The engine's `Pin<&'arena PoolFrame>` lifetime
threading and `NonNull<AtomicUsize>` arena progress indirection are
the surfaces most at risk of provenance violations; running MIRI
flags any drift.

Tree-borrows is the second-generation aliasing model. It catches
unsound `&mut` aliasing that stacked-borrows might miss. The
engine's interior-mutability via `AtomicBool` / `AtomicU32` /
`AtomicUsize` is the surface tree-borrows verifies.

## CI integration

CI wiring is tracked under mockspace task `#203`. The flag
documentation lands here so a developer with a local nightly
toolchain can run MIRI manually pre-merge; the CI integration adds
the per-PR check when mockspace's CI orchestrator surface lands.

## Limitations

MIRI does not execute SIMD intrinsics (`core::arch::aarch64::*`,
`core::arch::x86_64::*`). The cfg-gated NEON / SSE2 EMA kernels in
`hilavitkutin-providers::adapt_ema` fall back to scalar under MIRI;
the parity test against the scalar path validates the structural
correctness without exercising the SIMD instructions themselves.

MIRI is also slow. Full `cargo miri test` runs for the engine
typically take 10x longer than `cargo test`. Run targeted tests
(`cargo miri test -p hilavitkutin loom_atomics`) when iterating on
a specific atomic-ordering change.
