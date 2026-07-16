//! Shared storage-layout kernels for the six resource-storage variants.
//!
//! Round 202606210600. Each variant cdylib is a thin `#[bench_variant]` wrapper
//! that builds its layout from the harness byte input in the untimed `setup`
//! block, then runs the morsel loop in the timed `run` block. This crate holds
//! the layout builds, the per-record `combine`, the shared transform stages, and
//! the one shared bump `Arena`. The ONLY thing that differs across variants is
//! how the M scalar resource members reach `combine`; everything else is
//! identical here so a measured difference is purely the member-fetch shape.
//!
//! The harness fills the `&[u8; N]` input with seeded bytes; N is the per-call
//! byte payload (the column-record data, N/4 = R records of u32). The resource
//! (M scalar `Field` members + a `Seq` run + a `Map` run) is built from the
//! input head. The output is an 8-byte FNV-1a checksum of the produced column,
//! which the harness validates byte-exact across all six variants (the
//! cross-variant equality requirement, handled by the framework with
//! `MAY_DIFFER=false`).
//!
//! The load-bearing fairness property: every variant derives its resource
//! pointer(s) AND its data columns from one shared bump `Arena`, mirroring the
//! shipped `ArenaColumnStorage` (every column bumped from one backing block). So
//! a write to the output column and a read of a resource member share the
//! arena's provenance, the aliasing condition the spec's noalias win defeats.
//! Distinct buffers would hand LLVM trivial non-aliasing and refute the win
//! falsely.

use std::cell::Cell;
use std::mem::MaybeUninit;

// ----- resource shape constants ----------------------------------------------

/// Scalar `Field` member count. Const so the member fold unrolls and the
/// snapshot variants can hoist member reads out of the record loop (a real
/// `Resource<T>` has a const-derived `Decompose::Leaves` layout, so const M is
/// the faithful shape; a runtime length would itself prevent the residency the
/// win depends on).
pub const M: usize = 4;
/// `Seq` member length (consecutive u32 run). Small, as resources are small.
pub const SEQ_LEN: usize = 6;
/// `Map` member length (consecutive u32 run, the value side).
pub const MAP_LEN: usize = 4;

// ----- transform stages (shared so all six compute byte-identically) ---------

#[inline(always)]
pub fn stage1(i: u32) -> u32 {
    i.wrapping_mul(2654435761)
}
#[inline(always)]
pub fn stage2(a: u32) -> u32 {
    a.wrapping_mul(2246822519).wrapping_add(1)
}
#[inline(always)]
pub fn stage3(b: u32) -> u32 {
    (b >> 13) ^ b
}
#[inline(always)]
pub fn stage4(c: u32) -> u32 {
    c.wrapping_mul(3266489917)
}

#[inline(always)]
fn heavy_chain(seed: u32) -> u32 {
    let mut x = seed;
    let mut r = 0;
    while r < 12 {
        x = stage4(stage3(stage2(stage1(x))));
        r += 1;
    }
    x
}

/// The per-record body, shared. `HEAVY` selects op intensity: LIGHT (false) is
/// dispatch/memory dominated so the member-fetch difference is most visible;
/// HEAVY (true) makes real compute dominate so a live reload overlaps and hides.
#[inline(always)]
pub fn combine<const HEAVY: bool>(inp: u32, members: &[u32; M], seq_sum: u32, map_sum: u32) -> u32 {
    let mut x = stage1(inp);
    let mut k = 0;
    while k < M {
        x = x.wrapping_add(stage2(members[k]));
        k += 1;
    }
    x = stage3(x.wrapping_add(seq_sum));
    let mut out = stage4(x.wrapping_add(map_sum));
    if HEAVY {
        out = heavy_chain(out);
    }
    out
}

// ----- FNV-1a checksum over the produced column (the 8-byte output) ----------

#[inline(always)]
pub fn fnv1a_column(col: *const u32, n: usize) -> [u8; 8] {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let mut i = 0;
    while i < n {
        // SAFETY: col holds n u32 written by the kernel.
        let v = unsafe { *col.add(i) };
        for b in v.to_le_bytes() {
            h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3);
        }
        i += 1;
    }
    h.to_le_bytes()
}

// ----- one bump arena (the single provenance domain) -------------------------

pub const CACHE_LINE: usize = 64;

pub struct Arena {
    base: *mut u8,
    cap: usize,
    used: Cell<usize>,
    _buf: Box<[MaybeUninit<u8>]>,
}

impl Arena {
    pub fn new(bytes: usize) -> Self {
        let mut buf: Box<[MaybeUninit<u8>]> =
            vec![MaybeUninit::uninit(); bytes].into_boxed_slice();
        let base = buf.as_mut_ptr() as *mut u8;
        Arena { base, cap: bytes, used: Cell::new(0), _buf: buf }
    }
    pub fn alloc(&self, len: usize, align: usize) -> *mut u8 {
        let used = self.used.get();
        let aligned = (used + align - 1) / align * align;
        assert!(aligned + len <= self.cap, "arena exhausted");
        self.used.set(aligned + len);
        // SAFETY: aligned + len <= cap; base starts a cap-byte block.
        unsafe { self.base.add(aligned) }
    }
    pub fn alloc_col<T>(&self, count: usize) -> *mut T {
        self.alloc((count * std::mem::size_of::<T>()).max(1), CACHE_LINE) as *mut T
    }
    pub fn alloc_bytes(&self, bytes: usize) -> *mut u8 {
        self.alloc(bytes.max(1), CACHE_LINE)
    }
}

/// Byte budget for `cols` u32 columns over `n` records plus alignment slack.
pub fn arena_bytes(cols: usize, n: usize) -> usize {
    cols * n * 4 + cols * CACHE_LINE + (1 << 16)
}

/// Records derived from a byte payload of `bytes` bytes (u32 records).
#[inline(always)]
pub fn records_for(bytes: usize) -> usize {
    (bytes / 4).max(1)
}

/// Fill `In[i]` from the seeded input bytes so the column data is real (not a
/// constant the optimizer folds away), and fill the M members / Seq / Map from
/// the input head. Shared by every variant's setup.
#[inline(always)]
pub fn member_fill(c: usize, input: &[u8]) -> u32 {
    // derive a per-member value from the input head so members are runtime data
    let base = (c * 4) % input.len().max(1);
    let mut v = [0u8; 4];
    let mut k = 0;
    while k < 4 {
        v[k] = input[(base + k) % input.len().max(1)];
        k += 1;
    }
    u32::from_le_bytes(v).wrapping_add(c as u32 * 7 + 1)
}

// ----- the six variant kernels -----------------------------------------------
//
// Each `vN::<HEAVY>(input, output)` builds its layout from `input` over one
// shared `Arena`, runs the morsel loop, and writes the 8-byte checksum to
// `output`. The build is meant for the untimed `setup` block and the loop for
// the timed `run` block; each variant cdylib splits them across the `timed!`
// markers. Returning the column pointer + record count from the build lets the
// cdylib place the build in setup and the loop in run.

pub mod v0 {
    //! V0: one-record opaque blob, read LIVE every iteration (shipped status quo).
    use super::*;
    pub struct State {
        pub blob: *const u32, // [M | SEQ_LEN | MAP_LEN]
        pub inp: *const u32,
        pub out: *mut u32,
        pub n: usize,
        pub _arena: Arena,
    }
    pub fn build(input: &[u8]) -> State {
        let n = records_for(input.len());
        let arena = Arena::new(arena_bytes(3, n));
        let blob = arena.alloc_col::<u32>(M + SEQ_LEN + MAP_LEN);
        let inp = arena.alloc_col::<u32>(n);
        let out = arena.alloc_col::<u32>(n);
        // SAFETY: freshly reserved; sizes match.
        unsafe {
            for k in 0..M { *blob.add(k) = member_fill(k, input); }
            for k in 0..SEQ_LEN { *blob.add(M + k) = member_fill(M + k, input); }
            for k in 0..MAP_LEN { *blob.add(M + SEQ_LEN + k) = member_fill(M + SEQ_LEN + k, input); }
            for i in 0..n {
                let b = (i * 4) % input.len();
                *inp.add(i) = u32::from_le_bytes([input[b], input[(b+1)%input.len()], input[(b+2)%input.len()], input[(b+3)%input.len()]]);
            }
        }
        State { blob: blob as *const u32, inp: inp as *const u32, out, n, _arena: arena }
    }
    #[inline(always)]
    pub fn run<const HEAVY: bool>(s: &State) {
        let mut i = 0;
        while i < s.n {
            // LIVE: re-read all members through the blob pointer each iteration.
            let mut members = [0u32; M];
            let mut seq_sum = 0u32;
            let mut map_sum = 0u32;
            // SAFETY: blob holds M+SEQ+MAP; In/Out reserved for n.
            unsafe {
                let mut c = 0;
                while c < M { members[c] = *s.blob.add(c); c += 1; }
                for k in 0..SEQ_LEN { seq_sum = seq_sum.wrapping_add(*s.blob.add(M + k)); }
                for k in 0..MAP_LEN { map_sum = map_sum.wrapping_add(*s.blob.add(M + SEQ_LEN + k)); }
                let inv = *s.inp.add(i);
                *s.out.add(i) = combine::<HEAVY>(inv, &members, seq_sum, map_sum);
            }
            i += 1;
        }
    }
}

pub mod v1 {
    //! V1: same blob, SNAPSHOT scalar members to a stack local before the loop.
    use super::*;
    pub use super::v0::{build, State};
    #[inline(always)]
    pub fn run<const HEAVY: bool>(s: &State) {
        // Snapshot members + collection sums ONCE before the record loop.
        let mut members = [0u32; M];
        let mut seq_sum = 0u32;
        let mut map_sum = 0u32;
        // SAFETY: blob holds M+SEQ+MAP.
        unsafe {
            let mut c = 0;
            while c < M { members[c] = *s.blob.add(c); c += 1; }
            for k in 0..SEQ_LEN { seq_sum = seq_sum.wrapping_add(*s.blob.add(M + k)); }
            for k in 0..MAP_LEN { map_sum = map_sum.wrapping_add(*s.blob.add(M + SEQ_LEN + k)); }
        }
        let mut i = 0;
        while i < s.n {
            // SAFETY: i < n; In/Out reserved.
            unsafe {
                let inv = *s.inp.add(i);
                *s.out.add(i) = combine::<HEAVY>(inv, &members, seq_sum, map_sum);
            }
            i += 1;
        }
    }
}

pub mod v2 {
    //! V2: type-unique decomposed columns, one scattered per member; snapshot.
    use super::*;
    pub struct State {
        pub leaves: [*const u32; M],
        pub seq: *const u32,
        pub map: *const u32,
        pub inp: *const u32,
        pub out: *mut u32,
        pub n: usize,
        pub _arena: Arena,
    }
    pub fn build(input: &[u8]) -> State {
        let n = records_for(input.len());
        let arena = Arena::new(arena_bytes(3 + M, n));
        let mut leaves = [core::ptr::null::<u32>(); M];
        for k in 0..M {
            let p = arena.alloc_col::<u32>(1);
            unsafe { *p = member_fill(k, input) };
            leaves[k] = p as *const u32;
            let _scatter = arena.alloc_col::<u32>(8); // scatter next leaf
        }
        let seq = arena.alloc_col::<u32>(SEQ_LEN);
        let map = arena.alloc_col::<u32>(MAP_LEN);
        let inp = arena.alloc_col::<u32>(n);
        let out = arena.alloc_col::<u32>(n);
        unsafe {
            for k in 0..SEQ_LEN { *seq.add(k) = member_fill(M + k, input); }
            for k in 0..MAP_LEN { *map.add(k) = member_fill(M + SEQ_LEN + k, input); }
            for i in 0..n {
                let b = (i * 4) % input.len();
                *inp.add(i) = u32::from_le_bytes([input[b], input[(b+1)%input.len()], input[(b+2)%input.len()], input[(b+3)%input.len()]]);
            }
        }
        State { leaves, seq: seq as *const u32, map: map as *const u32, inp: inp as *const u32, out, n, _arena: arena }
    }
    #[inline(always)]
    pub fn run<const HEAVY: bool>(s: &State) {
        let mut members = [0u32; M];
        let mut c = 0;
        while c < M { unsafe { members[c] = *s.leaves[c] }; c += 1; }
        let mut seq_sum = 0u32;
        for k in 0..SEQ_LEN { unsafe { seq_sum = seq_sum.wrapping_add(*s.seq.add(k)) }; }
        let mut map_sum = 0u32;
        for k in 0..MAP_LEN { unsafe { map_sum = map_sum.wrapping_add(*s.map.add(k)) }; }
        let mut i = 0;
        while i < s.n {
            unsafe {
                let inv = *s.inp.add(i);
                *s.out.add(i) = combine::<HEAVY>(inv, &members, seq_sum, map_sum);
            }
            i += 1;
        }
    }
}

pub mod v3 {
    //! V3: shape-bound shared column, members by resource-slot stride; snapshot.
    use super::*;
    pub struct State {
        pub shared: *const u32, // slot*stride + member; slot 0 under test
        pub stride: usize,
        pub seq: *const u32,
        pub map: *const u32,
        pub inp: *const u32,
        pub out: *mut u32,
        pub n: usize,
        pub _arena: Arena,
    }
    pub fn build(input: &[u8]) -> State {
        let n = records_for(input.len());
        let arena = Arena::new(arena_bytes(3, n));
        let stride = M;
        let slots = 2; // slot 0 under test, slot 1 a sibling sharing the column
        let shared = arena.alloc_col::<u32>(stride * slots);
        let seq = arena.alloc_col::<u32>(SEQ_LEN);
        let map = arena.alloc_col::<u32>(MAP_LEN);
        let inp = arena.alloc_col::<u32>(n);
        let out = arena.alloc_col::<u32>(n);
        unsafe {
            for k in 0..M { *shared.add(k) = member_fill(k, input); } // slot 0 = baseline fill
            for k in 0..M { *shared.add(stride + k) = member_fill(k, input).wrapping_add(99); } // sibling
            for k in 0..SEQ_LEN { *seq.add(k) = member_fill(M + k, input); }
            for k in 0..MAP_LEN { *map.add(k) = member_fill(M + SEQ_LEN + k, input); }
            for i in 0..n {
                let b = (i * 4) % input.len();
                *inp.add(i) = u32::from_le_bytes([input[b], input[(b+1)%input.len()], input[(b+2)%input.len()], input[(b+3)%input.len()]]);
            }
        }
        State { shared: shared as *const u32, stride, seq: seq as *const u32, map: map as *const u32, inp: inp as *const u32, out, n, _arena: arena }
    }
    #[inline(always)]
    pub fn run<const HEAVY: bool>(s: &State) {
        let mut members = [0u32; M];
        let mut c = 0;
        while c < M { unsafe { members[c] = *s.shared.add(c) }; c += 1; } // slot 0
        let mut seq_sum = 0u32;
        for k in 0..SEQ_LEN { unsafe { seq_sum = seq_sum.wrapping_add(*s.seq.add(k)) }; }
        let mut map_sum = 0u32;
        for k in 0..MAP_LEN { unsafe { map_sum = map_sum.wrapping_add(*s.map.add(k)) }; }
        let mut i = 0;
        while i < s.n {
            unsafe {
                let inv = *s.inp.add(i);
                *s.out.add(i) = combine::<HEAVY>(inv, &members, seq_sum, map_sum);
            }
            i += 1;
        }
    }
}

pub mod v4 {
    //! V4: loimu-style type-erasure-via-shaping; backcast on access; snapshot.
    use super::*;
    pub struct State {
        pub bytes: *const u8,
        pub scalar_off: usize,
        pub seq_off: usize,
        pub map_off: usize,
        pub inp: *const u32,
        pub out: *mut u32,
        pub n: usize,
        pub _arena: Arena,
    }
    pub fn build(input: &[u8]) -> State {
        let n = records_for(input.len());
        let arena = Arena::new(arena_bytes(3, n));
        let total = (M + SEQ_LEN + MAP_LEN) * 4;
        let bytes = arena.alloc_bytes(total);
        let inp = arena.alloc_col::<u32>(n);
        let out = arena.alloc_col::<u32>(n);
        unsafe {
            let as_u32 = bytes as *mut u32;
            for k in 0..M { *as_u32.add(k) = member_fill(k, input); }
            for k in 0..SEQ_LEN { *as_u32.add(M + k) = member_fill(M + k, input); }
            for k in 0..MAP_LEN { *as_u32.add(M + SEQ_LEN + k) = member_fill(M + SEQ_LEN + k, input); }
            for i in 0..n {
                let b = (i * 4) % input.len();
                *inp.add(i) = u32::from_le_bytes([input[b], input[(b+1)%input.len()], input[(b+2)%input.len()], input[(b+3)%input.len()]]);
            }
        }
        State { bytes: bytes as *const u8, scalar_off: 0, seq_off: M * 4, map_off: (M + SEQ_LEN) * 4, inp: inp as *const u32, out, n, _arena: arena }
    }
    #[inline(always)]
    pub fn run<const HEAVY: bool>(s: &State) {
        let mut members = [0u32; M];
        let mut c = 0;
        while c < M { unsafe { members[c] = *(s.bytes.add(s.scalar_off + c * 4) as *const u32) }; c += 1; }
        let mut seq_sum = 0u32;
        for k in 0..SEQ_LEN { unsafe { seq_sum = seq_sum.wrapping_add(*(s.bytes.add(s.seq_off + k * 4) as *const u32)) }; }
        let mut map_sum = 0u32;
        for k in 0..MAP_LEN { unsafe { map_sum = map_sum.wrapping_add(*(s.bytes.add(s.map_off + k * 4) as *const u32)) }; }
        let mut i = 0;
        while i < s.n {
            unsafe {
                let inv = *s.inp.add(i);
                *s.out.add(i) = combine::<HEAVY>(inv, &members, seq_sum, map_sum);
            }
            i += 1;
        }
    }
}

pub mod v5 {
    //! V5: runtime handle-table; per-leaf store-ids resolved at runtime; snapshot.
    use super::*;
    pub struct State {
        pub slot_table: Vec<*const u32>,
        pub ids: [usize; M],
        pub seq_id: usize,
        pub map_id: usize,
        pub inp: *const u32,
        pub out: *mut u32,
        pub n: usize,
        pub _arena: Arena,
    }
    pub fn build(input: &[u8]) -> State {
        let n = records_for(input.len());
        let arena = Arena::new(arena_bytes(3 + M, n));
        let mut slot_table: Vec<*const u32> = Vec::new();
        let mut ids = [0usize; M];
        for k in 0..M {
            let p = arena.alloc_col::<u32>(1);
            unsafe { *p = member_fill(k, input) };
            ids[k] = slot_table.len();
            slot_table.push(p as *const u32);
            let _scatter = arena.alloc_col::<u32>(8);
        }
        let seq = arena.alloc_col::<u32>(SEQ_LEN);
        let map = arena.alloc_col::<u32>(MAP_LEN);
        let seq_id = slot_table.len(); slot_table.push(seq as *const u32);
        let map_id = slot_table.len(); slot_table.push(map as *const u32);
        let inp = arena.alloc_col::<u32>(n);
        let out = arena.alloc_col::<u32>(n);
        unsafe {
            for k in 0..SEQ_LEN { *seq.add(k) = member_fill(M + k, input); }
            for k in 0..MAP_LEN { *map.add(k) = member_fill(M + SEQ_LEN + k, input); }
            for i in 0..n {
                let b = (i * 4) % input.len();
                *inp.add(i) = u32::from_le_bytes([input[b], input[(b+1)%input.len()], input[(b+2)%input.len()], input[(b+3)%input.len()]]);
            }
        }
        State { slot_table, ids, seq_id, map_id, inp: inp as *const u32, out, n, _arena: arena }
    }
    #[inline(always)]
    pub fn run<const HEAVY: bool>(s: &State) {
        let mut members = [0u32; M];
        let mut c = 0;
        while c < M { let id = s.ids[c]; let p = s.slot_table[id]; unsafe { members[c] = *p }; c += 1; }
        let sp = s.slot_table[s.seq_id];
        let mut seq_sum = 0u32;
        for k in 0..SEQ_LEN { unsafe { seq_sum = seq_sum.wrapping_add(*sp.add(k)) }; }
        let mp = s.slot_table[s.map_id];
        let mut map_sum = 0u32;
        for k in 0..MAP_LEN { unsafe { map_sum = map_sum.wrapping_add(*mp.add(k)) }; }
        let mut i = 0;
        while i < s.n {
            unsafe {
                let inv = *s.inp.add(i);
                *s.out.add(i) = combine::<HEAVY>(inv, &members, seq_sum, map_sum);
            }
            i += 1;
        }
    }
}

// ----- axis D: Seq collection-member payload (live-stream vs snapshot-copy) ---
//
// The in-model large resource payload is a Seq/Map collection member. These two
// kernels fold a Seq of L u32 `passes` times: `seq_live` reads it in place from
// its column each pass (the V0 live shape), `seq_snapshot` copies it to a local
// buffer once then folds the copy (the V1 snapshot shape). N bytes = the Seq.
// Output is the 8-byte accumulator. As L exceeds cache the snapshot copy is pure
// overhead and live wins, the "residency is fine until megabytes" regime.

pub mod seqd {
    use super::*;

    #[inline(always)]
    fn fold(seq: &[u32]) -> u32 {
        let mut acc = 0u32;
        let mut i = 0;
        while i < seq.len() {
            acc = acc.wrapping_add(seq[i].wrapping_mul(2654435761).rotate_left(7));
            i += 1;
        }
        acc
    }

    pub struct State {
        pub seq: *const u32,
        pub len: usize,
        pub passes: usize,
        pub out: *mut u32,
        pub _arena: Arena,
    }

    /// Build a Seq of `len` u32 elements from a `seed`, HEAP-allocated in the
    /// arena. The seed (not a large byte payload) is what crosses the harness FFI
    /// boundary, so the payload size is decoupled from the input size and there
    /// is no stack-array ceiling (the `ByteRoutine` `[0u8; N]` overflows the
    /// stack past a few MiB; this Routine-form input is a tiny seed). `len` comes
    /// from the variant's const generic.
    pub fn build(seed: u64, len: usize) -> State {
        // passes scale down with length so bytes-touched per call is comparable
        // across the size sweep.
        let passes = (4_194_304 / len.max(1)).clamp(1, 64);
        let arena = Arena::new(arena_bytes(1, len) + 4096);
        let seq = arena.alloc_col::<u32>(len);
        let out = arena.alloc_col::<u32>(1);
        let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        unsafe {
            for i in 0..len {
                x ^= x >> 30;
                x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
                *seq.add(i) = (x >> 32) as u32 | 1;
            }
            *out = 0;
        }
        State { seq: seq as *const u32, len, passes, out, _arena: arena }
    }

    /// LIVE: fold the Seq in place from its column each pass (no resident copy).
    #[inline(always)]
    pub fn run_live(s: &State) {
        let mut p = 0;
        while p < s.passes {
            // SAFETY: seq holds len u32; reconstructed each pass.
            let slice = unsafe { std::slice::from_raw_parts(s.seq, s.len) };
            let r = fold(slice);
            unsafe { *s.out = (*s.out).wrapping_add(r) };
            p += 1;
        }
    }

    /// SNAPSHOT: copy the Seq to a local buffer once, then fold the copy.
    #[inline(always)]
    pub fn run_snapshot(s: &State) {
        let mut local: Vec<u32> = Vec::with_capacity(s.len);
        // SAFETY: seq holds len u32.
        unsafe {
            local.set_len(s.len);
            std::ptr::copy_nonoverlapping(s.seq, local.as_mut_ptr(), s.len);
        }
        let mut p = 0;
        while p < s.passes {
            let r = fold(&local);
            unsafe { *s.out = (*s.out).wrapping_add(r) };
            p += 1;
        }
    }

    #[inline(always)]
    pub fn checksum(s: &State) -> [u8; 8] {
        fnv1a_column(s.out as *const u32, 1)
    }

    /// Routine for the seed-driven seqd bench. `Input` is a `u64` seed (tiny,
    /// crosses the FFI boundary without a stack array), `Output` an 8-byte
    /// checksum. `N` is the Seq element count (the payload size axis). The seqd
    /// variants use the `#[bench_variant]` Routine form against this, building
    /// the N-element heap payload from the seed in untimed setup, so the payload
    /// size is not bounded by the `ByteRoutine` `[0u8; N]` stack array.
    pub struct SeqAlgo<const N: usize>;
    impl<const N: usize> mockspace_bench_core::Routine for SeqAlgo<N> {
        type Input = u64;
        type Output = [u8; 8];
        fn build_input(seed: u64) -> u64 {
            // Pass the seed through unchanged; the variant derives the payload.
            seed.wrapping_add(1)
        }
        fn ops_per_call(_input: &u64) -> u64 {
            N as u64
        }
    }
}

// ----- axis B: intra-resource fetch at high arity (blob vs decomposed) --------
//
// At M=64 the locality difference between a contiguous blob and scattered
// per-member columns shows. The timed region is the member GATHER repeated
// `passes` times (not a morsel loop, which would drown the gather in column
// streaming). N bytes sizes the scatter span for decomposed and is otherwise
// the pass count driver. Output is the 8-byte checksum of the gathered sum.

pub mod bwide {
    use super::*;
    pub const MW: usize = 64;

    pub struct Blob { pub blob: *const u32, pub passes: usize, pub out: *mut u32, pub _a: Arena }
    pub struct Dec { pub leaves: Vec<*const u32>, pub passes: usize, pub out: *mut u32, pub _a: Arena }

    fn passes_for(input_len: usize) -> usize {
        (2_000_000 / (input_len / 4).max(1)).clamp(8, 2000)
    }

    pub fn build_blob(input: &[u8]) -> Blob {
        let a = Arena::new(arena_bytes(2, 64));
        let blob = a.alloc_col::<u32>(MW);
        unsafe { for k in 0..MW { *blob.add(k) = member_fill(k, input); } }
        let out = a.alloc_col::<u32>(1);
        unsafe { *out = 0 };
        Blob { blob: blob as *const u32, passes: passes_for(input.len()), out, _a: a }
    }
    pub fn build_dec(input: &[u8]) -> Dec {
        // scatter each leaf onto its own line, far apart
        let a = Arena::new(arena_bytes(2, 64) + MW * CACHE_LINE * 2);
        let mut leaves = Vec::with_capacity(MW);
        for k in 0..MW {
            let p = a.alloc_col::<u32>(1);
            unsafe { *p = member_fill(k, input) };
            leaves.push(p as *const u32);
            let _scatter = a.alloc_bytes(CACHE_LINE); // push next leaf onto a new line
        }
        let out = a.alloc_col::<u32>(1);
        unsafe { *out = 0 };
        Dec { leaves, passes: passes_for(input.len()), out, _a: a }
    }

    #[inline(always)]
    pub fn run_blob(s: &Blob) {
        let mut acc = 0u32;
        let mut p = 0;
        while p < s.passes {
            let mut c = 0;
            while c < MW { acc = acc.wrapping_add(unsafe { *s.blob.add(c) }.rotate_left(p as u32 & 31)); c += 1; }
            p += 1;
        }
        unsafe { *s.out = acc };
    }
    #[inline(always)]
    pub fn run_dec(s: &Dec) {
        let mut acc = 0u32;
        let mut p = 0;
        while p < s.passes {
            let mut c = 0;
            while c < MW { acc = acc.wrapping_add(unsafe { *s.leaves[c] }.rotate_left(p as u32 & 31)); c += 1; }
            p += 1;
        }
        unsafe { *s.out = acc };
    }
    #[inline(always)]
    pub fn checksum_blob(s: &Blob) -> [u8; 8] { fnv1a_column(s.out as *const u32, 1) }
    #[inline(always)]
    pub fn checksum_dec(s: &Dec) -> [u8; 8] { fnv1a_column(s.out as *const u32, 1) }
}
