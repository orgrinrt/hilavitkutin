//! `StaticFreeListMemory` keeps the promises `MemoryProviderApi` asks of it,
//! and takes blocks back.
//!
//! A block is aligned, inside the buffer and apart from every other live
//! block, also when several threads allocate and free at once. A block given
//! back is handed out again, and blocks given back next to each other merge
//! into one that takes their whole length, in whichever order they come back.
//! A request that does not fit, or an alignment that is not a power of two,
//! is refused with null and changes nothing. `used` counts the grain padding
//! and comes back to zero. `allocate_zeroed` writes the zeros a reused block
//! does not have.
//!
//! The laws of taking back are functions over any provider, so each is shown
//! failing on `StaticBumpMemory`, which takes nothing back, over the same
//! buffer; the overlap check is shown seeing two ranges that share bytes.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin_api::MemoryProviderApi;
use hilavitkutin_providers::{StaticBumpMemory, StaticFreeListMemory};

/// The grain: a free-list node, two machine words.
const GRAIN: usize = 2 * core::mem::size_of::<usize>();

/// A leaked buffer of `bytes`, starting on a 64-byte boundary.
fn aligned(bytes: usize) -> &'static mut [MaybeUninit<u8>] {
    aligned_to(bytes, 64)
}

/// A leaked buffer of `bytes`, starting on an `align`-byte boundary.
fn aligned_to(bytes: usize, align: usize) -> &'static mut [MaybeUninit<u8>] {
    let whole = std::vec![MaybeUninit::<u8>::uninit(); bytes + align].leak();
    let to_boundary = whole.as_ptr().addr().wrapping_neg() & (align - 1);
    &mut whole[to_boundary .. to_boundary + bytes]
}

fn allocate<M: MemoryProviderApi>(m: &M, len: usize, align: usize) -> *mut u8 {
    // SAFETY: null is checked by every caller.
    unsafe { m.allocate(USize(len), USize(align)) }
}

fn free<M: MemoryProviderApi>(m: &M, ptr: *mut u8, len: usize, align: usize) {
    // SAFETY: every caller gives back a block it was handed, with the length
    // and alignment it asked for.
    unsafe { m.deallocate(ptr, USize(len), USize(align)) }
}

/// Whether two ranges of `len_a` and `len_b` bytes share a byte.
fn overlap(a: usize, len_a: usize, b: usize, len_b: usize) -> bool {
    a < b + len_b && b < a + len_a
}

#[test]
fn the_overlap_check_sees_two_ranges_that_share_bytes() {
    assert!(overlap(0, 16, 15, 16));
    assert!(!overlap(0, 16, 16, 16));
}

/// Blocks of several lengths and alignments are aligned, inside the buffer,
/// and apart from each other.
#[test]
fn blocks_are_aligned_inside_and_apart() {
    let buffer = aligned(4096);
    let (lo, hi) = (
        buffer.as_ptr().addr(),
        buffer.as_ptr().addr() + buffer.len(),
    );
    let m = StaticFreeListMemory::new(buffer);
    let asks = [(1, 1), (24, 8), (100, 64), (7, 16), (256, 128), (0, 4)];
    let mut live: Vec<(usize, usize)> = Vec::new();
    for (len, align) in asks {
        let p = allocate(&m, len, align);
        assert!(!p.is_null(), "{len} bytes at {align} fit");
        let at = p.addr();
        assert_eq!(at % align, 0, "aligned to {align}");
        assert!(at >= lo && at + len <= hi, "inside the buffer");
        for &(b, blen) in &live {
            assert!(
                !overlap(at, len.max(1), b, blen.max(1)),
                "apart from every live block"
            );
        }
        live.push((at, len));
    }
}

/// Whether a block that takes the whole span, given back, is handed out again.
fn reuses_a_given_back_block<M: MemoryProviderApi>(m: &M, span: usize) -> bool {
    let p = allocate(m, span, 16);
    assert!(!p.is_null(), "the first block takes the whole span");
    free(m, p, span, 16);
    !allocate(m, span, 16).is_null()
}

#[test]
fn a_block_given_back_is_handed_out_again() {
    assert!(reuses_a_given_back_block(
        &StaticFreeListMemory::new(aligned(512)),
        512
    ));
    assert!(!reuses_a_given_back_block(
        &StaticBumpMemory::new(aligned(512)),
        512
    ));
}

/// Whether three neighbouring blocks that fill the span, given back in
/// `order`, come together into one block of the whole span.
fn merges_three_neighbours<M: MemoryProviderApi>(m: &M, third: usize, order: [usize; 3]) -> bool {
    let blocks = [allocate(m, third, 16), allocate(m, third, 16), allocate(m, third, 16)];
    assert!(
        blocks.iter().all(|p| !p.is_null()),
        "three thirds fill the span"
    );
    for i in order {
        free(m, blocks[i], third, 16);
    }
    !allocate(m, 3 * third, 16).is_null()
}

#[test]
fn neighbours_given_back_in_any_order_merge_into_one() {
    let orders = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    for order in orders {
        let m = StaticFreeListMemory::new(aligned(192));
        assert!(
            merges_three_neighbours(&m, 64, order),
            "order {order:?} merges"
        );
        assert_eq!(m.used(), USize(192));
    }
    assert!(!merges_three_neighbours(
        &StaticBumpMemory::new(aligned(192)),
        64,
        [1, 0, 2]
    ));
}

/// A block between two live ones, given back, is handed out again for a
/// length that fits it, and a longer one is refused, so the gap is a gap and
/// not merged with live memory.
#[test]
fn a_gap_between_live_blocks_is_reused_and_not_widened() {
    let m = StaticFreeListMemory::new(aligned(192));
    let a = allocate(&m, 64, 16);
    let b = allocate(&m, 64, 16);
    let c = allocate(&m, 64, 16);
    assert!(!a.is_null() && !b.is_null() && !c.is_null());
    free(&m, b, 64, 16);
    assert!(allocate(&m, 80, 16).is_null(), "the gap is 64 bytes");
    assert_eq!(allocate(&m, 64, 16), b, "the gap is handed out again");
}

/// A block aligned above the grain leaves the space in front of it on the
/// list: a small block asked for after it lands in that space, and
/// everything given back makes the whole span one block again.
#[test]
fn the_space_in_front_of_an_aligned_block_stays_free() {
    let buffer = aligned_to(512, 128);
    let base = buffer.as_ptr().addr();
    let m = StaticFreeListMemory::new(buffer);
    let small = allocate(&m, GRAIN, GRAIN);
    assert_eq!(small.addr(), base, "the first block starts the buffer");
    let wide = allocate(&m, 64, 128);
    assert_eq!(wide.addr(), base + 128, "the next 128-byte boundary");
    let gap = allocate(&m, GRAIN, GRAIN);
    assert_eq!(gap.addr(), base + GRAIN, "the space in front of it is free");
    free(&m, gap, GRAIN, GRAIN);
    free(&m, wide, 64, 128);
    free(&m, small, GRAIN, GRAIN);
    assert_eq!(m.used(), USize(0));
    assert!(
        !allocate(&m, 512, GRAIN).is_null(),
        "the whole span is one block again"
    );
}

/// The list is laid over the grain-aligned interior of a buffer that starts
/// off a grain boundary and does not end on one: the interior is handed out
/// whole, at its first grain boundary, and nothing past the buffer's end is
/// written. A buffer shorter than its way to the first boundary, or with
/// less than a grain past it, hands out nothing.
#[test]
fn the_list_is_laid_over_the_grain_aligned_interior() {
    const CANARY: u8 = 0x5A;
    for offset in 1 .. GRAIN {
        for len in [1000 + offset, 1001, 999 + 2 * offset] {
            let whole = aligned(len + 2 * GRAIN);
            for b in whole.iter_mut() {
                b.write(CANARY);
            }
            let start = whole.as_ptr().addr() + offset;
            let lead = GRAIN - offset;
            let interior = (len - lead) & !(GRAIN - 1);
            // SAFETY: `offset + len` lies inside `whole`, which is leaked and
            // reached by nothing else while the provider runs.
            let m = unsafe {
                StaticFreeListMemory::from_raw_parts(whole.as_mut_ptr().add(offset), USize(len))
            };
            let p = allocate(&m, interior, GRAIN);
            assert_eq!(
                p.addr(),
                start + lead,
                "offset {offset}, len {len}: the interior starts at the first boundary"
            );
            assert!(p.addr() + interior <= start + len, "inside the buffer");
            assert!(
                allocate(&m, GRAIN, GRAIN).is_null(),
                "offset {offset}, len {len}: the interior was handed out whole"
            );
            for (i, b) in whole[offset + len ..].iter().enumerate() {
                // SAFETY: every byte of `whole` was written above.
                assert_eq!(
                    unsafe { b.assume_init() },
                    CANARY,
                    "offset {offset}, len {len}: byte {i} past the end is untouched"
                );
            }
        }
    }
    for (offset, len) in [(1, GRAIN - 2), (1, GRAIN), (GRAIN - 1, 1)] {
        let whole = aligned(2 * GRAIN);
        // SAFETY: as above.
        let m = unsafe {
            StaticFreeListMemory::from_raw_parts(whole.as_mut_ptr().add(offset), USize(len))
        };
        assert!(
            allocate(&m, 0, 1).is_null(),
            "offset {offset}, len {len}: no grain fits"
        );
    }
}

/// A zero length takes one grain, and every length is rounded up to the
/// grain; `used` counts it and comes back to zero.
#[test]
fn used_counts_grain_padding_and_comes_back_to_zero() {
    let m = StaticFreeListMemory::new(aligned(1024));
    assert_eq!(m.capacity(), USize(1024));
    let z = allocate(&m, 0, 1);
    assert!(!z.is_null());
    assert_eq!(m.used(), USize(GRAIN));
    let odd = allocate(&m, GRAIN + 1, 1);
    assert_eq!(m.used(), USize(3 * GRAIN));
    free(&m, odd, GRAIN + 1, 1);
    free(&m, z, 0, 1);
    assert_eq!(m.used(), USize(0));
    assert!(
        !allocate(&m, 1024, 16).is_null(),
        "the whole span is one block again"
    );
}

/// An alignment that is not a power of two, and a length past the free
/// space, are refused with null and change nothing.
#[test]
fn a_request_that_does_not_fit_is_refused_and_changes_nothing() {
    let m = StaticFreeListMemory::new(aligned(256));
    let p = allocate(&m, 64, 16);
    assert!(!p.is_null());
    let used = m.used();
    assert!(allocate(&m, 8, 3).is_null(), "3 is not a power of two");
    assert!(allocate(&m, 193, 16).is_null(), "past the 192 bytes left");
    assert!(
        allocate(&m, usize::MAX, 16).is_null(),
        "a length past usize when rounded"
    );
    assert_eq!(m.used(), used);
    assert!(
        !allocate(&m, 192, 16).is_null(),
        "the rest is still there whole"
    );
}

/// The default `allocate_zeroed` writes the zeros a block given back and
/// handed out again does not have: the same block through `allocate` still
/// carries what was written into it past its first grain, which the list
/// node overwrote.
#[test]
fn allocate_zeroed_writes_the_zeros_a_reused_block_lacks() {
    let m = StaticFreeListMemory::new(aligned(256));
    let p = allocate(&m, 256, 16);
    // SAFETY: p covers 256 bytes.
    unsafe { core::ptr::write_bytes(p, 0xAB, 256) };
    free(&m, p, 256, 16);
    let again = allocate(&m, 256, 16);
    assert_eq!(again, p);
    // SAFETY: again covers 256 bytes.
    assert_eq!(
        unsafe { again.add(GRAIN).read() },
        0xAB,
        "a reused block is not zero"
    );
    free(&m, again, 256, 16);
    // SAFETY: alignment is a power of two.
    let zeroed = unsafe { m.allocate_zeroed(USize(256), USize(16)) };
    assert_eq!(zeroed, p);
    for i in 0 .. 256 {
        // SAFETY: zeroed covers 256 bytes.
        assert_eq!(unsafe { zeroed.add(i).read() }, 0, "byte {i}");
    }
}

/// Several threads allocating, writing, checking and freeing at once never
/// see another thread's bytes in their block, and leave nothing handed out.
#[test]
fn threads_allocating_and_freeing_at_once_get_disjoint_blocks() {
    let m: &'static StaticFreeListMemory =
        Box::leak(Box::new(StaticFreeListMemory::new(aligned(64 * 1024))));
    let threads: Vec<_> = (0 .. 8u8)
        .map(|t| {
            std::thread::spawn(move || {
                let mut held: Vec<(*mut u8, usize)> = Vec::new();
                for round in 0 .. 2000usize {
                    let len = 16 + (round * 7 + t as usize * 13) % 200;
                    let p = allocate(m, len, 16);
                    if p.is_null() {
                        continue;
                    }
                    // SAFETY: p covers len bytes.
                    unsafe { core::ptr::write_bytes(p, t, len) };
                    held.push((p, len));
                    if held.len() > 4 {
                        let (q, hl) = held.remove(round % held.len());
                        for i in 0 .. hl {
                            // SAFETY: q covers hl bytes this thread holds.
                            assert_eq!(
                                unsafe { q.add(i).read() },
                                t,
                                "another thread wrote into this block"
                            );
                        }
                        free(m, q, hl, 16);
                    }
                }
                for (q, hl) in held {
                    free(m, q, hl, 16);
                }
            })
        })
        .collect();
    for t in threads {
        t.join().expect("a thread failed its check");
    }
    assert_eq!(m.used(), USize(0));
    assert!(
        !allocate(m, 64 * 1024, 16).is_null(),
        "everything merged back into one block"
    );
}
