//! Tests of the shape laws in `shape.rs`.

use super::*;

/// Every width a shape declares, summed, must be the whole handle. A shape
/// with a hole or an overlap has every accessor wrong.
///
/// Asserted for every shape rather than for the default alone:
/// choosing which instantiations to check is choosing what not to find out.
#[test]
fn every_shape_accounts_for_all_thirty_two_bits() {
    fn check<S: SymShape>(name: &'static str) {
        let total = S::KIND_BITS.0 + S::ORIGIN_BITS.0 + S::COUNTER_BITS.0 + 1;
        assert_eq!(total, 32, "{name} does not account for the whole handle");
    }
    check::<Standard>("Standard");
    check::<SixteenMinters>("SixteenMinters");
    check::<WideKind>("WideKind");
}

/// The id is the origin and the counter together, for every shape. A shape
/// that says otherwise mints into bits its id accessor cannot read.
#[test]
fn every_shape_id_is_origin_plus_counter() {
    fn check<S: SymShape>(name: &'static str) {
        assert_eq!(
            S::ID_BITS.0,
            S::ORIGIN_BITS.0 + S::COUNTER_BITS.0,
            "{name} id width disagrees with its parts"
        );
    }
    check::<Standard>("Standard");
    check::<SixteenMinters>("SixteenMinters");
    check::<WideKind>("WideKind");
}

/// The kind width genuinely varies across shipped shapes. Without this the
/// trait is a parameter over one value.
///
/// Stated as a difference between two shapes rather than as either one's
/// value, because asserting a constant equals the literal its own impl
/// declares cannot fail.
#[test]
fn the_kind_width_varies_across_shapes() {
    assert_ne!(
        Standard::KIND_BITS.0,
        WideKind::KIND_BITS.0,
        "two shapes must differ in tag width or the trait varies nothing"
    );
}

/// The law that makes every other width mean something.
///
/// A shape declares widths; its layout has them. Nothing tied the two, so a
/// shape could declare a three-bit field is nine bits wide and pass every
/// law, which made the declarations descriptions rather than constraints.
#[test]
fn every_shape_declares_the_widths_its_layout_actually_has() {
    fn check<S: SymShape>(name: &'static str) {
        assert_eq!(
            S::KIND_BITS.0,
            <<S::Layout as SymLayoutOps>::Kind as FieldWidth>::WIDTH.0,
            "{name} declares a kind width its layout does not carry"
        );
        assert_eq!(
            S::ID_BITS.0,
            <<S::Layout as SymLayoutOps>::Id as FieldWidth>::WIDTH.0,
            "{name} declares an id width its layout does not carry"
        );
    }
    check::<Standard>("Standard");
    check::<SixteenMinters>("SixteenMinters");
    check::<WideKind>("WideKind");
}

// The span law that stood here is gone, and its removal is a result rather
// than a loss. It asserted `origin_ceiling - origin_base == 2^COUNTER_BITS
// - 1`, which the derivation now computes: `ceiling` is defined as `base +
// (1 << COUNTER_BITS) - 1`. A test asserting what its subject computes is a
// tautology and cannot fail, so it is deleted rather than kept for the
// count. That the law became tautological is the evidence the derivation
// subsumes it.

/// Whether `S` maps distinct origins to distinct indices.
///
/// A verdict rather than an assertion, so the same law serves the
/// shipped shapes and a shape built to break it. A law that can only assert
/// cannot be pointed at a liar.
///
/// This is all that remains testable about origins. Where a run begins is
/// derived from the index, so overlapping runs are unwritable; what a shape
/// can still get wrong is its own mapping.
#[rustfmt::skip]
fn origins_map_to_distinct_indices<S: SymShape>(origins: &[S::Origin]) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a test-local verdict function; `bool` is the predicate's own return and never crosses a public surface; tracked: #34
    origins.iter().enumerate().all(|(i, a)| {
        origins
            .iter()
            .skip(i + 1)
            .all(|b| S::origin_index(*a).0 != S::origin_index(*b).0)
    })
}

/// And that every index fits the width the shape reserved for it.

/// Every origin of `S` is in range. Delegates to the public
/// [`origin_index_in_range`], which is the one a consumer can reach.
#[rustfmt::skip]
fn origin_indices_fit_origin_bits<S: SymShape>(origins: &[S::Origin]) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a test-local verdict function's own `bool` return; tracked: #34
    origins.iter().all(|o| origin_index_in_range::<S>(*o))
}

#[rustfmt::skip]
fn all_minter_ids() -> [MinterId; 16] {
    core::array::from_fn(|i| MinterId(Uint::<4, Hot>::from_raw(i as u8))) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: enumerating the sixteen origins; the index crosses arvo's own from_raw boundary; tracked: #34
}

#[test]
fn every_shipped_shape_maps_origins_injectively() {
    assert!(origins_map_to_distinct_indices::<Standard>(&[OneOrigin]));
    assert!(origins_map_to_distinct_indices::<WideKind>(&[OneOrigin]));
    assert!(origins_map_to_distinct_indices::<SixteenMinters>(
        &all_minter_ids()
    ));
}

#[test]
fn every_shipped_index_fits_its_origin_bits() {
    assert!(origin_indices_fit_origin_bits::<Standard>(&[OneOrigin]));
    assert!(origin_indices_fit_origin_bits::<WideKind>(&[OneOrigin]));
    assert!(origin_indices_fit_origin_bits::<SixteenMinters>(
        &all_minter_ids()
    ));
}

/// A shape that maps two origins onto one index.
///
/// This is the defect that survives the new mechanism, and it is a
/// smaller one: it is an error in the shape's own mapping rather than in
/// the id-space arithmetic, and it cannot make two distinct minters
/// collide, only make two names for one minter.
///
/// Round eleven's liar, which placed origins past the id field, and the
/// eighth review's, which placed them inside it but overlapping, are both
/// unwritable now: there is no base or ceiling for a shape to supply.
/// That is the result of this round, and it is recorded here because a
/// deleted test otherwise reads as coverage lost rather than as a case that
/// stopped existing.
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq, Debug)]
struct TwoOriginsOneIndex;

const impl SymShape for TwoOriginsOneIndex {
    type Layout = crate::handle::SymLayout;
    type Origin = MinterId;

    const COUNTER_BITS: USize = USize(24);
    const ID_BITS: USize = USize(28);
    const KIND_BITS: USize = USize(3);
    const ORIGIN_BITS: USize = USize(4);

    #[rustfmt::skip]
    fn origin_index(_: Self::Origin) -> USize {
        USize(0) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the deliberate defect this shape carries, mapping every origin onto one index; tracked: #34
    }
}

#[test]
fn the_injectivity_law_rejects_a_shape_that_collapses_its_origins() {
    assert!(
        !origins_map_to_distinct_indices::<TwoOriginsOneIndex>(&all_minter_ids()),
        "a shape mapping every origin onto one index must be rejected"
    );
}

/// And it passes every width law, which is why a width law was never going
/// to be the instrument for this.
#[test]
fn the_collapsing_shape_still_satisfies_every_width_law() {
    type S = TwoOriginsOneIndex;
    assert_eq!(
        S::KIND_BITS.0 + S::ORIGIN_BITS.0 + S::COUNTER_BITS.0 + 1,
        32
    );
    assert_eq!(S::ID_BITS.0, S::ORIGIN_BITS.0 + S::COUNTER_BITS.0);
}

/// Disjointness is arithmetic now, not an assertion.
///
/// Derived runs cannot overlap for distinct indices, checked over every
/// pair a four-bit origin admits. This is the property the two previous
/// rounds each tried to catch with a law aimed at the construction they had
/// just found.
#[test]
fn derived_runs_never_overlap_for_distinct_indices() {
    let ids = all_minter_ids();
    for (i, a) in ids.iter().enumerate() {
        for b in ids.iter().skip(i + 1) {
            let (ab, ac) = (
                origin_base::<SixteenMinters>(*a),
                origin_ceiling::<SixteenMinters>(*a),
            );
            let (bb, bc) = (
                origin_base::<SixteenMinters>(*b),
                origin_ceiling::<SixteenMinters>(*b),
            );
            assert!(
                ac < bb || bc < ab,
                "runs [{ab}, {ac}] and [{bb}, {bc}] overlap"
            );
        }
    }
}

/// A shape whose index runs past the width it reserved.
///
/// The second route the ninth review found, kept rather than reverted.
/// The derived runs are disjoint in `u32` and truncate onto each other
/// inside the id field, so two origins mint one handle while every width
/// law passes. `Generator::at` refuses it, which is where a check binds for
/// shapes nobody has written yet.
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq, Debug)]
struct IndexPastOriginBits;

const impl SymShape for IndexPastOriginBits {
    type Layout = crate::handle::SymLayout;
    type Origin = MinterId;

    const COUNTER_BITS: USize = USize(24);
    const ID_BITS: USize = USize(28);
    const KIND_BITS: USize = USize(3);
    const ORIGIN_BITS: USize = USize(4);

    #[rustfmt::skip]
    fn origin_index(o: Self::Origin) -> USize {
        USize(o.0.to_raw() as usize + 16) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the deliberate defect, an index past the reserved four bits; tracked: #34
    }
}

#[test]
fn the_range_check_rejects_an_index_past_its_origin_bits() {
    assert!(
        !origin_index_in_range::<IndexPastOriginBits>(all_minter_ids()[0]),
        "an index at or above 2^ORIGIN_BITS must be rejected"
    );
}

/// And it passes every width law, which is why a width law was never going
/// to be the instrument for this.
#[test]
fn the_out_of_range_shape_still_satisfies_every_width_law() {
    type S = IndexPastOriginBits;
    assert_eq!(
        S::KIND_BITS.0 + S::ORIGIN_BITS.0 + S::COUNTER_BITS.0 + 1,
        32
    );
    assert_eq!(S::ID_BITS.0, S::ORIGIN_BITS.0 + S::COUNTER_BITS.0);
}
