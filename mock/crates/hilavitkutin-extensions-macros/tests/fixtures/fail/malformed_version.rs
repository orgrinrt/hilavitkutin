//! Version string not conforming to MAJOR.MINOR.PATCH.

use hilavitkutin_extensions_macros::export_extension;

#[export_extension(version = "not-a-semver")]
pub struct BadVersion;

fn main() {}
