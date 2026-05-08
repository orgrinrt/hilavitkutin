//! Capability type does not implement `CapabilityExport`.

use hilavitkutin_extensions_macros::export_extension;

pub struct NotACap;

#[export_extension(capabilities = [NotACap])]
pub struct InvalidCap;

fn main() {}
