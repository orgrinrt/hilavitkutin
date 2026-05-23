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

use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

const MAX_N: usize = 256;

#[inline(never)]
fn build_dense(n: usize, adj: &mut [bool]) {
    for u in 0..n {
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
    for u in 0..n {
        for v in 0..n {
            if adj[u * n + v] {
                acc = (acc ^ (v as u64)).wrapping_mul(0x100000001b3);
            }
        }
    }
    acc
}

fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001b3);
    }
    acc
}

#[bench_variant("dep_graph_dense", sizes = [8, 16, 32, 64, 128, 256])]
fn run_dep_graph_dense<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let nodes = if N > MAX_N { MAX_N } else { N };

    timed! {
        run {
            let mut adj = [false; MAX_N * MAX_N];
            build_dense(nodes, &mut adj[..nodes * nodes]);
            let seed = seed_from_input(input);
            let acc = iter_all_successors_dense(nodes, &adj[..nodes * nodes], seed);
            output.copy_from_slice(&acc.to_le_bytes());
        }
    }
}
