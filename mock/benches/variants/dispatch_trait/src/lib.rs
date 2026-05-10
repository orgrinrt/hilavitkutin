//! Variant: sealed-trait static dispatch.
//!
//! This is the codegen shape hilavitkutin's actual WorkUnit dispatch resolves
//! to under monomorphisation. A const generic selects the impl; LLVM sees the
//! concrete type at every call site and can inline through the trait method.

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

mod sealed {
    pub trait Sealed {}
}

trait Mixer: sealed::Sealed {
    fn mix(chunk: u64, acc: u64) -> u64;
}

struct Fnv;
impl sealed::Sealed for Fnv {}
impl Mixer for Fnv {
    #[inline(never)]
    fn mix(chunk: u64, acc: u64) -> u64 {
        (acc ^ chunk).wrapping_mul(0x100000001b3)
    }
}

#[inline(always)]
fn dispatch<M: Mixer>(chunk: u64, acc: u64) -> u64 {
    M::mix(chunk, acc)
}

#[bench_variant("dispatch_trait", sizes = [64, 256, 1024, 4096, 16384])]
fn run_dispatch_trait<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let mut acc: u64 = 0xcbf29ce484222325;
            let chunks = N / 8;
            let in_ptr = input.as_ptr();
            for i in 0..chunks {
                let chunk =
                    unsafe { (in_ptr.add(i * 8) as *const u64).read_unaligned() };
                acc = dispatch::<Fnv>(chunk, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
