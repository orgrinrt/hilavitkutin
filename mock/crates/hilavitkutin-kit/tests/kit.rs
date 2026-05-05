//! Crate-isolation smoke test for `hilavitkutin-kit`.
//!
//! Round 4 declarative Kit shape: type-level only, no install body,
//! no builder transformation.

#![no_std]

use hilavitkutin_api::Empty;
use hilavitkutin_kit::Kit;

struct DummyKit;

impl Kit for DummyKit {
    type Units = Empty;
    type Owned = Empty;
}

#[test]
fn kit_trait_shape_compiles_standalone() {
    fn _type_check_only<K: Kit>() {}
    _type_check_only::<DummyKit>();
}
