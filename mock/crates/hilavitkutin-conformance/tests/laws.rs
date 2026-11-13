//! The laws over hand-written records: each verdict, and where it points.

use arvo::USize;
use arvo_hash::{Fnv1a, Hasher};
use hilavitkutin_conformance::{Breach, Fold, Law, Verdict, Word, isolation, model, split};

fn words(raw: &[u64]) -> Vec<Word> {
    raw.iter().map(|w| Word::from_raw(*w)).collect()
}

fn fails(law: Law, breach: Breach) -> Verdict {
    Verdict::Fails {
        law,
        breach,
    }
}

fn differs(stream: usize, at: usize) -> Breach {
    Breach::Differs {
        stream: USize(stream),
        at:     USize(at),
    }
}

fn length(stream: usize) -> Breach {
    Breach::Length {
        stream: USize(stream),
    }
}

fn too_few(given: usize) -> Breach {
    Breach::TooFew {
        given: USize(given),
        needs: USize(2),
    }
}

#[test]
fn the_model_law_holds_over_equal_words_and_over_none() {
    assert_eq!(
        model(&words(&[1, 2, 3]), &words(&[1, 2, 3])),
        Verdict::Holds
    );
    assert_eq!(model(&[], &[]), Verdict::Holds);
}

#[test]
fn the_model_law_names_the_first_word_that_differs() {
    assert_eq!(
        model(&words(&[1, 9, 9]), &words(&[1, 2, 3])),
        fails(Law::Model, differs(0, 1))
    );
}

#[test]
fn the_model_law_names_a_run_of_another_length() {
    // A run that is the model's prefix is not the model.
    assert_eq!(
        model(&words(&[1, 2]), &words(&[1, 2, 3])),
        fails(Law::Model, length(0))
    );
}

#[test]
fn the_split_law_needs_two_runs() {
    assert_eq!(split(&[]), fails(Law::Split, too_few(0)));
    assert_eq!(split(&[&words(&[1])]), fails(Law::Split, too_few(1)));
}

#[test]
fn the_split_law_holds_over_equal_runs() {
    let run = words(&[4, 5, 6]);
    assert_eq!(split(&[&run, &run, &run]), Verdict::Holds);
}

#[test]
fn the_split_law_names_the_first_run_that_differs_from_the_reference() {
    let reference = words(&[4, 5, 6]);
    let same = words(&[4, 5, 6]);
    let other = words(&[4, 5, 7]);
    assert_eq!(
        split(&[&reference, &same, &other]),
        fails(Law::Split, differs(2, 2))
    );
    assert_eq!(
        split(&[&reference, &words(&[4, 5])]),
        fails(Law::Split, length(1))
    );
}

#[test]
fn the_isolation_law_needs_two_poisoned_runs() {
    let sides = words(&[1, 2]);
    assert_eq!(
        isolation(&sides, &sides, &[&sides]),
        fails(Law::Isolation, too_few(1))
    );
}

#[test]
fn the_isolation_law_holds_where_nothing_reaches_the_frozen_sides() {
    let began = words(&[1, 2]);
    let committed = words(&[3, 4]);
    assert_eq!(
        isolation(&began, &began, &[&committed, &committed]),
        Verdict::Holds
    );
}

#[test]
fn the_isolation_law_names_a_frozen_side_that_moved_before_the_boundary() {
    let began = words(&[1, 2]);
    let before = words(&[1, 5]);
    let committed = words(&[3, 4]);
    assert_eq!(
        isolation(&began, &before, &[&committed, &committed]),
        fails(Law::Isolation, differs(0, 1))
    );
}

#[test]
fn the_isolation_law_names_the_poisoned_run_that_committed_otherwise() {
    let began = words(&[1, 2]);
    let committed = words(&[3, 4]);
    // The poisoned runs count from one, so the third of them is stream 3.
    assert_eq!(
        isolation(&began, &began, &[&committed, &committed, &words(&[3, 9])]),
        fails(Law::Isolation, differs(3, 1))
    );
    assert_eq!(
        isolation(&began, &began, &[&committed, &words(&[3])]),
        fails(Law::Isolation, length(2))
    );
}

#[test]
fn a_fold_of_no_words_is_the_offset_basis() {
    assert_eq!(Fold::new().finish().to_raw(), 0xCBF2_9CE4_8422_2325);
}

#[test]
fn a_word_is_folded_least_significant_byte_first() {
    let mut fold = Fold::new();
    fold.words(&words(&[0x0102_0304_0506_0708, 1]));
    let mut bytes = Fnv1a::<64>::new();
    bytes.update(&[8, 7, 6, 5, 4, 3, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(fold.finish().to_raw(), bytes.finalize().to_raw());
}

#[test]
fn the_fold_depends_on_the_sequence_of_the_words() {
    let digest = |raw: &[u64]| {
        let mut fold = Fold::new();
        fold.words(&words(raw));
        fold.finish().to_raw()
    };
    assert_ne!(digest(&[1, 2]), digest(&[2, 1]));
    assert_eq!(digest(&[1, 2]), digest(&[1, 2]));
}
