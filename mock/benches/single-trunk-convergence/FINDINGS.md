# Single-Trunk Convergence

Head-and-tail reaches 1.8x to 1.9x over one walker of the same crew where a frame holds about 150 us of work or more (fold_heavy at 65536 records and up, fold_light at 134217728, W = 2), and loses to it at 4096 records. N-way ties it at W = 2 and passes it at W = 4 by up to 1.81x and at W = 8 by up to 2.05x on the folds, and by up to 3.22x on the append where head-and-tail falls back to one walker; on compute-bound and mid-sized fibers its lead is 1.04x to 1.54x, and at fold_heavy n = 65536 W = 4 head-and-tail is ahead. On fibers of 16 to 1024 records no parallel arm beats the engine's sequential frame of 7 to 11 us: head-and-tail costs 20 to 34 us and the engine's N-way 34 to 97 us, rising with W, most of it the wake of a worker on another vCPU. All of it holds on the 4-vCPU KVM guest of the setup section, warm, release profile.

This bench times the two ways the engine's plan types allow one fiber to be split across cores: head-and-tail, where two walkers start at the two ends of the fiber and meet, and N-way, where each of W walkers takes one ceil slice. The question it answers was put to op as a choice between finishing either path, and he answered "measure first: a bench under the harness with both arms on single-trunk workloads, the question returns with the numbers, the code stays as it is meanwhile". So what follows is numbers and where they hold, with no recommendation.

The engine cannot run head-and-tail today: its dispatch never emits the `Head` and `Tail` record ranges. The head-and-tail arms therefore run on a crew the bench builds from the engine's own `PoolFrame` and frame protocol (`frame_publish`, `frame_await`, `frame_done_arrive`, `frame_await_done`), and two control arms run the same crew with the N-way slice, so a difference between the splits can be told apart from a difference between the crew and the engine's `run_parallel`.

Every statement below carries the region it holds in. Unless it names another, that region is the machine and build of the setup section, warm mode, and the median of the per-call times of every batch of three passes. "Holds for" lists the dimensions that move it; a dimension a statement does not name is one it was not measured across.

## The three answers

Does head-and-tail reach its designed 2x, and where. Against the same crew with one walker (`kernel_seq`), two head-and-tail walkers (`head_tail`) reach 1.8x to 1.9x where a frame holds hundreds of microseconds of work or more, and lose below that.

| workload | n | W | head_tail | kernel_seq | speedup |
| --- | --: | --: | --: | --: | --: |
| fold_heavy | 65536 | 2 | 151.6 us | 293.6 us | 1.94x |
| fold_heavy | 1048576 | 2 | 2.31 ms | 4.18 ms | 1.81x |
| fold_light | 134217728 | 2 | 45.20 ms | 83.20 ms | 1.84x |
| fold_light | 134217728 | 4 | 45.62 ms | 83.41 ms | 1.83x |
| fold_light | 134217728 | 8 | 47.03 ms | 83.72 ms | 1.78x |
| fold_heavy | 1048576 | 4 | 2.51 ms | 4.08 ms | 1.63x |
| fold_heavy | 1048576 | 8 | 2.90 ms | 4.15 ms | 1.43x |
| fold_heavy | 4096 | 2 | 40.4 us | 36.4 us | 0.90x |
| fold_light | 4096 | 2 | 31.3 us | 27.4 us | 0.88x |

Holds for: the fold workloads, W in {2, 4, 8}, the n listed. At every W from 2 up, head-and-tail runs the same crew of two walkers, so its numbers at W = 2, 4 and 8 are three runs of one configuration; the drop from 1.81x to 1.43x on fold_heavy n = 1048576 is spread between runs, and its per-pass medians at W = 8 range from 2.23 ms to 3.52 ms. Against the engine's own sequential path (`sequential`) the same two walkers give 1.81x at fold_heavy n = 65536 W = 2 (274.1 us to 151.6 us), 1.75x at fold_heavy n = 1048576 W = 2 (4.04 ms to 2.31 ms) and 1.94x at fold_light n = 134217728 W = 4 (88.58 ms to 45.62 ms). The floating meeting point (`head_tail_dyn`, one compare and swap per 256-record block) matches the fixed one on the heavy fold and loses to it on the light fold, by 2.4x at n = 1048576 W = 2 (325.5 us to 133.1 us) and by 1.18x at n = 134217728 W = 2 (53.18 ms to 45.20 ms). On the append, head-and-tail takes the design's sequential fallback, so it costs what one walker costs: `head_tail` and `kernel_seq` are within 1% of each other at n = 16777216 W = 2 (13.84 ms and 13.85 ms).

Where N-way passes it, and by how much. At W = 2 the two splits run the same number of walkers and land within 5% of each other on every fold at n of 65536 and up (fold_light n = 134217728: `kernel_nway` 43.11 ms, `head_tail` 45.20 ms; fold_heavy n = 1048576: 2.37 ms and 2.31 ms). At W = 4, N-way's two further walkers are the whole of its lead:

| workload | n | W | head_tail | kernel_nway | nway (engine) | N-way lead (crew, engine) |
| --- | --: | --: | --: | --: | --: | --: |
| fold_light | 134217728 | 4 | 45.62 ms | 25.17 ms | 29.06 ms | 1.81x, 1.57x |
| fold_light | 16777216 | 4 | 2.21 ms | 1.79 ms | 1.65 ms | 1.23x, 1.34x |
| fold_light | 1048576 | 4 | 135.3 us | 130.0 us | 191.5 us | 1.04x, 0.71x |
| fold_heavy | 1048576 | 4 | 2.51 ms | 2.19 ms | 2.45 ms | 1.15x, 1.02x |
| fold_heavy | 65536 | 4 | 195.6 us | 266.0 us | 181.3 us | 0.74x, 1.08x |
| append | 16777216 | 4 | 15.50 ms | 7.13 ms | 9.33 ms | 2.17x, 1.66x |
| append | 1048576 | 4 | 596.1 us | 389.7 us | 824.7 us | 1.53x, 0.72x |
| fold_light | 134217728 | 8 | 47.03 ms | 25.27 ms | 25.30 ms | 1.86x, 1.86x |
| fold_light | 16777216 | 8 | 2.67 ms | 1.42 ms | 1.30 ms | 1.88x, 2.05x |
| fold_heavy | 1048576 | 8 | 2.90 ms | 1.88 ms | 1.97 ms | 1.54x, 1.47x |
| append | 16777216 | 8 | 15.92 ms | 4.95 ms | 8.46 ms | 3.22x, 1.88x |

Holds for: W in {4, 8}, the n listed; W = 8 runs eight walkers on four vCPUs. The append lines compare N-way with the sequential fallback head-and-tail takes there. The fold lines put N-way's lead over head-and-tail at about what its extra walkers would give where the frame is memory-bound (fold_light n = 134217728: 1.81x at W = 4, 1.86x at W = 8, the machine having four vCPUs), and below it where the kernel is compute-bound (fold_heavy n = 1048576: 1.15x at W = 4, 1.54x at W = 8).

What each costs on short fibers. On fold_light fibers of 16 to 1024 records (one to four morsels), every parallel arm costs more per frame than the engine's sequential path, at every W measured, and the cost does not move with n:

| arm | W = 1 | W = 2 | W = 4 | W = 8 |
| --- | --: | --: | --: | --: |
| sequential (engine) | 7.6 to 11.1 us | 7.5 to 10.4 us | 7.2 to 9.6 us | 7.1 to 9.6 us |
| kernel_seq (crew, one walker) | 21.2 to 23.7 us | 21.0 to 30.3 us | 19.9 to 24.7 us | 19.8 to 22.0 us |
| head_tail | 20.6 to 24.6 us | 27.1 to 34.2 us | 27.3 to 33.8 us | 28.6 to 31.4 us |
| kernel_nway | 22.6 to 27.0 us | 29.8 to 31.8 us | 40.1 to 45.4 us | 66.6 to 76.8 us |
| nway (engine) | 34.0 to 35.8 us | 40.3 to 49.2 us | 49.4 to 62.2 us | 74.2 to 97.2 us |
| std_parallel | 38.8 to 47.4 us | 51.6 to 63.9 us | 84.1 to 100.2 us | 163.9 to 188.5 us |

Holds for: short, n in {16, 64, 256, 1024}, warm, unpinned. Each range is the lowest and the highest median over the four n. Head-and-tail's cost stops growing past W = 2 because it never wakes more than two walkers; the N-way arms wake W.

Most of a crew frame's cost here is the wake of a parked worker on another vCPU, and that cost is bimodal per process. Pinned to one CPU (`results-pinned-one-cpu/`, W = 1), the crew arms cost 1.7 to 3.1 us per frame in every pass and the engine's `nway` 11.3 to 17.0 us; unpinned, a process either gets the same 1.7 to 2.4 us or 20 to 37 us, and keeps it for the whole pass. Holds for: short, W = 1, n in {16, 64, 256, 1024}. The idle driver of this guest is `none`, so an idle vCPU halts and its wake goes through the hypervisor; that is the likely mechanism and it was not measured directly.

## Setup

The machine is a KVM guest with 4 vCPUs of an Intel Xeon at 2.10 GHz (family 6 model 207), one thread per core as the guest sees it, 2 MiB of L2 per core, 260 MiB of L3, 15 GB of memory, Linux 6.18.44, cpuidle driver `none`, transparent huge pages on `madvise` (the bench's columns do not ask for them), clocksource `tsc`. How the vCPUs sit on host cores is not visible from the guest. The machine maximum worker count is therefore 4, and W = 8 oversubscribes it two to one.

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

Head-and-tail inside the engine. The engine emits no `Head` or `Tail` range, so these numbers are the split on the bench's crew, built from the engine's frame protocol. What the engine's dispatch, plan and merge would add or remove around it is not measured.

Fibers in a full pipeline. One fiber runs alone per frame here, with one unit on it. Several trunks, several units per fiber, and the phase and waist structure around them are not measured.

The mechanism of the bimodal wake. The pinned run shows the slow mode needs a second CPU; the hypervisor exit is inferred from the idle driver and not traced.

Why compute-bound scaling stops near 2x at W = 4. fold_heavy at n = 1048576 runs 1.86x faster on four `kernel_nway` walkers than on one (4.08 ms to 2.19 ms) and `std_parallel` 2.4x (1.70 ms). Host placement of the vCPUs, frequency and the kernel's own throughput were not separated.

The spread at large n. Some arms at n = 16777216 are bimodal per pass (fold_light W = 2 `kernel_seq`: passes at 3.52 ms and 11.35 ms). Huge pages are ruled out (`madvise`, not requested); the cause is not established.

A floating meeting point finer than a block. `head_tail_dyn` claims 256-record blocks; the per-record claim of the head-and-tail sketch is not measured.

Other machines. Every number holds on this guest only, and a bare-metal machine with a real idle driver would change the short-fiber costs most.

## Disturbances

One W = 1 run was voided by background builds (the SessionStart `agent-regen` over another repository, and a commit's pre-commit hook) and rerun in full on an idle machine; ticket 7ddf5ef52d08. During the W = 2 run the disk-trim hook freed 1.9 GB around fold_heavy n = 65536; that section's numbers are kept. The W = 4 run was lost to a container restart and rerun in full.

## Files

`results/<section>/` holds the harness's CSV, meta and findings per section and size. `results/summary.csv` and `results/summary.md` are `single-trunk-convergence --summary results`: per section, size, mode and arm, the median, quartiles, fastest batch, the lowest and highest per-pass median, and the ratio to `sequential` and to `kernel_seq`. `results-pinned-one-cpu/` is the short W = 1 section run under `taskset -c 1`.
