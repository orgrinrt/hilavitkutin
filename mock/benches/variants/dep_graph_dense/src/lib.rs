//! Variant: dense adjacency matrix (NxN bool).
//!
//! N is interpreted as node count; the input bytes are only mixed into the
//! starting accumulator (consumed as a seed-bytes hash, so different input
//! workloads produce different outputs even though the DAG shape itself is
//! deterministic per N).
//!
//! Edge set: for each u in 0..N, edges to (u+1) % N, (u+2) % N, (u+5) % N
//! filtered to keep only v > u (DAG constraint). Deterministic; same across
//! all dep_graph variants for byte-exact validation.

use mockspace_bench_core::{FfiBenchCall, timed};
use mockspace_bench_macro::bench_variant;

const MAX_N: usize = 256;

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

fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xCBF29CE484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001B3);
    }
    acc
}

#[bench_variant("dep_graph_dense", sizes = [8, 16, 32, 64, 128, 256])]
fn run_dep_graph_dense<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let nodes = if N > MAX_N { MAX_N } else { N };

    // The matrix lives outside the timed region, and each call clears only the
    // `nodes * nodes` cells the graph owns. Allocating `MAX_N * MAX_N` inside
    // the call cost a fixed 64 KB of zeroing at every size, most of the time at
    // the small sizes the hybrid comparison was about.
    let mut adj = [false; MAX_N * MAX_N];
    timed! {
        run {
            adj[..nodes * nodes].fill(false);
            build_dense(nodes, &mut adj[..nodes * nodes]);
            let seed = seed_from_input(input);
            let acc = iter_all_successors_dense(nodes, &adj[..nodes * nodes], seed);
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
            let _ = super::run_dep_graph_dense::<256>(input, &mut out);
            u64::from_le_bytes(out)
        }));
    }
}
