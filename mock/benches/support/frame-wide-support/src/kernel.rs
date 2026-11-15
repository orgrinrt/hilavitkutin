//! The per-record arithmetic, and the constants that keep the twelve stages
//! apart.
//!
//! The regime is a type rather than a runtime flag because a `WorkUnit`'s
//! `execute` sees only its context, so the kernel has to be part of the unit's
//! type identity. The twelve stage constants are here for the same reason the
//! frame is in one place: two stages computing the same function of their
//! input would make a dropped stage invisible to the digest.
//!
//! Nothing here reaches a sibling module: this is the bottom of the crate.

/// How much per-record work each stage does.
///
/// The answer to what this frame costs differs by regime and one number for
/// both would be a lie. `Heavy` gives each record real compute, so per-record
/// work dominates dispatch. `Light` makes each stage a single wrapping
/// multiply, so the frame is memory-bound and the cost of materialising the
/// intermediate columns is the whole story.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Regime {
    /// Real per-record compute: the four-step mix iterated `HEAVY_ROUNDS` times.
    Heavy,
    /// One wrapping multiply per stage.
    Light,
}

/// The regime tag carried in the first word of the input image.
pub const TAG_HEAVY: u32 = 0x4845_4156;
/// The regime tag carried in the first word of the input image.
pub const TAG_LIGHT: u32 = 0x4C49_4748;

impl Regime {
    /// The tag this regime writes into the input image.
    pub const fn tag(self) -> u32 {
        match self {
            Regime::Heavy => TAG_HEAVY,
            Regime::Light => TAG_LIGHT,
        }
    }

    /// The regime a tag names. Panics on a tag no regime writes, which can
    /// only mean the input image and this crate disagree about the layout.
    pub fn from_tag(tag: u32) -> Regime {
        match tag {
            TAG_HEAVY => Regime::Heavy,
            TAG_LIGHT => Regime::Light,
            other => {
                panic!("frame_wide: input carries regime tag {other:#x}, which no regime writes")
            },
        }
    }
}

/// A per-record stage kernel, selected at compile time.
///
/// The regime cannot be a runtime branch inside a work unit: a `WorkUnit`'s
/// `execute` sees only its context, so the kernel has to be part of the unit's
/// type identity. It is a trait rather than a `const HEAVY: bool` because the
/// scheduler type then carries one type parameter instead of threading a
/// boolean through every generic position.
pub trait Kernel: Copy + 'static {
    /// Transform one record under one stage constant.
    fn apply(x: u32, k: u32) -> u32;
}

/// The heavy regime's kernel.
#[derive(Copy, Clone, Debug)]
pub struct Heavy;
/// The light regime's kernel.
#[derive(Copy, Clone, Debug)]
pub struct Light;

/// Knuth multiplicative constants for the heavy mix, as `engine_vs_std` uses
/// them, so a heavy record here costs what a heavy record costs there.
const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
const SH: u32 = 13;
const M4: u32 = 3266489917;

/// How many times the heavy kernel iterates the four-step mix.
pub const HEAVY_ROUNDS: usize = 8;

#[inline(always)]
fn mix(x: u32) -> u32 {
    let a = x.wrapping_mul(M1);
    let b = a.wrapping_mul(M2).wrapping_add(1);
    let c = (b >> SH) ^ b;
    c.wrapping_mul(M4)
}

impl Kernel for Heavy {
    #[inline(always)]
    fn apply(x: u32, k: u32) -> u32 {
        let mut v = x ^ k;
        let mut r = 0;
        while r < HEAVY_ROUNDS {
            v = mix(v);
            r += 1;
        }
        v
    }
}

impl Kernel for Light {
    #[inline(always)]
    fn apply(x: u32, k: u32) -> u32 {
        x.wrapping_mul(k | 1)
    }
}

/// The twelve stage constants, one per work unit, so no two stages compute the
/// same function of their input.
pub const K_A1: u32 = 0x9E37_79B1;
pub const K_A2: u32 = 0x85EB_CA6B;
pub const K_A3: u32 = 0xC2B2_AE35;
pub const K_B1: u32 = 0x27D4_EB2F;
pub const K_B2: u32 = 0x1656_67B1;
pub const K_B3: u32 = 0xD3A2_646D;
pub const K_C1: u32 = 0xFF51_AFD7;
pub const K_C2: u32 = 0x4F27_4143;
pub const K_C3: u32 = 0x2545_F491;
pub const K_J: u32 = 0x7FEB_352D;
pub const K_T1: u32 = 0x846C_A68B;
pub const K_T2: u32 = 0xBF58_476D;
