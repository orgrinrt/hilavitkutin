//! The native front end: runs every case of the kernel and prints one line
//! per digest, then one line for how the case ended.

use wasm_digest_kernel::{CASES, STEPS, run_case};

fn main() {
    for case in 0 .. CASES {
        let mut out = [0u64; STEPS];
        let report = run_case(case, &mut out);
        for (i, digest) in out.iter().take(report.written).enumerate() {
            println!("c{case} s{} {digest:016x}", i + 1);
        }
        println!("c{case} end {report}");
    }
}
