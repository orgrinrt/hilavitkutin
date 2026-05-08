//! Two capabilities declared via the `capabilities = [...]` attribute
//! list. Each type implements `CapabilityExport`.

use core::ffi::c_void;
use hilavitkutin_extensions::{CapabilityExport, CapabilityId};
use hilavitkutin_extensions_macros::export_extension;

pub struct CapA;

impl CapabilityExport for CapA {
    const ID: CapabilityId = CapabilityId::from_name("cap.a");
    const VTABLE_PTR: *const c_void = core::ptr::null();
}

pub struct CapB;

impl CapabilityExport for CapB {
    const ID: CapabilityId = CapabilityId::from_name("cap.b");
    const VTABLE_PTR: *const c_void = core::ptr::null();
}

#[export_extension(capabilities = [CapA, CapB])]
pub struct TwoCapsExtension;

fn main() {}
