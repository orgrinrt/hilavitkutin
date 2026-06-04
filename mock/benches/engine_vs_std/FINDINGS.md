# engine_vs_std (#660): single-core engine vs optimal fused std

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
