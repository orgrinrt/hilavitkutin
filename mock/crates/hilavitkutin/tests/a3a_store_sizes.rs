//! Per-fiber morsel A3: `store_sizes` collects per-store L1-morsel byte sizes.
//!
//! The fold over a global `Stores` cons-list writes each store's L1-morsel
//! footprint into the slot at its store position: columns at
//! `ceil(ColumnValue::BIT_WIDTH / 8)` (type-native stride), resource values at
//! their `ResourceFootprint::L1_BYTES` (the Seq/Map collection footprint; a
//! bare-scalar resource is 0, per R5 its Field cost rides the register
//! budget), accumulators and virtuals zero. Slack slots past the live stores
//! stay zero. A3b sums these over a fiber's write-mask union to drive the L1
//! morsel-window formula. Non-power-of-two column widths keep the byte-ceil
//! visible.

use arvo_tensor::Dim;
use hilavitkutin::plan::project::store_sizes;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::{Column, Resource};

type Pos = arvo::Uint<14, arvo::strategy::Warm>;
type Vel = arvo::Uint<11, arvo::strategy::Warm>;
type Mass = arvo::Uint<27, arvo::strategy::Cold>;

type Stores = Cons<Column<Pos>, Cons<Column<Vel>, Cons<Column<Mass>, Cons<Resource<arvo::Bool>, Empty>>>>;
type StoresCap = Dim<8>;

#[test]
fn store_sizes_collects_per_store_element_bytes() {
    let arr = store_sizes::<Stores, StoresCap>();
    let sizes = arr.as_ref();
    assert_eq!(sizes[0].0, core::mem::size_of::<Pos>(), "store 0 (Pos) byte size");
    assert_eq!(sizes[1].0, core::mem::size_of::<Vel>(), "store 1 (Vel) byte size");
    assert_eq!(sizes[2].0, core::mem::size_of::<Mass>(), "store 2 (Mass) byte size");
    assert_eq!(sizes[3].0, 0, "store 3: a bare-scalar resource has no Seq/Map members, 0 L1 bytes");
    assert_eq!(sizes[4].0, 0, "slack slot past the four live stores stays zero");
}
