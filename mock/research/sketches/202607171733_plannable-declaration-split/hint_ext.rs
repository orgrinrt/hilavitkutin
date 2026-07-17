// Sketch arm: can SchedulingHint gain an extension slot additively,
// keeping its seal and changing no existing hint?
#![allow(dead_code)]
mod hint_sealed { pub trait Sealed {} }
pub trait UrgencyValue: 'static {} pub trait DivisibilityValue: 'static {} pub trait SignificanceValue: 'static {}
pub trait SchedulingHint: hint_sealed::Sealed + 'static {}

// EXISTING ARM, same shape as hint.rs:70-74. Untouched by this round.
impl<U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue> hint_sealed::Sealed for (U, D, S) {}
impl<U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue> SchedulingHint for (U, D, S) {}

// NEW ARM. A 3-tuple and a 4-tuple are different types, so no overlap.
// The seal holds: only tuple shapes this crate names are hints. HintExt is
// the one open slot, so a foreign runner supplies its own dimension.
pub trait HintExt: 'static {}
impl<U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue, X: HintExt> hint_sealed::Sealed for (U, D, S, X) {}
impl<U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue, X: HintExt> SchedulingHint for (U, D, S, X) {}

pub struct Immediate; impl UrgencyValue for Immediate {}
pub struct Divisible; impl DivisibilityValue for Divisible {}
pub struct Major;     impl SignificanceValue for Major {}
pub struct TileFusable; impl HintExt for TileFusable {}  // a downstream runner's own dimension

fn takes_hint<H: SchedulingHint>() {}
fn main() {
    takes_hint::<(Immediate, Divisible, Major)>();              // existing, unchanged
    takes_hint::<(Immediate, Divisible, Major, TileFusable)>(); // extended
}
