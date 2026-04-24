//! Minimal valid use: no attribute arguments; version defaults to
//! `CARGO_PKG_VERSION`; name defaults to kebab-case ident.

use hilavitkutin_extensions_macros::export_extension;

#[export_extension]
pub struct MyExtension;

fn main() {}
