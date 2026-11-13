# Single-Trunk Convergence

The engine these numbers time has none of its headline features yet: no fusing, and no adaptive trunk, fiber and morsel work on the dataflow schedule. The engine arms (`sequential`, `nway`) therefore measure a bare engine, and every number here is a baseline to compare against once those features exist. No design call rests on them, and the choice between head-and-tail and N-way stays open until those features exist (op, 13 November 2026, the kaski channel, `briefs/fixup/op-answers-21.md`: "So no calls should be made based on those yet, they are a baseline to compare against then").

Op names the region the engine is meant for as "the cases that have millions of simulation and crunching work data going constantly" (`briefs/fixup/op-answers-20.md`). This file reads that, as its own reading and not op's, as fibers of 1048576 records and up, on the folds and the append; the bench measured it at n in {1048576, 16777216, 134217728} for fold_light, at n in {1048576, 16777216} for the append and at n = 1048576 only for fold_heavy, W in {2, 4, 8}, warm, one fiber per frame with the harness's light workload between frames. In that region two head-and-tail walkers measure 1.58x to 1.84x over one walker of the same crew on the fold rows whose per-pass spread stays at or under 1.5x, and the fold rows above that spread measure 1.43x to 4.72x, the high end from a `kernel_seq` baseline whose passes differ threefold. At W = 2 the crew's N-way (`kernel_nway`) and head-and-tail land within 5% of each other on the folds, except fold_light n = 1048576 where head-and-tail measures 1.21x faster. At W = 4 and W = 8, where N-way runs more walkers, the ratio of head-and-tail's time to N-way's measures 1.04x to 1.88x against the crew's N-way and 0.71x to 2.06x against the engine's `nway` on the folds; on the append, where head-and-tail takes its sequential fallback, 1.50x to 3.21x and 0.71x to 1.88x. All of it holds on the 4-vCPU KVM guest of the setup section, warm, release profile.

Outside that region, on fold_light fibers of 16 to 1024 records (16 and 64 below one morsel, 256 and 1024 one and four morsels) and unpinned, no parallel arm measured below the engine's sequential frame of 7 to 11 us; pinned to one CPU the crew arms measure below it. That band is a boundary note, not a case the engine is meant for (op, `briefs/fixup/op-answers-20.md`: "1-4 morsels is almost never a realistic use case for hilavitkutin in the first place").

This bench times the two ways the engine's plan types allow one fiber to be split across cores: head-and-tail, where two walkers start at the two ends of the fiber and meet, and N-way, where each of W walkers takes one ceil slice. It was asked for on 13 November 2026, when op was shown the two designs and asked which is the design for single-trunk phases, with four options: head-and-tail, finishing the deletion of the N-way path; N-way, retiring head-and-tail; both, chosen at plan time per fiber; and measure first, a bench under the harness with both arms on single-trunk workloads, the question returning with the numbers and the code staying as it is meanwhile. He answered "Measure first" (the kaski channel, `briefs/job-hv-bench-convergence.md` and `briefs/fixup/r6-two-designs.md`). Under his later answer 21 these numbers do not return that question; what follows is measurements and the region each holds in.

The engine cannot run head-and-tail today: its dispatch never emits the `Head` and `Tail` record ranges. The head-and-tail arms therefore run on a crew the bench builds from the engine's own `PoolFrame` and frame protocol (`frame_publish`, `frame_await`, `frame_done_arrive`, `frame_await_done`), and two control arms run the same crew with the N-way slice, so a difference between the splits can be told apart from a difference between the crew and the engine's `run_parallel`.

Every statement below carries the region it holds in. Unless it names another, that region is the machine and build of the setup section, warm mode, and the median of the per-call times of every batch of three passes. "Holds for" lists the dimensions that move it; a dimension a statement does not name is one it was not measured across.

## The three measurements

How close head-and-tail comes to its designed 2x, and where. Against the same crew with one walker (`kernel_seq`), two head-and-tail walkers (`head_tail`) measure as below. A row marked `spread` has a per-pass median of `kernel_seq` or `head_tail` more than 1.5x its lowest (`pass_hi_ns` over `pass_lo_ns` in `results/summary.csv`); with three passes, one per process, its median is a vote between modes, so it is listed and not used for the ranges above.

| workload | n | W | head_tail | kernel_seq | speedup | spread |
| --- | --: | --: | --: | --: | --: | --- |
| fold_light | 134217728 | 2 | 45.20 ms | 83.20 ms | 1.84x | |
| fold_light | 134217728 | 4 | 45.62 ms | 83.41 ms | 1.83x | |
| fold_light | 134217728 | 8 | 47.03 ms | 83.72 ms | 1.78x | |
| fold_light | 16777216 | 2 | 2.32 ms | 10.97 ms | 4.72x | kernel_seq 3.22x, head_tail 1.83x |
| fold_light | 16777216 | 4 | 2.21 ms | 5.36 ms | 2.43x | kernel_seq 2.74x |
| fold_light | 16777216 | 8 | 2.67 ms | 5.21 ms | 1.95x | kernel_seq 2.36x |
| fold_light | 1048576 | 2 | 133.1 us | 291.9 us | 2.19x | head_tail 1.61x |
| fold_light | 1048576 | 4 | 135.3 us | 213.8 us | 1.58x | |
| fold_light | 1048576 | 8 | 186.3 us | 330.8 us | 1.78x | kernel_seq 1.58x |
| fold_heavy | 1048576 | 2 | 2.31 ms | 4.18 ms | 1.81x | |
| fold_heavy | 1048576 | 4 | 2.51 ms | 4.08 ms | 1.63x | |
| fold_heavy | 1048576 | 8 | 2.90 ms | 4.15 ms | 1.43x | head_tail 1.58x |

Holds for: the fold workloads, W in {2, 4, 8}, n in {1048576, 16777216, 134217728} for fold_light and n = 1048576 for fold_heavy, warm, unpinned. Two walkers cannot give 4.72x: against the lowest per-pass median of `kernel_seq` (3.52 ms, 3.12 ms and 3.14 ms at W = 2, 4 and 8) the n = 16777216 rows give 1.52x, 1.41x and 1.18x.

Below the region, at n in {4096, 65536}, W in {2, 4, 8}, the same ratio measures 0.76x to 1.94x: at n = 4096, 0.76x to 0.95x on both folds at every W except fold_heavy W = 4 at 1.40x; at n = 65536, 1.94x for fold_heavy W = 2, 1.32x to 1.49x for fold_heavy W = 4 and 8, and 0.95x to 1.23x for fold_light (`results/summary.csv`, rows `warm`, arms `head_tail` and `kernel_seq`). The short band (16 to 1024 records) is further below again and runs even cheaper for one walker: there the ratio falls to 0.64x (fold_light W = 4 n = 16).

At every W from 2 up, head-and-tail runs the same crew of two walkers, so its numbers at W = 2, 4 and 8 are three runs of one configuration; the drop from 1.81x to 1.43x on fold_heavy n = 1048576 is spread between runs, and its per-pass medians at W = 8 range from 2.23 ms to 3.52 ms. Against the engine's own sequential path (`sequential`) the same two walkers give 1.81x at fold_heavy n = 65536 W = 2 (274.1 us to 151.6 us), 1.75x at fold_heavy n = 1048576 W = 2 (4.04 ms to 2.31 ms) and 1.94x at fold_light n = 134217728 W = 4 (88.58 ms to 45.62 ms). The floating meeting point (`head_tail_dyn`, one compare and swap per 256-record block) measures within 2% of the fixed one on fold_heavy at n = 1048576 W in {2, 4} (2.31 ms and 2.52 ms against 2.31 ms and 2.51 ms), and loses to it on the light fold at n of 1048576 and up, by 2.4x at n = 1048576 W = 2 (325.5 us to 133.1 us) and by 1.18x at n = 134217728 W = 2 (53.18 ms to 45.20 ms). Elsewhere the two swap places by row (at fold_heavy n = 65536 W = 8 the floating one measures 192.6 us against 266.1 us, at W = 2 and 4 there it is 10% and 12% slower), so no ordering holds across the whole sweep. On the append, head-and-tail takes the design's sequential fallback, so it costs what one walker costs: `head_tail` and `kernel_seq` are within 1% of each other at n = 16777216 W = 2 (13.84 ms and 13.85 ms).

Where N-way passes it, and by how much. At W = 2 the two splits run the same number of walkers. On the crew they land within 5% of each other on fold_heavy at n = 65536 and 1048576 and on fold_light at n = 16777216 and 134217728 (fold_light n = 134217728: `kernel_nway` 43.11 ms, `head_tail` 45.20 ms; fold_heavy n = 1048576: 2.37 ms and 2.31 ms), though at fold_heavy n = 1048576 `kernel_nway` and at fold_light n = 16777216 both arms are spread rows (above 1.5x), so their tie is as uncertain as the modes. On fold_light at n = 65536 and 1048576 head-and-tail measures ahead, 34.4 us against 38.3 us (1.11x) and 133.1 us against 160.5 us (1.21x), and the engine's `nway` measures 52.2 us and 166.0 us there; the n = 1048576 row is a spread row on both arms (`head_tail` 1.61x, `kernel_nway` 1.64x), so its 1.21x is a vote between modes. At W = 4 and 8, where N-way runs more walkers:

| workload | n | W | head_tail | kernel_nway | nway (engine) | head_tail time over N-way time (crew, engine) | spread |
| --- | --: | --: | --: | --: | --: | --: | --- |
| fold_light | 134217728 | 4 | 45.62 ms | 25.17 ms | 29.06 ms | 1.81x, 1.57x | |
| fold_light | 16777216 | 4 | 2.21 ms | 1.79 ms | 1.65 ms | 1.23x, 1.34x | nway 1.68x |
| fold_light | 1048576 | 4 | 135.3 us | 130.0 us | 191.5 us | 1.04x, 0.71x | |
| fold_heavy | 1048576 | 4 | 2.51 ms | 2.19 ms | 2.45 ms | 1.15x, 1.03x | |
| fold_heavy | 65536 | 4 | 195.6 us | 266.0 us | 181.3 us | 0.74x, 1.08x | kernel_nway 1.65x |
| append | 16777216 | 4 | 15.50 ms | 7.13 ms | 9.33 ms | 2.17x, 1.66x | |
| append | 1048576 | 4 | 596.1 us | 389.7 us | 824.7 us | 1.53x, 0.72x | |
| fold_light | 134217728 | 8 | 47.03 ms | 25.27 ms | 25.30 ms | 1.86x, 1.86x | |
| fold_light | 1048576 | 8 | 186.3 us | 156.5 us | 184.1 us | 1.19x, 1.01x | |
| append | 1048576 | 8 | 610.7 us | 408.4 us | 858.8 us | 1.50x, 0.71x | |
| fold_light | 16777216 | 8 | 2.67 ms | 1.42 ms | 1.30 ms | 1.88x, 2.06x | nway 1.51x |
| fold_heavy | 1048576 | 8 | 2.90 ms | 1.88 ms | 1.97 ms | 1.54x, 1.47x | head_tail 1.58x |
| append | 16777216 | 8 | 15.92 ms | 4.95 ms | 8.46 ms | 3.21x, 1.88x | |

Holds for: W in {4, 8}, the n listed; W = 8 runs eight walkers on four vCPUs. A `spread` cell marks the arm whose per-pass spread is above 1.5x, so that ratio is a vote between modes. The append lines compare N-way with the sequential fallback head-and-tail takes there. The fold ratios sit at about what N-way's extra walkers would give where the data does not fit cache (the largest fold_light), and below it on the one compute-bound size measured (fold_heavy n = 1048576); which of the two causes it was not separated (`## What was not measured`), and the compute-bound size sits on the region's lower edge at 6 batches from 3 processes.

A boundary note: what each costs on short fibers. Fibers of four morsels and fewer lie outside the region the engine is meant for, so this band says where the parallel arms stop paying and nothing about the engine's intended use. On fold_light fibers of 16 to 1024 records, unpinned, every parallel arm measures more per frame than the engine's sequential path, at every W measured, and the cost does not move with n:

| arm | W = 1 | W = 2 | W = 4 | W = 8 |
| --- | --: | --: | --: | --: |
| sequential (engine) | 7.6 to 11.1 us | 7.5 to 10.4 us | 7.2 to 9.6 us | 7.1 to 9.6 us |
| kernel_seq (crew, one walker) | 21.2 to 23.7 us | 21.0 to 30.3 us | 19.9 to 24.7 us | 19.8 to 22.0 us |
| head_tail | 20.6 to 24.6 us | 27.1 to 34.2 us | 27.3 to 33.8 us | 28.6 to 31.4 us |
| kernel_nway | 22.6 to 27.0 us | 29.8 to 31.8 us | 40.1 to 45.4 us | 66.6 to 76.8 us |
| nway (engine) | 34.0 to 35.8 us | 40.3 to 49.2 us | 49.4 to 62.2 us | 74.2 to 97.2 us |
| std_parallel | 38.8 to 47.4 us | 51.6 to 63.9 us | 84.1 to 100.2 us | 163.9 to 188.5 us |

Holds for: short, n in {16, 64, 256, 1024}, warm, unpinned. Each range is the lowest and the highest median over the four n. Head-and-tail's cost stops growing past W = 2 because it never wakes more than two walkers; the N-way arms wake W.

Most of a crew frame's cost here is the wake of a parked worker on another vCPU, and that cost is bimodal per process. Pinned to one CPU (`results-pinned-one-cpu/`, W = 1, n in {16, 64, 256, 1024}), the crew arms' medians measure 1.7 to 3.1 us per frame, with per-pass medians up to 3.26 us, against 7.6 to 8.1 us for `sequential`, and the engine's `nway` 11.3 to 17.0 us. Unpinned, a process keeps one of two modes for a whole pass: at fold_light n = 4096 W = 1 (`results/fold_light_w1/fold_light_w1_n4096.csv`, column `algo_ns`, the one the summary reads), `kernel_nway` passes 2 and 3 run at 3.2 to 5.5 us per batch and pass 1 at 26.5 to 33.1 us, and `head_tail` pass 3 at 2.2 to 3.8 us and passes 1 and 2 at 21.0 us and up. In the short W = 1 section every pass of every crew arm sat in the slow mode: 279 of its 288 crew batches lie between 18.7 and 36.5 us, 2 below 3 us and 7 above 37 us, the highest 132.7 us. The idle driver of this guest is `none`, so an idle vCPU halts and its wake goes through the hypervisor; that is the likely mechanism and it was not measured directly.

## Setup

The machine is a KVM guest with 4 vCPUs of an Intel Xeon at 2.10 GHz (family 6 model 207), one thread per core as the guest sees it, 2 MiB of L2 per core, 260 MiB of L3, 15 GB of memory, Linux 6.18.44, cpuidle driver `none`, transparent huge pages on `madvise` (the bench's columns do not ask for them), clocksource `tsc`. How the vCPUs sit on host cores is not visible from the guest. The machine maximum worker count is therefore 4, and W = 8 oversubscribes it two to one. Of these, the committed `*.meta.json` files carry only the cpu brand, the OS, the rustc build and the counter frequency; the cache sizes, the model, the memory, the cpuidle driver and the huge-page mode are read from this guest at review time and are not in any committed artifact (ticket 2894d40ab22f).

The build is rustc 1.98.0-nightly (57d06900f 2026-05-27), the release profile at opt-level 3, fat LTO, one codegen unit, `CARGO_INCREMENTAL=0`. Every arm is its own cdylib under `variants/`, loaded by the harness in a worker subprocess of its own for each pass and mode.

The harness calibrates the TSC at 2100.3 MHz, three one-second sleeps per worker subprocess (ticket c634312bdc34 is about the cost). The routine is `Conv<N>`, one call is one frame over N records, and every call is timed by `timed_calibrated!` repeating it to at least 2048 ticks. Each section runs 3 passes; a pass runs 60 timed calls in batches of 10 (short sections 150 in batches of 25, the large-n bands 8 in batches of 4), after a warm-up of a fifth of that. Warm mode keeps the arm's state across calls; cold mode builds it afresh when the seed changes, outside the timed region. The surrounding program is the harness's light workload: a scalar dependency stream and branchy context on the caller between calls.

## Arms

| arm | what runs | split | walkers |
| --- | --- | --- | --: |
| sequential | the engine, `Pin::get_unchecked_mut(..).run()` | none | 1 |
| nway | the engine, `run_parallel` over the bench's executor | ceil slices, the engine's own | W |
| head_tail | the crew | head forward from 0, tail backward from n, meeting at n / 2 | min(W, 2) |
| head_tail_dyn | the crew | as head_tail, the meeting point floating by 256-record blocks | min(W, 2) |
| kernel_seq | the crew | the whole fiber | 1 |
| kernel_nway | the crew | ceil slices, the engine's formula | W |
| std_parallel | `std::thread::scope`, threads spawned per frame | ceil slices | W |

The workloads are a wrapping-add fold over one `u32` column with an identity kernel (fold_light) and with a heavy arithmetic kernel (fold_heavy), and an order-sensitive append of one transformed value per record (append). The fold partials merge through the engine's `ConvergenceBuffer` inside the frame; the append's per-walker regions compact in core sequence after it. The short band is fold_light at 16 to 1024 records. Record counts run from below one morsel (256 records) to 134217728 records (512 MiB, far past the 260 MiB L3), fold_heavy up to 1048576.

Correctness is `support/tests/correctness.rs`: every arm against `sequential` and against a reference computed in the test, on ragged sizes from 1 record to 64 morsels plus 5, at W in {1, 2, 3, 4, 8}; the append's sequence; what a head-and-tail append without the fallback produces (the head half in sequence, the tail half reversed, differing from sequential exactly when the tail half is not a palindrome); that the floating meeting point covers every record once over repeated frames; and the case that must fail, a split that walks a range twice, caught by both the reference and the count. The harness also validates every arm against the others on three seeds, and checks each is deterministic, before it times anything.

## What was not measured

The intended region in full. fold_heavy was measured up to n = 1048576 only, so the compute-bound fold has one size inside the region; nothing lies between 1048576 and 16777216 records; and the large-n bands run 6 batches from 3 processes, so a bimodal arm there decides its own median. The sweeps the region still needs are fold_heavy at 16777216 and 134217728, the append at 134217728, a size between 1048576 and 16777216 on every workload, more processes on the large bands, a mode that runs frames back to back with no caller workload between them, and a plain single-thread loop arm as the baseline's reference. `std_parallel` was measured in the region and is left out of the statements above; at fold_light n = 134217728 W = 4 it measures 23.69 ms against 29.06 ms for `nway`, and at fold_heavy n = 1048576 W = 4 1.70 ms against 2.45 ms. Ticket 2894d40ab22f tracks these.

No engine with its headline features. The engine arms time an engine with no fusing and no adaptive trunk, fiber and morsel work; the numbers above are the baseline those are compared against.

Head-and-tail inside the engine. The engine emits no `Head` or `Tail` range, so these numbers are the split on the bench's crew, built from the engine's frame protocol. What the engine's dispatch, plan and merge would add or remove around it is not measured.

Fibers in a full pipeline. One fiber runs alone per frame here, with one unit on it. Several trunks, several units per fiber, and the phase and waist structure around them are not measured.

The mechanism of the bimodal wake. The pinned run shows the slow mode needs a second CPU; the hypervisor exit is inferred from the idle driver and not traced.

Why compute-bound scaling stops near 2x at W = 4. fold_heavy at n = 1048576 runs 1.86x faster on four `kernel_nway` walkers than on one (4.08 ms to 2.19 ms) and `std_parallel` 2.4x (1.70 ms). Host placement of the vCPUs, frequency and the kernel's own throughput were not separated.

The spread at large n. Some arms at n = 16777216 are bimodal per pass (fold_light W = 2 `kernel_seq`: passes at 3.52 ms and 11.35 ms). Huge pages are ruled out (`madvise`, not requested); the cause is not established.

A floating meeting point finer than a block. `head_tail_dyn` claims 256-record blocks; the per-record claim of the head-and-tail sketch is not measured.

Other machines. Every number holds on this guest only, and a bare-metal machine with a real idle driver would change the short-fiber costs most.

## Disturbances

One W = 1 run was voided by background builds (the SessionStart `agent-regen` over another repository, and a commit's pre-commit hook) and rerun in full on an idle machine; ticket 7ddf5ef52d08. During the W = 2 run the disk-trim hook freed 1.9 GB around fold_heavy n = 65536; that section's numbers are kept. The W = 4 run was lost to a container restart and rerun in full.

The W = 1, W = 8 and pinned section sets were recorded from a dirty working tree (`git_commit` in their meta files reads `ed2bf53-dirty`, `d11e290-dirty` and `e486b5f-dirty`); W = 2 and W = 4 are clean. `support/`, `variants/` and the engine source are unchanged between each base commit and the tip, so the arms are the same, but what else was dirty in those trees is not recorded. Ticket 2894d40ab22f carries pinning it down.

## Files

`results/<section>/` holds the harness's CSV, meta and findings per section and size. `results/summary.csv` and `results/summary.md` are `single-trunk-convergence --summary results`: per section, size, mode and arm, the median, quartiles, fastest batch, the lowest and highest per-pass median, and the ratio to `sequential` and to `kernel_seq`. `results-pinned-one-cpu/` is the short W = 1 section run under `taskset -c 1`.
