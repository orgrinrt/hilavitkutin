//! Each law fails its mutants, with the witness asserted, not a panic. The honest probe the
//! mutants are cut from holds every law in `laws.rs`.

mod common;

use common::probe::{
    HandsStagedAsRead,
    IgnoresColumn,
    Probe,
    SkipsLastRecord,
    SwapDropsUnwritten,
    UpdatesInPlace,
};
use common::{
    CELLS,
    HEAT,
    Input,
    MORSELS,
    STILL,
    cells_per_frame,
    digest_stream,
    first_of,
    isolation,
    model_frames,
};
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::{Count, Ordinal, USize};
use hilavitkutin_frozen_implementors::{Forward, Permuted};
use hilavitkutin_laws::{Law, Verdict, Witness, frame_isolation, model, morsel_split_and_order};

fn b1<S: common::Cells>() -> Verdict {
    let run = cells_per_frame::<S, _>(&Forward, MORSELS[1], Input::Scramble);
    model(&run, &model_frames(Input::Scramble))
}

fn b3<S: common::Cells>() -> Verdict {
    let r = isolation::<S, _>(&Forward, MORSELS[2]);
    frame_isolation(&r.start, &r.end, &[&r.poisoned[0], &r.poisoned[1]])
}

#[test]
fn b1_fails_store_skips_last_record() {
    assert_eq!(
        b1::<Probe<SkipsLastRecord>>(),
        Verdict::Fails(Law::Model, Witness::Differs {
            stream: Ordinal::FIRST,
            at:     first_of(HEAT).after(Count::from_host(CELLS - 1)),
        })
    );
}

#[test]
fn b1_fails_store_ignores_the_column() {
    assert_eq!(
        b1::<Probe<IgnoresColumn>>(),
        Verdict::Fails(Law::Model, Witness::Differs {
            stream: Ordinal::FIRST,
            at:     first_of(HEAT),
        }),
        "the writes land in column 0, which no swap commits, so the heat is never written"
    );
}

#[test]
fn b1_fails_swap_drops_unwritten_columns() {
    assert_eq!(
        b1::<Probe<SwapDropsUnwritten>>(),
        Verdict::Fails(Law::Model, Witness::Differs {
            stream: Ordinal::FIRST,
            at:     first_of(STILL),
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
            Verdict::Fails(Law::MorselSplitAndOrder, Witness::Differs { stream, .. })
                if stream == Ordinal::from_host(1)
        ),
        "{verdict:?}"
    );
}

#[test]
fn b3_fails_store_updates_in_place() {
    let verdict = b3::<Probe<UpdatesInPlace>>();
    assert!(
        matches!(verdict, Verdict::Fails(Law::FrameIsolation, Witness::Differs { stream, .. }) if stream == Ordinal::FIRST),
        "{verdict:?}"
    );
}

#[test]
fn b3_fails_store_hands_staged_as_read() {
    let verdict = b3::<Probe<HandsStagedAsRead>>();
    assert!(
        matches!(verdict, Verdict::Fails(Law::FrameIsolation, Witness::Differs { stream, .. }) if stream == Ordinal::from_host(2)),
        "{verdict:?}"
    );
}

#[test]
fn b3_fails_swap_drops_unwritten_columns() {
    assert_eq!(
        b3::<Probe<SwapDropsUnwritten>>(),
        Verdict::Fails(Law::FrameIsolation, Witness::Differs {
            stream: Ordinal::from_host(2),
            at:     first_of(STILL),
        })
    );
}
