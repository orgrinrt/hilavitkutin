//! Each law over plain data: the equal case holds, each kind of difference fails with its witness,
//! and a law handed too few records to compare fails rather than holding over nothing.

use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::{Count, Ordinal};
use hilavitkutin_laws::{Law, Verdict, Witness, frame_isolation, model, morsel_split_and_order};

fn w(n: u8) -> Word {
    Word::of([n; 8])
}

fn too_few(law: Law, given: usize) -> Verdict {
    Verdict::Fails(law, Witness::TooFew {
        given: Count::from_host(given),
        needs: Count::from_host(2),
    })
}

#[test]
fn model_law() {
    let m = [w(1), w(2), w(3)];
    assert_eq!(model(&m, &m), Verdict::Holds(Law::Model));
    assert_eq!(
        model(&[w(1), w(9), w(3)], &m),
        Verdict::Fails(Law::Model, Witness::Differs {
            stream: Ordinal::FIRST,
            at:     Ordinal::from_host(1),
        })
    );
    assert_eq!(
        model(&[w(1)], &m),
        Verdict::Fails(Law::Model, Witness::Length {
            stream: Ordinal::FIRST,
        })
    );
    assert!(model(&m, &m).holds().get());
    assert!(!model(&[w(1)], &m).holds().get());
}

#[test]
fn morsel_split_and_order_law() {
    let a = [w(1), w(2)];
    let b = [w(1), w(3)];
    assert_eq!(
        morsel_split_and_order(&[&a, &a, &a]),
        Verdict::Holds(Law::MorselSplitAndOrder)
    );
    assert_eq!(
        morsel_split_and_order(&[&a, &a, &b]),
        Verdict::Fails(Law::MorselSplitAndOrder, Witness::Differs {
            stream: Ordinal::from_host(2),
            at:     Ordinal::from_host(1),
        })
    );
    assert_eq!(
        morsel_split_and_order(&[&a, &[w(1)]]),
        Verdict::Fails(Law::MorselSplitAndOrder, Witness::Length {
            stream: Ordinal::from_host(1),
        })
    );
}

#[test]
fn morsel_split_and_order_fails_on_fewer_than_two_streams() {
    let a = [w(1), w(2)];
    assert_eq!(
        morsel_split_and_order(&[]),
        too_few(Law::MorselSplitAndOrder, 0)
    );
    assert_eq!(
        morsel_split_and_order(&[&a]),
        too_few(Law::MorselSplitAndOrder, 1)
    );
}

#[test]
fn frame_isolation_law() {
    let s = [w(1), w(2)];
    let p = [w(5), w(6)];
    assert_eq!(
        frame_isolation(&s, &s, &[&p, &p]),
        Verdict::Holds(Law::FrameIsolation)
    );
    assert_eq!(
        frame_isolation(&s, &p, &[&p, &p]),
        Verdict::Fails(Law::FrameIsolation, Witness::Differs {
            stream: Ordinal::FIRST,
            at:     Ordinal::FIRST,
        })
    );
    assert_eq!(
        frame_isolation(&s, &s, &[&p, &s]),
        Verdict::Fails(Law::FrameIsolation, Witness::Differs {
            stream: Ordinal::from_host(2),
            at:     Ordinal::FIRST,
        })
    );
}

#[test]
fn frame_isolation_fails_on_fewer_than_two_poisoned_runs() {
    let s = [w(1), w(2)];
    let p = [w(5), w(6)];
    assert_eq!(
        frame_isolation(&s, &s, &[]),
        too_few(Law::FrameIsolation, 0)
    );
    assert_eq!(
        frame_isolation(&s, &s, &[&p]),
        too_few(Law::FrameIsolation, 1)
    );
    assert_eq!(
        frame_isolation(&s, &p, &[&p]),
        too_few(Law::FrameIsolation, 1),
        "the count is checked before anything is compared"
    );
}
