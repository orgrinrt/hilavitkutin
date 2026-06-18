# Sketch findings: GATE-2 §9 per-core accumulator region + merge

**Date:** 2026-06-07
**Round:** 202606071933 (GATE-2 accumulator unit-outer threaded path, deviation §9)
**Sketch:** `percore_region_merge.rs` (standalone, `rustc --edition 2021 -O`, runnable)
**Outcome:** WORKS

## Hypothesis

The canonical convergence-accumulator (spec `:1750-1766`, "each convergence thread
gets its own stack-local accumulator, merge after") maps onto the SHIPPED
`Accum` / `AccumColPtr` / append machinery with NO change to the append path. Each
core appends into a disjoint sub-region of the single reserved capacity buffer,
then a post-phase forward compaction concatenates the per-core live prefixes in
core order. The result is byte-identical to the single-core unit-outer `run()`
append.

## What the sketch proves

The sketch mirrors the real append pointer-math exactly and runs it under real OS
threads (std, sketch-only), 200 iterations per case, asserting byte-identical
output against a single-core reference appender.

- The append body is the verbatim shipped `resolve_append`
  (`dispatch/engine_ctx.rs:1015-1038`): `&self`, the saturating guard
  `if live >= cap { return }`, `ptr::write(base.add(live), v)`, `len.set(live+1)`.
- The per-core handle is the shipped `AccumColPtr { base, len: &Cell<USize>, cap }`
  (`engine_ctx.rs:133`) constructed per core as `base = orig_base + lo_c`,
  `len = &core_local_cell`, `cap = hi_c - lo_c`. The append code is UNTOUCHED; only
  the projection that builds the handle changes. This is the whole feasibility
  claim, and it holds.
- The record split is the shipped §8 slice (`scheduler/mod.rs:1263-1265`):
  `per = ceil(total/ncores)`, `lo = (c*per).min(total)`, `hi = (lo+per).min(total)`.

Cases (all OK, byte-identical x200): `total ∈ {0, 5, 37, 256, 1000}` against
`ncores ∈ {1, 2, 3, 4, 7, 8}`, including total-not-divisible-by-ncores (surplus
cores get `lo==hi`, a no-op region) and `ncores > total`.

## The working shape (what the impl follows)

1. **Region placement = offset `lo_c`, NOT `c * per_cap`.** The accumulator
   reserves `cap = build-time record count`, the global `<=1`-append-per-record
   upper bound. Region c owns records `[lo_c, hi_c)`, needs at most `hi_c - lo_c`
   slots, and placing it at element offset `lo_c` tiles the SAME reserved buffer
   exactly (sum of slice sizes = total). A separate `c*per_cap` partition would
   overflow the reserved buffer when `ncores * ceil(total/ncores) > total`; offset
   `lo_c` avoids that and reuses the §8 split verbatim.

2. **Each core has its OWN live-length cell.** Disjoint byte ranges of one buffer
   plus separate cells = no shared mutable state = structurally race-free. The 200x
   threaded run is corroboration, not the proof; the proof is the disjointness.

3. **Merge = forward compaction in core order.** Region c's live prefix sits at
   `[lo_c, lo_c + live_c)`. Concatenate ascending into `[0, sum live_c)`. The write
   cursor `write_pos = sum of prior live_c <= sum of prior slice sizes = lo_c`, so
   `dst <= src` always: the copy is forward-safe (`copy_within` / memmove). Order
   is preserved because each core walks an ascending contiguous record slice and
   appends in record order, and regions concatenate in ascending core order.

4. **Per-core live counts reach the merge.** Each core reports its final live
   count; the merge reads them to size each copy and advance the cursor. In the
   engine this is a per-core slot written before the waist barrier; the last
   arriver (or the main thread post-join) runs the compaction.

## Ripples to carry into the doc CL (not walls)

- **Capacity policy for multi-append-per-record.** Region cap = slice size assumes
  `<=1` append per record (the convergence-accumulator pattern the spec scopes,
  and the head+tail-convergence eligibility at `:770-771`). A WU that appends
  `k > 1` per record needs region cap = per-region worst case, not slice size; with
  slice-size cap the saturating guard would silently drop overflow. The doc CL must
  state the policy: either (a) scope the threaded accumulator path to the `<=1`/rec
  convergence pattern and keep multi-append single-core, or (b) reserve
  `cap = k * record_count` and size regions by `k * slice`. Option (a) matches the
  spec's convergence-accumulator scope and is the smaller first slice.

- **Non-commutative fallback (spec `:770-771`, `:1750-1766`).** A non-commutative
  resource accumulation skips convergence and stays single-core. Decision needs a
  WU hint (domain 08 `COMMUTATIVE` / order-dependence). Confirm the hint is
  reachable at dispatch; if absent, the conservative default is single-core for
  any accumulator until the hint lands (correct, just not yet parallel). This is a
  dispatch-routing detail, not a mechanism risk.

- **Cell atomicity.** The shipped `AccumBinding.len` cell doc already says
  "non-atomic: correct single-core, swapped for an atomic when multi-core lands"
  (`resource/bindings.rs:117`). The per-core design means each core uses its OWN
  cell (worker-stack-local or a per-core PoolFrame slot), so the BINDING cell is
  not shared and need not become atomic for this path; the per-core cells are
  single-writer. Only the live-count handoff to the merge crosses threads, and that
  is a write-before-barrier / read-after-barrier, already the §4 barrier shape.

## Second sketch: the dispatch seam (`dispatch_seam_rebase.rs`, WORKS)

The first sketch proved the algorithm in isolation. The wiring into the real
projection machinery is a distinct risk: the append resolves an `AccumColPtr`
from the projected `AccPtrCons` bundle (`AccumProject::acc_project`,
`engine_ctx.rs:656-664`), so the per-core variant must hand each node a different
base offset, a worker-supplied cell, and a new cap, and the rebased bundle carries
the cells' (shorter) lifetime. The second sketch models the real `AccPtrCons` /
`AccumColPtr` shape (manual `Copy` without a `T: Copy` bound, mirroring
`engine_ctx.rs:139`) and proves a `RebaseAccums` walk:

- Each node takes `cells[0]` as its live cell, offsets its base by `lo` elements
  (the same `.add`/`wrapping_add` arithmetic the append uses), sets cap to the
  region size, and recurses on `&cells[1..]`. Cells are assigned positionally with
  NO type-level counter and `k` (the accumulator count) is never named.
- The output bundle type is `AccPtrCons<'a, H, ...>` where `'a` is the worker
  cells' lifetime, rethreaded from the source `'frame`. This compiles.
- A rebased head drives the verbatim shipped append unchanged; the binding cells
  stay zero (the rebase swapped the cell), and the per-core live count matches the
  expected kept-record count.

The one fix the sketch surfaced: `AccumColPtr` must keep its manual unconditional
`Copy`/`Clone` (no `T: Copy` bound), which the real type already has; a derived
`Copy` would add a `T: Copy` bound and break the by-value `get`.

## Remaining wiring (engineering, not a type-level wall)

Proven: the algorithm and the projection-rebase seam. Still to wire in the src CL,
all within existing barrier infrastructure (no new unknown):

- A new `RunFiber` method (`run_accum_percore` or similar) that projects the accum
  bundle from bindings, rebases it with per-core `(lo, region_cap, &worker_cells)`,
  builds the `EngineCtx` with the rebased accum bundle plus the normal
  column/resource bundles, and walks unit-outer over `[lo, hi)`.
- Worker holds a fixed `[Cell<USize>; GATE2_MAX_ACCUMS]` on its stack, lends a
  prefix slice; the rebase consumes exactly `k` via slice-split.
- Per-core live counts publish to a shared scheduler array (or PoolFrame slot)
  before the phase's closing barrier (the §4 sync point), so the merge reads them
  after the worker stack cells are gone.
- Merge (forward compaction) runs after the accumulator phase's barrier. First
  slice: scope to a single accumulator phase so the main thread compacts
  post-`frame_await_done`; multi-phase accumulator merge at interior waists is a
  follow-up.

## Conclusion

The per-core-region + forward-compaction-merge mechanism is feasible on the real
`Accum`/`AccumColPtr`/append surface with no append-path change. The projection
that builds the per-core handle and the post-phase compaction are the new code.
No Step-11 op-decision is triggered: the mechanism does not wall. Proceed to the
doc CL, scoping the first slice to the `<=1`-append-per-record convergence pattern
with non-commutative accumulators kept single-core.
