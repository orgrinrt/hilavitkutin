//! B1 to B3 over both frozen stores and the honest probe, every morsel size, both visiting orders
//! and two permutation seeds, each run equal to the model.

mod common;

use common::probe::{Honest, Probe};
use common::{
    COLUMNS,
    Cells,
    INPUTS,
    MORSELS,
    cells_per_frame,
    digest_stream,
    isolation,
    model_frames,
};
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::{Count, Index, USize};
use hilavitkutin_contracts::store::{Morsel, Visit, Visitor};
use hilavitkutin_frozen_authors::Plane;
use hilavitkutin_frozen_implementors::{Columns, Forward, Permuted, Records};
use hilavitkutin_laws::{Law, Verdict, frame_isolation, model, morsel_split_and_order};

type One = Columns<Plane, COLUMNS, { common::CELLS }>;
type Two = Records<Plane, COLUMNS, { common::CELLS }>;

const SEEDS: [Permuted; 2] = [
    Permuted {
        seed: USize::from_host(11),
    },
    Permuted {
        seed: USize::from_host(1_000_003),
    },
];

fn b1<S: Cells, V: Visit>(visit: &V) {
    for input in INPUTS {
        let expected = model_frames(input);
        for morsel in MORSELS {
            let run = cells_per_frame::<S, V>(visit, morsel, input);
            assert_eq!(
                model(&run, &expected),
                Verdict::Holds(Law::Model),
                "input {input:?}, morsel {morsel:?}"
            );
        }
    }
}

#[test]
fn b1_model_over_both_stores_both_orders_every_morsel_every_input() {
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
fn b3_frame_isolation_over_both_stores_both_orders() {
    b3::<One, _>(&Forward);
    b3::<Two, _>(&Forward);
    b3::<Probe<Honest>, _>(&Forward);
    for permuted in &SEEDS {
        b3::<One, _>(permuted);
        b3::<Two, _>(permuted);
        b3::<Probe<Honest>, _>(permuted);
    }
}

struct Tally {
    seen:    [u32; common::CELLS],
    morsels: Vec<usize>,
    order:   Vec<usize>,
}

impl Visitor for Tally {
    fn morsel(&mut self, len: Count) {
        self.morsels.push(len.host());
    }

    fn record(&mut self, at: Index) {
        self.seen[at.host()] += 1;
        self.order.push(at.host());
    }
}

fn tally<V: Visit>(visit: &V, extent: usize, morsel: Morsel) -> Tally {
    let mut t = Tally {
        seen:    [0; common::CELLS],
        morsels: Vec::new(),
        order:   Vec::new(),
    };
    visit.visit(Count::from_host(extent), morsel, &mut t);
    t
}

#[test]
fn both_visits_cover_every_record_once_in_morsels_of_the_asked_size() {
    for morsel in MORSELS {
        let size = morsel.size().host();
        for t in [
            tally(&Forward, common::CELLS, morsel),
            tally(&SEEDS[0], common::CELLS, morsel),
            tally(&SEEDS[1], common::CELLS, morsel),
        ] {
            assert!(t.seen.iter().all(|&n| n == 1));
            let (last, full) = t
                .morsels
                .split_last()
                .expect("a plane is at least one morsel");
            assert!(full.iter().all(|&n| n == size));
            assert!(*last >= 1 && *last <= size);
        }
    }
    assert_ne!(
        tally(&SEEDS[0], common::CELLS, Morsel::ONE).order,
        (0 .. common::CELLS).collect::<Vec<_>>(),
        "the permuted order is not forward"
    );
}

#[test]
fn an_extent_below_the_morsel_is_one_short_morsel_and_zero_is_none() {
    let seven = MORSELS[1];
    for t in [tally(&Forward, 3, seven), tally(&SEEDS[0], 3, seven)] {
        assert_eq!(t.morsels, vec![3]);
        assert_eq!(&t.seen[.. 3], &[1, 1, 1]);
    }
    for t in [tally(&Forward, 0, seven), tally(&SEEDS[0], 0, seven)] {
        assert!(t.morsels.is_empty());
        assert!(t.order.is_empty());
    }
}
