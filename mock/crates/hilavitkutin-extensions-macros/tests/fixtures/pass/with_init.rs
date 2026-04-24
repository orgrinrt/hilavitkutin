//! `init = TypePath` wires an `InitHandler` impl into the descriptor's
//! `init_fn` slot via a trampoline.

use core::ffi::c_void;
use hilavitkutin_extensions::{ExtensionAbiStatus, InitHandler};
use hilavitkutin_extensions_macros::export_extension;

pub struct Init;

impl InitHandler for Init {
    unsafe fn init(_host_ctx: *mut c_void) -> ExtensionAbiStatus {
        ExtensionAbiStatus::Ok
    }
}

#[export_extension(init = Init)]
pub struct ExtensionWithInit;

fn main() {}
