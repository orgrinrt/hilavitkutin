//! Seat 02 phase two. Question: is a per-cell width bound what keeps the
//! morsel formula's promise (a fiber's write set for one morsel fits the
//! usable L1)?
//!
//! Seat 01's c1 shows a 4 KiB cell breaking it, and says "The record's 16-byte
//! bound is what keeps the formula meaningful". The trunk's step 9 sums the
//! fiber's write set (hilavitkutin/src/plan/steps/step9_morsel.rs:40-56) and
//! clamps the window below at MIN_MORSEL (:73-75). So the quantity that breaks
//! the promise is the fiber's summed write width, not any one cell's.
//!
//! Three arms, each on the trunk planner at its default run config:
//! - control: one 16-byte write column. Expected green.
//! - forty 16-byte write columns in one unit, every cell inside R3's cap.
//!   Expected red if the per-cell cap is not what keeps the promise.
//! - two 384-byte write columns, every cell inside seat 01's derived bound
//!   `size_of <= L1_USABLE / MIN_MORSEL`. Expected red for the same reason.

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{ColumnWriterApi, EachApi, HasColumnWriter, HasEach};
use hilavitkutin_api::run_cfg::{DefaultRunCfg, RunCfg};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use s7_write_set_not_cell_width::ZeroedHeap;

fn store<M: hilavitkutin_api::MemoryProviderApi>(m: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(m)
}

macro_rules! cells {
    ($w:expr; $($n:ident),*) => { $( #[derive(Copy, Clone)] struct $n([u64; $w]); )* };
}
macro_rules! cols {
    () => { Empty };
    ($h:ident $(, $t:ident)*) => { Cons<Column<$h>, cols!($($t),*)> };
}
macro_rules! ptrs {
    () => { ColPtrNil };
    ($h:ident $(, $t:ident)*) => { ColPtrCons<$h, ptrs!($($t),*)> };
}

type Hints = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);

/// One unit that writes every listed column, and a test that reads the
/// planner's window for its fiber and checks the morsel's write set.
macro_rules! arm {
    ($test:ident, $unit:ident, $w:expr; $($n:ident),*) => {
        cells!($w; $($n),*);
        struct $unit;
        impl BuilderInput for $unit {
            type Dispatch = UnitDispatch<Self>;
            type Init = Self;
        }
        impl WorkUnit<Always> for $unit {
            type Ctx<'f> = EngineCtx<'f, Empty, cols!($($n),*), SnapNil, ColPtrNil, ptrs!($($n),*)>;
            type Hint = Hints;
            type Read = Empty;
            type Write = cols!($($n),*);
            fn execute<'f>(&self, ctx: &Self::Ctx<'f>) {
                EachApi::run(ctx.each(), |i| unsafe {
                    $( ctx.writer().write::<$n, _>(i, $n([i.0 as u64; $w])); )*
                });
            }
        }
        #[test]
        fn $test() {
            let s = Scheduler::builder()
                $( .with(Column::<$n>::new()) )*
                .with($unit)
                .build(store(ZeroedHeap), USize(4096))
                .unwrap_or_else(|_| panic!("build"));
            let window = s.__fiber_morsel_size(USize(0)).0;
            let per_record = 0 $( + core::mem::size_of::<$n>() )*;
            let widest = 0 $( .max(core::mem::size_of::<$n>()) )*;
            let write_bytes = window * per_record;
            let budget = <DefaultRunCfg as RunCfg>::L1_USABLE.0;
            let min = <DefaultRunCfg as RunCfg>::MIN_MORSEL.0;
            println!(
                "{}: widest cell {} B, write set {} B a record, window {} records (MIN_MORSEL {}), morsel write set {} B, L1 budget {} B",
                stringify!($test), widest, per_record, window, min, write_bytes, budget
            );
            assert!(
                write_bytes <= budget,
                "{}: every cell is {} B or less and the morsel still writes {} B against {} B",
                stringify!($test), widest, write_bytes, budget
            );
        }
    };
}

arm!(control_one_16_byte_write_column_fits, UnitControl, 2; K0);

arm!(forty_16_byte_write_columns_each_inside_r3_fit, UnitR3, 2;
    R00, R01, R02, R03, R04, R05, R06, R07, R08, R09,
    R10, R11, R12, R13, R14, R15, R16, R17, R18, R19,
    R20, R21, R22, R23, R24, R25, R26, R27, R28, R29,
    R30, R31, R32, R33, R34, R35, R36, R37, R38, R39);

arm!(two_384_byte_write_columns_each_inside_the_derived_bound_fit, UnitDerived, 48; D0, D1);
