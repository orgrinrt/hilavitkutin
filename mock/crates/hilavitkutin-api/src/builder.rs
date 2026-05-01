//! Builder support traits for the scheduler builder's `.build()`.
//!
//! Two sealed traits prove that a registered `Wus` tuple of
//! `WorkUnit` types is satisfied by a `Stores` tuple of
//! registered `Resource<T>` / `Column<T>` / `Virtual<T>` markers.
//!
//! `Buildable<Stores>` reduces per `Wus`-tuple arity into the
//! conjunction of `Stores: WuSatisfied<Wᵢ::Read> +
//! WuSatisfied<Wᵢ::Write>` for every `Wᵢ`. `WuSatisfied<A>`
//! reduces per `A`-tuple arity into the conjunction of `Self:
//! Contains<Tⱼ>` for every `Tⱼ` in `A`. Both arity series cap at
//! 12, matching `AccessSet`.
//!
//! Both traits are sealed via private supertraits in this module;
//! consumers cannot impl them.

use crate::access::{AccessSet, Contains};
use crate::work_unit::WorkUnit;

mod buildable_sealed {
    pub trait Sealed {}
}

mod wu_satisfied_sealed {
    pub trait Sealed<A> {}
}

/// Proof that every WorkUnit in `Wus` has its `Read` and `Write`
/// access sets satisfied by `Stores`.
///
/// The engine's `SchedulerBuilder::build` carries `Wus:
/// Buildable<Stores>` as its where-clause. Per-arity blanket impls
/// (0..=12) reduce this into per-WU `WuSatisfied<...>` proofs.
///
/// Sealed; consumers cannot impl directly.
#[allow(private_bounds)]
pub trait Buildable<Stores: AccessSet>: buildable_sealed::Sealed {}

/// Proof that all members of `A` are present in `Self`.
///
/// `Self` is the registered `Stores` tuple; `A` is a WU's `Read`
/// or `Write` access set. Per-arity blanket impls (0..=12) reduce
/// this into per-store `Self: Contains<Tⱼ>` proofs.
///
/// Sealed; consumers cannot impl directly.
#[allow(private_bounds)]
pub trait WuSatisfied<A: AccessSet>: wu_satisfied_sealed::Sealed<A> {}

// ---------------------------------------------------------------------
// WuSatisfied per `A` arity (0..=12).
// ---------------------------------------------------------------------

// Arity 0: trivially satisfied.
impl<S: AccessSet> wu_satisfied_sealed::Sealed<()> for S {}
impl<S: AccessSet> WuSatisfied<()> for S {}

// Arity 1.
impl<S, T0: 'static> wu_satisfied_sealed::Sealed<(T0,)> for S where S: Contains<T0> {}
impl<S, T0: 'static> WuSatisfied<(T0,)> for S where S: Contains<T0> + AccessSet {}

// Arity 2.
impl<S, T0: 'static, T1: 'static> wu_satisfied_sealed::Sealed<(T0, T1)> for S
where
    S: Contains<T0> + Contains<T1>,
{
}
impl<S, T0: 'static, T1: 'static> WuSatisfied<(T0, T1)> for S
where
    S: Contains<T0> + Contains<T1> + AccessSet,
{
}

// Arity 3.
impl<S, T0: 'static, T1: 'static, T2: 'static> wu_satisfied_sealed::Sealed<(T0, T1, T2)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2>,
{
}
impl<S, T0: 'static, T1: 'static, T2: 'static> WuSatisfied<(T0, T1, T2)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2> + AccessSet,
{
}

// Arity 4.
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static>
    wu_satisfied_sealed::Sealed<(T0, T1, T2, T3)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2> + Contains<T3>,
{
}
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static> WuSatisfied<(T0, T1, T2, T3)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2> + Contains<T3> + AccessSet,
{
}

// Arity 5.
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static>
    wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2> + Contains<T3> + Contains<T4>,
{
}
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static>
    WuSatisfied<(T0, T1, T2, T3, T4)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2> + Contains<T3> + Contains<T4> + AccessSet,
{
}

// Arity 6.
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static>
    wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4, T5)> for S
where
    S: Contains<T0> + Contains<T1> + Contains<T2> + Contains<T3> + Contains<T4> + Contains<T5>,
{
}
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static>
    WuSatisfied<(T0, T1, T2, T3, T4, T5)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + AccessSet,
{
}

// Arity 7.
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static, T6: 'static>
    wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4, T5, T6)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>,
{
}
impl<S, T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static, T6: 'static>
    WuSatisfied<(T0, T1, T2, T3, T4, T5, T6)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + AccessSet,
{
}

// Arity 8.
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
> wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4, T5, T6, T7)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>,
{
}
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
> WuSatisfied<(T0, T1, T2, T3, T4, T5, T6, T7)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + AccessSet,
{
}

// Arity 9.
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
> wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>,
{
}
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
> WuSatisfied<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>
        + AccessSet,
{
}

// Arity 10.
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
> wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>
        + Contains<T9>,
{
}
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
> WuSatisfied<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>
        + Contains<T9>
        + AccessSet,
{
}

// Arity 11.
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
    T10: 'static,
> wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>
        + Contains<T9>
        + Contains<T10>,
{
}
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
    T10: 'static,
> WuSatisfied<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>
        + Contains<T9>
        + Contains<T10>
        + AccessSet,
{
}

// Arity 12.
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
    T10: 'static,
    T11: 'static,
> wu_satisfied_sealed::Sealed<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>
        + Contains<T9>
        + Contains<T10>
        + Contains<T11>,
{
}
impl<
    S,
    T0: 'static,
    T1: 'static,
    T2: 'static,
    T3: 'static,
    T4: 'static,
    T5: 'static,
    T6: 'static,
    T7: 'static,
    T8: 'static,
    T9: 'static,
    T10: 'static,
    T11: 'static,
> WuSatisfied<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for S
where
    S: Contains<T0>
        + Contains<T1>
        + Contains<T2>
        + Contains<T3>
        + Contains<T4>
        + Contains<T5>
        + Contains<T6>
        + Contains<T7>
        + Contains<T8>
        + Contains<T9>
        + Contains<T10>
        + Contains<T11>
        + AccessSet,
{
}

// ---------------------------------------------------------------------
// Buildable per `Wus` arity (0..=12).
// ---------------------------------------------------------------------

// Arity 0: trivially buildable.
impl buildable_sealed::Sealed for () {}
impl<Stores: AccessSet> Buildable<Stores> for () {}

// Helper macro: per-arity Buildable impl.
//
// `impl_buildable!(N; W0, W1, ..., W{N-1});` emits
// `(W0, (W1, (..., (W{N-1}, ()))))` cons-list and the where-clause
// requiring `Stores: WuSatisfied<Wᵢ::Read> + WuSatisfied<Wᵢ::Write>`
// for every `Wᵢ`.
macro_rules! impl_buildable {
    ($($W:ident),+) => {
        impl<$($W: WorkUnit),+> buildable_sealed::Sealed
            for impl_buildable!(@cons $($W),+) {}

        impl<Stores, $($W: WorkUnit),+> Buildable<Stores>
            for impl_buildable!(@cons $($W),+)
        where
            Stores: AccessSet $(
                + WuSatisfied<<$W as WorkUnit>::Read>
                + WuSatisfied<<$W as WorkUnit>::Write>
            )+,
        {
        }
    };
    (@cons $H:ident) => { ($H, ()) };
    (@cons $H:ident, $($T:ident),+) => { ($H, impl_buildable!(@cons $($T),+)) };
}

impl_buildable!(W0);
impl_buildable!(W0, W1);
impl_buildable!(W0, W1, W2);
impl_buildable!(W0, W1, W2, W3);
impl_buildable!(W0, W1, W2, W3, W4);
impl_buildable!(W0, W1, W2, W3, W4, W5);
impl_buildable!(W0, W1, W2, W3, W4, W5, W6);
impl_buildable!(W0, W1, W2, W3, W4, W5, W6, W7);
impl_buildable!(W0, W1, W2, W3, W4, W5, W6, W7, W8);
impl_buildable!(W0, W1, W2, W3, W4, W5, W6, W7, W8, W9);
impl_buildable!(W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10);
impl_buildable!(W0, W1, W2, W3, W4, W5, W6, W7, W8, W9, W10, W11);
