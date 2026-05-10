# Findings — FiberShape: Sealed as generic parameter

**Outcome:** **WORKS**. Monomorphisation per shape produces distinct, independently-optimised bodies with all shape-specific constants baked in as immediates. No runtime shape branch in the dispatch path.

## Test setup

- Toolchain: stable nightly Rust (no nightly features required — sealed trait + associated `const` is stable).
- Profile: `release`, `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`.
- Target: aarch64-apple-darwin (Apple Silicon, native).
- Build: `cargo +nightly rustc --release --lib -- --emit=asm`.
- Inspection: `target/release/deps/sketch_fibershape_typing-*.s`.

## Result A — four distinct monomorphised symbols

```
__RINvCs..._24sketch_fibershape_typing18dispatch_per_shapeNtB2_10SequentialEB2_:
__RINvCs..._24sketch_fibershape_typing18dispatch_per_shapeNtB2_7StridedEB2_:
__RINvCs..._24sketch_fibershape_typing18dispatch_per_shapeNtB2_9PointwiseEB2_:
__RINvCs..._24sketch_fibershape_typing18dispatch_per_shapeNtB2_9ScatteredEB2_:
```

Each `dispatch_per_shape::<S>` instantiation emits its own symbol. The `.globl` directive on each confirms they are independent code-emit units, not folded together.

## Result B — shape-specific constants baked as immediates

`Sequential` body (`STRIDE=1, PREFETCH_AHEAD=8, MORSEL=256`):

```asm
add  x9, x8, #64          ; lookahead*stride*8 = 8*1*8 = 64 bytes
add  x10, x15, #256       ; MORSEL_RECORDS = 256
ldur x16, [x14, #-64]     ; prefetch offset baked
add  x15, x15, #1         ; STRIDE = 1
```

`Strided` body (`STRIDE=8, PREFETCH_AHEAD=16, MORSEL=128`):

```asm
add  x10, x9, #128        ; MORSEL_RECORDS = 128
ldr  x9, [x12, #1024]     ; lookahead*stride*8 = 16*8*8 = 1024 bytes
ldr  x9, [x12], #64       ; post-increment stride*8 = 64 bytes
add  x13, x13, #8         ; STRIDE = 8
```

`Pointwise` body (`STRIDE=1, PREFETCH_AHEAD=0, MORSEL=512`):

```asm
add  x10, x14, #512       ; MORSEL_RECORDS = 512
ldr  x15, [x13], #8       ; single load
add  x8, x8, x15, lsl #1  ; FUSED: prefetch_at==j+0 collapsed
```

`Pointwise` is stronger evidence than per-constant baking: LLVM propagated `PREFETCH_AHEAD = 0` through the prefetch arithmetic, proved `prefetch_at == j`, eliminated the redundant load, and fused the two `acc += column[j]` statements into `acc += 2 * column[j]` via `lsl #1`. Constant propagation reached deep enough to alter control-flow shape per shape.

## Result C — `MORSEL_RECORDS` distinct across shapes

Across all four bodies the `add ..., #N` for the morsel-window step uses `#256`, `#128`, `#64`, `#512` respectively. Zero shape-table loads. Zero shape-discriminating compares. Each body owns its constant set.

## Result D — runtime-enum counter-example confirms the cost

`dispatch_runtime_match`'s preamble:

```asm
adrp  x9, l_switch.table._RNvCs..._22dispatch_runtime_match@PAGE
add   x9, x9, l_switch.table._RNvCs..._22dispatch_runtime_match@PAGEOFF
ldr   x9, [x9, x11]        ; load stride from table[kind]
... ; same pattern for morsel and lookahead
ldr   x10, [x10, x11]
ldr   x11, [x12, x11]
```

Three constant-table indexed loads at function entry for `(stride, morsel, lookahead)`. The values are runtime variables in registers x9/x10/x11 for the rest of the loop. No constant propagation possible past this point: LLVM cannot fold `morsel + 1` when `morsel` is a register-held variable instead of an immediate.

The full D1 penalty: every dispatch pays three indexed memory loads + register-held shape constants throughout the hot loop, instead of inlined immediates.

## Decision implication

**Topic 4 Axis D locks at D2** (per-fiber-shape generic monomorphisation).

**Topic 3 FiberShape: Sealed amendment is load-bearing and confirmed sound.** The sealed-trait + associated-const shape produces the desired monomorphisation under release+LTO. No `feature(impl_trait_in_assoc_type)` or `feature(generic_const_exprs)` is required for this baseline — the sealed family + associated consts pattern works on stable. Future axis-specific TAIT-based extensions (Topic 4 Axis A) can stack on top without conflict.

## What this sketch does NOT prove

- It does not validate the *interaction* between FiberShape monomorphisation and the `DispatchCodegen<Cfg>` TAIT-traited entrypoint (Axis A). The codegen-entrypoint-tait sketch validates that path independently; the two abstractions compose by ordinary trait composition and should not interfere, but a follow-up sketch could combine them.
- It does not validate cache-line layout of per-fiber state. That is the `adapt-sidecar-layout` sketch's responsibility.
- It does not validate cross-shape morsel-loop SIMD vectorisation interaction. That is the `ema-vectorisation` sketch's responsibility for the relevant Axis H paths.

## Sketch retention

This sketch stays committed forever per `cl-claim-sketch-discipline.md`. The runtime-enum counter-example is part of the audit trail showing why D2 was selected over D1.
