//! The reference runner against a hand model, and the frames it refuses.

mod common;

use common::{LISTED, Model, RECORDS, Setup, flat, model_frames};
use hilavitkutin_conformance::{
    Ascending,
    Breach,
    Descending,
    Faithful,
    Fold,
    Law,
    Refusal,
    Verdict,
    Word,
    model,
    split,
};

const FRAMES: usize = 5;

#[test]
fn the_runner_keeps_the_model_frame_after_frame() {
    let run = record_run!(Faithful, Ascending, Setup::frames(FRAMES), LISTED).unwrap();
    let expected = model_frames(FRAMES, false);
    for (frame, (ran, modelled)) in run.frames.iter().zip(&expected).enumerate() {
        assert_eq!(model(ran, modelled), Verdict::Holds, "frame {frame}");
    }
    assert_eq!(run.frames.len(), FRAMES);
}

#[test]
fn the_model_walks_every_arm_in_the_frames_compared() {
    // What the comparison above leans on: over the frames it runs, the
    // virtual fires in one frame and not in the others, something is
    // appended, and the snapshot column is written in one frame only.
    let mut model = Model::new();
    let mut fired = Vec::new();
    let mut appended = 0;
    let c_at_start = model.c;
    for _ in 0 .. FRAMES {
        model.frame();
        fired.push(model.loud);
        appended += model.seen.len();
    }
    assert_eq!(fired, [false, true, false, false, false]);
    assert!(appended > 0);
    assert_eq!(model.c, c_at_start.map(|c| c + 1));
}

#[test]
fn what_the_live_sides_held_never_reaches_a_faithful_frame() {
    let clean = record_run!(Faithful, Ascending, Setup::frames(FRAMES), LISTED).unwrap();
    let poisoned = record_run!(
        Faithful,
        Ascending,
        Setup {
            poison: 9_999,
            ..Setup::frames(FRAMES)
        },
        LISTED
    )
    .unwrap();
    assert_eq!(
        split(&[&flat(&clean.frames), &flat(&poisoned.frames)]),
        Verdict::Holds
    );
}

#[test]
fn the_two_visitings_leave_the_same_stores_behind() {
    let sorted = Setup {
        sorted: true,
        ..Setup::frames(FRAMES)
    };
    let up = record_run!(Faithful, Ascending, sorted, LISTED).unwrap();
    let down = record_run!(Faithful, Descending, sorted, LISTED).unwrap();
    assert_eq!(
        split(&[&flat(&up.frames), &flat(&down.frames)]),
        Verdict::Holds
    );
    assert_eq!(
        model(&flat(&down.frames), &flat(&model_frames(FRAMES, true))),
        Verdict::Holds
    );
}

#[test]
fn the_appends_come_out_in_the_sequence_of_the_visiting() {
    // Unsorted, the two visitings differ in the appended values and in
    // nothing ahead of them, which is why a comparison across visitings
    // sorts.
    let up = record_run!(Faithful, Ascending, Setup::frames(1), LISTED).unwrap();
    let down = record_run!(Faithful, Descending, Setup::frames(1), LISTED).unwrap();
    // Four columns, the flag and the count stand ahead of the first value.
    let first_value = 4 * RECORDS + 2;
    assert_eq!(split(&[&up.frames[0], &down.frames[0]]), Verdict::Fails {
        law:    Law::Split,
        breach: Breach::Differs {
            stream: arvo::USize(1),
            at:     arvo::USize(first_value),
        },
    });
}

#[test]
fn a_column_shorter_than_the_count_is_refused() {
    let short = Setup {
        lent: RECORDS - 1,
        ..Setup::frames(1)
    };
    let refused = record_run!(Faithful, Ascending, short, LISTED);
    assert_eq!(refused.err(), Some(Refusal::Short));
}

#[test]
fn an_append_past_the_lent_room_is_refused() {
    // The first frame appends three values.
    let tight = Setup {
        room: 2,
        ..Setup::frames(1)
    };
    let refused = record_run!(Faithful, Ascending, tight, LISTED);
    assert_eq!(refused.err(), Some(Refusal::Full));
    let fits = Setup {
        room: 3,
        ..Setup::frames(1)
    };
    assert!(record_run!(Faithful, Ascending, fits, LISTED).is_ok());
}

#[test]
fn a_virtual_fired_behind_its_consumer_is_refused() {
    use common::{Blur, Bump, Echo, Note, Shout, Walk};
    // `Bump` is listed ahead of `Shout`, the unit that fires its virtual.
    let behind = (Walk, (Blur, (Note, (Bump, (Shout, (Echo, ()))))));
    // The first frame fires nothing, so there is nothing to refuse in it.
    assert!(record_run!(Faithful, Ascending, Setup::frames(1), behind).is_ok());
    // The second frame fires.
    let refused = record_run!(Faithful, Ascending, Setup::frames(2), behind);
    assert_eq!(refused.err(), Some(Refusal::FiredBehind));
}

#[test]
fn the_digest_of_a_run_is_the_fold_of_its_words() {
    let run = record_run!(Faithful, Ascending, Setup::frames(FRAMES), LISTED).unwrap();
    let digest = |frames: &[Vec<Word>]| -> Vec<u64> {
        frames
            .iter()
            .map(|frame| {
                let mut fold = Fold::new();
                fold.words(frame);
                fold.finish().to_raw()
            })
            .collect()
    };
    let ran = digest(&run.frames);
    assert_eq!(ran, digest(&model_frames(FRAMES, false)));
    // No two frames of the run fold to one word.
    let mut distinct = ran.clone();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(distinct.len(), FRAMES);
}
