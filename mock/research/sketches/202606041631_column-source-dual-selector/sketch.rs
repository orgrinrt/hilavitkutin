//! Sketch: unified bindings cons-list as BOTH resource source (`Selector` /
//! `Project`) AND column source (`ColSelector` / `ColProject`), Shape A of the
//! column-data-plane design (round 202606041631).
//!
//! HYPOTHESIS: a single interleaved bindings cons-list type can implement both
//! `Selector<T, I>` (resource lookup) and `ColSelector<T, I>` (column lookup),
//! with pass-through `There<I>` recursion on ALL node kinds (so a selected node
//! can sit behind any other node kind), and the dual blanket `Project` /
//! `ColProject` impls resolve their independent index lists unambiguously at a
//! concrete call site, given each store type appears once.
//!
//! This mirrors the real engine_ctx.rs Selector/ColSelector/Project/ColProject
//! shapes minimally (ptrs are usize tags so resolution is verifiable at runtime).
//!
//! Build: `rustc --edition 2021 sketch.rs -o /tmp/colsrc && /tmp/colsrc`

use core::marker::PhantomData;

// ---- access-set markers ----
struct Empty;
struct Cons<H, T>(PhantomData<(H, T)>);
struct Resource<T>(PhantomData<T>);
struct Column<T>(PhantomData<T>);
struct Virtual<T>(PhantomData<T>);

// ---- index witnesses (disjoint types: the two-impl non-overlap hinge) ----
struct Here;
struct There<I>(PhantomData<I>);

// ---- pointer stand-ins carrying a tag so resolution is checkable ----
// Copy/Clone unconditionally (no implicit T: Copy bound), mirroring the real
// repr(transparent) ResourcePtr<T>/ColumnPtr<T> NonNull wrappers.
struct RPtr<T>(usize, PhantomData<T>);
impl<T> Copy for RPtr<T> {}
impl<T> Clone for RPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}
struct CPtr<T>(usize, PhantomData<T>);
impl<T> Copy for CPtr<T> {}
impl<T> Clone for CPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// ---- interleaved bindings cons-list ----
struct BNil;
struct RBind<T, Tail> {
    ptr: RPtr<T>,
    tail: Tail,
}
struct CBind<T, Tail> {
    ptr: CPtr<T>,
    tail: Tail,
}
struct VBind<T, Tail> {
    tail: Tail,
    _m: PhantomData<T>,
}

// ---- Selector (resource lookup): Here on RBind<T>; There pass-through on ALL ----
trait Selector<T, I> {
    fn get(&self) -> RPtr<T>;
}
impl<T, Tail> Selector<T, Here> for RBind<T, Tail> {
    fn get(&self) -> RPtr<T> {
        self.ptr
    }
}
impl<T, U, Tail, I> Selector<T, There<I>> for RBind<U, Tail>
where
    Tail: Selector<T, I>,
{
    fn get(&self) -> RPtr<T> {
        self.tail.get()
    }
}
impl<T, U, Tail, I> Selector<T, There<I>> for CBind<U, Tail>
where
    Tail: Selector<T, I>,
{
    fn get(&self) -> RPtr<T> {
        self.tail.get()
    }
}
impl<T, U, Tail, I> Selector<T, There<I>> for VBind<U, Tail>
where
    Tail: Selector<T, I>,
{
    fn get(&self) -> RPtr<T> {
        self.tail.get()
    }
}

// ---- ColSelector (column lookup): Here on CBind<T>; There pass-through on ALL ----
trait ColSelector<T, I> {
    fn get(&self) -> CPtr<T>;
}
impl<T, Tail> ColSelector<T, Here> for CBind<T, Tail> {
    fn get(&self) -> CPtr<T> {
        self.ptr
    }
}
impl<T, U, Tail, I> ColSelector<T, There<I>> for CBind<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    fn get(&self) -> CPtr<T> {
        self.tail.get()
    }
}
impl<T, U, Tail, I> ColSelector<T, There<I>> for RBind<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    fn get(&self) -> CPtr<T> {
        self.tail.get()
    }
}
impl<T, U, Tail, I> ColSelector<T, There<I>> for VBind<U, Tail>
where
    Tail: ColSelector<T, I>,
{
    fn get(&self) -> CPtr<T> {
        self.tail.get()
    }
}

// ---- projected bundles ----
struct PtrNil;
struct PtrCons<H, Tail> {
    head: RPtr<H>,
    tail: Tail,
}
struct ColPtrNil;
struct ColPtrCons<H, Tail> {
    head: CPtr<H>,
    tail: Tail,
}

// ---- Project (resource projection over source A) — blanket on A ----
trait Project<R, Idx> {
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
impl<A, T, RTail, Idx> Project<Cons<Column<T>, RTail>, Idx> for A
where
    A: Project<RTail, Idx>,
{
    type Out = <A as Project<RTail, Idx>>::Out;
    fn project(&self) -> Self::Out {
        <A as Project<RTail, Idx>>::project(self)
    }
}
impl<A, T, RTail, Idx> Project<Cons<Virtual<T>, RTail>, Idx> for A
where
    A: Project<RTail, Idx>,
{
    type Out = <A as Project<RTail, Idx>>::Out;
    fn project(&self) -> Self::Out {
        <A as Project<RTail, Idx>>::project(self)
    }
}

// ---- ColProject (column projection over source C) — blanket on C ----
trait ColProject<Set, Idx> {
    type Out;
    fn col_project(&self) -> Self::Out;
}
impl<C> ColProject<Empty, Empty> for C {
    type Out = ColPtrNil;
    fn col_project(&self) -> ColPtrNil {
        ColPtrNil
    }
}
impl<C, T, I, STail, ITail> ColProject<Cons<Column<T>, STail>, Cons<I, ITail>> for C
where
    C: ColSelector<T, I>,
    C: ColProject<STail, ITail>,
{
    type Out = ColPtrCons<T, <C as ColProject<STail, ITail>>::Out>;
    fn col_project(&self) -> Self::Out {
        ColPtrCons {
            head: <C as ColSelector<T, I>>::get(self),
            tail: <C as ColProject<STail, ITail>>::col_project(self),
        }
    }
}
impl<C, T, STail, Idx> ColProject<Cons<Resource<T>, STail>, Idx> for C
where
    C: ColProject<STail, Idx>,
{
    type Out = <C as ColProject<STail, Idx>>::Out;
    fn col_project(&self) -> Self::Out {
        <C as ColProject<STail, Idx>>::col_project(self)
    }
}
impl<C, T, STail, Idx> ColProject<Cons<Virtual<T>, STail>, Idx> for C
where
    C: ColProject<STail, Idx>,
{
    type Out = <C as ColProject<STail, Idx>>::Out;
    fn col_project(&self) -> Self::Out {
        <C as ColProject<STail, Idx>>::col_project(self)
    }
}

// ---- the dispatch-shim call shape: ONE source object, all three projections ----
// Mirrors fiber_shim's lifted bounds: A is both resource source and column source.
fn project_both<A, R, W, RIdx, RCIdx, WCIdx>(
    a: &A,
) -> (
    <A as Project<R, RIdx>>::Out,
    <A as ColProject<R, RCIdx>>::Out,
    <A as ColProject<W, WCIdx>>::Out,
)
where
    A: Project<R, RIdx>,
    A: ColProject<R, RCIdx>,
    A: ColProject<W, WCIdx>,
{
    // fully-qualified, mirroring engine_ctx.rs EngineCtx::project: method-call
    // syntax `a.col_project()` is ambiguous between the R and W projections.
    (
        <A as Project<R, RIdx>>::project(a),
        <A as ColProject<R, RCIdx>>::col_project(a),
        <A as ColProject<W, WCIdx>>::col_project(a),
    )
}

// ---- distinct store types (each appears once, per the access-set invariant) ----
struct R1;
struct R2;
struct C1;
struct C2;

fn main() {
    // Interleaved bindings: RBind<R1> -> CBind<C1> -> RBind<R2> -> CBind<C2> -> BNil.
    // R2 sits behind a CBind (exercises Selector pass-through over a column node);
    // C1 sits behind an RBind (exercises ColSelector pass-through over a resource node).
    let bindings = RBind::<R1, _> {
        ptr: RPtr(11, PhantomData),
        tail: CBind::<C1, _> {
            ptr: CPtr(21, PhantomData),
            tail: RBind::<R2, _> {
                ptr: RPtr(12, PhantomData),
                tail: CBind::<C2, _> {
                    ptr: CPtr(22, PhantomData),
                    tail: BNil,
                },
            },
        },
    };

    // Read set R = [Resource<R2>, Column<C1>]; write set W = [Column<C2>].
    // Indices are inferred (RIdx/RCIdx/WCIdx all left to the solver).
    type R = Cons<Resource<R2>, Cons<Column<C1>, Empty>>;
    type W = Cons<Column<C2>, Empty>;

    let (reads, read_cols, write_cols) = project_both::<_, R, W, _, _, _>(&bindings);

    // reads = PtrCons<R2, PtrNil> (the Column<C1> in R contributes no resource node)
    assert_eq!(reads.head.0, 12, "resource R2 must resolve to tag 12 through the CBind pass-through");
    // read_cols = ColPtrCons<C1, ColPtrNil> (the Resource<R2> contributes no column node)
    assert_eq!(read_cols.head.0, 21, "column C1 must resolve to tag 21 through the RBind pass-through");
    // write_cols = ColPtrCons<C2, ColPtrNil>
    assert_eq!(write_cols.head.0, 22, "column C2 must resolve to tag 22");

    // confirm the tails are the empty leaves (structural shape correct)
    let _: PtrNil = reads.tail;
    let _: ColPtrNil = read_cols.tail;
    let _: ColPtrNil = write_cols.tail;

    println!("WORKS: dual Selector+ColSelector over interleaved bindings resolves unambiguously (R2=12, C1=21, C2=22)");
}
