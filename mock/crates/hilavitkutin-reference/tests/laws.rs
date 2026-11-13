//! B1 to B3 over both frozen stores and the honest probe, every morsel size, both visiting
//! orders, each run equal to the model.

mod common;

use common::probe::{Honest, Probe};
use common::{Cells, MORSELS, cells_per_frame, digest_stream, isolation, model_frames};
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::{Count, USize};
use hilavitkutin_contracts::store::Visit;
use hilavitkutin_frozen_authors::Plane;
use hilavitkutin_frozen_implementors::{Columns, Forward, Permuted, Records};
use hilavitkutin_laws::{Law, Verdict, frame_isolation, model, morsel_split_and_order};

type One = Columns<Plane, 1, { common::CELLS }>;
type Two = Records<Plane, 1, { common::CELLS }>;

const SEEDS: [Permuted; 2] = [
    Permuted {
        seed: USize::from_host(11),
    },
    Permuted {
        seed: USize::from_host(1_000_003),
    },
];

fn b1<S: Cells, V: Visit>(visit: &V) {
    let expected = model_frames();
    for morsel in MORSELS {
        let run = cells_per_frame::<S, V>(visit, morsel);
        assert_eq!(
            model(&run, &expected),
            Verdict::Holds(Law::Model),
            "morsel {morsel:?}"
        );
    }
}

#[test]
fn b1_model_over_both_stores_both_orders_every_morsel() {
    b1::<One, _>(&Forward);
    b1::<Two, _>(&Forward);
    b1::<Probe<Honest>, _>(&Forward);
    for permuted in &SEEDS {
        b1::<One, _>(permuted);
        b1::<Two, _>(permuted);
        b1::<Probe<Honest>, _>(permuted);
    }
}

fn streams<S: Cells>(into: &mut Vec<[Word; common::FRAMES]>) {
    for morsel in MORSELS {
        into.push(digest_stream::<S, _>(&Forward, morsel));
        for permuted in &SEEDS {
            into.push(digest_stream::<S, _>(permuted, morsel));
        }
    }
}

#[test]
fn b2_morsel_split_and_order_over_the_matrix() {
    let mut all = Vec::new();
    streams::<One>(&mut all);
    streams::<Two>(&mut all);
    streams::<Probe<Honest>>(&mut all);
    let views: Vec<&[Word]> = all.iter().map(|s| s.as_slice()).collect();
    assert_eq!(views.len(), 3 * MORSELS.len() * (1 + SEEDS.len()));
    assert_eq!(
        morsel_split_and_order(&views),
        Verdict::Holds(Law::MorselSplitAndOrder)
    );
}

fn b3<S: Cells, V: Visit>(visit: &V) {
    for morsel in MORSELS {
        let r = isolation::<S, V>(visit, morsel);
        let verdict = frame_isolation(&r.start, &r.end, &[&r.poisoned[0], &r.poisoned[1]]);
        assert_eq!(
            verdict,
            Verdict::Holds(Law::FrameIsolation),
            "morsel {morsel:?}"
        );
    }
}

#[test]
fn b3_frame_isolation_over_both_stores() {
    b3::<One, _>(&Forward);
    b3::<Two, _>(&Forward);
    b3::<Probe<Honest>, _>(&Forward);
    b3::<One, _>(&SEEDS[0]);
    b3::<Two, _>(&SEEDS[0]);
}

#[test]
fn both_visits_cover_every_record_once_in_morsels_of_the_asked_size() {
    struct Tally {
        seen:    [u32; common::CELLS],
        morsels: Vec<usize>,
    }
    impl hilavitkutin_contracts::store::Visitor for Tally {
        fn morsel(&mut self, len: Count) {
            self.morsels.push(len.host());
        }

        fn record(&mut self, at: hilavitkutin_contracts::num::Index) {
            self.seen[at.host()] += 1;
        }
    }
    fn tally<V: Visit>(visit: &V, size: Count) -> Tally {
        let mut t = Tally {
            seen:    [0; common::CELLS],
            morsels: Vec::new(),
        };
        visit.visit(Count::from_host(common::CELLS), size, &mut t);
        t
    }
    for size in MORSELS {
        for t in [tally(&Forward, size), tally(&SEEDS[0], size), tally(&SEEDS[1], size)] {
            assert!(t.seen.iter().all(|&n| n == 1));
            let (last, full) = t.morsels.split_last().unwrap();
            assert!(full.iter().all(|&n| n == size.host()));
            assert!(*last >= 1 && *last <= size.host());
        }
    }
    let forward: Vec<u32> = {
        let mut order = Vec::new();
        struct Order<'o>(&'o mut Vec<u32>);
        impl hilavitkutin_contracts::store::Visitor for Order<'_> {
            fn morsel(&mut self, _len: Count) {}

            fn record(&mut self, at: hilavitkutin_contracts::num::Index) {
                self.0.push(at.host() as u32);
            }
        }
        SEEDS[0].visit(
            Count::from_host(common::CELLS),
            Count::ONE,
            &mut Order(&mut order),
        );
        order
    };
    assert_ne!(
        forward,
        (0 .. common::CELLS as u32).collect::<Vec<_>>(),
        "the permuted order is not forward"
    );
}
