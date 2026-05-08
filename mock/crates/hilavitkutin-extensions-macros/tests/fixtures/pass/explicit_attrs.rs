//! Explicit `name` and `version` overrides.

use hilavitkutin_extensions_macros::export_extension;

#[export_extension(name = "my-named-ext", version = "2.3.4")]
pub struct NamedExtension;

fn main() {}
