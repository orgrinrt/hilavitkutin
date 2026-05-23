# WorkUnit / cdylib boundary

**Date:** 2026-05-23
**Scope:** hilavitkutin task #609; viola task #254; mockspace task #610
**Status:** recommendation memo, pre-src-CL
**Source topics:** `viola/docs/HILAVITKUTIN-APP-SHAPE.md`, `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-api/src/work_unit.rs`, `mock/crates/hilavitkutin-api/src/builder_input.rs`, `mock/research/EXTENSIONS_RESEARCH.md`

## Hypothesis

The conflict between "engine WorkUnit types must be known at compile time" and "lints load at runtime from cdylibs" is a false dilemma. The two concerns operate on opposite sides of a dispatch boundary and do not need to meet.

`RunLint<L>` is a host-side type compiled into viola with fully specified `Read` and `Write` AccessSets. The cdylib lint never implements `WorkUnit`. It implements a viola-domain vtable contract. `RunLint<L>::execute()` retrieves that vtable pointer at runtime from `Resource<ExtensionHost>`, marshals a `#[repr(C)]` call context from the scheduler column/resource accessors, and dispatches through a raw function pointer. AccessSet never crosses the FFI boundary. The engine typestate proof is complete before any cdylib loads.

## Design space

Three shapes were considered.

**Shape A (static MAX_LINTS, one RunLint per slot).** The scheduler builder registers `RunLint<0>` through `RunLint<MAX_LINTS - 1>`. Each instance has identical `Read`/`Write`. At runtime, `RunLint<L>::execute()` looks up slot `L` in `Resource<ExtensionHost>` and dispatches if occupied, or no-ops if empty. This is the recommended shape, refined as Shape D below.

**Shape B (single RunLint iterating all lints).** One WorkUnit type whose `execute` body iterates all loaded extensions sequentially. This loses the parallel fan-out that `viola/docs/HILAVITKUTIN-APP-SHAPE.md` line 53 specifies: "RunLint<*> parallel fan-out (all read Column<Nam>, all write Column<Diagnostic>; commutative on Column<Diagnostic>)." The scheduler can only parallelise distinct WorkUnit types. Shape B is wrong.

**Shape C (per-plugin types emerging at scheduler-build time).** Requires runtime WorkUnit registration, which `hilavitkutin/.claude/CLAUDE.md` bans: "Static composition only inside the engine. All hilavitkutin WorkUnits are registered at compile time via the scheduler builder." Shape C is ruled out.

**Shape D (refinement of A, recommended).** Same as A, with one clarification: `MAX_LINTS` is a compile-time constant on the host binary (viola-cli), not a constraint imposed by the engine. Picking it is the same category of decision as picking morsel count or fiber count. Unused slots silently no-op; the engine never treats an empty runtime slot as an error.

## Recommended shape

### Host-side WorkUnit

`RunLint<const L: usize>` in `viola-core` implements `WorkUnit` (from `mock/crates/hilavitkutin-api/src/work_unit.rs` line 40). Its AccessSets and commutative flag:

```rust
type Read = Cons<Resource<ExtensionHost>, Cons<Column<Nam>, Empty>>;
type Write = Cons<Column<Diagnostic>, Empty>;
const COMMUTATIVE: Bool = Bool::TRUE;
```

`COMMUTATIVE = Bool::TRUE` matches `HILAVITKUTIN-APP-SHAPE.md` line 53 and enables the scheduler to emit the reduce-style parallel fan-out across all `RunLint<L>` instances.

The `execute` body (signature at `work_unit.rs` line 73, `fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>)`):

1. Read `Resource<ExtensionHost>` from `ctx`. The viola `ExtensionHost` is a viola-owned POD container holding an array of loaded `Extension` handles, described at `HILAVITKUTIN-APP-SHAPE.md` line 88 as "ExtensionHost POD container." It is distinct from `hilavitkutin_extensions::ExtensionHost`; viola-core defines its own resource type and `LoadPlugins` populates it.
2. Access slot `L`. If `Maybe::Isnt`, return.
3. Call `Extension::provider(PROVIDER_VIOLA_LINTER)` on the handle. This is the `Extension::provider(id) -> Maybe<*const c_void>` accessor from `hilavitkutin-extensions` (`DESIGN.md.tmpl` line 66). If absent, return.
4. Cast the `*const c_void` to `*const ViolaLinterVtable`.
5. Marshal a `LintCallCtx` by reading from the Context accessors: the `Column<Nam>` morsel slice for the current record range, and a `Column<Diagnostic>` writer handle. All fields are `#[repr(C)]` and derived from what the WorkUnit's declared AccessSet already proves it may access.
6. Call `vtable.lint_fn(call_ctx_ptr)`.

Steps 1 through 6 are an implementation detail inside `execute`. The engine sees only the declared `Read`/`Write` sets and the compiled function body.

### Cdylib lint contract

The lint-author-facing surface lives in `viola-plugin-abi`, the domain-specific contract crate layered above `hilavitkutin-extensions` per the architectural position section of `DESIGN.md.tmpl` lines 114 to 129.

`ViolaLinterVtable` is a `#[repr(C)]` struct of concrete function pointers:

```rust
#[repr(C)]
pub struct ViolaLinterVtable {
    pub lint_fn: unsafe extern "C" fn(ctx: *const LintCallCtx) -> u32,
}
```

The `u32` return is a status code (`0` ok, non-zero fault, mapped to `ExtensionAbiStatus`). The `u32` is a bare FFI-wire primitive per the FFI-wire primitive policy at `DESIGN.md.tmpl` lines 49 to 58; each use site carries `// lint:allow(...) tracked: #206`.

`LintCallCtx` is a `#[repr(C)]` struct holding the morsel slice pointer, record range bounds, and a writer token for `Column<Diagnostic>`. Its exact field list is a follow-up concern after the `Column<Diagnostic>` write protocol is settled in viola task #254.

`PROVIDER_VIOLA_LINTER: ProviderId` is a `const` item in `viola-plugin-abi`:

```rust
pub const PROVIDER_VIOLA_LINTER: ProviderId =
    ProviderId::from_name(b"viola.linter.v1");
```

`ProviderId::from_name` is a `const fn` using FNV-64, documented at `DESIGN.md.tmpl` lines 100 to 104. Both the host and every cdylib reference this constant. Equality is guaranteed without runtime negotiation.

Lint authors implement `trait ViolaLinter` and annotate their struct with `#[viola::lint]`. The macro wraps `#[export_extension]` and emits the `ProviderExport` impl plus trampolines. This is the macro-driven static monomorphisation pattern from `EXTENSIONS_RESEARCH.md` lines 52 to 54: "explicit instantiation of monomorphized generic methods at compile time inside the plugin and a repr(C) descriptor containing extern C function pointers to those specific instances."

Example lint author code:

```rust
#[viola::lint]
struct MyLint;

impl ViolaLinter for MyLint {
    unsafe fn lint(&self, ctx: *const LintCallCtx) -> u32 {
        // read ctx.nam_slice, emit diagnostics via ctx.diag_writer
        0
    }
}
```

The `#[viola::lint]` macro generates the `ExtensionMeta` impl, a `static ViolaLinterVtable` with `lint_fn` pointing to a trampoline wrapping `MyLint::lint`, the `ProviderExport` impl with `ID = PROVIDER_VIOLA_LINTER`, and the standard `__hilavitkutin_extension_descriptor` symbol.

### Symbol export contract

A cdylib lint exports exactly two things:

1. The `__hilavitkutin_extension_descriptor` function returning `*const ExtensionDescriptor`. Standard pull-based discovery. The descriptor's `providers` table lists `PROVIDER_VIOLA_LINTER`.
2. The `ViolaLinterVtable` static, referenced by the `ProviderEntry` in the descriptor.

No linker sections, no `inventory`, no `.init_array`.

### How AccessSet flows across the FFI boundary

It does not. AccessSet is the host-side compile-time typestate. The cdylib never sees it. The lint author never declares `type Read` or `type Write`. The `LintCallCtx` is a flat `#[repr(C)]` struct; the host marshals it from what the WorkUnit's Context provides, inside `execute`, after the type-system proof is complete.

The engine typestate proof and the cdylib dispatch are temporally separated. The proof happens at scheduler-build time. The dispatch happens inside `execute` at runtime.

### Invariants preserved

No `dyn Trait` in the dispatch path. `ViolaLinterVtable` holds concrete `extern "C"` function pointers. The runtime call is through a raw function pointer, not a fat pointer. `#![no_std]` and no-alloc constraints hold on both sides: `LintCallCtx` and `ViolaLinterVtable` are plain structs; neither side allocates. Monomorphisation as dispatch: the scheduler registers `MAX_LINTS` distinct `RunLint<L>` instantiations; LLVM may fold identical bodies when all `L` values compile to identical code modulo the slot index constant, but correctness is independent of folding.

## Deferred but load-bearing

Two claims this memo asserts whose verification belongs in subsequent rounds. Both are load-bearing: if either fails to hold, Shape D itself is wrong, not just the surrounding implementation.

**Cross-WU commutative writes into a shared Column.** `RunLint<0>` through `RunLint<MAX_LINTS - 1>` all declare `Write = Cons<Column<Diagnostic>, Empty>` and `COMMUTATIVE = Bool::TRUE`. The `COMMUTATIVE` flag at `hilavitkutin-api::work_unit` line 70 documents commutativity "across record order" within a single WU's execute. The scheduler's interpretation of N distinct WU types writing commutatively into the same Column needs explicit confirmation. Either the dispatch codegen emits a reduce-style parallel pattern across the N types (matching `HILAVITKUTIN-APP-SHAPE.md` line 53's "parallel fan-out" intent), or it serialises them by access-set conflict. The runtime megaround's Pass 6 + 7 wiring (#423, #424) is the place this gets pinned; if it serialises, Shape D loses its parallelism guarantee and the design needs revisiting.

**Writer-handle as the AccessSet-meets-FFI point.** Step 5 of `RunLint<L>::execute()` marshals a `Column<Diagnostic>` writer handle into `LintCallCtx`. This is the single FFI surface where the engine's compile-time Write-set proof and the cdylib's actual writes intersect. The writer-handle protocol (its `#[repr(C)]` shape, its commutative-write semantics across FFI, the lifetime contract between host marshal and cdylib write call) must preserve the host-side typestate proof. Until that protocol lands in viola task #254 + the viola-plugin-abi crate, the AccessSet-never-crosses-FFI framing of this memo is a claim, not a verified property. Deferring the field list is cosmetic; deferring the semantic contract is not.

## What this memo does not cover

The exact field list of `LintCallCtx`: what the `Column<Diagnostic>` writer handle looks like as a `#[repr(C)]` type, and how the `Column<Nam>` morsel slice is represented across FFI. This belongs in a `viola-plugin-abi` design round after the `Column<Diagnostic>` write protocol is settled in viola task #254.

The exact value of `MAX_LINTS` and where viola-cli declares it. That is a viola-cli configuration decision.

ABI versioning strategy for `ViolaLinterVtable` when new optional hooks land. The `vtable_ptr` shape from `ProviderEntry` supports forward extension; the policy for bumping to `"viola.linter.v2"` is a follow-up.

Hot-reload: unload and reload a cdylib lint without restarting. `hilavitkutin-extensions` marks this BACKLOG at v1 (load-once, drop-once). `RunLint<L>::execute()` handles an empty slot safely; the `Resource<ExtensionHost>` teardown and repopulation protocol is a separate concern.

Runner and grammar plugin contracts. They follow the same pattern as `ViolaLinterVtable` but are out of scope for this memo.

## Implementation outline

The following are the next concrete steps. They are not src CL claims; those belong in the design rounds for each item.

**hilavitkutin task #609.** No engine-side source changes are required. The engine already supports `RunLint<L>` as a standard WorkUnit registration. This memo is the deliverable for task #609.

**viola-plugin-abi (pre-#254 viola work).** Define `ViolaLinterVtable`, `LintCallCtx` (placeholder fields pending #254), `PROVIDER_VIOLA_LINTER`, the `ViolaLinter` trait, and the `#[viola::lint]` proc-macro wrapping `#[export_extension]` with vtable emission. This unblocks mockspace task #610.

**viola-core (task #254 restructure).** Implement `RunLint<const L: usize>` with the WorkUnit impl described above. Define the viola-side `ExtensionHost` POD container holding `[Maybe<Extension>; MAX_LINTS]`. Implement `LoadPlugins` to populate `Resource<ExtensionHost>` by iterating configured plugin paths via `hilavitkutin_extensions::ExtensionHost::load`. Register `RunLint<0>` through `RunLint<MAX_LINTS - 1>` in viola-cli's scheduler builder.

**mockspace task #610.** Once `ViolaLinterVtable` and `#[viola::lint]` land, port the mockspace lint catalog to cdylibs. Each lint becomes a struct implementing `ViolaLinter`; vtable emission is automatic. The existing lint logic ports without changes to the lint-side Rust; only the export surface changes.
