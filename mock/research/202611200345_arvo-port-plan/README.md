# The Port of Hilavitkutin onto Current Arvo, as a Plan

This is the plan of part 2 of the stack plan (clause-dev `.shared/research/202611191916_stack-next-tracks/README.md`): hilavitkutin moved off the arvo revision it pins and onto arvo as it stands today, in full. It was asked for by stack-coord on 20 November 2026 as the job `hila-arvo-port-plan`. It plans and builds nothing. No source, design or registry row was changed for it, in hilavitkutin or in arvo.

The trigger is the ticket `hilavitkutin_builds_on_no_nightly_past_2026_05_28_until_its`. Hilavitkutin pins ten crates of arvo at `1b2796e9` (`mock/Cargo.toml:45-54`), that revision builds on no nightly past 2026-05-28, and a backport is real work in a tree arvo has already left (clause-dev `.shared/research/202611192200_nightly-bump/REPORT.md`, the hilavitkutin section). So hilavitkutin's toolchain pin, and the wild 0.11.0 that `build-pragmas` wants, wait on this port.

The plan is written so that opus and sonnet helpers build the port from it with no further planning. It gives the inventory of every arvo item hilavitkutin names, what arvo offers in its place today, the rows arvo still owes in hilavitkutin's own terms, the jobs in order with what each waits on, and the brief of the first job, which waits on nobody.

What is read and what is relayed is kept apart. A file, a row or a commit named with its path, id or hash was opened for this plan. Hilavitkutin is read at `dev` `9918113f`, arvo at `dev` `d15dd473`, arvo's contracts at `work/numeral-contracts` `f0615270`. A lane's state is the brief's and the state file's (`.shared/state/stack-chart.md`) and was not checked further. Nothing here was built, run or benched, and the section at the end lists what a job has to check before it leans on a sentence of this plan.

## What governs

The gate is op's and is the full port. File 40 of the stack record (`.shared/research/202611150830_stack-path/40-the-roadmap-final.md`, "The gate on kaski", part 2) reads: hilavitkutin uses arvo fully, the port is on hilavitkutin's `dev`, it pins no older arvo, and its `generic_const_exprs` line is gone. File 41 records that he accepted the gate at that size ("Yes, write the sketches"), with the floats and every algorithm family in it. So no job below keeps an old arvo crate as a stand-in past the last job, and none grows a numeral, a mask or a graph routine inside hilavitkutin.

Hilavitkutin's canon holds one question open on exactly this, and this plan does not fill it. `question::whether_hilavitkutin_grows_its_own_numeral_mask_graph_sparse_spectral_combinatorial_and_hash_code_or_drops_the_design_steps_those_arvo_crates_serve` (state open, from the round of 18 September) offers two options: grow the code here, or drop the plan steps. Op's gate of 15 November takes neither: arvo delivers the families and hilavitkutin waits for them. The row still stands open in the registry, so it is named here as waiting on him to close with his later words, and the first job files that as a question ticket and changes no row.

The rows the port stands on, each asked of `cargo mock canon`:

| Row | Standing | What it gives the port |
|---|---|---|
| `the_crates_are_the_api_the_engine_the_build_crate_and_the_arvo_family` | ratified | the numerals, masks, sparse, graph, spectral and combinatorial primitives are arvo's |
| `arvo_is_a_standalone_no_std_primitive_family_shared_across_projects` | binding | arvo is a dependency, never a part of this repository |
| `the_stack_supplies_its_own_vocabulary_for_each_bare_primitive_in_arvo_notko_and_the_string_crate` | provisional, stamped | every position a bare number would take has an arvo type |
| `container_capacity_lives_at_type_level_and_a_consumer_needing_more_picks_the_bigger_type` | provisional, stamped | a capacity is a type the consumer picks |
| `fixed_capacity_is_one_generic_dimension_marker_implementing_the_capacity_trait_and_the_macro_rung_ladder_is_dropped` | ratified | one marker over a literal, one capacity trait |
| `weights_in_the_arvo_crates_and_data_facing_structs_use_arvo_fixed_point_newtypes_and_not_raw_integers` | binding | costs and weights are fixed point, which bears on the spectral numeral below |
| `no_bare_primitive_exists_anywhere_in_the_stack` | held | binds nothing until he backs it; cited by no job |

From op's record, the later file holding: what kaski's code does today is no requirement on the stack (file 42, the divisor call), so no row below is justified by a kaski site. Capability, sink and codec are notko's and the engine-only items are the engine's (file 57, call 3, landed as `252c5190`). The read lists build on notko's product with its carrier as typestate (file 57, call 2). The nightly pin is kept moving, and where a bump breaks something the order is investigate, migrate, bump (file 60). The public name of arvo is his to give at the port (file 40, "Calls of his"; arvo's plan, step 20).

Arvo's own tiers are its registry, then its designs, then its code. The removed crate tree at `1b2796e9` is a dead tier. Nothing below maps hilavitkutin onto it, and where an old name is listed it is listed as what hilavitkutin writes today and never as what arvo should ship.

## Where each side stands

Hilavitkutin names arvo on 664 lines of Rust under `mock/crates`, `mock/benches`, `mock/tools` and `mock/lints`, which is 133 pairs of crate and item. The lines are in `SITES.txt` beside this file, each with its path and line at `9918113f`, produced by `rg -n -U '\barvo(_[a-z_]+)?::'` over those four directories. The pairs with their counts are in `ITEMS.tsv`. The engine crate carries about two thirds of them. Beyond the paths, `USize(` is written 1063 times under `mock/crates` as a tuple construction, and the old `USize` is read through its field, so the bulk of the port is the door of one type.

Arvo `dev` has seven crates: `arvo-bits`, `arvo-capacity`, `arvo-digest`, `arvo-format`, `arvo-numeral`, `arvo-placement` and `arvo-strategy`. There is no facade crate named `arvo`, so every `arvo::` path changes crate whatever else happens. The contracts crate `arvo-contracts` is on `work/numeral-contracts` and exports `Sum`, `Difference`, `Product`, `Constant`, `Identity`, `Additive`, `Multiplicative`, `MonotoneSum`, `MonotoneCompletion`, `Reciprocal`, `SquareRoot`, `Order`, `Magnitude` and `Sign` (its `src/lib.rs` at `f0615270`). It carries no bit tier: no width, access, logic or sequence contract. Division and conversion are in flight on `work/arvo-numeral-doors`.

Three facts about current arvo shape every job, and each was read in arvo's source.

A platform-sized integer takes an adaptation. `arvo_numeral::USize<A, C = Abort>` and `ISize<A, C = Abort>` (`arvo-numeral/src/alias.rs`) are keyed on a type that names a rounding and an overflow behaviour, where hilavitkutin writes a bare `USize` at every site. No preset of `arvo-strategy` implements `NamesAdaptation` today, so `USize<Warm>` and `Uint<4, Hot>` cannot be written yet. An explicit `Adapt<R, O>` can. The host value goes in through `from_host` and out through `host`, both `const fn`, and there is no public field.

A capacity is a type and never a value. `arvo_capacity::Capacity` is sealed with `Dim<const N: usize>` as its one implementor, projects `Array<T>`, and `count::<C, A, K>()` reads the count in a constant as a `USize` (`arvo-capacity/src/capacity.rs`, `count.rs`). There is no `Cap` value, no `cap`, no `cap_size`, and no const arm of the trait methods.

A bit container has no strategy parameter and no arithmetic. `arvo_bits::Bits<const N: u32>` has `masked`, `raw`, `cast`, `xor` and `halves`, and `Bytes` beside it. Hilavitkutin writes `Bits<N, Hot>`, `Bits::from_raw`, `Bit` and a `bitfield!` macro.

Both repositories pin `nightly-2026-05-28` today (`rust-toolchain.toml` of each). Arvo `dev` is green on `nightly-2026-11-18` after a re-bless of fixtures (the nightly-bump report, the arvo section), so arvo's source builds on both. That is what lets the port move in jobs: a hilavitkutin crate can take current arvo on the old nightly while its neighbours still take the old revision, and the toolchain moves once, at the end, when the last line of the old revision leaves `mock/Cargo.toml`. Whether cargo resolves the two revisions side by side in one workspace is the first job's first sketch, and the plan says below what happens if it does not.

## The map, crate by crate

Each table gives what the crate writes today with one site, the count of sites from `ITEMS.tsv`, and what arvo offers. "Nothing" means no crate of arvo `dev` or of the two branches carries it, and the row it needs is in the next section under the letter given.

### `hilavitkutin-linking`, `hilavitkutin-extensions`, `hilavitkutin-extensions-macros`

| Written today | Site | Count | Current arvo |
|---|---|---|---|
| `arvo::USize` | `hilavitkutin-linking/src/error.rs:3` | 3 | `arvo_numeral::USize<A>`, through `from_host` and `host` |
| `arvo::ISize`, in a doc comment only | `hilavitkutin-linking/src/error.rs:8` | 1 | `arvo_numeral::ISize<A>`, if the round keeps the sentence |
| the old revision by `rev` | `hilavitkutin-extensions-macros/tests/lto_smoke_fixture/Cargo.toml:35` | 1 | the same move, in the fixture's own manifest |

`hilavitkutin-linking` declares `arvo-bits` and names no path of it. `hilavitkutin-extensions` declares `arvo` and names no path of it. Both lines go. This island depends on no other hilavitkutin crate (the forbidden-imports rules of both), so its numerals cross no border.

### `hilavitkutin-sym` and `hilavitkutin-str`

| Written today | Site | Count | Current arvo |
|---|---|---|---|
| `arvo::bitfield` | `hilavitkutin-sym/src/handle.rs:13`, used at `:19` and `:73` | 4 | Nothing. Row A |
| `arvo_bits::{Bit, Bits, Hot}`, `Bits::from_raw` | `hilavitkutin-sym/src/handle.rs:14`, `kind.rs:15`, `shape.rs:19` | 22 in sym, 10 in str | `arvo_bits::Bits<N>` with `masked` and `raw` for the container. No `Bit`, no strategy parameter. Row A for the single bit |
| `arvo::strategy::{BitsContainerFor, Unsigned}` | `hilavitkutin-sym/src/shape.rs:17`, used at `:185` | 2 | Nothing for the container projection, and none is needed: `Bits<N>` has one container. `Sign` in the contracts for the sign |
| `arvo::strategy::{Additive, Identity}` | `hilavitkutin-sym/src/shape.rs:17` | 2 | `arvo_numeral::{Additive, Identity}` on `dev`; the contracts crate on its branch |
| `arvo::traits::FromConstant` | `hilavitkutin-sym/src/generator.rs:5` | 1 | `arvo_contracts::Constant`, on the branch |
| `arvo::{USize, Uint}`, `Uint::<4, Hot>::from_raw` | `hilavitkutin-sym/src/shape.rs:18`, `shape_tests.rs:167` | 20 | `arvo_numeral::{USize, Uint}`, each under an adaptation |
| `arvo_refit::Narrow`, `arvo_bits::BitsRefitCtor` | `hilavitkutin-str/src/interner.rs:5`, `entry.rs:3` | 3 | `Bits::<N>::cast::<M>` between containers; `convert` between signatures is on `work/arvo-numeral-doors` |
| `arvo::Bool` | `hilavitkutin-str/src/handle.rs:9` | 2 | `arvo_numeral::Bool` (`arvo_format::Bool`): `of`, `get`, `TRUE`, `FALSE`, `and`; no `From<bool>` |

`hilavitkutin-str/src/hash.rs:14` holds a `const_fnv1a` of its own. Arvo ships `arvo_digest::Fnv1a` on `dev`. The port replaces the local body with arvo's, which the workspace rule on one owner per concept asks for, since arvo's `Fnv1a::of` is a `const fn` over a byte run (`arvo-digest/src/fnv1a.rs:47`). Whether it gives the same 64-bit value the local one gives is job 2's first test, and a difference is row D.

### `hilavitkutin-persistence`

| Written today | Site | Count | Current arvo |
|---|---|---|---|
| `arvo_hash::ContentHash`, `ContentHash::from_raw` | `hilavitkutin-persistence/src/manifest.rs:11`, `:39`; `archive_str.rs:15`, `:26` | 5 | `arvo_digest::{Digest, Output, Fnv1a, Xxh3}` at a width the type names. The alias is hilavitkutin's to declare over it |
| `arvo_refit::Narrow` | `hilavitkutin-persistence/src/archive_str.rs:16` | 1 | `Bits::cast`, one masked cast, which is what the design prices it at |
| `arvo_bits::Bits` | one site in `SITES.txt` | 1 | `arvo_bits::Bits<N>` |
| `arvo::USize`, `arvo::Cap`, `Identity<Additive>`, `Bool` | `manifest.rs:9-10`, `primitives.rs:14`, `sieve.rs:12` | 15 | as in the api table below |

The design says the content hash is 28 bits in the runtime api and the old alias was 64 (the topic of 18 September, the row for `ContentHash`). Arvo's row `obligation::a_content_hash` records in its gap that the width and the collision bound are unstated. The port keeps the width the design names and files the population question as a ticket; it decides no width.

### `hilavitkutin-api`, `hilavitkutin-api-macros`, `hilavitkutin-kit`

| Written today | Site | Count | Current arvo |
|---|---|---|---|
| `arvo::USize` | `hilavitkutin-api/src/member.rs:11`, fields at `:194-195` | 29, and 2 emitted by the macro | `arvo_numeral::USize<A>` |
| `::arvo::USize(0 ...)` emitted by the derive | `hilavitkutin-api-macros/src/lib.rs:110` | 1 | the same type through `from_host`, in the emitted path |
| `arvo::Cap` at a const generic position | `hilavitkutin-api/src/member.rs:98` (`Reach<C, const N: Cap>`), `:192` (`Window<'a, T, const N: Cap>`) | 10 | Nothing, and arvo's row says it asks for none. `const N: usize` under op's exception, see below |
| `arvo_tensor::{cap, cap_size}` | `hilavitkutin-api/src/member.rs:12`, bound at `:237` (`cap_size(K) <= cap_size(N)`), `footprint.rs:22` | 11 | Nothing. With `const N: usize` the bound reads `K <= N` and both functions go |
| `arvo::{UFixed, fbits, ibits}`, `arvo::strategy::Hot` | `hilavitkutin-api/src/hint.rs:13`, aliases at `:18`, `:21`, `:24`; `platform.rs:18` (`Nanos`) | 13 | `arvo_numeral::Uint<BITS, A>` for the integer aliases, `UFix<BITS, FRAC, A>` for a fraction; `Hot` as the parameter waits on arvo's step 4 |
| `arvo::strategy::{Additive, Identity, Multiplicative}` | `hilavitkutin-api/src/access.rs:11`, `:38`; `builder.rs:8`, `:40`; `run_cfg.rs:21`, `:152` | 11 | `arvo_numeral::{Additive, Identity, Multiplicative}` on `dev` |
| `arvo::Bool` | `hilavitkutin-api/src/adapt.rs:31`, `platform.rs:11` | 6 | `arvo_numeral::Bool` |

`hilavitkutin-api` declares `arvo-bitmask` and names no path of it; the line goes. `arvo::Norm` at `hilavitkutin-api/src/adapt.rs:134` is prose inside a string.

The const generic positions are the one place this plan reads op's words for the vehicle. Arvo's row `obligation::a_primitive_for_every_position_a_bare_number_would_take` carries his exception: "No bare usize other than in const generics for smoother and more ergonomic api, and even there, only when truly painful otherwise." A parameter of an arvo type at a const generic position needs a feature line in whoever declares it, and arvo's capacity row asks for none. So the api's value-position counts become `const N: usize`, or where a count sizes an array from generic code, a `C: Capacity` type parameter. Which sites take which is the design round's, with the rule that a count compared with another count stays a const generic and a count that sizes storage becomes a capacity.

### `hilavitkutin-providers`, `hilavitkutin-conformance`, `hilavitkutin-frozen-authors`

| Written today | Site | Count | Current arvo |
|---|---|---|---|
| `arvo::{Hot, UFixed, fbits, ibits}` | `hilavitkutin-providers/src/adapt_ema.rs:30`, aliases at `:41` (`Sample`), `:48` (`BlendFactor`) | 7 | `Uint<64, A>` and `UFix<16, -16, A>`; the product of a sample and a blend factor needs the fixed-point product, which is on arvo `dev` since step 3 |
| `arvo_tensor::{Capacity, Dim}` | one site each in `SITES.txt` | 2 | `arvo_capacity::{Capacity, Dim}` |
| `arvo::Signed` | `hilavitkutin-providers/examples/particle_collision.rs:19`, `examples/audio_mix.rs:18` | 2 | `arvo_contracts::Sign`, on the branch |
| `arvo_hash::{Fnv1a, Hasher}`, `arvo::{Bits, Hot}` | `hilavitkutin-conformance/src/digest.rs:7-8` | 7 | `arvo_digest::{Digest, Fnv1a}`, `arvo_bits::Bits<N>` |
| `arvo::{USize, Bool, Cap, Uint}`, `arvo_tensor::{cap, cap_size}` | `hilavitkutin-conformance/src/oracle/walk.rs:8`; `hilavitkutin-frozen-authors/src/values.rs:4-5`, `:12`, `:14` | 44 | as in the api table |

`hilavitkutin-providers` declares `arvo-bits` and names no path of it; the line goes. Every file of `hilavitkutin-frozen-authors` outside `tests/` is under `LEDGER.toml`, so each file the port touches there takes a line in `BREAKS.toml` saying the numeric vocabulary moved, and the hand model keeps computing the same values.

### `hilavitkutin`, the engine

| Written today | Site | Count | Current arvo |
|---|---|---|---|
| `arvo::USize` | throughout; `plan/steps/mod.rs:46` is one | 257 | `arvo_numeral::USize<A>` |
| `arvo::strategy::{Identity, Additive, Multiplicative}`, `arvo::{Identity, Additive}` | `plan/dims.rs:14` is one | 136 | `arvo_numeral::{Identity, Additive, Multiplicative}` |
| `arvo::Bool`, `Bool::TRUE`, `Bool::FALSE` | throughout | 56 | `arvo_numeral::Bool` |
| `arvo_tensor::{Capacity, Dim}` | `dispatch/core_dispatch.rs:11`, `dispatch/order.rs:304` | 69 | `arvo_capacity::{Capacity, Dim}` |
| `arvo_tensor::ConstCapacity` | `dispatch/trunk_gate.rs:69` (`CU: Capacity + const ConstCapacity`) | 12 | `arvo_capacity::count::<C, A, K>()`, a `const fn` over a plain `Capacity` bound |
| `arvo_tensor::{cap, cap_size}`, `arvo::Cap` | `SITES.txt` | 39 | Nothing, as in the api |
| `arvo::{Bits, Hot, Unsigned, Uint}` | `plan/dims.rs:14`, `thread/mod.rs:108`, `dispatch/codegen.rs:23` | 43 | `arvo_bits::Bits<N>`, `arvo_numeral::Uint<BITS, A>`, `Sign` |
| `arvo::traits::{FromConstant, TotalOrd}` | `dispatch/codegen.rs:22`, `plan/laplacian.rs:23` | 3 | `arvo_contracts::{Constant, Order}`, on the branch |
| `arvo::strategy::{Warm, Cold}` | `SITES.txt` | 3 | `arvo_strategy::presets::{Warm, Cold}`, as names; as parameters they wait on step 4 |
| `arvo_bitmask::{BitAccess, BitLogic, BitSequence, Mask, Mask64, Mask256}` | `scheduler/dirty.rs:11`, `scheduler/run.rs:15`, `dispatch/core_mask.rs:22`, `plan/dims.rs:15` | 28 | Nothing. `obligation::set_operations_over_a_fixed_size_bit_set`, arvo's step 10 |
| `arvo_bitmask::{BitMatrix, NodeId}` | `plan/steps/step3_waists.rs:7`, `plan/graph.rs:27` | 10 | Nothing, and no row asks for either. Row B |
| `arvo_graph::{waist_detect, waist_detect_const}` | `plan/steps/step3_waists.rs:8`, `plan/grouping.rs:28` | 5 | Nothing. `obligation::ordering_a_directed_acyclic_graph`, step 12 |
| `arvo_sparse::{Csr, CsrBidirectional}` | `plan/graph.rs:28` | 3 | Nothing, and arvo's row says the matrix is the consumer's. These two types come home to the engine |
| `arvo_sparse::{SparseAdjacency, BidirectionalSparseAdjacency, rcm_reorder_via, block_diagonal_via}` | `plan/laplacian.rs:26`, `plan/steps/step4_5_rcm_block.rs:9` | 6 | Nothing. `obligation::a_sparse_adjacency_a_plan_can_be_built_on`, step 13 |
| `arvo_spectral::{LinearOperator, k_way_partition}` | `plan/laplacian.rs:27`, `plan/steps/step6_spectral.rs:7` | 2 | Nothing. `obligation::a_spectral_partition_of_a_dependency_graph`, step 14 |
| `arvo::FastFloat<f32>` | `plan/steps/mod.rs:91` (`SpectralFloat`), `plan/mod.rs:485` | 2 | Nothing. Row C |
| `#![feature(generic_const_exprs)]` | `hilavitkutin/src/lib.rs:22` | 1 | goes with `cap_size` in array-length position; the gate names its removal |

The engine declares `arvo-comb` and names no path of it; the line goes. No source file of hilavitkutin names an array, a matrix or any other container of the old tensor crate: the five names it takes from there are `Capacity`, `ConstCapacity`, `Dim`, `cap` and `cap_size` (counted over `mock/crates` and `mock/benches`). So the tensor containers that file 40 puts on the port's path ("Dependences that were unwritten", item 1) are not on it. Nor is arvo's step 15, the cost programs, by what the code names; the engine's grouping carries its own.

### The benches

`mock/benches` names arvo on 37 lines: `USize`, `Bool`, `arvo_tensor::Dim`, one `waist_detect`, one `BitAccess`, and `arvo::ifixed::IFixed` with `ibits`, `FBits` and `strategy::Precise` in one arm. They move with the engine job that moves the same family. File 40, lane H, already says a bench of the model run before the port is marked as rewritten at it. `mock/benches/support/build-call` and `build-fixture` are held by `build-bench-rerun` and are touched by no job here. The 86 sketch manifests under `mock/research/sketches/` that name arvo are frozen evidence and are left as they are.

## The rows arvo still owes, in hilavitkutin's terms

These are proposals for arvo's registry, each to be put by an arvo seat through arvo's own flow and run by op as it is proposed (the stack plan, "What waits on the maintainer"). None is written into any registry by this plan. Each states the need and never the mechanism, as arvo's `obligation` namespace asks.

Row A, a packed word of named exact-width fields. Hilavitkutin's identity handle is one 32-bit word holding an origin, a counter and flag bits, each read and replaced by name, with the layout fixed when the consumer builds and the single-bit fields typed as one bit and not as a number (`hilavitkutin-sym/src/handle.rs:19`, `:73`; `shape.rs:185`). The need: a word of a declared total width divided into named fields of declared widths, a field read and written in a constant, a one-bit field with its own type, the widths summing to the word checked at build, and the whole convertible to and from the exact-width container at no cost. `obligation::an_exact_width_container_a_consumer_can_alias_and_pin` is the nearest and asks for the container only.

Row B, a square bit matrix and a node index. Phase boundaries are found over a reachability relation held as one bit per ordered pair of units (`hilavitkutin/src/plan/steps/step3_waists.rs:7`), and the graph, sparse and spectral routines all take the same index of a unit in the plan (`NodeId`, ten sites, `plan/graph.rs:27` among them). The need: a bit relation over a count of nodes fixed at build, with row union and a test of one pair each lowering as the set row asks, and one index type that every graph-shaped routine of arvo takes, so a consumer converts its own id once. `obligation::set_operations_over_a_fixed_size_bit_set` names sets and not relations, and no row names the index.

Row C, the numeral the spectral split computes in. The engine holds its partition vectors as `FastFloat<f32>` (`hilavitkutin/src/plan/steps/mod.rs:91`). Arvo's own row for the split, `obligation::a_spectral_partition_of_a_dependency_graph`, asks that it be "reproducible bit for bit at a fixed iteration count the consumer names", and its float row records that a float regime changed by a build flag gives one type name two meanings. A fast float is the numeral least able to hold the first of these. The need, in hilavitkutin's terms: a value the power iteration can run in that gives the same partition on every target and every build of one plan, at a width the consumer names. Whether that is a strict float of step 17 or a fixed-point value of the steps already landed is arvo's to answer in the round of step 14, and file 40 lists it as unchecked ("whether the old spectral crate can go to fixed point instead of waiting on floats"). If fixed point holds, the port does not wait on step 17 at all.

Row D, a digest computed in a constant. Hilavitkutin hashes a string's bytes at build to key its static string table (`hilavitkutin-str/src/hash.rs:14`, `const fn`). The need: the digest of a byte run available in a constant, with the same value as the run-time path. Arvo's `Fnv1a::of` is a `const fn` already, so this row is proposed only if job 2 finds the value differs from the one persisted tables were keyed with, or the width hilavitkutin needs has no carrier.

Row E, a preset that names an adaptation. Hilavitkutin writes a strategy at 40 sites and a bare `USize` at several hundred, and current arvo asks for an adaptation at each. The need: a consumer names its index and count type once per crate with a strategy and nothing else, and gets the rounding and overflow that strategy stands for. This is arvo's step 4 (`work/overflow-default-and-override`) with the cells of step 2, and arvo's ack row `a_value_alias_is_keyed_on_its_strategy_and_defaults_to_warm` already points there. It is listed because the port cannot write a single numeral without it or without the explicit form the first job uses.

Existing rows the port needs met, with the step that meets them: `the_surface_expressible_as_contracts_before_anything_implements_it` (step 5, on its branch), `set_operations_over_a_fixed_size_bit_set` and the bit tier's contracts (step 10), `ordering_a_directed_acyclic_graph` (step 12), `a_sparse_adjacency_a_plan_can_be_built_on` (step 13), `a_spectral_partition_of_a_dependency_graph` (step 14), and the conversion between signatures of step 7. Already met for what the port takes: `a_count_fixed_at_build_that_sizes_arrays_from_generic_code` (`arvo-capacity` on `dev`), `a_content_hash` as far as a stable digest goes (`arvo-digest` on `dev`), and the platform-sized integer as a type (`arvo-numeral` on `dev`).

So the arvo steps on the port's path are 4, 5, 7, 10, 12, 13 and 14, with 17 only if row C is answered by a float. That is shorter than file 40's list (9 to 15, 17 and the tensor containers): 9 and 11 are landed, 15 and the containers are named by no source file here.

## The jobs

Six jobs, each one helper, one branch and one design round where it changes a design or a declaration. The engine job is split by what arvo has, since half of it can run now and half cannot.

| Job | Branch | Overhauls | Adds to | Waits on | Seat |
|---|---|---|---|---|---|
| 1. The seam and the loader island | `work/port-seam-and-loader` | `hilavitkutin-linking` | `mock/Cargo.toml`, `hilavitkutin-extensions`, `hilavitkutin-extensions-macros` (one fixture manifest), `mock/tools` | nobody | sonnet high, after one opus read of its round |
| 2. Identity and strings | `work/port-identity-and-strings` | `hilavitkutin-sym`, `hilavitkutin-str` | `hilavitkutin-persistence` (the hash and refit sites) | job 1; row A delivered in arvo; `hila-consumer-surface` landed | opus medium |
| 3. The numeral vocabulary | `work/port-numeral-vocabulary` | `hilavitkutin-api`, `hilavitkutin`, `hilavitkutin-providers`, `hilavitkutin-conformance`, `hilavitkutin-frozen-authors` | `hilavitkutin-api-macros`, `hilavitkutin-kit`, `hilavitkutin-persistence`, `mock/benches` outside the two held support crates | job 1; arvo's step 5 landed; step 4 or the stamp of job 1; `hila-consumer-surface` landed; the shape agreed with loru-coord; hilavitkutin's job 5 landed or parked | fable low to shape the round, opus high to build, haiku sweeps per crate for the door of `USize` |
| 4. The bit sets | `work/port-bit-sets` | none; the mask sites of `hilavitkutin` | `mock/benches` | job 3; arvo's step 10 with row B | sonnet high |
| 5. The plan chain families | `work/port-plan-chain` | `hilavitkutin` under `src/plan` only | `mock/benches` | job 4; arvo's steps 12, 13 and 14; row C answered | opus high |
| 6. The pin and the toolchain | `work/port-pin-and-toolchain` | none | `mock/Cargo.toml`, both `rust-toolchain.toml`, the refusal fixtures | jobs 2 to 5; arvo's nightly bump landed | sonnet high |

Job 1 starts first and waits on nobody. Jobs 2 and 3 share no crate that either overhauls and can run side by side once their waits clear; job 3 adds only the sites of `hilavitkutin-persistence` that job 2 does not own (its `USize`, `Cap`, `Identity` and `Bool` lines), and the two briefs name the lines. Jobs 4 and 5 run one after the other, since both are inside the engine crate. Job 6 is last.

Against the live lanes. `build-pragmas` holds `hilavitkutin-build`, which names no arvo path and is touched by no job. `hila-consumer-surface` holds the api's host import and dump and the arena and interner in `hilavitkutin-providers` and `hilavitkutin-str`, so jobs 2 and 3 both wait for it to land; job 1 reaches none of its crates. `arvo-capacity` has landed. `arvo-numeral-doors` delivers the conversion job 2 leans on for `Narrow`, and where it has not landed job 2 uses `Bits::cast` alone, which the two sites allow. `notko-mockspace` changes notko's layout and codec, and job 3 builds against notko `dev` as hilavitkutin already does. The arvo contracts land from `work/numeral-contracts`, and job 3 pins the revision that carries them. loru builds on hilavitkutin's api and arvo from the start (the state file), so the api round of job 3 is agreed with loru-coord before its changelist locks. The unit's shape (questions `fd8aa7fa67fa` and `01af7eebc194`) changes `WorkUnit` in the same crate job 3 overhauls: the two rounds do not run at once, and whichever is ready second is cut from the first's landing.

### Job 1. The seam and the loader island

Drafted as its brief in the next section.

### Job 2. Identity and strings

The handle of `hilavitkutin-sym` moves from the old `bitfield!` onto what arvo delivers for row A, `Bits<N, Hot>` becomes `Bits<N>`, `from_raw` becomes `masked`, the `FieldWidth` impl at `shape.rs:185` loses its container projection, and the numerals take the adaptation job 1 settled. `hilavitkutin-str` follows it, and its local FNV-1a goes onto `arvo-digest` or row D is proposed. `hilavitkutin-persistence` gets its `ContentHash` as a local alias over arvo's digest output at the width its design names, with the narrowing as one `cast`. The round's first act is to check the handle's layout bit for bit against the old one with a test that fails on a moved field, since persisted ids outlive the port. Proof: `cargo mock test` green for the three crates, the old-name tool of job 1 at zero for `arvo_bits::`, `arvo_refit::`, `arvo_hash::` and `arvo::bitfield`, and the layout test shown red on a mutant.

It needs opus medium because the handle's layout is a wire fact with no compiler to catch a slip, and it waits on arvo for row A. If row A is refused in arvo, the handle's packing is a hilavitkutin design question for op and the job stops there with the refusal quoted.

### Job 3. The numeral vocabulary

This is the wide one: every `USize`, `Bool`, `Identity`, `Uint`, fixed-point alias and capacity in the api, the engine and the three crates that test them, in one branch, because the engine takes the api's types at every signature and a half-moved pair does not compile.

The round is shaped before it is built. It settles, in the api's design: the alias or aliases the stack's index and count take, declared once in the api for every crate above it; which of the api's counts stay `const N: usize` and which become `C: Capacity`; the hint aliases (`Urgency`, `Divisibility`, `Significance`, `Nanos`) over `Uint`; and the spelling the derive emits. In the engine's design it settles `PlanDims` over `arvo_capacity::Capacity`, the fifteen dimensions as `Dim<N>`, `ConstCapacity` bounds replaced by `count`, and every `cap_size` in array-length position replaced by a projection, which is what removes `#![feature(generic_const_exprs)]` from `hilavitkutin/src/lib.rs:22`. `arvo-capacity`'s methods are not `const`, so each engine structure built in a constant today is listed in the round with how it is built after, and a structure that cannot be is a finding sent to arvo as the const arm its design calls a follow-up.

The algorithm families stay on the old revision through this job. At each call into `arvo_bitmask`, `arvo_graph`, `arvo_sparse` and `arvo_spectral` the engine converts through the host integer, in one private module per family, so jobs 4 and 5 delete a module each and touch no caller. The 1063 `USize(` constructions and the field reads are a sweep per crate with a fixed rewrite, `USize(x)` to `USize::from_host(x)` and `.0` to `.host()`, checked by the build; the rest is judgement and is the opus seat's.

Proof: `cargo mock test` green; the conformance suite holds `run`, `run_parallel` and `run_fused` (or the one entry, if job 5 of the engine path has landed) to the reference at the worker counts it runs today; the old-name tool at zero for `arvo::` and `arvo_tensor::`; `rg generic_const_exprs mock/crates` at zero; every file of `hilavitkutin-frozen-authors` it touched has its `BREAKS.toml` line; and the bench of both record-set shapes, where it has run, is marked as rewritten.

### Job 4. The bit sets

The scheduler's dirty masks, the core and trunk masks of dispatch and the plan's dimension masks move onto arvo's bit-set crate of step 10, and the private conversion module of job 3 for masks is deleted. The row asks that intersection, union and the emptiness test each lower to one instruction, so the job commits `cargo mock bench disasm` readings of the dirty walk before and after and writes a difference as a finding. Proof: suite green, old-name tool at zero for `arvo_bitmask::` outside `BitMatrix` and `NodeId` if row B lands later than the sets, and the two disassemblies committed.

### Job 5. The plan chain families

`Csr` and `CsrBidirectional` come into the engine under `src/plan`, as arvo's row says the matrix is the consumer's. Waist detection, the reordering, the block partition and the spectral split move onto arvo's crates of steps 12 to 14 over the contracts those crates declare, the Laplacian stays the engine's operator, and the partition's numeral is what row C settled. The plan a fixed registration produces is compared before and after, unit order, phase and trunk ids and fiber grouping, over the fixtures the plan tests already hold, and a difference is a finding with the two plans committed, never re-blessed in silence. Proof: suite green, the plan comparison, the old-name tool at zero for every old path, and the three remaining conversion modules gone.

It is opus high: numerics with no one to ask, and a schedule that must not move.

### Job 6. The pin and the toolchain

The ten lines of the old revision leave `mock/Cargo.toml`, the arvo crates are pinned at one revision of `dev`, both `rust-toolchain.toml` files take the workspace's nightly, the refusal fixtures are regenerated with `TRYBUILD=overwrite` and each changed message is read (the nightly-bump report found three in arvo that read worse), and the two `CARGO_TARGET_DIR` sites the report lists (`mock/benches/asm_gate_fixtures/tests/asm_gate.rs:94`, `mock/benches/frame_wide/tests/negative_control.rs:53`) get the `build-dir` beside them. It closes the trigger ticket and tells `build-pragmas` the pin moved. Proof: `cargo mock` and `cargo mock test` green on the new pin, `rg 1b2796e mock --glob '!research/**'` at zero.

If the side-by-side sketch of job 1 fails, jobs 2 to 5 cannot land one at a time. They then stack on one long branch `work/port-hilavitkutin` in the same order, each still its own round, and the branch lands once with job 6. The order and the waits do not change; what changes is that nothing of the port reaches `dev` before arvo's step 14.

## The brief of job 1

```
# job port-seam-and-loader

Your task is the first job of hilavitkutin's port onto current arvo. The plan is
hilavitkutin `mock/research/202611200345_arvo-port-plan/README.md`; read it whole
before anything else. You do three things and they land as one branch.

1. A sketch, `mock/research/sketches/<stamp>_two-arvo-revisions-side-by-side/`:
   one cargo workspace under `nightly-2026-05-28` taking the old arvo at
   `1b2796e9` and arvo `dev` at a pinned revision, under renamed dependency keys,
   with one crate using `arvo::USize` of the old and one using
   `arvo_numeral::USize` of the new, both built and tested in one `cargo test`.
   WORKS: it resolves, builds and runs. FAILS: cargo or the compiler refuses,
   with its output. INCONCLUSIVE: it builds only with a patch section. Use the
   `sketching` skill. If it FAILS, stop after step 2 and post the outcome: the
   plan's fallback then holds and step 3 waits.

2. The dead dependency lines go, each shown dead by a build without it:
   `arvo-comb` in `hilavitkutin`, `arvo-bitmask` in `hilavitkutin-api`,
   `arvo-bits` in `hilavitkutin-linking` and in `hilavitkutin-providers`, `arvo`
   in `hilavitkutin-extensions`. Then a tool `mock/tools/old-arvo-names`, asked
   as `cargo mock old-arvo-names`, that prints the count of each old arvo path
   per crate from the tracked `.rs` files under `mock/crates` and `mock/benches`
   and exits 0. It is a tool and gates nothing. Copy the shape of an existing
   tool in `mock/tools`; arvo's `mock/research/202611130950_canon-double-check/tools/`
   holds the counting script it replaces. Its test plants one old path in a
   fixture and asserts the count moves.

3. `hilavitkutin-linking` moves onto `arvo-numeral`, through a design round:
   its error type carries the OS code as `arvo_numeral::USize`, entered by
   `from_host` and read by `host` (three imports: `src/error.rs:3`,
   `src/backend/unix.rs:5`, `src/backend/windows.rs:4`). `ISize` is named only
   in the doc comment at `src/error.rs:8`; the round keeps or drops that
   sentence. The crate does no arithmetic on the code. Current arvo asks for an
   adaptation on the type
   and no strategy preset names one yet, so the round declares one alias in the
   crate for the code's type, with an explicit `Adapt<R, O>`, and says in the
   design that it becomes the stack's default strategy when arvo's step 4 lands.
   Which rounding and overflow the alias names is not yours to settle alone:
   bring the round's choice to stack-coord for a stamp before the doc changelist
   locks. The fixture manifest
   `hilavitkutin-extensions-macros/tests/lto_smoke_fixture/Cargo.toml:35` moves
   to the same revision if the fixture names an arvo path, and loses the line if
   it does not.

You overhaul `hilavitkutin-linking`. You add to `mock/Cargo.toml`,
`hilavitkutin-extensions`, `hilavitkutin-providers`, `hilavitkutin-api` and the
engine's manifest only the removed lines of step 2. You do not touch
`hilavitkutin-build`, `hilavitkutin-str`, `hilavitkutin-sym`, the api's source,
or `mock/benches/support`.

Gates before the task: `cargo mock canon subject extensions` and
`cargo mock canon find linking` in hilavitkutin; `cargo mock query
obligation::a_platform_sized_unsigned_integer_at_an_api_position` in arvo. The
removed arvo tree is a dead tier: read arvo `dev` for what arvo is. Canon rows
you derive from: `the_linking_crate_is_no_std_depends_on_libc_or_windows_sys_and_on_no_other_hilavitkutin_crate`,
`the_extensions_crate_depends_on_linking_notko_and_arvo_only_and_on_no_engine_api_build_ctx_persistence_or_str_crate`,
`the_stack_supplies_its_own_vocabulary_for_each_bare_primitive_in_arvo_notko_and_the_string_crate`.
Run `cargo mock canon check` on the three first.

File two question tickets for op with `ask`, holder op, and wait on neither: that
the open question row
`whether_hilavitkutin_grows_its_own_numeral_mask_graph_sparse_spectral_combinatorial_and_hash_code_or_drops_the_design_steps_those_arvo_crates_serve`
be closed with his gate of 15 November (stack record file 41, "The plan and its
gate"), options close it so, keep it open, or answer it otherwise; and arvo's
public name, which arvo's plan brings him at the port, options keep the crates
as they are named, a facade crate named `arvo`, or his own.

Proof: the sketch's FINDINGS with its outcome; `cargo mock` and `cargo mock test`
green; `cargo mock old-arvo-names` showing `hilavitkutin-linking` at zero and
the engine unchanged; the tool's test red on its mutant. The report goes to
`mock/research/<stamp>_port-seam-and-loader/REPORT.md`. Land with `land-own`.
Result op can see: the linking crate on current arvo on `dev`, and the count of
old arvo names per crate as one command.
```

The brief goes out through `helper-prompt` with its `overhauls` line as `hilavitkutin-linking`, and stack-coord adds the ticket id and the memory sets.

## What waits on the maintainer

The open question row on growing or dropping, closed with his later gate, as job 1 files it. Arvo's public name at the port, as job 1 files it. Rows A to E, each as an arvo seat proposes it, and of these row C is the one with a real choice in it: a float or a fixed-point value for the spectral split. The adaptation hilavitkutin's indices and counts carry, which job 1 stamps for one crate and job 3 settles for the stack, unless arvo's step 4 has given the default by then. The population the content hash has to hold without a collision, which arvo's row records as unstated and job 2 tickets.

What waits on arvo's panel and not on him: what `Warm` names for overflow and for rounding, which is what turns row E from an explicit adaptation into a strategy name.

## What was not checked

Whether cargo takes two revisions of arvo in one workspace under renamed keys, and whether arvo `dev` at a pinned revision builds beside the old tree on `nightly-2026-05-28`: both are job 1's sketch. Whether `arvo_digest::Fnv1a` gives the value hilavitkutin's own FNV-1a gives. Whether every engine structure built in a constant today can be built through `Capacity::filled` and `filled_with`, which are not `const`. Whether the two fixed-point products `hilavitkutin-providers/src/adapt_ema.rs` computes are the ones arvo's step 3 gives. The counts are of paths and constructions at `9918113f` and move with every landing; a job recounts with the tool of job 1. `SITES.txt` catches a path written with `arvo` in it and misses a name used bare after an import, which is why the counts in the tables are of imports and qualified paths and not of uses. The standing of the lanes is the brief's and the state file's. The diff of `work/numeral-contracts` was read at its `lib.rs` and not line by line, and `work/arvo-numeral-doors` by its log only. Nothing was built for this plan.
