//! The laws a runner and a store are held to, as functions over plain data returning a verdict.
//!
//! A law takes what a run produced, written down as words, and says whether it holds and, where it
//! does not, where it first fails. It runs nothing: the caller runs, records, and hands the
//! records over, so a law reads the same over any runner. Every law has a mutant beside the runner
//! it was written for that it must fail.
//!
//! The ledger check of the frozen crates is here too, a function over the text of their ledgers.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod ledger;

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::{Bool, Count, Index};

/// The laws, by name.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Law {
    /// B1: a run's columns after every frame equal the model's.
    Model,
    /// B2: the digest streams of runs that differ only in morsel size, visiting order or store
    /// layout are equal.
    MorselSplitAndOrder,
    /// B3: a step leaves the committed cells as it found them, and what it commits does not depend
    /// on what the staged storage held before it.
    FrameIsolation,
}

/// Where a law first failed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Witness {
    /// Record `stream` differs from the reference at word `at`.
    Differs {
        stream: Index,
        at:     Index,
    },
    /// Record `stream` is not as long as the reference.
    Length {
        stream: Index,
    },
}

/// A law's verdict.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    Holds(Law),
    Fails(Law, Witness),
}

impl Verdict {
    /// Whether the law held.
    pub const fn holds(self) -> Bool {
        Bool::of(matches!(self, Self::Holds(_)))
    }
}

fn first_difference(reference: &[Word], other: &[Word], stream: Index) -> Maybe<Witness> {
    if reference.len() != other.len() {
        return Maybe::Is(Witness::Length {
            stream,
        });
    }
    let mut at = 0;
    while at < reference.len() {
        if reference[at] != other[at] {
            return Maybe::Is(Witness::Differs {
                stream,
                at: Index::from_host(at),
            });
        }
        at += 1;
    }
    Maybe::Isnt
}

fn all_equal(law: Law, reference: &[Word], others: &[&[Word]], first: Index) -> Verdict {
    let mut i = 0;
    while i < others.len() {
        if let Maybe::Is(witness) =
            first_difference(reference, others[i], first.after(Count::from_host(i)))
        {
            return Verdict::Fails(law, witness);
        }
        i += 1;
    }
    Verdict::Holds(law)
}

/// B1. `run` holds a run's written cells, frame after frame; `model` holds the model's, in the same
/// order. Stream `0` is the run.
pub fn model(run: &[Word], model: &[Word]) -> Verdict {
    all_equal(Law::Model, model, &[run], Index::FIRST)
}

/// B2. Each of `streams` holds the digests of one run of the matrix; every one equals the first.
pub fn morsel_split_and_order(streams: &[&[Word]]) -> Verdict {
    match streams.split_first() {
        Some((first, rest)) => {
            all_equal(Law::MorselSplitAndOrder, first, rest, Index::from_host(1))
        },
        None => Verdict::Holds(Law::MorselSplitAndOrder),
    }
}

/// B3. `start` and `end` are the committed cells at a step's start and at its end, before the
/// swap; `poisoned` holds the committed cells after the swap of runs from one committed state
/// whose staged storage was filled with different words before the step. Stream `0` is the end,
/// the rest the poisoned runs in order.
pub fn frame_isolation(start: &[Word], end: &[Word], poisoned: &[&[Word]]) -> Verdict {
    if let Verdict::Fails(law, witness) =
        all_equal(Law::FrameIsolation, start, &[end], Index::FIRST)
    {
        return Verdict::Fails(law, witness);
    }
    match poisoned.split_first() {
        Some((first, rest)) => all_equal(Law::FrameIsolation, first, rest, Index::from_host(2)),
        None => Verdict::Holds(Law::FrameIsolation),
    }
}
