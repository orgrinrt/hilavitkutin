# Src changelist: the loop core contracts

**Date:** 2026-11-13
**Phase:** SRC
**Round:** 202611130150-x0pf (loop-core-contracts, branch `work/loop-core`)
**Doc changelist:** `202611130153-yegv_changelist.doc.lock.md`
**Replaces:** `202611130207-v2ky_changelist.src.deprecated.md`

Builds the five crates the doc changelist designs, the three repo lints it names, the cross-target tool, and the legacy mark on `hilavitkutin-api`'s crate docs. Nothing in a legacy crate changes behaviour, no lint severity moves and no exemption is added. The source this changelist claims is already on the branch from the deprecated changelist; what changed since is said in the comparison at the end.

## CHANGE: the workspace takes the five crates
Files: `mock/Cargo.toml`, `mock/Cargo.lock`.
The five crates and `tools/cross-target` join the members. `notko-hlist` (from the notko repository at notko's own source, with a `version` so `cargo deny` does not read it as a wildcard), `trybuild` (pinned `=1.0.122`) and the five crates join the workspace dependencies.
Verification: `cargo mock test` builds each of them; `cargo deny` passes in `cargo mock --lint-only`.

## CHANGE: `hilavitkutin-contracts`
Files: `mock/crates/hilavitkutin-contracts/{Cargo.toml,src/**,tests/surface.rs}`.
Modules `num` (`USize`, `Count`, `Index`, `Bool`, `Int`, `Lane`, `Word`, `Coefficient`), `domain`, `list` (`Values` over the shared hlist), `kernel` (`KernelId`, `ValueKernel`, `Kernels`), `store` (`Read`, `Write`, `Store`, `Visitor`, `Visit`), `bind` (`Binding`, `Role`, `BindRefusal`, `bind`, `Bound`, `Unit`) and `digest`, as the crate design states. The seal behind `Cell` is crate-private, as the legacy api's seals are. The role count a read member takes is a `Count`. `Values` and the `Int` proxy carry a `FIXME` naming tickets `33d72d28dd71` and `69b1c00469a6`; the 128-bit kernel mask carries one naming `cc007451af6f`.
Verification: `tests/surface.rs` reaches every numeric door in a const context and at run time, and holds the exact term bound of `Int<8>`.

## CHANGE: `hilavitkutin-frozen-implementors`
Files: `mock/crates/hilavitkutin-frozen-implementors/{Cargo.toml,LEDGER.toml,BREAKS.toml,src/**,tests/ledger.rs}`.
`Columns` and `Records`, `Forward` and `Permuted`, written against `hilavitkutin-contracts` alone, with no `unsafe` and no comparison of formats beyond what `Store::format` reports. `tests/ledger.rs` includes the laws' shared ledger tests.
Verification: `tests/ledger.rs` holds; the frozen lints report nothing.

## CHANGE: `hilavitkutin-frozen-authors`
Files: `mock/crates/hilavitkutin-frozen-authors/{Cargo.toml,LEDGER.toml,BREAKS.toml,src/**,tests/ledger.rs}`.
`Plane` and its offsets, `Heat`, `Rate` and `Diffuse` (author 1), and the plain model `model::diffuse`. `tests/ledger.rs` includes the laws' shared ledger tests.
Verification: `tests/ledger.rs` holds; the frozen lints report nothing.

## CHANGE: `hilavitkutin-reference`
Files: `mock/crates/hilavitkutin-reference/{Cargo.toml,src/lib.rs,tests/**}`.
`step`, `swap`, `digest` and `run`, with `Refusal` and `Run`, and the two private `UnitFn`s the design names, `Stepper` and `Digester`.
Verification: `tests/laws.rs` holds B1 to B3 over both stores and a probe store, by forward and permuted visits; `tests/mutants.rs` asserts each mutant's failing verdict; `tests/bind.rs` has one test per `BindRefusal` and asserts the store unchanged; `tests/ui.rs` runs the eight compile-fail fixtures, each beside its compile-pass twin, with the committed diagnostics.

## CHANGE: `hilavitkutin-laws`
Files: `mock/crates/hilavitkutin-laws/{Cargo.toml,src/{lib.rs,ledger.rs},tests/**}`.
`model`, `morsel_split_and_order` and `frame_isolation` returning a `Verdict` that names its `Law`, and the ledger check. The two ledger tests a frozen crate runs are `tests/support/ledger.rs`, outside `src/`, which no longer holds a macro naming `std`. The ledger's line numbers and counts are `Index` and `Count`, its flags `Bool`, and the edits of a file come back as an `Outcome`.
Verification: `tests/verdicts.rs` and `tests/ledger.rs` assert both verdicts of every law and every ledger finding.

## CHANGE: the three frozen lints
Files: `mock/lints/frozen_no_feature_gate.rs`, `mock/lints/frozen_no_recursion_limit.rs`, `mock/lints/frozen_no_foreign_turbofish.rs`, each with its tests under `mock/lints/<file stem>/tests.rs`; `mock/lints/frozen/scan.rs`, what the three share; `mock/lints/scan/code.rs`, the line reader `engine_spawn_through_gate.rs` held, moved out so the four lints read code the same way; `mock/lints/engine_spawn_through_gate.rs`, which now includes it.
Each frozen lint reads the `src/` of the two frozen crates and reports at error: an inner `feature` attribute, bare or under `cfg_attr`; a `recursion_limit` or `type_length_limit` attribute; a `::<` whose item, the segment right before it, the file does not declare.
Verification: each lint's tests hold a case it reports and one it lets pass; `engine_spawn_through_gate`'s tests pass unchanged; `cargo mock check` reports nothing in the frozen crates.

## CHANGE: the cross-target tool
Files: `mock/tools/cross-target/**`; `mock/research/sketches/202611130039_wasm-digest-under-node/` is left in place as the evidence the tool grew from.
The WD1 sketch becomes a project tool. `front/` is a workspace of its own: a `no_std` loop crate drives author 1 over `Columns` and `Records`, forward and permuted, through `hilavitkutin_reference::run`, in four cases, the last lending too little digest storage so the refusal is compared as well. The native host prints one line per digest and node prints the same from the wasm host. The tool builds both into the directory `cargo metadata` names, compares the lines with each other and with `front/digests.txt`, and with `--control` builds the loop's `control` feature, which folds the pointer width into each case's first digest, and requires the two to differ.
Verification: `cargo mock cross-target` exits zero; `cargo mock cross-target --control` exits zero; the tool's tests hold each way the comparison holds and refuses.

## CHANGE: `hilavitkutin-api` is marked legacy in its crate docs
Files: `mock/crates/hilavitkutin-api/src/lib.rs`.
The crate doc gains the legacy paragraph the README carries. Ticket `083903437698` tracks the removal.
Verification: `cargo doc -p hilavitkutin-api` renders it.

## Tests

The suites named above are the whole of the new tests. No existing test changes in what it asserts.

## Out of scope

- Porting the engine onto the contracts, and removing `hilavitkutin-api`.
- The two inherited topics the doc changelist names, which stay filed.
- The host arithmetic of the frozen visits and record slots, which the contract numbers have no `mul` or `rem` for: ticket `596bb348cf63`.
- Two reds that are `dev`'s, found by `cargo mock test`: `hilavitkutin-persistence`'s `archive_str` does not link (ticket `5f3b8d4e6aec`), and `mock/benches` does not build against the harness (ticket `cabbe4550fc2`).

## Comparison to deprecated changelist

The deprecated changelist claimed the same source. It was deprecated because the build gate refused three types no design named (`Law`, `Stepper`, `Digester`), and because `src/ledger.rs` held a macro whose body named `std`, which `forbidden-imports` and `no-alloc` refuse in a `hilavitkutin*` crate. The doc changelist was reopened to name the three and to move the ledger tests out of `src/`.

### change: the workspace takes the five crates

Kept. Adds the tool as a member and a `version` on `notko-hlist`, which `cargo deny` asked for.

### change: `hilavitkutin-contracts`

Kept, with every module and type it named: `USize`, `Count`, `Index`, `Bool`, `Int`, `Lane`, `Word`, `Coefficient`, `Values`, `KernelId`, `ValueKernel`, `Kernels`, `Read`, `Write`, `Store`, `Visitor`, `Visit`, `Binding`, `Role`, `BindRefusal`, `Bound`, `Unit`, and the three `FIXME` markers with their tickets. Changed: the seal behind `Cell` is crate-private, which `single-source` asked for since the engine has a public seal of the same shape, and a member's role count is a `Count`.

### change: `hilavitkutin-frozen-implementors`

Kept: `Columns`, `Records`, `Forward`, `Permuted`, no `unsafe`, nothing compared beyond `Store::format`. Changed: `tests/ledger.rs` includes the shared tests instead of expanding a macro.

### change: `hilavitkutin-frozen-authors`

Kept: `Plane`, `Heat`, `Rate`, `Diffuse` and the model. Changed: `tests/ledger.rs` as above, and the term count `Diffuse` checks against its lane is a `USize`.

### change: `hilavitkutin-reference`

Kept: `step`, `swap`, `digest`, `run`, `Refusal`, `Run`, and every suite. Changed: nothing in the code; the design now names `Stepper` and `Digester`.

### change: `hilavitkutin-laws`

Kept: the three laws and `Verdict`, and the ledger check. Changed: the ledger tests left `src/` for `tests/support/ledger.rs`, the design names `Law`, and the ledger's numbers and flags are proxies.

### change: the three frozen lints

Kept: the three lints and what each refuses. Changed: the shared line reader is one file the engine lint includes as well, rather than living in the engine lint alone.

### change: the cross-target tool

Kept: the WD1 sketch stays in place and the tool runs the loop through `run`. Changed: the tool builds into the workspace's one build directory, which a hook requires, and gains the `--control` run.

### change: `hilavitkutin-api` is marked legacy in its crate docs

Kept unchanged.

### tests

Kept unchanged.

### out of scope

Kept, and adds the host arithmetic and the two reds that are `dev`'s.
