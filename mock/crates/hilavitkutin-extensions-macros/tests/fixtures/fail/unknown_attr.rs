//! Unknown attribute key.

use hilavitkutin_extensions_macros::export_extension;

#[export_extension(typo_parameter = "oops")]
pub struct UnknownAttr;

fn main() {}
