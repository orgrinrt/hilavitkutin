//! Plan capacity dimensions bundled as one type parameter.
//!
//! Each engine plan/thread/dispatch structure sizes its fixed arrays by some
//! subset of thirteen capacity dimensions. Rather than carry that subset as
//! individual `Cap` const generics (which feed `[T; cap_size(N)]` and overflow
//! `generic_const_exprs` when threaded from generic scheduler code), the engine
//! bundles them into one `PlanDims` trait whose associated types are each an
//! arvo `Capacity`. A structure takes one `D: PlanDims` and projects
//! `<D::Units as Capacity>::Array<T>` and the like; the dimension is a type, so
//! no `cap_size` sits in array-length position and the bundle threads through
//! generic code with no overflow. The live count stays a runtime `USize`
//! bounded by the dimension's `CAP`.

use arvo_tensor::{Capacity, Dim};

/// The engine's plan capacity dimensions, each a `Capacity` type.
pub trait PlanDims {
    /// WorkUnits in the pipeline.
    type Units: Capacity;
    /// Distinct stores (access-mask width).
    type Stores: Capacity;
    /// Dependency-graph edges.
    type Edges: Capacity;
    /// Waist-delimited synchronisation phases.
    type Phases: Capacity;
    /// Trunks plan-wide.
    type Trunks: Capacity;
    /// Trunks within one phase.
    type TrunksPerPhase: Capacity;
    /// Fibers plan-wide.
    type Fibers: Capacity;
    /// Parallel dispatch lanes.
    type Lanes: Capacity;
    /// Distinct columns tracked plan-wide.
    type Columns: Capacity;
    /// Components within one trunk.
    type ComponentsPerTrunk: Capacity;
    /// WorkUnits within one fiber.
    type UnitsPerFiber: Capacity;
    /// Columns one fiber touches.
    type ColumnsPerFiber: Capacity;
    /// Hardware cores driving dispatch.
    type Cores: Capacity;
}

/// The default engine capacity budget. Consumers override `PlanDims` to size
/// the engine to their workload; these are tunable defaults, not hard limits
/// (the live count is a runtime value bounded by each dimension's `CAP`).
pub struct DefaultPlanDims;

impl PlanDims for DefaultPlanDims {
    type Units = Dim<64>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Stores = Dim<64>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Edges = Dim<256>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Phases = Dim<32>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Trunks = Dim<64>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type TrunksPerPhase = Dim<32>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Fibers = Dim<64>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Lanes = Dim<32>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Columns = Dim<64>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal, kept >= Stores; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type ComponentsPerTrunk = Dim<32>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type UnitsPerFiber = Dim<32>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type ColumnsPerFiber = Dim<16>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
    type Cores = Dim<256>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: capacity budget literal, matches thread/class.rs MAX_CORES; Dim<N> array-length-grammar root, the permitted bare primitive in the capacity-as-type convention; tracked: #649
}
