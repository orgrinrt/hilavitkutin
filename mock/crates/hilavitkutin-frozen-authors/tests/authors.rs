//! The frozen authors under the hand model, the reference runner and the
//! scheduler.
//!
//! The hand model is what the reference runner is held to, and the reference
//! runner is what each entry of the scheduler is held to. Every comparison is
//! a law of `hilavitkutin-conformance` over the words `record::words` wrote.
//!
//! Where an entry of the scheduler leaves the reference runner, the case is
//! here as an ignored test that names its ticket, and the units the entry
//! does keep are held beside it.

mod common;

use arvo::USize;
use common::{Entry, FRAMES, N, POISON, first_failing, flat, model_frames, store};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::store::Column;
use hilavitkutin_conformance::mutant::{ReadsLive, SkipsLast};
use hilavitkutin_conformance::{
    Ascending,
    Breach,
    Descending,
    Faithful,
    Law,
    Verdict,
    Word,
    model,
    split,
};
use hilavitkutin_frozen_authors::{
    Alarm,
    Ash,
    Crest,
    Cross,
    Drift,
    Fade,
    Glow,
    LIMIT,
    Model,
    Pos,
    RECORDS,
    Smooth,
    Spread,
    Tally,
    Warm,
};
use notko::Outcome;

/// The frames of a run under the scheduler, and of the same units under the
/// reference runner.
type Pair = (Vec<Vec<Word>>, Vec<Vec<Word>>);

/// The nine units, in the sequence they are registered.
fn all(entry: Entry, poison: u32, sorted: bool) -> Pair {
    both!(with_cross, entry, poison, sorted;
        Drift, Spread, Cross, Alarm, Tally, Smooth, Crest, Warm, Fade)
}

/// The units without `Tally`, the one a trigger schedules.
fn untriggered(entry: Entry, poison: u32, sorted: bool) -> Pair {
    both!(with_cross, entry, poison, sorted;
        Drift, Spread, Cross, Alarm, Smooth, Crest, Warm, Fade)
}

/// The units without `Alarm` and `Tally`: nothing fires and nothing waits on
/// a fire.
fn unfired(entry: Entry, poison: u32, sorted: bool) -> Pair {
    both!(with_cross, entry, poison, sorted;
        Drift, Spread, Cross, Smooth, Crest, Warm, Fade)
}

/// The units without `Cross`, the one that appends, and with no accumulator
/// registered.
fn all_without_cross(entry: Entry, poison: u32) -> Pair {
    both!(without_cross, entry, poison, false;
        Drift, Spread, Alarm, Tally, Smooth, Crest, Warm, Fade)
}

/// The units without `Cross` and `Tally`, with no accumulator registered.
fn untriggered_without_cross(entry: Entry, poison: u32) -> Pair {
    both!(without_cross, entry, poison, false;
        Drift, Spread, Alarm, Smooth, Crest, Warm, Fade)
}

fn reference(sorted: bool) -> Vec<Vec<Word>> {
    oracle_frames!(
        Faithful,
        Ascending,
        FRAMES,
        0,
        sorted,
        units!(
            Drift, Spread, Cross, Alarm, Tally, Smooth, Crest, Warm, Fade
        )
    )
    .expect("no refusal")
}

fn reference_without_cross() -> Vec<Vec<Word>> {
    oracle_frames!(
        Faithful,
        Ascending,
        FRAMES,
        0,
        false,
        units!(Drift, Spread, Alarm, Tally, Smooth, Crest, Warm, Fade)
    )
    .expect("no refusal")
}

fn assert_keeps(run: &[Vec<Word>], want: &[Vec<Word>], what: &str) {
    let failing = first_failing(run, want);
    let wrong = failing.map(|(frame, _)| common::wrong(&run[frame], &want[frame]));
    assert_eq!(
        failing, None,
        "{what} leaves the reference at this frame, counted from zero; the first words that \
         differ, as (store, record, got, wanted): {wrong:?}"
    );
}

fn assert_pair((run, want): Pair, what: &str) {
    assert_keeps(&run, &want, what);
}

// The reference runner on the frozen authors is the hand model, frame for
// frame.
#[test]
fn the_reference_runner_keeps_the_hand_model() {
    assert_keeps(
        &reference(false),
        &model_frames(FRAMES, false, true),
        "the reference runner",
    );
    assert_keeps(
        &reference_without_cross(),
        &model_frames(FRAMES, false, false),
        "the reference runner without `Cross`",
    );
}

// Visiting the last record first leaves the same stores, with the crossed
// values in another sequence.
#[test]
fn the_last_record_first_leaves_the_same_stores() {
    let nine = units!(
        Drift, Spread, Cross, Alarm, Tally, Smooth, Crest, Warm, Fade
    );
    let down = oracle_frames!(Faithful, Descending, FRAMES, 0, true, nine).expect("no refusal");
    let up = reference(true);
    assert_eq!(split(&[&flat(&up), &flat(&down)]), Verdict::Holds);

    let unsorted =
        oracle_frames!(Faithful, Descending, FRAMES, 0, false, nine).expect("no refusal");
    assert_ne!(
        split(&[&flat(&reference(false)), &flat(&unsorted)]),
        Verdict::Holds,
        "the appends of the two visitings come in different sequences"
    );
}

// The frames the comparisons run over take every arm of the units: a frame
// that fires `Overheat` and one that does not, a frame that appends, a pool
// that changes and then goes quiet.
#[test]
fn the_frames_walk_every_arm() {
    let mut model = Model::new();
    let mut fired = Vec::new();
    let mut appended = Vec::new();
    let mut levels_moved = Vec::new();
    for _ in 0 .. FRAMES {
        let before = model;
        model.frame();
        fired.push(model.overheat.0);
        appended.push(model.crossed.0);
        levels_moved.push((0 .. N).any(|i| before.level[i] != model.level[i]));
    }
    assert!(fired.contains(&true) && fired.contains(&false), "{fired:?}");
    assert!(!fired[0] && fired[FRAMES - 1], "{fired:?}");
    assert!(
        appended.iter().filter(|n| **n > 0).count() > 1,
        "{appended:?}"
    );
    assert_eq!(levels_moved[.. 3], [true, true, false], "{levels_moved:?}");
    assert!(!levels_moved[FRAMES - 1]);
    let alarms = fired.iter().filter(|f| **f).count() as u32;
    assert!((0 .. N).all(|i| model.alarms[i].0.to_raw() == alarms));
    assert!((0 .. N).all(|i| model.ash[i].0.to_raw() == model.glow[i].0.to_raw() + LIMIT.to_raw()));
}

// The comparison can fail: a runner that reads the live side of a buffered
// column, and one that leaves the last record out, leave the hand model in
// the first frame.
#[test]
fn a_wrong_runner_is_caught_on_the_authors() {
    let want = model_frames(FRAMES, false, true);
    let nine = units!(
        Drift, Spread, Cross, Alarm, Tally, Smooth, Crest, Warm, Fade
    );

    let live =
        oracle_frames!(ReadsLive, Ascending, FRAMES, POISON, false, nine).expect("no refusal");
    assert!(matches!(
        first_failing(&live, &want),
        Some((0, Verdict::Fails {
            law:    Law::Model,
            breach: Breach::Differs {
                stream: USize(0),
                ..
            },
        }))
    ));

    let short = oracle_frames!(SkipsLast, Ascending, FRAMES, 0, false, nine).expect("no refusal");
    assert_eq!(
        first_failing(&short, &want),
        Some((0, Verdict::Fails {
            law:    Law::Model,
            breach: Breach::Differs {
                stream: USize(0),
                at:     USize(N - 1),
            },
        }))
    );
}

// `run`, with the positions marked changed by the host ahead of every frame,
// keeps every frame of the units no trigger schedules.
#[test]
fn run_keeps_the_frames_where_the_host_marks_the_positions() {
    assert_pair(untriggered(Entry::RunMarked, 0, false), "`run`");
    assert_pair(untriggered_without_cross(Entry::RunMarked, 0), "`run`");
}

// With nothing marked by the host, `Drift` writes the positions it reads and
// is rested from the second frame on.
#[test]
#[ignore = "catalogue: a unit's own write of a plain column it reads seeds no next pass; tracked #hilavitkutin_reads_a_write_s_changed_mark_for_maintenance_un"]
fn run_keeps_the_frames_where_the_host_marks_nothing() {
    assert_pair(untriggered(Entry::Run, 0, false), "`run`");
}

// `Tally` runs in the frames `Overheat` fires, and in no other.
#[test]
#[ignore = "catalogue: the incremental skip rests a unit in the pass its trigger fires; tracked #hilavitkutin_a_unit_scheduled_on_a_virtual_takes_no_edge_fro"]
fn run_runs_the_triggered_unit_in_the_frame_its_trigger_fires() {
    assert_pair(all(Entry::RunMarked, 0, false), "`run`");
    assert_pair(all_without_cross(Entry::RunMarked, 0), "`run`");
}

// With an accumulator in the carrier `run_parallel` walks each unit over a
// slice of the records per worker. Where nothing fires it keeps the frames
// at one, two and four workers; the appends are compared sorted.
#[test]
fn run_parallel_keeps_the_frames_on_one_worker() {
    assert_pair(unfired(Entry::Parallel(1), 0, true), "`run_parallel`");
}

#[test]
fn run_parallel_keeps_the_frames_on_two_workers() {
    assert_pair(unfired(Entry::Parallel(2), 0, true), "`run_parallel`");
}

#[test]
fn run_parallel_keeps_the_frames_on_four_workers() {
    assert_pair(unfired(Entry::Parallel(4), 0, true), "`run_parallel`");
}

#[test]
#[ignore = "catalogue: beside an accumulator the triggered unit runs whether or not its trigger fired; tracked #hilavitkutin_beside_an_accumulator_run_parallel_never_asks_t"]
fn run_parallel_gates_the_triggered_unit_beside_an_accumulator_on_one_worker() {
    assert_pair(all(Entry::Parallel(1), 0, true), "`run_parallel`");
}

#[test]
#[ignore = "catalogue: beside an accumulator the triggered unit runs whether or not its trigger fired; tracked #hilavitkutin_beside_an_accumulator_run_parallel_never_asks_t"]
fn run_parallel_gates_the_triggered_unit_beside_an_accumulator_on_two_workers() {
    assert_pair(all(Entry::Parallel(2), 0, true), "`run_parallel`");
}

#[test]
#[ignore = "catalogue: beside an accumulator the triggered unit runs whether or not its trigger fired; tracked #hilavitkutin_beside_an_accumulator_run_parallel_never_asks_t"]
fn run_parallel_gates_the_triggered_unit_beside_an_accumulator_on_four_workers() {
    assert_pair(all(Entry::Parallel(4), 0, true), "`run_parallel`");
}

// The host reads whether `Overheat` fired in a frame off the scheduler's own
// binding.
#[test]
#[ignore = "catalogue: beside an accumulator a fire stamps a per-core copy the host never reads; tracked #hilavitkutin_beside_an_accumulator_run_parallel_never_asks_t"]
fn a_fire_beside_an_accumulator_reaches_the_host() {
    assert_pair(untriggered(Entry::Parallel(1), 0, true), "`run_parallel`");
}

// Without the accumulator `run_parallel` takes its phase loop, and keeps the
// frames of the units no trigger schedules at one, two and four workers.
#[test]
fn the_phase_loop_keeps_the_frames_on_one_worker() {
    assert_pair(
        untriggered_without_cross(Entry::Parallel(1), 0),
        "`run_parallel`",
    );
}

#[test]
fn the_phase_loop_keeps_the_frames_on_two_workers() {
    assert_pair(
        untriggered_without_cross(Entry::Parallel(2), 0),
        "`run_parallel`",
    );
}

#[test]
fn the_phase_loop_keeps_the_frames_on_four_workers() {
    assert_pair(
        untriggered_without_cross(Entry::Parallel(4), 0),
        "`run_parallel`",
    );
}

// One worker walks the carrier in the sequence registered, the firer ahead
// of the unit its trigger schedules, and keeps all eight units.
#[test]
fn the_phase_loop_runs_the_triggered_unit_on_one_worker() {
    assert_pair(all_without_cross(Entry::Parallel(1), 0), "`run_parallel`");
}

#[test]
#[ignore = "catalogue: nothing orders a firer ahead of the unit its trigger schedules across cores; tracked #hilavitkutin_a_unit_scheduled_on_a_virtual_takes_no_edge_fro"]
fn the_phase_loop_runs_the_triggered_unit_on_two_workers() {
    for _ in 0 .. 8 {
        assert_pair(all_without_cross(Entry::Parallel(2), 0), "`run_parallel`");
    }
}

#[test]
#[ignore = "catalogue: nothing orders a firer ahead of the unit its trigger schedules across cores; tracked #hilavitkutin_a_unit_scheduled_on_a_virtual_takes_no_edge_fro"]
fn the_phase_loop_runs_the_triggered_unit_on_four_workers() {
    for _ in 0 .. 8 {
        assert_pair(all_without_cross(Entry::Parallel(4), 0), "`run_parallel`");
    }
}

// However the records are split, the frames are the same: the split law over
// every entry at once.
#[test]
fn every_split_of_the_records_leaves_the_same_frames() {
    let runs = [
        flat(&unfired(Entry::RunMarked, 0, true).0),
        flat(&unfired(Entry::Parallel(1), 0, true).0),
        flat(&unfired(Entry::Parallel(2), 0, true).0),
        flat(&unfired(Entry::Parallel(4), 0, true).0),
    ];
    let streams: Vec<&[Word]> = runs.iter().map(Vec::as_slice).collect();
    assert_eq!(split(&streams), Verdict::Holds);
}

// What the live side of a buffered column holds before the first frame never
// reaches a frame: a run over poisoned live sides is the run over clean ones.
#[test]
fn a_poisoned_live_side_never_reaches_run() {
    assert_pair(untriggered(Entry::RunMarked, POISON, true), "`run`");
}

#[test]
fn a_poisoned_live_side_never_reaches_run_parallel() {
    assert_pair(
        unfired(Entry::Parallel(4), POISON, true),
        "`run_parallel` on 4",
    );
}

#[test]
fn a_poisoned_live_side_never_reaches_the_phase_loop() {
    assert_pair(
        untriggered_without_cross(Entry::Parallel(4), POISON),
        "`run_parallel` on 4 without an accumulator",
    );
}

/// `Warm` and `Fade` alone: glow and ash after one frame through `$entry`.
macro_rules! warm_and_fade {
    ($entry:ident) => {{
        let mut scheduler = Scheduler::builder()
            .with(Column::<Pos>::new())
            .with(Column::<Glow>::new())
            .with(Column::<Ash>::new())
            .with(Warm)
            .with(Fade)
            .build(store(), RECORDS)
            .unwrap_or_else(|_| panic!("the two units build"));
        let start = Model::new();
        let ash = scheduler.__bindings().__ptr().as_ptr();
        let glow = scheduler.__bindings().__tail().__ptr().as_ptr();
        let pos = scheduler.__bindings().__tail().__tail().__ptr().as_ptr();
        for i in 0 .. N {
            // SAFETY: the three reservations, before the first frame.
            unsafe {
                pos.add(i).write(start.pos[i]);
                glow.add(i).write(start.glow[i]);
                ash.add(i).write(start.ash[i]);
            }
        }
        assert!(matches!(scheduler.$entry(), Outcome::Ok(())));
        // SAFETY: the frame is over, and both columns hold `N` records.
        let seen: (Vec<Glow>, Vec<Ash>) = unsafe {
            (
                (0 .. N).map(|i| glow.add(i).read()).collect(),
                (0 .. N).map(|i| ash.add(i).read()).collect(),
            )
        };
        seen
    }};
}

/// Glow and ash after one frame of `Warm` and `Fade` under the reference
/// runner.
fn warm_and_fade_reference() -> (Vec<Glow>, Vec<Ash>) {
    let want: Vec<Word> = oracle_frames!(Faithful, Ascending, 1, 0, false, units!(Warm, Fade))
        .expect("no refusal")
        .remove(0);
    // glow and ash sit behind the positions, the heat, the alarm counts, the
    // levels and the peaks in a frame's words
    let glow_from = 5 * N;
    let glow: Vec<Glow> = (0 .. N)
        .map(|i| Glow(common::num(want[glow_from + i].to_raw() as u32)))
        .collect();
    let ash: Vec<Ash> = (0 .. N)
        .map(|i| Ash(common::num(want[glow_from + N + i].to_raw() as u32)))
        .collect();
    let by_hand: Vec<u32> = (0 .. N as u32).map(|i| i + i).collect();
    assert!(
        glow.iter()
            .zip(&by_hand)
            .all(|(glow, hand)| glow.0.to_raw() == *hand),
        "the reference runner doubles the positions"
    );
    assert!(
        ash.iter()
            .zip(&by_hand)
            .all(|(ash, hand)| ash.0.to_raw() == *hand + LIMIT.to_raw()),
        "and adds the limit"
    );
    (glow, ash)
}

// `Warm` and `Fade` are a carrier the scheduler fuses into one loop. `run`
// leaves both columns as the reference runner does, and the fused entry
// leaves the last one so.
#[test]
fn the_fused_entry_keeps_the_last_column() {
    let (glow, ash) = warm_and_fade_reference();
    let run = warm_and_fade!(run);
    assert!(run.0 == glow && run.1 == ash, "`run`");
    let fused = warm_and_fade!(run_fused);
    assert!(fused.1 == ash, "`run_fused`");
}

#[test]
#[ignore = "catalogue: the fused entry leaves the column between two fused units unwritten; tracked #hilavitkutin_run_fused_leaves_the_column_between_two_fused_u"]
fn the_fused_entry_stores_the_column_between_the_two_units() {
    let (glow, _) = warm_and_fade_reference();
    let fused = warm_and_fade!(run_fused);
    assert!(
        fused.0 == glow,
        "`run_fused` leaves the glow the reference runner leaves"
    );
}

// The laws hold the words, so a law over two runs of one entry has to hold
// before a law between two entries says anything.
#[test]
fn a_run_is_the_same_twice() {
    let once = flat(&unfired(Entry::Parallel(4), 0, true).0);
    let twice = flat(&unfired(Entry::Parallel(4), 0, true).0);
    assert_eq!(model(&once, &twice), Verdict::Holds);
}
