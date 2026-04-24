//! `init = T` but `T` does not implement `InitHandler`.

use hilavitkutin_extensions_macros::export_extension;

pub struct NotAnInitHandler;

#[export_extension(init = NotAnInitHandler)]
pub struct InvalidInit;

fn main() {}
