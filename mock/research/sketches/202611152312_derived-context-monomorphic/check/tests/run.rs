//! The block units of `unit`, registered with the scheduler of the engine copy
//! through its `Blk` and run by `Scheduler::run`.
//!
//! Two holds. The first and the last frame against values worked by hand.
//! Every frame against the same three units written the way the engine takes
//! them today (`native`: a `WorkUnit` naming `CtxFor`, the body on the engine's
//! keyed accessors), under a second scheduler built the same way. A frame after the
//! first skips units whose inputs did not change, and nothing outside the
//! frames changes them here, so before each later frame both schedulers are
//! marked dirty through the engine's hidden test accessor, and every unit runs
//! in every frame.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::Blk;
use hilavitkutin::resource::ColumnPtr;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::store::{Column, StartsFromDefault};
use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};
use native::{NDrift, NStamp, NWarm};
use notko::Outcome;
use unit::{Drift, Glow, Pos, Seen, Stamp, Vel, Warm};

/// Records in the frame: enough for the scheduler to cut more than one window.
const N: usize = 4096;
/// Frames run.
const FRAMES: usize = 4;

fn provider() -> StaticBumpMemory {
    // A byte that is not zero, so a record left unwritten does not read as a
    // value worked by hand.
    StaticBumpMemory::new(std::vec![MaybeUninit::new(0xA5u8); 1 << 20].leak())
}

fn store(provider: StaticBumpMemory) -> ArenaColumnStorage<StaticBumpMemory> {
    ArenaColumnStorage::new(provider)
}

fn vel_of(i: usize) -> i64 {
    3 * i as i64 - 7
}

// ---------------------------------------------------------------------
// Building, seeding and reading a scheduler.
// ---------------------------------------------------------------------

/// Build a scheduler over the four columns and the three given units, and
/// seed `Pos` and `Vel`. Columns registered Seen, Glow, Vel, Pos: Pos is the
/// bindings head, Vel one tail down, Glow two, Seen three. Units registered
/// stamp, drift, warm: drift writes the Pos that warm reads, so the carrier
/// is topological.
macro_rules! seeded {
    ($stamp:expr, $drift:expr, $warm:expr) => {{
        let sched = Scheduler::builder()
            .with(Column::<Seen>::start())
            .with(Column::<Glow>::start())
            .with(Column::<Vel>::start())
            .with(Column::<Pos>::start())
            .with($stamp)
            .with($drift)
            .with($warm)
            .build(store(provider()), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        let pos: ColumnPtr<Pos> = sched.__bindings().__ptr();
        let vel: ColumnPtr<Vel> = sched.__bindings().__tail().__ptr();
        for i in 0 .. N {
            // SAFETY: both columns hold N reserved records and the scheduler
            // is alive; each slot is written once, before any frame runs.
            unsafe {
                *pos.as_ptr().add(i) = Pos(i as i64);
                *vel.as_ptr().add(i) = Vel(vel_of(i));
            }
        }
        sched
    }};
}

/// The three written columns of a scheduler built by `seeded!`, copied out.
macro_rules! stores {
    ($sched:expr) => {{
        let b = $sched.__bindings();
        let pos: ColumnPtr<Pos> = b.__ptr();
        let glow: ColumnPtr<Glow> = b.__tail().__tail().__ptr();
        let seen: ColumnPtr<Seen> = b.__tail().__tail().__tail().__ptr();
        // SAFETY: each column holds N reserved records; the scheduler is alive.
        unsafe {
            (
                core::slice::from_raw_parts(pos.as_ptr(), N).to_vec(),
                core::slice::from_raw_parts(glow.as_ptr(), N).to_vec(),
                core::slice::from_raw_parts(seen.as_ptr(), N).to_vec(),
            )
        }
    }};
}

#[test]
fn block_units_run_under_the_scheduler() {
    let mut block = seeded!(Blk(Stamp), Blk(Drift), Blk(Warm));
    let mut native = seeded!(NStamp, NDrift, NWarm);

    // The first frame, against the hand.
    assert!(
        matches!(block.run(), Outcome::Ok(())),
        "the block frame completes"
    );
    assert!(
        matches!(native.run(), Outcome::Ok(())),
        "the native frame completes"
    );
    let (pos, glow, seen) = stores!(block);
    for i in 0 .. N {
        let want = i as i64 + vel_of(i);
        assert_eq!(
            pos[i],
            Pos(want),
            "position at record {i} after the first frame"
        );
        assert_eq!(
            glow[i],
            Glow(2 * want),
            "glow at record {i} after the first frame"
        );
    }

    // Every record was stamped with its index inside its window, and the
    // indices start over at least once: the frame ran in more than one window,
    // so the reads and writes above were offset by each window's start.
    let restarts = (1 .. N).filter(|&i| seen[i] == Seen(0)).count();
    assert!(
        restarts >= 1,
        "the frame ran in one window only; window offsets were not exercised"
    );
    let mut expect = 0i64;
    for i in 0 .. N {
        if seen[i] == Seen(0) {
            expect = 0;
        }
        assert_eq!(seen[i], Seen(expect), "stamp at record {i}");
        expect += 1;
    }
    std::println!("windows in the first frame: {}", restarts + 1);

    // Every frame, against the native form.
    assert_eq!(
        stores!(block),
        stores!(native),
        "frame 1: the block units leave the native stores"
    );
    for frame in 2 ..= FRAMES {
        block.__mark_store_dirty();
        native.__mark_store_dirty();
        assert!(
            matches!(block.run(), Outcome::Ok(())),
            "the block frame completes"
        );
        assert!(
            matches!(native.run(), Outcome::Ok(())),
            "the native frame completes"
        );
        assert_eq!(
            stores!(block),
            stores!(native),
            "frame {frame}: the block units leave the native stores"
        );
    }
    let (pos, glow, _) = stores!(block);
    for i in 0 .. N {
        let want = i as i64 + FRAMES as i64 * vel_of(i);
        assert_eq!(
            pos[i],
            Pos(want),
            "position at record {i} after the last frame"
        );
        assert_eq!(
            glow[i],
            Glow(2 * want),
            "glow at record {i} after the last frame"
        );
    }
    std::println!(
        "record 5 after {FRAMES} frames: {:?} (seed 5, velocity {})",
        pos[5],
        vel_of(5)
    );
}
