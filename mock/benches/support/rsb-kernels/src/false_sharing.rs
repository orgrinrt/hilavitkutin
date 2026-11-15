//! Axis E of the resource-storage round: cross-core false sharing.
//!
//! One thread per slot, each writing only its own `u32` slot in a tight loop.
//! Packed puts every slot inside one 64-byte cache line, the hazard of the V3
//! shape-bound layout, where several resources' members share one column and so
//! one line. Padded puts each slot on its own line, as the per-resource-line
//! layouts do. The writes never touch another thread's slot, so any difference
//! between the two arms is the line moving between cores.
//!
//! The thread count is the machine's parallelism, at most sixteen, so sixteen
//! four-byte slots fit the one packed line. Both arms on one machine run the
//! same threads and the same writes, and both return the same digest.

/// The cache line the slots are packed into or spread over.
pub const LINE: usize = 64;

/// The most threads a pass runs: sixteen `u32` slots fill one line.
pub const MAX_THREADS: usize = LINE / 4;

/// How many threads a pass runs on this machine.
pub fn threads() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(MAX_THREADS)
}

/// One pass: every thread writes its slot `iters_per` times. Returns the sum
/// of the slots read back after the threads join, which is
/// `threads() * iters_per` in both layouts.
pub fn pass(packed: bool, iters_per: u64) -> u64 {
    let slots = threads();
    let region = if packed { LINE } else { LINE * slots };
    let mut backing = vec![0u64; (region + LINE) / 8 + 1];
    let base = {
        let p = backing.as_mut_ptr() as usize;
        p.div_ceil(LINE) * LINE
    };
    let stride = if packed { 4 } else { LINE };
    std::thread::scope(|sc| {
        for s in 0 .. slots {
            let addr = base + s * stride;
            sc.spawn(move || {
                let p = addr as *mut u32;
                let mut v = 0u32;
                let mut it = 0u64;
                while it < iters_per {
                    v = v.wrapping_add(1);
                    // SAFETY: `addr` lies inside `backing`, which outlives the
                    // scope, and only this thread writes it.
                    unsafe { std::ptr::write_volatile(p, v) };
                    it += 1;
                }
            });
        }
    });
    let mut sum = 0u64;
    for s in 0 .. slots {
        // SAFETY: as above; the threads have joined.
        sum += u64::from(unsafe { std::ptr::read_volatile((base + s * stride) as *const u32) });
    }
    drop(backing);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_layouts_leave_every_slot_at_its_write_count() {
        let want = threads() as u64 * 1000;
        assert_eq!(pass(true, 1000), want);
        assert_eq!(pass(false, 1000), want);
    }

    #[test]
    fn the_digest_counts_the_writes() {
        // The case that must fail if the slots were not all written: a pass
        // of zero writes reads back zero, not the count of a longer pass.
        assert_eq!(pass(true, 0), 0);
        assert_ne!(pass(true, 7), pass(true, 8));
    }
}
