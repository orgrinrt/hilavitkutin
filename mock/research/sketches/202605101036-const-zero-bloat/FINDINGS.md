# Findings — `pub const` zero binary bloat

**Outcome:** **WORKS**. `pub const` declarations emit zero binary symbols and consume zero bytes. Reading a const inlines the literal at the use site. Confirmed via `nm` symbol table inspection plus asm body comparison.

## Test setup

- Toolchain: `nightly` with `release` profile, `lto = "fat"`, `codegen-units = 1`, `strip = true`.
- Target: aarch64 (Apple Silicon, native).
- Crate has both a `pub const` and a `pub static` with literal initialisers, plus two `extern "C"` reader fns to force a use site for each.

## Result A — `nm` symbol table

```
S _SKETCH_STATIC_DEFAULT_DECAY     <- static IS present (S = data section)
T _read_const                       <- reader fn for const
T _read_static                      <- reader fn for static
```

`_SKETCH_CONST_DEFAULT_DECAY` is absent from the symbol table entirely. Only the static contributes a symbol entry; the const is purely a compile-time entity.

## Result B — function body asm comparison

```asm
_read_const:
    mov  x0, #48879           ; immediate low-half of 0xDEAD_BEEF_DEAD_BEEF
    movk x0, #57005, lsl #16  ; immediate high-half of low word
    orr  x0, x0, x0, lsl #32  ; replicate to high word
    ret

_read_static:
    mov  x0, #47806           ; immediate low-half of 0xCAFE_BABE_CAFE_BABE
    movk x0, #51966, lsl #16  ; immediate high-half of low word
    orr  x0, x0, x0, lsl #32  ; replicate to high word
    ret
```

Both functions materialise the literal as a sequence of immediate `mov`/`movk`/`orr` instructions. Neither does a memory load. The difference is binary footprint:

- `read_const`: zero bytes in `.data`/`.rodata` for the const value (the symbol is not emitted).
- `read_static`: 8 bytes in `.data` for the static's storage (the symbol IS emitted, ready for address-taking by consumers).

LLVM's constant-folding pass recognises the static's value is a known literal with no interior mutability, so the *load* is replaced by an immediate. But the static's storage still exists in the binary because anyone could `&SKETCH_STATIC_DEFAULT_DECAY` to take its address.

## Implication for AdaptConfig (Topic 5 Axis C)

Each `pub const DEFAULT_*` in a sub-config struct contributes zero bytes to the binary. Across the nine sub-configs, even if every one shipped 4 named defaults, that is 36 const declarations costing exactly 0 bytes. Consumer ergonomics (named, discoverable, scalable-relative-to-default) win at no perf cost.

The bloat concern would apply if the design used `pub static`. It does not.

## Decision

**Axis C locks at C1.** Each sub-config that EMA-tracks exposes `pub const DEFAULT_DECAY_WEIGHT: arvo::Norm = ...;` and other named defaults, with `Default` impl referencing them. Zero bloat per Rust const semantics; consumer ergonomics preserved.

## Sketch retention

Keep this sketch as the canonical reference for "consts are free" claims in future workspace decisions. The `nm` + asm comparison pattern generalises to any future "is this declaration emitting bytes" question.
