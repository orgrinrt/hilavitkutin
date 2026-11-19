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

Holds for: toolchain `nightly-2026-05-28`, x86_64 linux, a 4-core container; the support crate's test profile (`opt-level = 2`); arvo rev `1b2796e9`; notko `8a74dbcd` (the lock seeded from `mock/Cargo.lock`); seed `0x5EED_0000_0000_0001`; the record counts, group widths and worker counts named above; one column, one fold, the bench's crew. It does not hold for the engine, which the sketch covers on a copy, nor for a carrier with several units, meta units, leading-band appends or a fold into a resource.

## B1b: the price

Not run yet. The bench is `accumulator_grouped`, six sections: W = 1, 2, 3 and 4 at G = 64, and G = 8 and 1024 at W = 4, each at 4096, 65536 and 1048576 records, with `per_core` as the baseline and every arm validated against its own model before timing.

On hilavitkutin `dev` at `3177527a` no bench of the tree builds: the generated driver and the arms resolve notko `dev` at `15a95316`, where `Project` takes a carrier parameter, and `hilavitkutin-api` fails with E0107 at its `WriteSet` bound. An engine-free bench, `rotate_strategy`, fails at the driver the same way. The tickets are `hilavitkutin_names_notko_by_branch_dev_so_a_consumer_pinning` and `bench_members_follow_notko_on_branch_dev_with_every_cargo_lo`, and mockspace's `bench_run_regenerates_a_member_arm_s_workspace_lockfile_from`, which is why a lock seeded with the working notko does not survive a run. The numbers are added here, with their run directories, once the tree builds.

## What is not established

The cost of option 3 on the engine, where the merge, the region capacity and the refusal behaviour under the rounded split are the engine's and not this model's. Where G would live in a design (the morsel, a plan dimension, the run configuration). Any count above 256, Miri, any other target.
