//! Compile-time Send/Sync checks for pointer-provenance newtypes.
//!
//! Asserts `ResourcePtr<T: Send>: Send`, `ResourcePtr<T: Sync>: Sync`,
//! and the same for `ColumnPtr<T>`. If the impls regress, monomorph
//! fails to compile and the test crate does not build.

use hilavitkutin::resource::provenance::{ColumnPtr, ResourcePtr};

fn require_send<T: Send>() {}
fn require_sync<T: Sync>() {}

#[test]
fn resource_ptr_is_send_when_t_is_send() {
    require_send::<ResourcePtr<u32>>();
    require_sync::<ResourcePtr<u32>>();
}

#[test]
fn column_ptr_is_send_when_t_is_send() {
    require_send::<ColumnPtr<u32>>();
    require_sync::<ColumnPtr<u32>>();
}
