//! The registry: the one list of every lane the frame holds, and the storage,
//! shared, exclusive and raw shapes derived from it.

use core::marker::PhantomData;

use notko_hlist::{Cons, Empty, Here, List, There};

use crate::lane::Lane;

/// A value carrying the lane it belongs to in its type, so a list of them can
/// be searched by lane.
pub struct Tag<X, V> {
    v:     V,
    _lane: PhantomData<fn() -> X>,
}

impl<X, V> Tag<X, V> {
    const fn new(v: V) -> Self {
        Tag {
            v,
            _lane: PhantomData,
        }
    }
}

impl<X, V: Copy> Clone for Tag<X, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<X, V: Copy> Copy for Tag<X, V> {}

/// The one lane `X` of a tagged list, found at position `P`.
///
/// `P` is inferred at the call: a lane present once has exactly one position
/// and the solver finds it; a lane absent has none, which is an `E0277` at the
/// call naming the lane.
#[diagnostic::on_unimplemented(
    message = "the registry has no lane `{X}`",
    note = "A lane is read or staged only when the registry lists it."
)]
pub trait Pick<X, P> {
    /// What the list holds for that lane.
    type V;
    fn pick(&self) -> &Self::V;
    fn pick_mut(&mut self) -> &mut Self::V;
}

impl<X, V, T> Pick<X, Here> for (Tag<X, V>, T) {
    type V = V;

    fn pick(&self) -> &V {
        &self.0.v
    }

    fn pick_mut(&mut self) -> &mut V {
        &mut self.0.v
    }
}

impl<X, H, T: Pick<X, P>, P> Pick<X, There<P>> for (H, T) {
    type V = T::V;

    fn pick(&self) -> &T::V {
        self.1.pick()
    }

    fn pick_mut(&mut self) -> &mut T::V {
        self.1.pick_mut()
    }
}

/// N records of one lane, owned by the frame.
pub struct Buf<X: Lane, const N: usize>([X::Item; N]);

impl<X: Lane, const N: usize> Clone for Buf<X, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<X: Lane, const N: usize> Copy for Buf<X, N> {}

/// The registry list and every shape the frame derives from it.
///
/// Implemented for `Empty` and `Cons<Lane, Registry>`, so an author's registry
/// is a type alias and nothing else.
pub trait Registry: List {
    /// The frame's storage, N rows per lane.
    type Bufs<const N: usize>: Copy;
    /// A shared slice per lane, which is what the view holds.
    type Shared<'s>: Copy;
    /// An exclusive slice per lane, which is what a close unit is handed.
    type Excl<'s>;
    /// A raw slice pointer per lane, from which the staged handles are cut.
    type Raw: Copy;

    fn init<const N: usize>() -> Self::Bufs<N>;
    fn shared<const N: usize>(bufs: &Self::Bufs<N>) -> Self::Shared<'_>;
    fn excl<const N: usize>(bufs: &mut Self::Bufs<N>) -> Self::Excl<'_>;
    fn raw<const N: usize>(bufs: &mut Self::Bufs<N>) -> Self::Raw;
}

impl Registry for Empty {
    type Bufs<const N: usize> = ();
    type Excl<'s> = ();
    type Raw = ();
    type Shared<'s> = ();

    fn init<const N: usize>() {}

    fn shared<const N: usize>(_bufs: &()) {}

    fn excl<const N: usize>(_bufs: &mut ()) {}

    fn raw<const N: usize>(_bufs: &mut ()) {}
}

impl<H: Lane, T: Registry> Registry for Cons<H, T> {
    type Bufs<const N: usize> = (Buf<H, N>, T::Bufs<N>);
    type Excl<'s> = (Tag<H, &'s mut [H::Item]>, T::Excl<'s>);
    type Raw = (Tag<H, *mut [H::Item]>, T::Raw);
    type Shared<'s> = (Tag<H, &'s [H::Item]>, T::Shared<'s>);

    fn init<const N: usize>() -> Self::Bufs<N> {
        (Buf([H::INIT; N]), T::init())
    }

    fn shared<const N: usize>(bufs: &Self::Bufs<N>) -> Self::Shared<'_> {
        (Tag::new(&bufs.0.0[..]), T::shared(&bufs.1))
    }

    fn excl<const N: usize>(bufs: &mut Self::Bufs<N>) -> Self::Excl<'_> {
        (Tag::new(&mut bufs.0.0[..]), T::excl(&mut bufs.1))
    }

    fn raw<const N: usize>(bufs: &mut Self::Bufs<N>) -> Self::Raw {
        let head: *mut [H::Item] = &mut bufs.0.0[..];
        (Tag::new(head), T::raw(&mut bufs.1))
    }
}

/// The raw lanes of the staging buffers, held for `'s`: the borrow of the
/// buffers the pointers came from lives exactly as long as this does, so no
/// staged handle cut from it outlives the step.
pub struct RawLanes<'s, Reg: Registry> {
    pub(crate) lanes: Reg::Raw,
    _borrow:          PhantomData<&'s mut ()>,
}

impl<'s, Reg: Registry> RawLanes<'s, Reg> {
    pub(crate) fn new<const N: usize>(bufs: &'s mut Reg::Bufs<N>) -> Self {
        RawLanes {
            lanes:   Reg::raw(bufs),
            _borrow: PhantomData,
        }
    }
}
