# The bench matrix for the region hilavitkutin is actually for

Hilavitkutin is for workloads with millions of records of simulation and crunching work going
constantly. The benches in the tree today do not measure that. Twenty-nine of the thirty-four
registered benches are microbenchmarks over 64 to 16384 bytes, answering lowering questions
(popcount, rotate, modulo, branch shape) that have nothing to say about a frame. The one bench
that runs the engine, `engine_vs_std`, is not on the harness at all: it has its own `Instant`
loop, its own iteration budget and no confidence intervals, no cold mode and no subprocess
isolation. Its largest size is 4194304 records, which is the only point in the tree that touches
the intended region.

Everything the engine arms measure today is an engine without its headline features. Op, on 13
November: the benches that compare hilavitkutin against a hand-written optimised path and against
plain Rust measure something that "doesn't yet include any of the actual headline features like
fusing and the adaptive trunk/fiber/morsel crunching on the datadlow-ish schedule", and "no calls
should be made based on those yet, they are a baseline to compare against then". That governs
every findings note this matrix produces. A number here is a baseline for a later comparison and
decides nothing.

This document fixes the matrix before any bench is written: which scenarios, which arms, which
sizes, what each will be able to say later, and in what order they get built. It is laid out
first so the arms and the claims are settled while there are no numbers to be tempted by. The
companion note `202611131250_bench-harness-at-a81e195.md` carries what the harness offers and
which of it the current benches use, with a path and line for each; this one builds on it and
does not repeat it.

## What the harness offers that these benches have never used

Five of the thirty capabilities in the companion note decide the shape of everything below.

Threaded benches are supported and no hilavitkutin bench declares one. `threaded = true` in a
bench section removes the coordinating thread's self-pin and is passed to every worker
(`bench-harness/src/config.rs:262`, `bench-harness/src/harness.rs:284,770`). A variant may spawn
threads and the run block is measured as wall-clock elapsed time
(`bench-core/src/lib.rs:27-39`). Without that key, a threaded arm measures a pinned coordinator
and is wrong.

Validation runs on the driver path only. Both hilavitkutin bench binaries call `harness::run`,
which times whatever dylibs the config names and checks nothing
(`bench-harness/src/lib.rs:13-27`), and neither has the `--mode validate` arm a validation worker
needs, so every variant would be skipped silently for returning zero outputs
(`bench-harness/src/validation.rs:196-208`). The comments in the tree that claim byte-exact
cross-variant validation describe a constant on a bridge type that nothing currently reads.

A heavyweight per-process state is explicitly sanctioned. Each worker is a dedicated subprocess
per variant, mode and pass, so a variant may cache a built scheduler or a loaded dataset in a
process-local static keyed by `n`, and at least one warmup call always runs before any timed
batch, so the build lands in untimed territory (`bench-core/src/lib.rs:41-49`). That is what makes
an engine arm possible at all under this ABI.

Per-bench timing overrides exist and hilavitkutin sets one global `[timing]` block instead
(`bench-harness/src/config.rs:350`, `mock/benches/bench.toml:768-773`). A frame over sixteen
million records cannot share a budget with a popcount over sixty-four bytes.

The worker deadline comes from the routine. With no `max_call_us` a worker is killed at 300
seconds and yields nothing; with one, the deadline is the larger of five seconds and three times
the expected total (`bench-harness/src/harness.rs:759-766`). It is not a manifest key
(`bench-harness/src/config.rs:875-876`), so a bench whose call is milliseconds rather than
nanoseconds needs its own `Routine` implementation rather than `ByteRoutine`.

## The four scenarios

Each scenario is one bench member with its own manifest. The record count sweep is shared and the
arms are shared; what differs is the shape of the frame.

### `frame_wide`: one simulation frame, many work units over many columns

Twelve work units over ten columns in a DAG that is not one chain: three independent three-stage
chains, a fan-in stage that joins them, and two independent single-stage transforms. That gives
the plan real phases, trunks and fibers to find, where the existing `element_wise` gives it one
linear chain and `wide_parallel` gives it four disjoint ones. The measured block is one frame over
the whole record range.

Two sweeps on this member, because the answer differs by regime and one number for both would be
a lie. The `heavy` sweep gives each record real compute, so per-record work dominates dispatch.
The `light` sweep makes each stage a single wrapping multiply, so the frame is memory-bound and
the cost of materialising intermediate columns is the whole story. Fusing is aimed squarely at the
second; the adaptive spread at the first.

### `sustained`: crunching that goes constantly

The measured block is `M` frames back to back on one built scheduler, with the input marked dirty
each frame so no frame is skipped. `M` is chosen per size so the block clears the timing floor and
stays inside the deadline. This is the shape op's words describe: not one frame measured once, but
the steady-state per-frame cost of a pipeline that keeps running. It is also the only shape where
the schedule-once-reuse design and the adapt EMA have anything to work with, since both need more
than one frame to show an effect.

The non-engine arms run the same `M` iterations over the same preallocated buffers, so the
comparison is steady state against steady state.

### `request_batch`: a daemon's request and batch pipeline

The measured block is one service interval: `K` small batches of 1024 records each, then one large
batch of the sweep's record count. The small batches are where the engine's fixed per-frame cost
is paid most often, and op has already ruled on how to read that: one to four morsels is almost
never a realistic case for hilavitkutin, so a result there is a boundary note and never a verdict
on the engine. The bench exists because a daemon really does mix the two, not to relitigate the
small case.

This is the scenario the adaptive single-versus-parallel selection is for, so it is the one whose
later number will show whether that selection works.

### `mixed_stages`: a generic application's unlike stages in one frame

One frame mixing heavy per-record units, light per-record units and resource-only units over the
same record count. The engine builds with one record count for the whole scheduler
(`mock/crates/hilavitkutin/src/scheduler/builder.rs:167-174` takes a single `record_count`), so a
frame whose columns have genuinely different lengths is not expressible today. That is a finding
rather than a bench I can write: the scenario as op described it, small and large stages in one
application, is approximated here by cost per record rather than by record count, and the gap is
filed as a ticket.

## The arms

The same eight arms wherever they are meaningful. Four of them are the engine, four are the bar it
is measured against. Every arm computes the same outputs and is checked against the reference
before any timing counts.

| Arm | What it runs | What it is the bar for |
|---|---|---|
| `engine` | `Scheduler::run`, single core | the general per-unit dispatch path |
| `engine_fused` | `Scheduler::run_fused`, single core | the within-fiber linear fusion entry, where the shape is a fusible chain |
| `engine_par` | `Scheduler::run_parallel` on a `ThreadPoolApi` executor | the per-core trunk dispatch |
| `hand` | a hand-written optimised path, fused, single threaded | the best a person writes for this workload |
| `hand_par` | the same hand-written path, threaded over the same core budget | the fair bar for `engine_par` |
| `rust_nostd` | the obvious loop over preallocated slices, no std facilities | what the workload costs written plainly without a heap |
| `rust_std` | the idiomatic std shape, `Vec` and iterators | what most consumers would write |
| `rust_std_par` | the idiomatic std shape threaded with `std::thread::scope` | the idiomatic parallel bar |

The threaded arms are the reason the bench sections declare `threaded = true`. Comparing a
multi-core engine against a single-threaded loop flatters the engine, and this tree has already
made that mistake once and corrected it (`mock/benches/engine_vs_std/src/lib.rs:320-327`). Every
parallel claim here is engine against an equally threaded bar, and the engine against a serial
loop is reported as context only.

## Sizes

The main sweep is 1048576, 4194304 and 16777216 records. Those are the intended region and the
headline of every findings note. The harness has already carried 16777216 in this tree
(`mock/benches/resource_storage/bench.toml:102-103`), so the size is known to be reachable rather
than hoped for.

One boundary point at 65536 is carried on each bench and reported separately, with its own
predicate and a sentence saying it is a boundary note rather than a verdict. Nothing below that is
measured: a popcount sweep already covers the small end for the questions the small end can
answer.

## What each bench will be able to say once the headline features exist

None of these benches decides anything now. Each is built so that the same run, repeated after
fusing and the adaptive trunk, fiber and morsel work land, answers one question by the difference.

`frame_wide` light measures the intermediate-column traffic a fused hand path never pays, which is
exactly what within-fiber fusion is meant to remove, so the delta between the two runs is the
measure of that feature on a real frame rather than on a four-stage toy. `frame_wide` heavy
measures whether the engine's spread across cores is as good as a person's chunking when
per-record work dominates, which is what the adaptive trunk and fiber work is meant to earn.
`sustained` measures per-frame cost at steady state, so it is where the adapt EMA either moves the
number over a long run or does not. `request_batch` measures the fixed per-frame cost against a
small batch, so it is where adaptive single-versus-parallel selection either removes the small
case loss or does not. `mixed_stages` measures the cost of one global morsel size across unlike
work, so it is where per-fiber morsel sizing either pays or does not.

## What has to exist before any of them

Four pieces, and every bench in the matrix waits on them, so they are the first work.

The bench driver has to compile against the harness as it is on dev. The consumer calls
`harness::run_worker` with eleven arguments and the current signature takes twelve, with
`threaded` last (`mock/benches/src/main.rs:297-309`,
`mock/benches/resource_storage/src/main.rs:235-238` against
`bench-harness/src/harness.rs:267-280`). Nothing in the tree has a committed lockfile, so the git
dependency resolves to whatever dev is. This is checked by building, not by reading, and it is
checked first.

The driver needs a `--mode validate` arm dispatching to `harness::run_worker_validate`, and the
benches need to go through the validating path rather than `harness::run`. Without it the
correctness check this matrix promises does not exist, however many comments say it does.

The engine arms need one support crate that builds a scheduler in a process-local static keyed by
record count and exposes a thin entry per arm, so eight variant crates per bench do not each
re-derive the pointer walk that `engine_vs_std` does by hand. Reuse is the pillar here: one
abstraction the arms share, never a copy per arm.

The engine benches need their own `Routine` rather than `ByteRoutine`, to set `max_call_us` so the
worker deadline scales with a call measured in milliseconds, and to build a multi-megabyte input
through `build_input_bytes` rather than by value.

## Correctness before any timing counts

Every arm writes the same outputs and the run is validated across arms byte-exact, with
`may_differ = false` and `required = true` in every section, so an arm that drifts fails the run
rather than quietly producing a faster wrong answer.

The negative control is the part that makes that claim worth anything. A deliberately wrong arm,
one stage dropped from the chain, is run through the validation path once and shown to be
rejected, in a committed test. Until that test has been seen to fail for the right reason, the
validation is an assumption. A green validation over arms that were never shown to be capable of
going red says nothing.

## The order of work

The driver compiling against dev comes first, because every other piece is downstream of it and it
is the cheapest to settle. The validate arm and the shared engine support crate come next, in one
piece, since the support crate is what the validate arm validates. Then `frame_wide`, both sweeps,
because it is the headline and because building it exercises every part of the support crate the
others reuse. Then `sustained`, which is the same arms with a different measured block. Then
`request_batch` and `mixed_stages`. Last, the existing benches move onto what the harness now
offers and gain the large sweep where they have only small sizes.

Each bench lands as its own ready and its own landing, with its results and its findings note
committed, rather than one landing at the end.

## Budget, and the machine this runs on

The host is four cores of an Intel Xeon at 2.1GHz with 16GB of memory and 17GB of free disk. Ten
columns of 16777216 four-byte records is 640MB, which fits, and the workers run one at a time.

A cell costs `harness_runs x passes x cooldowns x 2 modes x arms` worker subprocesses
(`bench-harness/src/harness.rs:714-752`). At one harness run, three passes, one cooldown and eight
arms that is forty-eight workers per bench and size. The timing knobs per bench are set from a
measured per-frame cost before the first full run, not guessed, and any run that would pass thirty
minutes is split and gets a ticket.

## What this matrix leaves out, and why

The single-trunk convergence bench on `work/bench-convergence` is another job's and is not touched
here. The bench variants on `work/recover-landing-source` wait until that branch is on dev, and
are taken in by a merge afterwards.

Hardware counters are not available: they need the `perf-counters` feature on macOS aarch64 as
root, and the Linux promise in the harness manifest has no code behind it
(`bench-harness/src/perf.rs:74`). Every finding here rests on wall-clock medians with bootstrap
confidence intervals and no instruction counts.

Pinning does nothing on this host either: `pin_to_perf_cores` is empty off macOS
(`bench-core/src/counter.rs:176`). Idleness is established by the machine's load rather than
assumed, and how it was checked is stated in each findings note.
