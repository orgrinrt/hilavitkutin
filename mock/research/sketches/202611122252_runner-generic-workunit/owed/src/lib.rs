//! What `notko-hlist` lacks for a kernel that is handed things: a selector that goes
//! from a member to its place, and a projection from a type list to a product of
//! values (or of columns, or of arrays). Written once here and re-exported by both
//! vocabularies, so each unit still depends on its vocabulary alone. FINDINGS.md lists
//! these under "owed to notko-hlist".

#![no_std]

pub use notko_hlist::{Cons, Empty, Here, List, Place, There};

/// A member of a list: a marker the author passes as a value, naming one value type.
pub trait Elem: 'static {
    type Value: Copy + 'static;
}

/// What a product holds per member: the value, a column, a mutable column, an array.
pub trait Carrier {
    type Of<E: Elem>;
}

/// One value per member.
pub struct Val;
/// A shared column per member.
pub struct Col<'a>(core::marker::PhantomData<&'a ()>);
/// An exclusive column per member.
pub struct ColMut<'a>(core::marker::PhantomData<&'a mut ()>);
/// An owned array of `N` per member.
pub struct Arr<const N: usize>;

impl Carrier for Val {
    type Of<E: Elem> = E::Value;
}
impl<'a> Carrier for Col<'a> {
    type Of<E: Elem> = &'a [E::Value];
}
impl<'a> Carrier for ColMut<'a> {
    type Of<E: Elem> = &'a mut [E::Value];
}
impl<const N: usize> Carrier for Arr<N> {
    type Of<E: Elem> = [E::Value; N];
}

/// A carrier read by row.
pub trait ByRow: Carrier {
    fn at<E: Elem>(c: &Self::Of<E>, i: usize) -> E::Value;
}
/// A carrier written by row.
pub trait ByRowMut: ByRow {
    fn set<E: Elem>(c: &mut Self::Of<E>, i: usize, v: E::Value);
}
/// A carrier whose per-member item is itself `Copy`.
pub trait Dup: Carrier {
    fn dup<E: Elem>(c: &Self::Of<E>) -> Self::Of<E>;
}

impl<'a> ByRow for Col<'a> {
    fn at<E: Elem>(c: &Self::Of<E>, i: usize) -> E::Value {
        c[i]
    }
}
impl<'a> ByRow for ColMut<'a> {
    fn at<E: Elem>(c: &Self::Of<E>, i: usize) -> E::Value {
        c[i]
    }
}
impl<'a> ByRowMut for ColMut<'a> {
    fn set<E: Elem>(c: &mut Self::Of<E>, i: usize, v: E::Value) {
        c[i] = v;
    }
}
impl<const N: usize> ByRow for Arr<N> {
    fn at<E: Elem>(c: &Self::Of<E>, i: usize) -> E::Value {
        c[i]
    }
}
impl<const N: usize> ByRowMut for Arr<N> {
    fn set<E: Elem>(c: &mut Self::Of<E>, i: usize, v: E::Value) {
        c[i] = v;
    }
}
impl Dup for Val {
    fn dup<E: Elem>(c: &Self::Of<E>) -> Self::Of<E> {
        *c
    }
}
impl<'a> Dup for Col<'a> {
    fn dup<E: Elem>(c: &Self::Of<E>) -> Self::Of<E> {
        c
    }
}

/// A cell of a product: what carrier `C` holds for member `H`, then the rest.
pub struct PCons<C: Carrier, H: Elem, T> {
    pub head: C::Of<H>,
    pub tail: T,
}
/// The end of a product.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PNil;

impl<C: Carrier, H: Elem, T: Default> Default for PCons<C, H, T>
where
    C::Of<H>: Default,
{
    fn default() -> Self {
        PCons {
            head: Default::default(),
            tail: T::default(),
        }
    }
}

impl<C: Carrier, H: Elem, T: Clone> Clone for PCons<C, H, T>
where
    C::Of<H>: Clone,
{
    fn clone(&self) -> Self {
        PCons {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
}
impl<C: Carrier, H: Elem, T: Copy> Copy for PCons<C, H, T> where C::Of<H>: Copy {}
impl<C: Carrier, H: Elem, T: PartialEq> PartialEq for PCons<C, H, T>
where
    C::Of<H>: PartialEq,
{
    fn eq(&self, o: &Self) -> bool {
        self.head == o.head && self.tail == o.tail
    }
}

/// The projection from a type list to a product: `Cons<H, T>` holds `C::Of<H>` then
/// `T`'s product. Every operation here recurses on the list's structure alone.
pub trait Project: List {
    type Prod<C: Carrier>;

    fn read_row<C: ByRow>(p: &Self::Prod<C>, i: usize) -> Self::Prod<Val>;
    fn write_row<C: ByRowMut>(p: &mut Self::Prod<C>, i: usize, v: &Self::Prod<Val>);
    fn cols<const N: usize>(p: &Self::Prod<Arr<N>>) -> Self::Prod<Col<'_>>;
    fn cols_mut<const N: usize>(p: &mut Self::Prod<Arr<N>>) -> Self::Prod<ColMut<'_>>;
    fn split_mut<'a>(
        p: Self::Prod<ColMut<'a>>,
        mid: usize,
    ) -> (Self::Prod<ColMut<'a>>, Self::Prod<ColMut<'a>>);
    /// In place: a by-value copy of a deep product costs a frame per level.
    fn copy_from<const N: usize>(dst: &mut Self::Prod<Arr<N>>, src: &Self::Prod<Arr<N>>);
    /// In place, field by field, so building a deep product costs one frame.
    fn splat_in<const N: usize>(
        dst: &mut core::mem::MaybeUninit<Self::Prod<Arr<N>>>,
        row: &Self::Prod<Val>,
    );

    /// A product of arrays, every row `row`, built in place.
    fn made<const N: usize>(row: &Self::Prod<Val>) -> Self::Prod<Arr<N>> {
        let mut p = core::mem::MaybeUninit::uninit();
        Self::splat_in::<N>(&mut p, row);
        // SAFETY: `splat_in` writes every field of the product, recursively to `PNil`.
        unsafe { p.assume_init() }
    }
}

impl Project for Empty {
    type Prod<C: Carrier> = PNil;

    fn read_row<C: ByRow>(_: &PNil, _: usize) -> PNil {
        PNil
    }

    fn write_row<C: ByRowMut>(_: &mut PNil, _: usize, _: &PNil) {}

    fn cols<const N: usize>(_: &PNil) -> PNil {
        PNil
    }

    fn cols_mut<const N: usize>(_: &mut PNil) -> PNil {
        PNil
    }

    fn split_mut<'a>(
        _: Self::Prod<ColMut<'a>>,
        _: usize,
    ) -> (Self::Prod<ColMut<'a>>, Self::Prod<ColMut<'a>>) {
        (PNil, PNil)
    }

    fn copy_from<const N: usize>(_: &mut PNil, _: &PNil) {}

    fn splat_in<const N: usize>(dst: &mut core::mem::MaybeUninit<PNil>, _: &PNil) {
        dst.write(PNil);
    }
}

impl<H: Elem, T: Project> Project for Cons<H, T> {
    type Prod<C: Carrier> = PCons<C, H, T::Prod<C>>;

    fn read_row<C: ByRow>(p: &Self::Prod<C>, i: usize) -> Self::Prod<Val> {
        PCons {
            head: C::at::<H>(&p.head, i),
            tail: T::read_row::<C>(&p.tail, i),
        }
    }

    fn write_row<C: ByRowMut>(p: &mut Self::Prod<C>, i: usize, v: &Self::Prod<Val>) {
        C::set::<H>(&mut p.head, i, v.head);
        T::write_row::<C>(&mut p.tail, i, &v.tail);
    }

    fn cols<const N: usize>(p: &Self::Prod<Arr<N>>) -> Self::Prod<Col<'_>> {
        PCons {
            head: &p.head[..],
            tail: T::cols::<N>(&p.tail),
        }
    }

    fn cols_mut<const N: usize>(p: &mut Self::Prod<Arr<N>>) -> Self::Prod<ColMut<'_>> {
        PCons {
            head: &mut p.head[..],
            tail: T::cols_mut::<N>(&mut p.tail),
        }
    }

    fn split_mut<'a>(
        p: Self::Prod<ColMut<'a>>,
        mid: usize,
    ) -> (Self::Prod<ColMut<'a>>, Self::Prod<ColMut<'a>>) {
        let (a, b) = p.head.split_at_mut(mid);
        let (ta, tb) = T::split_mut(p.tail, mid);
        (
            PCons {
                head: a,
                tail: ta,
            },
            PCons {
                head: b,
                tail: tb,
            },
        )
    }

    fn copy_from<const N: usize>(dst: &mut Self::Prod<Arr<N>>, src: &Self::Prod<Arr<N>>) {
        dst.head = src.head;
        T::copy_from::<N>(&mut dst.tail, &src.tail);
    }

    fn splat_in<const N: usize>(
        dst: &mut core::mem::MaybeUninit<Self::Prod<Arr<N>>>,
        row: &Self::Prod<Val>,
    ) {
        let p = dst.as_mut_ptr();
        // SAFETY: `p` points at storage for the whole product; the head is written
        // through its own field pointer, and the tail is handed on as storage of its
        // own type, which `MaybeUninit<X>` has the layout of.
        unsafe {
            core::ptr::addr_of_mut!((*p).head).write([row.head; N]);
            let tail = &mut *(core::ptr::addr_of_mut!((*p).tail)
                as *mut core::mem::MaybeUninit<T::Prod<Arr<N>>>);
            T::splat_in::<N>(tail, &row.tail);
        }
    }
}

/// The selector: member `M` as the trait parameter, its place `P` inferred. This is the
/// direction `notko_hlist::At` does not go (`At<P>` goes from place to member).
pub trait Has<M: Elem, P: Place>: Project {
    fn pick<C: Carrier>(p: &Self::Prod<C>) -> &C::Of<M>;
    fn pick_mut<C: Carrier>(p: &mut Self::Prod<C>) -> &mut C::Of<M>;
}

impl<M: Elem, T: Project> Has<M, Here> for Cons<M, T> {
    fn pick<C: Carrier>(p: &Self::Prod<C>) -> &C::Of<M> {
        &p.head
    }

    fn pick_mut<C: Carrier>(p: &mut Self::Prod<C>) -> &mut C::Of<M> {
        &mut p.head
    }
}

impl<H: Elem, M: Elem, P: Place, T: Has<M, P>> Has<M, There<P>> for Cons<H, T> {
    fn pick<C: Carrier>(p: &Self::Prod<C>) -> &C::Of<M> {
        T::pick::<C>(&p.tail)
    }

    fn pick_mut<C: Carrier>(p: &mut Self::Prod<C>) -> &mut C::Of<M> {
        T::pick_mut::<C>(&mut p.tail)
    }
}

/// A list whose every member is in `S`, at the places `Ps`: the projection of one
/// list's product out of another's.
pub trait SubOf<S: Project, Ps>: Project {
    fn sub<C: Dup>(s: &S::Prod<C>) -> Self::Prod<C>;
    fn put_row<C: ByRowMut>(s: &mut S::Prod<C>, i: usize, v: &Self::Prod<Val>);
    fn put_vals(s: &mut S::Prod<Val>, v: &Self::Prod<Val>);
}

impl<S: Project> SubOf<S, Empty> for Empty {
    fn sub<C: Dup>(_: &S::Prod<C>) -> PNil {
        PNil
    }

    fn put_row<C: ByRowMut>(_: &mut S::Prod<C>, _: usize, _: &PNil) {}

    fn put_vals(_: &mut S::Prod<Val>, _: &PNil) {}
}

impl<S, H, T, P, Ps> SubOf<S, Cons<P, Ps>> for Cons<H, T>
where
    S: Has<H, P>,
    H: Elem,
    P: Place,
    T: SubOf<S, Ps>,
{
    fn sub<C: Dup>(s: &S::Prod<C>) -> Self::Prod<C> {
        PCons {
            head: C::dup::<H>(S::pick::<C>(s)),
            tail: T::sub::<C>(s),
        }
    }

    fn put_row<C: ByRowMut>(s: &mut S::Prod<C>, i: usize, v: &Self::Prod<Val>) {
        C::set::<H>(S::pick_mut::<C>(s), i, v.head);
        T::put_row::<C>(s, i, &v.tail);
    }

    fn put_vals(s: &mut S::Prod<Val>, v: &Self::Prod<Val>) {
        *S::pick_mut::<Val>(s) = v.head;
        T::put_vals(s, &v.tail);
    }
}

/// The values of a list, read and written by passing the member's marker as a value.
pub struct Vals<L: Project>(pub L::Prod<Val>);

impl<L: Project> Vals<L> {
    pub fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
    {
        *L::pick::<Val>(&self.0)
    }

    pub fn set<M: Elem, P: Place>(&mut self, _member: M, v: M::Value)
    where
        L: Has<M, P>,
    {
        *L::pick_mut::<Val>(&mut self.0) = v;
    }
}
