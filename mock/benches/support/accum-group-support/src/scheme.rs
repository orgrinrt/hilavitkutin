//! The schemes of the fold split, and the hand model of each.
//!
//! A scheme says which records walker `core` of `walkers` folds, and how many
//! records go into one partial. Every walker cuts its slice into runs of
//! [`Scheme::GROUP`] records from the slice's first record, folds each run on
//! its own and appends one partial per run into its own region; the merge
//! keeps the partials in core sequence and folds them left to right. This is
//! the engine's threaded accumulator: a core's region starts at its record
//! slice and the regions are compacted in core sequence after the frame.
//!
//! Four schemes:
//!
//! 1. [`Sequential`]: one walker folds every record. The canon's answer for a
//!    fold that is not commutative, and the one result that is the left fold.
//! 2. [`PerCore`]: the engine's N-way ceil slice with one partial per core.
//!    Its result depends on the worker count under a law that is not
//!    associative. The price's baseline.
//! 3. [`Grouped`]: option 3, the accumulator grouped by the data. Groups of
//!    `G` records from record zero, one partial per group, and the slice
//!    rounded up to whole groups so no core's slice cuts a group.
//! 4. [`GroupedOnCeil`]: groups of `G` from each slice's start on the
//!    unrounded ceil slice. This is what a unit that groups by itself gets on
//!    the engine as shipped, and it is the case that must fail: wherever a
//!    slice boundary falls inside a group, the grouping is the worker's and
//!    not the data's. No arm times it.

use std::ops::Range;

use stc_support::split::ceil_slice;

use crate::law::{Sat, left_fold};

/// The ids an output carries, one per scheme. Named here so validation can
/// match on them; a scheme's [`Scheme::ID`] is one of these.
pub const ID_SEQUENTIAL: u32 = 0;
pub const ID_PER_CORE: u32 = 1;
pub const ID_GROUPED: u32 = 2;
pub const ID_GROUPED_ON_CEIL: u32 = 3;

/// A way to split one fold across walkers.
pub trait Scheme: Sync + 'static {
    /// The arm's name, the key of its state.
    const NAME: &'static str;
    /// The id an output carries, so validation checks it against this
    /// scheme's model.
    const ID: u32;
    /// The records one partial folds; `usize::MAX` for a whole slice.
    const GROUP: usize;

    /// How many walkers the scheme runs at `workers` workers.
    fn walkers(workers: usize) -> usize {
        workers
    }

    /// The records walker `core` of `walkers` folds, out of `n`.
    fn slice(core: usize, walkers: usize, n: usize) -> Range<usize>;

    /// The partials this scheme must give over `data` at `workers` workers,
    /// computed on one thread from the scheme's definition and nothing else.
    fn model(data: &[Sat], workers: usize) -> Vec<Sat>;
}

/// The ceil slice rounded up to whole groups of `group` records: the group
/// count is split by the ceil slice and scaled back to records.
pub fn group_slice(core: usize, walkers: usize, n: usize, group: usize) -> Range<usize> {
    let groups = ceil_slice(core, walkers, n.div_ceil(group));
    (groups.start * group).min(n) .. (groups.end * group).min(n)
}

/// One walker over every record.
pub struct Sequential;

impl Scheme for Sequential {
    const GROUP: usize = usize::MAX;
    const ID: u32 = ID_SEQUENTIAL;
    const NAME: &'static str = "sequential";

    fn walkers(_workers: usize) -> usize {
        1
    }

    fn slice(_core: usize, _walkers: usize, n: usize) -> Range<usize> {
        0 .. n
    }

    fn model(data: &[Sat], _workers: usize) -> Vec<Sat> {
        if data.is_empty() { Vec::new() } else { vec![left_fold(data)] }
    }
}

/// The engine's ceil slice, one partial per core that has records.
pub struct PerCore;

impl Scheme for PerCore {
    const GROUP: usize = usize::MAX;
    const ID: u32 = ID_PER_CORE;
    const NAME: &'static str = "per_core";

    fn slice(core: usize, walkers: usize, n: usize) -> Range<usize> {
        ceil_slice(core, walkers, n)
    }

    fn model(data: &[Sat], workers: usize) -> Vec<Sat> {
        // A surplus core's slice is empty and appends nothing; the empty
        // slices are the last ones, so the rest stay in core sequence.
        let per = data.len().div_ceil(workers.max(1)).max(1);
        data.chunks(per).map(left_fold).collect()
    }
}

/// Option 3: groups of `G` fixed by the data, on slices of whole groups.
pub struct Grouped<const G: usize>;

impl<const G: usize> Scheme for Grouped<G> {
    const GROUP: usize = G;
    const ID: u32 = ID_GROUPED;
    const NAME: &'static str = "grouped";

    fn slice(core: usize, walkers: usize, n: usize) -> Range<usize> {
        group_slice(core, walkers, n, G)
    }

    /// The groups of `G` from record zero, each folded on its own. The
    /// worker count does not appear: that is the claim the bench tests.
    fn model(data: &[Sat], _workers: usize) -> Vec<Sat> {
        data.chunks(G).map(left_fold).collect()
    }
}

/// Groups of `G` from each core's slice start, on the unrounded ceil slice.
pub struct GroupedOnCeil<const G: usize>;

impl<const G: usize> Scheme for GroupedOnCeil<G> {
    const GROUP: usize = G;
    const ID: u32 = ID_GROUPED_ON_CEIL;
    const NAME: &'static str = "grouped_on_ceil";

    fn slice(core: usize, walkers: usize, n: usize) -> Range<usize> {
        ceil_slice(core, walkers, n)
    }

    fn model(data: &[Sat], workers: usize) -> Vec<Sat> {
        let per = data.len().div_ceil(workers.max(1)).max(1);
        data.chunks(per)
            .flat_map(|slice| slice.chunks(G).map(left_fold))
            .collect()
    }
}
