# engine_vs_std (#660 / #664): single-core engine vs optimal std, gap attributed

> STATUS: PREMATURE / SUPERSEDED. This bench measured a placeholder dispatch,
> not the engine. The single-core runtime is NOT complete (`run()` is an interim
> unit-outer stand-in; the fiber walk is resource-only; `codegen_fiber` /
> `codegen_core` / `run_fiber` are stubs). Benching an incomplete engine tells us
> nothing about the engine's real single-core performance, which was the entire
> point of the exercise, so as a perf verdict this effort means nothing and is
> retained only as a record. The one durable byproduct is a direction signal for
> the build, not a result: the dispatch machinery is free (dispatch(unit->engine)
> is about 1.0x), so the gap is purely the unit-outer placeholder layout and the
> designed morsel-outer dispatch is the right thing to build. The real comparison
> happens only once the single-core runtime is complete and benched in that
> state.

Op asked, before any multi-threaded work, whether the hilavitkutin engine
running single-core beats the same workload written on a std base as optimally
as possible, on startup (get ready) and runtime (process to finish), and if it
does not, where the inefficiency is. The first cut measured a 2x to 4x runtime
loss and attributed it to engine machinery. This revision attributes the gap
properly by adding two intermediate std arms that reproduce the engine's data
layout without its dispatch machinery, and the attribution changes the
conclusion: the machinery is nearly free, and most of the gap is the layout of
a dispatch path that is an explicit placeholder, not the engine as designed.

## What `run()` actually does today

The dispatch `scheduler.run()` walks is an interim stand-in, not the designed
runtime. Its own docstring says the real morsel loop "waits on the
`codegen_fiber` / `codegen_core` LLVM tier" (#340, unbuilt). The placeholder is
unit-outer: each WorkUnit completes its entire record range before the next
runs, so every intermediate column materialises at full N. The per-fiber
morsel-outer dispatch that would keep a morsel's columns cache-resident across
stages does not exist yet. This bench therefore measures the placeholder, and
the headline number from the first cut described the placeholder's layout, not
the engine's architecture.

## Setup

Workload: four RAW-chained element-wise stages over N records, `u32` in every
arm. An input column `In[i] = i` is host-populated before the frame; `S1` reads
In and writes A, `S2` reads A writes B, `S3` writes C, `S4` writes D, each a
wrapping multiply / shift / xor. FNV-1a over D validates that all arms compute
the identical result (`checksum_ok = true` at every N).

Four runtime arms walk the same stages over the same input and differ only in
scheduling:

1. fused std: A/B/C in registers, one body autovectorised across all four
   stages, only D materialised. The optimal shape op asked for.
2. morsel-outer std: one 256-record morsel walks all four stages through
   L1-resident scratch before the next morsel. The designed #340 layout, minus
   fusion (intermediates are still written, but to L1).
3. unit-outer std: four full-N passes, each writing a whole intermediate before
   the next reads it. The placeholder's layout, expressed in std.
4. engine: unit-outer, scalar `each` bodies, the real dispatch machinery.

The three steps between them attribute the gap: materialise (fused to morsel) is
std's cross-stage fusion advantage; evict (morsel to unit) is the full-N cache
cost; dispatch (unit to engine) isolates engine machinery against an
identical-layout std baseline.

Numbers are medians on aarch64 (apple), release (opt-level 3, fat LTO, one
codegen unit), `caffeinate` pinned. The 4096 and 65536 sizes are stable across
runs. The 1048576 size is memory-bound and its absolute numbers swing with
background load on this machine (two runs disagreed by 5x to 7x on the unit-outer
and engine arms); the direction is stable, the magnitude is not. Representative
medians in nanoseconds (two runs shown for the noisy size):

| N | fused | morsel-outer | unit-outer | engine | materialise | evict | dispatch |
|---|---|---|---|---|---|---|---|
| 4096 | 417 | 1084 | 833 | 875 | 2.60x | 0.77x | 1.05x |
| 65536 | 6584 | 20250 | 22333 | 22833 | 3.08x | 1.10x | 1.02x |
| 1048576 (run A) | 130083 | 582417 | 3153833 | 2586584 | 4.48x | 5.42x | 0.82x |
| 1048576 (run B) | 106167 | 325542 | 452125 | 550250 | 3.07x | 1.39x | 1.22x |

Startup (both runs agree): engine is a near-constant 64000 to 67000 ns (plan
computation, N-independent); std allocation is 500 ns at 4096, growing to 150000
to 298000 ns at 1048576 (zeroing two N-sized buffers). The engine startup loses
badly at small N and wins at large N (crossover near 1M).

## Result

The decisive ratio is `dispatch (unit to engine)`, and it is approximately 1.0x
at every size in both runs (0.82x to 1.22x). The engine performs like
identical-layout std. Its dispatch machinery (scalar `each` closures, the slot
shims, the morsel windowing) is essentially free. The engine is not slow because
of its machinery.

The gap is the layout. `materialise (fused to morsel)` is a stable 2.6x to 3.1x
at the sizes that hold still: this is std's register-fusion plus cross-stage
vectorisation, an advantage available only because the workload is trivially
fusible. `evict (morsel to unit)` is near 1.0x at sizes that fit cache and grows
with N as full-N intermediates spill (the unit-outer arm streams five N-sized
buffers; the morsel-outer arm keeps three of them in L1). The growth of the
total gap with N, the 2x to 4x reported in the first cut, is this eviction term.

## Why this matters for the engine

The placeholder's unit-outer layout is the entire reason the first-cut number
looked bad, and it is the layout the designed dispatch replaces. The morsel-outer
std arm models that designed layout: at the sizes that hold still it already
matches or beats unit-outer, and at large N it is the cheapest non-fused arm by a
wide margin. Building the per-fiber morsel-outer dispatch (#340) is expected to
recover the eviction term, which is the part that grows with N and dominates the
large workloads the engine targets.

What #340 does not recover is the materialise term: std's fusion keeps A/B/C in
registers and vectorises across stages, and the engine matches that only with its
own stage fusion (emitting a fiber of element-wise WorkUnits as one fused record
loop). The fiber already names the unit that fusion would operate on, so the
engine is well-positioned for it, but it is a codegen feature beyond #340.

## Implication for the parallel decision and single-core parity

Single-core parity on this workload needs both pieces: the morsel-outer dispatch
(#340) for the eviction term, and stage fusion for the materialise term. With
#340 alone the engine lands near the morsel-outer std arm, roughly 3x off fused
on this chain, because std fuses and the engine does not yet.

The workload matters. This is a trivially-fusible element-wise chain, which is
std's best case and the engine's worst case: std collapses four stages into one
register-resident vectorised loop. On a non-fusible workload (a stage needing a
whole upstream column, a gather, a reduction feeding a broadcast) std cannot fuse
either and must materialise its intermediates, so the materialise term largely
disappears and the engine's morsel-outer dispatch matches std single-core. That
is the regime where the original prediction (single-core comparable, parallel
ahead) holds. A non-fusible counter-bench would measure it directly.

For the parallel question: parallel scaling multiplies whatever the per-core
single-core number is. Benching parallel on top of the placeholder would measure
scaling against the wrong (unit-outer, eviction-heavy) per-core baseline.
Building the designed morsel-outer dispatch first gives an honest single-core
baseline for the parallel comparison to multiply.

## Caveats

- The morsel-outer std arm uses per-stage SIMD; the real #340 engine path would
  use scalar `each` bodies. The dispatch step being near 1.0x at the unit-outer
  layout shows scalar-versus-SIMD barely matters here (the passes are
  memory-bound), so the morsel-outer std arm is a fair proxy for #340's
  achievable single-core, but it is a proxy, not the built path.
- The 1048576 absolute numbers are unstable on this machine under unknown
  background load. The conclusion rests on the stable sizes (dispatch near 1.0x,
  materialise 2.6x to 3.1x) and on the direction at large N (morsel-outer well
  below unit-outer), not on the precise large-N magnitude.
- `u32` in every arm isolates scheduling from the arvo-vs-bare-numeric question.
  A follow-up could re-run the engine arm with arvo `Uint<32>` columns to confirm
  the `repr(transparent)` lowering is zero-cost.
- Startup is build versus allocate. The engine's build does plan computation once
  and amortises across frames; a single-frame workload pays it in full.

## Perf gate (#664): the standing red oracle

The #660 finding above is prose; the gate turns it into an executable
definition of "the single-core engine is complete". It was `#[ignore]` tests in
`tests/perf_gate.rs` with a timing loop of its own, and since 13 November 2026
it is the bench harness: every workload is a section of `bench.toml`, every arm
is a variant cdylib validated against the reference digest before it is timed,
and `engine_vs_std --verdict` holds the medians a run promoted into `results/`
to the ceilings in `support/src/expected.rs`. It is RED until Phase D (#340)
lands the two load-bearing mechanisms (dispatch devirtualisation and
within-fiber stage fusion) and the engine reaches the designed 0.95x to 1.02x
parity. Run it deliberately, from a stage:

```text
../stage.sh .
<stage>/release/engine_vs_std
<stage>/release/engine_vs_std --verdict
```

A cell's arms are held to the reference digest by the harness before any
timing counts, so a red verdict is unambiguously "engine slower" (the gate
working) and never "the arms diverged" (a broken bench). The comparison also
runs in `./run-checks.sh`, at a small size, and `../stage.sh` runs those
checks before it builds anything for timing.

The sections that follow predate the harness. Their numbers
were taken with the old loop over an input of `In[i] = i`, and the harness's
seeded input and per-arm worker processes are not the same measurement.

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
