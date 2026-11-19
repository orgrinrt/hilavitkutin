//! Attempt c1, behind the feature `attempt-read-vals-on-the-read-set`: the
//! read values bounded on `ReadSet`, as the shipped `ReadVals` is, over the
//! read product on the carrier. Built once, and its compiler output is kept
//! under `diagnostics/`.

use notko_hlist::Place;

use crate::values::{Read, ReadProd, ReadSet, Readable};

/// `ReadVals`, bounded on the read set and not on one frame's product.
pub struct ReadValsOnSet<'a, R: ReadSet>(pub ReadProd<'a, R>);

impl<'a, R: ReadSet> ReadValsOnSet<'a, R> {
    /// The value of `member` at this record.
    pub fn get<M: Readable, P: Place>(&self, member: M) -> M::Val<'a>
    where
        R: model::Has<Read<'a>, M, P>,
    {
        core::mem::forget(member);
        *<R as model::Has<Read<'a>, M, P>>::pick(&self.0)
    }
}
