**Date:** 2026-05-08
**Phase:** TOPIC
**Scope:** hilavitkutin-api, hilavitkutin-kit
**Source topics:** task #332 (HILA-AUDIT-A3), round 202605042200 follow-up (audit findings)

# Trait diagnostics across the public API + recursion-limit guidance + prelude module

## Background

Substrate-completion task #332 names two related ergonomic gaps inherited from earlier rounds:

1. **Trait diagnostics.** When a consumer mistakes a bound (forgets to register a store, hands the engine a non-WorkUnit type, builds a Kit without the required associated types, etc.) rustc emits the bare unsatisfied-trait-bound message naming the substrate trait. The default error reads as substrate-internal vocabulary (`Contains<Column<Pos>>` is not implemented for `Cons<Column<Vel>, Empty>`); a substrate that ships its own diagnostics via `#[diagnostic::on_unimplemented]` can route the consumer to the actual fix instead. Round 202605011500 already added the attribute to `Contains<S>` and `ContainsAll<L>`. Every other consumer-facing trait in hilavitkutin-api (and the `Kit` trait in hilavitkutin-kit) ships without a custom diagnostic message.

2. **Recursion-limit guidance.** Round 202605011500 set `#![recursion_limit = "512"]` on api, kit, and engine crate roots. Realistic Kit composition (4+ nested layers, 30+ flat WUs) approaches the limit and consumers occasionally hit "overflow evaluating the requirement" with no in-substrate guidance pointing at the fix. Round 202605042200 specced a `recursion_limit_for_kits!()` macro shipping from `hilavitkutin-api::prelude` that consumers invoke at their crate root. The macro was supposed to expand to `#![recursion_limit = "1024"]`.

Sketch S1 (`mock/research/sketches/202605082230_recursion_limit_macro/`) verified that the macro shape **does not work in current rustc**. `macro_rules!` macros cannot expand to crate-level inner attributes when invoked at the crate root; the compiler rejects with "an inner attribute is not permitted in this context". The locked round 202605042200 design must yield to the practical limitation. Workspace task #396 records the regression and tracks the rustc gap.

This round ships the parts that are actually buildable: the diagnostics across the public trait surface, a `prelude` module with re-exports (no macro), and DESIGN.md.tmpl prose that documents the recursion-limit directive consumers must write themselves.

## Workspace sweep

The hilavitkutin-api public-trait surface enumerated:

| Trait | File | Currently has `on_unimplemented`? |
|---|---|---|
| `AccessSet` | `src/access.rs` | sealed; consumers don't impl |
| `Contains<S>` | `src/access.rs` | yes |
| `ContainsAll<L>` | `src/access.rs` | yes |
| `Concat<L>` | `src/access.rs` | no |
| `WorkUnit<Schedule>` | `src/work_unit.rs` | no |
| `WorkUnitBundle` | `src/work_unit.rs` | no |
| `StoreBundle` | `src/store.rs` | no |
| `Replaceable` | `src/store.rs` | no |
| `ColumnValue` | `src/column_value.rs` | no |
| `SchedulingHint` | `src/hint.rs` | sealed; no |
| `UrgencyValue` | `src/hint.rs` | sealed; no |
| `DivisibilityValue` | `src/hint.rs` | sealed; no |
| `SignificanceValue` | `src/hint.rs` | sealed; no |
| `Depth` | `src/builder.rs` | sealed; no |
| `Push<T>` | `src/capability.rs` | no |
| `BulkPush<T>` | `src/capability.rs` | no |
| `Len` | `src/capability.rs` | no |
| `Capacity` | `src/capability.rs` | no |
| `BoundedPush<T>` | `src/capability.rs` | no |
| `Collector<T>` | `src/sink.rs` | no |
| `DiagnosticSink<E>` | `src/sink.rs` | no |
| `ByteEmitter` | `src/sink.rs` | no |
| `Encoder<T>` | `src/codec.rs` | no |
| `Decoder<T>` | `src/codec.rs` | no |
| `EncoderExt<T>` | `src/codec.rs` | extension trait, has default methods |
| `DecoderExt<T>` | `src/codec.rs` | extension trait, has default methods |
| `MemoryProviderApi` | `src/platform.rs` | no |
| `ThreadPoolApi` | `src/platform.rs` | no |
| `ClockApi` | `src/platform.rs` | no |
| `HasMemoryProvider` | `src/platform.rs` | no |
| `HasThreadPool` | `src/platform.rs` | no |
| `HasClock` | `src/platform.rs` | no |
| `ColumnReaderApi<R>` | `src/context.rs` | no |
| `ColumnWriterApi<W>` | `src/context.rs` | no |
| `ResourceProviderApi<R>` | `src/context.rs` | no |
| `VirtualFirerApi<W>` | `src/context.rs` | no |
| `EachApi<R, W>` | `src/context.rs` | no |
| `BatchApi<R, W>` | `src/context.rs` | no |
| `ReduceApi<R, W>` | `src/context.rs` | no |
| `Kit` | `hilavitkutin-kit/src/lib.rs` | no |

## Decisions

### Decision 1: `prelude` module ships, macro does not

Add `mock/crates/hilavitkutin-api/src/prelude.rs` and `pub mod prelude;` to `lib.rs`. The prelude re-exports the most-used types so consumers can `use hilavitkutin_api::prelude::*;` and pick up the `Empty`, `Cons`, `read!`, `write!`, `WorkUnit`, `Resource`, `Column`, `Virtual`, `Always`, `On`, and the load-bearing trait names without naming each individually.

Rationale: prelude modules are an idiomatic Rust pattern that makes the consumer-facing surface discoverable. Round 202605042200's prelude design called for the recursion-limit macro to live here too; that macro is dropped (sketch S1 finding), but the module is independently valuable and still ships.

### Decision 2: `#[diagnostic::on_unimplemented]` covers the consumer-facing trait surface

Add the attribute to every trait in the table above marked "no", excluding the extension traits `EncoderExt`/`DecoderExt` (which have default-method default impls and rarely produce raw bound failures) and excluding `AccessSet` (sealed; consumers reach for `Contains`/`ContainsAll` not the supertrait directly). Sealed marker traits like `Depth`, `SchedulingHint`, `UrgencyValue`, `DivisibilityValue`, `SignificanceValue` get diagnostics that name the available implementors so consumers know their menu.

Each diagnostic carries:

- `message`: one-line summary that names the substrate concept in consumer-facing English. Avoids substrate-internal vocabulary like "AccessSet typestate" or "cons-list cardinality" except where unavoidable.
- `note` (optional): the actual fix. Names the constructor, the `.add` / `.resource` / `.column` method, the macro invocation, or the recursion-limit directive depending on the trait.

### Decision 3: messages by trait

Concrete message text per trait. Substrate-internal phrasing is allowed only where the consumer needs to recognise the type name in their own error trail.

**`Contains<S>`** (already shipped, retain):
> store `{Self}` does not contain `{S}`
> note: Register it with `.resource::<T>(initial)`, `.column::<T>()`, `.add_virtual::<T>()`, or install a Kit that registers it.

**`ContainsAll<L>`** (already shipped, retain).

**`Concat<L>`**:
> cannot concatenate `{L}` onto `{Self}`
> note: `Concat` is implemented for `Empty` and `Cons<H, T>` where `T: Concat<L>`. Make sure both sides are cons-lists built from `Empty` and `Cons<H, T>` (the `read!` / `write!` macros emit this shape).

**`WorkUnit<Schedule>`**:
> `{Self}` is not a WorkUnit (or its `Schedule` does not match `{Schedule}`)
> note: Implement `WorkUnit` on `{Self}`. Declare `type Read`, `type Write`, `type Hint`, `type Ctx`, and `fn execute(&self, ctx: &Self::Ctx)`. The default `Schedule` is `Always`; override with `On<V>` for virtual-fired WUs.

**`WorkUnitBundle`**:
> `{Self}` is not a WorkUnitBundle
> note: WorkUnitBundle is auto-implemented for `Empty` and for `Cons<W, T>` where `W: WorkUnit` and `T: WorkUnitBundle`. Build the bundle through the scheduler builder's `.add::<W>()` calls.

**`StoreBundle`**:
> `{Self}` is not a StoreBundle
> note: StoreBundle is auto-implemented for `Empty` and `Cons<H, T>` where `T: StoreBundle`. Build the bundle through the scheduler builder's `.resource::<T>(initial)`, `.column::<T>()`, and `.add_virtual::<T>()` calls, or install a Kit whose `Owned` declares it.

**`ColumnValue`**:
> `{Self}` cannot be stored in a `Column`
> note: ColumnValue requires `Copy + 'static`. Reduce or transform the value to a fixed-size `Copy` type. arvo's `UFixed`, `IFixed`, `Bits<N, S>`, `Bool`, and `USize` are valid; `String`, `Vec<T>`, and `Box<T>` are not.

**`Replaceable`**:
> `{Self}` is not opt-in for Scheduler::replace_resource
> note: Mark the type with `impl Replaceable for {Self} {}` to opt in. Replaceable is intentionally explicit: stores omitted from the replacement set are stable for plan-time analysis.

**`SchedulingHint`** (sealed):
> `{Self}` is not a SchedulingHint tuple
> note: SchedulingHint is implemented on the tuple `(U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue)`. Use the substrate-provided ZST markers (`Immediate` / `Deferred` / etc for U, `Atomic` / `Interruptible` for D, `Critical` / `Normal` / `Optional` for S).

**`UrgencyValue` / `DivisibilityValue` / `SignificanceValue`** (sealed):
> `{Self}` is not a {axis} marker
> note: Available markers are: {axis-specific list}. Sealed; consumer-defined markers are not supported.

**`Depth`** (sealed):
> `{Self}` does not carry a Depth
> note: Depth is sealed and implemented internally by the scheduler builder. If you reach this from consumer code, you have likely named a builder shape that does not exist; check the builder method chain.

**`Push<T>` / `BulkPush<T>` / `Len` / `Capacity` / `BoundedPush<T>`** (capability traits):

`Push<T>`:
> `{Self}` cannot accept items of type `{T}` via Push
> note: Implement `Push<T>` to declare item-acceptance. The substrate ships impls for the standard sink types in `hilavitkutin-api::sink`; consumer-side push targets implement Push directly.

(Similar pattern for the other four; each names the trait family and points at the sink module.)

**`Collector<T>`**:
> `{Self}` is not a Collector for `{T}`
> note: Collector is `Push<T>` plus the contract that pushes always succeed. Implement Push<T> first; consumers that can refuse on full implement BoundedPush instead.

**`DiagnosticSink<E>`**:
> `{Self}` is not a DiagnosticSink for `{E}`
> note: DiagnosticSink is `Push<E> + Len`. Implement both to expose the sink to WorkUnit diagnostics.

**`ByteEmitter`**:
> `{Self}` is not a ByteEmitter
> note: ByteEmitter is `Push<u8> + BulkPush<u8>`. Implement both to expose the byte-stream contract; codecs `Encoder<T>` and `Decoder<T>` write through this trait.

**`Encoder<T>` / `Decoder<T>`**:
> `{Self}` does not encode/decode `{T}`
> note: Implement `Encoder<T>` (or `Decoder<T>`) for the wire format you control. The hilavitkutin-api::codec module ships defaults for common cases.

**`MemoryProviderApi` / `ThreadPoolApi` / `ClockApi`**:
> `{Self}` does not implement {Memory|Thread|Clock} provider contract
> note: Provide a platform-specific impl. The engine builds against these traits via const generics; supply your own `MemoryProvider` / `ThreadPool` / `Clock` to the scheduler at construction time.

**`HasMemoryProvider` / `HasThreadPool` / `HasClock`**:
> provider tuple `{Self}` does not expose a {Memory|Thread|Clock} provider
> note: Compose the provider tuple with the `provider_generic!` / `provider_generic2!` accessors. The substrate's `Context<P>` framework wires this from the scheduler builder.

**`ColumnReaderApi<R>` / `ColumnWriterApi<W>` / `ResourceProviderApi<R>` / `VirtualFirerApi<W>` / `EachApi<R, W>` / `BatchApi<R, W>` / `ReduceApi<R, W>`**:

These are read- or write-set-parameterised provider APIs. The diagnostic message points the consumer at the relevant `Has*` accessor and notes the `R` / `W` set as the constraint.

> `{Self}` does not provide {column-read | column-write | resource | virtual-fire | each | batch | reduce} API for sets `{R}` / `{W}`
> note: This trait is implemented by the scheduler-generated context type. If your WorkUnit Ctx hits this bound, ensure the provider tuple satisfies `Has*` for each accessor needed.

**`Kit`** (in hilavitkutin-kit):
> `{Self}` is not a Kit
> note: Implement `Kit` by declaring `type Units: WorkUnitBundle` (the WorkUnit cons-list, often built with `read!` / `write!`) and `type Owned: StoreBundle` (the Resource / Column / Virtual cons-list). The engine reads these at compile time on `.add_kit::<K>()`.

### Decision 4: Recursion-limit guidance prose, not macro

Add a "Recursion limit" subsection to `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` (under the section that documents `Contains` / `ContainsAll`). Content:

- Default `recursion_limit = 128` in rustc accommodates apps up to roughly 30 WUs by 30 stores.
- The api, kit, and engine crates declare `#![recursion_limit = "512"]` internally to handle their own machinery.
- Consumer crates whose `.build()` hits "overflow evaluating the requirement" should declare `#![recursion_limit = "1024"]` (or higher) at their own crate root.
- The substrate cannot ship a macro that expands to `#![recursion_limit]` because rustc rejects macro-expansion of crate-level inner attributes (sketch S1 finding 2026-05-08).
- The trait diagnostics on `Contains` / `ContainsAll` / `Kit` / `WorkUnit` mention the recursion-limit directive in their notes when the bound failure could be caused by depth rather than a missing registration.

The `Contains` and `ContainsAll` diagnostic notes get an additional sentence: "If your `.build()` hits 'overflow evaluating the requirement', declare `#![recursion_limit = "1024"]` at your crate root."

Cross-reference the kit DESIGN.md.tmpl with a one-line note pointing at the api section.

### Decision 5: prelude module contents

```rust
//! Convenient re-exports for hilavitkutin-api consumers.
//!
//! `use hilavitkutin_api::prelude::*;` brings in:
//!
//! - The cons-list typestate primitives: `Empty`, `Cons`.
//! - The construction macros: `read!`, `write!`.
//! - The schedule + WorkUnit traits: `WorkUnit`, `Always`, `On`.
//! - The store markers: `Resource`, `Column`, `Virtual`.
//! - The membership witnesses: `AccessSet`, `Contains`, `ContainsAll`.
//! - `WorkUnitBundle`, `StoreBundle` for kit authors.
//! - `Concat` for type-level cons-list append.
//!
//! Provider-side and platform-contract traits stay out of the
//! prelude; consumers that need them name them directly to keep
//! their import set self-documenting.

pub use crate::access::{AccessSet, Concat, Cons, Contains, ContainsAll, Empty};
pub use crate::store::{Column, Replaceable, Resource, StoreBundle, Virtual};
pub use crate::work_unit::{Always, On, WorkUnit, WorkUnitBundle};
pub use crate::{read, write};
```

The `read!` / `write!` macros are `#[macro_export]` from `src/macros.rs` and reach the consumer via `crate-name::macro_name`; re-exporting them via `pub use crate::{read, write};` from the prelude makes the prelude self-contained for the typical consumer pattern.

## Out of scope

- The `recursion_limit_for_kits!()` macro itself. Sketch S1 proved it cannot work; task #396 tracks the rustc gap.
- New trait additions or removals. This round only adds attributes to existing traits and a re-exporting prelude module.
- Trybuild compile-fail fixtures verifying the diagnostic messages reach the user. Tracked separately as task #296. The diagnostics are added now; fixture-level verification follows when trybuild infrastructure lands.
- Diagnostic coverage for `EncoderExt<T>` / `DecoderExt<T>`. Extension traits with default-method default impls rarely surface raw bound failures to consumers; deferred until a consumer reports otherwise.
- Diagnostic message i18n. The substrate is single-language for v0.x.

## Lock criteria

- `prelude` module created with the re-exports listed above.
- `pub mod prelude;` added to `mock/crates/hilavitkutin-api/src/lib.rs`.
- Every trait listed in the workspace-sweep table marked "no" gains a `#[diagnostic::on_unimplemented]` attribute matching Decision 3's text (sealed-axis traits get the available-markers list per their axis).
- `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` gains a "Recursion limit" subsection per Decision 4.
- `mock/crates/hilavitkutin-kit/DESIGN.md.tmpl` cross-references the api section.
- `Contains` and `ContainsAll` diagnostic notes get the recursion-limit hint sentence.
- `cargo check --workspace` passes clean. `cargo test -p hilavitkutin-api` passes (where it was passing before).
- Sketch artefacts in `mock/research/sketches/202605082230_recursion_limit_macro/` committed for audit trail.
