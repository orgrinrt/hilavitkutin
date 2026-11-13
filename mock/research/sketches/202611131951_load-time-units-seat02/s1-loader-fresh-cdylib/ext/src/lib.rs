//! Spike: the smallest extension the shipped macro emits, built as a cdylib so the
//! host test can load a library that is not already mapped into the process.

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
pub struct SeatTwoProbe;
