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

/// Write the frame's state to `sink`, a word at a time.
///
/// The sequence is the positions, the heat, the alarm counts, the levels, the
/// peaks, the glow, the ash, whether `Overheat` fired, the count of crossed
/// values and then the values. `heat` and `level` are the sides the next
/// frame reads, and `crossed` is what the frame appended, in whatever
/// sequence its runner kept.
#[allow(clippy::too_many_arguments)]
pub fn words(
    pos: &[Pos],
    heat: &[Heat],
    alarms: &[Alarms],
    level: &[Level],
    peak: &[Peak],
    glow: &[Glow],
    ash: &[Ash],
    overheat: Bool,
    crossed: &[Crossed],
    mut sink: impl FnMut(Bits<64, Hot>),
) {
    pos.iter().for_each(|v| sink(word(v.0)));
    heat.iter().for_each(|v| sink(word(v.0)));
    alarms.iter().for_each(|v| sink(word(v.0)));
    level.iter().for_each(|v| sink(word(v.0)));
    peak.iter().for_each(|v| sink(word(v.0)));
    glow.iter().for_each(|v| sink(word(v.0)));
    ash.iter().for_each(|v| sink(word(v.0)));
    sink(Bits::from_raw(overheat.0.into()));
    sink(Bits::from_raw(crossed.iter().fold(0, |count, _| count + 1)));
    crossed.iter().for_each(|v| sink(word(v.0)));
}
