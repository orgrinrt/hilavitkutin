//! The std arm: the fair parallel bar of
//! `mock/research/202606081100_gate2-parallel-bench-fairness.md`.
//!
//! `std::thread::scope` over the record range in `ceil(n / W)` chunks, each
//! thread folding or filling its own chunk, joined at the end. The threads
//! are spawned and joined inside every frame, which is what idiomatic std
//! parallelism costs; the crew and the engine park theirs between frames.

use crate::ConvOutput;
use crate::engine::Op;
use crate::kernel::{Kernel, fold_forward};

/// The fold arm on `threads` scoped threads.
pub fn fold<K: Kernel>(data: Vec<u32>, threads: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
    let n = data.len();
    let per = n.div_ceil(threads.max(1)).max(1);
    let mut last = ConvOutput::default();
    move |op| {
        match op {
            Op::Frame => {
                let value = std::thread::scope(|sc| {
                    let handles: Vec<_> = data
                        .chunks(per)
                        .map(|c| sc.spawn(move || fold_forward::<K>(0, c)))
                        .collect();
                    handles
                        .into_iter()
                        .map(|h| h.join().expect("a fold thread panicked"))
                        .fold(0u64, u64::wrapping_add)
                });
                last = ConvOutput {
                    value,
                    count: n as u64,
                };
                ConvOutput::default()
            },
            Op::Read => last,
        }
    }
}

/// The append arm on `threads` scoped threads, each filling the output
/// chunk of its input chunk.
pub fn append<K: Kernel>(data: Vec<u32>, threads: usize) -> impl FnMut(Op) -> ConvOutput + 'static {
    let n = data.len();
    let per = n.div_ceil(threads.max(1)).max(1);
    let mut out = vec![0u32; n];
    move |op| {
        match op {
            Op::Frame => {
                std::thread::scope(|sc| {
                    for (oc, ic) in out.chunks_mut(per).zip(data.chunks(per)) {
                        sc.spawn(move || {
                            for (o, &x) in oc.iter_mut().zip(ic) {
                                *o = K::apply(x);
                            }
                        });
                    }
                });
                ConvOutput::default()
            },
            Op::Read => {
                ConvOutput {
                    value: crate::data::hash(&out),
                    count: n as u64,
                }
            },
        }
    }
}
