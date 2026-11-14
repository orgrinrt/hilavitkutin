//! The column family of members, and the lookup of a column by type.
//!
//! A unit lists a column store bare, or under `Reach`, `Written` or
//! `Differs`. Every one of those names one store of one value type and one
//! kind, and `ColMember` says which. The bundle folds and the projections
//! take a member through it, so a wrapper needs no arm of its own anywhere.
//!
//! `ColSelector` finds a column in the bindings or in a projected bundle by
//! its value type and its kind together. A `Column<T>` and a `Buffered<T, M>`
//! of one `T` are two stores, and neither answers for the other.

use arvo::Cap;
use hilavitkutin_api::store::{BufferMode, Buffered, Column};
use hilavitkutin_api::{ColumnValue, Differs, Reach, Written};

use super::{ColPtrCons, ColRef, Here, There};
use crate::resource::bindings::{
    AccumBinding,
    ColKind,
    ColumnBinding,
    Plain,
    ResourceBinding,
    VirtualBinding,
};

/// A member of an access set that stands for a column store.
///
/// Implemented for `Column<T>` and `Buffered<T, M>`, and for `Reach<C, N>`,
/// `Written<C>` and `Differs<C>` over either. No other member has it, which
/// is what keeps the column arm of a fold apart from the resource, the
/// accumulator and the virtual arm.
pub trait ColMember {
    /// The value type of the store.
    type T: ColumnValue;
    /// The kind of the store: `Plain`, or the buffer mode.
    type K: ColKind;
}

impl<T: ColumnValue> ColMember for Column<T> {
    type K = Plain;
    type T = T;
}

impl<T: ColumnValue, M: BufferMode + ColKind> ColMember for Buffered<T, M> {
    type K = M;
    type T = T;
}

impl<C: ColMember, const N: Cap> ColMember for Reach<C, N> {
    type K = C::K;
    type T = C::T;
}

impl<C: ColMember> ColMember for Written<C> {
    type K = C::K;
    type T = C::T;
}

impl<C: ColMember> ColMember for Differs<C> {
    type K = C::K;
    type T = C::T;
}

/// Lookup by type yielding the `ColRef<T>` of the column of value type `T`
/// and kind `K`.
///
/// `Index` is `Here` (the head is that column) or `There<I>` (it is `I`
/// steps into the tail). `K` defaults to `Plain`, the kind every positional
/// accessor resolves through, so a buffered column is out of their reach.
pub trait ColSelector<T, Index, K: ColKind = Plain> {
    /// The column's bases, its changed mark and its record count.
    fn get(&self) -> ColRef<T>;
}

// Over the projected column bundle.

impl<T, Tail, K: ColKind> ColSelector<T, Here, K> for ColPtrCons<T, Tail, K> {
    #[inline(always)]
    fn get(&self) -> ColRef<T> {
        self.head
    }
}

impl<T, U, Tail, I, K: ColKind, J: ColKind> ColSelector<T, There<I>, K> for ColPtrCons<U, Tail, J>
where
    Tail: ColSelector<T, I, K>,
{
    #[inline(always)]
    fn get(&self) -> ColRef<T> {
        self.tail.get()
    }
}

// Over the scheduler's bindings: the list that resolves resources through
// `Selector` resolves columns through this. `Here` matches a column node of
// the same value type and kind, and `There<I>` passes over a node of any
// sort.

impl<T, Tail, K: ColKind> ColSelector<T, Here, K> for ColumnBinding<T, Tail, K> {
    #[inline(always)]
    fn get(&self) -> ColRef<T> {
        ColRef {
            read:  self.__frozen(),
            write: self.__live(),
            mark:  self.__mark(),
            count: self.__count(),
        }
    }
}

impl<T, U, Tail, I, K: ColKind, J: ColKind> ColSelector<T, There<I>, K>
    for ColumnBinding<U, Tail, J>
where
    Tail: ColSelector<T, I, K>,
{
    #[inline(always)]
    fn get(&self) -> ColRef<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I, K: ColKind> ColSelector<T, There<I>, K> for ResourceBinding<U, Tail>
where
    Tail: ColSelector<T, I, K>,
{
    #[inline(always)]
    fn get(&self) -> ColRef<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I, K: ColKind> ColSelector<T, There<I>, K> for VirtualBinding<U, Tail>
where
    Tail: ColSelector<T, I, K>,
{
    #[inline(always)]
    fn get(&self) -> ColRef<T> {
        self.__tail().get()
    }
}

impl<T, U, Tail, I, K: ColKind> ColSelector<T, There<I>, K> for AccumBinding<U, Tail>
where
    Tail: ColSelector<T, I, K>,
{
    #[inline(always)]
    fn get(&self) -> ColRef<T> {
        self.__tail().get()
    }
}
