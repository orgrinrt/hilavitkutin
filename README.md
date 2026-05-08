# `hilavitkutin`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/hilavitkutin.svg)](https://github.com/orgrinrt/hilavitkutin/stargazers)
[![Crates.io](https://img.shields.io/crates/v/hilavitkutin)](https://crates.io/crates/hilavitkutin)
[![docs.rs](https://img.shields.io/docsrs/hilavitkutin)](https://docs.rs/hilavitkutin)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/hilavitkutin.svg)](https://github.com/orgrinrt/hilavitkutin/issues)
![License](https://img.shields.io/github/license/orgrinrt/hilavitkutin?color=%23009689)

> Static scheduler and runtime that turns read / write access sets into per-core dispatch with cache-aware morsels. `no_std`, no alloc, no spawn, no dyn.

</div>

Typed WorkUnits declare their read / write access sets to the scheduler builder. Each `.add::<W>()` accumulates the WorkUnit type onto the builder's typestate; each `.column::<T>()`, `.resource::<T>(initial)`, or `.add_virtual::<T>()` registers a store. At `.build()`, every declared access is type-checked against the registered stores. A WorkUnit that names a store the builder does not have fails to compile, with the diagnostic pointing at the missing marker.

`.build()` then runs the plan stage end to end. Access-set overlap builds the dependency graph; topological sort and upward rank find the critical path; waist detection partitions phases; RCM reordering and block-diagonal layout cluster adjacent work for cache locality; spectral partitioning groups trunks; combinatorial DP groups fibers; per-fiber morsels are sized to cache. The output is a per-core dispatch program: a monomorphised function per physical core encoding phases, record ranges, morsel boundaries, and sync points.

At run time, the dispatch programs drive a pre-allocated thread pool. Trunks of fibers pin to specific cores for L1 warmth; morsels window into columns of records for cache-aware iteration; LLVM devirtualises the per-core programs into straight-line code. Phases overlap via atomic progress counters: phase N+1 starts as soon as phase N produces enough records for one morsel. Pipeline parallelism is derived from the declared access sets at plan time, not synthesised at run time from hand-written sync primitives.

## Scheduler builder

The scheduler builder is a typestate. Each registration accumulates a type parameter onto the builder; `.build()` reads the accumulated state and type-checks every registered WorkUnit's read / write access against the registered stores.

Three registration shapes cover the surface. `.add::<W>()` adds one `WorkUnit` type to the typestate. `.column::<T>()`, `.resource::<T>(initial)`, and `.add_virtual::<T>()` each register one store. `.add_kit::<K>()` reads a `Kit` impl's `type Units` and `type Owned` declarations at compile time and prepends them onto the accumulators in one step; Kits are how a crate ships a bundled set of WorkUnits and stores under a single named registration.

```rust
use hilavitkutin::Scheduler;
use hilavitkutin_providers::{InternerKit, default_interner};

let scheduler = Scheduler::builder()
    .resource(default_interner::<4096, 256>())
    .add_kit::<InternerKit<4096, 256>>()
    .add_kit::<RunnerKit>()
    .add_kit::<LinterKit>()
    .add::<MyWU>()
    .build();
```

`.build()` carries the typestate constraint that every registered WorkUnit's `Read` and `Write` access sets are fully covered by the accumulated stores. A WorkUnit naming a `Resource<T>` that no registration covers fails to compile; the diagnostic names the missing store directly:

```text
note: store `Empty` does not contain `Resource<Interner>`. Register it with
      `.resource::<Interner>(initial)`, `.column::<Interner>()`,
      `.add_virtual::<Interner>()`, or install a Kit that owns it.
```

Order of registration matters for the type-level proof. A Kit whose WorkUnits read from another Kit's owned state must come after the owning Kit in the chain (or after the relevant `.resource(_)` / `.column()` calls). The builder accepts any order that satisfies the proof; the diagnostic above pinpoints what is missing when it does not.

Once `.build()` returns, the resulting `Scheduler<Wus, Stores>` is the runtime handle. The two type parameters are phantom and carry the typestate forward; methods on `Scheduler` consume that typestate to dispatch work and, for stores marked `Replaceable`, to allow targeted resource swaps between runs.

## Plan and runtime

The plan stage selects a strategy per pipeline shape and a config per phase. Strategy selection happens at plan time based on record count and DAG topology: shallow pipelines run sequential, wide pipelines run adaptive, mixed pipelines run phased with parallel trunks. Per-phase configs (`MAX_FUSE`, `BALANCED`, `MAX_SPLIT`) tune how aggressively the dispatch fuses or splits within the phase. None of this changes during a run; the next plan recompute fires only on structural change to the pipeline (new WorkUnits registered, record-count shift, DAG modification).

At run time, two extras compound the per-core cache locality. Commutative fibers parallelise from both ends: a head thread iterates forward, a tail thread iterates backward, accumulators merge at convergence; every commutative fiber gets roughly 2x parallelism for free. On targets with heterogeneous cores, critical-path trunks land on performance cores, and branches and leaves land on efficiency cores with proportionally smaller morsels. Sync points use predictive parking, with short waits as spin loops and long waits as parked threads, sized from runtime measurements rather than fixed thresholds.

The adapt subsystem watches per-morsel timing across runs and proposes recompute when EMA trends drift past tolerance. Plan recompute stays rare by construction: the typestate fixes the structural shape; the adapt subsystem only revises sizes and assignments. The scheduler is itself a pipeline; the plan, dispatch, and adapt stages run as meta WorkUnits on the same engine that runs registered code.

## Provider context

The provider context framework lives in `hilavitkutin-ctx` as a standalone crate (no engine dependency, zero dependencies, `#![no_std]`). It is the mechanism behind every WorkUnit's `Self::Ctx` slot, and it is independently usable by any consumer that needs typed access to a tuple of provider implementations.

The shape is small. `Context<P>` wraps a provider tuple `P`. Three macros generate the rest:

- `provider!(SomeApi as HasSomeProvider => some_method)` declares an accessor trait; the method on the right becomes the call-site form (`ctx.some_method()`).
- `tuple!(A: HasFoo => foo, B: HasBar => bar)` writes the accessor impls per tuple layout, so position 0 gives `Foo` and position 1 gives `Bar`. Wrong order at the call site fails to compile because the bound is not satisfied.
- `define_providers!` is a facade that bundles `provider!` and `tuple!` calls into one declaration.

A domain trait declares its `type Ctx: HasFoo + HasBar` and writes the body against `ctx.foo()` / `ctx.bar()`. The same `Self::Ctx` slot resolves per trait, so a single struct can implement two traits with different `Ctx` shapes without ambiguity.

Within hilavitkutin, every WorkUnit's `Self::Ctx` is a provider tuple expressing read / write access plus the engine-required platform contracts (`HasColumnReader<R>`, `HasColumnWriter<W>`, `HasResourceProvider<R>`, `HasVirtualFirer<W>`, `HasEach<R, W>`, `HasBatch<R, W>`, `HasReduce<R, W>`). The `provider_generic!` / `tuple_generic!` variants thread the access-set type parameters through. Outside hilavitkutin, the same machinery works elsewhere: a separate consumer crate can declare its own domain trait with `type Ctx: HasConnector + HasWriter` and stay independent of the engine entirely.

```rust
use hilavitkutin_ctx::define_providers;
use notko::Outcome;

// declare provider apis and one tuple layout.
define_providers! {
    providers {
        FormatterApi as HasFormatter => formatter,
        SinkApi      as HasSink      => sink,
    }
    layouts {
        (FormatterApi, SinkApi),       // pos 0 = formatter, pos 1 = sink
    }
}

// a domain trait declares which providers its ctx carries.
trait Logger {
    type Ctx: HasFormatter + HasSink;
    fn log(&self, ctx: &Self::Ctx, event: Event) -> Outcome<(), Oops>;
}

// concrete impl supplies a tuple that matches the layout.
struct JsonLogger;
impl Logger for JsonLogger {
    type Ctx = (JsonFormatter, FileSink);            // both impl their api traits
    fn log(&self, ctx: &Self::Ctx, event: Event) -> Outcome<(), Oops> {
        let line = ctx.formatter().format(event)?;   // tuple pos 0 via HasFormatter
        ctx.sink().write(line)?;                     // tuple pos 1 via HasSink
        Outcome::Ok(())
    }
}
```

## Extensions and extensibility

`hilavitkutin-linking` is the cross-platform `dlopen` / `LoadLibrary` primitive: explicit symbol lookup by name, `no_std`, no allocator, no linker-magic registration. The handle is `'static`, and the typed `Symbol<'lib, F>` and `StaticRef<'lib, T>` handles tie their lifetime to the source `Library` so use-after-close is a compile-time error. v1 supports macOS / Linux / Windows on the common architectures; wasm32 is explicitly out of scope.

`hilavitkutin-extensions` adds the `ExtensionDescriptor` `#[repr(C)]` shape and per-extension lifecycle on top. Discovery is one well-known exported symbol resolved through the linking layer; no linker sections are scanned. The descriptor carries the ABI version, name, version, required host capabilities, and a capabilities table keyed by stable `CapabilityId`. Each loaded `Extension` handle owns one `Library` plus a host-opaque context pointer threaded through optional init and shutdown calls. A pluggable failure policy classifies required-vs-optional load failures; an optional shutdown observer fires from both `Extension::close` and `Drop` for symmetry.

`hilavitkutin-extensions-macros` is the proc-macro companion. `#[export_extension]` on a struct that implements the relevant capability traits emits the `#[repr(C)]` descriptor constant, the `__hilavitkutin_extension_descriptor` exported function, the capability table, and per-capability trampolines. The single attribute is the entire surface; downstream consumer ecosystems can wrap it under domain-named attributes (for example `#[viola::plugin]`) without cooperation from this crate.

The architectural invariant across the layer: any extension loads, runs, and drops at arbitrary points independent of siblings. No global registry, no init-ordering gate, no ecosystem-wide lifecycle. Consumer hosts that need ecosystem-wide semantics build them in their own layer above.

```rust
use hilavitkutin_extensions::CapabilityId;
use hilavitkutin_extensions_macros::export_extension;
use notko::Outcome;

// downstream consumer crate ships this contract for its plugin ecosystem.
pub trait Compressor {
    fn compress(&self, input: Bytes) -> Outcome<Bytes, Oops>;
}
pub const CAP_COMPRESSOR: CapabilityId = CapabilityId::from_name(b"my_app.compressor.v1");

// extension author writes the impl plus one attribute.
#[export_extension(name = "gzip")]
struct Gzip;

impl Compressor for Gzip {
    fn compress(&self, input: Bytes) -> Outcome<Bytes, Oops> {
        // gzip logic here
        Outcome::Ok(input)
    }
}
```

## Build tooling and hooks

`hilavitkutin-build` runs at build time only; its code never links into the produced runtime binary. Integration is one `configure().run()` call from a `build.rs`. The bootstrap writes a generated `target/hilavitkutin-build/hilavitkutin-config.toml` (containing the rustc-workspace-wrapper path, Cargo profile settings such as `lto = "fat"`, `codegen-units = 1`, `profile-use`, plus any flags the active pragmas require) and ensures the matching `include = "..."` line exists in `.cargo/config.toml` so Cargo picks the generated TOML up. Everything mutable lives in the generated file; `cargo clean` wipes all generated state and the next build regenerates it. The committed `.cargo/config.toml` keeps just the include directive; the bootstrap manages it on first run.

Typed pragmas drive the artefact. Each pragma names what it is and what it needs. `LoopOptimization` activates the LLVM pass plugin that runs IRCE, LoopPredication, LoopInterchange, LoopDistribute, and LoopDataPrefetch. `FastMath` flips `-C llvm-args=-enable-unsafe-fp-math` and emits the `arvo_fast_math` cfg. `MathPeephole` (which requires `FastMath`) loads the math-peephole pass plugin. `ExpandedLto` writes `lto = "fat"` + `codegen-units = 1` (required for devirtualisation of the monomorphised dispatch). Combinators (`All<(A, B)>`, `Any<(A, B)>`) declare requirements; conflicts surface at `configure().run()` time, not at build time.

`Pgo` and `Bolt` integrate as post-build hooks. The first release build runs PGO-instrumented benchmarks in the background to generate `merged.profdata`; subsequent builds pick up the profile and recompile with profile-guided inlining, followed by BOLT post-link rewriting on Linux targets. Profiles are gitignored, expire on `cargo clean`, and warn when HEAD has diverged more than 50 commits since generation.

```rust
// build.rs
use hilavitkutin_build::{configure, pragmas::*};

fn main() {
    configure()
        .profile("release", |p| p.enable::<(
            ExpandedLto,        // fat lto + codegen-units=1
            FastMath,           // unsafe-fp-math + arvo_fast_math cfg
            LoopOptimization,   // llvm pass plugin
            Pgo, Bolt,          // profile-guided + post-link rewriting
        )>())
        .run();
}
```

## Ecosystem extensions

Three standalone crates round out the ecosystem. `hilavitkutin-persistence` is the generic hot/cold storage bridge: rkyv-archived cold store on disk, SIEVE eviction over the hot store, content-hash translation for `Str` values across the persistence boundary so handles stay session-specific while disk identity stays stable. `hilavitkutin-str` is the interned string system: `Str` is `#[repr(transparent)]` over a 32-bit packed bitfield (one bit distinguishes const from runtime origin, 28 bits hold the id), const handles content-hash via FNV-1a at compile time and register through linker sections, runtime handles get sequential ids from a host-supplied `ArenaInterner` impl.

`hilavitkutin-providers` ships default Kit implementations on top of the api primitives. v0 is the `InternerKit<BYTES, ENTRIES>` plus a `default_interner()` constructor backed by an inline `MemoryArena<BYTES, ENTRIES>`; future modules add default ColumnStorage, Clock, and MemoryProvider as consumer demand surfaces. Each of the three crates is independently usable and none depend on the engine; the engine reaches for them only when registered via `.add_kit::<K>()` or direct `.resource(_)` / `.column()` calls on the scheduler builder.

```rust
use hilavitkutin_persistence::ColdStore;
use hilavitkutin_str::{Str, str_const};
use notko::Outcome;

const APP_NAME: Str = str_const!("my_app");           // fnv-1a at compile time, linker section

fn warm(data_dir: PathBytes) -> Outcome<(), Oops> {
    let cold = ColdStore::open(data_dir)?;
    cold.load(APP_NAME)?;                             // const str resolves via linker section
    Outcome::Ok(())
}
```

## Status

Design is mature across the engine, the plugin-host layer, and the standalone ecosystem crates; implementation status varies per crate. The plugin-host layer (linking + extensions + extensions-macros) is implemented. The api crate and the standalone ecosystem extensions (ctx, str, persistence, providers) are partially implemented. The engine's runtime modules (the plan analysis chain, dispatch codegen, adapt subsystem, thread pool, morsel loop, resource resolution, plan caching) are designed and not yet shipped. hilavitkutin-build runs as a build-dependency stub today; full wrapper script generation and config schema are deferred to follow-up rounds.

Public names may change; renames ship cleanly without deprecation aliases. Several pieces gate on unstable rustc features: `const_trait_impl`, `adt_const_params`, `generic_const_exprs`, `marker_trait_attr` for the trait solver work, plus `-Z config-include` for the build crate.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/hilavitkutin/blob/dev/LICENSE)
