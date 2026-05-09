//! Two providers declared via the `providers = [...]` attribute
//! list. Each type implements `ProviderExport`.

use core::ffi::c_void;
use hilavitkutin_extensions::{ProviderExport, ProviderId};
use hilavitkutin_extensions_macros::export_extension;

pub struct CapA;

impl ProviderExport for CapA {
    const ID: ProviderId = ProviderId::from_name("cap.a");
    const VTABLE_PTR: *const c_void = core::ptr::null();
}

pub struct CapB;

impl ProviderExport for CapB {
    const ID: ProviderId = ProviderId::from_name("cap.b");
    const VTABLE_PTR: *const c_void = core::ptr::null();
}

#[export_extension(providers = [CapA, CapB])]
pub struct TwoCapsExtension;

fn main() {}
