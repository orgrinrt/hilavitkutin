//! Variant: `match` with 64 arms (large dense range).
//!
//! Lowers to a 64-entry jump table on aarch64. Same instruction
//! shape as 8-arm match but the table is larger (512 bytes vs
//! 64 bytes), so it consumes more icache. Models plan-stage
//! enum dispatch when an enum grows large (potentially WorkUnit
//! kind dispatch in a large-app scheduler).

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

#[inline(always)]
fn step_64(v: u64, acc: u64) -> u64 {
    // Each arm does a tiny distinct op so LLVM doesn't fold arms.
    // Pattern repeats every 8 arms but the constants vary to defeat folding.
    match v & 63 {
        0 => acc.wrapping_add(v ^ 0xA1),
        1 => acc ^ (v.wrapping_mul(0xB1)),
        2 => acc.wrapping_sub(v ^ 0xC1),
        3 => acc.rotate_left(1) ^ v,
        4 => acc.wrapping_mul((v | 1) ^ 0xD1),
        5 => acc & v.wrapping_add(0xE1),
        6 => acc | v.wrapping_add(0xF1),
        7 => acc.rotate_right(2) ^ v,
        8 => acc.wrapping_add(v ^ 0xA2),
        9 => acc ^ (v.wrapping_mul(0xB2)),
        10 => acc.wrapping_sub(v ^ 0xC2),
        11 => acc.rotate_left(3) ^ v,
        12 => acc.wrapping_mul((v | 1) ^ 0xD2),
        13 => acc & v.wrapping_add(0xE2),
        14 => acc | v.wrapping_add(0xF2),
        15 => acc.rotate_right(4) ^ v,
        16 => acc.wrapping_add(v ^ 0xA3),
        17 => acc ^ (v.wrapping_mul(0xB3)),
        18 => acc.wrapping_sub(v ^ 0xC3),
        19 => acc.rotate_left(5) ^ v,
        20 => acc.wrapping_mul((v | 1) ^ 0xD3),
        21 => acc & v.wrapping_add(0xE3),
        22 => acc | v.wrapping_add(0xF3),
        23 => acc.rotate_right(6) ^ v,
        24 => acc.wrapping_add(v ^ 0xA4),
        25 => acc ^ (v.wrapping_mul(0xB4)),
        26 => acc.wrapping_sub(v ^ 0xC4),
        27 => acc.rotate_left(7) ^ v,
        28 => acc.wrapping_mul((v | 1) ^ 0xD4),
        29 => acc & v.wrapping_add(0xE4),
        30 => acc | v.wrapping_add(0xF4),
        31 => acc.rotate_right(8) ^ v,
        32 => acc.wrapping_add(v ^ 0xA5),
        33 => acc ^ (v.wrapping_mul(0xB5)),
        34 => acc.wrapping_sub(v ^ 0xC5),
        35 => acc.rotate_left(9) ^ v,
        36 => acc.wrapping_mul((v | 1) ^ 0xD5),
        37 => acc & v.wrapping_add(0xE5),
        38 => acc | v.wrapping_add(0xF5),
        39 => acc.rotate_right(10) ^ v,
        40 => acc.wrapping_add(v ^ 0xA6),
        41 => acc ^ (v.wrapping_mul(0xB6)),
        42 => acc.wrapping_sub(v ^ 0xC6),
        43 => acc.rotate_left(11) ^ v,
        44 => acc.wrapping_mul((v | 1) ^ 0xD6),
        45 => acc & v.wrapping_add(0xE6),
        46 => acc | v.wrapping_add(0xF6),
        47 => acc.rotate_right(12) ^ v,
        48 => acc.wrapping_add(v ^ 0xA7),
        49 => acc ^ (v.wrapping_mul(0xB7)),
        50 => acc.wrapping_sub(v ^ 0xC7),
        51 => acc.rotate_left(13) ^ v,
        52 => acc.wrapping_mul((v | 1) ^ 0xD7),
        53 => acc & v.wrapping_add(0xE7),
        54 => acc | v.wrapping_add(0xF7),
        55 => acc.rotate_right(14) ^ v,
        56 => acc.wrapping_add(v ^ 0xA8),
        57 => acc ^ (v.wrapping_mul(0xB8)),
        58 => acc.wrapping_sub(v ^ 0xC8),
        59 => acc.rotate_left(15) ^ v,
        60 => acc.wrapping_mul((v | 1) ^ 0xD8),
        61 => acc & v.wrapping_add(0xE8),
        62 => acc | v.wrapping_add(0xF8),
        _ => acc.rotate_right(16) ^ v,
    }
}

#[bench_variant("match_64_arms", sizes = [256, 1024, 4096, 16384])]
fn run_match_64_arms<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    timed! {
        run {
            let words = N / 8;
            let p = input.as_ptr() as *const u64;
            let mut acc: u64 = 0xcbf29ce484222325;
            for i in 0..words {
                let v = unsafe { *p.add(i) };
                acc = step_64(v, acc);
            }
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
