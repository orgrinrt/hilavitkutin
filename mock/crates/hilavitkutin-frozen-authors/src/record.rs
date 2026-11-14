//! One frame's state written down as words, in a fixed sequence.
//!
//! The stores come in as slices, so one function records the hand model, a
//! reference runner and a scheduler alike, and a comparison between any two
//! of them is a comparison of what this wrote.

use arvo::{Bits, Bool, Hot};

use crate::values::{Alarms, Ash, Crossed, Glow, Heat, Level, Num, Peak, Pos};

/// The word a number is written down as.
fn word(num: Num) -> Bits<64, Hot> {
    Bits::from_raw(num.to_raw().into())
}

/// One frame's state: the stores as slices and the flag, in the sequence
/// [`words`] writes them.
///
/// `heat` and `level` are the sides the next frame reads, and `crossed` is
/// what the frame appended, in whatever sequence its runner kept.
#[derive(Copy, Clone)]
pub struct State<'a> {
    /// The positions.
    pub pos:      &'a [Pos],
    /// The heat, the side the next frame reads.
    pub heat:     &'a [Heat],
    /// The alarm counts.
    pub alarms:   &'a [Alarms],
    /// The levels, the side the next frame reads.
    pub level:    &'a [Level],
    /// The peaks.
    pub peak:     &'a [Peak],
    /// The glow.
    pub glow:     &'a [Glow],
    /// The ash.
    pub ash:      &'a [Ash],
    /// Whether `Overheat` fired.
    pub overheat: Bool,
    /// What the frame appended.
    pub crossed:  &'a [Crossed],
}

/// Write the frame's state to `sink`, a word at a time.
///
/// The sequence is the positions, the heat, the alarm counts, the levels, the
/// peaks, the glow, the ash, whether `Overheat` fired, the count of crossed
/// values and then the values.
pub fn words(state: State<'_>, mut sink: impl FnMut(Bits<64, Hot>)) {
    state.pos.iter().for_each(|v| sink(word(v.0)));
    state.heat.iter().for_each(|v| sink(word(v.0)));
    state.alarms.iter().for_each(|v| sink(word(v.0)));
    state.level.iter().for_each(|v| sink(word(v.0)));
    state.peak.iter().for_each(|v| sink(word(v.0)));
    state.glow.iter().for_each(|v| sink(word(v.0)));
    state.ash.iter().for_each(|v| sink(word(v.0)));
    sink(Bits::from_raw(state.overheat.0.into()));
    sink(Bits::from_raw(
        state.crossed.iter().fold(0, |count, _| count + 1),
    ));
    state.crossed.iter().for_each(|v| sink(word(v.0)));
}
