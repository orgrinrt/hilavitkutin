//! Stand-in for the revised parts of `hilavitkutin-api`.
//!
//! Numbers are bare here, as in the sketches before this one; the production
//! form takes arvo's types. Everything a unit author names is in this crate.

#![no_std]

use core::marker::PhantomData;

pub use notko_hlist::{
    Carrier,
    Cons,
    Contains,
    Elem,
    Empty,
    Has,
    Here,
    List,
    PCons,
    PNil,
    Place,
    Project,
    There,
    Val,
    Vals,
};

// ---------------------------------------------------------------------------
// The api's own message on a store missing from a list.
// ---------------------------------------------------------------------------

/// The list holds the store `S`, said with the api's own message.
///
/// `Contains` is the shared crate's and carries the shared crate's message. A
/// bound on this trait reports the message below instead, because the blanket
/// is marked so that the compiler does not descend into it.
#[diagnostic::on_unimplemented(
    message = "the store `{S}` is not in `{Self}`",
    note = "register it with `.with(..)` before `build`, or install a kit that owns it"
)]
pub trait Holds<S>: List {}

#[diagnostic::do_not_recommend]
impl<L: Contains<S>, S> Holds<S> for L {}

// ---------------------------------------------------------------------------
// Record domains.
// ---------------------------------------------------------------------------

/// One axis of a domain. Its value in a shape is the count of places along it.
pub trait Axis: Elem<Value = usize> {}

/// A list of axes, outermost first, with the arithmetic of the key order.
///
/// The record at key `k` has index `k0 * n1 * n2 + k1 * n2 + k2` for three
/// axes of counts `n0, n1, n2`: the first axis named varies slowest.
pub trait Grid: Project<Prod<Val> = <Self as Grid>::Shape> {
    /// One count per axis, and one place per axis when read as a key.
    type Shape: Copy + 'static;
    /// How many records a shape holds.
    fn count(shape: &Self::Shape) -> usize;
    /// The index of the record at `key`.
    fn index(key: &Self::Shape, shape: &Self::Shape) -> usize;
    /// The key of the record at `index`.
    fn key(index: usize, shape: &Self::Shape) -> Self::Shape;
}

impl Grid for Empty {
    type Shape = PNil;

    fn count(_: &PNil) -> usize {
        1
    }

    fn index(_: &PNil, _: &PNil) -> usize {
        0
    }

    fn key(_: usize, _: &PNil) -> PNil {
        PNil
    }
}

impl<A: Axis, T: Grid> Grid for Cons<A, T> {
    type Shape = PCons<Val, A, T::Shape>;

    fn count(shape: &Self::Shape) -> usize {
        shape.head * T::count(&shape.tail)
    }

    fn index(key: &Self::Shape, shape: &Self::Shape) -> usize {
        key.head * T::count(&shape.tail) + T::index(&key.tail, &shape.tail)
    }

    fn key(index: usize, shape: &Self::Shape) -> Self::Shape {
        let inner = T::count(&shape.tail);
        PCons {
            head: index / inner,
            tail: T::key(index % inner, &shape.tail),
        }
    }
}

/// A set of records with its own extent and key order.
pub trait Domain: 'static {
    /// Its axes, outermost first.
    type Axes: Grid;
    /// The offsets a neighbour read in this domain may name.
    type Reach: List;
}

/// The run-time shape of domain `D`: one count per axis.
pub type Shape<D> = <<D as Domain>::Axes as Grid>::Shape;

/// The one axis of [`Records`].
pub struct Ordinal;
impl Elem for Ordinal {
    type Value = usize;
}
impl Axis for Ordinal {}

/// The default domain: one axis, no neighbour reads.
pub struct Records;
impl Domain for Records {
    type Axes = Cons<Ordinal, Empty>;
    type Reach = Empty;
}

/// `N` places along axis `A`, negative for back. An offset is a list of these.
pub struct Shift<A, const N: isize>(PhantomData<A>);

/// A list of shifts, applied to a key of the axes `Ax`.
pub trait Offset<Ax: Grid, Ps>: 'static {
    /// Moves `key` by the offset. False when it leaves the shape.
    fn step(key: &mut Ax::Shape, shape: &Ax::Shape) -> bool;
}

impl<Ax: Grid> Offset<Ax, ()> for Empty {
    fn step(_: &mut Ax::Shape, _: &Ax::Shape) -> bool {
        true
    }
}

impl<Ax, A, const N: isize, T, P, Ps> Offset<Ax, (P, Ps)> for Cons<Shift<A, N>, T>
where
    Ax: Grid + Has<A, P>,
    A: Axis,
    P: Place + 'static,
    Ps: 'static,
    T: Offset<Ax, Ps>,
{
    fn step(key: &mut Ax::Shape, shape: &Ax::Shape) -> bool {
        let count = *Ax::pick::<Val>(shape);
        let at = Ax::pick_mut::<Val>(key);
        let moved = *at as isize + N;
        if moved < 0 || moved >= count as isize {
            return false;
        }
        *at = moved as usize;
        T::step(key, shape)
    }
}

/// What a neighbour read past the shape returns, declared at the read.
pub trait Edge<V>: 'static {
    /// The value read outside.
    const OUTSIDE: V;
}

// ---------------------------------------------------------------------------
// Store kinds.
// ---------------------------------------------------------------------------

macro_rules! marker {
    ($(#[$doc:meta])* $name:ident<$($p:ident),*>) => {
        $(#[$doc])*
        pub struct $name<$($p),*>(PhantomData<($($p,)*)>);
        impl<$($p),*> $name<$($p),*> {
            /// The marker, as a value to name the member by.
            pub const fn new() -> Self {
                Self(PhantomData)
            }
        }
    };
}

/// A value per record of domain `D`, written in place.
pub struct Column<T, D = Records>(PhantomData<(T, D)>);
/// A value per record of domain `D`, frame-buffered: reads see the committed
/// frame, writes go to the staged side, and the close of the pass swaps them.
pub struct Buffered<T, D = Records>(PhantomData<(T, D)>);

impl<T, D> Column<T, D> {
    /// The marker, as a value to name the member by.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T, D> Buffered<T, D> {
    /// The marker, as a value to name the member by.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

marker!(
    /// An append-only run of values.
    Accum<T>
);
marker!(
    /// One value.
    Resource<T>
);
marker!(
    /// A fired flag.
    Virtual<V>
);
marker!(
    /// The run-time shape of domain `D`, given to the builder.
    Extent<D>
);

/// A frame-buffered store: the kind a neighbour read may name, since its
/// committed side does not change during a pass.
pub trait Committed: Elem {
    /// The domain its records are in.
    type Domain: Domain;
}

/// The offsets a neighbour read of committed store `C` may name.
pub type ReachOf<C> = <<C as Committed>::Domain as Domain>::Reach;

/// The committed value of `C` at offset `O` from the record, `E` past the shape.
pub struct Near<C, O, E>(PhantomData<(C, O, E)>)
where
    C: Committed,
    ReachOf<C>: Contains<O>;

impl<C, O, E> Near<C, O, E>
where
    C: Committed,
    ReachOf<C>: Contains<O>,
{
    /// The marker, as a value to name the member by.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Copy + 'static, D: Domain> Elem for Column<T, D> {
    type Value = T;
}
impl<T: Copy + 'static, D: Domain> Elem for Buffered<T, D> {
    type Value = T;
}
impl<T: Copy + 'static, D: Domain> Committed for Buffered<T, D> {
    type Domain = D;
}
impl<T: Copy + 'static> Elem for Accum<T> {
    /// Written only: a value to append, or none.
    type Value = Option<T>;
}
impl<T: Copy + 'static> Elem for Resource<T> {
    type Value = T;
}
impl<V: 'static> Elem for Virtual<V> {
    /// Written only: whether to fire.
    type Value = bool;
}
impl<D: Domain> Elem for Extent<D> {
    type Value = Shape<D>;
}
impl<C, O: 'static, E: 'static> Elem for Near<C, O, E>
where
    C: Committed,
    ReachOf<C>: Contains<O>,
{
    type Value = C::Value;
}

// ---------------------------------------------------------------------------
// One domain per unit.
// ---------------------------------------------------------------------------

/// A member that names no domain.
pub struct Free;
/// A member of domain `D`.
pub struct In<D>(PhantomData<D>);

/// The domain two members agree on. No impl for two different domains.
#[diagnostic::on_unimplemented(
    message = "a unit iterates one domain, and these members are in two",
    note = "every `Column`, `Buffered` and `Near` member of one unit names the same domain; read the other domain's data through a `Resource`"
)]
pub trait Join<X> {
    /// The agreed side.
    type Out;
}
impl Join<Free> for Free {
    type Out = Free;
}
impl<D> Join<In<D>> for Free {
    type Out = In<D>;
}
impl<D> Join<Free> for In<D> {
    type Out = In<D>;
}
impl<D> Join<In<D>> for In<D> {
    type Out = In<D>;
}

/// The domain a unit iterates, from what its members agreed on.
pub trait Pick {
    /// The domain.
    type Domain: Domain;
}
impl Pick for Free {
    type Domain = Records;
}
impl<D: Domain> Pick for In<D> {
    type Domain = D;
}

/// A member of a value-form list.
pub trait Member: Elem {
    /// `Free`, or `In<D>`.
    type Where;
    /// The store it reads or writes.
    type Store;
}
impl<T: Copy + 'static, D: Domain> Member for Column<T, D> {
    type Store = Self;
    type Where = In<D>;
}
impl<T: Copy + 'static, D: Domain> Member for Buffered<T, D> {
    type Store = Self;
    type Where = In<D>;
}
impl<T: Copy + 'static> Member for Accum<T> {
    type Store = Self;
    type Where = Free;
}
impl<T: Copy + 'static> Member for Resource<T> {
    type Store = Self;
    type Where = Free;
}
impl<V: 'static> Member for Virtual<V> {
    type Store = Self;
    type Where = Free;
}
impl<D: Domain> Member for Extent<D> {
    type Store = Self;
    type Where = Free;
}
impl<C, O: 'static, E: Edge<C::Value>> Member for Near<C, O, E>
where
    C: Committed,
    ReachOf<C>: Contains<O>,
{
    type Store = C;
    type Where = In<C::Domain>;
}

/// A member a value-form unit may read.
pub trait Reads: Member {}
impl<T: Copy + 'static, D: Domain> Reads for Column<T, D> {}
impl<T: Copy + 'static, D: Domain> Reads for Buffered<T, D> {}
impl<T: Copy + 'static> Reads for Resource<T> {}
impl<D: Domain> Reads for Extent<D> {}
impl<C, O: 'static, E: Edge<C::Value>> Reads for Near<C, O, E>
where
    C: Committed,
    ReachOf<C>: Contains<O>,
{
}

/// A member a value-form unit may write.
pub trait Writes: Member {}
impl<T: Copy + 'static, D: Domain> Writes for Column<T, D> {}
impl<T: Copy + 'static, D: Domain> Writes for Buffered<T, D> {}
impl<T: Copy + 'static> Writes for Accum<T> {}
impl<V: 'static> Writes for Virtual<V> {}

/// A value-form read list.
pub trait ReadList: Project {
    /// What its members agreed on.
    type Where;
    /// The stores it reads.
    type Stores: List;
}
impl ReadList for Empty {
    type Stores = Empty;
    type Where = Free;
}
impl<M: Reads, T: ReadList> ReadList for Cons<M, T>
where
    M::Where: Join<T::Where>,
{
    type Stores = Cons<M::Store, T::Stores>;
    type Where = <M::Where as Join<T::Where>>::Out;
}

/// A value-form write list.
pub trait WriteList: Project {
    /// What its members agreed on.
    type Where;
    /// The stores it writes.
    type Stores: List;
}
impl WriteList for Empty {
    type Stores = Empty;
    type Where = Free;
}
impl<M: Writes, T: WriteList> WriteList for Cons<M, T>
where
    M::Where: Join<T::Where>,
{
    type Stores = Cons<M::Store, T::Stores>;
    type Where = <M::Where as Join<T::Where>>::Out;
}

/// A write list in the same domain as the read list `R`.
pub trait Along<R: ReadList>: WriteList {
    /// The domain the unit iterates.
    type Domain: Domain;
}
impl<R: ReadList, W: WriteList> Along<R> for W
where
    R::Where: Join<W::Where>,
    <R::Where as Join<W::Where>>::Out: Pick,
{
    type Domain = <<R::Where as Join<W::Where>>::Out as Pick>::Domain;
}

// ---------------------------------------------------------------------------
// The two unit forms and their routers.
// ---------------------------------------------------------------------------

/// The value form: the values of the reads at one record in, the values of the
/// writes out. No index, no range, no context.
pub trait RecordOp: 'static {
    /// What it reads.
    type Read: ReadList;
    /// What it writes.
    type Write: Along<Self::Read>;
    /// One record.
    fn apply(&self, read: &Vals<Self::Read>) -> Vals<Self::Write>;
}

/// The domain a value-form unit iterates.
pub type DomainOf<U> = <<U as RecordOp>::Write as Along<<U as RecordOp>::Read>>::Domain;

/// The block form, as it stands: the author names the context.
pub trait WorkUnit: 'static {
    /// What it reads.
    type Read: List;
    /// What it writes.
    type Write: List;
    /// The context its body is handed.
    type Ctx<'frame>;
    /// One morsel.
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>);
}

/// What a scheduler builder accepts.
pub trait BuilderInput {
    /// The router for this input.
    type Dispatch;
}

/// Routes a block-form unit.
pub struct UnitDispatch<W>(PhantomData<W>);
/// Routes a value-form unit.
pub struct RecordDispatch<U>(PhantomData<U>);

/// What the plan reads off a unit, whichever form it is in.
pub trait Declares<W> {
    /// The stores it reads.
    type Read: List;
    /// The stores it writes.
    type Write: List;
    /// The domain it iterates.
    type Domain: Domain;
}
impl<W: WorkUnit> Declares<W> for UnitDispatch<W> {
    // The sketch gives a block unit the default domain. The production form
    // folds its access sets the way `Along` does.
    type Domain = Records;
    type Read = W::Read;
    type Write = W::Write;
}
impl<U: RecordOp> Declares<U> for RecordDispatch<U> {
    type Domain = DomainOf<U>;
    type Read = <U::Read as ReadList>::Stores;
    type Write = <U::Write as WriteList>::Stores;
}

/// The values of a write list, in the order it names its members.
#[macro_export]
macro_rules! rec {
    () => { $crate::Vals($crate::PNil) };
    ($($v:expr),+ $(,)?) => { $crate::Vals($crate::rec!(@cells $($v),+)) };
    (@cells $head:expr $(, $rest:expr)*) => {
        $crate::PCons { head: $head, tail: $crate::rec!(@cells $($rest),*) }
    };
    (@cells) => { $crate::PNil };
}
