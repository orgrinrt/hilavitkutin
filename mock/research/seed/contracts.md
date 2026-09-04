# Contracts: WorkUnit, Context, Virtuals

The consumer-facing execution contract is small: declare what you read, what
you write, how you want to be scheduled, and a body. Everything else (DAG,
ordering, phases, trunks, fibers, morsel sizes, strategy, codegen, sync) is
derived.

## The WorkUnit trait

```rust
pub trait WorkUnit<Schedule = Always>: BuilderInput<Init = Self> + Send + Sync + 'static {
    type Read: AccessSet;
    type Write: AccessSet;
    type Hint: SchedulingHint;
    type Ctx<'frame>: /* provider-tuple bounds over Read + Write */;
    const COMMUTATIVE: Bool = Bool::FALSE;
    fn execute(&self, ctx: &Self::Ctx<'frame>);
}
```

This is the current canonical shape as amended through the locked GATE-2
rounds: the founding spec's `NAME` const dropped (the monomorphised type is
the identity), the hint default dropped (the implementing tuple is
marker-specific), `COMMUTATIVE` is arvo `Bool` with a false default, and
the context is the GAT `Ctx<'frame>` the poolframe-lifetime sketch proved,
not a concrete `Context<R, W>`. Of these, only the `Ctx` spelling carries a
registered ruling (A1-6); the `NAME` drop, the hint-default drop, and the
`COMMUTATIVE` default are the shipped shape from the GATE-2 round material
(tier 4 under A2-1), carried here as the design of record with their tier-2
registration owed as a registry row at drain time.

Schedule conditions are `Always` (unconditional, the default) and `On<V>`
(runs when virtual V fires); an `After<U>` condition was dropped in the
founding design in favour of Virtual plus `On<V>`. The meta pipeline adds
`OnMeta` for lifecycle virtuals (see [[scheduler]]). Scheduling hints carry
three axes: Urgency, Divisibility (Atomic runs all morsels at once,
Interruptible one at a time with steal opportunities, Adaptive by EMA), and
Significance. Write implies Read. The authoring guideline is at most eight
unique columns per WU (register pressure, a guideline not a constraint).

The canonical consumer spelling reduces to `Read`, `Write`, `Hint`, and the
schedule; the full engine context type is computed by an api-side type
function (`CtxFor<'frame, R, W, Sched>`) rather than hand-spelled per WU
(A1-6, feasibility proven and shipped). `BuilderInput` and schedule impls
stay explicit.

## Context access rules

Two facts here are load-bearing correctness, not style:

**The `&self` receiver on Context.** Context methods take `&self`, never
`&mut self`. A `&mut self` receiver gets LLVM noalias metadata, which lets
the optimiser reorder column writes from one WU past reads from the next WU
in the same fused fiber, producing wrong results on every record (verified in
the founding bench). Column writes go through raw pointers, which carry no
aliasing assumptions and execute in program order. Violating this anywhere in
the Context API breaks every fused fiber.

**Raw pointers, not slices.** Column access is
`unsafe fn read<T>(&self, i) -> T` and `unsafe fn write<T>(&self, i, val)`
over raw pointers; there are no slice-returning methods, because slices
assert noalias that fused access violates. Resource access goes through
pointers to external storage per [[storage]]. Op registration surfaces
(`ctx.each()`, `ctx.batch()`, `ctx.reduce()`) layer on top.

The inner-loop contract: the op author controls the inner loop, the
framework controls morsel boundaries. The morsel dispatch loop is
`#[inline(always)]`; the framework constrains inputs to ColumnValue types
with type-native stride, contiguous storage, and non-aliasing Read/Write
sets. Type-keyed projection out of the heterogeneous access structures uses
inferred index witnesses (the `Here`/`There<I>` and `Selector`/`Locate`
families), never type-equality specialization (A1 constraint note 2).

## Registration

WorkUnits are manually implementable; there is no registration attribute in
hilavitkutin itself, and linker-magic registration (inventory, ctor,
distributed slices, init_array) is banned in every hilavitkutin crate.
Consumers hand WUs to the scheduler through the explicit builder API
([[scheduler]]); consumer ecosystems may build their own registration sugar
on top. The Read/Write declarations are the dependency injection: ECS-as-DI.

Registration order currently carries a provisional constraint: the builder
appends WUs to the carrier in registration order, and `build()` validates
that order is producer-before-consumer (a `BuildError` names the
RCM-recommended permutation otherwise). Op accepted this as a temporary
compromise (r2 decision b), explicitly to be relaxed: the guarded-walk
auto-RCM mechanism is sketch-proven (each WU carries a const RCM position;
the walk executes by position with const-folded guards, devirt-free), and
landing the RCM row-order work dissolves the constraint. It must never be
presented to consumers as permanent.

## The virtual flag system

Virtuals are pure flags, never data carriers; data travels in a Column or
Resource alongside the Virtual. Fire is `ctx.fire::<T>()`: one byte load,
one OR, one byte store, with the type's index const-folded from compile-time
identity.

Storage is bit-packed: one bit per registered virtual per gating consumer,
packed into bytes and grouped into words (arvo-bitmask types), with
hierarchical zero-tests to skip cold words and bytes. Bit assignment is
affinity-ordered at plan time by two-level greedy bin-packing, so co-firing
virtuals share a byte or word and connector-heavy passes keep cold words at
zero. For a typical population the whole flag store is one cache line.

Per-(virtual, consumer) tracking: each consumer has its own bit for every
virtual it gates on; a fire sets all consumer bits for that virtual; a bit
clears on first dispatch (clear-on-dispatch, not on completion), so a re-fire
during execution runs the consumer again next pass and no fire is lost.

Reset is epoch-based: a global epoch counter increments per pass, "flag set"
means the flag value equals the current epoch, so reset costs one increment
rather than a memset, with a full clear every 256 passes at wraparound.
Cross-thread firing uses a relaxed atomic store; internal firing is
non-atomic.

Signal-ordered execution (record reordering by signal state for branch-free
loops) exists in the founding record as theory only, never bench-validated;
it is not part of the committed design.
