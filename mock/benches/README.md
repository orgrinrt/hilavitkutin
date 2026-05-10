# benches

Canonical mockspace bench framework. Consumer-side scaffolding generated
by `mock bench init`.

## Layout

- `Cargo.toml` + `src/main.rs`: the bench binary. Defines `Routine`
  impls, builds a workload program, dispatches to the harness.
- `bench.toml`: per-bench config (sizes, timing, variant cdylib paths).
- `variants/<name>/`: one workspace per variant. Each compiles to a
  cdylib that exports `bench_entry`, `bench_name`, `bench_abi_hash`.
- `target/release/benches`: the built bench binary.
- `target/release/lib<variant>.{dylib,so,dll}`: the built variant cdylibs.

## Workflow

1. Edit `src/main.rs`: replace `IdentityAdd` with the Routine you
   want to benchmark. The trait specifies what is computed (input
   shape, output shape, validation, scoring, ops count).
2. Edit `bench.toml`: set sizes and the cdylib path for each
   variant.
3. Add a variant under `variants/<name>/` for each implementation.
   Each variant exports `bench_entry` calling its own algorithm via
   the `timed!` macro.
4. `mock bench run` builds everything and runs the harness.
5. `mock bench report` regenerates `findings.md` from the CSV cache
   without re-running.

## Status

v2 of the bench framework. The harness ships full orchestration:
variant isolation via subprocess + dlopen, hardware counter timing
(`CNTVCT_EL0` / `rdtsc`), CSV cache with drift correction, validation
across variants (byte-exact / approximate / per-variant validity),
analysis (quintile + bootstrap CI + sign test + Pareto + multi-N
scaling), findings.md generator, history log with regression
detection, optional perf counter integration, asm dedup check.

## References

- `mockspace-bench-core` (the framework): see Routine trait docs.
- `mockspace-bench-harness` (the orchestrator): see `harness::run`
  and `harness::write_report`.
- Origin: framework was extracted from `polka-dots/mock/benches/`.
