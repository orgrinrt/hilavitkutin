//! Storage both runners stand on: a lane per store, the gather of a read list
//! at one record, the scatter of a write list, and the close of a pass.
//!
//! This is plumbing. It uses `std` because a sketch runner may; nothing here is
//! named by a unit.

use api::{
    Accum,
    Buffered,
    Carrier,
    Column,
    Committed,
    Cons,
    Contains,
    Domain,
    Edge,
    Elem,
    Empty,
    Extent,
    Grid,
    Has,
    Near,
    Offset,
    PCons,
    PNil,
    Place,
    Project,
    Resource,
    Shape,
    Val,
    Virtual,
};

/// What a world holds per store.
pub struct Slot;

/// One store. `front` is what reads see. `back` is the staged side of a
/// frame-buffered store and unused by the other kinds.
pub struct Lane<V> {
    pub front:  Vec<V>,
    pub back:   Vec<V>,
    pub staged: bool,
}

impl<V> Default for Lane<V> {
    fn default() -> Self {
        Lane {
            front:  Vec::new(),
            back:   Vec::new(),
            staged: false,
        }
    }
}

impl Carrier for Slot {
    type Of<E: Elem> = Lane<E::Value>;
}

/// Every store of the list `S`.
pub struct World<S: Project> {
    pub lanes: S::Prod<Slot>,
}

impl<S: Project> World<S> {
    /// A world with every lane empty.
    pub fn new() -> Self
    where
        S::Prod<Slot>: Default,
    {
        World {
            lanes: Default::default(),
        }
    }

    /// Gives domain `D` its shape.
    pub fn size<D: Domain, P: Place>(&mut self, shape: Shape<D>)
    where
        S: Has<Extent<D>, P>,
    {
        S::pick_mut::<Slot>(&mut self.lanes).front = vec![shape];
    }

    /// Fills a per-record store of domain `D` from the record index.
    pub fn fill<M, D, P, Pe>(&mut self, _: M, f: impl Fn(usize) -> M::Value)
    where
        M: Elem,
        D: Domain,
        P: Place,
        Pe: Place,
        S: Has<M, P> + Has<Extent<D>, Pe>,
    {
        let shape = <S as Has<Extent<D>, Pe>>::pick::<Slot>(&self.lanes).front[0];
        let count = <D::Axes as Grid>::count(&shape);
        let lane = <S as Has<M, P>>::pick_mut::<Slot>(&mut self.lanes);
        lane.front = (0 .. count).map(&f).collect();
        lane.back = (0 .. count).map(&f).collect();
    }

    /// Sets a one-value store.
    pub fn put<M: Elem, P: Place>(&mut self, _: M, v: M::Value)
    where
        S: Has<M, P>,
    {
        S::pick_mut::<Slot>(&mut self.lanes).front = vec![v];
    }

    /// One store, to look at.
    pub fn lane<M: Elem, P: Place>(&self, _: M) -> &Lane<M::Value>
    where
        S: Has<M, P>,
    {
        S::pick::<Slot>(&self.lanes)
    }

    /// How many records domain `D` holds.
    pub fn count<D: Domain, P: Place>(&self) -> usize
    where
        S: Has<Extent<D>, P>,
    {
        <D::Axes as Grid>::count(&S::pick::<Slot>(&self.lanes).front[0])
    }
}

impl<S: Project> Default for World<S>
where
    S::Prod<Slot>: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Gather: the values of a read list at one record.
// ---------------------------------------------------------------------------

/// The values of a read list at record `i`, out of the stores `S`.
pub trait Gather<S: Project, Ps>: Project {
    fn gather(lanes: &S::Prod<Slot>, i: usize) -> Self::Prod<Val>;
}

impl<S: Project> Gather<S, ()> for Empty {
    fn gather(_: &S::Prod<Slot>, _: usize) -> PNil {
        PNil
    }
}

impl<S, T, D, Tl, P, Ps> Gather<S, (P, Ps)> for Cons<Column<T, D>, Tl>
where
    S: Has<Column<T, D>, P>,
    T: Copy + 'static,
    D: Domain,
    P: Place,
    Tl: Gather<S, Ps>,
{
    fn gather(lanes: &S::Prod<Slot>, i: usize) -> Self::Prod<Val> {
        PCons {
            head: S::pick::<Slot>(lanes).front[i],
            tail: Tl::gather(lanes, i),
        }
    }
}

impl<S, T, D, Tl, P, Ps> Gather<S, (P, Ps)> for Cons<Buffered<T, D>, Tl>
where
    S: Has<Buffered<T, D>, P>,
    T: Copy + 'static,
    D: Domain,
    P: Place,
    Tl: Gather<S, Ps>,
{
    fn gather(lanes: &S::Prod<Slot>, i: usize) -> Self::Prod<Val> {
        PCons {
            head: S::pick::<Slot>(lanes).front[i],
            tail: Tl::gather(lanes, i),
        }
    }
}

impl<S, T, Tl, P, Ps> Gather<S, (P, Ps)> for Cons<Resource<T>, Tl>
where
    S: Has<Resource<T>, P>,
    T: Copy + 'static,
    P: Place,
    Tl: Gather<S, Ps>,
{
    fn gather(lanes: &S::Prod<Slot>, i: usize) -> Self::Prod<Val> {
        PCons {
            head: S::pick::<Slot>(lanes).front[0],
            tail: Tl::gather(lanes, i),
        }
    }
}

impl<S, D, Tl, P, Ps> Gather<S, (P, Ps)> for Cons<Extent<D>, Tl>
where
    S: Has<Extent<D>, P>,
    D: Domain,
    P: Place,
    Tl: Gather<S, Ps>,
{
    fn gather(lanes: &S::Prod<Slot>, i: usize) -> Self::Prod<Val> {
        PCons {
            head: S::pick::<Slot>(lanes).front[0],
            tail: Tl::gather(lanes, i),
        }
    }
}

impl<S, C, O, E, Tl, P, Pe, Po, Ps> Gather<S, (P, Pe, Po, Ps)> for Cons<Near<C, O, E>, Tl>
where
    C: Committed,
    <C::Domain as Domain>::Reach: Contains<O>,
    S: Has<C, P> + Has<Extent<C::Domain>, Pe>,
    O: Offset<<C::Domain as Domain>::Axes, Po>,
    E: Edge<C::Value>,
    P: Place,
    Pe: Place,
    Tl: Gather<S, Ps>,
{
    fn gather(lanes: &S::Prod<Slot>, i: usize) -> Self::Prod<Val> {
        let shape: Shape<C::Domain> =
            <S as Has<Extent<C::Domain>, Pe>>::pick::<Slot>(lanes).front[0];
        let mut key = <<C::Domain as Domain>::Axes as Grid>::key(i, &shape);
        let head = if O::step(&mut key, &shape) {
            let at = <<C::Domain as Domain>::Axes as Grid>::index(&key, &shape);
            <S as Has<C, P>>::pick::<Slot>(lanes).front[at]
        } else {
            E::OUTSIDE
        };
        PCons {
            head,
            tail: Tl::gather(lanes, i),
        }
    }
}

// ---------------------------------------------------------------------------
// Scatter: the values of a write list, into the stores.
// ---------------------------------------------------------------------------

/// Writes the values of a write list at record `i` into the stores `S`.
pub trait Scatter<S: Project, Ps>: Project {
    fn scatter(lanes: &mut S::Prod<Slot>, i: usize, v: &Self::Prod<Val>);
}

impl<S: Project> Scatter<S, ()> for Empty {
    fn scatter(_: &mut S::Prod<Slot>, _: usize, _: &PNil) {}
}

impl<S, T, D, Tl, P, Ps> Scatter<S, (P, Ps)> for Cons<Column<T, D>, Tl>
where
    S: Has<Column<T, D>, P>,
    T: Copy + 'static,
    D: Domain,
    P: Place,
    Tl: Scatter<S, Ps>,
{
    fn scatter(lanes: &mut S::Prod<Slot>, i: usize, v: &Self::Prod<Val>) {
        S::pick_mut::<Slot>(lanes).front[i] = v.head;
        Tl::scatter(lanes, i, &v.tail);
    }
}

impl<S, T, D, Tl, P, Ps> Scatter<S, (P, Ps)> for Cons<Buffered<T, D>, Tl>
where
    S: Has<Buffered<T, D>, P>,
    T: Copy + 'static,
    D: Domain,
    P: Place,
    Tl: Scatter<S, Ps>,
{
    fn scatter(lanes: &mut S::Prod<Slot>, i: usize, v: &Self::Prod<Val>) {
        let lane = S::pick_mut::<Slot>(lanes);
        #[cfg(not(feature = "mutant-in-place"))]
        {
            lane.back[i] = v.head;
            lane.staged = true;
        }
        #[cfg(feature = "mutant-in-place")]
        {
            lane.front[i] = v.head;
        }
        Tl::scatter(lanes, i, &v.tail);
    }
}

impl<S, T, Tl, P, Ps> Scatter<S, (P, Ps)> for Cons<Accum<T>, Tl>
where
    S: Has<Accum<T>, P>,
    T: Copy + 'static,
    P: Place,
    Tl: Scatter<S, Ps>,
{
    fn scatter(lanes: &mut S::Prod<Slot>, i: usize, v: &Self::Prod<Val>) {
        if v.head.is_some() {
            S::pick_mut::<Slot>(lanes).front.push(v.head);
        }
        Tl::scatter(lanes, i, &v.tail);
    }
}

impl<S, V, Tl, P, Ps> Scatter<S, (P, Ps)> for Cons<Virtual<V>, Tl>
where
    S: Has<Virtual<V>, P>,
    V: 'static,
    P: Place,
    Tl: Scatter<S, Ps>,
{
    fn scatter(lanes: &mut S::Prod<Slot>, i: usize, v: &Self::Prod<Val>) {
        if v.head {
            S::pick_mut::<Slot>(lanes).front = vec![true];
        }
        Tl::scatter(lanes, i, &v.tail);
    }
}

// ---------------------------------------------------------------------------
// Close: the end of a pass.
// ---------------------------------------------------------------------------

/// The close of a pass over every store: a frame-buffered store that was
/// written swaps its sides, and nothing else moves.
pub trait Close: Project {
    fn close(lanes: &mut Self::Prod<Slot>);
}

impl Close for Empty {
    fn close(_: &mut PNil) {}
}

macro_rules! close_keeps {
    ($($kind:ident<$($p:ident),*> where [$($bound:tt)*];)*) => {$(
        impl<$($p,)* Tl: Close> Close for Cons<$kind<$($p),*>, Tl>
        where
            $($bound)*
        {
            fn close(lanes: &mut Self::Prod<Slot>) {
                Tl::close(&mut lanes.tail);
            }
        }
    )*};
}

close_keeps! {
    Column<T, D> where [T: Copy + 'static, D: Domain];
    Accum<T> where [T: Copy + 'static];
    Resource<T> where [T: Copy + 'static];
    Virtual<V> where [V: 'static];
    Extent<D> where [D: Domain];
}

impl<T: Copy + 'static, D: Domain, Tl: Close> Close for Cons<Buffered<T, D>, Tl> {
    fn close(lanes: &mut Self::Prod<Slot>) {
        if lanes.head.staged {
            core::mem::swap(&mut lanes.head.front, &mut lanes.head.back);
            lanes.head.staged = false;
        }
        Tl::close(&mut lanes.tail);
    }
}
