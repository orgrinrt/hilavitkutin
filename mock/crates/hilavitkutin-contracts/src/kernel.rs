//! The value kernel, the author's form, and the list of kernels a table is bound against.

use notko::Outcome;

use crate::bind::roles::{Formats, MintList};
use crate::bind::{BindRefusal, Binding, UnitFn, check_kernel, unit_of};
use crate::domain::{Domain, Reads, Roles, Writes};
use crate::list::{Cons, Empty, Values};
use crate::num::{Bool, ColumnNo, Ordinal, USize};

/// A kernel's declared identity, plain data an author writes as its `ID` and a binding names.
///
/// Its representation is unlocked. Any number is an id, and a binding naming one no kernel of the
/// list declares is refused as a value. The kernels of a bound list declare ids below 128, since
/// the check that two of them do not share an id folds those bits into a 128-bit word: an author's
/// id of 128 or more fails the build where its list is bound. The word is a stand-in for arvo's
/// masks and its width is no interface.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct KernelId(USize);

impl KernelId {
    /// The kernel numbered `n`.
    pub const fn new(n: USize) -> Self {
        Self(n)
    }

    const fn fits(self) -> Bool {
        self.0.lt(USize::from_host(128))
    }

    // FIXME(cc007451af6f): a 128-bit word stands in for a kernel set of any size.
    const fn bit(self) -> u128 {
        if self.fits().get() { 1u128 << self.0.host() } else { 0 }
    }
}

/// The value kernel: receiver-less, handed the values of its reads, returning the values of its
/// writes. It is handed no position, no range and no view.
pub trait ValueKernel: 'static {
    /// The domain its records are in.
    type Domain: Domain;
    /// The columns it names, each once: its roles, which a binding fills one stored column each.
    type Roles: Roles<Self::Domain>;
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

    /// The walk over a kernel list: the id fold, and finding a binding's kernel by its id. `P`
    /// carries the places of each kernel's members in its role list.
    pub trait KernelList<D: Domain, P> {
        const MASK: u128;
        const DISTINCT: Bool;
        const FITS: Bool;
        fn check<F: Formats>(
            binding: &Binding<'_>,
            at: Ordinal,
            formats: &F,
        ) -> Outcome<(), BindRefusal>;
        fn writes<F: FnMut(ColumnNo)>(binding: &Binding<'_>, f: &mut F);
        fn dispatch<'t, U: UnitFn<'t, D>>(binding: &'t Binding<'t>, f: &mut U);
    }

    impl<D: Domain> KernelList<D, ()> for Empty {
        const DISTINCT: Bool = Bool::TRUE;
        const FITS: Bool = Bool::TRUE;
        const MASK: u128 = 0;

        fn check<F: Formats>(
            binding: &Binding<'_>,
            at: Ordinal,
            _formats: &F,
        ) -> Outcome<(), BindRefusal> {
            Outcome::Err(BindRefusal::UnknownKernel {
                binding: at,
                kernel:  binding.kernel,
            })
        }

        fn writes<F: FnMut(ColumnNo)>(_binding: &Binding<'_>, _f: &mut F) {}

        fn dispatch<'t, U: UnitFn<'t, D>>(_binding: &'t Binding<'t>, _f: &mut U) {}
    }

    impl<D: Domain, K: ValueKernel<Domain = D>, T: KernelList<D, PT>, PR, PW, PT>
        KernelList<D, ((PR, PW), PT)> for Cons<K, T>
    where
        K::Reads: MintList<D, K::Roles, PR>,
        K::Writes: MintList<D, K::Roles, PW>,
    {
        const DISTINCT: Bool = T::DISTINCT.and(Bool::of(T::MASK & K::ID.bit() == 0));
        const FITS: Bool = T::FITS.and(K::ID.fits());
        const MASK: u128 = T::MASK | K::ID.bit();

        fn check<F: Formats>(
            binding: &Binding<'_>,
            at: Ordinal,
            formats: &F,
        ) -> Outcome<(), BindRefusal> {
            if binding.kernel != K::ID {
                return T::check(binding, at, formats);
            }
            check_kernel::<D, K, PR, PW>(binding, at, formats)
        }

        fn writes<F: FnMut(ColumnNo)>(binding: &Binding<'_>, f: &mut F) {
            if binding.kernel != K::ID {
                return T::writes(binding, f);
            }
            <K::Writes as MintList<D, K::Roles, PW>>::columns(binding, f);
        }

        fn dispatch<'t, U: UnitFn<'t, D>>(binding: &'t Binding<'t>, f: &mut U) {
            if binding.kernel != K::ID {
                return T::dispatch(binding, f);
            }
            f.unit::<K>(unit_of::<D, K, PR, PW>(binding));
        }
    }
}

/// A list of value kernels of domain `D`, each with its own id: what a table is bound against. `P`
/// is the places of each kernel's members in its role list, which the compiler infers where the
/// list is bound and which no one writes out.
#[allow(private_bounds)]
pub trait Kernels<D: Domain, P>: sealed::KernelList<D, P> {}

impl<D: Domain> Kernels<D, ()> for Empty {}
impl<D: Domain, K: ValueKernel<Domain = D>, T: Kernels<D, PT>, PR, PW, PT>
    Kernels<D, ((PR, PW), PT)> for Cons<K, T>
where
    K::Reads: MintList<D, K::Roles, PR>,
    K::Writes: MintList<D, K::Roles, PW>,
{
}
