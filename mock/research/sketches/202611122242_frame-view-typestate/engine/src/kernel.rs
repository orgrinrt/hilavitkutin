//! The two step kernel forms, the close unit, and the one shape the runner
//! drives both kernel forms through.

use crate::lane::{FrameList, LandedList};
use crate::registry::{RawLanes, Registry};
use crate::sealed::Sealed;
use crate::stage::Stage;
use crate::view::{Landed, Row, View};

/// The staged handles of a write list, one per member in declared order.
pub type Staged<'s, W> = <W as FrameList>::Parts<'s>;

/// The values a value kernel returns for one row, one per member of its write
/// list in declared order.
pub type Values<W> = <W as FrameList>::Values;

/// The block kernel: handed the committed view and one write-only staged
/// handle per lane of its write list, and nothing else.
pub trait BlockKernel<Reg: Registry> {
    type Reads: FrameList;
    type Writes: FrameList;

    fn run<'s>(view: View<'s, Reg, Self::Reads>, out: Staged<'s, Self::Writes>);
}

/// The value kernel: handed the committed view and a row, returns the row's
/// values for its write list, and is handed no staged handle.
pub trait ValueKernel<Reg: Registry> {
    type Reads: FrameList;
    type Writes: FrameList;

    fn row(view: &View<'_, Reg, Self::Reads>, row: Row) -> Values<Self::Writes>;
}

/// The close unit: runs after the step has landed, names landed stores, and
/// is handed those stores and nothing else. It never sees the frame's view.
pub trait CloseUnit<Reg: Registry> {
    type Stores: LandedList;

    fn close(landed: Landed<'_, Reg, Self::Stores>);
}

/// A block kernel, as the runner takes it.
pub struct Block<K>(pub K);

/// A value kernel, as the runner takes it.
pub struct Value<K>(pub K);

/// What the runner drives: either kernel form, with its write list placed in
/// the registry at `Ps`.
///
/// Sealed (`sealed.rs`): implemented only for `Block` and `Value`, whose masks
/// are their write lists' `Stage::MASK`. An author implements `BlockKernel` or
/// `ValueKernel` and never this.
pub trait Kernel<Reg: Registry, Ps>: Sealed {
    /// The write list's position mask (`Stage::MASK`).
    const MASK: u128;
    /// The write list's length (`Stage::LEN`).
    const LEN: u32;

    /// Runs the kernel for one step.
    ///
    /// # Safety
    ///
    /// As `Stage::parts`: no other live reference reaches a lane in `MASK`,
    /// and the write list names each lane once.
    unsafe fn run_in<'s>(view: Reg::Shared<'s>, rows: u32, raw: &RawLanes<'s, Reg>);
}

impl<Reg, K, Ps> Kernel<Reg, Ps> for Block<K>
where
    Reg: Registry,
    K: BlockKernel<Reg>,
    K::Writes: Stage<Reg, Ps>,
{
    const LEN: u32 = <K::Writes as Stage<Reg, Ps>>::LEN;
    const MASK: u128 = <K::Writes as Stage<Reg, Ps>>::MASK;

    unsafe fn run_in<'s>(view: Reg::Shared<'s>, rows: u32, raw: &RawLanes<'s, Reg>) {
        // SAFETY: passed on from the caller unchanged.
        let out = unsafe { <K::Writes as Stage<Reg, Ps>>::parts(raw) };
        K::run(View::new(view, rows), out);
    }
}

impl<Reg, K, Ps> Kernel<Reg, Ps> for Value<K>
where
    Reg: Registry,
    K: ValueKernel<Reg>,
    K::Writes: Stage<Reg, Ps>,
{
    const LEN: u32 = <K::Writes as Stage<Reg, Ps>>::LEN;
    const MASK: u128 = <K::Writes as Stage<Reg, Ps>>::MASK;

    unsafe fn run_in<'s>(view: Reg::Shared<'s>, rows: u32, raw: &RawLanes<'s, Reg>) {
        // SAFETY: passed on from the caller unchanged.
        let mut out = unsafe { <K::Writes as Stage<Reg, Ps>>::parts(raw) };
        let view = View::new(view, rows);
        for row in view.rows() {
            let values = K::row(&view, row);
            <K::Writes as FrameList>::put(&mut out, row, values);
        }
    }
}
