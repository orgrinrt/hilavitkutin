//! Value-carrying registration list for the scheduler builder.
//!
//! Distinct from the type-level `AccessSet` cons-list (which tracks
//! store identity): `StageList` carries the actual registered values
//! so they survive from `.with` to `build()` instead of being dropped
//! at the call site. The arena drain (HILA-RUNTIME-C6) walks this list
//! to move each value into scheduler-owned storage.

mod sealed {
    pub trait Sealed {}
}

/// A value-carrying registration list.
///
/// Sealed: only `StageEmpty` and `Stage` inhabit it, so the builder's
/// staged accumulator cannot be forged by a consumer.
pub trait StageList: sealed::Sealed {}

/// The empty staged list, the builder's initial state.
pub struct StageEmpty;

impl sealed::Sealed for StageEmpty {}
impl StageList for StageEmpty {}

/// One staged value `head` of type `H`, followed by the rest, `tail`.
///
/// `.with` prepends a node per registration. The fields are read by
/// the arena drain (HILA-RUNTIME-C6) and by the retention test; the
/// node owns `head` so the registered value stays alive until
/// `build()`.
#[allow(dead_code)] // head/tail are moved into the arena by HILA-RUNTIME-C6 and read by the retention test; the node owns the value to keep it alive
pub struct Stage<H, T: StageList> {
    pub(crate) head: H,
    pub(crate) tail: T,
}

impl<H, T: StageList> sealed::Sealed for Stage<H, T> {}
impl<H, T: StageList> StageList for Stage<H, T> {}

#[cfg(test)]
mod tests {
    use super::{Stage, StageEmpty};

    #[test]
    fn stage_carries_value() {
        let s = Stage {
            head: 42u32,
            tail: StageEmpty,
        };
        assert_eq!(s.head, 42u32);
    }
}
