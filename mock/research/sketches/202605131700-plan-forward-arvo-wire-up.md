# Plan forward: arvo wire-up at the three plan/steps.rs stubs

**Date:** 2026-05-13
**Status:** all blocking unknowns sketched and proven; one mechanical gap remains (S6) and can be resolved during apply.

Backed by four committed sketches:

| Sketch | Question | Verdict |
|---|---|---|
| `202605121400-call-site-cap-turbofish` | Does `cap_of` bridge the `usize -> Cap` const-arg conversion? | YES, via `const fn cap_of`. Inline tuple-struct form is rejected. |
| `202605131500-witness-propagation` | Does the witness clause cause super-linear trait-solver work when propagated through engine-shaped caller chains? | NO. 3-level chain, 11 const generics, 10-generic return type, 3 concrete monomorphisations: warm 2.48s, cold 20s. |
| `202605131530-csr-to-bitmatrix` | Can the engine's CSR `DependencyGraph` be converted to arvo's `BitMatrix` at the wrapper site, including UnitId/NodeId width adaptation? | YES. `transmute_copy(UnitId -> u32)` then `NodeId::new(USize(u32 as usize))`. End-to-end compiles. |
| `202605131600-array-reshape-consumed` | Can arvo's `[NodeId; cap_size(cap_of(MAX_UNITS))]` reshape to the engine's `[UnitId; MAX_UNITS]` AND be consumed by downstream engine code? | YES, per-element copy with width conversion. CAUGHT FiberId u8/u16 size mismatch via const-block assertion; fix applied. |

## What is proven

1. `const fn cap_of(n: usize) -> Cap` is accepted in generic-const-arg position. The inline `{ Cap(USize(n)) }` form is not.
2. A single witness clause `where [(); cap_size(cap_of(MAX_UNITS))]:` on a stub propagates up through `compute_execution_plan` without trait-solver blowup. The attempt-3 hang was caused by sweeping witnesses over EVERY MAX_X (11 const generics × N callers); this approach uses ONE witness over ONE MAX_X.
3. The CSR-to-BitMatrix conversion is straightforward (O(N + E)) and fits inside the wrapper body.
4. UnitId is u32-shaped (Warm at 16 bits picks u32). FiberId is u16-shaped (Warm at 7 bits picks u16). Both confirmed via `const _: () = { assert!(size_of...) };` blocks at compile time.
5. The three reshape patterns (per-element copy with width conversion for rcm + spectral; scalar projection for block) all compile end-to-end with the caller-side consumer reading the wrapper's return.

## The one remaining mechanical gap (S6, resolves during apply)

The spectral wrapper needs a real Fiedler vector. The S5 sketch passes a zero vector to focus on the reshape; the real wrapper composes:

```rust
use arvo_spectral::{fiedler_vector, SparseLaplacian};
let lap = SparseLaplacian::from(&matrix);
let fiedler: [Fl; cap_size(cap_of(MAX_UNITS))] = fiedler_vector(&lap, sigma, iterations);
let (class_count, per_node_class) = arvo_spectral::spectral_bisection::<...>(&fiedler);
```

The `sigma` (upper bound on `lambda_max(L)`) and `iterations` parameters are policy. Two options:
- **Substrate defaults for now**: `sigma = Fl::from_constant::<{ USize(8) }>()`, `iterations = USize(64)`. These match arvo-spectral's own example values and ship the wrapper without RunCfg dependency.
- **RunCfg policy fields**: add `type SpectralSigma: ...`, `const SPECTRAL_ITERATIONS: USize` to RunCfg. This is the long-term shape; defer to a separate round.

Pick substrate defaults for the immediate wire-up; document the deferral inline. Not a sketch-level question; ten lines of arvo-spectral API.

## Apply plan (step by step)

The wire-up edits `mock/crates/hilavitkutin/src/plan/steps.rs` plus a one-line witness add at `mock/crates/hilavitkutin/src/plan/mod.rs:190` (the `compute_execution_plan` signature). All other engine fns stay usize-typed.

### Step 1: helpers in `plan/steps.rs`

Add a private module at the top of `steps.rs`:

```rust
mod arvo_bridge {
    use arvo::{Bool, Cap, Hot, Identity, USize};
    use arvo_bitmask::{BitMatrix, NodeId, cap_size};
    use arvo_bits::Bits;
    use hilavitkutin_api::{FiberId, UnitId};

    pub(super) const fn cap_of(n: usize) -> Cap { Cap(USize(n)) }

    pub(super) fn unit_id_to_usize(u: UnitId) -> usize {
        let raw: u32 = unsafe { core::mem::transmute_copy(&u) };
        raw as usize
    }

    pub(super) fn usize_to_unit_id(n: usize) -> UnitId {
        let raw_u32 = n as u32;
        unsafe { core::mem::transmute_copy(&raw_u32) }
    }

    pub(super) fn usize_to_fiber_id(n: usize) -> FiberId {
        let raw_u16 = n as u16;
        unsafe { core::mem::transmute_copy(&raw_u16) }
    }

    const _: () = {
        assert!(core::mem::size_of::<UnitId>() == core::mem::size_of::<u32>());
        assert!(core::mem::size_of::<FiberId>() == core::mem::size_of::<u16>());
    };

    // csr_to_bitmatrix body verbatim from S3 sketch.
}
```

The size assertions sit alongside the conversions; any future strategy-table change that resizes UnitId or FiberId surfaces here as a compile error.

### Step 2: rewrite the three stubs

For each stub, replace the passthrough body with the wrapper body. New signature gains one where-clause; everything else stays usize-typed.

**rcm_reorder**:

```rust
pub fn rcm_reorder<
    const MAX_UNITS: usize,
    const MAX_EDGES: usize,
>(
    graph: &DependencyGraph<MAX_UNITS, MAX_EDGES>,
    _topo: &[UnitId; MAX_UNITS],
) -> [UnitId; MAX_UNITS]
where
    [(); arvo_bitmask::cap_size(arvo_bridge::cap_of(MAX_UNITS))]:,
{
    let matrix = arvo_bridge::csr_to_bitmatrix::<MAX_UNITS, MAX_EDGES>(graph);
    let arvo_out = arvo_sparse::rcm_reorder::<Bits<64, Hot>, { arvo_bridge::cap_of(MAX_UNITS) }>(&matrix);
    let mut out: [UnitId; MAX_UNITS] = [UnitId::ZERO; MAX_UNITS];
    let mut i = 0;
    while i < MAX_UNITS {
        out[i] = arvo_bridge::usize_to_unit_id(arvo_out[i].0.0);
        i += 1;
    }
    out
}
```

**block_diagonalise**: same shape, scalar projection of `block_count` to `Bool`.

**spectral_partition**: builds `SparseLaplacian`, calls `fiedler_vector` with substrate-default sigma+iterations, then `spectral_bisection`, reshapes the class-id array into `FiberGrouping`.

### Step 3: witness clause at `compute_execution_plan`

Add the one where-clause line:

```rust
pub fn compute_execution_plan<
    const MAX_UNITS: usize,
    ...
>(...)
where
    [(); arvo_bitmask::cap_size(arvo_bridge::cap_of(MAX_UNITS))]:,
{
    ...
}
```

The bridge module's path is `crate::plan::steps::arvo_bridge`. Make `cap_of` and `cap_size` reachable from `mod.rs` either via re-export or by inlining the bound expression to use the arvo paths directly (`arvo_bitmask::cap_size(arvo::Cap(arvo::USize(MAX_UNITS)))` — wait, that's the rejected inline form). The witness clause's CONTENT must use the same const-fn form, so re-export `cap_of` from `steps.rs` or move `cap_of` to a shared location like `crate::plan::cap_bridge` for reachability.

### Step 4: cargo check incrementally

After step 2 for rcm only (the simplest), run `cargo check -p hilavitkutin`. Expect warm-cache success in single-digit seconds based on S2 evidence. If it blows past 60 seconds, abort and revert; the witness propagation is not behaving as the sketch predicted.

Then add block, check again. Then spectral, check again. Three independent checkpoints.

### Step 5: commit per stub

One commit per stub. Each commit is independently revertable. If the third blows up rustc, the first two stay landed.

## Abort triggers

Pivot to path 2 (arvo-side `*_usize` adapter shims) if:

1. `cargo check -p hilavitkutin` warm time exceeds 60s on any single-stub state.
2. Concrete consumer apps that bind `compute_execution_plan::<256, ..., ...>` show super-linear scaling.
3. The witness clause on `compute_execution_plan` causes any unrelated test in the crate to fail to compile.

Pivot recipe: open an arvo round for `rcm_reorder_usize<const MAX_UNITS: usize>(...)` shims that wrap into Cap internally. Engine side stays pure usize. Sketch S2 evidence is no longer load-bearing for the engine; the witness lives entirely inside arvo.

## What this plan does NOT cover (defer to separate rounds)

- **RunCfg policy for spectral sigma + iterations**: substrate defaults ship now; consumer-tunable policy is a separate round once RunCfg's policy axis lands (Pass 6 follow-up).
- **Other arvo algorithms** (arvo-graph, arvo-comb) at non-stub call sites in the engine: only the three megaround-blocking stubs are in scope.
- **MAX_FIBERS clamping behaviour** when arvo emits a class id ≥ MAX_FIBERS: the sketch chose saturating-clamp; the engine policy may want to error. Decide at apply time; one line change either way.

## File touched-list (precise)

- `mock/crates/hilavitkutin/src/plan/steps.rs`: add `arvo_bridge` module + rewrite three stub bodies + add where-clauses on three stub signatures.
- `mock/crates/hilavitkutin/src/plan/mod.rs`: add one where-clause on `compute_execution_plan`.

Two files. Branch stays `feat/runtime-megaround-202605101036`.

## Src CL section to add

Add a new section in `mock/design_rounds/202605101036_changelist.src.md` after the existing Pass 2.5 (reverted) block:

```markdown
## Pass 2.5b: stubs wired to arvo via call-site `cap_of` bridge

Backed by sketches `202605121400`, `202605131500`, `202605131530`,
`202605131600`. The path-2 (arvo-side adapter shim) framing in the
prior Pass 2.5 section is no longer the default; path 1 with a
single witness clause per affected fn is proven viable. ...

### CHANGE: arvo_bridge helper module in steps.rs
File: mock/crates/hilavitkutin/src/plan/steps.rs:33
Verification: `grep -q "mod arvo_bridge" mock/crates/hilavitkutin/src/plan/steps.rs`

### CHANGE: rcm_reorder body now calls arvo_sparse::rcm_reorder
File: mock/crates/hilavitkutin/src/plan/steps.rs:210
Verification: `grep -A 20 "pub fn rcm_reorder<" mock/crates/hilavitkutin/src/plan/steps.rs | grep -q "arvo_sparse::rcm_reorder"`

... (one CHANGE block per file/function per cl-claim-sketch-discipline.md)
```

The src CL is currently in SRC phase (not locked); these blocks can be added before `cargo mock lock`.
