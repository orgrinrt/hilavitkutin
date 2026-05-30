//! Sketch (HILA-RUNTIME C2 / #340): run_fiber WuTuple walk with per-WU EngineCtx.
//!
//! Hypothesis: a monomorphic recursive walk over a fiber's typed WU
//! list can construct EACH WU's own projected Context from a SHARED
//! arena, and call `wu.execute(&ctx)`, type-checking under
//! nightly-2026-05-28.
//!
//! The single-WU construct-and-execute is ALREADY proven in shipping
//! source (`tests/engine_ctx.rs::context_drives_wu_execute`): there `W`
//! is concrete, so `W::Ctx<'_>` resolves to a concrete `EngineCtx` and
//! `EngineCtx::project` infers its generics to match. The NEW question
//! is the RECURSIVE WALK over a HETEROGENEOUS list, where `W` is
//! abstract: each element has a distinct Read set, hence a distinct
//! projection index and a distinct `EngineCtx` bundle type, hence a
//! distinct `W::Ctx<'frame>` GAT instantiation. To call `execute` the
//! walk must tell the solver that each abstract `W`'s Ctx GAT equals
//! the projection of its set over the shared arena. That tie is an
//! HRTB associated-type-equality bound on a GAT:
//!
//!   for<'f> W: WorkUnit<Ctx<'f> = EngineCtx<'f, W::Read, Bundle>>
//!
//! where `Bundle = <A as Project<W::Read, RIdx>>::Out` is
//! lifetime-independent. Whether rustc accepts and USES this bound is
//! the crux. If it works, the C2 `run_fiber` slice is pure Rust
//! generics (no codegen, no LLVM), matching the architect's framing.
//!
//! Faithfulness: this models the real `Project` / `Selector` index
//! witness, the GAT `Ctx`, and the projecting constructor closely
//! enough that a WORKS result transfers to the engine. Columns are
//! omitted (resource-only): they add more `Project`/`Selector`
//! recursion of the same shape, not a new trait-solver question. WU
//! VALUES are carried in the walk list to isolate the trait-solver
//! crux; the engine sources WU values from the registered bundle,
//! which is a separate slice (slice 2). Stand-in types; no substrate
//! dep, per the sketch convention.
//!
//! Outcome recorded at the bottom of this file.

#![allow(dead_code)]

use std::cell::RefCell;
use std::marker::PhantomData;

// ---------------------------------------------------------------------
// Type-level cons-list for ACCESS SETS (markers only, no values),
// mirroring api `access::{Cons, Empty}`.
// ---------------------------------------------------------------------
struct Empty;
struct Cons<H, T>(PhantomData<(H, T)>);

// Value-carrying list for the FIBER's WU sequence. Distinct from the
// access-set cons because these nodes carry WU instances.
struct FiberEnd;
struct Fiber<W, T> {
    head: W,
    tail: T,
}

// ---------------------------------------------------------------------
// Peano index witnesses (mirror engine_ctx `Here` / `There`).
// ---------------------------------------------------------------------
struct Here;
struct There<I>(PhantomData<I>);

// ---------------------------------------------------------------------
// Store marker + resource pointer + arena node chain (mirror the
// engine shapes).
// ---------------------------------------------------------------------
struct Resource<T>(PhantomData<T>);

struct ResourcePtr<T>(*const T);
// A raw pointer is Copy regardless of `T`; hand-write the impls so the
// derive does not graft an implicit `T: Copy` bound (mirrors the
// engine's `ResourcePtr`, whose payloads are not themselves Copy).
impl<T> Clone for ResourcePtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for ResourcePtr<T> {}
impl<T> ResourcePtr<T> {
    fn as_ptr(self) -> *const T {
        self.0
    }
}

struct ArenaTail;
struct ArenaResourceNode<T, Tail> {
    ptr: ResourcePtr<T>,
    tail: Tail,
}

// Projected resource bundle (mirror engine_ctx `PtrCons` / `PtrNil`).
struct PtrNil;
struct PtrCons<H, Tail> {
    head: ResourcePtr<H>,
    tail: Tail,
}

// ---------------------------------------------------------------------
// Selector: type-keyed lookup over arena nodes and over the projected
// bundle (mirror engine_ctx).
// ---------------------------------------------------------------------
trait Selector<T, Index> {
    fn get(&self) -> ResourcePtr<T>;
}

impl<T, Tail> Selector<T, Here> for ArenaResourceNode<T, Tail> {
    fn get(&self) -> ResourcePtr<T> {
        self.ptr
    }
}
impl<T, U, Tail, I> Selector<T, There<I>> for ArenaResourceNode<U, Tail>
where
    Tail: Selector<T, I>,
{
    fn get(&self) -> ResourcePtr<T> {
        self.tail.get()
    }
}

impl<T, Tail> Selector<T, Here> for PtrCons<T, Tail> {
    fn get(&self) -> ResourcePtr<T> {
        self.head
    }
}
impl<T, U, Tail, I> Selector<T, There<I>> for PtrCons<U, Tail>
where
    Tail: Selector<T, I>,
{
    fn get(&self) -> ResourcePtr<T> {
        self.tail.get()
    }
}

// ---------------------------------------------------------------------
// Project: arena -> resource bundle for an access set (mirror
// engine_ctx). `Indices` infers at the call site.
// ---------------------------------------------------------------------
trait Project<R, Indices> {
    type Out;
    fn project(&self) -> Self::Out;
}

impl<A> Project<Empty, Empty> for A {
    type Out = PtrNil;
    fn project(&self) -> PtrNil {
        PtrNil
    }
}

impl<A, T, I, RTail, ITail> Project<Cons<Resource<T>, RTail>, Cons<I, ITail>> for A
where
    A: Selector<T, I>,
    A: Project<RTail, ITail>,
{
    type Out = PtrCons<T, <A as Project<RTail, ITail>>::Out>;
    fn project(&self) -> Self::Out {
        PtrCons {
            head: <A as Selector<T, I>>::get(self),
            tail: <A as Project<RTail, ITail>>::project(self),
        }
    }
}

// ---------------------------------------------------------------------
// EngineCtx (resource-only model) + the projecting constructor as a
// free fn (clean inference vs a method whose Self generics need pins).
// ---------------------------------------------------------------------
struct EngineCtx<'frame, R, RBundle> {
    reads: RBundle,
    _frame: PhantomData<&'frame ()>,
    _r: PhantomData<R>,
}

impl<'frame, R, RBundle> EngineCtx<'frame, R, RBundle> {
    fn resource<T, I>(&self) -> &T
    where
        RBundle: Selector<T, I>,
    {
        let ptr = <RBundle as Selector<T, I>>::get(&self.reads);
        // SAFETY: sketch arena values outlive the walk (stack locals in
        // `main` that the fiber walk borrows); the pointer is non-null
        // and aligned. Mirrors the engine's `'frame`-tied read.
        unsafe { &*ptr.as_ptr() }
    }
}

fn project_ctx<'frame, A, R, RIdx>(
    arena: &'frame A,
) -> EngineCtx<'frame, R, <A as Project<R, RIdx>>::Out>
where
    A: Project<R, RIdx>,
{
    EngineCtx {
        reads: arena.project(),
        _frame: PhantomData,
        _r: PhantomData,
    }
}

// ---------------------------------------------------------------------
// WorkUnit contract (GAT Ctx), mirror of the api shape.
// ---------------------------------------------------------------------
trait WorkUnit {
    type Read;
    type Ctx<'frame>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>);
}

// ---------------------------------------------------------------------
// THE CRUX: the recursive fiber walk. Per element, build the WU's own
// projected EngineCtx and call execute. `RIdx` (the per-WU projection
// index list) and the bundle are solver-inferred per element.
// ---------------------------------------------------------------------
// `Witnesses` is the parallel per-element projection-index list (one
// `RIdx` per WU, each itself a per-member index list). Carrying it as a
// trait parameter constrains each `RIdx` (dodging E0207), exactly as the
// engine's `BundleProject<Stores, Witnesses, ...>` does. The whole
// nested list infers at the entry call.
trait RunFiber<A, Witnesses> {
    fn run(&self, arena: &A);
}

impl<A> RunFiber<A, Empty> for FiberEnd {
    fn run(&self, _arena: &A) {}
}

impl<A, W, Tail, RIdx, WTail> RunFiber<A, Cons<RIdx, WTail>> for Fiber<W, Tail>
where
    W: WorkUnit,
    A: Project<W::Read, RIdx>,
    // Tie each WU's Ctx GAT to the projection of its Read set over the
    // shared arena, for all frame lifetimes. The bundle is
    // lifetime-independent, so the equality holds definitionally from
    // the WU's own `type Ctx<'frame> = EngineCtx<'frame, Read, Bundle>`.
    for<'f> W: WorkUnit<Ctx<'f> = EngineCtx<'f, <W as WorkUnit>::Read, <A as Project<<W as WorkUnit>::Read, RIdx>>::Out>>,
    Tail: RunFiber<A, WTail>,
{
    fn run(&self, arena: &A) {
        let ctx: <W as WorkUnit>::Ctx<'_> = project_ctx::<A, W::Read, RIdx>(arena);
        self.head.execute(&ctx);
        self.tail.run(arena);
    }
}

/// Entry point: drive a fiber's WU sequence over the arena. The
/// `Witnesses` index list infers from the `RunFiber` bound, exactly as
/// `project_reads::<R, _, _>` infers its selector indices.
fn run_fiber<F, A, Witnesses>(fiber: &F, arena: &A)
where
    F: RunFiber<A, Witnesses>,
{
    fiber.run(arena);
}

// ---------------------------------------------------------------------
// Scenario: two WUs reading DISTINCT resources from a shared 2-node
// arena. W0 reads RA at arena index Here; W1 reads RB at There<Here>.
// Distinct Read sets => distinct Ctx GAT => distinct projection index
// per element, the heterogeneity the crux is about.
// ---------------------------------------------------------------------
struct RA(u32);
struct RB(u32);

struct W0;
impl WorkUnit for W0 {
    type Read = Cons<Resource<RA>, Empty>;
    type Ctx<'frame> = EngineCtx<'frame, Cons<Resource<RA>, Empty>, PtrCons<RA, PtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let v: &RA = ctx.resource::<RA, _>();
        OBSERVED.with(|o| o.borrow_mut().push(v.0));
    }
}

struct W1;
impl WorkUnit for W1 {
    type Read = Cons<Resource<RB>, Empty>;
    type Ctx<'frame> = EngineCtx<'frame, Cons<Resource<RB>, Empty>, PtrCons<RB, PtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let v: &RB = ctx.resource::<RB, _>();
        OBSERVED.with(|o| o.borrow_mut().push(v.0));
    }
}

thread_local! {
    static OBSERVED: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

fn main() {
    let a = RA(10);
    let b = RB(20);
    // Arena: RA at Here, RB at There<Here>.
    let arena = ArenaResourceNode {
        ptr: ResourcePtr(&a as *const RA),
        tail: ArenaResourceNode {
            ptr: ResourcePtr(&b as *const RB),
            tail: ArenaTail,
        },
    };
    // Fiber WU sequence: [W0, W1].
    let fiber = Fiber {
        head: W0,
        tail: Fiber {
            head: W1,
            tail: FiberEnd,
        },
    };

    run_fiber(&fiber, &arena);

    OBSERVED.with(|o| {
        let got = o.borrow().clone();
        assert_eq!(
            got,
            vec![10, 20],
            "both WUs ran in order; each saw its OWN projected resource through its own Ctx"
        );
    });
    println!("WORKS: heterogeneous WuTuple walk with per-WU projected EngineCtx + execute");
}

// OUTCOME: WORKS (nightly-2026-05-28, rustc 1.98.0-nightly 57d06900f).
// Compiled clean and ran; the assert passed (`[10, 20]`): both WUs ran
// in order and each resolved its OWN projected resource through its OWN
// `Ctx<'frame>`. No nightly feature needed beyond stable GATs (no
// specialization, no generic_const_exprs, no impl_trait_in_assoc_type).
//
// The load-bearing bound that the solver accepted and USED:
//
//   for<'f> W: WorkUnit<Ctx<'f> = EngineCtx<'f, W::Read, <A as Project<W::Read, RIdx>>::Out>>
//
// HRTB associated-type-equality on a GAT, with the right side a
// projected associated type that is lifetime-independent. rustc resolves
// it against each concrete WU's own `type Ctx<'frame> = EngineCtx<'frame,
// Read, Bundle>` declaration. The `Witnesses` parallel-index list
// (mirroring the engine's `BundleProject<Stores, Witnesses, ...>`)
// constrains each per-WU `RIdx`, dodging E0207, and infers at the entry
// call (`run_fiber(&fiber, &arena)`) with no turbofish.
//
// => C2 slice 1 (`RunFiber` walk over a fiber's WU sequence, per-WU
// EngineCtx::project + execute) is feasible as pure Rust generics, no
// codegen / LLVM, matching the architect's framing. The engine slice
// reuses the shipped `Project` / `Selector` / `EngineCtx` directly; only
// the `RunFiber<A, Witnesses>` walk trait + entry fn are new. WU value
// sourcing (the engine holds WUs; here values are carried in the list)
// is the separate slice-2 question and is NOT answered by this sketch.
