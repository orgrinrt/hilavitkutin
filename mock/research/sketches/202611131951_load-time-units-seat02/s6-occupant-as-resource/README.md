# s6: an occupant swapped after build, and the record's own seam

Spike, seat 02 phase two, round 202611131951. Uses the shipped `hilavitkutin-api`, `hilavitkutin` and
`hilavitkutin-providers` at `271909e5`. Written to decide two disagreements with seat 01: whether
load-time code can reach a compiled unit after `build` with the unit set fixed, and whether the
record's own seam, `ExtensionSurface<dyn F>`, can carry it today.

| Test | Result | What it says |
|---|---|---|
| `tests/occupant.rs` `an_occupant_swapped_after_build_changes_the_next_frame_and_opens_the_plan_band_once` | green | A compiled host unit reads a `Resource<Occupant>`, where `Occupant` is a C-ABI kernel pointer marked `PlanAffecting`, and a column; it calls the occupant once a morsel. Frame 1 runs occupant v1 on all 20 000 records. A clean frame 2 skips the host (7 to 7 runs) and leaves the plan band shut. `replace_resource(Occupant(v2))` after build, the stand-in for a library loaded after the schedule exists: frame 3 opens the plan band once, the host reruns (7 to 14), every record is v2's. Frame 4 shuts the band. The unit set never changed. |
| `tests/surface.rs` `a_unit_reading_the_surface_builds_and_runs` | does not compile | Registering `ExtensionSurface::<dyn Family>::new()` with `.with` and building fails: `ExtensionSurface` implements none of `StoreAccumKind`, `StoreElemBytes`, `BindingsFor`, `DrainStores`, which `build` requires of every registered store (`OUTPUT-surface-does-not-compile.txt`). The api design's "the typestate proves coverage at `.build()` time" (`hilavitkutin-api/DESIGN.md.tmpl:208-210`) has no expressible form on the trunk. Not restructured. |

The occupants are two in-process `extern "C"` functions standing in for a loaded library's exports;
the transport (a fresh cdylib through the loader) is s1's and seat 01's `a4-slot-unit`. The host
copies each morsel into scratch and back because the trunk context gives per-record access only
(`hilavitkutin-api/src/context.rs:117`, `:134`); that copy is forced scaffolding and is not part of
the claim. No timing; nothing here is a bench.

What it does not show: unloading. A library dropped while a resource still holds one of its
function pointers leaves a dangling occupant; nothing here ties the occupant's lifetime to the
library handle.

Run: `env -C <this dir> CARGO_INCREMENTAL=0 cargo test --no-fail-fast --test occupant -- --nocapture --test-threads=1`
and `env -C <this dir> CARGO_INCREMENTAL=0 cargo test --no-fail-fast --test surface`.
