//! The control. The same library mapped first by dlopen with this libc's own
//! flags, then loaded through the shipped loader. This is the condition the
//! loader's own suite runs under (`hilavitkutin-linking/tests/basic.rs` loads the
//! libc that is already mapped into every process), so it shows why that suite is
//! green.

use hilavitkutin_linking::Library;
use notko::Outcome;

#[test]
fn the_shipped_loader_opens_a_library_only_once_something_else_has() {
    let mut path = std::env::var("EXT_SO").expect("EXT_SO").into_bytes();
    path.push(0);

    let raw = unsafe {
        libc::dlopen(path.as_ptr() as *const libc::c_char, libc::RTLD_NOW | libc::RTLD_LOCAL)
    };
    eprintln!("raw dlopen with this libc's RTLD_NOW | RTLD_LOCAL: {}", if raw.is_null() { "null" } else { "mapped" });
    assert!(!raw.is_null(), "control: the library itself is loadable");

    let sym = unsafe { libc::dlsym(raw, c"__hilavitkutin_extension_descriptor".as_ptr()) };
    assert!(!sym.is_null(), "control: the macro-emitted descriptor symbol is exported");

    let after = match Library::load(&path) {
        Outcome::Ok(_) => "ok".to_string(),
        Outcome::Err(e) => format!("err {}", s1_host::link_label(&e)),
    };
    eprintln!("hilavitkutin_linking::Library::load once the library is mapped: {after}");
    assert_eq!(after, "ok");
}
