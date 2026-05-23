//! Engine-side home for `StandardCodegen` emit code (Topic 4 axis A).
//!
//! `StandardCodegen` itself (the marker) and the trait impls
//! (`DispatchCodegen<Cfg>`, `LockFreeDispatch`, `Scheduled`) live in
//! `hilavitkutin_api::dispatch_codegen`. The seal is api-internal,
//! and the orphan rule requires the trait impls to live with the
//! marker; ergo the api crate carries both. This module hosts the
//! engine-side emit code that consumes the marker.
//!
//! Currently re-exports `StandardCodegen` for convenience. The
//! per-core dispatch emit entrypoint, `dispatch_for_core<Cfg>()`,
//! lands here once the closure body wires across the remaining
//! Pass 3 CHANGE blocks (`run_fiber`, `wu_fn`, `progress`, `sync`,
//! `morsel`). Sketch heritage confirms the TAIT pattern is sound:
//!
//! - `mock/research/sketches/202605101036-codegen-entrypoint-tait/`
//!   (TAIT lowers transparently: zero `blr`, closed-form arithmetic).
//! - `mock/research/sketches/202605101036-codegen-tait-capture/`
//!   (TAIT under realistic envelope: 10 const generics on Cfg,
//!   captured `&mut`, multiple sealed impls coexisting).
//! - `mock/research/sketches/202605101036-fibershape-typing/`
//!   (per-shape monomorphisation via sealed FiberShape).
//! - `mock/research/sketches/202605101036-progress-counter-arena/`
//!   (single `stlr`/`ldar` per progress operation).

pub use hilavitkutin_api::dispatch_codegen::StandardCodegen;
