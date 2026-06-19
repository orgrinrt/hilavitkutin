//! Sketch A3: per-store element byte-size collection + per-fiber write-byte sum.
//!
//! Premise: the A3 per-fiber L1 morsel-window formula needs, per fiber,
//! the sum of element byte sizes over the stores that fiber WRITES. This
//! sketch proves the two unproven plumbing halves:
//!
//!   1. Per-store size collection: a const fold over the global `Stores`
//!      cons-list emitting each store's element byte size into a
//!      `<Stores as Capacity>::Array<USize>`, mirroring the existing
//!      `AccumStoresMask` fold in `plan/project.rs`. Element bits come
//!      from `ColumnValue::BIT_WIDTH`; bytes = ceil(bits / 8).
//!
//!   2. Per-fiber write-byte sum: given a fiber's write `AccessMask<Stores>`
//!      (the existing `BundleMasks` writes output / `project_access_set`)
//!      and the size array from (1), sum the sizes of stores whose bit is
//!      set, yielding the formula's denominator. The clamp + `& !3` rounding
//!      is plain arithmetic on top and is not in doubt; included for
//!      completeness.
//!
//! Mechanism choice: a per-marker `StoreElemBytes` trait with DISJOINT
//! concrete impls (one per store-marker shape), mirroring `StoreAccumKind`
//! in `plan/project.rs`. This dodges the marker-trait coherence wall a
//! blanket-plus-specific pair would hit, and lets each marker extract its
//! own element type `T` and read `<T as ColumnValue>::BIT_WIDTH`.

use arvo::USize;
use arvo::strategy::Identity; // brings USize::ZERO into scope
use arvo_tensor::{cap_size, Capacity, Dim};
use hilavitkutin::plan::project::project_access_set;
use hilavitkutin::plan::AccessMask;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::store::{Column, Resource};

// ----------------------------------------------------------------------
// Part 1: per-store element-byte-size, per-marker (mirrors StoreAccumKind).
// ----------------------------------------------------------------------

/// Element byte size of a store marker's value type.
///
/// Disjoint concrete impls per store-marker shape (no blanket), so the
/// fold stays clear of the marker-trait coherence wall, exactly as
/// `StoreAccumKind` does. Each impl pulls the inner `T` and rounds its
/// `ColumnValue::BIT_WIDTH` (bits) up to whole bytes.
trait StoreElemBytes {
    /// ceil(BIT_WIDTH / 8) for this store's element type.
    const BYTES: USize;
}

// Round bits up to whole bytes. `(bits + 7) / 8`.
const fn bytes_of_bits(bits: USize) -> USize {
    USize((bits.0 + 7) / 8) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch; byte-ceil arithmetic; tracked: #121
}

impl<T: ColumnValue> StoreElemBytes for Column<T> {
    const BYTES: USize = bytes_of_bits(<T as ColumnValue>::BIT_WIDTH);
}

impl<T: ColumnValue> StoreElemBytes for Resource<T> {
    const BYTES: USize = bytes_of_bits(<T as ColumnValue>::BIT_WIDTH);
}

// (Virtual / Accum / StagedResource would each get a disjoint impl in the
// real A3; Virtual<T> is a fired marker carrying no record bytes, so its
// BYTES would be USize::ZERO. Two markers suffice to prove the mechanism.)

/// Fold the global `Stores` cons-list into a `CS`-capacity byte-size array:
/// `out[i]` = element byte size of store `i`. Mirrors `AccumStoresMask`'s
/// per-store walk, but writes a size into an array slot instead of setting
/// a mask bit.
trait StoreSizes<CS: Capacity> {
    /// Write each store's byte size into `out`, walking from store `idx`.
    fn fill_sizes(out: &mut [USize], idx: USize);
}

impl<CS: Capacity> StoreSizes<CS> for Empty {
    #[inline]
    fn fill_sizes(_out: &mut [USize], _idx: USize) {}
}

impl<H: StoreElemBytes, T: StoreSizes<CS>, CS: Capacity> StoreSizes<CS> for Cons<H, T> {
    #[inline]
    fn fill_sizes(out: &mut [USize], idx: USize) {
        out[idx.0] = <H as StoreElemBytes>::BYTES; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch; slice index; tracked: #121
        <T as StoreSizes<CS>>::fill_sizes(out, USize(idx.0 + 1)); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch; position successor; tracked: #121
    }
}

/// Build the per-store byte-size array for a global `Stores` list.
fn store_sizes<Stores, CS: Capacity>() -> <CS as Capacity>::Array<USize>
where
    Stores: StoreSizes<CS>,
    <CS as Capacity>::Array<USize>: Copy,
{
    let mut arr = <CS as Capacity>::filled(USize::ZERO);
    <Stores as StoreSizes<CS>>::fill_sizes(arr.as_mut(), USize::ZERO);
    arr
}

// ----------------------------------------------------------------------
// Part 2: per-fiber write-byte sum + the clamp/round formula.
// ----------------------------------------------------------------------

const MIN_MORSEL: USize = USize(64); // lint:allow(no-bare-numeric) reason: sketch placeholder for RunCfg assoc const; tracked: #121
const MAX_MORSEL: USize = USize(8192); // lint:allow(no-bare-numeric) reason: sketch placeholder; tracked: #121
const L1_USABLE: USize = USize(24576); // lint:allow(no-bare-numeric) reason: sketch placeholder (24KiB of a 32KiB L1); tracked: #121

/// Sum the element byte sizes of the stores whose bit is set in `write_mask`.
fn write_bytes_of_fiber<CS: Capacity>(
    write_mask: &AccessMask<CS>,
    sizes: &[USize],
) -> USize {
    let mut total = 0usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch accumulator; tracked: #121
    let mut i = 0usize; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch index; tracked: #121
    let cap = cap_size(<CS as Capacity>::CAP);
    while i < cap && i < 64 && i < sizes.len() {
        if write_mask.contains(USize(i)).0 {
            total += sizes[i].0;
        }
        i += 1;
    }
    USize(total)
}

/// The A3 window formula: (L1_usable / write_bytes).clamp(MIN, MAX) & !3.
fn morsel_window(write_bytes: USize) -> USize {
    if write_bytes.0 == 0 {
        return MAX_MORSEL; // no writes: no L1 pressure, take the max.
    }
    let raw = L1_USABLE.0 / write_bytes.0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch division; tracked: #121
    let clamped = if raw < MIN_MORSEL.0 {
        MIN_MORSEL.0
    } else if raw > MAX_MORSEL.0 {
        MAX_MORSEL.0
    } else {
        raw
    };
    USize(clamped & !3usize) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch; round down to multiple of 4; tracked: #121
}

// ----------------------------------------------------------------------
// Fixture: a 2-fiber scenario with KNOWN column element types.
// ----------------------------------------------------------------------

// arvo fixed-point element types of known width (non-power-of-two to make
// the byte-ceil visible). UFixed<I,F,S> lowers to a sized container.
type Pos = arvo::Uint<14, arvo::strategy::Warm>; // 14 bits, lowers to a 2-byte container
type Vel = arvo::Uint<11, arvo::strategy::Warm>; // 11 bits, lowers to a 2-byte container
type Mass = arvo::Uint<27, arvo::strategy::Cold>; // 27 bits

// Store markers over those element types. Positions in `Stores`:
//   SPos = 0, SVel = 1, SMass = 2, SCfg = 3.
type SPos = Column<Pos>;
type SVel = Column<Vel>;
type SMass = Column<Mass>;
type SCfg = Resource<arvo::Bool>;

type Stores = Cons<SPos, Cons<SVel, Cons<SMass, Cons<SCfg, Empty>>>>;
type StoresCap = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sketch store capacity; tracked: #649

fn main() {
    // --- Part 1: the per-store size array. ---
    let sizes_arr = store_sizes::<Stores, StoresCap>();
    let sizes = sizes_arr.as_ref();

    // size_of for the lowered containers. The exact byte count is whatever
    // the UFixed/IFixed/Bool repr-transparent container lowers to; we read
    // it the same way ColumnValue does (size_of). Compute expected directly.
    let exp_pos = core::mem::size_of::<Pos>();
    let exp_vel = core::mem::size_of::<Vel>();
    let exp_mass = core::mem::size_of::<Mass>();
    let exp_cfg = core::mem::size_of::<arvo::Bool>();

    assert_eq!(sizes[0].0, exp_pos, "store 0 (Pos) byte size");
    assert_eq!(sizes[1].0, exp_vel, "store 1 (Vel) byte size");
    assert_eq!(sizes[2].0, exp_mass, "store 2 (Mass) byte size");
    assert_eq!(sizes[3].0, exp_cfg, "store 3 (Cfg) byte size");
    // slack tail past the 4 live stores stays zero.
    assert_eq!(sizes[4].0, 0, "slack slot 4 is zero");

    // --- Part 2a: per-fiber write masks via the existing projection. ---
    // Fiber 0 (HEAVY) writes {SPos, SVel, SMass}; fiber 1 (LIGHT) writes {SCfg}.
    type F0Writes = Cons<SPos, Cons<SVel, Cons<SMass, Empty>>>;
    type F1Writes = Cons<SCfg, Empty>;
    let f0_mask: AccessMask<StoresCap> =
        project_access_set::<F0Writes, Stores, _, StoresCap>();
    let f1_mask: AccessMask<StoresCap> =
        project_access_set::<F1Writes, Stores, _, StoresCap>();

    // --- Part 2b: the per-fiber write-byte sums. ---
    let f0_bytes = write_bytes_of_fiber(&f0_mask, sizes);
    let f1_bytes = write_bytes_of_fiber(&f1_mask, sizes);
    assert_eq!(f0_bytes.0, exp_pos + exp_vel + exp_mass, "fiber 0 = Pos + Vel + Mass bytes");
    assert_eq!(f1_bytes.0, exp_cfg, "fiber 1 = Cfg bytes");
    assert!(f0_bytes.0 > f1_bytes.0, "fiber 0 has the heavier write footprint");

    // --- Part 2c: the window formula (arithmetic, sanity only). ---
    let w0 = morsel_window(f0_bytes);
    let w1 = morsel_window(f1_bytes);
    assert_eq!(w0.0 % 4, 0, "window 0 is a multiple of 4");
    assert_eq!(w1.0 % 4, 0, "window 1 is a multiple of 4");
    assert!(w0.0 >= MIN_MORSEL.0 && w0.0 <= MAX_MORSEL.0, "window 0 clamped");
    assert!(w1.0 >= MIN_MORSEL.0 && w1.0 <= MAX_MORSEL.0, "window 1 clamped");
    // The HEAVIER fiber 0 gets a window no wider than the lighter fiber 1.
    assert!(w0.0 <= w1.0, "heavier fiber gets a narrower-or-equal window");

    // success.
    let _ = (w0, w1);
}
