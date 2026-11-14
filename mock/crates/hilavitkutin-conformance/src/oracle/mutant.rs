//! The mutants: each keeps a frame wrongly in one named way.
//!
//! A law that cannot fail holds nothing. Each mutant here is run through the
//! laws in place of [`Faithful`](super::Faithful) and has to fail the ones
//! named for it. They ship in the crate so that a consumer who holds a runner
//! of its own to the laws has the failing cases to hand as well.

use arvo::Bool;

use super::keeping::Keeping;
use super::sealed;

macro_rules! mutants {
    ($($(#[$doc:meta])* $name:ident sets $wrong:ident;)*) => {$(
        $(#[$doc])*
        pub struct $name;

        impl sealed::OracleSealed for $name {}

        impl Keeping for $name {
            const $wrong: Bool = Bool(true);
        }
    )*};
}

mutants! {
    /// A read of a buffered column sees the live side. Fails the model and
    /// the isolation law.
    ReadsLive sets READS_LIVE;
    /// A write to a buffered column lands on the frozen side, in place. Fails
    /// the model, the split and the isolation law.
    WritesFrozen sets WRITES_FROZEN;
    /// The boundary brings in a live side nobody wrote. Fails the model and
    /// the isolation law.
    BringsUnwritten sets BRINGS_UNWRITTEN;
    /// The last record visited is left out. Fails the model.
    SkipsLast sets SKIPS_LAST;
    /// An accumulator is not emptied at the start of a frame. Fails the
    /// model.
    KeepsAppends sets KEEPS_APPENDS;
    /// A virtual stays fired across the boundary. Fails the model.
    KeepsFlags sets KEEPS_FLAGS;
    /// An `On` unit runs whether its virtual fired or not. Fails the model.
    RunsUntriggered sets RUNS_UNTRIGGERED;
}
