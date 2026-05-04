**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-api (1 trait + 1 sealed module + 1 impl), hilavitkutin (1 BuilderResource impl), hilavitkutin-providers (InternerKit type + Kit impl + tests)
**Source topics:** #329 (carry-over from round 202605041257 deferral)

# Topic: hilavitkutin-providers v0.1: InternerKit + BuilderResource bridge

## Background

Round 202605041257 shipped the v0 surface for `hilavitkutin-providers`:
`InternerApi`, `HasInterner`, `MemoryArena<BYTES, ENTRIES>`, the
`StringInterner<A>` blanket impl, and the `default_interner<...>()`
constructor. v0 deliberately deferred the `InternerKit` Kit impl
because shipping it required a third trait the api crate did not yet
expose. The carry-over is task #329.

The v0 wiring story consumers see today:

```rust
let scheduler = Scheduler::builder()
    .resource(default_interner::<4096, 256>())
    .build();
```

This works. The Kit pattern would let consumers write:

```rust
let scheduler = Scheduler::builder()
    .add_kit(InternerKit::<4096, 256>)
    .build();
```

Both are equivalent. The Kit is more idiomatic when several
provider registrations bundle into one named preset (the original
`hilavitkutin-kit` crate's reason for existing, per its DESIGN). For
a single-resource preset like `InternerKit` the ergonomic gain is
small; the design reason for shipping it is consistency, so consumers
can reach for the Kit shape uniformly without learning that some
providers ship Kits and others don't.

## The bridging problem

A Kit impl declared in `hilavitkutin-providers` cannot name
`SchedulerBuilder` directly. The crate's forbidden-imports rule lists
`hilavitkutin::*` as forbidden (per
`.claude/rules/lint-forbidden-hilavitkutin-providers.md`). The rule
exists for the right reason: providers ships standalone Kit impls,
keeping the engine out of providers's dep graph keeps providers
swappable.

The `hilavitkutin-kit::Kit<B>` trait is generic over the input
builder type `B`; this much is already in place. What providers's Kit
body needs is a way to call `B::resource(builder, init_value)`
without naming `SchedulerBuilder`. Today, `SchedulerBuilder::resource`
is an inherent method on the engine type, so providers cannot reach
it.

## Proposed shape: `BuilderResource<T>` sealed trait in api

Add a fourth sealed builder-support trait next to the existing
`Buildable` / `WuSatisfied` / `BuilderExtending` / `Depth` family in
`hilavitkutin-api::builder`:

```rust
pub trait BuilderResource<T: 'static>: builder_resource_sealed::Sealed<T> {
    type WithResource;
    fn with_resource(self, init: T) -> Self::WithResource;
}
```

The engine impls it for `SchedulerBuilder` by forwarding to the
existing inherent `.resource()` method:

```rust
impl<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores, T>
    BuilderResource<T>
    for SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, Stores>
where
    T: 'static,
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<T>, Stores): AccessSet,
{
    type WithResource =
        SchedulerBuilder<MAX_UNITS, MAX_STORES, MAX_LANES, Wus, (Resource<T>, Stores)>;

    fn with_resource(self, init: T) -> Self::WithResource {
        self.resource(init)
    }
}
```

The InternerKit then reads:

```rust
pub struct InternerKit<const BYTES: usize, const ENTRIES: usize>;

impl<B, const BYTES: usize, const ENTRIES: usize>
    Kit<B> for InternerKit<BYTES, ENTRIES>
where
    B: BuilderResource<StringInterner<MemoryArena<BYTES, ENTRIES>>>,
{
    type Output = B::WithResource;

    fn install(self, builder: B) -> Self::Output {
        builder.with_resource(default_interner::<BYTES, ENTRIES>())
    }
}
```

No `SchedulerBuilder` name in providers. The Kit composes by trait
bound on `B`, which any builder type can satisfy by implementing
`BuilderResource<T>`. The engine's `add_kit` already carries
`K::Output: BuilderExtending<Self>`, which the engine impl satisfies
automatically because `with_resource` forwards to `.resource()` (the
established type-state extension).

## Why a method on the trait, not a free fn

The trait's contract is "this builder lets you register a
`Resource<T>` and tells you the resulting type". Both pieces (the
ability and the resulting type) belong on the trait so a Kit
referencing only `B: BuilderResource<T>` has both. A free fn would
have to take `B` plus the type-level proof separately, and the call
syntax `BuilderResource::with_resource(builder, init)` is not better
than `builder.with_resource(init)`.

## Why no symmetric `BuilderColumn<T>` / `BuilderVirtual<T>` yet

Same shape would generalise to columns and virtuals. v0.1 ships
`BuilderResource<T>` only. Providers's near-term Kits all register
`Resource<T>`s. When a column-registering Kit lands (a future
provider that ships a `Column<Diagnostic>` default, for instance),
the round that adds it adds the symmetric `BuilderColumn<T>` trait at
the same time. Per `no-legacy-shims-pre-1.0.md` discipline: ship what
the shipping work needs, defer what is speculative.

## Decisions to record

1. **Trait name**: `BuilderResource<T>` (parallel to `Buildable`,
   `WuSatisfied`, `BuilderExtending`, `Depth` in the same module).
2. **Method name**: `with_resource(self, init: T) -> Self::WithResource`.
   The verb `with_` matches Rust builder convention. It does not
   collide with `SchedulerBuilder::resource` (different name on a
   different surface; the engine's inherent method stays as-is for
   direct callers).
3. **Sealed**: yes, via a private supertrait module
   `builder_resource_sealed::Sealed<T>`. Single legal impl is the
   engine's; consumers cannot impl `BuilderResource` for their own
   builder types. (If a future round needs to lift the seal, it does
   so deliberately; the seal stays the default.)
4. **InternerKit naming**: `InternerKit<const BYTES: usize, const ENTRIES: usize>`.
   Matches `default_interner::<BYTES, ENTRIES>()` constructor's
   const-generic shape. Unit struct (no fields).
5. **InternerKit impl bound**: only `B: BuilderResource<StringInterner<MemoryArena<BYTES, ENTRIES>>>`.
   No engine-shape name leaks into providers.
6. **Test coverage in providers**: one smoke test that constructs the
   Kit and confirms its `install` method type-checks against a stub
   `BuilderResource` impl (no engine dep in providers tests). The
   end-to-end `Scheduler::builder().add_kit(InternerKit::<...>)` test
   lives in the engine's tests, where `SchedulerBuilder` is already
   in scope.

## What this round does NOT do

- Does not add `BuilderColumn<T>` or `BuilderVirtual<T>` (deferred
  per the v0.1-narrow-scope decision above).
- Does not add a `Default` impl on `InternerKit` (would enable
  `add_kit::<InternerKit<...>>(Default::default())`; see #299 for
  the broader `add_kit::<K: Default>()` overload). Out of scope.
- Does not change the engine's inherent `.resource()` method or the
  established `BuilderExtending` impl. The new trait sits beside
  those, not in place of them.
- Does not change the v0 `default_interner<...>()` constructor or
  any v0 surface; the InternerKit composes on top of v0.

## Out of scope (parking lot)

- `BuilderColumn<T>` / `BuilderVirtual<T>` symmetric traits: when a
  consumer Kit needs them.
- `Default` impl on `InternerKit` for the `#299` ergonomic overload.
- M-parameterised arena Kit (the `MemoryArena<M, BYTES>` over a
  runtime memory provider variant from the original 2026-05-01 task
  description): tracked in providers's BACKLOG, lands when the
  builder gains allocate-at-builder-time plumbing.
- Sync arena variant: tracked in providers's BACKLOG.
