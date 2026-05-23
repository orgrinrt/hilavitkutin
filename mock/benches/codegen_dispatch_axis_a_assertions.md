# Topic 4 Axis A — codegen dispatch 5-check disasm assertions

Companion to the `dispatch_static_n*` and `dispatch_dynamic_n*`
bench series. Those benches measure end-to-end + algorithm-only
timing for the dispatch shapes under Topic 4 Axis A. This file
records the five disasm-level perf invariants the dispatch codegen
must satisfy on the static-devirt path. The bench harness's variant
asm dedup check is the running safety net; this file is the
canonical statement of intent.

`StandardCodegen` (see `mock/crates/hilavitkutin-api/src/dispatch_codegen.rs` and `mock/crates/hilavitkutin/src/dispatch/standard.rs`)
emits per-core dispatch closures via TAIT; per Topic 4 Axis A
Rider 1 the lowering must preserve full LLVM transparency. Sketch
heritage `mock/research/sketches/202605101036-codegen-entrypoint-tait/`
and `mock/research/sketches/202605101036-codegen-tait-capture/` both
confirm WORKS. The five checks below codify what "transparent" means
at the disasm boundary for the `dispatch_static` bench shape.

## The five checks

Each check applies to the inner per-record body of the emitted
per-core closure (the `iter_morsel` body, with per-WU shim
invocations from `super::wu_fn::invoke_wu_in_fiber` and the
fiber-tail Release store via `super::progress::store_progress_arena`
preceded by `super::sync::emit_progress_release_fence`).

### Check 1: zero `blr` in inner body

No indirect call instruction (`blr` on aarch64, `call *reg` on
x86_64) anywhere in the per-record body. Indirect calls defeat LLVM
constant propagation past the call site and serve as the bright-line
signal that devirtualisation failed. The `dispatch_dynamic_n*` bench
exists as the counter-example: opaque-fn-pointer and runtime-table
variants exhibit `blr` and pay the documented penalty. The static
shape must show zero.

### Check 2: indexed addressing on column loads

Per-record loads from columns use indexed addressing (`ldr xN,
[xBase, xIdx, lsl #SCALE]` on aarch64; `mov rax, [rBase + rIdx*8]`
on x86_64). Address compute folded into the load instruction is the
signal that LLVM proved the address arithmetic through the morsel
loop's induction variable. Any pattern where the index is computed
into a separate register, then the load uses that register as a base
without scale, indicates LLVM lost track of the stride.

### Check 3: no `[sp, ...]` accesses in inner loop

Stack-relative loads or stores in the per-record inner loop signal
register spills. The body of `iter_morsel` should live entirely in
the register file across the per-record step; spills appear only at
the morsel boundary (where `sync_probe` runs) or at the closure
entry/exit. Any `[sp, ...]` inside the iteration body means the
register allocator gave up under pressure and the morsel size is too
large for the per-WU sequence, or the per-WU shims failed to inline.

### Check 4: immediate-constant morsel size

The morsel size (`Cfg::MICRO_MORSEL_INTERVAL`,
`Cfg::MAX_DRIFT_RECORDS`, the morsel range's `len`) must appear as
immediates in the emitted asm. Per the
`codegen-tait-capture` sketch's Result B: per-Cfg constants bake as
immediates because the const-generic propagation reaches the
monomorphised per-core closure body. If the morsel size appears as a
register-held variable, the runtime-config path was reached instead
of the typestate path, and LLVM lost the ability to unroll the inner
loop or fold the bound check.

### Check 5: no `bl` to morsel-size or fiber-dispatch helpers

No direct call instructions (`bl` on aarch64, `call rel32` on
x86_64) targeting any function whose name pattern matches `morsel`,
`fiber_dispatch`, `wu_fn`, or any helper that should have inlined.
The `#[inline(always)]` discipline on
`wu_fn::invoke_wu_in_fiber` and `progress::store_progress_arena`
plus the sketch-validated TAIT transparency together imply that
every per-record-body helper dissolves into the body. A `bl` to a
helper symbol means the helper failed to inline; the consumer-
visible cost is one mispredicted branch per record on the worst
shape.

## How the checks run

Three layers cooperate:

1. **`mock bench run` builds variant cdylibs at `-O3 lto=fat
   codegen-units=1` and records timing.** The harness's variant asm
   dedup check (see `mockspace-bench-harness/src/asm`) reads the
   emitted asm out of each variant's release artefact, normalises
   it, and flags variants whose normalised asm matches the
   `dispatch_dynamic` baseline rather than the `dispatch_static`
   baseline. That dedup check is the running safety net for the five
   checks above.

2. **The disasm artefacts under `target/release/lib<variant>.dylib`
   (or `.so`) carry the live asm at every run.** When a check
   regresses, `cargo mock bench` flags the variant; the engineer
   re-runs disasm against this file's five checks to pinpoint which
   property degraded.

3. **A future `tests/codegen_fence.rs` integration test (Pass 3
   CHANGE 6 follow-up) asserts the S3 fence shape per arch.** That
   landing pairs with Check 5: the fence is a `dmb ishst` (aarch64)
   or `_mm_sfence` (x86_64), not a `bl` to a fence helper. The
   integration test confirms the emit-site discipline; the disasm
   check confirms LLVM did not regress past the emit-site.

## Status

CHANGE 7 of Pass 3 lands this file as the 5-check assertion
document. The running asm dedup check ships with the bench
harness; the five checks above are this round's statement of what
the harness is checking against on the Axis A path. Future Pass 3
follow-ups + Pass 7 (test-utils + examples) wire the integration
test for the S3 fence emission shape.

## See also

- `mock/research/sketches/202605101036-codegen-entrypoint-tait/FINDINGS.md`
- `mock/research/sketches/202605101036-codegen-tait-capture/FINDINGS.md`
- `mock/research/sketches/202605101036-progress-counter-arena/FINDINGS.md`
- `dispatch_static_n*_findings.md` (per-N timing summaries)
- `dispatch_dynamic_n*_findings.md` (counter-example timing summaries)
- `mock/design_rounds/202605101036_changelist.src.md` Pass 3 CHANGE 7
