//! Each law fails its mutant: the verdict is asserted, not a panic.

mod common;

use common::probe::{HandsStagedAsRead, Probe, SkipsLastRecord, UpdatesInPlace};
use common::{MORSELS, cells_per_frame, digest_stream, isolation, model_frames};
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::{Index, USize};
use hilavitkutin_frozen_implementors::{Forward, Permuted};
use hilavitkutin_laws::{Law, Verdict, Witness, frame_isolation, model, morsel_split_and_order};

#[test]
fn b1_fails_store_skips_last_record() {
    let run = cells_per_frame::<Probe<SkipsLastRecord>, _>(&Forward, MORSELS[1]);
    let verdict = model(&run, &model_frames());
    assert_eq!(
        verdict,
        Verdict::Fails(Law::Model, Witness::Differs {
            stream: Index::FIRST,
            at:     Index::from_host(common::CELLS - 1),
        })
    );
}

#[test]
fn b2_fails_store_updates_in_place() {
    let permuted = Permuted {
        seed: USize::from_host(11),
    };
    let forward = digest_stream::<Probe<UpdatesInPlace>, _>(&Forward, MORSELS[0]);
    let shuffled = digest_stream::<Probe<UpdatesInPlace>, _>(&permuted, MORSELS[0]);
    let verdict = morsel_split_and_order(&[&forward as &[Word], &shuffled]);
    assert!(
        matches!(
            verdict,
            Verdict::Fails(Law::MorselSplitAndOrder, Witness::Differs { .. })
        ),
        "{verdict:?}"
    );
}

#[test]
fn b3_fails_store_updates_in_place() {
    let r = isolation::<Probe<UpdatesInPlace>, _>(&Forward, MORSELS[2]);
    let verdict = frame_isolation(&r.start, &r.end, &[&r.poisoned[0], &r.poisoned[1]]);
    assert!(
        matches!(verdict, Verdict::Fails(Law::FrameIsolation, Witness::Differs { stream, .. }) if stream == Index::FIRST),
        "{verdict:?}"
    );
}

#[test]
fn b3_fails_store_hands_staged_as_read() {
    let r = isolation::<Probe<HandsStagedAsRead>, _>(&Forward, MORSELS[2]);
    let verdict = frame_isolation(&r.start, &r.end, &[&r.poisoned[0], &r.poisoned[1]]);
    assert!(
        matches!(verdict, Verdict::Fails(Law::FrameIsolation, Witness::Differs { stream, .. }) if stream == Index::from_host(2)),
        "{verdict:?}"
    );
}
