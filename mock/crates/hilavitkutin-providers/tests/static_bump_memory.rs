//! `StaticBumpMemory` keeps the promises `MemoryProviderApi` asks of it.
//!
//! A block is aligned by its address, inside the buffer, and apart from every
//! other block, also when several threads ask at once; it stays where it was
//! when the provider moves; a request that does not fit, or an alignment that
//! is not a power of two, is refused with null and takes nothing. A provider
//! built with `from_raw_parts` over a block it does not own is dropped before
//! the block is freed, the order its caller promises.
//!
//! Where a property is judged by a check, the check is shown seeing the break
//! it is for: the alignment check on the address an offset-aligned bump would
//! give over the same misaligned buffer, both in the single test and in the
//! threaded one's checking function, and the overlap check on two ranges that
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

/// The first block that breaks a promise of `MemoryProviderApi` over a buffer
/// at `lo .. hi`: one not aligned by its address, one reaching outside the
/// buffer, or two sharing a byte. Each block is `(address, length, alignment)`.
fn first_broken(blocks: &mut [(usize, usize, usize)], lo: usize, hi: usize) -> Option<String> {
    for &(addr, len, align) in blocks.iter() {
        if addr % align != 0 {
            return Some(format!("block at {addr:#x} is not aligned to {align}"));
        }
        if addr < lo || addr + len > hi {
            return Some(format!(
                "block at {addr:#x} of {len} bytes is outside the buffer"
            ));
        }
    }
    blocks.sort_unstable();
    blocks
        .windows(2)
        .find(|pair| pair[0].0 + pair[0].1 > pair[1].0)
        .map(|pair| format!("{pair:?} overlap"))
}

/// The requests every caller of the threaded test makes, `EACH` of them for
/// thread `t`.
fn requests(t: usize, each: usize) -> impl Iterator<Item = (usize, usize)> {
    (0 .. each).map(move |k| (1 + (t + k) % 13, 1 << (k % 4)))
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
                    requests(t, EACH)
                        .map(|(len, align)| {
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
    assert_eq!(first_broken(&mut all, lo, hi), None);
}

#[test]
fn the_block_check_rejects_a_bump_that_aligns_by_offset() {
    // The same requests over the same skewed buffer, handed out by a bump that
    // aligns its offset into the buffer rather than the address, the way the
    // provider did before it was fixed. The check must see it.
    let buffer = skewed(8 * 200 * 32, 3);
    let (lo, hi) = (
        buffer.as_ptr().addr(),
        buffer.as_ptr().addr() + buffer.len(),
    );
    let mut offset: usize = 0;
    let mut blocks: Vec<(usize, usize, usize)> = (0 .. 8)
        .flat_map(|t| requests(t, 200))
        .map(|(len, align)| {
            offset = offset.next_multiple_of(align);
            let block = (lo + offset, len, align);
            offset += len;
            block
        })
        .collect();
    let broken = first_broken(&mut blocks, lo, hi);
    assert!(
        broken.as_deref().is_some_and(|b| b.contains("not aligned")),
        "the check reports the misaligned block; reported {broken:?}"
    );
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

#[test]
fn a_provider_over_raw_parts_is_dropped_before_its_block_is_freed() {
    // The order a provider over a block it does not own keeps: build it over
    // the block, use it, drop it, and only then free the block. Run under Miri
    // as well, which sees a use of the block after the free.
    const BYTES: usize = 256;
    let block = Box::into_raw(Box::new([MaybeUninit::<u8>::uninit(); BYTES]));
    let lo = block.addr();
    // SAFETY: the box's `BYTES` bytes are valid for reads and writes, nothing
    // but the provider touches them, and the provider is dropped before the
    // box is freed below.
    let m = core::mem::ManuallyDrop::new(unsafe {
        StaticBumpMemory::from_raw_parts(block.cast(), USize(BYTES))
    });
    let p = allocate(&m, 8, 8);
    assert!(!p.is_null());
    assert!(
        p.addr() >= lo && p.addr() + 8 <= lo + BYTES,
        "the block is inside the box"
    );
    // SAFETY: `p` is a live block of 8 bytes aligned to 8.
    unsafe { p.cast::<u64>().write(0xC0FFEE) };
    // SAFETY: as above.
    assert_eq!(unsafe { p.cast::<u64>().read() }, 0xC0FFEE);
    drop(core::mem::ManuallyDrop::into_inner(m));
    // SAFETY: `block` came from `Box::into_raw` and the provider over it is
    // gone.
    drop(unsafe { Box::from_raw(block) });
}
