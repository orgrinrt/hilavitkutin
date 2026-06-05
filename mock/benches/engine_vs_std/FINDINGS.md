# engine_vs_std (#660 / #664): single-core engine vs optimal fused std

Op asked, before any multi-threaded work, whether the hilavitkutin engine
running single-core beats the same workload written on a std base as optimally
as possible, on startup (get ready) and runtime (process to finish). If it does
not, find the inefficiency. This measures that.

## Setup

Workload: four RAW-chained element-wise stages over N records, `u32` in both
arms. An input column `In[i] = i` is host-populated before the frame (the
engine's input model); `S1: A = stage1(In)`, `S2: B = stage2(A)`,
`S3: C = stage3(B)`, `S4: D = stage4(C)`, each stage a wrapping multiply / shift
/ xor. FNV-1a over D validates the two arms compute the identical result
(`checksum_ok = true` at every N).

Engine arm: four WorkUnits over `Column<Av..Dv>`, dispatched single-core through
`scheduler.run()`; the engine materialises all four intermediate columns,
windowed per 256-record morsel. Std arm: one fused pass that zips the input and
output slices (bounds-check-free, autovectorisable), keeping A/B/C in registers
and materialising only D.

Numbers are medians on aarch64 (apple), release (opt-level 3, fat LTO, one
codegen unit), `caffeinate` pinned, 20 to 4000 iterations per size (more at
small N), warmup discarded. Two runs agreed within noise; representative
medians in nanoseconds:

| N | engine startup | std startup | engine runtime | std runtime | startup engine/std | runtime engine/std |
|---|---|---|---|---|---|---|
| 4096 | 62400 | 440 | 875 | 417 | ~140x | 2.1x |
| 65536 | 62300 | 3200 | 22000 | 6459 | ~19x | 3.4x |
| 1048576 | 62000 | 95000 | 445000 | 105000 | ~0.7x | 4.2x |

## Result

Single-core, the engine does NOT win. On runtime it is 2.1x slower at N=4096,
growing to ~4.2x at N=1048576, and the gap widens monotonically with N. On
startup the engine pays a near-constant ~62us (plan computation, N-independent)
that dwarfs the std arm's allocation at small N (~140x) and only draws level
once the std arm's own allocation and zeroing of two N-sized buffers exceeds
~62us (around N=1M).

## Why (the inefficiency op asked to surface)

The workload is trivially fusible, which is the optimal-std arm's best case and
the engine's worst case:

1. Intermediate materialisation. The engine writes four full columns (Av, Bv,
   Cv, Dv) to the arena; the fused std loop keeps A/B/C in registers and writes
   only D. The engine streams five columns (In read, then Av/Bv/Cv/Dv each
   written then read) where the fused loop streams two (In read, D written), so
   the engine moves on the order of 2.5x the bytes. That figure is a
   stream-count estimate, not a measured byte count.
2. Lost cross-stage vectorisation. The std loop is one body the compiler
   autovectorises across all four transforms (SIMD lanes over `u32`). The engine
   dispatches a separate scalar `each` closure per stage through the column
   reader/writer; the optimiser does not vectorise across the per-stage dispatch
   boundary, so the engine runs scalar arithmetic where std runs SIMD.
3. Per-morsel dispatch machinery. Each 256-record morsel re-enters the
   unit-outer dispatch loop; the fused loop has no such boundary.

The morsel windowing keeps each morsel's columns L1-resident across the four
stages, which is why the gap is a low constant factor rather than a
memory-bandwidth blowup, but it does not recover the materialisation traffic or
the lost vectorisation.

## Implication for the parallel decision

On a single-core, trivially-fusible, element-wise workload, a hand-written fused
SIMD loop is the thing to beat, and it wins by 2 to 4x. The engine's value is
therefore not single-core throughput on fusible chains; it is (a) parallelism,
where the staged columnar model spreads trunks across cores, and (b) complex,
non-fusible dependency graphs, where hand-fusion is not available and the
scheduler's analysis earns its cost. To break even with the fused loop on a
per-core basis the parallel runtime must recover the 2 to 4x single-core
handicap; with enough cores it does, but the handicap is real and sets the bar.

Two engine-side costs are candidate follow-ups if single-core throughput on
fusible chains is ever a goal: fusing adjacent element-wise stages so the
intermediate columns never materialise, and vectorising the per-morsel `each`
body. Both are codegen work well beyond this bench.

## Caveats

- `u32` in both arms isolates the engine's dispatch and materialisation cost
  from the arvo-vs-bare-numeric question. A follow-up could re-run the engine
  arm with arvo `Uint<32>` columns to confirm the `repr(transparent)` lowering
  is zero-cost.
- One workload (fusible, element-wise). A non-fusible workload (a stage that
  needs a whole upstream column, a gather, a reduction feeding a broadcast)
  would force the std arm to materialise intermediates too, narrowing or
  reversing the gap; that is a separate bench.
- Startup is reported as build vs allocate. The engine's build does plan
  computation once and is amortised across frames; a single-frame workload pays
  it in full. The std startup measures allocation of its two buffers only, not
  seeding them; the input fill is untimed, the analogue of the engine's untimed
  In-column population (both arms time pure compute over already-seeded input).

## Perf gate (#664): the standing red oracle

The #660 finding above is prose; the gate turns it into an executable
definition of "the single-core engine is complete". It lives as `#[ignore]`
tests in `tests/perf_gate.rs` over the same harness (`src/lib.rs`), asserting
the engine is no worse than the optimal fused std arm. It is RED until Phase D
(#340) lands the two load-bearing mechanisms (dispatch devirtualisation and
within-fiber stage fusion) and the engine reaches the designed 0.95x to 1.02x
parity, at which point it goes GREEN and signals Gate-1 (#661) perf-done. Run
it deliberately:

```text
caffeinate -dimsu cargo test --release -- --ignored --test-threads=1
```

The tests are ignored by default because they are timing assertions that are
expected red and need the release profile to be meaningful; auto-running them
would fail every unrelated `cargo test` and report noise in debug. Every test
asserts checksum equality first, so a failure is unambiguously "engine slower"
(the gate working) and never "the two arms diverged" (a broken bench).

### Workload matrix

Three shapes form a gradient rather than a single cliff, so the gates show
progress mechanism by mechanism as Phase D lands:

1. `element_wise`: the original #660 four-stage RAW chain. Pure fusion territory.
2. `branching`: two independent transforms over the same input joined by a
   third. A multi-fiber DAG exercising dispatch across fibers.
3. `accumulator`: one transform feeding the append surface, dispatched
   unit-outer, against an optimal std buffer fill.

Representative runtime ratios (engine / std, median, release, single-thread
pinned, 2026-06-05):

| workload | N=4096 | N=65536 | N=1048576 |
|---|---|---|---|
| element_wise | 2.2x | 3.4x | 5.0x to 5.7x |
| branching | 1.75x | 2.45x | 2.6x to 3.1x |
| accumulator | 6.3x | 3.9x to 6.4x | 6.5x |

The runtime axis is asserted at every size (the headline drive-toward-parity
gate). The startup axis is asserted only at the largest size, where the
schedule-once design makes startup parity reachable (the engine's fixed plan
build beats std re-allocating two N-sized buffers; at N=1M the engine startup
is 0.07x to 0.48x of std). At small N the engine's plan build cannot match two
`vec!` calls, and that gap amortises across reused frames by design, so raw
startup is reported by the bench at every size but not asserted as a forever-red
gate Phase D cannot close.

### Finding: the accumulator workload is the widest gap, and frames do not reset accumulators

The `accumulator` shape has the widest measured ratio at every size (roughly
6.3x to 6.5x, near-flat with N), while `element_wise` is the canonical fusion
case whose ratio grows monotonically with N (2.2x to 5.7x, the
memory-bandwidth-bound signature the audit memo names). These are not in
tension: the two reds come from different costs. The append surface advances a
live-length cell per record and dispatches unit-outer (no morsel-local fusion),
so the accumulator pays per-record append accounting on top of the
materialisation and dispatch costs the other workloads pay, which is why its
magnitude is largest and roughly N-independent. The element-wise chain pays the
pure intermediate-materialisation traffic that fusion removes, which is why its
gap is the cleanest demonstration of the missing fusion mechanism and why the
memo treats it as the headline fusion workload. Fusion plus the per-fiber
morsel-outer path close both, the accumulator gated additionally on the append
surface dispatching morsel-local where its records are not externally observed.

Building the accumulator workload surfaced a Gate-1 gap unrelated to throughput:
the accumulator's live-length is zeroed by the store drain at BUILD time
(`scheduler::build`), not at the start of each `run`. The schedule-once-reuse
model reuses one built scheduler across many frames, so without a per-frame
reset the second frame starts at the reserved capacity and every append
saturates (drops). A per-frame accumulator reset is not yet implemented in
`run`. The bench works around it by zeroing the live-length cell before each
timed `run` (an O(1) `Cell` write, negligible against N appends), which stands
in for the reset a completed frame lifecycle must perform. Tracked as a Gate-1
follow-up; it belongs with the frame lifecycle / resource resolution work
(#344), not with this gate.
