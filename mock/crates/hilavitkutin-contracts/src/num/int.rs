//! `Int<BITS>` and its accumulator `Wide<BITS>`: the one lane the proxy carries.
//!
//! A signed integer of `BITS` bits, 1 to 64, saturating at its range, which is how kaski's lanes
//! adapt. It is a stand-in for arvo's `Int<BITS, A>` under a saturating adaptation and goes at the
//! swap; frozen authors never name it, since they are generic over [`Lane`]. Tests and the
//! cross-target tool name it.
//!
//! The accumulator is a register of `2 * BITS` bits. Its term bound is `2^BITS - 1`: that many
//! members of the widest magnitude, `2^(BITS - 1)` (a subtracted minimum), sum to
//! `2^(2 * BITS - 1) - 2^(BITS - 1)`, which the register holds, and one more does not fit. Held at
//! the platform count's maximum where that is smaller. Past the bound a debug build asserts, and a
//! release build wraps the register in two's complement.

use super::{Additive, Bool, Identity, Lane, USize};
use crate::digest::{Absorb, Digest};
use crate::domain::{Cell64, FormatId, Value, Word};

/// A signed lane value of `BITS` bits, saturating at its range.
// FIXME(69b1c00469a6): a stand-in for arvo-numeral's integer, swapped when it lands.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Int<const BITS: u32>(i64);

/// The exact accumulator of `Int<BITS>`: a signed register of `2 * BITS` bits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Wide<const BITS: u32>(i128);

impl<const BITS: u32> Int<BITS> {
    const HIGH: i64 = if BITS == 64 { i64::MAX } else { (1i64 << (BITS - 1)) - 1 };
    const LOW: i64 = {
        assert!(BITS >= 1 && BITS <= 64, "an Int is 1 to 64 bits wide");
        if BITS == 64 { i64::MIN } else { -(1i64 << (BITS - 1)) }
    };
    /// The largest member.
    pub const MAX: Self = Self(Self::HIGH);
    /// The smallest member.
    pub const MIN: Self = Self(Self::LOW);

    /// The member `V`; a value outside the range fails the build.
    pub const fn constant<const V: i64>() -> Self {
        const {
            assert!(
                V >= Self::LOW && V <= Self::HIGH,
                "the constant is not a member"
            )
        };
        Self(V)
    }

    /// The member nearest to `raw`: total, saturating at the range.
    pub const fn from_raw(raw: i64) -> Self {
        Self(if raw < Self::LOW {
            Self::LOW
        } else if raw > Self::HIGH {
            Self::HIGH
        } else {
            raw
        })
    }

    /// The unwrap door: the carrier's value.
    pub const fn raw(self) -> i64 {
        self.0
    }

    /// One term of the accumulator.
    pub const fn widen(self) -> Wide<BITS> {
        Wide(self.0 as i128)
    }
}

impl<const BITS: u32> Wide<BITS> {
    /// How many widened members the register holds exactly, each added or subtracted.
    pub const MAX_TERMS: USize =
        USize::from_host(if BITS >= usize::BITS { usize::MAX } else { (1usize << BITS) - 1 });
    const REG_HIGH: i128 = if BITS == 64 { i128::MAX } else { (1i128 << (2 * BITS - 1)) - 1 };
    const REG_LOW: i128 = if BITS == 64 { i128::MIN } else { -(1i128 << (2 * BITS - 1)) };
    /// The empty sum.
    pub const ZERO: Self = Self(0);

    const fn inside(v: Option<i128>) -> Bool {
        match v {
            Some(v) => Bool::of(v >= Self::REG_LOW && v <= Self::REG_HIGH),
            None => Bool::FALSE,
        }
    }

    const fn wrap(v: i128) -> i128 {
        if BITS == 64 {
            v
        } else {
            let shift = 128 - 2 * BITS;
            (v << shift) >> shift
        }
    }

    /// Whether the exact sum is inside the register.
    pub const fn fits_add(self, rhs: Self) -> Bool {
        Self::inside(self.0.checked_add(rhs.0))
    }

    /// Whether the exact difference is inside the register.
    pub const fn fits_sub(self, rhs: Self) -> Bool {
        Self::inside(self.0.checked_sub(rhs.0))
    }

    /// The exact sum where `fits_add`; a debug build asserts it, a release build wraps.
    pub const fn add(self, rhs: Self) -> Self {
        debug_assert!(
            self.fits_add(rhs).get(),
            "the accumulator left its register"
        );
        Self(Self::wrap(self.0.wrapping_add(rhs.0)))
    }

    /// The exact difference where `fits_sub`; a debug build asserts it, a release build wraps.
    pub const fn sub(self, rhs: Self) -> Self {
        debug_assert!(
            self.fits_sub(rhs).get(),
            "the accumulator left its register"
        );
        Self(Self::wrap(self.0.wrapping_sub(rhs.0)))
    }

    /// The one adaptation: the member nearest the sum, saturating at the lane's range.
    pub const fn narrow(self) -> Int<BITS> {
        let low = Int::<BITS>::LOW as i128;
        let high = Int::<BITS>::HIGH as i128;
        Int(if self.0 < low {
            Int::<BITS>::LOW
        } else if self.0 > high {
            Int::<BITS>::HIGH
        } else {
            self.0 as i64
        })
    }
}

impl<const BITS: u32> Identity<Additive> for Int<BITS> {
    const IDENTITY: Self = Self(0);
}

impl<const BITS: u32> Lane for Int<BITS> {
    type Wide = Wide<BITS>;

    const MAX_TERMS: USize = Wide::<BITS>::MAX_TERMS;
    const WIDE_ZERO: Wide<BITS> = Wide::ZERO;

    fn widen(self) -> Wide<BITS> {
        Int::widen(self)
    }

    fn wide_add(a: Wide<BITS>, b: Wide<BITS>) -> Wide<BITS> {
        a.add(b)
    }

    fn wide_sub(a: Wide<BITS>, b: Wide<BITS>) -> Wide<BITS> {
        a.sub(b)
    }

    fn narrow(wide: Wide<BITS>) -> Self {
        wide.narrow()
    }
}

impl<const BITS: u32> Value for Int<BITS> {
    type Cell = Cell64;

    const FORMAT: FormatId = FormatId::new(USize::from_host(0x0100 + BITS as usize));

    fn from_cell(cell: Cell64) -> Self {
        Self::from_raw(i64::from_le_bytes(cell.bytes()))
    }

    fn cell(self) -> Cell64 {
        Cell64::of(self.0.to_le_bytes())
    }
}

impl<const BITS: u32> Absorb for Int<BITS> {
    /// As its slot index in sixteen bytes, little-endian, low half first.
    fn absorb(self, into: &mut Digest) {
        let wide = (self.0 as i128).to_le_bytes();
        let mut low = [0u8; 8];
        let mut high = [0u8; 8];
        let mut i = 0;
        while i < 8 {
            low[i] = wide[i];
            high[i] = wide[8 + i];
            i += 1;
        }
        into.word(Word::of(low));
        into.word(Word::of(high));
    }
}
