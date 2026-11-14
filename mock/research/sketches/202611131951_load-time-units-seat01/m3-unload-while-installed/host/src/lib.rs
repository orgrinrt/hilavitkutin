//! Host scaffolding for the M3 spike. Spike, not a shape to copy: names,
//! arities and the closure-shaped uninstall are whatever reached the check.
//!
//! - `open_fresh` maps the extension so that the shipped `Library` holds the
//!   only reference to it. `Library::load` cannot map a fresh image on linux
//!   glibc (it passes 4, `RTLD_NOLOAD`, as `RTLD_LOCAL`:
//!   hilavitkutin-linking/src/backend/unix.rs:9, used at :26), so the
//!   image is mapped first
//!   with libc's own flags, `Library::load` then finds it resident, and the
//!   preload reference is closed again. After that, dropping the `Library`
//!   is the last `dlclose` and the image is unmapped.
//! - `Occupant` is seat 02's s6 shape: a C-ABI kernel pointer in an ordinary
//!   `Resource`. It is `Replaceable` here, not `PlanAffecting`: swapping a
//!   kernel changes no plan input, so it needs a dirty seed and no plan band.
//! - `Installed` is a host-side owner of the library. Its only path to
//!   release the library hands a fallback occupant to the caller's swap
//!   first; dropping it any other way leaks the mapping instead of
//!   unmapping it. Perimeter of that guarantee: it covers the copy of the
//!   kernel pointer the scheduler holds through the swap the caller passes.
//!   `Occupant` is `Copy` and its field is public, so any other copy the host
//!   keeps is outside it.

use core::mem::ManuallyDrop;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use arvo::USize;
use hilavitkutin_linking::Library;
use notko::Outcome;

/// One morsel: input pointer, output pointer, record count.
pub type Kernel = extern "C" fn(*const i64, *mut i64, usize);

/// The occupant of the host unit: whatever kernel the resource holds.
#[derive(Copy, Clone)]
pub struct Occupant(pub Kernel);
impl hilavitkutin_api::store::Replaceable for Occupant {}
impl hilavitkutin_api::footprint::ResourceFootprint for Occupant {
    const L1_BYTES: USize = USize(0);
}

/// The host's own kernel, installed before any library and after one leaves.
pub extern "C" fn host_fallback(a: *const i64, out: *mut i64, len: usize) {
    for i in 0 .. len {
        unsafe { *out.add(i) = *a.add(i) - 1000 };
    }
}

/// A second in-process kernel, for the swap-without-a-plan-band check.
pub extern "C" fn host_double(a: *const i64, out: *mut i64, len: usize) {
    for i in 0 .. len {
        unsafe { *out.add(i) = *a.add(i) * 2 };
    }
}

/// What the extension computes, for checking frames that ran it.
pub fn ext_expected(a: i64) -> i64 {
    a.wrapping_mul(7).wrapping_add(1)
}

/// The extension cdylib in the same profile directory as the running test.
pub fn ext_path() -> PathBuf {
    let exe = std::env::current_exe().expect("exe");
    let profile = exe.parent().and_then(|d| d.parent()).expect("profile dir");
    profile.join("libm3_ext.so")
}

fn c_path() -> Vec<u8> {
    let mut p = ext_path().as_os_str().as_bytes().to_vec();
    p.push(0);
    p
}

/// Whether the extension image is in this process's address space.
pub fn is_mapped() -> bool {
    let maps = std::fs::read_to_string("/proc/self/maps").expect("maps");
    maps.contains("libm3_ext.so")
}

/// Map the extension so the returned `Library` holds the only reference.
pub fn open_fresh() -> Library {
    let p = c_path();
    assert!(
        !is_mapped(),
        "the extension must not be mapped before the test maps it"
    );
    let pre = unsafe {
        libc::dlopen(
            p.as_ptr() as *const libc::c_char,
            libc::RTLD_NOW | libc::RTLD_LOCAL,
        )
    };
    assert!(
        !pre.is_null(),
        "preload {:?}; run `cargo build -p m3-ext` first",
        ext_path()
    );
    let lib = match Library::load(&p) {
        Outcome::Ok(l) => l,
        Outcome::Err(_) => panic!("Library::load refused an image already mapped"),
    };
    assert_eq!(
        unsafe { libc::dlclose(pre) },
        0,
        "close the preload reference"
    );
    assert!(is_mapped(), "the Library keeps the image mapped");
    lib
}

/// Resolve the kernel. `Symbol::get` hands back the bare function pointer by
/// value, with no lifetime: this is the step that lets a kernel outlive its
/// library, whatever `Symbol<'lib, _>` promises.
pub fn kernel_of(lib: &Library) -> Kernel {
    match lib.resolve::<Kernel>(b"ext_kernel\0") {
        Outcome::Ok(s) => s.get(),
        Outcome::Err(_) => panic!("resolve ext_kernel"),
    }
}

/// A host-side owner of one loaded extension.
pub struct Installed {
    lib:    ManuallyDrop<Library>,
    kernel: Kernel,
}

impl Installed {
    pub fn load() -> Self {
        let lib = open_fresh();
        let kernel = kernel_of(&lib);
        Self {
            lib: ManuallyDrop::new(lib),
            kernel,
        }
    }

    pub fn occupant(&self) -> Occupant {
        Occupant(self.kernel)
    }

    /// The one path that releases the library. The fallback is handed to
    /// the caller's swap before the `Library` is dropped, so no occupant the
    /// scheduler holds can point into the unmapped image.
    pub fn uninstall(mut self, swap_in: impl FnOnce(Occupant)) {
        swap_in(Occupant(host_fallback));
        // SAFETY: taken once; `self` is forgotten right after, so the
        // ManuallyDrop is never read again.
        let lib = unsafe { ManuallyDrop::take(&mut self.lib) };
        core::mem::forget(self);
        drop(lib);
    }
}

impl Drop for Installed {
    fn drop(&mut self) {
        // Dropped without `uninstall`: an occupant may still be installed,
        // so the image stays mapped. Leak, never dangle.
    }
}
