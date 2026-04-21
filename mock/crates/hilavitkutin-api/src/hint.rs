//! Scheduling hints.
//!
//! Three orthogonal axes: urgency, divisibility, significance. Each
//! axis has its own sealed trait, its own exact-width `UFixed` alias,
//! and a fixed set of ZST carrier types. A WU declares
//! `type Hint = (U, D, S)` where each position is a marker on the
//! matching axis.
//!
//! Higher discriminant = higher priority. Tie-break by most deps
//! first, then deterministic fallback.

use arvo::{FBits, IBits, UFixed, strategy::Hot};

mod hint_sealed {
    /// Hint-subsystem private seal. Separate from the crate-level
    /// `sealed::Sealed` to avoid clashing with the tuple impls in
    /// `access.rs`.
    pub trait Sealed {} // lint:allow(undocumented_type)
}

/// How soon the WU must run. 4 levels fit in 2 bits.
pub type Urgency = UFixed<{ IBits(2) }, { FBits::ZERO }, Hot>;

/// Whether the WU can be split or paused. 3 levels fit in 2 bits.
pub type Divisibility = UFixed<{ IBits(2) }, { FBits::ZERO }, Hot>;

/// Relative importance of the WU's output. 5 levels fit in 3 bits.
pub type Significance = UFixed<{ IBits(3) }, { FBits::ZERO }, Hot>;

/// Axis 1: how soon the WU must run.
pub trait UrgencyValue: hint_sealed::Sealed + 'static {
    /// Discriminant. Higher = higher urgency.
    const VALUE: Urgency;
}

/// Axis 2: whether the WU can be split or paused.
pub trait DivisibilityValue: hint_sealed::Sealed + 'static {
    /// Discriminant. Higher = more rigid scheduling.
    const VALUE: Divisibility;
}

/// Axis 3: relative importance of the WU's output.
pub trait SignificanceValue: hint_sealed::Sealed + 'static {
    /// Discriminant. Higher = more significant.
    const VALUE: Significance;
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
    const VALUE: Urgency = Urgency::from_raw(3);
}

/// Run at the frame's steady cadence.
#[derive(Copy, Clone, Default, Debug)]
pub struct Steady;
impl hint_sealed::Sealed for Steady {}
impl UrgencyValue for Steady {
    const VALUE: Urgency = Urgency::from_raw(2);
}

/// Run when convenient.
#[derive(Copy, Clone, Default, Debug)]
pub struct Relaxed;
impl hint_sealed::Sealed for Relaxed {}
impl UrgencyValue for Relaxed {
    const VALUE: Urgency = Urgency::from_raw(1);
}

/// Run at idle only.
#[derive(Copy, Clone, Default, Debug)]
pub struct Deferred;
impl hint_sealed::Sealed for Deferred {}
impl UrgencyValue for Deferred {
    const VALUE: Urgency = Urgency::from_raw(0);
}

// ---- Divisibility markers --------------------------------------------

/// Indivisible, must run to completion on one core.
#[derive(Copy, Clone, Default, Debug)]
pub struct Atomic;
impl hint_sealed::Sealed for Atomic {}
impl DivisibilityValue for Atomic {
    const VALUE: Divisibility = Divisibility::from_raw(2);
}

/// Splittable at morsel boundaries.
#[derive(Copy, Clone, Default, Debug)]
pub struct Adaptive;
impl hint_sealed::Sealed for Adaptive {}
impl DivisibilityValue for Adaptive {
    const VALUE: Divisibility = Divisibility::from_raw(1);
}

/// Pausable mid-morsel.
#[derive(Copy, Clone, Default, Debug)]
pub struct Interruptible;
impl hint_sealed::Sealed for Interruptible {}
impl DivisibilityValue for Interruptible {
    const VALUE: Divisibility = Divisibility::from_raw(0);
}

// ---- Significance markers --------------------------------------------

/// System correctness depends on this WU.
#[derive(Copy, Clone, Default, Debug)]
pub struct Critical;
impl hint_sealed::Sealed for Critical {}
impl SignificanceValue for Critical {
    const VALUE: Significance = Significance::from_raw(4);
}

/// High-value output; skipping degrades results.
#[derive(Copy, Clone, Default, Debug)]
pub struct Important;
impl hint_sealed::Sealed for Important {}
impl SignificanceValue for Important {
    const VALUE: Significance = Significance::from_raw(3);
}

/// Default significance.
#[derive(Copy, Clone, Default, Debug)]
pub struct Normal;
impl hint_sealed::Sealed for Normal {}
impl SignificanceValue for Normal {
    const VALUE: Significance = Significance::from_raw(2);
}

/// Run when slack permits.
#[derive(Copy, Clone, Default, Debug)]
pub struct Opportunistic;
impl hint_sealed::Sealed for Opportunistic {}
impl SignificanceValue for Opportunistic {
    const VALUE: Significance = Significance::from_raw(1);
}

/// Drop under pressure.
#[derive(Copy, Clone, Default, Debug)]
pub struct Optional;
impl hint_sealed::Sealed for Optional {}
impl SignificanceValue for Optional {
    const VALUE: Significance = Significance::from_raw(0);
}
