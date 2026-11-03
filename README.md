# `hilavitkutin`

> A static scheduler and runtime that turns declared read and write access sets into per-core dispatch with cache-aware morsels. `no_std`, no alloc, no spawn, no dyn.

Work is written as typed WorkUnits that declare what they read and write, and they are registered on a scheduler builder with one method, `.with(input)`. A WorkUnit goes onto the unit side of the builder's typestate and a `Column`, `Resource`, `Virtual` or `Kit` registers a store. At `.build()` every declared access is type-checked against the registered stores, so a WorkUnit that names a store the builder doesn't have fails to compile, with the diagnostic pointing at the missing marker.

```rust
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::column::Column;

let scheduler = Scheduler::builder()
    .with(Column::<Inv>::new())
    .with(Column::<Outv>::new())
    .with(Copyer)
    .build(store(provider), USize(4))
    .unwrap_or_else(|_| panic!("build should succeed"));
```

Building also runs the plan stage. Access-set overlap gives the dependency graph, the critical path and the phases are worked out from it, and adjacent work is reordered and grouped for cache locality into trunks of fibers with morsels sized to the cache. The result is a monomorphised dispatch program per physical core, which drives a pre-allocated thread pool at run time, and phase N+1 starts as soon as phase N has produced enough records for one morsel. Plans are recomputed only when the pipeline's structure changes, and an adapt stage watches per-morsel timings to revise sizes and assignments between runs.

## Crates

The engine is `hilavitkutin`, with `hilavitkutin-api` holding the traits it is built from. `hilavitkutin-ctx` is the provider context framework behind every WorkUnit's `Self::Ctx`, and it stands alone with no dependencies, so any crate that wants typed access to a tuple of providers can use it. `hilavitkutin-linking`, `hilavitkutin-extensions` and `hilavitkutin-extensions-macros` make up the plugin host, where `#[export_extension]` on a struct emits the descriptor an extension is discovered by, and every extension loads and drops independently of the others.

`hilavitkutin-build` is a build-time crate that is called from a `build.rs` and never links into the produced binary. It turns typed pragmas such as `ExpandedLto`, `FastMath`, `LoopOptimization`, `Pgo` and `Bolt` into the compiler flags and post-build steps they need. `hilavitkutin-persistence` is a hot and cold storage bridge, `hilavitkutin-providers` has default kits, and `hilavitkutin-str`, `hilavitkutin-sym` and `hilavitkutin-kit` cover interned strings, symbols and the kit bundle.

## Status

The design is further along than the code, and implementation varies per crate. The plugin host is implemented, and names will change without deprecation aliases. It needs a nightly toolchain, for `const_trait_impl`, `adt_const_params`, `generic_const_exprs`, `marker_trait_attr` and `-Z config-include`. `cargo +nightly test --workspace` runs the suites, and the atomic-ordering paths are checked under MIRI as described in [`MIRI.md`](./MIRI.md).

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/hilavitkutin/blob/dev/LICENSE)
