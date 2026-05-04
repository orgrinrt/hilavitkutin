**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-extensions DESIGN reframing + hilavitkutin-extensions-macros emission discriminator + hilavitkutin-extensions-macros test extension + hilavitkutin-linking RTLD_LOCAL flag + hilavitkutin-linking DESIGN paragraph
**Source topics:** PR #54 senior review F1, F2, F3 (round 202605041648 follow-up)

# Topic: LTO smoke review followup

The pr-reviewer-senior pass on PR #54 found three issues warranting in-PR fixes. This round addresses all three on the same feature branch (`chore/trampoline-lto-survival-d4`) per the workspace's "multiple sequential rounds per branch" pattern.

## F1 — DESIGN trampoline preservation prose is empirically wrong

The DESIGN paragraph from round 202605041648 claims `#[unsafe(no_mangle)]` makes trampolines "ineligible for ICF merging because the linker cannot prove that no external code observes their distinct addresses." The reviewer ran `nm -m` and `objdump -d` on the actual fixture cdylib and found both trampoline names point to address `0x418`. Mach-O `ld64` folds same-content code AND emits both names as aliases for one body. `#[unsafe(no_mangle)]` preserves the *symbol-table entries*; it does not prevent the *address* fold.

The practical effect remains correct because real consumer extensions ship `init` and `shutdown` impls that compile to non-byte-identical bodies, so ICF naturally won't fold them. For the trivial-impl test fixture (both impls return `Ok`), the fold is observationally indistinguishable: both fn-pointer slots produce the same correct result.

This round reframes DESIGN to match what the framework actually guarantees: symbol-name preservation (against dead-code stripping), with address aliasing under ICF tolerated for byte-identical bodies because real consumer impls naturally avoid byte identity. Combined with F2's test extension below, the framework also defensively forces distinct addresses via a per-trampoline discriminator.

## F2 — Extend the smoke test to verify distinct fn-pointer addresses

Symbol survival in `nm` is necessary but not sufficient. The contract that matters at runtime is "descriptor's `init_fn` and `shutdown_fn` slots reference distinct callable bodies when the consumer's impls are not byte-identical." The current test does not catch the address-aliasing case.

Extending the test in this round: dlopen the fixture cdylib via `libloading` (dev-dep), resolve `__hilavitkutin_extension_descriptor`, read its `init_fn` and `shutdown_fn` slots, assert they are distinct fn-pointer addresses. To make this assertion hold with the trivial-impl fixture, the macro emission gains a per-trampoline byte-string discriminator passed through `core::hint::black_box`, which forces ICF to see different bodies.

The discriminator overhead is one constant load + one black_box (a no-op assembly fence) per trampoline call. Trampolines fire once per extension load and once per extension drop, so the overhead is negligible.

## F3 — `hilavitkutin-linking` does not currently set `RTLD_LOCAL`

The reviewer checked `mock/crates/hilavitkutin-linking/src/backend/unix.rs:24` and found the `dlopen` call uses `RTLD_NOW` only, with no `RTLD_LOCAL` bit. POSIX-defined behaviour:

- glibc (Linux): default-when-neither-flag-given is `RTLD_LOCAL` semantics, so the round 202605041648 DESIGN claim is accidentally correct.
- macOS, *BSD: default is `RTLD_GLOBAL` semantics, so the DESIGN claim is wrong on the platforms where the test runs.

The DESIGN paragraph from round 202605041648 explicitly relies on `RTLD_LOCAL` semantics for the namespace-isolation argument that bounds the `#[unsafe(no_mangle)]` collision concern. Making the DESIGN claim true everywhere requires explicitly OR'ing `RTLD_LOCAL` (value `4`) into the dlopen call.

This round adds `const RTLD_LOCAL: c_int = 4;` to the unix backend and changes the dlopen call to `RTLD_NOW | RTLD_LOCAL`. The hilavitkutin-linking DESIGN.md.tmpl gains a sentence noting the explicit flag choice.

## What stays untouched

- The `#[used]` attribute on `__HILAVITKUTIN_EXT_DESCRIPTOR`.
- The `#[unsafe(no_mangle)] pub extern "C"` on `__hilavitkutin_extension_descriptor`.
- The trampoline `unsafe extern "C" fn` shape and `#[unsafe(no_mangle)] #[inline(never)]` attribute pair from round 202605041648.
- The fixture's `Cargo.toml`, `build.rs`, `src/lib.rs`. The fixture-side intent (trivial impls that would naturally fold) is exactly what the test extension exercises.
- `hilavitkutin-linking::Library::resolve` / `resolve_static` / `close` semantics. Only the `dlopen` flag changes.

## Decision

Adopt as proposed. Five edits land in this round.
