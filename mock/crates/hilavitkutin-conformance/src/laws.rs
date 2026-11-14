//! The laws: functions over recorded words that return a verdict.
//!
//! A law runs nothing. The caller runs, records and hands the records over,
//! so a law reads the same over any runner. A verdict that fails names the
//! law and where it first failed.

use arvo::USize;
use notko::Maybe;

use crate::digest::Word;

/// Which law a verdict is about.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Law {
    /// A run equals the model.
    Model,
    /// Runs that differ only in how the records were split and visited are
    /// equal.
    Split,
    /// A frame reads the frozen sides it began with, whatever it wrote.
    Isolation,
}

/// Where a law first failed.
///
/// Streams are counted from zero. For [`model`] stream zero is the run. For
/// [`split`] the first stream is the reference and the rest count from one.
/// For [`isolation`] stream zero is the frozen sides before the boundary, and
/// the poisoned runs count from one, the first of them being the reference
/// for the rest.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Breach {
    /// The stream's word at `at` is not the reference's.
    Differs {
        /// The stream that differs.
        stream: USize,
        /// The first word that differs.
        at:     USize,
    },
    /// The stream is not as long as the reference.
    Length {
        /// The stream of another length.
        stream: USize,
    },
    /// A law that compares runs with each other was handed too few.
    TooFew {
        /// How many it was handed.
        given: USize,
        /// How many it needs.
        needs: USize,
    },
}

/// What a law says of the records it was handed.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Verdict {
    /// The law holds over the records.
    Holds,
    /// The law fails, and this is where it first did.
    Fails {
        /// The law that fails.
        law:    Law,
        /// Where it first failed.
        breach: Breach,
    },
}

/// How many runs a law that compares runs with each other needs.
const AT_LEAST: USize = USize(2);

/// The first word at which two streams of one length differ.
const fn first_difference(one: &[Word], other: &[Word]) -> Maybe<USize> {
    let mut at = USize(0);
    while at.0 < one.len() && at.0 < other.len() {
        if one[at.0].to_raw() != other[at.0].to_raw() {
            return Maybe::Is(at);
        }
        at = USize(at.0 + 1);
    }
    Maybe::Isnt
}

/// How `stream` breaches against `reference`, if it does.
const fn breach_of(stream: USize, words: &[Word], reference: &[Word]) -> Maybe<Breach> {
    if words.len() != reference.len() {
        return Maybe::Is(Breach::Length {
            stream,
        });
    }
    match first_difference(words, reference) {
        Maybe::Is(at) => {
            Maybe::Is(Breach::Differs {
                stream,
                at,
            })
        },
        Maybe::Isnt => Maybe::Isnt,
    }
}

const fn fails(law: Law, breach: Breach) -> Verdict {
    Verdict::Fails {
        law,
        breach,
    }
}

/// The run's words equal the model's, in the same sequence.
///
/// ```
/// use arvo::USize;
/// use hilavitkutin_conformance::{Breach, Law, Verdict, Word, model};
///
/// let w = Word::from_raw;
/// assert_eq!(model(&[w(1), w(2)], &[w(1), w(2)]), Verdict::Holds);
/// assert_eq!(model(&[w(1), w(3)], &[w(1), w(2)]), Verdict::Fails {
///     law:    Law::Model,
///     breach: Breach::Differs {
///         stream: USize(0),
///         at:     USize(1),
///     },
/// });
/// ```
pub const fn model(run: &[Word], model: &[Word]) -> Verdict {
    match breach_of(USize(0), run, model) {
        Maybe::Is(breach) => fails(Law::Model, breach),
        Maybe::Isnt => Verdict::Holds,
    }
}

/// Every stream equals the first.
///
/// The streams are runs that differ only in how the records were split and
/// visited: the entry, the worker count, the morsel size, the visiting.
pub const fn split(streams: &[&[Word]]) -> Verdict {
    if streams.len() < AT_LEAST.0 {
        return fails(Law::Split, Breach::TooFew {
            given: USize(streams.len()),
            needs: AT_LEAST,
        });
    }
    let mut stream = USize(1);
    while stream.0 < streams.len() {
        if let Maybe::Is(breach) = breach_of(stream, streams[stream.0], streams[0]) {
            return fails(Law::Split, breach);
        }
        stream = USize(stream.0 + 1);
    }
    Verdict::Holds
}

/// The frozen sides before the boundary equal the frozen sides the frame
/// began with, and the committed sides of runs that began with different
/// live sides are equal.
///
/// `began` and `before` are one run's frozen sides, recorded ahead of the
/// first unit and between the last unit and the boundary. `committed` holds
/// the frozen sides after the boundary, one stream for each run, the runs
/// differing only in what their live sides held when the frame began.
pub const fn isolation(began: &[Word], before: &[Word], committed: &[&[Word]]) -> Verdict {
    if committed.len() < AT_LEAST.0 {
        return fails(Law::Isolation, Breach::TooFew {
            given: USize(committed.len()),
            needs: AT_LEAST,
        });
    }
    if let Maybe::Is(breach) = breach_of(USize(0), before, began) {
        return fails(Law::Isolation, breach);
    }
    let mut run = USize(1);
    while run.0 < committed.len() {
        // The poisoned runs count from one, so run `n` is stream `n + 1`.
        let stream = USize(run.0 + 1);
        if let Maybe::Is(breach) = breach_of(stream, committed[run.0], committed[0]) {
            return fails(Law::Isolation, breach);
        }
        run = USize(run.0 + 1);
    }
    Verdict::Holds
}
