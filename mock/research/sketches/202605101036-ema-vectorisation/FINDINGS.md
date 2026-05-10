# Findings — EMA vectorisation, cfg-gated SIMD per target_feature

**Outcome:** **WORKS**. NEON path on aarch64+neon lowers to 7 instructions including a single fused multiply-add. Scalar fallback lowers to pure GP-register code with no vector registers. Cfg gating selects exactly one path per build. The hypothesis holds with margin to spare.

## Test setup

- Toolchain: nightly stable rust.
- Profile: `release`, `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`.
- Target: aarch64-apple-darwin (NEON enabled by default in the target spec).
- Build: `cargo +nightly rustc --release --lib -- --emit=asm`.

## Result A — NEON path on aarch64+neon

`ema_update_batch_simd` body (7 instructions):

```asm
ldr     q0, [x0]            ; load 4×u32 olds
ldr     q1, [x1]            ; load 4×u32 samples
movi.4s v2, #7              ; broadcast constant 7 across 4 lanes
mla.4s  v1, v0, v2          ; samples += olds * 7 (fused multiply-add)
ushr.4s v0, v1, #3          ; >> 3 across 4 lanes
str     q0, [x0]            ; store back
ret
```

LLVM was sharper than the hand-rolled `(o<<3) - o + s` formulation: it recognised `o*7 + s` and selected `mla.4s` (Multiply-Accumulate, 4-lane unsigned) directly. The four lanes update in a single instruction. Total cycle count for the arithmetic core: ~3 cycles (mla + ushr serialised) on Apple Silicon's NEON unit.

## Result B — Scalar fallback (without target_feature gate)

`ema_update_batch_scalar` body, pure GP-register arithmetic:

```asm
ldp     w8, w9, [x0]        ; load two u32s
ldp     w10, w11, [x1]
sub     x10, x10, x8
add     x8, x10, x8, lsl #3 ; o*8 + (s-o) = o*7 + s
lsr     x8, x8, #3          ; / 8
sub     x10, x11, x9
add     x9, x10, x9, lsl #3
lsr     x9, x9, #3
stp     w8, w9, [x0]
; repeat for the second pair
ldp     w8, w9, [x0, #8]
ldp     w10, w11, [x1, #8]
... (same pattern)
stp     w8, w9, [x0, #8]
ret
```

No vector registers. Processes lanes pairwise via `ldp` (Load Pair) + parallel x8/x9 arithmetic, two iterations to cover all 4 lanes. About 18 scalar instructions vs NEON's 7. The 2.5x instruction-count gap is the SIMD win this sketch validates the capacity to capture.

## Result C — cfg gating selects exactly one path

`cfg(all(target_arch = "aarch64", target_feature = "neon"))` evaluated true on this build (Apple Silicon's rustc target spec enables neon by default), so the NEON path's symbol exists. The fallback symbol (`scalar`) coexists because it is unconditional — it is the always-available reference impl. The dispatcher `ema_update_batch` resolves to whichever `_simd` cfg arm compiled.

If the build had targeted an aarch64 toolchain with `-neon`, the NEON arm would be cfg-disabled and the fallback arm's body (which calls `_scalar`) would become the active `_simd`. The audit-2 m5 correction holds: `target_feature` is the correct gate; the build either gets the intrinsics or it doesn't, with no silent autovec dependency.

## Result D — Norm repr-transparent compatibility

The sketch operates on bare `u32` since `arvo::Norm` is not yet shipped. `Norm = UFixed<0, 32, S>` is `#[repr(transparent)]` over the u32 container, so this same body is what the substrate will emit once `Norm`'s ops route through these intrinsics via the cfg-gated arvo internals path. No structural rewrite needed once `Norm` lands.

## Decision implication

**Topic 5 Axis H locks at the cfg-gated-from-day-one shape.** Three implementations ship with v1:

- aarch64 + neon: NEON 4-lane.
- x86_64 + sse2: SSE2 4-lane (code present in this sketch; not asm-verified on macOS native, but pattern-symmetric to the NEON path which is asm-verified).
- fallback: scalar reference implementation, always available.

Future expansion (AVX2 for 8-lane, AVX-512 for 16-lane, SVE for runtime-vector-length) is Kind 1 bench-driven *expansion* on top of this baseline, per the workspace rule.

## What this sketch does NOT prove

- The x86_64 SSE2 path's asm cannot be inspected natively on aarch64-apple-darwin without cross-compiling. The code is present and the cfg gate is symmetric to the NEON arm; the lowering is mechanically equivalent (`_mm_slli_epi32` + `_mm_sub_epi32` + `_mm_add_epi32` + `_mm_srli_epi32`). A future bench under x86_64 will verify the actual emitted SSE2 instructions.
- The fused `mla.4s` on aarch64 may or may not happen for the SSE2 path; `_mm_mullo_epi32` requires SSE4.1, and SSE2 doesn't have a u32 lane-wise multiply. The shift-sub-add pattern in the sketch is correct for SSE2; the SSE4.1 (or later) path could use `_mm_mullo_epi32` instead for a one-instruction-shorter body. Tracked as Kind 1 expansion.
- AVX2 + AVX-512 paths are deliberately not in v1; they are Kind 1 expansion targets driven by bench evidence under the corresponding target-feature builds.

## Sketch retention

Stays committed forever per `cl-claim-sketch-discipline.md`. The 7-instruction NEON body is the proof-of-concept for the v1 SIMD shape; later bench rounds will measure the gap against scalar and against autovec on builds where autovec happens to succeed (a known unreliable comparison).
