//! The rules step 1 builds its edges by, over inputs written by hand.
//!
//! A reader waits on a writer of a store it reads, whichever registered
//! first. A read at the frozen side waits on nobody. Two units of different
//! rank take the sequence of their ranks, so an edge is built only where it
//! agrees with them, and the registration check passes over an edge between
//! two ranks. The stores the leading bands write, less the buffered ones, are
//! what a pass seeds its dirty units from.
//!
//! Lives under `tests/` so the bare numeric positions do not trip the
//! src-tree primitive lints.

use arvo::{Bool, USize};
use hilavitkutin::plan::edge::{raw_edge, reach_behind_a_writer, with_the_ranks};
use hilavitkutin::plan::{AccessMask, BandWrites, DefaultPlanDims, PlanDims, PlanInputs, steps};
use hilavitkutin_api::meta::{
    RANK_CONSUMER,
    RANK_PASS_START,
    RANK_PLAN_STAGE,
    RANK_SCHEDULE_END,
    RANK_SCHEDULE_READY,
};
use notko::Maybe;

type D = DefaultPlanDims;
type Inputs = PlanInputs<<D as PlanDims>::Units, <D as PlanDims>::Stores>;
type Mask = AccessMask<<D as PlanDims>::Stores>;

/// The mask of the stores at `positions`.
fn mask(positions: &[usize]) -> Mask {
    positions
        .iter()
        .fold(Mask::empty(), |m, p| m.set(USize(*p)))
}

/// One unit of a fixture: its rank, what it reads, which of those reads are
/// at the frozen side, and what it writes.
struct Unit {
    rank:   USize,
    reads:  &'static [usize],
    frozen: &'static [usize],
    writes: &'static [usize],
}

fn inputs(units: &[Unit]) -> Inputs {
    let mut inputs = Inputs::new();
    for (u, unit) in units.iter().enumerate() {
        inputs.reads.as_mut()[u] = mask(unit.reads);
        inputs.frozen.as_mut()[u] = mask(unit.frozen);
        inputs.writes.as_mut()[u] = mask(unit.writes);
        inputs.access.as_mut()[u] = mask(unit.reads).union(&mask(unit.writes));
        inputs.ranks.as_mut()[u] = unit.rank;
    }
    inputs.unit_count = USize(units.len());
    inputs
}

fn edge(inputs: &Inputs, from: usize, to: usize) -> bool {
    steps::build_dag::<D>(inputs)
        .has_edge(USize(from), USize(to))
        .0
}

const fn consumer(reads: &'static [usize], writes: &'static [usize]) -> Unit {
    Unit {
        rank: RANK_CONSUMER,
        reads,
        frozen: &[],
        writes,
    }
}

#[test]
fn the_ranks_agree_where_the_writer_is_not_past_the_reader() {
    let ranks = [
        RANK_PLAN_STAGE,
        RANK_SCHEDULE_READY,
        RANK_PASS_START,
        RANK_CONSUMER,
        RANK_SCHEDULE_END,
    ];
    for (i, from) in ranks.iter().enumerate() {
        for (j, to) in ranks.iter().enumerate() {
            assert_eq!(
                with_the_ranks(*from, *to),
                Bool(i <= j),
                "rank {i} before rank {j}"
            );
        }
    }
}

#[test]
fn an_edge_wants_the_ranks_and_an_overlap() {
    let writes = mask(&[2]);
    assert!(raw_edge(&writes, RANK_CONSUMER, &mask(&[2, 5]), RANK_CONSUMER).0);
    assert!(raw_edge(&writes, RANK_PASS_START, &mask(&[2]), RANK_CONSUMER).0);
    assert!(
        !raw_edge(&writes, RANK_CONSUMER, &mask(&[5]), RANK_CONSUMER).0,
        "no store in common"
    );
    assert!(
        !raw_edge(&writes, RANK_SCHEDULE_END, &mask(&[2]), RANK_CONSUMER).0,
        "the writer's rank is past the reader's"
    );
}

#[test]
fn a_reader_waits_on_its_writer_in_either_registration_sequence() {
    let first = inputs(&[consumer(&[], &[0]), consumer(&[0], &[1])]);
    assert!(edge(&first, 0, 1));
    assert!(!edge(&first, 1, 0));
    let second = inputs(&[consumer(&[0], &[1]), consumer(&[], &[0])]);
    assert!(
        edge(&second, 1, 0),
        "the writer registered second still comes first"
    );
    assert!(!edge(&second, 0, 1));
}

#[test]
fn a_read_at_the_frozen_side_waits_on_no_writer() {
    let reader = Unit {
        rank:   RANK_CONSUMER,
        reads:  &[0, 2],
        frozen: &[0],
        writes: &[1],
    };
    let i = inputs(&[consumer(&[], &[0]), reader, consumer(&[], &[2])]);
    assert!(!edge(&i, 0, 1), "store 0 is read at the frozen side");
    assert!(
        edge(&i, 2, 1),
        "store 2 is read plain, and its writer comes first"
    );
}

#[test]
fn a_reader_and_a_writer_of_one_buffered_store_in_one_unit_take_no_edge() {
    let unit = || {
        Unit {
            rank:   RANK_CONSUMER,
            reads:  &[0],
            frozen: &[0],
            writes: &[0],
        }
    };
    let i = inputs(&[unit(), unit()]);
    // Two writers of the store are still put in sequence, the earlier first.
    assert!(edge(&i, 0, 1));
    assert!(!edge(&i, 1, 0));
    assert!(matches!(steps::first_back_edge::<D>(&i), Maybe::Isnt));
}

#[test]
fn an_edge_that_would_run_against_the_ranks_is_not_built() {
    // The close writes what a consumer reads. The consumer does not wait on
    // it: the close of this pass comes after every consumer.
    let close = Unit {
        rank:   RANK_SCHEDULE_END,
        reads:  &[1],
        frozen: &[],
        writes: &[0],
    };
    let i = inputs(&[close, consumer(&[0], &[1])]);
    assert!(
        !edge(&i, 0, 1),
        "the close does not come before the consumer"
    );
    assert!(edge(&i, 1, 0), "the close reads what the consumer wrote");
}

#[test]
fn two_writers_of_different_rank_take_the_sequence_of_their_ranks() {
    let close = Unit {
        rank:   RANK_SCHEDULE_END,
        reads:  &[],
        frozen: &[],
        writes: &[0],
    };
    // Registered first, the close would come first by position. Its rank
    // says otherwise, so no write-write edge is built from it.
    let i = inputs(&[close, consumer(&[], &[0])]);
    assert!(!edge(&i, 0, 1));
}

#[test]
fn the_registration_check_passes_over_an_edge_between_two_ranks() {
    let setup = || {
        Unit {
            rank:   RANK_PASS_START,
            reads:  &[],
            frozen: &[],
            writes: &[0],
        }
    };
    let close = || {
        Unit {
            rank:   RANK_SCHEDULE_END,
            reads:  &[1],
            frozen: &[],
            writes: &[2],
        }
    };
    // A reader ahead of the setup it reads from, and a close ahead of the
    // consumer it follows: both edges run against the registration and
    // neither is a back-edge.
    let i = inputs(&[close(), consumer(&[0], &[1]), setup()]);
    assert!(edge(&i, 2, 1));
    assert!(edge(&i, 1, 0));
    assert!(matches!(steps::first_back_edge::<D>(&i), Maybe::Isnt));
}

#[test]
fn the_registration_check_refuses_a_reader_ahead_of_a_writer_of_its_own_rank() {
    let i = inputs(&[consumer(&[0], &[1]), consumer(&[], &[0])]);
    match steps::first_back_edge::<D>(&i) {
        Maybe::Is((from, to)) => assert_eq!((from.0, to.0), (1, 0)),
        Maybe::Isnt => panic!("the reader is registered ahead of its writer"),
    }
    // Two maintenance units of one rank are held to the same.
    let reads = Unit {
        rank:   RANK_SCHEDULE_END,
        reads:  &[0],
        frozen: &[],
        writes: &[1],
    };
    let writes = Unit {
        rank:   RANK_SCHEDULE_END,
        reads:  &[],
        frozen: &[],
        writes: &[0],
    };
    let i = inputs(&[reads, writes]);
    assert!(matches!(steps::first_back_edge::<D>(&i), Maybe::Is(_)));
}

#[test]
fn a_reach_behind_a_writer_is_seen_only_behind_an_earlier_writer_its_rank_allows() {
    let none = Mask::empty();
    let reaches = [none, none, mask(&[0]), mask(&[3])];
    let writes = [mask(&[0]), mask(&[1]), none, none];
    let ranks = [RANK_CONSUMER; 4];
    assert!(reach_behind_a_writer(&reaches, &writes, &ranks, USize(2)).0);
    assert!(
        !reach_behind_a_writer(&reaches, &writes, &ranks, USize(3)).0,
        "nobody earlier writes store 3"
    );
    assert!(!reach_behind_a_writer(&reaches, &writes, &ranks, USize(0)).0);

    // The writer is of a later rank than the reader: it runs after it, so the
    // reader's reach is not behind it.
    let ranks = [RANK_SCHEDULE_END, RANK_CONSUMER, RANK_CONSUMER, RANK_CONSUMER];
    assert!(!reach_behind_a_writer(&reaches, &writes, &ranks, USize(2)).0);
}

#[test]
fn the_band_seed_holds_what_the_leading_bands_write_less_the_buffered() {
    let plan = Unit {
        rank:   RANK_PLAN_STAGE,
        reads:  &[],
        frozen: &[],
        writes: &[0],
    };
    let setup = Unit {
        rank:   RANK_PASS_START,
        reads:  &[],
        frozen: &[],
        writes: &[1, 2],
    };
    let reader = Unit {
        rank:   RANK_CONSUMER,
        reads:  &[1, 2],
        frozen: &[2],
        writes: &[3],
    };
    let close = Unit {
        rank:   RANK_SCHEDULE_END,
        reads:  &[3],
        frozen: &[],
        writes: &[4],
    };
    let bands = BandWrites::of(&inputs(&[plan, setup, reader, close]));
    let lead = bands.seed(Bool::FALSE);
    let all = bands.seed(Bool::TRUE);
    let held = |m: &Mask| -> Vec<usize> { (0 .. 6).filter(|p| m.contains(USize(*p)).0).collect() };
    assert_eq!(
        held(&lead),
        [1],
        "the setup's plain store, and not its buffered one"
    );
    assert_eq!(
        held(&all),
        [0, 1],
        "the plan band's store joins in a pass that runs it"
    );
}
