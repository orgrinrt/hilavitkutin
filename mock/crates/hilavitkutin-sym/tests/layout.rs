#![feature(const_trait_impl)]
//! The eight layout operations, which nothing exercised.
//!
//! `SymLayoutOps` documents eight methods across every shape and no test
//! touched any of them. The width laws that did exist read one field at a time,
//! so an overlap between two fields was invisible to all of them: a law that
//! never reads two fields together cannot see them share a bit.
//!
//! Every case runs against **both** layouts rather than the default alone.
//! `SixteenMinters` shares `SymLayout` with `Standard`, so the two layouts are
//! the whole matrix at this level; the shape-level laws live beside the shapes.

use arvo_bits::{Bit, Bits, Hot};
use hilavitkutin_sym::shape::FieldWidth;
use hilavitkutin_sym::{SymLayout, SymLayoutOps, WideKindLayout};

/// The largest value a field of `width` bits holds.
fn field_max(width: usize) -> u32 {
    (1u32 << width) - 1
}

/// A value with alternating bits, clipped to `width`.
///
/// The case that catches a shift in the wrong direction, which a maximum
/// cannot: every bit set is symmetric, so it survives being reversed.
fn alternating(width: usize) -> u32 {
    0x5555_5555u32 & field_max(width)
}

/// Zero, the maximum, and an alternating pattern. Zero finds a field that
/// never clears; the maximum finds one a bit too narrow; the alternating value
/// finds one shifted the wrong way.
fn boundary_values(width: usize) -> [u32; 3] {
    [0, field_max(width), alternating(width)]
}

macro_rules! layout_suite {
    ($modname:ident, $layout:ty, $kraw:ty, $iraw:ty) => {
        mod $modname {
            use super::*;

            type L = $layout;
            /// Derived rather than restated. A literal here would be one more
            /// hand-written number saying what the layout already says, which
            /// is the defect this file exists to catch.
            type K = <$layout as SymLayoutOps>::Kind;
            type I = <$layout as SymLayoutOps>::Id;

            fn kind_width() -> usize {
                <K as FieldWidth>::WIDTH.0
            }

            fn id_width() -> usize {
                <I as FieldWidth>::WIDTH.0
            }

            /// Every accessor returns what was written, at each boundary.
            #[test]
            fn every_accessor_round_trips_at_every_boundary() {
                for v in boundary_values(kind_width()) {
                    let k = <K>::from_raw(v as $kraw);
                    assert_eq!(
                        L::zeroed().set_kind(k).get_kind(),
                        k,
                        "kind did not survive a round trip at raw {v:#x}"
                    );
                }
                for v in boundary_values(id_width()) {
                    let i = <I>::from_raw(v as $iraw);
                    assert_eq!(
                        L::zeroed().set_id(i).get_id(),
                        i,
                        "id did not survive a round trip at raw {v:#x}"
                    );
                }
                for v in [0u32, 1] {
                    let f = Bit::<Hot>::from_raw(v as u8);
                    assert_eq!(
                        L::zeroed().set_flag(f).get_flag(),
                        f,
                        "flag did not survive a round trip at raw {v:#x}"
                    );
                }
            }

            /// **Writing one field disturbs no other.**
            ///
            /// The overlap case every width law was blind to. Each field is set
            /// to its maximum, which is the value that spills furthest if the
            /// field is one bit too wide or positioned wrongly, and the other
            /// two are read back.
            #[test]
            fn writing_one_field_disturbs_no_other() {
                let k = <K>::from_raw(field_max(kind_width()) as $kraw);
                let i = <I>::from_raw(field_max(id_width()) as $iraw);
                let f = Bit::<Hot>::from_raw(1u8);

                let only_kind = L::zeroed().set_kind(k);
                assert_eq!(
                    only_kind.get_id(),
                    <I>::from_raw(0 as $iraw),
                    "setting kind to its maximum spilled into id"
                );
                assert_eq!(
                    only_kind.get_flag(),
                    Bit::<Hot>::from_raw(0u8),
                    "setting kind to its maximum spilled into the flag"
                );

                let only_id = L::zeroed().set_id(i);
                assert_eq!(
                    only_id.get_kind(),
                    <K>::from_raw(0 as $kraw),
                    "setting id to its maximum spilled into kind"
                );
                assert_eq!(
                    only_id.get_flag(),
                    Bit::<Hot>::from_raw(0u8),
                    "setting id to its maximum spilled into the flag"
                );

                let only_flag = L::zeroed().set_flag(f);
                assert_eq!(
                    only_flag.get_kind(),
                    <K>::from_raw(0 as $kraw),
                    "setting the flag spilled into kind"
                );
                assert_eq!(
                    only_flag.get_id(),
                    <I>::from_raw(0 as $iraw),
                    "setting the flag spilled into id"
                );
            }

            /// All three fields at once occupy the whole handle and nothing
            /// else. This is the other half of the overlap check: disjointness
            /// alone would pass on a layout that leaves a bit unreachable.
            #[test]
            fn the_three_fields_together_are_the_whole_handle() {
                let full = L::zeroed()
                    .set_kind(<K>::from_raw(field_max(kind_width()) as $kraw))
                    .set_id(<I>::from_raw(field_max(id_width()) as $iraw))
                    .set_flag(Bit::<Hot>::from_raw(1u8));

                assert_eq!(
                    full.raw_bits(),
                    Bits::<32, Hot>::from_raw(u32::MAX),
                    "every field at its maximum must leave no bit unset, or a \
                     bit belongs to no field"
                );
                assert_eq!(
                    kind_width() + id_width() + 1,
                    32,
                    "the field widths must account for the whole handle"
                );
            }

            /// A zeroed layout reads zero everywhere, in the fields and in the
            /// raw bits alike.
            #[test]
            fn a_zeroed_layout_reads_zero_everywhere() {
                let z = L::zeroed();
                assert_eq!(z.get_kind(), <K>::from_raw(0 as $kraw));
                assert_eq!(z.get_id(), <I>::from_raw(0 as $iraw));
                assert_eq!(z.get_flag(), Bit::<Hot>::from_raw(0u8));
                assert_eq!(z.raw_bits(), Bits::<32, Hot>::from_raw(0));
            }

            /// **Layout equality is raw-bit equality.**
            ///
            /// The crate's stated central promise, and it was asserted nowhere.
            /// Two layouts agreeing in every field must compare equal, and two
            /// differing in any one field must not, for each field in turn.
            #[test]
            fn layout_equality_is_raw_bit_equality() {
                let k = <K>::from_raw(alternating(kind_width()) as $kraw);
                let i = <I>::from_raw(alternating(id_width()) as $iraw);
                let base = L::zeroed().set_kind(k).set_id(i);

                let same = L::zeroed().set_kind(k).set_id(i);
                assert_eq!(base, same, "equal fields must compare equal");
                assert_eq!(
                    base.raw_bits(),
                    same.raw_bits(),
                    "equal fields must give equal raw bits"
                );

                let other_kind = base.set_kind(<K>::from_raw(field_max(kind_width()) as $kraw));
                assert_ne!(base, other_kind, "a differing kind must not compare equal");

                let other_id = base.set_id(<I>::from_raw(field_max(id_width()) as $iraw));
                assert_ne!(base, other_id, "a differing id must not compare equal");

                let other_flag = base.set_flag(Bit::<Hot>::from_raw(1u8));
                assert_ne!(base, other_flag, "a differing flag must not compare equal");
            }
        }
    };
}

layout_suite!(sym_layout, SymLayout, u8, u32);
layout_suite!(wide_kind_layout, WideKindLayout, u8, u32);

/// **A shape need not derive anything for its handles to compare.**
///
/// `SymShape` requires `Copy + 'static`. A derive on `Sym<S>` would additionally
/// demand `PartialEq + Eq + Hash + Default + Debug` of `S`, so a legal shape
/// deriving less would produce a handle with no equality: this crate's central
/// promise absent by construction, hidden by every shipped shape deriving
/// everything.
///
/// The shape below derives the minimum its own bound requires and nothing else.
/// If the impls regress to a derive, this stops compiling.
#[test]
fn a_shape_deriving_the_minimum_still_yields_a_comparable_handle() {
    use hilavitkutin_sym::{Standard, Sym, SymShape};

    #[derive(Copy, Clone)]
    struct Spartan;

    const impl SymShape for Spartan {
        type Layout = <Standard as SymShape>::Layout;
        type Origin = <Standard as SymShape>::Origin;

        const COUNTER_BITS: arvo::USize = <Standard as SymShape>::COUNTER_BITS;
        const ID_BITS: arvo::USize = <Standard as SymShape>::ID_BITS;
        const KIND_BITS: arvo::USize = <Standard as SymShape>::KIND_BITS;
        const ORIGIN_BITS: arvo::USize = <Standard as SymShape>::ORIGIN_BITS;

        fn origin_index(o: Self::Origin) -> arvo::USize {
            <Standard as SymShape>::origin_index(o)
        }
    }

    let a: Sym<Spartan> = Sym::default();
    let b: Sym<Spartan> = Sym::default();
    assert_eq!(
        a, b,
        "two default handles of a minimal shape must compare equal"
    );

    let c = a.with_flag(Bit::<Hot>::from_raw(1u8));
    assert_ne!(a, c, "handles differing in the flag must not compare equal");
}
