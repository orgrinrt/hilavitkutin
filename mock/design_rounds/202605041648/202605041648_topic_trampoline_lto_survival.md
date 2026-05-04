**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-extensions-macros (proc-macro emission + new integration test + standalone cdylib fixture), hilavitkutin-extensions (DESIGN paragraph)
**Source topics:** task #349 (PLUGIN-HOST-D4, plugin-host audit F7)

# Topic: trampoline survival under release + LTO

## Background

`#[export_extension]` emits two trampolines (`__ext_init_trampoline_<Struct>`, `__ext_shutdown_trampoline_<Struct>`) that wrap consumer-supplied `init` / `shutdown` types. Their addresses are stored in the `__HILAVITKUTIN_EXT_DESCRIPTOR` static, which is `#[used]`. The descriptor function `__hilavitkutin_extension_descriptor` (`#[unsafe(no_mangle)] pub extern "C"`) returns a pointer to that static.

The plugin-host audit (2026-05-04, F7) flagged a concern: with no `#[used]` on the trampolines themselves and no `#[inline(never)]`, aggressive LTO might dead-strip them despite the descriptor's address-take. The risk: a quiet "init never runs" failure in release builds. Hard to debug.

## What the chain actually preserves

The current emission already gives LLVM enough information to preserve the trampolines, in theory:

1. `__hilavitkutin_extension_descriptor` is exported (`#[unsafe(no_mangle)] pub extern "C"`). External linkage forces the linker to keep it.
2. The function returns `&__HILAVITKUTIN_EXT_DESCRIPTOR`. The static is reachable from the export.
3. `__HILAVITKUTIN_EXT_DESCRIPTOR` is `#[used]`. LLVM is forced to emit the static (even without `#[used]`, the export-reachability would keep it; the attribute is belt-and-suspenders).
4. The static contains `init_fn: Some(__ext_init_trampoline_<Struct>)` and `shutdown_fn: Some(__ext_shutdown_trampoline_<Struct>)`. These are constant fn-pointer values.
5. LLVM cannot elide a function whose address has been taken in a preserved static. The address must be a real callable location in the final binary, so the function bodies are preserved.

That is the theoretical argument. The audit asks for verification under real LTO.

## The verification gap

Theoretical preservation is not the same as observed preservation. LTO + cdylib stripping involves rustc, LLVM, and the platform linker. Edge cases exist:

- Some cargo configurations (`strip = "symbols"` in release profile, certain target-specific `RUSTFLAGS`) can strip non-exported function symbols from the final binary's symbol table, even when the function bytes are kept.
- Symbol-table stripping does not affect runtime callability (the bytes are still at the right address; the descriptor's fn-pointer points to them; calls work). But it makes the trampolines invisible to `nm`, which is the standard observability tool.
- Aggressive cross-language LTO with `-Cembed-bitcode=yes` can trigger inter-procedural optimisations that have surprised real users in the past.

So verification needs both: (a) symbol survival when the binary is not symbol-stripped, and (b) ideally, runtime callability through a dlopen + descriptor-fn-pointer call.

## Proposed shape

Two changes in this round.

**1. Macro emission: add `#[inline(never)]` to both trampolines.**

`#[inline(never)]` is not a preservation directive — it's an inlining-policy directive. But its presence signals intent ("this function is meant to be a real, callable function, not inlined into call sites") and explicitly defends against an exotic LTO path that would inline the trampoline's body into the descriptor static's initializer expression and then elide the function. That path is not known to occur in practice, but the attribute closes the door cheaply.

The macro does not add `#[unsafe(no_mangle)]` to the trampolines. Reason: `#[no_mangle]` would expose the trampoline as an externally-linked symbol with the literal Rust ident name (`__ext_init_trampoline_<Struct>`). Two extensions defining a struct with the same name would then collide if the host loaded both into the same process namespace (RTLD_GLOBAL, or through some symbol-leakage path). The cdylib's RTLD_LOCAL default avoids this in practice, but the implicit collision risk is unnecessary scope creep for a polish round. The mangled-name + `#[used]`-static-pointer chain is sufficient.

**2. Integration test fixture.**

Create `tests/lto_smoke_fixture/`, a standalone (non-workspace-member) cdylib crate that uses `#[export_extension(init = Init, shutdown = Shutdown)]`. Its release profile sets `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`, `strip = false` so symbols are observable.

Create `tests/lto_smoke.rs`, a std integration test that:

1. Shells out to `cargo build --release --manifest-path tests/lto_smoke_fixture/Cargo.toml`.
2. Runs `nm -gU <output>` on the resulting cdylib.
3. Asserts that:
   - `__hilavitkutin_extension_descriptor` is exported.
   - The trampolines survive: any symbol containing `ext_init_trampoline` or `ext_shutdown_trampoline` substring exists. The Rust mangler keeps the original ident as a substring within the mangled form (`_ZN<...>32__ext_init_trampoline_<Struct>17h<hash>E` shape).
4. Skips on non-Unix platforms (Windows uses different tooling; the `nm`-based check is dev-targeted, runtime-correctness is platform-independent through the descriptor static mechanism).

If the trampoline substring is NOT in the symbol table after the test runs, the round adds `#[unsafe(no_mangle)]` to the trampoline emission as a follow-up edit and re-verifies.

## What stays untouched

- The descriptor static's `#[used]` attribute. Already correct.
- The descriptor function's `#[unsafe(no_mangle)] pub extern "C"`. Already correct.
- The trampoline body shapes. Only the attribute set on them changes.
- All other macro emission. The capability table, name, version, required-caps lookups are unchanged.
- The trybuild test harness. Continues to operate on the same fixture set.

## What this round explicitly defers

- A runtime test that dlopens the fixture and calls the trampoline through the descriptor's fn-pointer. The simpler `nm` substring check is sufficient for symbol survival; runtime callability becomes free once a real consumer ships an extension and exercises the path. A BACKLOG entry tracks this.
- Windows-side verification. The fixture builds on macOS and Linux; Windows requires `dumpbin` instead of `nm`, which is a separate harness. BACKLOG.

## Decision

Adopt as proposed: add `#[inline(never)]`, ship the integration test fixture, document the preservation chain, defer runtime + Windows verification to BACKLOG.
