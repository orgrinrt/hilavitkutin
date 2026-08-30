# Scheduler: Builder, Meta Pipeline, Surface

The scheduler is the integration point: consumers compose a pipeline through
its builder, the engine schedules itself through its own meta pipeline, and
the operational surface (errors, persistence, introspection, extensions)
hangs off it.

## Builder API

```rust
Scheduler::builder()
    .add::<MyWU>()               // registers any WorkUnit<S>
    .resource::<T>(initial)      // singleton with initial value
    .resource_default::<T>()     // singleton where T: Default
    .column::<T>()               // column registration
    .memory(my_provider)         // override MemoryProvider
    .threads(my_pool)            // override ThreadPool
    .clock(my_clock)             // override Clock
    .build()                     // -> Scheduler, or BuildError
```

One `.add` handles every schedule condition; `Always`, `On<V>`, and the meta
conditions live on the trait impl, not on separate builder calls. Sizing
flows through the `Capacity`/`Dim<N>` plan-dimension axes rather than raw
const-generic parameters, per the caps-are-defaults rule ([[constraints]]).
Resource initialisation takes the consumer's value; the Default overload is
convenience. Value-carrying platform providers get dedicated slots (memory,
threads, clock); a generic `.with(...)` cannot carry values (A1 constraint
note 7).

Everything registered through the builder is statically defined; the
schedule is statically analysable and computable, with no runtime WU
registration and no dynamic schedules (R6). `build()` runs cycle detection
and validates the registration sequence is producer-before-consumer,
returning a `BuildError` naming the RCM-recommended permutation on
violation (the provisional constraint recorded in [[contracts]]).

## The self-hosting meta pipeline

The scheduler is itself a pipeline. Meta work units run at lifecycle points
on the same carrier through the same const-gated walk as consumer work;
self-hosting is not a second engine.

Meta virtuals: `meta::PlanStage` (fired when the plan must recompute),
`meta::ScheduleReady` (plan-stage units complete), `meta::PassStart` (start
of each pass), `meta::ScheduleEnd` (after all consumer work). Meta
resources, restricted behind the `MetaAccess` marker: `meta::Dag`,
`meta::ExecutionPlan`, `meta::LaneAssignment`, `meta::SchedulerMetrics`. The
irreducible kernel is on the order of fifty lines: fire PlanStage, wait for
plan WUs, fire ScheduleReady, dispatch consumer WUs, fire ScheduleEnd.

Engine-owned mutable meta state lives in the scheduler's `MetaBlock` field,
a recorded and contained deviation from the uniform data model (consumer
resources are Copy read-only and cannot carry interior mutability); the
`MetaAccess`-gated accessor exists only on a meta-bearing context, enforced
at compile time (A1 constraint note 5). In parallel execution the meta
lifecycle bands run on a designated core, at parity with the single-core
bands (round-level amendment, `202606110855`).

The two PlanStage cases from [[identity]] apply here: parameter recompute is
the live path; a structural DAG change is a rebuild, never a runtime
mutation.

## Version stamps

Parallel stamp arrays (structure-of-arrays): contiguous before and current
stamps per store. Before-versions are captured before a WU runs, compared
after; the changed set is a bitmask of actually modified stores. Stamps
increment only on completion (no partial writes, [[data-model]]), and a unit
stamp view packs into one cache line.

## Error handling

`execute()` returns unit; failure aborts (R7). No Result on the hot path:
abort enables better codegen (no unwind tables, no landing pads), and the
os and no_os tiers abort on panic anyway. A WU that must signal failure
writes an error column or fires an error virtual: data flow, not control
flow.

Propagation: a failed fiber freezes its progress counter and poisons all
dependent fibers; independent fibers continue. `PipelineResult` reports
per-fiber status (Completed, Failed, Poisoned); the consumer decides whether
partial failure is acceptable. There is no result extraction from the run
call: the consumer reads its own column storage directly, and the run
returns status only.

## Persistence and incremental execution

hilavitkutin owns per-morsel generation counters and skip-unchanged
propagation; the consumer owns persistence (R2). The bridge is the
evict/dump and inject/import column APIs: evict extracts a column's data
without copies or retained references and releases the memory; inject moves
column data in. Column type registration stays static; consumers build
hot/cold storage layering (the persistence ecosystem crate) on these
primitives. On-disk state is never the engine's concern.

## Extensions and the facade pattern

The plugin-host layer ([[identity]]) loads extensions at arbitrary times;
the engine's composition stays static. The two extensibility surfaces from
[[identity]] resolve here:

New data flows through existing WUs with record count as an adaptive
parameter, needing nothing. New code integrates through a statically
registered facade WorkUnit that calls the extension through the capability
ABI at the FFI seam: one C-ABI hop per morsel batch (never per record), with
the host walk staying devirtualised and the plugin keeping its own
monomorphised internals inside its cdylib. Behind the facade the extension
runs its own static sub-engine or acts as a per-morsel pure-function
capability. The facade declares bridge stores normally, so the plan sees
honest access sets. Both feasibility questions are sketch-proven WORKS: an
opaque facade access set plans correctly through the containment bounds, and
the per-morsel ABI hop keeps the host walk free of indirect branches.

## Introspection and the operational surface

The canonical operational surface (designed; the per-fiber result type is
not yet shipped, `run` currently returns the `Out` outcome of `RunCfg`,
the run-configuration parameter of the run entry, see [[execution]]):
`PipelineResult` with dependent
poisoning; the work-stealing `Executor` extension point (deterministic
default); schedule introspection for consumer tooling; the evict/inject
persistence bridge; kits and providers as the preset layer (a `Kit` declares
`type Units: WorkUnitBundle; type Owned: StoreBundle`; the providers crate
ships defaults); and the metrics resource read through
`On<meta::ScheduleEnd>` hooks.
