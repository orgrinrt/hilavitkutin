//! Spike s2, seat 02: a compiled host unit over an extension surface.
//!
//! Checks four things and nothing else:
//!
//! 1. An extension's declared reads and writes, by stable name, can be admitted or
//!    refused at load against the envelope a compiled host unit declared at build.
//! 2. What that admission cannot see: an extension that writes through a pointer it
//!    was handed for reading. The C boundary carries no proof.
//! 3. A per-morsel handle holding the extension's current variant (an interpreted
//!    one, then a compiled one) can be swapped between morsels with no change in
//!    the result, the switch point of Kohn, Leis and Neumann (ICDE 2018, fig. 5).
//! 4. The shape of the code: one indirect call per morsel against one per record.
//!    Read off the emitted assembly (`asm/`), never timed here.
//!
//! The host's column table is a plain `Vec<Vec<i64>>` standing in for the
//! scheduler's store; nothing here is hilavitkutin's api.

use core::sync::atomic::{AtomicPtr, Ordering};

/// FNV-64 over the name, the same derivation as `hilavitkutin_extensions::ProviderId::from_name`.
pub const fn store_id(name: &str) -> u64 {
    let b = name.as_bytes();
    let mut h: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < b.len() {
        h ^= b[i] as u64;
        h = h.wrapping_mul(0x100000001b3);
        i += 1;
    }
    h
}

/// What crosses the boundary per morsel: base pointers already offset to the
/// morsel start, in the order the extension declared its stores, and the length.
#[repr(C)]
pub struct WireView {
    pub reads: *const *const i64,
    pub writes: *const *mut i64,
    pub len: usize,
}

pub type MorselFn = unsafe extern "C" fn(view: *const WireView);

/// The extension's declaration, as a descriptor provider would hand it over.
#[repr(C)]
pub struct UnitDecl {
    pub reads: *const u64,
    pub reads_len: u32,
    pub writes: *const u64,
    pub writes_len: u32,
    pub run: MorselFn,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    ReadOutsideEnvelope(u64),
    WriteOutsideEnvelope(u64),
    TooManyStores,
}

/// The envelope the compiled host unit declared at build: the stores it reads and
/// writes, by stable name. In hilavitkutin these would be the host unit's access
/// set, so the plan orders the host unit like any other.
pub struct HostEnvelope {
    pub reads: &'static [u64],
    pub writes: &'static [u64],
}

pub struct Columns {
    pub ids: Vec<u64>,
    pub data: Vec<Vec<i64>>,
}

impl Columns {
    fn slot(&self, id: u64) -> usize {
        self.ids.iter().position(|&x| x == id).expect("store registered")
    }
}

const MAX: usize = 8;

pub struct Admitted {
    reads: [u64; MAX],
    read_n: usize,
    writes: [u64; MAX],
    write_n: usize,
    current: AtomicPtr<()>,
}

/// Admission at load. A read must fall in the host's reads or writes, a write in
/// the host's writes.
pub fn admit(env: &HostEnvelope, decl: &UnitDecl) -> Result<Admitted, Refusal> {
    let (rn, wn) = (decl.reads_len as usize, decl.writes_len as usize);
    if rn > MAX || wn > MAX {
        return Err(Refusal::TooManyStores);
    }
    let reads = unsafe { core::slice::from_raw_parts(decl.reads, rn) };
    let writes = unsafe { core::slice::from_raw_parts(decl.writes, wn) };
    for &r in reads {
        if !env.reads.contains(&r) && !env.writes.contains(&r) {
            return Err(Refusal::ReadOutsideEnvelope(r));
        }
    }
    for &w in writes {
        if !env.writes.contains(&w) {
            return Err(Refusal::WriteOutsideEnvelope(w));
        }
    }
    let mut a = Admitted {
        reads: [0; MAX],
        read_n: rn,
        writes: [0; MAX],
        write_n: wn,
        current: AtomicPtr::new(decl.run as *mut ()),
    };
    a.reads[..rn].copy_from_slice(reads);
    a.writes[..wn].copy_from_slice(writes);
    Ok(a)
}

impl Admitted {
    /// Swap the variant every following morsel runs. The handle of fig. 5.
    pub fn swap(&self, f: MorselFn) {
        self.current.store(f as *mut (), Ordering::Release);
    }

    /// One pass over every record, `morsel` records at a time: one load of the
    /// handle and one indirect call per morsel.
    pub fn run_pass(&self, cols: &mut Columns, morsel: usize, swap_after: Option<(usize, MorselFn)>) {
        let n = cols.data[0].len();
        let mut start = 0;
        let mut m = 0;
        while start < n {
            if let Some((k, f)) = swap_after {
                if m == k {
                    self.swap(f);
                }
            }
            let len = morsel.min(n - start);
            let mut rp = [core::ptr::null::<i64>(); MAX];
            let mut wp = [core::ptr::null_mut::<i64>(); MAX];
            for i in 0..self.read_n {
                let s = cols.slot(self.reads[i]);
                rp[i] = unsafe { cols.data[s].as_ptr().add(start) };
            }
            for i in 0..self.write_n {
                let s = cols.slot(self.writes[i]);
                wp[i] = unsafe { cols.data[s].as_mut_ptr().add(start) };
            }
            let view = WireView { reads: rp.as_ptr(), writes: wp.as_ptr(), len };
            let f: MorselFn = unsafe { core::mem::transmute(self.current.load(Ordering::Acquire)) };
            unsafe { f(&view) };
            start += len;
            m += 1;
        }
    }
}

// ---- What an extension's code is, written here so one crate holds the spike. ----

/// The compiled variant: `out = a * 3 + b` over a morsel. Its loop is the
/// extension's own and vectorises inside the extension; see `asm/`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn s2_ext_compiled(view: *const WireView) {
    let v = unsafe { &*view };
    let a = unsafe { core::slice::from_raw_parts(*v.reads, v.len) };
    let b = unsafe { core::slice::from_raw_parts(*v.reads.add(1), v.len) };
    let out = unsafe { core::slice::from_raw_parts_mut(*v.writes, v.len) };
    for i in 0..v.len {
        out[i] = a[i].wrapping_mul(3).wrapping_add(b[i]);
    }
}

/// The interpreted variant: the same function as bytecode over a stack, the form
/// a content script has before anything compiles it.
#[derive(Clone, Copy)]
pub enum Op {
    LoadRead(u8),
    Const(i64),
    Mul,
    Add,
}

pub static PROGRAM: [Op; 5] = [Op::LoadRead(0), Op::Const(3), Op::Mul, Op::LoadRead(1), Op::Add];

pub unsafe extern "C" fn s2_ext_interpreted(view: *const WireView) {
    let v = unsafe { &*view };
    for i in 0..v.len {
        let mut st = [0i64; 4];
        let mut sp = 0;
        for op in PROGRAM.iter() {
            match *op {
                Op::LoadRead(k) => {
                    st[sp] = unsafe { *(*v.reads.add(k as usize)).add(i) };
                    sp += 1;
                }
                Op::Const(c) => {
                    st[sp] = c;
                    sp += 1;
                }
                Op::Mul => {
                    sp -= 1;
                    st[sp - 1] = st[sp - 1].wrapping_mul(st[sp]);
                }
                Op::Add => {
                    sp -= 1;
                    st[sp - 1] = st[sp - 1].wrapping_add(st[sp]);
                }
            }
        }
        unsafe { *(*v.writes).add(i) = st[0] };
    }
}

/// An extension that declares `a` as a read and writes it anyway, through the
/// pointer it was handed for reading.
pub unsafe extern "C" fn s2_ext_liar(view: *const WireView) {
    let v = unsafe { &*view };
    let a = unsafe { *v.reads } as *mut i64;
    let out = unsafe { *v.writes };
    for i in 0..v.len {
        unsafe {
            *out.add(i) = *a.add(i);
            *a.add(i) = 0;
        }
    }
}

// ---- The per-record shape, for the assembly comparison only. ----

pub type RecordFn = unsafe extern "C" fn(a: i64, b: i64) -> i64;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn s2_ext_one_record(a: i64, b: i64) -> i64 {
    a.wrapping_mul(3).wrapping_add(b)
}

/// The host calling the extension once per record through a pointer it cannot see
/// through, which is what a cdylib boundary is.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn s2_host_per_record(f: RecordFn, a: &[i64], b: &[i64], out: &mut [i64]) {
    let f = core::hint::black_box(f);
    for i in 0..out.len() {
        out[i] = unsafe { f(a[i], b[i]) };
    }
}

/// The host calling once per morsel through the same kind of opaque pointer.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn s2_host_per_morsel(f: MorselFn, a: &[i64], b: &[i64], out: &mut [i64], morsel: usize) {
    let f = core::hint::black_box(f);
    let mut s = 0;
    while s < out.len() {
        let len = morsel.min(out.len() - s);
        let rp = [unsafe { a.as_ptr().add(s) }, unsafe { b.as_ptr().add(s) }];
        let wp = [unsafe { out.as_mut_ptr().add(s) }];
        let view = WireView { reads: rp.as_ptr(), writes: wp.as_ptr(), len };
        unsafe { f(&view) };
        s += len;
    }
}
