# Doc changelist: the loop core contracts

**Date:** 2026-11-13
**Phase:** DOC
**Round:** 202611130150-x0pf (loop-core-contracts, branch `work/loop-core`)
**Plan:** engine roadmap r7, step HL0a, sections 8 and 9
**Sketches:** `mock/research/sketches/202611122252_runner-generic-workunit/`, `mock/research/sketches/202611122242_frame-view-typestate/`, `mock/research/sketches/202611130039_wasm-digest-under-node/`

This changelist births five crates as designs, adds them to the root design, marks `hilavitkutin-api` as the legacy contract, and records, line by line, which statements of the existing designs do not carry into the new crates and what the new crates add beside them. Every superseding below holds for the five new crates only: the legacy crates keep their designs and their code until the port, so nothing is nuked. The repository declares no canon (`mockspace.toml` names no `canon_paths`), so the designs are the tier this round reasons from.

The cited lines were read at `af823b7`, the plan's reading, and re-verified at `0928042`, the head this round opens on; none has moved.

## CHANGE: five new crates, each with its README and design

**Files:**

- `mock/crates/hilavitkutin-contracts/README.md.tmpl`, `DESIGN.md.tmpl`
- `mock/crates/hilavitkutin-reference/README.md.tmpl`, `DESIGN.md.tmpl`
- `mock/crates/hilavitkutin-laws/README.md.tmpl`, `DESIGN.md.tmpl`
- `mock/crates/hilavitkutin-frozen-implementors/README.md.tmpl`, `DESIGN.md.tmpl`
- `mock/crates/hilavitkutin-frozen-authors/README.md.tmpl`, `DESIGN.md.tmpl`

`hilavitkutin-contracts` is what a kernel author and a store implementor write against, in seven modules: `list` (notko-hlist's list and the value projection `Values`), `num` (the proxies `Bool`, `USize`, `Index`, `Count`, the lane vocabulary `Lane`, the proxy lane `Int<BITS>` with its accumulator `Wide<BITS>`, and `Identity` with `Additive`), `domain` (`Domain`, `Offset`, `Column`, `Resource`, the cell formats and `Value`, and the members `Own`, `Near`, `Param`), `digest` (`Digest` and `Absorb`), `kernel` (`ValueKernel`, `KernelId`, `Kernels`), `bind` (`Row`, `Role`, `Coefficient`, `UnitId`, `bind`, `BindRefusal`, `Bound`, `Col`, `Unit`, `UnitFn`) and `store` (`Store`, `Read`, `Write`, `Visit`, `Visitor`). `hilavitkutin-reference` is the one-thread runner: `step`, `swap`, `digest`, and `run` over a checked table with `Run` and `Refusal`. `hilavitkutin-laws` holds `Law`, `Witness`, `Verdict`, the functions `model`, `morsel_split_and_order` and `frame_isolation`, and the ledger check. `hilavitkutin-frozen-implementors` holds `Columns` and `Records`, two stores of different layouts, and the visiting orders `Forward` and `Permuted`. `hilavitkutin-frozen-authors` holds author 1, `Diffuse` on the two-axis domain `Plane`, and its model.

**Verification:** each crate's design names its own types only, and the src changelist adds every one of them; `cargo mock check` reads the designs against source once the src changelist lands.

## CHANGE: the root design's crate table, dependency graph and vocabulary

**Files:** `mock/DESIGN.md.tmpl`

The crate layout table (lines 12 to 26) gains five rows. The dependency graph (lines 28 to 34) gains a paragraph: none of the five depends on `hilavitkutin` or `hilavitkutin-api`; the frozen crates depend on `hilavitkutin-contracts` alone; the laws crate depends on no runner; and `hilavitkutin-api` is the legacy contract until the engine is ported onto the contracts (ticket `083903437698`). A section on the contracts and the reference loop goes after the runtime section, carrying what the lines below add. The vocabulary line (line 114) gains binding, role, logged form, member and `Lane`.

## CHANGE: `hilavitkutin-api` marked legacy

**Files:** `mock/crates/hilavitkutin-api/README.md.tmpl`; the crate docs of `mock/crates/hilavitkutin-api/src/lib.rs` in the src changelist

Its README opens by saying it is the legacy contract, kept until the port and then removed, and names `hilavitkutin-contracts` as the contract new work is written against. Ticket `083903437698` owns the removal.

## CHANGE: the agent templates count eighteen crates

**Files:** `mock/agent/` templates naming the crate count or the crate table

They say twelve crates, a count already one short of the thirteen in the tree (`hilavitkutin-sym` was not counted); they say eighteen, gain a loop-core grouping, and gain the same dependency statements as the root design.

## CHANGE: `mockspace.toml`, forbidden imports and three lints

**Files:** `mockspace.toml`

Rows added under `[lints.forbidden-imports]`, at the table's existing severity: the frozen crates (`hilavitkutin-frozen-*`) may import no hilavitkutin crate but `hilavitkutin_contracts`; `hilavitkutin-laws` may import no runner (`hilavitkutin_reference`) and no engine; and none of the five imports `hilavitkutin::*` or `hilavitkutin_api::*`. Three repo lints under `mock/lints/`, each with a row at error on commit, build and push, reading the frozen crates' `src/`: `frozen-no-feature-gate` (no `#![feature` line), `frozen-no-recursion-limit` (no `recursion_limit` attribute), and `frozen-no-foreign-turbofish` (no `::<` on an item the file did not declare). No severity moves and no exemption is added.

## CHANGE: revised after the first src changelist

**Files:** `mock/crates/hilavitkutin-laws/DESIGN.md.tmpl`, `mock/crates/hilavitkutin-reference/DESIGN.md.tmpl`

The first src changelist, `202611130207-v2ky_changelist.src.deprecated.md`, built three types these designs did not name, and a ledger macro whose body names `std` inside `src/`, which the `forbidden-imports` and `no-alloc` lints refuse in a `hilavitkutin*` crate whatever the macro expands into. The laws design now names `Law`, and puts the two ledger tests in `tests/support/ledger.rs`, which each frozen crate's `tests/ledger.rs` includes, so `src/` stays `no_std` and the tests are still written once. The reference design names `Stepper` and `Digester`, the runner's two private `UnitFn`s. Nothing else in either design moves.

## Supersedes, for the five new crates

1. `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:359-360`, "No NAME const. WU identity is the type itself. Naming is a consumer concern". The new crates give a kernel a declared `KernelId` and a unit a declared `UnitId`. A kernel is bound by data, so one kernel type is many units, and an order key needs an identity that is the same across builds, which a type is not. The code says the same as the design at `hilavitkutin-api/src/work_unit.rs:5`, and the legacy crate has its own `UnitId` with a public field at `hilavitkutin-api/src/dispatch_codegen.rs:191`; the new one shares the name and nothing else.
2. `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:366-368`, "Write-write ambiguity resolved by scheduling hint priority across three dimensions. Tie-break: most deps first, then deterministic fallback." In the new crates two writers of one stored column in one step are refused at bind as `BindRefusal::TwoWritersOfOneColumn`, and no `Bound` is made. In a two-regime step two writers of one staged column have no order a priority could resolve.
3. `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:322-337`, the `WorkUnit` trait, and `mock/DESIGN.md.tmpl:102-104`, the `Ctx` GAT: `fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>)`, `const COMMUTATIVE: bool = false`, `type Hint`. The value kernel has no receiver, is handed values and returns values, and carries no `Ctx`, no hint and no `COMMUTATIVE` constant; commutativity is a merge law declared on a kind. This is the lead designer's property, and a context type is where a later capability becomes an edit to every author.
4. `mock/DESIGN.md.tmpl:86`, "Per-morsel vs per-record WU dispatch inferred from AccessSet shape: column-shaped WUs receive a `Morsel<T>` window". No author form of the new crates but the block kernel (roadmap step V3) is handed a window, a position or a range; the value kernel of this round is handed none.

## Adds to, without contradicting

1. `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:140-148`, "Three store types, one scheduling mechanism", with `Resource<T>`, `Column<T>`, `Virtual<T>`. This round uses two of the three words, a column of a domain and a resource, and adds that a column named in a kernel is a role, bound to a stored column by data. It says nothing on what an emitted kind's log or identity is. It records that the code at head already holds a fourth marker the sentence does not list (`Accum<T>`, `hilavitkutin-api/src/store.rs:119`; "accumulator columns", `mock/DESIGN.md.tmpl:94`), a drift whose resolution is the lead designer's.
2. `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:300-308`, the resource sentence. Added: a per-binding coefficient is a resource value supplied by the binding and read as a `Param` member.
3. `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:150-157`, "The scheduler sees StoreId pairs for edge construction", "`StoreId` is a dense index assigned at plan time", and `AccessMask` as `Mask64` or `Mask256`. Added: the new crates name a stored column by a typed handle minted by the bind, and the 128-bit word of the `KernelId` collision check is a stand-in for arvo's masks, marked so, its width no interface (ticket `cc007451af6f`).
4. `mock/DESIGN.md.tmpl:42`, the parameterless `run(&mut self) -> notko::Outcome<Cfg::Ok, Cfg::Err>`. Added: the reference runner's `run` is a library function over a checked table, a store and a visiting order; from HL0b the close returns `Outcome<Seal, Refusal>`.
5. `mock/DESIGN.md.tmpl:114`, the vocabulary line. Added words: binding, role, logged form, member (of a read list), and `Lane` for the arithmetic vocabulary as distinct from a column. No retired word is used.
6. `mock/DESIGN.md.tmpl:138-139`, principles 1 and 2. Kept without change and restated in each new crate's design: the model of a frozen author is fixed-capacity because of the first, and `Read::get` takes a typed handle because of the second.
7. `mock/DESIGN.md.tmpl:12-34`, the crate layout table and the dependency graph. Added: five rows, the statement that no new crate depends on a legacy crate, and that `hilavitkutin-api` is the legacy contract until the port.

## Calls the plan left open, and what each rests on

1. The neighbour member is `Near<C, O>`, `O` a type the domain declares with its step as plain data (`Offset::STEP`). It rests on the runner-generic sketch's two-axis case, where the offset is plain data per axis, which the brief settles as the spelling.
2. A domain declares a run-time `Shape`, and `first`, `next` and `reach` over it, beside the plan's `Key` and `Step`. With them one store is written for every domain: it walks the keys in order to lay records out and answers a neighbour read by key. Unoptimal is allowed at this step, and the stores find a key's position by halving their key table.
3. The proxy lane's term bound is `2^BITS - 1`, where the plan's figure is `2^BITS`. Subtracting the minimum, of magnitude `2^(BITS - 1)`, `2^BITS` times gives `2^(2 * BITS - 1)`, one past the largest value of the `2 * BITS` register. The surface test shows the bound exact both ways. Author 1 asserts at build time that its six terms fit its lane's bound.
4. The B1 mutant `store_skips_last_record` is a store that drops its write to the last record. A store sees no morsel, the visiting order does, so "the last record of a morsel" is the last record of the last morsel in forward order.
5. B3 is stated as two clauses over plain data: a step leaves the committed cells as it found them, and two runs from one committed state, their staged storage poisoned with two different words before the step, commit equal cells. The plan's one clause, start equals end, does not fail its own mutant `store_hands_staged_as_read`, which writes nowhere committed; the second clause does.
6. A coefficient whose format is not its `Param` member's is refused as its own `BindRefusal::CoefficientMismatch`, so a refusal names a role or a coefficient and never one as the other.
7. The digest order of a frame: the frame's number, then unit by unit in table order, record by record in key order, each written column's value. Unlocked until the journal format locks.
8. `Values` is the only list projection born; nothing uses a positional selector, since an author destructures. It is local until notko step NK1 (ticket `33d72d28dd71`). The proxy lane and `Lane` go at the swap (ticket `69b1c00469a6`).
9. The ledger check lives in the laws crate and the frozen crates' `tests/ledger.rs` expands it, so both crates read their ledgers by one function; it also refuses a source file with no ledger line.
10. The cross-target script of the wasm sketch moves to `mock/tools/cross-target/` as a check tool over a loop crate shared by a native and a wasm32 front end, both running author 1 through `run`.

## Flat topics this round inherits

1. `202607200100_topic.b1-cap-lifting-completion.md` moves the engine scheduler's scratch onto `PlanDims` capacities. This round touches neither the engine nor `hilavitkutin-api`'s `PoolFrame`, so it needs nothing here and stays filed for the engine's own next round.
2. `202607281547_topic.adopt-the-shared-hlist.md` switches `hilavitkutin-api`'s list to `notko-hlist`. The five new crates are born on `notko-hlist`, which is the half of it this round can do; the switch of the legacy crate is moot if the port deletes it first, and stays filed until the port decides which.
3. The branch `origin/recover/unlanded-rounds-and-benches` parks older rounds as a record (`b24f158`). It is no flat topic of this branch and is left to whoever recovers it.

## Out of scope

No design text classifies an emitted kind's log or identity as a store kind. Nothing here takes a stance on roadmap r6's lines 177 to 179. The engine, `hilavitkutin-api` and every other existing crate keep their designs and their code, apart from the legacy mark.
