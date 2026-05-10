# Findings — codegen entrypoint via sealed trait with TAIT

**Outcome:** **WORKS**. Validated dramatically.

## Test setup

- Toolchain: `nightly` with `feature(impl_trait_in_assoc_type)`.
- Profile: `release`, `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`.
- Target: aarch64 (Apple Silicon, native).
- Build: `cargo +nightly rustc --release --lib -- --emit=asm`.
- Inspection: `target/release/deps/sketch_codegen_tait-*.s`.

## Result A — trait + TAIT call site (the hypothesis)

`call_through_trait` body:

```asm
__RNvCs..._19sketch_codegen_tait18call_through_trait:
    cbz x0, LBB0_2          ; branch on zero record_count
    sub x8, x0, #1
    sub x9, x0, #2
    umulh x10, x8, x9
    mul x8, x8, x9
    sub x9, x0, #3
    mul x9, x8, x9
    lsr x9, x9, #1
    mov x11, #6148914691236517205
    movk x11, #21850
    extr x8, x10, x8, #1
    mov w10, #35
    mul x8, x8, x10
    madd x8, x9, x11, x8
    mov w9, #21
    madd x8, x0, x9, x8
    sub x0, x8, #21
LBB0_2:
    ret
```

Properties verified:

- Zero `bl` instructions to user fns.
- Zero `blr` instructions.
- Zero stack frame setup (no `sub sp, sp, #N`).
- Zero loop — LLVM recognised the loop-carried recurrence and reduced
  `sum(wu_a(i)*wu_b(i) + wu_c(i) for i in 0..record_count)` to a
  closed-form polynomial in `record_count`.
- Outcome: stronger than "devirtualisation works" — the entire
  abstraction is provably zero-cost at LLVM level. LLVM proved
  through:
  1. The sealed trait dispatch.
  2. The TAIT associated type resolution.
  3. The returned closure body.
  4. The three `#[inline(always)]` WU bodies.
  5. The loop accumulation.
  6. Into a single closed-form expression.

## Result B — struct-field FAIL pattern (Domain 17 L1540 confirmation)

`call_through_struct_field` body (excerpted inner loop):

```asm
__RNvCs..._19sketch_codegen_tait25call_through_struct_field:
    sub sp, sp, #64                 ; stack frame
    ...
    cbz x0, LBB1_3
    mov x20, x0
    mov x19, #0
    mov x21, #0
    ldr x22, [x1]                   ; load fn pointer from struct field
LBB1_2:
    stp x21, xzr, [sp]              ; spill ctx to stack
    mov x0, sp                      ; pass &ctx via stack
    blr x22                         ; INDIRECT CALL — the 12.6x pattern
    add x19, x0, x19
    add x21, x21, #1
    cmp x20, x21
    b.ne LBB1_2
    b LBB1_4
```

Properties confirmed (matching Domain 17 prediction):

- `blr x22` per iteration — indirect branch through the register
  loaded from the struct field.
- Stack-spill of ctx every iteration (`stp x21, xzr, [sp]`).
- Real loop, no constant fold; LLVM cannot prove through the struct
  field reference.
- Stack frame setup (`sub sp, sp, #64`) and full prologue/epilogue.
- This is the exact failure mode polka-dots T6 L1024-1082 measured
  at 12.6x penalty.

## Bound generalisation note

The closed-form fold in Result A is partly an artifact of the sketch's
arithmetic-only WU bodies — LLVM happened to recognise a polynomial
recurrence. With more realistic WU bodies (column reads, fixed-point
math, conditional branches), the loop would not constant-fold. But
the load-bearing property — the trait + TAIT lower transparently to
the optimiser — is independently confirmed by the absence of any
`blr`/`bl` in the trait-path body. Once LLVM sees through the
abstraction, ordinary loop optimisation applies.

A follow-up sketch can validate the same property with WU bodies that
escape constant folding (e.g., reading from a `&[u64]` parameter).
That validation is **not** required to lock Axis A; the trait + TAIT
abstraction is proven transparent here.

## Decision implication

**Axis A locks at A3 with both riders:**

- **A3:** `trait DispatchCodegen<Cfg: RunCfg>: Sealed` with
  `type CoreDispatch: Fn(...)` and method `fn build(...) -> Self::CoreDispatch`.
- **Rider 1:** `type CoreDispatch = impl Fn(...)` (TAIT via
  `feature(impl_trait_in_assoc_type)`). Confirmed transparent to LLVM.
- **Rider 2:** Sealed via private `Sealed` supertrait per Topic 3 S2.
  Ships `StandardCodegen` in v1; future passes via additional sealed
  impls (`BenchInstrumentedCodegen`, `DebugCodegen`).

The struct-field FAIL pattern is captured for posterity in
`call_through_struct_field` — keep this sketch as a regression artefact
so future agents can re-verify Domain 17 L1540 if doubts arise.

## Sketch retention

This sketch stays committed forever per `cl-claim-sketch-discipline.md`.
The struct-field counter-example is part of the audit trail showing
why TAIT is load-bearing rather than ergonomic noise.
