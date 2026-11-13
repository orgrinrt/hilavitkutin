//! The laws a runner and a store are held to, as functions over plain data returning a verdict.
//!
//! A law takes what a run produced, written down as words, and says whether it holds and, where it
//! does not, where it first fails. It runs nothing: the caller runs, records, and hands the
//! records over, so a law reads the same over any runner. Every law has a mutant beside the runner
//! it was written for that it must fail, and a law comparing records refuses to hold over fewer
//! than it needs to compare.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::domain::Word;
use hilavitkutin_contracts::num::{Bool, Count, Ordinal};

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
        /// Which record, counted as the law's documentation counts them.
        stream: Ordinal,
        /// The first word that differs.
        at:     Ordinal,
    },
    /// Record `stream` is not as long as the reference.
    Length {
        /// Which record.
        stream: Ordinal,
    },
    /// The law was handed `given` records to compare where it needs at least `needs`.
    TooFew {
        /// How many it was handed.
        given: Count,
        /// How many it needs.
        needs: Count,
    },
}

/// A law's verdict.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    /// The law held over what it was handed.
    Holds(Law),
    /// The law failed, and where.
    Fails(Law, Witness),
}

impl Verdict {
    /// Whether the law held.
    pub const fn holds(self) -> Bool {
        Bool::of(matches!(self, Self::Holds(_)))
    }
}

/// The fewest records B2 and B3 compare: one is compared with nothing.
const AT_LEAST: Count = Count::from_host(2);

fn first_difference(reference: &[Word], other: &[Word], stream: Ordinal) -> Maybe<Witness> {
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
                at: Ordinal::from_host(at),
            });
        }
        at += 1;
    }
    Maybe::Isnt
}

fn all_equal(law: Law, reference: &[Word], others: &[&[Word]], first: Ordinal) -> Verdict {
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

fn enough(law: Law, given: usize) -> Maybe<Verdict> {
    let given = Count::from_host(given);
    if given.lt(AT_LEAST).get() {
        Maybe::Is(Verdict::Fails(law, Witness::TooFew {
            given,
            needs: AT_LEAST,
        }))
    } else {
        Maybe::Isnt
    }
}

/// B1. `run` holds a run's cells, frame after frame; `model` holds the model's, in the same order.
/// Stream `0` is the run.
pub fn model(run: &[Word], model: &[Word]) -> Verdict {
    all_equal(Law::Model, model, &[run], Ordinal::FIRST)
}

/// B2. Each of `streams` holds the digests of one run of the matrix; every one equals the first.
/// Fewer than two streams fail with [`Witness::TooFew`].
pub fn morsel_split_and_order(streams: &[&[Word]]) -> Verdict {
    if let Maybe::Is(verdict) = enough(Law::MorselSplitAndOrder, streams.len()) {
        return verdict;
    }
    match streams.split_first() {
        Some((first, rest)) => {
            all_equal(Law::MorselSplitAndOrder, first, rest, Ordinal::FIRST.next())
        },
        None => Verdict::Holds(Law::MorselSplitAndOrder),
    }
}

/// B3. `start` and `end` are the committed cells at a step's start and at its end, before the
/// swap; `poisoned` holds the committed cells after the swap of runs from one committed state
/// whose staged storage was filled with different words before the step. Stream `0` is the end,
/// the rest the poisoned runs in order, from `1`. Fewer than two poisoned runs fail with
/// [`Witness::TooFew`].
pub fn frame_isolation(start: &[Word], end: &[Word], poisoned: &[&[Word]]) -> Verdict {
    if let Maybe::Is(verdict) = enough(Law::FrameIsolation, poisoned.len()) {
        return verdict;
    }
    if let Verdict::Fails(law, witness) =
        all_equal(Law::FrameIsolation, start, &[end], Ordinal::FIRST)
    {
        return Verdict::Fails(law, witness);
    }
    match poisoned.split_first() {
        Some((first, rest)) => {
            all_equal(
                Law::FrameIsolation,
                first,
                rest,
                Ordinal::FIRST.after(AT_LEAST),
            )
        },
        None => Verdict::Holds(Law::FrameIsolation),
    }
}
