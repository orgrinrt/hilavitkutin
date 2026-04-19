//! Scheduling hints.
//!
//! Three orthogonal axes: urgency, divisibility, significance. Each
//! axis has its own sealed trait and a fixed set of ZST carrier
//! types. A WU declares `type Hint = (U, D, S)` where each position
//! is a marker on the matching axis.
//!
//! Lower discriminant is lower priority. Tie-break by most deps
//! first, then deterministic fallback.

mod hint_sealed {
    /// Hint-subsystem private seal. Separate from the crate-level
    /// `sealed::Sealed` to avoid clashing with the tuple impls in
    /// `access.rs`.
    pub trait Sealed {} // lint:allow(undocumented_type)
}

/// Axis 1: how soon the WU must run.
pub trait UrgencyValue: hint_sealed::Sealed + 'static {
    /// Discriminant. Higher = higher urgency.
    const VALUE: u8;
}

/// Axis 2: whether the WU can be split or paused.
pub trait DivisibilityValue: hint_sealed::Sealed + 'static {
    /// Discriminant. Higher = more rigid scheduling.
    const VALUE: u8;
}

/// Axis 3: relative importance of the WU's output.
pub trait SignificanceValue: hint_sealed::Sealed + 'static {
    /// Discriminant. Higher = more significant.
    const VALUE: u8;
}

/// A full scheduling hint: one marker per axis.
///
/// Sealed. The only implementer is the 3-tuple
/// `(U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue)`.
pub trait SchedulingHint: hint_sealed::Sealed + 'static {}

impl<U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue> hint_sealed::Sealed
    for (U, D, S)
{
}
impl<U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue> SchedulingHint for (U, D, S) {}

// ---- Urgency markers -------------------------------------------------

/// Run before any scheduled work.
#[derive(Copy, Clone, Default, Debug)]
pub struct Immediate;
impl hint_sealed::Sealed for Immediate {}
impl UrgencyValue for Immediate {
    const VALUE: u8 = 255;
}

/// Run at the frame's steady cadence.
#[derive(Copy, Clone, Default, Debug)]
pub struct Steady;
impl hint_sealed::Sealed for Steady {}
impl UrgencyValue for Steady {
    const VALUE: u8 = 170;
}

/// Run when convenient.
#[derive(Copy, Clone, Default, Debug)]
pub struct Relaxed;
impl hint_sealed::Sealed for Relaxed {}
impl UrgencyValue for Relaxed {
    const VALUE: u8 = 85;
}

/// Run at idle only.
#[derive(Copy, Clone, Default, Debug)]
pub struct Deferred;
impl hint_sealed::Sealed for Deferred {}
impl UrgencyValue for Deferred {
    const VALUE: u8 = 0;
}

// ---- Divisibility markers --------------------------------------------

/// Indivisible, must run to completion on one core.
#[derive(Copy, Clone, Default, Debug)]
pub struct Atomic;
impl hint_sealed::Sealed for Atomic {}
impl DivisibilityValue for Atomic {
    const VALUE: u8 = 255;
}

/// Splittable at morsel boundaries.
#[derive(Copy, Clone, Default, Debug)]
pub struct Adaptive;
impl hint_sealed::Sealed for Adaptive {}
impl DivisibilityValue for Adaptive {
    const VALUE: u8 = 128;
}

/// Pausable mid-morsel.
#[derive(Copy, Clone, Default, Debug)]
pub struct Interruptible;
impl hint_sealed::Sealed for Interruptible {}
impl DivisibilityValue for Interruptible {
    const VALUE: u8 = 0;
}

// ---- Significance markers --------------------------------------------

/// System correctness depends on this WU.
#[derive(Copy, Clone, Default, Debug)]
pub struct Critical;
impl hint_sealed::Sealed for Critical {}
impl SignificanceValue for Critical {
    const VALUE: u8 = 255;
}

/// High-value output; skipping degrades results.
#[derive(Copy, Clone, Default, Debug)]
pub struct Important;
impl hint_sealed::Sealed for Important {}
impl SignificanceValue for Important {
    const VALUE: u8 = 192;
}

/// Default significance.
#[derive(Copy, Clone, Default, Debug)]
pub struct Normal;
impl hint_sealed::Sealed for Normal {}
impl SignificanceValue for Normal {
    const VALUE: u8 = 128;
}

/// Run when slack permits.
#[derive(Copy, Clone, Default, Debug)]
pub struct Opportunistic;
impl hint_sealed::Sealed for Opportunistic {}
impl SignificanceValue for Opportunistic {
    const VALUE: u8 = 64;
}

/// Drop under pressure.
#[derive(Copy, Clone, Default, Debug)]
pub struct Optional;
impl hint_sealed::Sealed for Optional {}
impl SignificanceValue for Optional {
    const VALUE: u8 = 0;
}
