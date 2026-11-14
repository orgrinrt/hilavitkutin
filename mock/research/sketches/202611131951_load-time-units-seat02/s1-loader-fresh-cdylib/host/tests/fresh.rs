//! A library that is not mapped into the process yet, loaded through the shipped
//! loader and through the shipped host. Its own process (one test file, one
//! test), so no other test has mapped the library first.
//!
//! The library path comes from `EXT_SO`, the cdylib built from `../ext`.

use hilavitkutin_extensions::{ExtensionHost, ExtensionRequirement};
use hilavitkutin_linking::Library;
use notko::{Maybe, Outcome};

fn ext_path() -> Vec<u8> {
    let mut p = std::env::var("EXT_SO")
        .expect("EXT_SO names the cdylib built from ../ext")
        .into_bytes();
    p.push(0);
    p
}

#[test]
fn a_fresh_extension_loads_through_the_shipped_loader_and_host() {
    let path = ext_path();

    let linking = match Library::load(&path) {
        Outcome::Ok(_lib) => "ok".to_string(),
        Outcome::Err(e) => format!("err {}", s1_host::link_label(&e)),
    };
    eprintln!("hilavitkutin_linking::Library::load on a fresh library: {linking}");

    let host = ExtensionHost::new(&[]);
    let hosted = match host.load(&path, ExtensionRequirement::Required, core::ptr::null_mut()) {
        Outcome::Ok(Maybe::Is(ext)) => format!("ok, name {:?}", core::str::from_utf8(ext.name())),
        Outcome::Ok(Maybe::Isnt) => "ok but nothing loaded".to_string(),
        Outcome::Err(e) => format!("err {}", s1_host::ext_label(&e)),
    };
    eprintln!("hilavitkutin_extensions::ExtensionHost::load on a fresh library: {hosted}");

    assert_eq!(linking, "ok", "the shipped loader cannot open a library it has not already got");
    assert!(hosted.starts_with("ok, name"), "the shipped host cannot load the extension its own macro emits");
}

#[cfg(target_os = "linux")]
#[test]
fn the_flag_the_loader_passes_as_local_is_glibc_noload() {
    // The loader passes 4 as RTLD_LOCAL (src/backend/unix.rs:9). These are glibc's
    // own values, read from the libc crate, not restated by hand.
    eprintln!(
        "glibc: RTLD_LOCAL={} RTLD_NOLOAD={} RTLD_NOW={} RTLD_GLOBAL={}",
        libc::RTLD_LOCAL,
        libc::RTLD_NOLOAD,
        libc::RTLD_NOW,
        libc::RTLD_GLOBAL
    );
    assert_ne!(4, libc::RTLD_NOLOAD, "4 is RTLD_NOLOAD on this libc, so the loader asks dlopen not to load");
}
