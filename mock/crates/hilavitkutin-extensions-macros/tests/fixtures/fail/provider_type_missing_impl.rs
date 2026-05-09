//! Provider type does not implement `ProviderExport`.

use hilavitkutin_extensions_macros::export_extension;

pub struct NotACap;

#[export_extension(providers = [NotACap])]
pub struct InvalidCap;

fn main() {}
