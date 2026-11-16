# benches

Canonical mockspace bench framework. `mock bench run` generates the
driver from `bench.toml`, builds the arms and runs the harness; the tree
holds configuration, the measured code and the hooks library, and no
runner of its own.

## Layout

- `bench.toml`: the byte dispatch's points under `[dispatch]`, every
  workload program under `[workload.*]`, and the default timing.
- `<bench>/bench.toml`: one bench, one question. Its arms are the
  directories under `<bench>/arms/`, each one cdylib exporting
  `bench_entry`, `bench_name`, `bench_abi_hash`. A bench with several
  sweeps (`engine_vs_std`, `single-trunk-convergence`, `resource_storage`,
  `frame_wide`) declares them as `[sweep.<name>]`; its arms build in the
  workspace its own `Cargo.toml` declares over `arms/*`.
- `support/`: the library crates the arms and the generated driver
  share: the routine types, the kernels, the five-check disassembly
  checker and the design's ceilings, each with its own tests.
- `src/lib.rs`: the hooks library the generated driver compiles in. It
  serves the routines that are not byte routines and runs what follows a
  cell: the five checks over `dispatch_*`, the ceilings over
  `engine_vs_std`, and `frame_wide`'s report framed on `hand`.
- `asm_gate_fixtures/`: the D6 ASM-dispatch gate, a cdylib of fixtures
  and the test in `tests/asm_gate.rs` that holds their dispatch bodies to
  the five checks.
- `results/<bench>/<sweep>_n<point>/<machine class>/<run>/`: one run of
  one cell, holding `samples.csv`, `meta.json`, `report.md` and
  `summary.tsv`. A run is written once; runs on other machines sit
  beside it.

## Workflow

1. `mock bench add <bench> <arm>` scaffolds a bench or an arm; the
   bench's `bench.toml` names its arms, points and workload.
2. Each arm under `<bench>/arms/<arm>/` exports `bench_entry` calling its
   own algorithm through the `timed!` macro. Code two arms share is a
   crate under `support/`.
3. A routine that is not plain bytes goes in a support crate, and
   `routine_for` in `src/lib.rs` reaches it.
4. `mock bench run` generates the driver, builds everything and runs
   the harness; `mock bench test` runs the tests of every crate in the
   tree.
5. `mock bench report` renders `report.md` for any run that has none.

## Status

v2 of the bench framework. The harness ships full orchestration:
variant isolation via subprocess + dlopen, hardware counter timing
(`CNTVCT_EL0` / `rdtsc`), CSV cache with drift correction, validation
across variants (byte-exact / approximate / per-variant validity),
analysis (quintile + bootstrap CI + sign test + Pareto + multi-N
scaling), report generator, per-run summaries with regression detection
within a machine class, optional perf counter integration, asm dedup
check.

## References

- `mockspace-bench-core` (the framework): see Routine trait docs.
- `mockspace-bench-harness` (the orchestrator): see `harness::run`
  and `harness::write_report`.
- Origin: framework was extracted from `polka-dots/mock/benches/`.
