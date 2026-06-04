//! Sketch: accumulator-column append surface over the unified bindings.
//!
//! HYPOTHESIS: the scheduler bindings cons-list can implement a THIRD
//! type-keyed selector, `AccumSelector` (for `Accum<T>` members), alongside
//! the landed `Selector` (resources) and `ColSelector` (columns), with
//! pass-through `There<I>` recursion on all node kinds, and resolve all three
//! unambiguously at a concrete call site. The new wrinkle vs the round-1
//! dual-selector sketch: an accumulator projection must retain a `'frame`
//! borrow of the binding (the live-length `Cell`) so the `&self` `append`
//! accessor can advance it, whereas the resource/column projections copy a
//! `Copy` pointer and retain no borrow. This sketches that the borrowed-cell
//! projection composes with the copy-pointer projections from one
//! `&'frame bindings`, and that `append` (read len, write base+len, advance
//! len) works under `&self` via `Cell` interior mutability.
//!
//! Mirrors the real engine_ctx.rs Selector/ColSelector/Project/ColProject
//! shapes minimally (ptrs are usize tags / Vec-backed so resolution and the
//! append round-trip are verifiable at runtime).
//!
//! Build: `rustc --edition 2021 sketch.rs -o /tmp/accsel && /tmp/accsel`

use core::cell::Cell;
use core::marker::PhantomData;

// ---- access-set markers ----
struct Empty;
struct Cons<H, T>(PhantomData<(H, T)>);
struct Resource<T>(PhantomData<T>);
struct Column<T>(PhantomData<T>);
struct Accum<T>(PhantomData<T>);
struct Virtual<T>(PhantomData<T>);

// ---- index witnesses (disjoint types: the non-overlap hinge) ----
struct Here;
struct There<I>(PhantomData<I>);

// ---- pointer stand-ins ----
// Resource / column ptrs are Copy (no retained borrow), mirroring the real
// repr(transparent) ResourcePtr/ColumnPtr NonNull wrappers.
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

// An accumulator projection node: the capacity-buffer base (a raw pointer to a
// growable slot vector for the sketch) plus a 'frame borrow of the live-length
// counter. THIS is the load-bearing difference: it carries a reference, so the
// projection is lifetime-tied, not Copy-by-pointer.
struct AccPtr<'f, T> {
    base: *mut Vec<usize>, // sketch stand-in for the reserved capacity buffer
    len: &'f Cell<usize>,
    _m: PhantomData<T>,
}
// Copy/Clone unconditionally: a shared ref + a raw ptr are both Copy, so the
// projected bundle materialises without moving out of the binding.
impl<'f, T> Copy for AccPtr<'f, T> {}
impl<'f, T> Clone for AccPtr<'f, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'f, T> AccPtr<'f, T> {
    // append under &self: read live len, write at base[len], advance len.
    // Sound because `len` is a Cell (interior mutability); the base write goes
    // through a raw pointer the engine proves single-writer at plan time.
    fn append(&self, v: usize) {
        let i = self.len.get();
        // SAFETY (sketch): single-writer; the buffer has capacity > i.
        unsafe {
            (*self.base).push(v);
        }
        let _ = i;
        self.len.set(self.len.get() + 1);
    }
    fn live_len(&self) -> usize {
        self.len.get()
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
struct ABind<T, Tail> {
    base: *mut Vec<usize>,
    len: Cell<usize>,
    tail: Tail,
    _m: PhantomData<T>,
}
struct VBind<T, Tail> {
    tail: Tail,
    _m: PhantomData<T>,
}

// ---- Selector (resource): Here on RBind<T>; There pass-through on ALL ----
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
impl<T, U, Tail, I> Selector<T, There<I>> for ABind<U, Tail>
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

// ---- ColSelector (column): Here on CBind<T>; There pass-through on ALL ----
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
impl<T, U, Tail, I> ColSelector<T, There<I>> for ABind<U, Tail>
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

// ---- AccumSelector (accumulator): Here on ABind<T>; There pass-through ----
// The hinge: `get(&'s self) -> AccPtr<'s, T>` borrows self for the returned
// lifetime, so the projected accum node ties to the bindings borrow. The
// pass-through arms thread the SAME borrow down the tail.
trait AccumSelector<T, I> {
    fn get(&self) -> AccPtr<'_, T>;
}
impl<T, Tail> AccumSelector<T, Here> for ABind<T, Tail> {
    fn get(&self) -> AccPtr<'_, T> {
        AccPtr {
            base: self.base,
            len: &self.len,
            _m: PhantomData,
        }
    }
}
impl<T, U, Tail, I> AccumSelector<T, There<I>> for ABind<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    fn get(&self) -> AccPtr<'_, T> {
        self.tail.get()
    }
}
impl<T, U, Tail, I> AccumSelector<T, There<I>> for CBind<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    fn get(&self) -> AccPtr<'_, T> {
        self.tail.get()
    }
}
impl<T, U, Tail, I> AccumSelector<T, There<I>> for RBind<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    fn get(&self) -> AccPtr<'_, T> {
        self.tail.get()
    }
}
impl<T, U, Tail, I> AccumSelector<T, There<I>> for VBind<U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    fn get(&self) -> AccPtr<'_, T> {
        self.tail.get()
    }
}

// ---- projected bundles (resource, column, accum) ----
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
struct AccNil;
struct AccCons<'f, H, Tail> {
    head: AccPtr<'f, H>,
    tail: Tail,
}

// ---- Project (resources) ----
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
impl<A, T, RTail, Idx> Project<Cons<Accum<T>, RTail>, Idx> for A
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

// ---- ColProject (columns) ----
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
impl<C, T, STail, Idx> ColProject<Cons<Accum<T>, STail>, Idx> for C
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

// ---- AccumProject (accumulators): the lifetime-tied projection ----
// `Out` carries `AccPtr<'_, T>` nodes, so the bundle borrows the source for
// the projection's lifetime. The blanket impl is over `C` with the projection
// method `&self`, yielding `AccCons<'_, ...>`.
trait AccumProject<Set, Idx> {
    type Out<'s>
    where
        Self: 's;
    fn acc_project(&self) -> Self::Out<'_>;
}
impl<C> AccumProject<Empty, Empty> for C {
    type Out<'s> = AccNil where Self: 's;
    fn acc_project(&self) -> AccNil {
        AccNil
    }
}
impl<C, T, I, STail, ITail> AccumProject<Cons<Accum<T>, STail>, Cons<I, ITail>> for C
where
    C: AccumSelector<T, I>,
    C: AccumProject<STail, ITail>,
{
    type Out<'s> = AccCons<'s, T, <C as AccumProject<STail, ITail>>::Out<'s>> where Self: 's;
    fn acc_project(&self) -> Self::Out<'_> {
        AccCons {
            head: <C as AccumSelector<T, I>>::get(self),
            tail: <C as AccumProject<STail, ITail>>::acc_project(self),
        }
    }
}
impl<C, T, STail, Idx> AccumProject<Cons<Resource<T>, STail>, Idx> for C
where
    C: AccumProject<STail, Idx>,
{
    type Out<'s> = <C as AccumProject<STail, Idx>>::Out<'s> where Self: 's;
    fn acc_project(&self) -> Self::Out<'_> {
        <C as AccumProject<STail, Idx>>::acc_project(self)
    }
}
impl<C, T, STail, Idx> AccumProject<Cons<Column<T>, STail>, Idx> for C
where
    C: AccumProject<STail, Idx>,
{
    type Out<'s> = <C as AccumProject<STail, Idx>>::Out<'s> where Self: 's;
    fn acc_project(&self) -> Self::Out<'_> {
        <C as AccumProject<STail, Idx>>::acc_project(self)
    }
}
impl<C, T, STail, Idx> AccumProject<Cons<Virtual<T>, STail>, Idx> for C
where
    C: AccumProject<STail, Idx>,
{
    type Out<'s> = <C as AccumProject<STail, Idx>>::Out<'s> where Self: 's;
    fn acc_project(&self) -> Self::Out<'_> {
        <C as AccumProject<STail, Idx>>::acc_project(self)
    }
}

// ---- AccumSelector over the projected accum bundle (so append resolves) ----
impl<'f, T, Tail> AccumSelector<T, Here> for AccCons<'f, T, Tail> {
    fn get(&self) -> AccPtr<'_, T> {
        self.head
    }
}
impl<'f, T, U, Tail, I> AccumSelector<T, There<I>> for AccCons<'f, U, Tail>
where
    Tail: AccumSelector<T, I>,
{
    fn get(&self) -> AccPtr<'_, T> {
        self.tail.get()
    }
}

// ---- the dispatch-shim call shape: ONE 'frame bindings, all projections ----
// Mirrors fiber_shim: A is resource + column + accum source. Resource and
// column projections copy pointers (no retained borrow); the accum projection
// retains a 'frame borrow of the bindings. Fully-qualified calls, mirroring
// engine_ctx.rs (method syntax is ambiguous across the read/write projections).
fn project_all<'f, A, R, W, RIdx, RCIdx, WCIdx, WAIdx>(
    bindings: &'f A,
) -> (
    <A as Project<R, RIdx>>::Out,
    <A as ColProject<R, RCIdx>>::Out,
    <A as ColProject<W, WCIdx>>::Out,
    <A as AccumProject<W, WAIdx>>::Out<'f>,
)
where
    A: Project<R, RIdx>,
    A: ColProject<R, RCIdx>,
    A: ColProject<W, WCIdx>,
    A: AccumProject<W, WAIdx>,
{
    (
        <A as Project<R, RIdx>>::project(bindings),
        <A as ColProject<R, RCIdx>>::col_project(bindings),
        <A as ColProject<W, WCIdx>>::col_project(bindings),
        <A as AccumProject<W, WAIdx>>::acc_project(bindings),
    )
}

// ---- distinct store types (each appears once per the access-set invariant) --
struct R1;
struct C1;
struct A1;
struct A2;

fn main() {
    let mut buf_a1: Vec<usize> = Vec::new();
    let mut buf_a2: Vec<usize> = Vec::new();
    let p_a1: *mut Vec<usize> = &mut buf_a1;
    let p_a2: *mut Vec<usize> = &mut buf_a2;

    // Interleaved bindings: RBind<R1> -> CBind<C1> -> ABind<A1> -> ABind<A2>.
    // A1 sits behind a resource and a column node (exercises AccumSelector
    // pass-through over RBind + CBind); A2 sits behind A1 (pass-through over
    // ABind).
    let bindings = RBind::<R1, _> {
        ptr: RPtr(11, PhantomData),
        tail: CBind::<C1, _> {
            ptr: CPtr(21, PhantomData),
            tail: ABind::<A1, _> {
                base: p_a1,
                len: Cell::new(0),
                tail: ABind::<A2, _> {
                    base: p_a2,
                    len: Cell::new(0),
                    tail: BNil,
                    _m: PhantomData,
                },
                _m: PhantomData,
            },
        },
    };

    // Read set R = [Resource<R1>, Column<C1>]; write set W = [Accum<A1>, Accum<A2>].
    type R = Cons<Resource<R1>, Cons<Column<C1>, Empty>>;
    type W = Cons<Accum<A1>, Cons<Accum<A2>, Empty>>;

    let (reads, read_cols, _write_cols, accums) =
        project_all::<_, R, W, _, _, _, _>(&bindings);

    // resource + column projections resolved (Copy-by-pointer)
    assert_eq!(reads.head.0, 11, "resource R1 resolves to tag 11");
    assert_eq!(read_cols.head.0, 21, "column C1 resolves to tag 21");

    // append through the accum bundle: A1 gets [100, 101], A2 gets [200].
    let a1: AccPtr<'_, A1> = <_ as AccumSelector<A1, _>>::get(&accums);
    a1.append(100);
    a1.append(101);
    let a2: AccPtr<'_, A2> = <_ as AccumSelector<A2, _>>::get(&accums);
    a2.append(200);

    assert_eq!(a1.live_len(), 2, "A1 live length advanced to 2");
    assert_eq!(a2.live_len(), 1, "A2 live length advanced to 1");
    assert_eq!(buf_a1, vec![100, 101], "A1 appended values landed in order");
    assert_eq!(buf_a2, vec![200], "A2 appended value landed");

    println!(
        "WORKS: 3-way Selector+ColSelector+AccumSelector resolves; the 'frame-borrowed \
         accum projection composes with copy-pointer projections; append advances the \
         live-length under &self (A1 len=2 [100,101], A2 len=1 [200])"
    );
}
