//! The value kernel, the author's form, and the list of kernels a table is bound against.

use notko::Outcome;

use crate::bind::roles::{Cursor, Formats, ReadRoles};
use crate::bind::{BindRefusal, Binding, Unit, UnitFn};
use crate::domain::{Domain, Reads, Writes};
use crate::list::{Cons, Empty, Values};
use crate::num::{Bool, Count, Index, USize};

/// A kernel's declared identity, plain data an author writes as its `ID`.
///
/// Its representation is unlocked: today a number below 128, a bit of a 128-bit word, because
/// the check that two kernels of one list do not share an id folds those bits. The word is a
/// stand-in for arvo's masks and its width is no interface.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct KernelId(USize);

impl KernelId {
    /// The kernel numbered `n`; `n` must be below 128.
    pub const fn new(n: USize) -> Self {
        assert!(
            n.host() < 128,
            "a KernelId is below 128 while the id check folds a 128-bit word"
        );
        Self(n)
    }

    // FIXME(cc007451af6f): a 128-bit word stands in for a kernel set of any size.
    const fn bit(self) -> u128 {
        1u128 << self.0.host()
    }
}

/// The value kernel: receiver-less, handed the values of its reads, returning the values of its
/// writes. It is handed no position, no range and no view.
pub trait ValueKernel: 'static {
    /// The domain its records are in.
    type Domain: Domain;
    /// `Own`, `Near` and `Param` members: the one read list.
    type Reads: Reads<Self::Domain>;
    /// `Own` members of the same domain.
    type Writes: Writes<Self::Domain>;
    /// Its declared identity.
    const ID: KernelId;
    /// The values of the writes at one record, from the values of the reads there.
    fn at(reads: <Self::Reads as Values>::Of) -> <Self::Writes as Values>::Of;
}

mod sealed {
    use super::*;

    /// The walk over a kernel list: the id fold, and finding a binding's kernel by its id.
    pub trait KernelList<D: Domain> {
        const MASK: u128;
        const DISTINCT: Bool;
        fn check<F: Formats>(
            binding: &Binding<'_>,
            cur: &mut Cursor,
            formats: &F,
        ) -> Outcome<(), BindRefusal>;
        fn read_roles(binding: &Binding<'_>) -> Count;
        fn dispatch<U: UnitFn<D>>(binding: &Binding<'_>, f: &mut U);
    }

    impl<D: Domain> KernelList<D> for Empty {
        const DISTINCT: Bool = Bool::TRUE;
        const MASK: u128 = 0;

        fn check<F: Formats>(
            binding: &Binding<'_>,
            cur: &mut Cursor,
            _formats: &F,
        ) -> Outcome<(), BindRefusal> {
            Outcome::Err(BindRefusal::UnknownKernel {
                binding: cur.binding,
                kernel:  binding.kernel,
            })
        }

        fn read_roles(_binding: &Binding<'_>) -> Count {
            Count::ZERO
        }

        fn dispatch<U: UnitFn<D>>(_row: &Binding<'_>, _f: &mut U) {}
    }

    impl<D: Domain, K: ValueKernel<Domain = D>, T: KernelList<D>> KernelList<D> for Cons<K, T> {
        const DISTINCT: Bool = T::DISTINCT.and(Bool::of(T::MASK & K::ID.bit() == 0));
        const MASK: u128 = T::MASK | K::ID.bit();

        fn check<F: Formats>(
            binding: &Binding<'_>,
            cur: &mut Cursor,
            formats: &F,
        ) -> Outcome<(), BindRefusal> {
            if binding.kernel != K::ID {
                return T::check(binding, cur, formats);
            }
            match K::Reads::check(binding, cur, formats) {
                Outcome::Ok(()) => K::Writes::check(binding, cur, formats),
                refused => refused,
            }
        }

        fn read_roles(binding: &Binding<'_>) -> Count {
            if binding.kernel == K::ID {
                <K::Reads as ReadRoles<D>>::ROLES
            } else {
                T::read_roles(binding)
            }
        }

        fn dispatch<U: UnitFn<D>>(binding: &Binding<'_>, f: &mut U) {
            if binding.kernel != K::ID {
                return T::dispatch(binding, f);
            }
            let mut cur = Cursor::at(Index::FIRST);
            let reads = K::Reads::mint(binding, &mut cur);
            let writes = K::Writes::mint(binding, &mut cur);
            f.unit::<K>(Unit::new(binding, reads, writes));
        }
    }
}

/// A list of value kernels of domain `D`, each with its own id: what a table is bound against.
#[allow(private_bounds)]
pub trait Kernels<D: Domain>: sealed::KernelList<D> {}

impl<D: Domain> Kernels<D> for Empty {}
impl<D: Domain, K: ValueKernel<Domain = D>, T: Kernels<D>> Kernels<D> for Cons<K, T> {}
