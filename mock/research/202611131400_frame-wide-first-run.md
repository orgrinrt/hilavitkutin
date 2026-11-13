# Frame Wide, First Run at 65536 Records

This note records the first end-to-end run of the `frame_wide` bench member at its boundary size, 65536 records, on 2026-11-13. The run exists to prove the machinery: a support crate holding the workload once, seven arms that are thin cdylibs over it, a driver that validates every arm against the reference and against each other before it times anything, and a committed negative control that shows the validation refusing a wrong frame. The numbers are what that machinery printed. They are a first look, taken with deliberately small timing knobs, and nothing below decides anything.

The engine arms in particular measure the engine as it stands. In op's words, 13 November: the engine arms measure something that "doesn't yet include any of the actual headline features like fusing and the adaptive trunk/fiber/morsel crunching on the datadlow-ish schedule. It's bare bones, barely lowers to anything interesting yet. So no calls should be made based on those yet, they are a baseline to compare against then." The intent there is that the engine numbers are a baseline for later comparison and carry no verdict; the rest is kept for provenance.

The frame is one simulation step over one input column: three independent pipelines of three stages each, a fan-in stage joining their three tails, and two independent single-stage transforms of the input. That is twelve work units over thirteen columns (the input, nine pipeline columns, the join and the two transforms). The plan in `202611131300_bench-matrix-for-the-intended-region.md` says ten columns for the same shape; the shape it describes needs thirteen, and the work unit count, twelve, is as planned. Two sweeps run it. `heavy` gives every stage eight rounds of a multiply, xor and shift mix, so the frame is bound by arithmetic. `light` gives every stage one wrapping multiply, so the frame is bound by memory traffic and by what each arm pays per stage.

## What was run

The member lives at `mock/benches/frame_wide/` and is one cargo workspace: the driver, `support/frame-wide-support`, and eight variant crates under `variants/` (seven arms and the negative control). It is built and run from that directory:

```
CARGO_INCREMENTAL=0 cargo build --release --workspace --all-targets
<target-dir>/release/frame_wide --n 65536
CARGO_INCREMENTAL=0 cargo test --release --test negative_control
```

`cargo mock bench list` shows the member as `frame_wide/heavy` and `frame_wide/light`. `cargo mock bench run` cannot run it, because the launcher hands a member key to the repo's root driver, which knows only the root manifest (ticket `c9d2fc212037`).

Build profile: release, `opt-level = 3`, `lto = "fat"`, `codegen-units = 1`, no `target-cpu` flag, so baseline x86-64 code generation. Toolchain `nightly-2026-05-28`. Harness: mockspace `a81e195`. Host: four cores of an Intel Xeon at 2.1GHz with AVX2 present but unused by the build, 16GB of memory. No pinning (it does nothing off macOS) and no hardware counters, so every figure is a wall-clock median with a bootstrap interval. Timing: two passes of twenty runs, batches of five calls, one harness run, one zero-millisecond cooldown cohort, warm and cold modes, eight samples per arm per mode. The host was idle apart from the bench while it ran: a process listing during the run showed the bench and its workers, and nothing else above a few percent of one core.

The seven arms are these. `engine` is the twelve work units on `Scheduler::run`. `engine_par` is the same scheduler on `Scheduler::run_parallel` over a `ThreadPoolApi` executor with four workers, spawned once and parked between frames. `hand` computes the whole frame per record in registers and writes only the three output columns. `hand_par` runs `hand` over one contiguous chunk of records per core. `rust_nostd` materialises every intermediate, one index loop per stage, into buffers allocated before the timed region. `rust_std` writes the same twelve passes as `map` and `collect` into fresh vectors every frame. `rust_std_par` runs `rust_std` over one chunk of records per core. Both chunked arms spawn their threads with `std::thread::scope` inside every frame, so they pay a spawn and a join per frame that `engine_par` does not.

`run_fused` is not an arm. It needs a `FuseCarrier`, which folds a linear read-after-write sequence of `RecordOp` maps, and this frame is a graph with a fan-in, so there is nothing for it to fold.

## Correctness before timing

Every cell went through `validation::validate` before `harness::run`, called from the member's driver, with `may_differ = false` and `required = true` in both sections. The routine overrides `validate_output` to recompute the expected digest from the reference `record` alone, so each arm is held to the reference on its own as well as byte-exact to its neighbours. The digest is the FNV-1a hash of the join column and of the two transform columns; every pipeline feeds the join, so a stage dropped anywhere moves it.

Both cells passed: all seven arms agreed with the reference and with each other over four seeds of 65536 records each, and all seven were deterministic over the same four seeds. The driver also refuses a cell in which any arm was dropped before comparison, which the harness otherwise allows.

The negative control, `tests/negative_control.rs`, runs `hand`, `engine` and `drop_stage` together through the same validation path at 4096 records and three seeds, in both sweeps. `drop_stage` is the fused frame with the second stage of the first pipeline missing. In both sweeps the run was refused, the refusal named `drop_stage`, and it counted exactly six mismatches across three seeds: one per seed against the reference and one per seed against `hand`. `engine` contributed none. A check refusing everything would have counted `engine` too, and a check doing nothing would have counted zero, so that one count carries both halves of the control.

## Per-call cost at 65536 records

The figures are the medians of the timed region only, which is the frame. Loading the input and computing the digest sit outside it for every arm; the findings files call that span the bridge, and it is about one millisecond a call at this size, nearly all of it the digest.

| Arm | heavy, ms | light, µs |
|---|---|---|
| `engine` | 2.79 | 219 |
| `engine_par` | 2.74 | 345 |
| `hand` | 3.42 | 107 |
| `hand_par` | 1.35 | 271 |
| `rust_nostd` | 2.73 | 194 |
| `rust_std` | 3.06 | 175 |
| `rust_std_par` | 1.61 | 603 |

The full reports, with intervals, are `mock/benches/frame_wide/heavy_n65536_findings.md` and `light_n65536_findings.md`, beside their CSVs and meta files.

Some of what the table shows is a property of the bars rather than of anything they are compared against, and is recorded so that nobody reads it as more. Under `heavy`, `hand` is slower than the column-at-a-time arms although it touches far less memory; all three vectorise the kernel's multiply with SSE2 `pmuludq`, and why the fused form loses was not looked into. Under `light`, both chunked arms are slower than their single-threaded forms, which at a frame of a fifth of a millisecond is consistent with the per-frame thread spawn they pay. With eight samples an arm, the intervals are wide and the per-pass autocorrelation the reports flag is an artefact of two passes.

## What a large cell costs

The large sweep (1048576, 4194304 and 16777216 records) is configured in `bench.toml` and was not run. On x86_64 every worker process first sleeps three seconds calibrating its counter (ticket `3ec38cb5c7c8`), and one cell at these knobs spawns 63 workers across the seven arms: five validation probes, one validation worker and four timing workers each, plus one sanity worker. That is about three minutes of fixed cost per cell before any frame runs. Each measured cell at 65536 took about three minutes and twenty seconds from start to finish (13:47:50 to 13:51:09 for `heavy`, to 13:54:26 for `light`, from the output files' times), of which about 90 seconds were the timed phase, and nearly all of both phases was that sleep.

Scaling the measured frame cost linearly with the record count, which a frame that leaves the cache will not honour, and adding the untimed load and digest at the measured rate of about 15 ns a record, a `heavy` call at 16777216 records costs about one second and a `light` call about a third of a second. Each arm makes about 110 calls per cell between validation and timing. So one `heavy` cell at 16777216 records is about thirteen minutes of calls plus the three minutes of fixed cost, about a quarter of an hour, and a `light` one about eight minutes. At 1048576 records the fixed cost dominates and a cell is four or five minutes. These are extrapolations from one size, not measurements, and the first large run should replace them.

## Tickets

`ef57305fb5f4`: the engine has no supported host-side column write between frames; the engine arms reach the input column through `Scheduler::__bindings()`. `3ec38cb5c7c8`: the harness's three-second counter calibration per worker outlasts the two-second preflight probe, which is why `max_call_us` has a 400-millisecond floor here. `029de6ee5158`: the two existing bench drivers apply the dylib prefix and suffix twice. `c9d2fc212037`: `mock bench run` cannot reach a member bench behind a root driver, and the harness resolves arms to a per-variant target directory that a workspace build never uses. `fc42e522574d`: the chunked bars spawn their threads every frame, and the fused bar loses to the column arms under `heavy`.
