# The accumulator grouped by the data, as a bench

This note is roadmap steps B1a and B1b for option 3 of the fold split, the accumulator grouped by the data (clause-dev `.shared/research/202611150830_stack-path/40-the-roadmap-final.md:97-99`, agenda ticket `bench_the_accumulator_grouped_by_the_data_as_a_model_in_the`). The maintainer asked for option 3 to be benched first ("Bench the option 3 and depending on its result something like option 1 or a new approach or bench finds one") and ruled that the price carries no pass line: the numbers are recorded with their run and go to him (file 42 of that record, "The pass line of the morsel-grouped bench").

The bench is a model inside the bench tree and changes nothing shipped. It lives in `mock/benches/accumulator_grouped/` with its support crate `mock/benches/support/accum-group-support/`. Its crew of parked workers is `evs-support`'s, and its per-process state slot, ceil slice and frame protocol are `stc-support`'s. The engine-side evidence for the same grouping is the sketch `mock/research/sketches/202611152345_accumulator-grouped-by-data/`, which ran it on a copy of the engine with `accum_per` rounded to whole groups.

Nothing here is authority. It was written unattended, and it is evidence for a call that is the maintainer's: four ratified rows send a fold that is not commutative to one core (the sketch's FINDINGS, "The canon, and what adopting this would take"), and option 3 splits it across cores on a grid fixed by the data.

## The model

A fold runs under arvo's signed saturating addition, `IFixed<7, 0, Precise>`, which is not associative. Every walker cuts its record slice into runs from the slice's first record, folds each run on its own and writes one partial per run into its own region; after the frame the regions are compacted in core sequence and the partials folded left to right. This is the engine's threaded accumulator path, with the merge inside the frame. Three schemes are timed and a fourth is held to failing:

1. `sequential`: one walker over every record. The canon's answer, and the one result that is the left fold.
2. `per_core`: the ceil slice, one partial per core. The engine's shape today and the price's baseline.
3. `grouped`: option 3. Groups of G from record zero, one partial per group, each core's slice rounded up to whole groups (`group_slice`, the ceil slice over the group count scaled back to records), the same rounding the sketch's `accum_per` change makes.
4. `grouped_on_ceil`: groups of G from each core's slice start on the unrounded ceil slice. This is what a unit that groups by itself gets on the engine as shipped. Not timed.

Every arm's output carries its scheme, and the routine validates it against that scheme's hand model rebuilt from the seed. The hand model of `grouped` is the column cut into chunks of G and folded, with no worker count in it.

## B1a: the equality

`tests/equality.rs` of the support crate runs each scheme through the same frame the timed arm runs, on real worker threads, at 1, 2, 3, 4 and 256 workers. 256 is the engine's `MAX_CORES` and the crew's clamp.

`grouped` gave the same digest, partial count and combined value at all five counts, equal to its hand model, at every record count in 1, 3, 8, 9, 255, 1001, 2049, 6144 and 65537 and every group width in 8, 64 and 1024 (`grouped_by_the_data_gives_the_same_bytes_at_every_worker_count`).

The cases that must fail, failed. `per_core` gave more than one result across the five counts at 1001, 6144 and 65537 records, each equal to its own model at its count. `grouped_on_ceil` at 1001 records and G = 8 gave more than one result: at 2 workers the slice is 501 records and cuts a group. At 6144 records, where every count's slice is a whole number of groups, it agreed with `grouped`. The grouped result at one worker is not the left fold of the records (-110 against -42 in the sketch's inputs; here `the_grouped_result_is_not_the_left_fold_of_the_records`): option 3 fixes the answer across worker counts by changing what the answer is.

`tests/law.rs` holds the law's witness, `(100 + 100) + -100 = 27` against `100 + (100 + -100) = 100`, the clamp at both bounds, and that no grouping, groups of 8 and groups of 7 give three different results on the bench's inputs, so an equality across counts is one grouping and not inputs the law forgave.

The routine's check is held to refusing: `tests/validate.rs` changes one field of a real output (a bit of the digest, the combined value, the partial count, the worker echo, the scheme id) and each is refused, and so are the `per_core` partials of two workers presented at three; the grouped partials of two workers presented at three pass, which is the claim. `tests/group_slice.rs` holds the rounded split at every walker count from 1 to 256: the slices are in sequence, disjoint, cover the records and start on a group boundary, beside the unrounded ceil slice failing the same check at 1001 records.

Holds for: toolchain `nightly-2026-05-28`, x86_64 linux, a 4-core container; the support crate's test profile (`opt-level = 2`); arvo rev `1b2796e9`; notko `15a95316` (the support crate's own lock resolved fresh; a lock left from before the port pins `8a74dbcd` and does not build); seed `0x5EED_0000_0000_0001`; the record counts, group widths and worker counts named above; one column, one fold, the bench's crew. It does not hold for the engine, which the sketch covers on a copy, nor for a carrier with several units, meta units, leading-band appends or a fold into a resource.

## B1b: the price

Two runs, under `mock/benches/results/accumulator_grouped/<section>_n<n>/linux-x86_64-c4-container-intel/`. The first, `20261119T211727Z-abb93e97`, is the branch at `abb93e97` (dev's notko port merged). The second, `20261119T215331Z-a2c60c57-dirty`, is the branch at `a2c60c57` with the review's fixes in the tree: the arms' code is the same, only an arm's doc comment and the hooks' miss handling had changed, which is the `-dirty`. Profile for both: the harness's pinned release (opt-level 3, fat LTO, one codegen unit), the `light` workload around each call, warm regime, three passes of 60 runs in batches of 10. Six sections, W = 1, 2, 3 and 4 at G = 64 and G = 8 and 1024 at W = 4, each at 4096, 65536 and 1048576 records. Validation passed in all 18 cells of each run, three seeds each: every arm equal to its own hand model, so `grouped` gave its worker-count-free model at W = 1, 2, 3 and 4 in every cell, which is B1a again inside the timed tree. The check is shown refusing in `tests/validate.rs`.

The price is `grouped` against `per_core`, the median of each, with the absolute delta first. The first run:

| W, G | 4096 records | 65536 records | 1048576 records |
|---|---|---|---|
| 1, 64 | +6.95 us (35.06 against 28.11, 1.25x) | +73.0 us (380.58 against 307.55, 1.24x) | +0.94 ms (5.73 against 4.79, 1.20x) |
| 2, 64 | +4.64 us (40.56 against 35.92, 1.13x) | +31.5 us (202.15 against 170.62, 1.18x) | +0.37 ms (2.89 against 2.51, 1.15x) |
| 3, 64 | -1.2 us (40.65 against 41.85, 0.97x) | +21.7 us (132.55 against 110.84, 1.20x) | +0.14 ms (1.84 against 1.70, 1.08x) |
| 4, 64 | +2.15 us (46.93 against 44.78, 1.05x) | +9.6 us (161.69 against 152.07, 1.06x) | +0.28 ms (2.59 against 2.31, 1.12x) |
| 4, 8 | -3.2 us (45.46 against 48.63, 0.93x) | +36.1 us (168.34 against 132.25, 1.27x) | +0.68 ms (3.07 against 2.38, 1.29x) |
| 4, 1024 | -0.7 us (42.93 against 43.64, 0.98x) | -42.4 us (137.89 against 180.33, 0.76x) | -0.04 ms (2.20 against 2.24, 0.98x) |

The second run:

| W, G | 4096 records | 65536 records | 1048576 records |
|---|---|---|---|
| 1, 64 | -1.9 us (29.46 against 31.32, 0.94x) | +24.7 us (337.76 against 313.11, 1.08x) | +0.32 ms (4.89 against 4.57, 1.07x) |
| 2, 64 | +0.9 us (32.25 against 31.34, 1.03x) | +29.8 us (164.57 against 134.79, 1.22x) | +0.06 ms (2.55 against 2.49, 1.03x) |
| 3, 64 | -6.8 us (40.75 against 47.54, 0.86x) | +13.7 us (125.62 against 111.92, 1.12x) | +0.03 ms (1.81 against 1.78, 1.02x) |
| 4, 64 | +5.1 us (46.35 against 41.21, 1.12x) | +28.6 us (166.92 against 138.29, 1.21x) | +0.22 ms (2.56 against 2.34, 1.09x) |
| 4, 8 | +9.1 us (53.77 against 44.67, 1.20x) | +38.2 us (209.94 against 171.78, 1.22x) | +0.72 ms (3.17 against 2.45, 1.30x) |
| 4, 1024 | -7.4 us (40.96 against 48.33, 0.85x) | -12.1 us (131.57 against 143.63, 0.92x) | +0.00 ms (2.28 against 2.28, 1.00x) |

`sequential`, the one walker, took 4.54 to 4.97 ms at 1048576 records in every section of both runs and 24 to 31 us at 4096, where it was the fastest arm in every section with more than one worker: at that size the crew's frame costs more than the split saves, for both split arms alike.

What the two runs carry, as a model. One cell agrees across them: at W = 4 and G = 8, 1048576 records, option 3 costs 0.68 and 0.72 ms over 2.38 and 2.45 (1.29x and 1.30x), where it writes a partial per 8 records, 131072 of them. That is about 5 ns per partial, about what `per_core` spends on one record under this law (4.57 to 4.79 ms over 1048576 records at one worker, 4.4 to 4.6 ns). At G = 1024 it costs nothing measurable in either run (0.98x, 1.00x). At G = 64 and 1048576 records the runs disagree: 1.20x against 1.07x at one worker, 1.15x against 1.03x at two, 1.08x against 1.02x at three, 1.12x against 1.09x at four. The first run's one-worker gap is not reproduced, so it is not read as a cost: 0.94 ms over 16384 partials would have been 58 ns a partial, which no mechanism in the arm accounts for, and the second run's 0.32 ms is 20 ns. What holds over both runs at G = 64 and 1048576 records is a cost between 2 and 20 percent, and the bench cannot place it more closely on this machine. At 65536 records and below the deltas are of the size of the noise (below) in both runs.

The disassembly of both arms' `bench_entry` is committed under `results/accumulator_grouped/disasm/linux-x86_64-c4-container-intel/` (runs `20261119T221408Z` and `20261119T221414Z`). It shows only the dispatch on the record count into the arm's `visit_group` instance; the walkers' fold is not an exported symbol, so the per-partial cost is not read from it.

The noise: every W = 4 cell is the whole of a 4-core container, the harness's process included, and the reports flag an alternating pattern in most cells (autocorrelation -0.51 to -0.66, "throttle bounce"). `per_core` does the same work at G = 64 and G = 1024 and read 152.07 and 180.33 us at 65536 records in the first run, so a difference under about 20 percent at W = 4 and 65536 records says nothing here, and the 0.76x of `w4_g1024` at that size is that noise. The run-to-run spread at W = 1 and G = 64, 1048576 records, is 0.84 ms on `grouped` (5.73 against 4.89 ms) and 0.22 ms on `per_core`, so a single run's delta under about a fifth is not a result on this container. The upper ends of the harness's intervals are in each cell's `summary.tsv`.

Holds for: x86_64 linux, a 4-core container (`linux-x86_64-c4-container-intel`), toolchain `nightly-2026-05-28`, the release profile named above; arvo `1b2796e9`, notko `15a95316`; two runs; the law `IFixed<7, 0, Precise>`; the model on the bench's crew and not the engine; G = 8, 64 and 1024; W = 1 to 4; the record counts named. No pass line, by the maintainer's ruling: these are a baseline for his call and not a verdict.

## What is not established

The cost of option 3 on the engine, where the merge, the region capacity and the refusal behaviour under the rounded split are the engine's and not this model's. Where G would live in a design (the morsel, a plan dimension, the run configuration). The cost at G = 64 more closely than 2 to 20 percent, and what a partial costs in the walker, which needs the fold exported as a symbol to disassemble. Any count above 256, Miri, any other target.
