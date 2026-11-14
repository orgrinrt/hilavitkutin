//! The carrier of the column swap bench: what a pass costs when the column
//! between a producer and its reader is plain, and when it is buffered and
//! brought in by a copy or by an exchange of its two sides.
//!
//! One carrier, three stores. A producer reads a `Replaceable` value and a
//! column of record numbers and writes the middle column over the whole
//! range. A reader maps the middle column into an output column. The arms
//! differ in the middle column alone: `Column`, `Buffered` under `Snapshot`,
//! `Buffered` under `Exchange`. Over a plain column the reader follows the
//! producer in the same pass. Over a buffered one it reads the frozen side,
//! so it sees a pass's values one pass later, and the boundary brings the
//! live side in between the two.
//!
//! The untimed setup of a frame installs the value again, which marks the
//! producer dirty. In the settled state every timed frame then holds one
//! producer pass, one reader pass and, in the buffered arms, one boundary
//! that brings the column in. The installed value is a pure function of the
//! seed, so the output column holds the same records in every arm and after
//! any number of frames.

use std::cell::RefCell;
use std::rc::Rc;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrNil,
    ColPtrCons,
    ColPtrNil,
    EngineCtx,
    SnapCons,
    SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::store::{Buffered, Column, Exchange, Resource, Snapshot, StartsFromDefault};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::{Written, rec};

use super::{C1, C2, HeapBump, Hints, SwapVal, fnv1a_u32_slice, store};

/// The record's own number, filled once before the first pass.
#[derive(Copy, Clone, Default)]
pub struct Slot(pub u32);

/// A prepared carrier behind type-erased closures.
pub struct PreparedColumn {
    /// The value the carrier is settled on. An arm installs it again before
    /// every timed frame.
    pub settled:   u64,
    /// Untimed per-call setup: install a value, which marks the producer.
    pub install:   Box<dyn FnMut(u64)>,
    /// The timed unit: one frame.
    pub run_frame: Box<dyn FnMut()>,
    /// FNV over the output column for harness validation.
    pub finish:    Box<dyn FnMut() -> u64>,
}

/// The checksum the output column holds once the reader has seen `value`.
pub fn column_checksum(value: u64, records: usize) -> u64 {
    let out: Vec<u32> = (0 .. records as u32)
        .map(|slot| (value as u32).wrapping_add(slot).rotate_left(7))
        .collect();
    fnv1a_u32_slice(&out)
}

macro_rules! carrier {
    ($(#[$doc:meta])* $prepare:ident, $module:ident, $col:ty, $cols:ty, $registered:expr) => {
        mod $module {
            use super::*;

            type Col = $col;
            type Cols = $cols;
            type Feed = Cons<Resource<SwapVal>, Cons<Column<Slot>, Empty>>;
            type Mid = Cons<Col, Empty>;
            // Both writes are listed under `Written`: the store is taken as
            // changed by the write itself, with no compare against the held
            // value, so a frame that writes the same values again still pays
            // its boundary.
            type MidOut = Cons<Written<Col>, Empty>;
            type Out = Cons<Written<Column<C2>>, Empty>;

            // Writes the middle column from the value and the record number.
            struct Produce;
            impl BuilderInput for Produce {
                type Dispatch = UnitDispatch<Self>;
                type Init = Self;
            }
            impl WorkUnit<Always> for Produce {
                type Ctx<'frame> = EngineCtx<
                    'frame,
                    Feed,
                    MidOut,
                    SnapCons<SwapVal, SnapNil>,
                    ColPtrCons<Slot, ColPtrNil>,
                    Cols,
                    AccPtrNil,
                >;
                type Hint = Hints;
                type Read = Feed;
                type Write = MidOut;

                fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                    ctx.each().map(|read| {
                        let seed: SwapVal = read.get(Resource::<SwapVal>::default());
                        let slot = read.get(Column::<Slot>::new());
                        rec![C1((seed.0 as u32).wrapping_add(slot.0))]
                    });
                }
            }

            // Maps the middle column into the output column.
            struct Consume;
            impl BuilderInput for Consume {
                type Dispatch = UnitDispatch<Self>;
                type Init = Self;
            }
            impl WorkUnit<Always> for Consume {
                type Ctx<'frame> = EngineCtx<
                    'frame,
                    Mid,
                    Out,
                    SnapNil,
                    Cols,
                    ColPtrCons<C2, ColPtrNil>,
                    AccPtrNil,
                >;
                type Hint = Hints;
                type Read = Mid;
                type Write = Out;

                fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                    ctx.each()
                        .map(|read| rec![C2(read.get(Col::new()).0.rotate_left(7))]);
                }
            }

            pub fn prepare(seed: u64, records: usize) -> PreparedColumn {
                // Five reservations of `records` four-byte records at most: the
                // record numbers, two sides of the middle column, the output.
                let provider = HeapBump::new(5 * records * 4 + (1 << 16));
                let sched = Scheduler::builder()
                    .with(Column::<Slot>::start())
                    .with($registered)
                    .with(Column::<C2>::start())
                    .with(Resource::new(SwapVal(seed)))
                    .with(Produce)
                    .with(Consume)
                    .build(store(provider), USize(records))
                    .unwrap_or_else(|_| panic!("engine build should succeed"));

                // The bindings lead with the last store registered: the value,
                // the output column, the middle column, the record numbers.
                let slots = sched
                    .__bindings()
                    .__tail()
                    .__tail()
                    .__tail()
                    .__ptr()
                    .as_ptr();
                for i in 0 .. records {
                    // SAFETY: the build reserved `records` records and no
                    // pass has run.
                    unsafe { slots.add(i).write(Slot(i as u32)) };
                }
                let sched = Rc::new(RefCell::new(sched));

                // Settle: a cold pass, then enough passes on the installed
                // value that both sides of a buffered column and the output
                // hold it, and the last pass before the first timed one ran
                // the producer.
                let installed = SwapVal(seed ^ 0x5150_C0DE_5150_C0DE);
                {
                    let mut s = sched.borrow_mut();
                    let _ = s.run();
                    for _ in 0 .. 3 {
                        s.replace_value(installed);
                        let _ = s.run();
                    }
                }

                let install_rc = Rc::clone(&sched);
                let install = Box::new(move |value: u64| {
                    install_rc.borrow_mut().replace_value(SwapVal(value))
                }) as Box<dyn FnMut(u64)>;

                let frame_rc = Rc::clone(&sched);
                let run_frame = Box::new(move || {
                    let r = frame_rc.borrow_mut().run();
                    core::hint::black_box(&r);
                }) as Box<dyn FnMut()>;

                let finish_rc = Rc::clone(&sched);
                let finish = Box::new(move || {
                    let s = finish_rc.borrow();
                    let base = s.__bindings().__tail().__ptr().as_ptr() as *const u32;
                    // SAFETY: the output column holds `records` records the
                    // reader wrote, and no pass is running.
                    let slice = unsafe { core::slice::from_raw_parts(base, records) };
                    fnv1a_u32_slice(slice)
                }) as Box<dyn FnMut() -> u64>;

                PreparedColumn {
                    settled: installed.0,
                    install,
                    run_frame,
                    finish,
                }
            }
        }

        $(#[$doc])*
        pub use $module::prepare as $prepare;
    };
}

carrier!(
    /// The carrier over a plain middle column.
    prepare_column_plain,
    plain,
    Column<C1>,
    ColPtrCons<C1, ColPtrNil>,
    Column::<C1>::start()
);
carrier!(
    /// The carrier over a buffered middle column brought in by a copy.
    prepare_column_snapshot,
    snapshot,
    Buffered<C1, Snapshot>,
    ColPtrCons<C1, ColPtrNil, Snapshot>,
    Buffered::<C1, Snapshot>::start_from(C1(0))
);
carrier!(
    /// The carrier over a buffered middle column brought in by an exchange.
    prepare_column_exchange,
    exchange,
    Buffered<C1, Exchange>,
    ColPtrCons<C1, ColPtrNil, Exchange>,
    Buffered::<C1, Exchange>::start_from(C1(0))
);

/// The seed an arm takes from the harness input: FNV-1a over its bytes.
pub fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xCBF2_9CE4_8422_2325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x0100_0000_01B3);
    }
    acc
}

/// One arm of the column swap bench, whole: the cached carrier and the bench
/// entry around it.
///
/// The harness size `N` is seed-material length and the record count is
/// `N * 1024`. The carrier is built once in a worker process, in the untimed
/// warmup call, and kept by `N`. The untimed setup of a frame installs the
/// settled value again and the timed run block is exactly one frame.
#[macro_export]
macro_rules! column_arm {
    ($name:literal, $entry:ident, $prepare:path) => {
        use ::mockspace_bench_core::{FfiBenchCall, timed};
        use ::mockspace_bench_macro::bench_variant;
        use ::std::cell::RefCell;

        thread_local! {
            static STATE: RefCell<Option<(usize, $crate::PreparedColumn)>> =
                const { RefCell::new(None) };
        }

        #[bench_variant($name, sizes = [64, 1024, 8192])]
        fn $entry<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
            timed! {
                STATE.with(|s| {
                    let mut s = s.borrow_mut();
                    if s.as_ref().map(|(n, _)| *n) != Some(N) {
                        *s = Some((N, $prepare($crate::seed_from_input(input), N * 1024)));
                    }
                });
                setup {
                    STATE.with(|s| {
                        let mut s = s.borrow_mut();
                        let carrier = &mut s.as_mut().unwrap().1;
                        (carrier.install)(carrier.settled);
                    });
                }
                run {
                    STATE.with(|s| (s.borrow_mut().as_mut().unwrap().1.run_frame)());
                }
                STATE.with(|s| {
                    let h = (s.borrow_mut().as_mut().unwrap().1.finish)();
                    output.copy_from_slice(&h.to_le_bytes());
                });
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECORDS: usize = 20_000;
    const SEED: u64 = 0x1234_5678;

    /// Install a value before each frame and name the value the output
    /// column shows after it. `lag` is how many frames the reader trails
    /// the producer.
    fn follows(mut carrier: PreparedColumn, lag: usize) {
        assert_eq!(
            (carrier.finish)(),
            column_checksum(carrier.settled, RECORDS),
            "the carrier is settled on its value"
        );
        let mut seen = vec![carrier.settled; lag + 1];
        for value in [11_u64, 22, 33, 33, 44] {
            (carrier.install)(value);
            (carrier.run_frame)();
            seen.push(value);
            let shown = seen[seen.len() - 1 - lag];
            assert_eq!(
                (carrier.finish)(),
                column_checksum(shown, RECORDS),
                "after the frame that installed {value} the output shows {shown}"
            );
        }
    }

    // Every frame runs the producer, since the output follows each value
    // installed, and the reader, since it shows in the frame the lag gives.
    #[test]
    fn the_reader_of_a_plain_column_shows_a_value_in_its_own_frame() {
        follows(prepare_column_plain(SEED, RECORDS), 0);
    }

    // In the buffered arms the same holds one frame later, which the boundary
    // between two frames has to bring in for.
    #[test]
    fn the_reader_of_a_snapshot_column_shows_a_value_one_frame_later() {
        follows(prepare_column_snapshot(SEED, RECORDS), 1);
    }

    #[test]
    fn the_reader_of_an_exchange_column_shows_a_value_one_frame_later() {
        follows(prepare_column_exchange(SEED, RECORDS), 1);
    }

    #[test]
    fn the_three_carriers_settle_on_one_output() {
        let want = column_checksum(SEED ^ 0x5150_C0DE_5150_C0DE, RECORDS);
        for mut carrier in [
            prepare_column_plain(SEED, RECORDS),
            prepare_column_snapshot(SEED, RECORDS),
            prepare_column_exchange(SEED, RECORDS),
        ] {
            for _ in 0 .. 4 {
                (carrier.install)(carrier.settled);
                (carrier.run_frame)();
                assert_eq!((carrier.finish)(), want);
            }
        }
    }
}
