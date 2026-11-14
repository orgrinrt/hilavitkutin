//! `StaticBumpMemory` keeps the promises `MemoryProviderApi` asks of it.
//!
//! A block is aligned by its address, inside the buffer, and apart from every
//! other block, also when several threads ask at once; it stays where it was
//! when the provider moves; a request that does not fit, or an alignment that
//! is not a power of two, is refused with null and takes nothing.
//!
//! Where a property is judged by a check, the check is shown seeing the break
//! it is for: the alignment check on the address an offset-aligned bump would
//! give over the same misaligned buffer, the overlap check on two ranges that
//! share bytes.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_providers::StaticBumpMemory;

/// A leaked buffer of `bytes`, starting `skew` bytes past a 64-byte boundary.
fn skewed(bytes: usize, skew: usize) -> &'static mut [MaybeUninit<u8>] {
    let whole = std::vec![MaybeUninit::<u8>::uninit(); bytes + 128].leak();
    let to_boundary = whole.as_ptr().addr().wrapping_neg() & 63;
    &mut whole[to_boundary + skew .. to_boundary + skew + bytes]
}

fn allocate(m: &StaticBumpMemory, len: usize, align: usize) -> *mut u8 {
    // SAFETY: every call passes a length and an alignment; null is checked by
    // the caller, which is the whole of the caller's half for `allocate`.
    unsafe { m.allocate(USize(len), USize(align)) }
}

/// Whether two blocks share a byte.
fn overlap(a: (*mut u8, usize), b: (*mut u8, usize)) -> bool {
    let (a0, b0) = (a.0.addr(), b.0.addr());
    a0 < b0 + b.1 && b0 < a0 + a.1
}

#[test]
fn a_block_is_aligned_by_its_address_over_a_misaligned_buffer() {
    let m = StaticBumpMemory::new(skewed(4096, 3));
    let a = allocate(&m, 5, 1);
    let b = allocate(&m, 64, 64);
    assert!(!a.is_null() && !b.is_null());
    assert_eq!(b.addr() % 64, 0, "the address is aligned, not the offset");
    // An offset-aligned bump over the same buffer starts its 64-aligned block
    // at offset 64, three bytes past a boundary: the check above must see it.
    let base = skewed(4096, 3).as_ptr().addr();
    assert_ne!((base + 64) % 64, 0, "the misaligned control is misaligned");
}

#[test]
fn blocks_are_disjoint_and_inside_the_buffer() {
    let buffer = skewed(1024, 0);
    let (lo, hi) = (
        buffer.as_ptr().addr(),
        buffer.as_ptr().addr() + buffer.len(),
    );
    let m = StaticBumpMemory::new(buffer);
    let blocks: Vec<(*mut u8, usize)> = [(7, 1), (16, 8), (1, 1), (100, 64), (3, 2)]
        .into_iter()
        .map(|(len, align)| (allocate(&m, len, align), len))
        .collect();
    for (i, &a) in blocks.iter().enumerate() {
        assert!(!a.0.is_null());
        assert!(
            a.0.addr() >= lo && a.0.addr() + a.1 <= hi,
            "block {i} inside"
        );
        for &b in &blocks[i + 1 ..] {
            assert!(!overlap(a, b), "blocks {a:?} and {b:?} share a byte");
        }
    }
    let base = blocks[0].0;
    assert!(
        overlap((base, 8), (base.wrapping_add(4), 8)),
        "the overlap check sees two ranges that share bytes"
    );
}

#[test]
fn a_request_that_does_not_fit_is_refused_and_takes_nothing() {
    let m = StaticBumpMemory::new(skewed(256, 0));
    assert!(!allocate(&m, 200, 1).is_null());
    let used = m.used();
    assert!(allocate(&m, 100, 1).is_null(), "past the end is refused");
    assert!(
        allocate(&m, usize::MAX, 1).is_null(),
        "an overflowing length is refused"
    );
    assert!(
        allocate(&m, 8, 3).is_null(),
        "a non-power-of-two alignment is refused"
    );
    assert!(allocate(&m, 8, 0).is_null(), "a zero alignment is refused");
    assert_eq!(m.used(), used, "a refusal moves no cursor");
    assert!(
        !allocate(&m, 56, 1).is_null(),
        "what is left is still handed out"
    );
}

#[test]
fn concurrent_callers_get_aligned_disjoint_blocks_inside_the_buffer() {
    const THREADS: usize = 8;
    const EACH: usize = 200;
    // Three bytes past a boundary, so a block aligned by its offset in the
    // buffer is misaligned here and only one aligned by its address passes.
    let buffer = skewed(THREADS * EACH * 32, 3);
    let (lo, hi) = (
        buffer.as_ptr().addr(),
        buffer.as_ptr().addr() + buffer.len(),
    );
    let m = StaticBumpMemory::new(buffer);
    let mut all: Vec<(usize, usize, usize)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0 .. THREADS)
            .map(|t| {
                let m = &m;
                s.spawn(move || {
                    (0 .. EACH)
                        .map(|k| {
                            let len = 1 + (t + k) % 13;
                            let align = 1 << (k % 4);
                            let p = allocate(m, len, align);
                            assert!(!p.is_null());
                            (p.addr(), len, align)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    });
    for &(addr, len, align) in &all {
        assert_eq!(addr % align, 0, "block at {addr:#x} is aligned to {align}");
        assert!(
            addr >= lo && addr + len <= hi,
            "block at {addr:#x} of {len} bytes is inside the buffer"
        );
    }
    all.sort_unstable();
    for pair in all.windows(2) {
        assert!(pair[0].0 + pair[0].1 <= pair[1].0, "{pair:?} overlap");
    }
}

#[test]
fn a_block_stays_where_it_was_when_the_provider_moves() {
    let m = StaticBumpMemory::new(skewed(256, 0));
    let p = allocate(&m, 4, 4);
    // SAFETY: `p` is a live block of 4 bytes aligned to 4.
    unsafe { p.cast::<u32>().write(0xC0FFEE) };
    let moved = Box::new(m);
    // SAFETY: the block lives in the 'static buffer, which the move of the
    // provider into a box does not touch.
    assert_eq!(unsafe { p.cast::<u32>().read() }, 0xC0FFEE);
    let q = allocate(&moved, 4, 4);
    assert!(
        !overlap((p, 4), (q, 4)),
        "the moved provider keeps its cursor"
    );
}
