# WorkUnit / cdylib boundary

**Date:** 2026-05-23 (revised 2026-05-24)
**Scope:** hilavitkutin task #609; viola task #254; mockspace task #610
**Status:** recommendation memo, aligned with landed `viola-plugin-abi`
**Source topics:** `viola/docs/internal/HILAVITKUTIN-APP-SHAPE.md`, `viola/crates/viola-plugin-abi/src/vtable.rs`, `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-api/src/work_unit.rs`, `mock/crates/hilavitkutin-api/src/builder_input.rs`, `mock/research/EXTENSIONS_RESEARCH.md`

## Revision note (2026-05-24)

The original draft proposed a host-side `LintCallCtx` aggregation struct wrapping the morsel slice, record range, and a `Column<Diagnostic>` writer handle, passed across FFI as a single `*const LintCallCtx`. That proposal is dropped. The landed `viola-plugin-abi::LintEvaluateVtable` (at `viola/crates/viola-plugin-abi/src/vtable.rs:102`) takes four direct args: `host_ctx`, `*const NamPayload`, `*const u8 + arvo::USize` config-bytes pair, `*mut DiagnosticBatch`. Three parallel specialists (architect, code-explorer, code-reviewer) independently converged on Option A (drop the wrapper, align with the landed direct-args shape) on the grounds that: (1) the landed vtable already names the four semantic arguments; (2) `hilavitkutin-workunit-mental-model.md` requires FFI wire forms to be constructed inline at the call site, not held as named types; (3) `RunnerExecuteScopeVtable` (whose `RunScope` wrapper exists because that struct names a spec concept) and `GrammarExtractVtable` (direct args) already establish the pattern that aggregation wrappers only earn their weight when they correspond to a named domain concept; the lint invocation has no such named concept. The writer-handle protocol that originally motivated the wrapper proposal remains a load-bearing open question; it is preserved below as a viola task #254 deferral. If that protocol later turns out to require host-side write dispatch (rather than the current plugin-owned `DiagnosticBatch` copy-buffer model), the host-side marshalling helper that consequence demands lives in `viola-core`, not in the ABI vtable, and does not retroactively change the landed direct-args signature.

## Hypothesis

The conflict between "engine WorkUnit types must be known at compile time" and "lints load at runtime from cdylibs" is a false dilemma. The two concerns operate on opposite sides of a dispatch boundary and do not need to meet.

`RunLint<L>` is a host-side type compiled into viola with fully specified `Read` and `Write` AccessSets. The cdylib lint never implements `WorkUnit`. It implements a viola-domain vtable contract. `RunLint<L>::execute()` retrieves that vtable pointer at runtime from `Resource<ExtensionHost>`, reads its Context accessors to obtain the morsel slice and writer-handle for `Column<Diagnostic>`, and dispatches through a raw function pointer with the args passed direct. AccessSet never crosses the FFI boundary. The engine typestate proof is complete before any cdylib loads.

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

1. Read `Resource<ExtensionHost>` from `ctx`. `ExtensionHost` is the canonical `hilavitkutin_extensions::ExtensionHost`, re-exported by `viola-core` (see `viola-core/src/lib.rs` `pub use hilavitkutin_extensions::{... ExtensionHost ...}`). Viola does not define a parallel host type; it consumes the foundation one. `LoadPlugins` populates the Resource by iterating configured plugin paths via `ExtensionHost::load`.
2. Access slot `L`. If `Maybe::Isnt`, return.
3. Call `Extension::provider(PROVIDER_LINT_EVALUATE)` on the handle. This is the `Extension::provider(id) -> Maybe<*const c_void>` accessor from `hilavitkutin-extensions` (`DESIGN.md.tmpl` line 66). If absent, return.
4. Cast the `*const c_void` to `*const LintEvaluateVtable` (definition at `viola/crates/viola-plugin-abi/src/vtable.rs:102`).
5. Read four args directly from the Context accessors at the call site: an `*const NamPayload` pointing into the `Column<Nam>` morsel slice for the current record range; a `*const u8 + arvo::USize` pair pointing at the resolved per-lint config bytes (from `Resource<LintConfigStore>` or the equivalent viola-side resource); and a `*mut DiagnosticBatch` writer handle marshalled from `Column<Diagnostic>`. No aggregation type holds these between read and call; the call site passes them direct.
6. Call `vtable.evaluate(host_ctx, nam, config_bytes, config_len, out_batch)` and map the returned `AbiStatus` (`ExtensionAbiStatus` alias) into the host's diagnostic-emission path.

Steps 1 through 6 are an implementation detail inside `execute`. The engine sees only the declared `Read`/`Write` sets and the compiled function body. No `*const LintCallCtx`-shaped wrapper exists at any layer; the four direct args ARE the FFI wire form.

### Cdylib lint contract

The lint-author-facing surface lives in `viola-plugin-abi`, the domain-specific contract crate layered above `hilavitkutin-extensions`. `LintEvaluateVtable` (`viola/crates/viola-plugin-abi/src/vtable.rs:102`) is the landed `#[repr(C)]` vtable struct:

```rust
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LintEvaluateVtable {
    pub evaluate: unsafe extern "C" fn(
        host_ctx: *mut c_void,
        nam: *const NamPayload,
        lint_config_bytes: *const u8,
        lint_config_len: arvo::USize,
        out_batch: *mut DiagnosticBatch,
    ) -> AbiStatus,
}
```

`AbiStatus` is the canonical `ExtensionAbiStatus` re-exported from `hilavitkutin-extensions` (alias declared at `viola-plugin-abi/src/lib.rs:45` for historical-callsite compatibility). The five direct args are the complete invocation surface; nothing the host passes is bundled into a wrapper struct ahead of the call.

`PROVIDER_LINT_EVALUATE: ProviderId` is a `const` item in `viola-plugin-abi/src/lib.rs:97`:

```rust
pub const PROVIDER_LINT_EVALUATE: ProviderId =
    ProviderId::from_name("viola.lint.evaluate.v1");
```

`ProviderId::from_name` is a `const fn` using FNV-64. Both the host and every cdylib reference this constant. Equality is guaranteed without runtime negotiation. The `v1` suffix anchors the ABI major; introducing a `LintEvaluateVtable` variant with a new function pointer registers as `viola.lint.evaluate.v2` rather than mutating the v1 shape.

Lint authors declare the export via `#[hilavitkutin_extensions_macros::export_extension]` (the descriptor + lifecycle macro), with the lint function pointer captured in a `LintEvaluateVtable` static referenced from a `ProviderEntry` in the emitted descriptor. The descriptor's `providers` table lists `PROVIDER_LINT_EVALUATE`; the `ProviderEntry::vtable_ptr` points at the static. This follows the macro-driven static-monomorphisation pattern from `EXTENSIONS_RESEARCH.md` lines 52 to 54: explicit instantiation of monomorphized methods at compile time inside the plugin, a `#[repr(C)]` descriptor pointing at those specific instances.

A higher-level lint-author ergonomic surface (a `LintEvaluator` trait + `#[viola::lint]` macro wrapping `#[export_extension]` plus vtable emission) is a downstream proposal that lives in viola-core / `viola-plugin-abi-derive`, not in `viola-plugin-abi` itself. Its shape settles in viola task #254 and is out of scope for this memo; what `viola-plugin-abi` ships today is the vtable + provider-id pair, and that is enough for either a hand-written `export_extension` invocation or a future ergonomic macro to target.

### Symbol export contract

A cdylib lint exports exactly two things:

1. The `__hilavitkutin_extension_descriptor` function returning `*const ExtensionDescriptor` (the `DESCRIPTOR_SYMBOL` constant re-exported from `viola-plugin-abi`). Standard pull-based discovery. The descriptor's `providers` table lists `PROVIDER_LINT_EVALUATE`.
2. The `LintEvaluateVtable` static, referenced by the `ProviderEntry::vtable_ptr` in the descriptor.

No linker sections, no `inventory`, no `.init_array`.

### How AccessSet flows across the FFI boundary

It does not. AccessSet is the host-side compile-time typestate. The cdylib never sees it. The lint author never declares `type Read` or `type Write`. The four direct args passed to `LintEvaluateVtable::evaluate` are flat FFI primitives (raw pointers, `arvo::USize`, `*mut DiagnosticBatch`) marshalled from what the WorkUnit's Context provides, inline at the call site inside `execute`, after the type-system proof is complete.

The engine typestate proof and the cdylib dispatch are temporally separated. The proof happens at scheduler-build time. The dispatch happens inside `execute` at runtime.

### Invariants preserved

No `dyn Trait` in the dispatch path. `LintEvaluateVtable` holds a concrete `extern "C"` function pointer. The runtime call is through a raw function pointer, not a fat pointer. `#![no_std]` and no-alloc constraints hold on both sides: `LintEvaluateVtable` is a plain `#[repr(C)]` struct of one word, and the call-site marshalling holds nothing it has to allocate; neither side allocates. Monomorphisation as dispatch: the scheduler registers `MAX_LINTS` distinct `RunLint<L>` instantiations; LLVM may fold identical bodies when all `L` values compile to identical code modulo the slot index constant, but correctness is independent of folding.

## Deferred but load-bearing

Two claims this memo asserts whose verification belongs in subsequent rounds. Both are load-bearing: if either fails to hold, Shape D itself is wrong, not just the surrounding implementation.

**Cross-WU commutative writes into a shared Column.** `RunLint<0>` through `RunLint<MAX_LINTS - 1>` all declare `Write = Cons<Column<Diagnostic>, Empty>` and `COMMUTATIVE = Bool::TRUE`. The `COMMUTATIVE` flag at `hilavitkutin-api::work_unit` line 70 documents commutativity "across record order" within a single WU's execute. The scheduler's interpretation of N distinct WU types writing commutatively into the same Column needs explicit confirmation. Either the dispatch codegen emits a reduce-style parallel pattern across the N types (matching `HILAVITKUTIN-APP-SHAPE.md`'s "parallel fan-out" intent), or it serialises them by access-set conflict. The runtime megaround's Pass 6 + 7 wiring (#423, #424) pinned the dispatch codegen; this memo defers the explicit confirmation pass against that landed code to a follow-up audit. If the scheduler serialises, Shape D loses its parallelism guarantee and the design needs revisiting.

**Writer-handle protocol across FFI.** Step 5 of `RunLint<L>::execute()` marshals a `Column<Diagnostic>` writer handle into a `*mut DiagnosticBatch` for the FFI call. The current `DiagnosticBatch` wire shape (`viola-plugin-abi/src/diagnostic.rs`) assumes a plugin-owned output buffer that the host copies after the call. That assumption settles the writer-handle protocol as "copy-buffer" and keeps the direct-args ABI sound: the writer never crosses FFI as a host-owned object the plugin calls into. If viola task #254 later changes the protocol to host-owned write dispatch (the plugin invokes a host-provided writer through a vtable handle), a host-side marshalling helper appears in `viola-core` at that point; it does not retroactively change the landed `LintEvaluateVtable` direct-args signature, and it does not require resurrecting a `LintCallCtx` wrapper at the ABI layer. The protocol choice is the load-bearing question for #254; the ABI direct-args shape is correct under either resolution.

**Meta-virtual registration contract.** CLOSED 2026-05-24 via Option C, already landed. Three parallel specialists (architect / code-explorer / code-reviewer) converged unanimously on Option C, which the runtime megaround already landed via `AdaptWu`. The mechanism: `WorkUnit` carries a `Schedule` generic parameter defaulting to `Always`; lifecycle-bound WUs implement `WorkUnit<On<Marker>>` where `Marker` is one of `PlanStage` / `ScheduleReady` / `PassStart` / `ScheduleEnd`. The scheduler reads the `Schedule` type at plan-build time to route the WU to the corresponding boundary fire point.

Landed precedent: `impl WorkUnit<On<ScheduleEnd>> for AdaptWu` at `hilavitkutin-providers/src/adapt_wu.rs:63`. The `WorkUnit<Schedule = Always>` trait sits at `hilavitkutin-api/src/work_unit.rs:40`. `On<V>` and `Always` are the two shipped schedule markers. Registration follows the same `.with(WuStruct::default())` path as any other WU; no separate `.on_pass_start::<WU>()` builder surface exists or needs to exist.

For viola's lint-batch lifecycle, this means:

- `LoadPlugins` implements `WorkUnit<On<PlanStage>>` to populate `Resource<ExtensionHost>` once during scheduler build.
- A `SortAndEmitDiagnostics` WU implements `WorkUnit<On<ScheduleEnd>>` to read `Column<Diagnostic>`, sort, and emit at pass end.
- The `RunLint<L>` host WUs themselves stay on the default `Always` schedule (per-record / per-pass dispatch), as today.

**Optional follow-up flagged by the reviewer**: seal `On<V>` to require `V: ScheduleMarker` (a new sealed trait covering the four marker structs), so accidentally writing `On<ArbitraryType>` does not compile. Additive constraint, not a redesign; lands as a focused PR in `hilavitkutin-api` before viola starts registering its meta-virtual WUs.

With this resolution, **both** original "deferred but load-bearing" #254 design questions are now closed (the LintCallCtx question was closed 2026-05-23 via the same 3-specialist pattern). The two genuine load-bearing deferrals remaining in this memo are now: (1) cross-WU commutative writes verification against landed dispatch codegen, and (2) writer-handle protocol across FFI (`DiagnosticBatch` copy-buffer model assumption).

## What this memo does not cover

The exact value of `MAX_LINTS` and where viola-cli declares it. That is a viola-cli configuration decision.

ABI versioning strategy for `LintEvaluateVtable` when new optional fields or alternative dispatch shapes land. The `ProviderId` is the version anchor; adding a new optional hook registers as `viola.lint.evaluate.v2` rather than mutating v1.

Hot-reload: unload and reload a cdylib lint without restarting. `hilavitkutin-extensions` marks this BACKLOG at v1 (load-once, drop-once). `RunLint<L>::execute()` handles an empty slot safely; the `Resource<ExtensionHost>` teardown and repopulation protocol is a separate concern.

Runner and grammar plugin contracts. `RunnerExecuteScopeVtable` and `GrammarExtractVtable` already ship in `viola-plugin-abi` alongside `LintEvaluateVtable`. The runner role uses a bundled `*const RunScope` because the spec names a "configured run scope" as a domain concept; the grammar role uses direct args (like the lint role). Both follow the same dispatch pattern outlined above for `RunLint<L>` but with their own host-side WorkUnit shape. Out of scope for this memo.

## Implementation outline

The following are the next concrete steps. They are not src CL claims; those belong in the design rounds for each item.

**hilavitkutin task #609.** No engine-side source changes are required. The engine already supports `RunLint<L>` as a standard WorkUnit registration. This memo is the deliverable for task #609.

**viola-plugin-abi.** The vtable + provider-id pair (`LintEvaluateVtable`, `PROVIDER_LINT_EVALUATE`) already ships. No further pre-#254 ABI work is required by the boundary design. An ergonomic lint-author surface (a `LintEvaluator` trait + `#[viola::lint]` macro) is a downstream proposal that lives in viola-core or a sibling derive crate; it lands when #254 picks the shape.

**viola-core (task #254 restructure).** Implement `RunLint<const L: usize>` with the WorkUnit impl described above. The `Resource<ExtensionHost>` carrier is the canonical `hilavitkutin_extensions::ExtensionHost` re-exported by viola-core. `LoadPlugins` (a WorkUnit on the `Virtual<PlanStage>` meta-marker, per `HILAVITKUTIN-APP-SHAPE.md`) populates the Resource by iterating configured plugin paths via `ExtensionHost::load`. Register `RunLint<0>` through `RunLint<MAX_LINTS - 1>` in viola-cli's scheduler builder. The meta-virtual registration contract for `PassStart` / `ScheduleEnd` lands in this same task.

**mockspace task #610.** Once viola-core's `RunLint<L>` lands, port the mockspace lint catalog to cdylibs. Each lint becomes a function compiled into a cdylib that exports a `LintEvaluateVtable` static with a function pointer matching the v1 `evaluate` signature. The existing lint logic ports with mechanical changes (signature adaptation + descriptor + provider-export wiring); the lint-evaluation body itself stays the same.
