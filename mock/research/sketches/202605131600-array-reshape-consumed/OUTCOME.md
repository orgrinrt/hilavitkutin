# Outcome S5: end-to-end consumption with array reshape

**Date:** 2026-05-13
**Status:** HYPOTHESIS CONFIRMED (with one bug caught and fixed)

## Verdict

The full end-to-end consumption path is shape-stable across all three
stubs. Engine input flows through the wrapper, into arvo, back through
the wrapper's reshape, into the engine consumer's read site. No
intermediate discarding; the caller-side reads exercise the real
consumer shape.

| Run | Wall clock |
|---|---|
| Cold (after fixes) | ~19 s |
| Warm `cargo check` | 2.98 s |

Three concrete monomorphisations at `MAX_UNITS = 64`, `128`, `256`
all compile and force full trait-solver work through the wrapper
bodies, the reshape loops, and the caller's consumption sites.

## What the sketch caught (bug worth recording)

The first iteration assumed FiberId is u8-sized because the logical
width is 7 bits. The const-block size assertion immediately failed:

```
error[E0080]: evaluation of `_` failed here
  --> src/lib.rs:113
  |  assert!(size_of::<FiberId>() == size_of::<u8>())
```

The actual size is u16. The engine's existing code at
`plan/steps.rs:288-292` uses `u16` for the FiberId transmute_copy,
confirming u16 is the right container. Warm at 7 bits picks u16, not
u8, because the substrate's storage projection table favours wider
containers for codegen quality.

Had we proceeded with the deferral framing and written the wire-up
without S5, the FiberId reshape would have shipped with a 1-byte
read where 2 bytes are needed. Symptoms would have been silent data
corruption on the upper byte (depending on the platform's endianness),
caught only by integration testing under load. The cost of finding
and fixing it then versus now is at least an order of magnitude.

## The three reshape stories (proven)

### rcm: arvo array -> engine array, per-element with width conversion

```rust
let arvo_out: [NodeId; cap_size(cap_of(MAX_UNITS))] = arvo_call(...);
let mut out: [UnitId; MAX_UNITS] = [UnitId::ZERO; MAX_UNITS];
let mut i = 0usize;
while i < MAX_UNITS {
    let raw_usize = arvo_out[i].0.0;        // NodeId -> USize -> usize
    out[i] = usize_to_unit_id(raw_usize);   // usize -> u32 -> UnitId
    i += 1;
}
```

Strategy A (per-element copy) chosen over Strategy B (whole-array
transmute) because element sizes differ: NodeId = USize = 8 bytes
on 64-bit; UnitId = u32-container = 4 bytes. Whole-array transmute
would be unsound. Per-element copy compiles, the runtime cost is
O(MAX_UNITS) once per plan, which is trivial.

The indexing `arvo_out[i]` with `i: usize` bounded by `i < MAX_UNITS`
against an array of length `cap_size(cap_of(MAX_UNITS))` is sound
because the two constants are numerically equal; rustc accepts the
indexing because the bounds check is runtime, not requiring const-
arg unification at compile time. LLVM folds the comparison away
since both sides are compile-time constants.

### block: arvo tuple -> engine Bool, scalar projection

```rust
let (block_count, _per_node) = arvo_sparse::block_diagonal::<W, N>(&matrix);
if block_count.0 >= 1 { Bool::TRUE } else { Bool::FALSE }
```

The array half of arvo's return is discarded by design: the engine's
`block_diagonalise` consumes only the feasibility decision, not the
per-node block assignment. If a future engine round wants the
assignment, that's a separate consumption path; the current scalar
projection is correct for the current consumer.

### spectral: arvo array -> engine FiberGrouping, per-element with width

```rust
let (class_count, per_node_class) = arvo_spectral::spectral_bisection::<N, F>(&fiedler);
let mut grouping = FiberGroupingLike::new();
let mut i = 0usize;
while i < MAX_UNITS {
    let class_id = per_node_class[i].0;
    let bounded = if class_id < MAX_FIBERS { class_id } else { MAX_FIBERS.saturating_sub(1) };
    grouping.assignment[i] = usize_to_fiber_id(bounded);  // usize -> u16 -> FiberId
    i += 1;
}
grouping.fiber_count = if class_count.0 <= MAX_FIBERS { class_count } else { USize(MAX_FIBERS) };
```

Class-id-to-FiberId conversion goes through u16 per the size
assertion. The MAX_FIBERS cap is enforced inline; a class_id from
arvo that exceeds the engine's FiberGrouping shape gets clamped
to the last fiber. The real engine policy may want to error instead;
that's a policy choice, not a shape question.

## What about the dummy fiedler vector?

The sketch passes a zero-initialised fiedler to `spectral_bisection`.
That's wrong for a real engine call (the spectral algorithm needs
the actual Fiedler vector of the graph's Laplacian). The sketch
ignores this because the QUESTION under test is the reshape, not
the spectral correctness. The real engine wrapper composes:

```rust
use arvo_spectral::{fiedler_vector, SparseLaplacian};
let lap = SparseLaplacian::from(&matrix);
let fiedler = fiedler_vector(&lap, sigma, iterations);
let (class_count, per_node_class) = arvo_spectral::spectral_bisection(&fiedler);
```

The `sigma` and `iterations` parameters are policy and need to come
from RunCfg eventually. For the immediate wire-up they can be
substrate-default constants. Not in scope for S5; the reshape
question is fully answered.

## Acceptance scorecard

| Criterion | Threshold | Result |
|---|---|---|
| All three reshapes compile end-to-end | required | PASS |
| Engine consumer reads from wrapper return | required | PASS (compute_plan_proxy reads each) |
| Strategy choice documented | required | PASS (Strategy A throughout; B was unsound) |
| Warm `cargo check` under 8 seconds | required | 2.98 s |
| Layout-soundness assertions cover unsafe | required | PASS (UnitId == u32, FiberId == u16) |
| Bug catch from disciplined sketching | bonus | FiberId u8/u16 mismatch caught and fixed |

All criteria pass plus the bonus catch. The reshape path is shape-
stable; the wire-up can proceed.
