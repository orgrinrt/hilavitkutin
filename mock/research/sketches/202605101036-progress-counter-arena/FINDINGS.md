# Findings — progress-counter-arena lowering

**Outcome:** **WORKS**. E2 (plan-stage scratch arena) lowers to single `stlr`/`ldar` per progress operation. S3 (Topic 3 single-stlr invariant) is satisfied. Axis E2 locks.

## Result A — arena (E2 shape)

```asm
__RNv...store_progress_arena:
    add x8, x0, x1, lsl #3   ; arena_base + fiber_id*8
    stlr x2, [x8]             ; single Release store
    ret
```

Three instructions including return. The `add` is a single-cycle indexed-address compute, free in any aarch64 pipeline (no register pressure beyond the destination). `stlr` is the canonical Release-store, exactly as Domain 17 L1622-1623 prescribes.

## Result B — direct (E1 shape, comparison)

```asm
__RNv...store_progress_direct:
    stlr x1, [x0]             ; single Release store
    ret
```

Two instructions; no offset compute since the arg is already a pointer-to-counter.

## Result C — acquire-load arena

```asm
__RNv...load_progress_arena:
    add x8, x0, x1, lsl #3
    ldar x0, [x8]             ; single Acquire load
    ret
```

Symmetric — `ldar` matches Domain 17 L1624.

## Decision

E2 is sound. The arena indirection is a single shift-add instruction; no `ldr` between the offset compute and the `stlr`/`ldar`; no stack spills; no register clobbering. The S3 invariant ("dmb st / sfence between final stnp and progress-counter Release store") is unaffected — that fence applies to `stnp` (non-temporal store) ordering, separate from the progress-counter store.

The architectural advantages (lifetime cleanly tied to `Scheduler::run()` frame; clean thread-pool handoff; no per-core-fn stack-pressure spike from `[AtomicUsize; MAX_FIBERS]`) are uncontested by the lowering.

## Sketch retention

Keep this sketch as a regression artefact for the S3 ASM verification check. If a future codegen change (e.g., adding extra prologue logic) breaks the single-stlr property, this sketch's disasm comparison surfaces it immediately.
