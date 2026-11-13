//! The walk from a binding to its unit's handles, kept out of reach: a runner meets the handles
//! only inside a [`Unit`](super::Unit), paired with the view of the store they were checked
//! against.

use super::Binding;
use super::roles::ReadRoles;
use crate::domain::Domain;
use crate::kernel::ValueKernel;

/// The handles of one unit's reads and writes, minted from its binding. Crate-private: a runner
/// reaches them only through a [`Unit`](super::Unit), which pairs them with the view of their own store.
pub struct Handles<'t, K: ValueKernel> {
    pub(crate) binding: &'t Binding<'t>,
    pub(crate) reads:   <K::Reads as ReadRoles<K::Domain>>::Handles,
    pub(crate) writes:  <K::Writes as ReadRoles<K::Domain>>::Handles,
}

impl<K: ValueKernel> Clone for Handles<'_, K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K: ValueKernel> Copy for Handles<'_, K> {}

impl<'t, K: ValueKernel> Handles<'t, K> {
    pub(crate) const fn new(
        binding: &'t Binding<'t>,
        reads: <K::Reads as ReadRoles<K::Domain>>::Handles,
        writes: <K::Writes as ReadRoles<K::Domain>>::Handles,
    ) -> Self {
        Self {
            binding,
            reads,
            writes,
        }
    }
}

/// What the kernel walk hands each unit's handles to. Crate-private: the two public ways in are
/// [`Bound::each`](super::Bound::each) and [`Units::next`](super::Units::next), which add the
/// committed view.
pub trait Each<'t, D: Domain> {
    /// One unit's handles, its kernel as a type.
    fn handles<K: ValueKernel<Domain = D>>(&mut self, handles: Handles<'t, K>);
}
