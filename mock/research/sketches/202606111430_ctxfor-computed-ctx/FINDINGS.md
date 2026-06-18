# FINDINGS: computed per-WU Context type (`CtxFor`)

## HYPOTHESIS

The six derived `EngineCtx` parameters (RBundle, RCols, WCols, WAccum, WVirt,
MP) are pure type functions of a WorkUnit's Read / Write access sets and its
schedule. An engine-side type-level map (disjoint impls per access-set head
kind, the same kind dispatch the shipped `Project` / `ColProject` /
`AccumProject` / `VirtualProject` traits already use; no specialization
anywhere) can therefore compute the whole nine-parameter type, so a consumer
writes `type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, Sched>`
instead of hand-spelling the bundles. The dispatch-side `RunFiber`
projection-equality bound must unify with the computed form (both sides
normalize to the same concrete cons chains), proven by real `Scheduler::run`
passes over column, virtual-firing, and meta-bridge DAGs.

## OUTCOME

WORKS

All three probe layers pass, dev and release, on `nightly-2026-05-28`:

1. Type-identity assertions (`assert_same::<A, B>()` compiles only when A and
   B are literally the same type, generic over the frame lifetime): `CtxFor`
   output equals the hand-spelled `EngineCtx` aliases across all four store
   kinds, interleaved set orders, empty sets, and all three schedule kinds
   (`Always` / `On<V>` keying MP to `MetaNil`, `OnMeta<V>` keying MP to
   `MetaRef<'f>`).
2. Self-referential consumer spelling normalizes: `Self::Read` / `Self::Write`
   inside the very `impl WorkUnit` being defined, and
   `<Self as HasSchedule>::Sched` resolved BOTH through an explicit
   `HasSchedule` impl and through the blanket
   `impl<W: WorkUnit<Always>> HasSchedule for W` (the candidate normalization
   cycle: the blanket impl is gated on the trait whose `Ctx` GAT is being
   defined). No cycle; the solver normalizes it.
3. Real dispatch over `Scheduler::run`, every WU's Ctx computed, asserting
   data round-trips (not just compilation):
   scenario A, resource + column RAW chain (producer seeds from
   `Resource<InA>`, reader observes 100..104 and writes a second column);
   scenario B, write-virtual bundle + gating (`On<Tick>` consumer ran,
   `On<Never>` consumer skipped, mirroring `tests/gate2_virtual_firing.rs`);
   scenario C, accumulator bundle + OnMeta meta bridge (the computed `EndWu`
   Ctx comes out with MP = `MetaRef<'frame>`, so `ctx.meta::<SchedulerMetrics>()`
   exists and reads engine-owned `pass_count` 1 then 2 across two frames,
   mirroring `tests/gate2_meta_metrics.rs`).

The `RunFiber` projection-equality bounds normalize against the computed form
with zero changes to the engine or the api: the probe runs on the GATE-2
branch crates as-is. The only fix against the cut-off draft was removing an
illegal explicit-lifetime turbofish on a late-bound probe fn (E0794), a
sketch-local call-site slip, not a finding.

## The working shape

`CtxFor` lives ENGINE-SIDE (`hilavitkutin::dispatch::engine_ctx` in the real
change). It cannot live in `hilavitkutin-api`: every output it names
(`EngineCtx`, `PtrCons`/`PtrNil`, `ColPtrCons`/`ColPtrNil`,
`AccPtrCons`/`AccPtrNil`, `VirtCons`/`VirtNil`, `MetaRef`/`MetaNil`, and the
`MetaPtrFor` keying trait) is an engine type defined in
`mock/crates/hilavitkutin/src/dispatch/engine_ctx.rs`, and the api must not
depend on the engine. The api side needs no change: `WorkUnit::Ctx` is already
an unconstrained GAT, so a consumer assigning an engine alias to it is the
existing contract.

Four new public engine traits, each a pure fold over the access set keyed on
the head's store kind. `Cons<Resource<T>, Tail>` / `Cons<Column<T>, Tail>` /
`Cons<Accum<T>, Tail>` / `Cons<Virtual<T>, Tail>` are four distinct concrete
type constructors, so the four impls per trait are disjoint and coherent with
no specialization, the same dispatch the shipped projection traits use. The
contributing kind conses its bundle node; the other three pass the tail
through, so output order is the kind-filtered subsequence of set order, which
is exactly the order `Project` / `ColProject` / `AccumProject` /
`VirtualProject` build the runtime values in. The computed type and the
projected value type agree node for node by construction.

```rust
// engine-side, next to EngineCtx
pub trait ResourceBundleOf { type Out; }          // -> PtrCons chain,    PtrNil leaf
pub trait ColBundleOf { type Out; }               // -> ColPtrCons chain, ColPtrNil leaf
pub trait AccumBundleOf<'frame> { type Out; }     // -> AccPtrCons chain, AccPtrNil leaf
pub trait VirtBundleOf<'frame> { type Out; }      // -> VirtCons chain,   VirtNil leaf

pub type CtxFor<'frame, R, W, S = Always> = EngineCtx<
    'frame,
    R,
    W,
    <R as ResourceBundleOf>::Out,
    <R as ColBundleOf>::Out,
    <W as ColBundleOf>::Out,
    <W as AccumBundleOf<'frame>>::Out,
    <W as VirtBundleOf<'frame>>::Out,
    <S as MetaPtrFor<'frame>>::Ptr,   // shipped trait, reused as-is
>;
```

`AccumBundleOf` and `VirtBundleOf` carry `'frame` because their cons nodes are
lifetime-bearing (the runtime bundles borrow the bindings' cells), the same
way `AccumProject<'s, ...>` already does. MP keys off the schedule via the
shipped `MetaPtrFor`: `MetaNil` for `Always` and `On<V>`, `MetaRef<'frame>`
for `OnMeta<V>`. `S` defaults to `Always`, mirroring the
`WorkUnit<Schedule = Always>` default.

## Post-change consumer spelling

```rust
impl WorkUnit<Always> for MyWu {
    type Read = Cons<Resource<Cfg>, Cons<Column<In1>, Empty>>;
    type Write = Cons<Column<Out1>, Empty>;
    type Hint = ...;
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) { ... }
}

// non-Always schedules name the schedule, directly or via HasSchedule
type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, On<Tick>>;
type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, <Self as HasSchedule>::Sched>;
```

This replaces the hand-spelled nine-parameter aliases of the shape at
`tests/gate2_meta_metrics.rs:90` (`ConsumerCtx`) and `:95` (`EndCtx`); the
probe's `HandMix` / `HandFlip` / `HandMeta` aliases reproduce that shape and
the identity assertions pin the equivalence.

## Caveats observed

The probe covers sets up to three members and the three schedule kinds; the
fold is structurally uniform per node, so longer sets add solver steps, not
new shapes. Duplicate-member or otherwise malformed access sets were not
probed; those are rejected upstream by the existing `AccessSet` machinery,
not by `CtxFor`. A consumer can still hand-spell `EngineCtx` if it wants;
`CtxFor` is an alias layer, not a contract change, so adoption is mechanical
and incremental (no api break, no engine dispatch change).

## Next step unblocked

A small engine round: add the four fold traits + the `CtxFor` alias to
`dispatch/engine_ctx.rs`, re-export alongside `EngineCtx`, and mechanically
migrate the in-repo test/bench WUs off hand-spelled aliases. No api round
needed.
