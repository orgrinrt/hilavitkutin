//! The copy half of the host data path: records into a column and out of it.
//!
//! `import` and `dump` move records between the host and a built scheduler's
//! column store between frames. A store is named by its marker type and found
//! through `ColSelector`, the lookup a context projection walks, so the host
//! and a unit resolve one store the same way. `ColMember` with `Kept` is the
//! bound, and only `Column<T>` and `Buffered<T, M>` carry both.

use arvo::{Bool, USize};
use hilavitkutin_api::ColumnStorage;
use hilavitkutin_api::run_cfg::RunCfg;
use hilavitkutin_api::store_values::StoreValues;
use notko::Outcome;
use notko::sink::BulkPush;

use super::Scheduler;
use crate::dispatch::engine_ctx::{ColMember, ColSelector};
use crate::plan::PlanDims;
use crate::plan::project::{Locate, WitnessIndex};
use crate::resource::bindings::{BindingsFor, Kept};

/// Why the host path refused a range.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HostRefused {
    /// The range reaches past the store's record count, which it carries.
    /// Nothing was copied.
    PastCount {
        /// The record count of the store.
        count: USize,
    },
}

/// Whether `len` records from `at` fit inside `count`, asked without a sum
/// that could overflow.
#[inline(always)]
const fn fits(at: USize, len: USize, count: USize) -> Bool {
    Bool(at.0 <= count.0 && len.0 <= count.0 - at.0)
}

impl<
    Cfg: RunCfg,
    WuVals,
    Vals: StoreValues + BindingsFor,
    CS: ColumnStorage,
    D: PlanDims,
    Stores,
    Clk: hilavitkutin_api::platform::ClockApi,
> Scheduler<Cfg, WuVals, Vals, CS, D, Stores, Clk>
{
    /// Copy `records` into the column store `C` from record `at`.
    ///
    /// A buffered column takes the copy on both of its reservations: the
    /// frozen one, which the next pass reads, and the live one, so the
    /// boundary after that pass does not drop it. The store is then marked
    /// changed for the next frame, as `mark_dirty` marks it. A range past the
    /// store's record count is refused before any record moves; an empty one
    /// is accepted and marks nothing. App-only: called between `build` and
    /// `run`, or between two frames.
    pub fn import<C, I, J>(&mut self, at: USize, records: &[C::T]) -> Outcome<(), HostRefused>
    where
        C: ColMember + Kept,
        <Vals as BindingsFor>::Bindings: ColSelector<C::T, I, C::K>,
        Stores: Locate<C, J>,
        J: WitnessIndex,
    {
        let col = ColSelector::<C::T, I, C::K>::get(&self.bindings);
        let len = USize(records.len());
        let count = col.count();
        if !fits(at, len, count).0 {
            return Outcome::Err(HostRefused::PastCount {
                count,
            });
        }
        if len.0 == 0 {
            return Outcome::Ok(());
        }
        let read = col.__read().as_ptr();
        let write = col.__write().as_ptr();
        // SAFETY: both bases name reservations of `count` records of `C::T`
        // made at build, and `at + len <= count` was checked above, so every
        // record written lies inside them. `records` is a host slice and no
        // column memory, so the copy does not overlap. That no unit reads the
        // store during the copy is not established: under `run` and
        // `run_fused` no frame is in flight across `&mut self`, but under
        // `run_parallel` the parked workers hold a shared borrow of the
        // scheduler across it.
        // FIXME(placeholders_tagged_74e2b9918149): under `run_parallel` a
        // parked worker still holds a `&Scheduler` across this `&mut self`,
        // as it does across `replace_value`.
        unsafe {
            core::ptr::copy_nonoverlapping(records.as_ptr(), read.add(at.0), len.0);
            if write != read {
                core::ptr::copy_nonoverlapping(records.as_ptr(), write.add(at.0), len.0);
            }
        }
        // FIXME(a_host_import_marks_its_whole_store_changed): this marks the
        // whole store, so every morsel of every unit reading it reruns; the
        // mark finer than the store waits on the per-morsel generation
        // counters.
        self.mark_dirty::<C, J>();
        Outcome::Ok(())
    }

    /// Push records `at` to `at + len` of the column store `C` into `sink`.
    ///
    /// The records come from the side the next pass reads, which on a
    /// buffered column is the one the last boundary published. The slice the
    /// sink receives is a view of column memory for the length of the call
    /// and no longer. A range past the store's record count is refused before
    /// any record moves. App-only, as `import`.
    pub fn dump<C, I, P>(&self, at: USize, len: USize, sink: &mut P) -> Outcome<(), HostRefused>
    where
        C: ColMember + Kept,
        <Vals as BindingsFor>::Bindings: ColSelector<C::T, I, C::K>,
        P: BulkPush<C::T>,
    {
        let col = ColSelector::<C::T, I, C::K>::get(&self.bindings);
        let count = col.count();
        if !fits(at, len, count).0 {
            return Outcome::Err(HostRefused::PastCount {
                count,
            });
        }
        if len.0 == 0 {
            return Outcome::Ok(());
        }
        // SAFETY: the read base names a reservation of `count` initialised
        // records of `C::T`, and `at + len <= count` was checked above. Between
        // frames nothing writes the store, and the slice does not outlive this
        // call, so no write can alias it while the sink holds it.
        let records =
            unsafe { core::slice::from_raw_parts(col.__read().as_ptr().add(at.0), len.0) };
        sink.push_bulk(records);
        Outcome::Ok(())
    }
}
