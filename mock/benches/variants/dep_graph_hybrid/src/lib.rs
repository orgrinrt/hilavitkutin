//! Variant: hybrid dense-or-CSR adjacency based on compile-time N.
//!
//! Threshold N <= 16: dense path. Threshold N > 16: CSR path. The bench_variant
//! macro instantiates one specialised function per N; LLVM elides the unused
//! branch entirely.
//!
//! Same deterministic edge set as dep_graph_dense / dep_graph_csr for
//! byte-exact validation.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

const MAX_N: usize = 256;
const CSR_THRESHOLD: usize = 16;
const MAX_EDGES: usize = MAX_N * 3 + 16;

#[inline(never)]
fn build_dense(n: usize, adj: &mut [bool]) {
    for u in 0 .. n {
        for &step in &[1usize, 2, 5] {
            let v = u + step;
            if v < n {
                adj[u * n + v] = true;
            }
        }
    }
}

#[inline(never)]
fn iter_all_successors_dense(n: usize, adj: &[bool], seed: u64) -> u64 {
    let mut acc: u64 = seed;
    for u in 0 .. n {
        for v in 0 .. n {
            if adj[u * n + v] {
                acc = (acc ^ (v as u64)).wrapping_mul(0x100000001B3);
            }
        }
    }
    acc
}

#[inline(never)]
fn build_csr(n: usize, row_offsets: &mut [u32], col_indices: &mut [u32]) {
    let mut write = 0u32;
    row_offsets[0] = 0;
    for u in 0 .. n {
        for &step in &[1usize, 2, 5] {
            let v = u + step;
            if v < n {
                col_indices[write as usize] = v as u32;
                write += 1;
            }
        }
        row_offsets[u + 1] = write;
    }
}

#[inline(never)]
fn iter_all_successors_csr(n: usize, row_offsets: &[u32], col_indices: &[u32], seed: u64) -> u64 {
    let mut acc: u64 = seed;
    for u in 0 .. n {
        let start = row_offsets[u] as usize;
        let end = row_offsets[u + 1] as usize;
        for v in &col_indices[start .. end] {
            acc = (acc ^ (*v as u64)).wrapping_mul(0x100000001B3);
        }
    }
    acc
}

fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xCBF29CE484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001B3);
    }
    acc
}

#[bench_variant("dep_graph_hybrid", sizes = [8, 16, 32, 64, 128, 256])]
fn run_dep_graph_hybrid<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let nodes = if N > MAX_N { MAX_N } else { N };
    // The scratch of both paths lives outside the timed region and each call
    // clears the prefix it uses, as the dense and CSR arms do. The seed is taken
    // inside the call, as it is there.
    let mut adj = [false; MAX_N * MAX_N];
    let mut row_offsets = [0u32; MAX_N + 1];
    let mut col_indices = [0u32; MAX_EDGES];
    timed! {
        run {
            let seed = seed_from_input(input);
            let acc = if nodes <= CSR_THRESHOLD {
                adj[..nodes * nodes].fill(false);
                build_dense(nodes, &mut adj[..nodes * nodes]);
                iter_all_successors_dense(nodes, &adj[..nodes * nodes], seed)
            } else {
                row_offsets[..nodes + 1].fill(0);
                col_indices[..nodes * 3].fill(0);
                build_csr(nodes, &mut row_offsets[..nodes + 1], &mut col_indices);
                iter_all_successors_csr(nodes, &row_offsets[..nodes + 1], &col_indices, seed)
            };
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_arm_does_not_time_a_constant() {
        assert!(kernel_checks::varies(|input| {
            let mut out = [0u8; 8];
            let _ = super::run_dep_graph_hybrid::<256>(input, &mut out);
            u64::from_le_bytes(out)
        }));
    }
}
