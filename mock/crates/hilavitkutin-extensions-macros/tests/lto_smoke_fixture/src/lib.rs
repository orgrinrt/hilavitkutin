//! Standalone cdylib fixture exercising `#[export_extension]` end-to-end
//! under `lto = "fat"`. The parent crate's integration test
//! (`tests/lto_smoke.rs`) builds this and inspects the resulting symbol
//! table to verify that trampolines survive aggressive LTO stripping.

#![no_std]

use core::ffi::c_void;
use hilavitkutin_extensions::{ExtensionAbiStatus, InitHandler, ShutdownHandler};
use hilavitkutin_extensions_macros::export_extension;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub struct Init;

impl InitHandler for Init {
    unsafe fn init(_host_ctx: *mut c_void) -> ExtensionAbiStatus {
        ExtensionAbiStatus::Ok
    }
}

pub struct Shutdown;

impl ShutdownHandler for Shutdown {
    unsafe fn shutdown(_host_ctx: *mut c_void) -> ExtensionAbiStatus {
        ExtensionAbiStatus::Ok
    }
}

#[export_extension(init = Init, shutdown = Shutdown)]
pub struct LtoSmokeExtension;
