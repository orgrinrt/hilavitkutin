//! Each law over plain data: the equal case holds, and each kind of difference fails with its
//! witness.

use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::Index;
use hilavitkutin_laws::{Law, Verdict, Witness, frame_isolation, model, morsel_split_and_order};

fn w(n: u8) -> Word {
    Word::of([n; 8])
}

#[test]
fn model_law() {
    let m = [w(1), w(2), w(3)];
    assert_eq!(model(&m, &m), Verdict::Holds(Law::Model));
    assert_eq!(
        model(&[w(1), w(9), w(3)], &m),
        Verdict::Fails(Law::Model, Witness::Differs {
            stream: Index::FIRST,
            at:     Index::from_host(1),
        })
    );
    assert_eq!(
        model(&[w(1)], &m),
        Verdict::Fails(Law::Model, Witness::Length {
            stream: Index::FIRST,
        })
    );
    assert!(model(&m, &m).holds().get());
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
        morsel_split_and_order(&[]),
        Verdict::Holds(Law::MorselSplitAndOrder)
    );
    assert_eq!(
        morsel_split_and_order(&[&a, &a, &b]),
        Verdict::Fails(Law::MorselSplitAndOrder, Witness::Differs {
            stream: Index::from_host(2),
            at:     Index::from_host(1),
        })
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
            stream: Index::FIRST,
            at:     Index::FIRST,
        })
    );
    assert_eq!(
        frame_isolation(&s, &s, &[&p, &s]),
        Verdict::Fails(Law::FrameIsolation, Witness::Differs {
            stream: Index::from_host(2),
            at:     Index::FIRST,
        })
    );
}
