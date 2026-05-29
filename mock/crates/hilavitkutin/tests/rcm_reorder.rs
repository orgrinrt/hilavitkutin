//! Step 4 RCM reorder wiring test (Phase C C1b).

// `rcm_reorder` returns `[UnitId; cap_size(MAX_UNITS)]`, a type carrying
// a `[(); cap_size(N)]:` bound through the C1a CsrBidirectional adapter,
// so this crate enables generic_const_exprs to normalise that bound, the
// same gate the csr_adapter test carries. adt_const_params is not needed:
// only Cap values via cap(N) are named, never a Cap const param.
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, USize};
use arvo_tensor::{cap, cap_size};
use hilavitkutin::plan::{steps, DependencyGraph};

const UNITS: Cap = cap(8); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const EDGES: Cap = cap(16); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121

// The chain's unit indices, named once so the per-literal lint:allow
// lives in one place and the edges / expectations read as structure.
const U0: USize = USize(0); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U1: USize = USize(1); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U2: USize = USize(2); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U3: USize = USize(3); // lint:allow(no-bare-numeric) reason: chain unit index; tracked: #427
const U4: USize = USize(4); // lint:allow(no-bare-numeric) reason: first slack unit index; tracked: #427

/// Build the chain DAG 0->1->2->3 in an eight-unit / sixteen-edge cap,
/// four units and three edges live.
///
/// Node 3 is a pure sink, so `add_edge` alone leaves `unit_count` at 3.
/// The engine's `build_dag` finishes by advancing `unit_count` to the
/// true node count and giving the trailing node an empty row; this
/// mirrors that single-trailing-sink finalise so node 3 is a live,
/// zero-out-degree row rather than slack.
fn chain() -> DependencyGraph<UNITS, EDGES> {
    let mut g: DependencyGraph<UNITS, EDGES> = DependencyGraph::new();
    g.add_edge(U0, U1);
    g.add_edge(U1, U2);
    g.add_edge(U2, U3);
    g.row_offsets[g.unit_count.0] = g.edge_count;
    g.unit_count = U4;
    g
}

#[test]
fn rcm_renumbers_chain_to_bandwidth_reverse() {
    let order = steps::rcm_reorder::<UNITS, EDGES>(&chain());
    // RCM of the undirected chain 0-1-2-3: min-degree start (node 0),
    // ascending-degree BFS, final reverse, so the live order is
    // [3, 2, 1, 0]. Positions past the live count (4..8) keep the
    // default-zero fill, because rcm_reorder_via seeds only over the
    // CSR's live node_count(). Asserted by unit index across all eight
    // slots. This fails on a passthrough stub (which returns the topo
    // order [0, 1, 2, 3, ...]) and on a packed CSR (where the degree-0
    // slack nodes 4-7 would seed first and scramble the order), so it
    // transitively guards the C1a live-count plumbing.
    let expect: [USize; cap_size(UNITS)] = [U3, U2, U1, U0, U0, U0, U0, U0];
    for (pos, want) in expect.iter().enumerate() {
        assert_eq!(order[pos].index(), *want, "rcm slot {pos}");
    }
}
