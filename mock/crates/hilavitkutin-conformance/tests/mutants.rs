//! Every mutant fails the laws named for it, at the place it gets wrong.
//!
//! The faithful runner holds each law over the same runs, in the first test
//! of each group, so a failure below is the mutant's and not the harness's.
//! A frame's words are the four columns of six records, the flag, the count
//! of appends and then the appended values, so word 6 is the first `B`, word
//! 12 the first `C` and word 24 the flag.

mod common;

use arvo::USize;
use common::{LISTED, Setup, flat, model_frames};
use hilavitkutin_conformance::mutant::{
    BringsUnwritten,
    KeepsAppends,
    KeepsFlags,
    ReadsLive,
    RunsUntriggered,
    SkipsLast,
    WritesFrozen,
};
use hilavitkutin_conformance::{
    Ascending,
    Breach,
    Descending,
    Faithful,
    Law,
    Verdict,
    Word,
    isolation,
    model,
    split,
};

const FRAMES: usize = 4;

/// The first frame whose words are not the model's, and the verdict there.
fn first_failure(frames: &[Vec<Word>]) -> Option<(usize, Verdict)> {
    let expected = model_frames(frames.len(), false);
    frames
        .iter()
        .zip(&expected)
        .map(|(ran, modelled)| model(ran, modelled))
        .enumerate()
        .find(|(_, verdict)| *verdict != Verdict::Holds)
}

fn differs(law: Law, stream: usize, at: usize) -> Verdict {
    Verdict::Fails {
        law,
        breach: Breach::Differs {
            stream: USize(stream),
            at:     USize(at),
        },
    }
}

macro_rules! model_of {
    ($keeping:ty) => {
        first_failure(
            &record_run!($keeping, Ascending, Setup::frames(FRAMES), LISTED)
                .unwrap()
                .frames,
        )
    };
}

#[test]
fn the_faithful_runner_holds_the_model() {
    assert_eq!(model_of!(Faithful), None);
}

#[test]
fn reading_the_live_side_fails_the_model_at_the_first_buffered_value() {
    // `Blur` reads the live side, which began as zeros, so the first `B` of
    // the first frame is wrong.
    assert_eq!(model_of!(ReadsLive), Some((0, differs(Law::Model, 0, 6))));
}

#[test]
fn writing_the_frozen_side_fails_the_model_at_the_second_buffered_value() {
    // The first record still reads untouched neighbours. The second reads
    // the first one's new value where the frozen one belongs.
    assert_eq!(
        model_of!(WritesFrozen),
        Some((0, differs(Law::Model, 0, 7)))
    );
}

#[test]
fn bringing_an_unwritten_side_fails_the_model_at_the_snapshot_column() {
    // Nothing writes `C` in the first frame, and the boundary copies its
    // live side in all the same.
    assert_eq!(
        model_of!(BringsUnwritten),
        Some((0, differs(Law::Model, 0, 12)))
    );
}

#[test]
fn skipping_the_last_record_fails_the_model_at_that_record() {
    assert_eq!(model_of!(SkipsLast), Some((0, differs(Law::Model, 0, 5))));
}

#[test]
fn keeping_the_appends_fails_the_model_in_the_second_frame() {
    // The first frame's three appends are still there, so the second frame
    // records six values where the model has three.
    assert_eq!(
        model_of!(KeepsAppends),
        Some((1, Verdict::Fails {
            law:    Law::Model,
            breach: Breach::Length {
                stream: USize(0),
            },
        }))
    );
}

#[test]
fn keeping_the_flags_fails_the_model_in_the_frame_after_the_firing() {
    // The virtual fires in the second frame. Kept fired, it runs `Bump` in
    // the third, which moves `C` where the model leaves it.
    assert_eq!(model_of!(KeepsFlags), Some((2, differs(Law::Model, 0, 12))));
}

#[test]
fn running_an_untriggered_unit_fails_the_model_in_the_first_frame() {
    assert_eq!(
        model_of!(RunsUntriggered),
        Some((0, differs(Law::Model, 0, 12)))
    );
}

macro_rules! split_of {
    ($keeping:ty) => {{
        let sorted = Setup {
            sorted: true,
            ..Setup::frames(1)
        };
        let up = record_run!($keeping, Ascending, sorted, LISTED).unwrap();
        let down = record_run!($keeping, Descending, sorted, LISTED).unwrap();
        split(&[&flat(&up.frames), &flat(&down.frames)])
    }};
}

#[test]
fn the_faithful_runner_holds_the_split_law() {
    assert_eq!(split_of!(Faithful), Verdict::Holds);
}

#[test]
fn writing_the_frozen_side_fails_the_split_law() {
    // Visited from the far end, the first record reads a neighbour that was
    // already written in place.
    assert_eq!(split_of!(WritesFrozen), differs(Law::Split, 1, 6));
}

macro_rules! isolation_of {
    ($keeping:ty) => {{
        let poisoned = |poison| {
            Setup {
                poison,
                ..Setup::frames(1)
            }
        };
        let one = record_run!($keeping, Ascending, poisoned(0), LISTED).unwrap();
        let other = record_run!($keeping, Ascending, poisoned(7), LISTED).unwrap();
        isolation(&one.began[0], &one.before[0], &[
            &one.committed[0],
            &other.committed[0],
        ])
    }};
}

#[test]
fn the_faithful_runner_holds_the_isolation_law() {
    assert_eq!(isolation_of!(Faithful), Verdict::Holds);
}

#[test]
fn reading_the_live_side_fails_isolation_across_the_poisoned_runs() {
    // The frozen sides stand untouched until the boundary, and what is
    // committed then depends on what the live side began with.
    assert_eq!(isolation_of!(ReadsLive), differs(Law::Isolation, 2, 0));
}

#[test]
fn writing_the_frozen_side_fails_isolation_before_the_boundary() {
    assert_eq!(isolation_of!(WritesFrozen), differs(Law::Isolation, 0, 0));
}

#[test]
fn bringing_an_unwritten_side_fails_isolation_at_the_snapshot_column() {
    // The six `B` words are the same in both runs. The first `C` is whatever
    // the live side began with.
    assert_eq!(
        isolation_of!(BringsUnwritten),
        differs(Law::Isolation, 2, 6)
    );
}
