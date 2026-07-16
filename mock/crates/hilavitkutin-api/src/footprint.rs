//! Per-store L1-morsel write-footprint contracts (canonical R5, domain 12).
//!
//! The per-fiber morsel-window formula is
//! `morsel = (L1_USABLE / Σ write_sizes).clamp(MIN_MORSEL, MAX_MORSEL) & !3`,
//! where `Σ write_sizes = Σ write_column_sizes + Σ write_resource_collection_sizes`
//! (spec lines 832-842, 548-555). This module supplies the resource side of that
//! sum: how much L1 write budget a resource's value contributes.
//!
//! Per R5, a resource value composes of three field kinds:
//! - `Field<T>` (<=16-byte scalar, register-cached): contributes 0 to the L1
//!   morsel budget (its cost is register pressure, a separate constraint).
//! - `Seq<T, N>` / `Map<K, V, N>` (const-sized arena collections): contribute
//!   their static `N * size_of::<elem>()` footprint when written.
//!
//! `CollectionBytes` reports a single field kind's L1 footprint; `ResourceFootprint`
//! reports a resource value type's total (the sum over its field kinds, `Field`
//! adding 0). A consumer resource value type implements `ResourceFootprint` (a
//! derive is the ergonomic path); the engine's per-store fold reads `L1_BYTES` for
//! `Resource<T>` stores.

use arvo::{Cap, Identity, USize};
use arvo_tensor::cap_size;

use crate::column_value::ColumnValue;
use crate::store::{Field, Map, Seq};

/// Round a bit count up to whole bytes.
const fn bytes_of_bits(bits: USize) -> USize {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte-ceil arithmetic on the const bit width; tracked: #121
    USize((bits.0 + 7) / 8)
}

/// The L1-morsel write-budget byte footprint of one resource field kind.
///
/// `Seq`/`Map` collections report their static `N * elem_bytes`; `Field<T>`
/// scalars report 0 (register-cached, not L1-morsel budget). Element bytes come
/// from `ColumnValue::BIT_WIDTH` (the same hook columns use), ceiled to whole
/// bytes. `N` is an arvo `Cap`, read in const via `cap_size`.
pub trait CollectionBytes {
    /// This field kind's contribution to the L1 morsel write budget.
    const BYTES: USize;
}

impl<U: ColumnValue, const N: Cap> CollectionBytes for Seq<U, N> {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: N*elem const arithmetic; tracked: #121
    const BYTES: USize = USize(cap_size(N) * bytes_of_bits(<U as ColumnValue>::BIT_WIDTH).0);
}

impl<K: ColumnValue, V: ColumnValue, const N: Cap> CollectionBytes for Map<K, V, N> {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: N*(k+v) const arithmetic; tracked: #121
    const BYTES: USize = USize(
        cap_size(N)
            * (bytes_of_bits(<K as ColumnValue>::BIT_WIDTH).0
                + bytes_of_bits(<V as ColumnValue>::BIT_WIDTH).0),
    );
}

impl<T: ColumnValue> CollectionBytes for Field<T> {
    /// A `Field` scalar is register-cached, so it adds nothing to the L1 morsel
    /// write budget (its cost lands in the register budget, a separate axis).
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: zero footprint literal; tracked: #121
    const BYTES: USize = USize(0);
}

/// The total L1-morsel write-budget byte footprint of a resource value type.
///
/// Equal to the sum of its field kinds' `CollectionBytes::BYTES` (so `Seq`/`Map`
/// fields add `N * elem`, `Field` fields add 0). A resource value type implements
/// this (a derive folds the field kinds; a hand impl sums them explicitly). The
/// engine reads `L1_BYTES` for a written `Resource<T>` store when computing the
/// per-fiber morsel window.
pub trait ResourceFootprint {
    /// `Σ` of this resource value type's field-kind footprints.
    const L1_BYTES: USize;
}

// Bare scalar primitives as resource values: the degenerate single-Field
// case. A scalar resource has no Seq/Map collection members, so it
// contributes nothing to the L1 morsel write budget (its cost is register
// pressure, like any Field). Explicit impls, NOT a blanket over
// `ColumnValue`: a blanket would cover every consumer struct and turn the
// `#[derive(ResourceFootprint)]` impl into a coherence conflict.
macro_rules! impl_scalar_footprint {
    ($($t:ty),* $(,)?) => {
        $(
            impl ResourceFootprint for $t {
                const L1_BYTES: USize = USize::ZERO;
            }
        )*
    };
}

impl_scalar_footprint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool, char, ()); // lint:allow(no-bare-numeric) reason: definition-site scalar list for the zero-footprint impls; tracked: #121

impl ResourceFootprint for USize {
    const L1_BYTES: USize = USize::ZERO;
}

impl ResourceFootprint for arvo::Bool {
    const L1_BYTES: USize = USize::ZERO;
}
