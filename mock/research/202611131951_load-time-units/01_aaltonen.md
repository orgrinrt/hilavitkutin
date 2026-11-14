# Seat 01: load-time units, growth, and the consumer's cell format

Seat 01 of two, written 2026-11-13 on `research/load-time-units-seat01`, cut from `origin/dev` at `271909e5`. Lens: memory access patterns, what the hardware sees, and what a schedule has to know before it runs. Every sketch named here is checked in under `mock/research/sketches/202611131951_load-time-units-seat01/`, with its output beside it. Every sketch is a spike: one instance, one check, written to reach that check and not as a shape to copy. No timing figure in this file was taken by me. Where a question turns on how much something costs, it is unpriced, and the bench it needs is named.

Citations without a path prefix of the form `spec :N` point into `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`. `core :N` is `202603141800_topic.hilavitkutin-core-design.md`, and `api topic :N` is `202603151200_topic.hilavitkutin-api-surface.md`, both in the same directory. Source paths are relative to `mock/crates/`.

## 0. What the gates found

### 0.1 Canon gate

What governs: hilavitkutin declares no `canon_paths` and has no `mock/registry/`, so the founding record and the trunk designs are the canon. Op's own words outrank both. I checked the question and the state it builds on against R2, R3, R4 and R6 (spec :2407-2446), domains 04, 05, 07, 16 and 23, the trunk `hilavitkutin-api` design, and op's captures of 9 October, 15 October and 13 November.

Result: the question is aligned with the record. The state it builds on is not, in five places. Each is drift in the trunk against the founding record, and none has an op ratification I could find.

1. The value bound is gone. The record says the 16-byte maximum is "Enforced at compile time on the blanket impl's where clause" (spec :460-461) and keeps "Compile-time size enforcement" (spec :476). The trunk has `pub trait ColumnValue: Copy + 'static` and `impl<T: Copy + 'static> ColumnValue for T {}` with no bound of any size (`hilavitkutin-api/src/column_value.rs:25-34`). The trunk design copies the same shape and still heads the section "domain 05, R4" (`hilavitkutin-api/DESIGN.md.tmpl:216-231`). Claiming R4's lineage while deleting R4's bound is not a revision. It is a deletion with a citation attached. Sketch c1 shows what that deletion costs (section 3).
2. `repr(C)` on stored types is gone. The record retains "repr(C) on stored types (T3 §Q3)" (spec :477) and says it is enforced "by `#[derive(ColumnValue)]` macro or lint" (spec :462). The trunk has neither the derive nor the lint. Sketch c6 shows why this matters once a column crosses a library seam (section 3).
3. The packed layout is gone. The record has BIT_WIDTH informing "BitPacked column layout mode" (spec :454-457, :402-404). The trunk keeps the constant and says "nothing reads it today" (`hilavitkutin-api/src/column_value.rs:8-12`).
4. Growth is gone. The record has "All dynamic storage via SlabColumn" (spec :285, :308), describes SlabColumn as "zero-copy-growth" (api topic :663), and keeps its principles when its type went away (spec :570-571). It puts growth on the consumer (spec :1470) and has the plan rerun "When the consumer adds records" (spec :1487-1489). On the trunk, `build` consumes the storage and the record count (`hilavitkutin/src/scheduler/builder.rs:167-171`). No public method on `Scheduler` replaces the count or hands the storage back: the whole public surface is `run`, `run_parallel`, `run_fused`, `run_one_trunk*`, `replace_resource`, `replace_value`, `mark_dirty` and borrow-only `__storage` (`hilavitkutin/src/scheduler/dirty.rs:48-157`, `run_fused.rs:273`). The one storage entry that claims to resize, `reserve`, frees the old block (`hilavitkutin-providers/src/storage.rs:68-77`), and sketch b2 shows the records are lost.
5. The record count is not adaptive. R6 lists "record counts (from consumer data)" among the plan parameters that adapt (spec :2443). On the trunk it is a build argument and nothing else.

One ambiguity in the record itself, which I hand to op rather than resolve (call A-2 in section 1). R6 says "No runtime WU registration, no dynamic schedules" (spec :2440), and domain 23 repeats it (spec :2113-2114). Domain 16 says the plan reruns when the consumer "changes WU registration" (spec :1487). The core design fired `meta::PlanStage` on "plugin load" (core :248, :723). The spec's own meta list says "fired when DAG changes" (spec :2125) and drops the plugin wording. The api topic left "dynamic schedules (work units registered at runtime, e.g., plugin loading)" as an open question (api topic :1348-1352). The consolidation closed it with R6. I read R6 as governing and domain 16's "WU registration" as a change made at compile time followed by a rebuild. That is my reading, and it is the kind of reading the canon rules tell you to distrust, so it goes to op.

Two-expert rule: I am the first read. Seat 02 owes an independent one before anything here about what the record permits is treated as settled.

### 0.2 Test gate

I ran the whole workspace suite from `mock/` with `CARGO_INCREMENTAL=0` and no fail-fast. It does not finish green, and the failure is not where anyone would look first. Two crates' test binaries fail to link: `hilavitkutin-persistence` (`archive_str`) and `hilavitkutin-providers` (`smoke`). Both stop on undefined `__start_hilavitkutin_strings` and `__stop_hilavitkutin_strings`. These are the linker-section bounds `hilavitkutin-str` relies on (`hilavitkutin-str/src/section.rs:24-25`), and lld's start-stop garbage collection drops them when nothing in the binary keeps the section alive (lld documentation, fetched 2026-11-13, `/home/user/.data/fetched/16252115e57c41671a02a395ca03f07d63b17c4fdb36bdf1fa98da59ec31cf18.html`). With those two binaries excluded: 464 passed, 0 failed, 21 ignored. The lib tests of the two crates contain 0 tests each, so excluding them hides no assertion.

Then I read the bodies of the tests in the surface my sketches touch.

- `hilavitkutin-api/tests/column_storage.rs:47-54`, `columnstorage_is_implementable_and_reserve_count_roundtrips`, is tautological. It defines its own `StubStore` (`:18-44`), whose `reserve` stores `len` into `last_count` and whose `count` returns `last_count`. It then asserts that `count` returns what `reserve` was given. The only thing under test is the stub written three lines above it, and no implementation of `ColumnStorage` that anyone ships is involved. The comment above it says it "Proves the trait is implementable and that reserve / count interact as a sane impl would" (`:15-17`). It proves neither: the compiler proves the trait is implementable, and the stub proves only that the stub is the stub. This is a decorative test sitting where a real contract test of `reserve` belongs, and it should be deleted, not improved.
- No test anywhere asks whether a re-reserve keeps the records, although the api design and the doc comment both promise "Re-reserving an existing id resizes it" (`hilavitkutin-api/DESIGN.md.tmpl:267`, `hilavitkutin-api/src/storage.rs:46-47`). The provider test closest to it, `a_second_reserve_frees_the_first_block_as_it_was_reserved` (`hilavitkutin-providers/tests/column_storage.rs:215`), pins that the free happens. So the one place the suite looks at a resize, it asserts the behaviour that breaks the documented contract. Sketch b2 is the missing test, and it is red.
- `hilavitkutin-linking/tests/basic.rs:17-20` loads `/lib/x86_64-linux-gnu/libc.so.6` in every load test (`:38`, `:47`, `:65`, `:87`, `:103`, `:140`). Every process already has libc mapped. This is setup that helps: it is the one library for which the crate's wrong `dlopen` flag still works. Sketch `a4-slot-unit/host/tests/loader.rs` loads a library that is not yet mapped, and `Library::load` refuses it (section 1.4, finding A4-1).

The gate tells me to refuse until this is fixed. The brief forbids me to edit crate source or tests, so I cannot fix it. I went ahead anyway: my sketches carry their own assertions and do not lean on any of these tests, and op's standing instruction on 13 November was to do the work. Under the canon rules, that call of mine is the most suspect thing in this section, and the dispatcher should treat it that way. The three suite defects above stand as findings. The tautological test should be deleted, a re-reserve contract test belongs in `hilavitkutin-providers`, and the linking tests need a library that is not already resident.

### 0.3 Breaking the brief

The brief's factual claims, checked against source:

- "nothing resizes a store" (part B, from the ticket text at `/home/user/.shared/scripts/life/tickets-hilavitkutin.toml:442`). This is wrong on the trunk in a worse way than the brief says. The trunk claims to resize (`hilavitkutin-api/src/storage.rs:46-47`, `hilavitkutin-api/DESIGN.md.tmpl:267`), and what it does is free the block and hand back a fresh one (`hilavitkutin-providers/src/storage.rs:68-77`). Sketch b2: eight records written as 1000..1007 read back as eight zeros after a re-reserve from 8 to 16, and the base pointer moves (`b2-resize-loses-records/test-output.txt:17-27`).
- The stack audit says "re-reserving is undefined (`crates/hilavitkutin-api/src/storage.rs:46-47,:59`)" (`mock/research/202611131215_stack-audit/DOWNSTREAM.md:15`). Those lines say the opposite. They say it resizes. The audit misquotes its own citation, and the brief imported the misquote.
- Part C's ticket premises are "Word (eight bytes) is the storage unit and Cell64 the widest cell" and "Value is sealed and Int<BITS> is its only impl" (`tickets-hilavitkutin.toml:450`, `:506`). These describe the voided `hilavitkutin-contracts`. They do not describe the trunk. On the trunk, any `Copy + 'static` type is a column value (`hilavitkutin-api/src/column_value.rs:34`). A consumer already has its own value type, and narrow lanes are already expressible: every sketch in this file stores `i32`. Both C tickets are malformed against `hilavitkutin-api`. The real gap is the reverse of what they say: the trunk is too loose, not too narrow.
- The brief does not mention the question that is already in front of op: `which_registration_hilavitkutin_s_contract_takes_beside_the` (`tickets-hilavitkutin.toml:565-572`, state "waiting, held by op"). It asks whether content-declared stores and units join the contract, and it carries a sketch and a harness bench (`registration_binding`) on `origin/work/api-revision` at `c0bb6a07`, not landed. That is part A asked a second time from a different side, so a panel on A that does not read it repeats it. I read it. Its numbers are tier 3, one host, one workload, but they are the only harness numbers anywhere near this question.
- The brief tells me to write only under `mock/research/` and also allows "a bench under `mock/benches` if one fits". Those two instructions conflict. I followed the stricter one, wrote no bench, and name each bench owed.
- The op question put on 13 November framed A as "an extension's work units cannot join a schedule" (`/home/user/.data/op-responses/202611131350_op-on-last-option-this-is-already-specced-in-the.md:285`). That merges two things: a unit joining the schedule, and extension behaviour running inside the schedule. The second needs no new unit (arms A3 and A4, both sketched and green on the trunk engine). Option 1 ("Content is data, units fixed") and option 2 ("A seam for load-time units") are not the only two shapes. There is a third family, in which the units are fixed, the behaviour is loaded, and the seam is inside a unit's body.
- kaski's `modding_is_content_only_never_engine_code` has standing "demoted" (`/home/user/kaski/mock/registry/tenet/content.toml:15-16`) and binds nothing. What binds kaski is `content_kinds_are_open_and_interned`, ratified on 13 November (`content.toml:26-28`): kinds are "interned when content loads into a dense typed id", and "a new kind is new content, never an engine change". The ikiuni rows that keep coming up near this topic (`the_freeze_split_...`, `the_600_expresses_declarations_...`, in `/home/user/kaski/mock/registry/ikiuni/`) carry no standing and sit outside kaski's `canon_paths` (`/home/user/kaski/mockspace.toml:30`). They are references and govern nothing.

### 0.4 Unlicensed things met on the way

Reported because the standing instruction asks for it, whether or not they are in scope.

- `hilavitkutin-linking` passes `RTLD_LOCAL = 4` (`hilavitkutin-linking/src/backend/unix.rs:8-9`, used at `:26`). On linux glibc, `RTLD_LOCAL` is 0 and 4 is `RTLD_NOLOAD` (libc 0.2.191, `src/unix/linux_like/mod.rs:1205` and `src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:288`). 4 is `RTLD_LOCAL` only on apple targets. So on linux, `Library::load` returns a handle only for a library that is already mapped, and loading a mod for the first time cannot work. Sketch: `a4-slot-unit/host/tests/loader.rs`, red at `:34`. Control: the same file opens with libc's own constants, and after that `Library::load` finds it (`test-output.txt:70-88`). The whole plugin-host layer, the one sanctioned way to bring code in at run time, has never loaded a plugin on linux. Its suite stays green only because the suite loads libc.
- `hilavitkutin-str` walks a linker section and grants itself an exemption from the repo's linker-magic ban (`hilavitkutin-str/DESIGN.md.tmpl:92-145`). The plugin-host layer's own research names this mechanism, a linker section read across `dlopen`, as the anti-pattern that "leads to silent failures where plugins successfully `dlopen` but register zero descriptors in the host process" (`mock/research/EXTENSIONS_RESEARCH.md:42-46`). Sketch `a4-slot-unit/host/tests/slot.rs` shows exactly that failure for names (section 1.4). The exemption's point 4, "same string in, same `Str` out", is false across an image boundary (sketch, `test-output.txt:108-110`). Its point 1 says the interner "reads the section once at startup, populates a lookup table". The code does a linear scan of the section on every const lookup (`hilavitkutin-str/src/interner.rs:119-140`), and point 5's "O(1) const lookup" is a linear scan as well.
- `static_entries` returns `&[]` on every target that is not linux, android, macos or ios (`hilavitkutin-str/src/section.rs:69-80`). wasm32 is one of them. On wasm, no name declared with `str_const!` resolves, not even in its own image (sketch `a4-names-on-wasm/output-wasm.txt`). Native control: `output-linux.txt`. The options put to op on kaski's lattice ticket make a lane's assignment wait on "matching digests on native and wasm" (`/home/user/.data/op-responses/202611130448_answers-to-the-fix-up-calls-batch-1.md:147`), so wasm is a target kaski counts on.
- `str_const!` expands to paths through `arvo_bits` rather than `$crate`, so every crate that declares a const name has to depend on `arvo-bits` directly (compile error captured while building `a4-names-on-wasm`, now fixed in its `Cargo.toml:18-20`). It also needs `#![feature(const_trait_impl)]` in the declaring crate (`a4-slot-unit/ext/src/lib.rs:1`). That is an unstable gate leaking into every consumer through a macro. I found no vetting row for that leak.
- The trunk api design copies the record's resize sentence (`hilavitkutin-api/DESIGN.md.tmpl:267`) while the implementation under it discards the records. The design is right and the code is wrong. Fix the code toward the design (section 2).

## 1. Part A: units known only at load time

### 1.1 What the record says

- R6: "Pipeline composition is static: all WUs registered via the builder at compile time. The WU set, DAG structure, fiber/trunk/phase topology, and monomorphised dispatch functions are all fixed at build time... No runtime WU registration, no dynamic schedules." Then: "Plan-stage parameters are adaptive: morsel sizes..., record counts (from consumer data), per-phase strategy selection..., and morsel-to-core affinity" (spec :2435-2446).
- Domain 23: "The plan stage adjusts only what compile time cannot", and this "eliminates the need for fallback dispatch approaches (Approach C) entirely" (spec :2112-2117). Approach C, an indirect call per fiber per morsel, measured 1.17x (spec :1553) and was "superseded... (1.17x overhead unacceptable)" (spec :1653-1654).
- Domain 23 also: "No `#[register]` in hilavitkutin itself; consumers handle registration their way (polka: inventory, saalis: Named, loimu: macros)" (spec :2105-2107). Registration machinery is the consumer's, at compile time.
- R2: "Column type registration is static (compile-time), no runtime column type creation" (spec :2410-2411).
- Domain 07 keeps "HasShape/ErasedColumn from loimu (T5 L674), for plugin type access" (spec :577). That is the record's one nod toward a plugin reading columns: an erased, shape-described view.
- The plugin-host layer (CLAUDE.md, crates `hilavitkutin-linking`, `-extensions`, `-extensions-macros`): pull-based explicit-symbol loading, arbitrary-time load and drop, "the engine never loads WUs at runtime". Op on 13 November: "The extension model is a literal c abi, it's standard ffi. Neither it nor me care about what supplies the whatever links through it" (`/home/user/.data/op-responses/202611131806_op-on-the-extension-model-is-a-literal-c-abi.md:20`).
- Op on kaski's content acts, 15 October. A layer-two act is "a genuinely new kind of act, written in the content syntax as a composition of the engine's Mutation forms and predicates, checked at pack load". Op answered: "Okay yeah we'll want a dsl... a placeholder dsl in similar shape as vehje would... Preferably behind an interface" (`/home/user/.data/op-responses/202610152016_op-on-the-kaski-morning-questions-planet-the-action-dsl.md:282`, `:300`). On 9 October: "I'd much prefer we use vehje to design our own like Clausewitz does" (`202610091035_op-on-spoil-the-way-into-a-pit-roofs-under-snow-and-mod.md:115`).

Where it stops: the record says what may not change at load (the unit set, the graph, the dispatch functions) and what may (parameters). It says nothing about behaviour that arrives as data or as foreign code and runs inside a unit the schedule already has. Domain 16's "changes WU registration" (spec :1487) is the one sentence that could be read as permitting more, and section 0.1 hands it back.

### 1.2 The arms, by category

Five categories, not five variants of one.

- Data only. A1: content is rows in typed columns that compiled units process. A2: a compiled superset of units, with content switching them on (a mask resource, or a `Virtual` fired from content).
- Behaviour as data, run by a compiled unit. A3: an interpreter unit whose access set is fixed in Rust and whose program is loaded with the content, run a vector at a time per morsel. A5: the same with the program compiled to native code at load (a JIT), falling back to A3.
- Behaviour as foreign code, called by a compiled unit. A4: a slot unit whose access set is fixed in Rust and whose body calls a kernel loaded through the plugin-host layer, native or wasm.
- Composition changed, but at build time. A6: an install-time rebuild in which mod crates are compiled into the binary and the schedule stays static. A7: a menu of schedules compiled ahead, with content choosing one.
- Composition changed at run time. A8: a dynamic sub-schedule beside the compiled one, which is op's option 2.
- Locus. A9: all of this is kaski's, above `hilavitkutin-api`, and hilavitkutin owes only the primitives the arms above need.

### 1.3 What each arm's failing case shows

A1, content as data. This arm fits the record without residue, and kaski's ratified tenet asks for exactly this for kinds (interned dense ids at content load). Its failing case is op's own words: "a genuinely new kind of act... a composition of the engine's Mutation forms and predicates". If only rows are allowed, the compiled unit has to already contain every composition, and then the act is not new. A1 alone does not meet op's 15 October answer. A1 plus A3 does. No sketch, because the failure is definitional, not empirical.

A2, compiled superset. Failing case: a mod whose behaviour no compiled unit has. That is definitional too. The open cost is that idle units in the superset still take a slot in the plan, and a fiber that holds them may be fused differently. Unpriced. Prior art: Factorio's lifecycle. A settings stage and a data stage build prototypes, which then freeze before the control stage runs scripts against events (Factorio "Data lifecycle", fetched 2026-11-13, `/home/user/.data/fetched/000da04e15507af174efa60aeec2d10b1824b8784715906bb2bfe030514a24ea.html`). That is A1 for kinds, plus a scripted control layer, which is A3's family and not A2.

A3, interpreter unit. Sketched: `a3-morsel-interpreter/`. A compiled unit with a fixed access set (`pos`, `vel`, `kind` read; `out` written) runs a program loaded as text: `"load vel; load kind; coef; mul; shr 1; load pos; add; store out"`. It runs inside the real trunk engine at N = 10000. The program is checked at load against the unit's declared slots, so the access set stays the compile-time fact, and refusals happen at pack load, as op asked. Results (`a3-morsel-interpreter/test-output.txt:69-75`): the load-time program agrees with the same rule compiled in Rust in both evaluation forms, per morsel a vector at a time and per record. A control program disagrees. A program naming an undeclared store, loading a write-only store, or storing into a read-only store is refused at load. 5 of 5 pass.

The case that had to fail, an access outside the declared set, does fail, and it fails at load rather than at run. That is the property that makes A3 legal under R6: no unit was registered and the graph did not change. The program is data, like a morsel size.

What the trunk is missing for A3: the Context gives one record at a time, `read(i)` and `write(i, v)` (`hilavitkutin-api/src/context.rs:117`, `:134`), and no window slice. An interpreter running a vector at a time wants `&[T]` for the morsel. That is "One base pointer per morsel, columns at `base + col_offset + i * stride`" (spec :397-398), which the record has and the api does not expose. The sketch evaluates through per-record reads, so it proves the property and not the speed. Prior art: MonetDB/X100 runs interpretation a vector at a time so that the per-tuple overhead amortises over a cache-resident vector (Boncz, Zukowski, Nes, CIDR 2005, fetched 2026-11-13, `/home/user/.data/fetched/5d675401aa740e1b7ec64a91ef4d66214530af2bb30269b66445d11dd5737a2f.pdf`). That is the morsel exactly.

Price: unpriced. The nearest harness numbers are tier 3: a table consulted once a morsel was inside the noise of two builds of one source, and once a record cost 7 to 8 times the typed loop (`registration_binding`, quoted at `tickets-hilavitkutin.toml:567`). That is consistent with A3 per morsel being cheap and A3 per record being out of the question. It is not a measurement of an interpreter. Bench owed: `mock/benches/` arm set {typed fused rule, A3 per morsel over a window slice, A3 per morsel through per-record reads, A3 per record}, at 1e4, 1e5, 1e6 and 1e7 records, programs of 4, 8 and 16 ops, one fiber and three, at opt-level 3 with fat LTO.

A4, slot unit. Sketched: `a4-slot-unit/`. A cdylib built separately (`a4-slot-unit/ext/`) exports `ext_kernel(pos, vel, out, len)` over a plain C ABI. A compiled unit in the trunk's schedule calls it once per morsel, at N = 20000, and every one of the 20000 records agrees with `pos + vel * 2` (`a4-slot-unit/host/test-output.txt:107`). So op's c ABI stance already gives load-time behaviour inside a static schedule, with no change to R6. The access set is fixed in Rust, and the kernel is a parameter of the unit, as a morsel size is a parameter of the plan.

Six things fail on the way, and they are the real findings of this arm:

- A4-1. `Library::load` cannot open a library that is not already mapped on linux. The flag is wrong (section 0.4). Red: `loader.rs:34`. The sketch gets past it by mapping the image first with libc's own flags (`slot.rs:52-75`). That is scaffolding to reach the slot question, not a fix.
- A4-2. A name the extension declares with `str_const!` does not resolve in the host. The host's section has 1 entry and the extension's has 1. The extension's id 240139607 resolves to nothing in the host (`test-output.txt:94-97`). Each image has its own section, and the host walks only its own.
- A4-3. One name, two handles. The host interns the extension's name from text and gets run-time handle 0, while the extension holds const handle 240139607 for the same text, and equality says they differ (`test-output.txt:108-110`). Any content key that crosses the seam as a `Str` is wrong without any error being raised. For kaski, whose ratified tenet makes every kind an interned id, this is the shape of the bug that corrupts a save.
- A4-4. On wasm, const names do not resolve at all (`a4-names-on-wasm/output-wasm.txt`: section entries 0, own const name resolves false). Native control: `output-linux.txt`.
- A4-5. The trunk's Context has no window accessor, so the unit copies the morsel into scratch, calls the kernel, and copies it back (`slot.rs:157-174`). That is two extra passes over the window per morsel. The pointer the extension needs is the record's per-morsel base (spec :397-398).
- A4-6. Layout across the seam (sketch `c6-layout-across-seam/`, section 3). Host and extension are compiled apart, so a default-repr cell has no layout both sides are promised to agree on.

What A4 costs per call is unpriced. Web prior art, not fetched (search results only, 2026-11-13): Wasmtime host-to-guest calls are reported in the tens of nanoseconds (Bytecode Alliance zulip, wasmtime issue 3319). At one call per morsel of 6144 records that is noise. At one per record it is fatal, the same split the tier-3 bench shows. Bevy removed its dynamic plugin loading as unsound across the Rust ABI (Bevy 0.14 to 0.15 migration guide, fetched 2026-11-13, `/home/user/.data/fetched/bce33dd7af5c55b5a9caecd11b5e0cabefeff161457f0c7db63a3ead79cd4529.html`). Op's c ABI is the remedy for that, and it carries one obligation the trunk dropped: a layout that both sides agree on.

A5, load-time JIT. Not sketched. Prior art says this is the mature version of A3. Kohn, Leis and Neumann start a morsel-driven query in a bytecode interpreter and compile hot pipelines in the background, switching per morsel when the compiled code is ready (Adaptive Execution of Compiled Queries, ICDE 2018, fetched 2026-11-13, `/home/user/.data/fetched/0a1f809945ad88e8bb850dd4a3e1a4ab3182ef3cb07be5398a1b383002daa2c3.pdf`). That is HyPer's lineage, the same morsel lineage this engine comes from. Failing case, as a hypothesis: targets that forbid writable-then-executable pages (wasm, iOS, consoles) have no JIT, so A3 has to be the floor everywhere and A5 an optimisation where it is allowed. A JIT needs an allocator and a code buffer. It cannot live in the engine (`no_std`, no alloc) and can only be a consumer crate.

A6, install-time rebuild. Not sketched. It fits R6 perfectly, because the schedule stays static. Failing case: it needs a Rust toolchain on the player's machine, or a build service. It is also the only arm with zero run-time cost, and the arm that A3 and A5 have to be measured against.

A7, a menu of schedules. Not sketched. Failing case: combinatorial. Each independent toggle doubles the menu. It is fine for a handful of modes (a fidelity tier) and wrong for mods.

A8, a dynamic sub-schedule. This is forbidden by R6 (spec :2440) and domain 23 (spec :2113-2114), and it brings back the Approach C cost class the record killed (spec :1653-1654). It needs op to reverse the spec. It is listed so that it is visibly considered and visibly not proposed. A3 and A4 deliver the behaviour without it.

A9, locus. kaski's need, per op's 15 October answer, is a DSL composed over engine forms and checked at pack load. That is A3 in kaski, behind an interface, later ported to vehje. hilavitkutin's part is to make A3 and A4 fast and correct, not to host them. What hilavitkutin owes back is concrete: a per-morsel window slice on the Context; a fixed `Library::load`; a cross-image story for interned names; and a layout bound on values that cross the seam.

### 1.4 Composition for A

- Kinds and parameters: A1, everywhere.
- New behaviour on every platform: A3 per morsel, in the consumer, checked at pack load.
- New behaviour where native code is allowed and trusted: A4, per morsel, over the window slice.
- Hot programs where a JIT is allowed: A5 over A3.
- Zero-cost shipping of first-party packs: A6.
- The answer flips toward A6 when the mod set is fixed at install and a build service exists. It flips toward A8 only if op reverses R6, and nothing I found needs that.

### 1.5 Op calls for A

A-1. Does anything load-time enter `hilavitkutin` itself?
- a. No. Load-time behaviour runs inside compiled units (A3, A4), built by the consumer. hilavitkutin owes only the window accessor, the loader fix and the name story. R6 stands as written.
- b. An opt-in crate over the api ships a slot-unit contract: a unit type with a fixed access set, plus a per-morsel kernel or program parameter, checked at build of the scheduler.
- c. Reverse R6 for a declared dynamic sub-schedule (A8).
- What decides between a and b: whether two consumers want the same slot shape. Between a or b and c: the bench owed in 1.3. If A3 per morsel is close to typed, c buys nothing.

A-2. Domain 16's "changes WU registration" (spec :1487) against R6 (spec :2440).
- a. It means a compile-time change followed by a rebuild, and R6 governs. This is my reading.
- b. It licenses a plan recompute on registration at load, which would be a spec change.
- c. Strike the phrase from domain 16 as a leftover of the plugin-load wording the consolidation dropped (core :248, :723).

A-3. Interned names across an image boundary.
- a. An extension never declares const names. It interns at run time through the host's interner, passed across the c ABI as a pair of function pointers, so there is one table.
- b. The host pulls each library's const table through an exported symbol at load and merges it. That is pull-based explicit symbols, the mechanism the plugin-host layer already permits, and not linker magic.
- c. Const and run-time handles of the same text compare equal by hash, so the host's arena keys on the 28-bit hash. This changes `Str` equality and the collision story.
- d. Retire the linker section and make const names a build-time table. This also fixes wasm (A4-4) and the link failures in the suite (0.2).
- What decides: whether a mod may define a name the base game reads by constant. The wasm finding by itself says the current mechanism cannot be the only one.

A-4. The pending `which_registration_hilavitkutin_s_contract_takes_beside_the`. This panel's evidence bears on it: A3 and A4 deliver content-defined behaviour with typed registration unchanged, which is that ticket's option a, typed only. I flag it to the dispatcher so the two are consolidated and not answered twice.

## 2. Part B: growth

### 2.1 What the record says

- Domain 16: the consumer owns "storage lifecycle (allocation, growth, eviction via R2 APIs)" (spec :1470). hilavitkutin will "Recompute morsel splits at plan stage on any significant pipeline change (more records, new WUs, structural DAG change)" (spec :1476-1477). "When the consumer adds records... the plan stage reruns. This is a microsecond cost" (spec :1487-1489).
- R6: record counts are an adaptive plan parameter (spec :2443).
- Storage: "All dynamic storage via SlabColumn (mmap on os, std::alloc on std, consumer-provided on no_os)" (spec :285, :308). "Contiguous, fixed-stride, zero-copy-growth column" (api topic :663). The SlabColumn type was dropped and its principles retained (spec :570-571).
- Domain 04: the fiber arena has "One base pointer per morsel, columns at `base + col_offset + i * stride`" (spec :397-398). R3: "Null/tombstone handled via separate bitmask per column" (spec :385-386).
- Op, 13 November, on differing record counts: "Isn't the intent a contiguous per prototype and indices to process needed windows to it?" On extents [replaceable between passes up to a capacity fixed at build | fixed at build | both, declared per domain], op answered "Both, declared per domain". On how record sets of different counts are held, op answered "Bench both first", and that is the later answer, which governs (api revision changelist, section 2, `origin/work/api-revision:mock/design_rounds/202611130617-h6mb/202611130617-h6mb_changelist.doc.lock.md:127`).
- Op on mutation (op-answers-25): "Intended case in the mutation paradigm is writing through queues. Now that can be either a second write buffer OR it can be queued up ops that drain verctoeised and optimized at the close" (`/home/user/.data/op-responses/202611131254_op-on-your-guys-are-sitting-waiting-for-you-again.md:100`).

Where it stops: the record puts growth on the consumer and puts the replan on the engine. It does not say how the engine learns the new count, whether a grown column may move, or what a position means after a removal. Op's "replaceable between passes up to a capacity fixed at build" is the one explicit shape, and the trunk has not built it.

### 2.2 The arms

- B1. Reserve the maximum and carry a live count as a plan parameter. This is op's "replaceable up to a capacity".
- B2. Virtual reservation per column: reserve address space once, commit pages as the column grows, base never moves.
- B3. Pages: fixed-size blocks, a base per page, growth by adding a page, positions never move.
- B4. Relocate on grow between frames: copy, new base, rebind.
- B5. Removal: a tombstone bitmask (R3), or swap-remove with identity kept in a key column. Generations if handles must be checked.
- B6. Spawn through a queue: an `Accum<T>` the frame appends to, drained and merged at the close. This is op's stated intent.
- B7. Locus: the consumer's `ColumnStorage` implements B2, B3 or B4. The engine owes the count parameter and the windowing rule.

### 2.3 What each arm's failing case shows

B4 as the trunk claims it. Sketched: `b2-resize-loses-records/`. A re-reserve from 8 to 16 through `ArenaColumnStorage` moves the base (passes, `test-output.txt:17`) and loses the records (red, `:18-27`, read `[0; 8]` against `[1000..1007]`). Of everything in part B, this is the one that is an outright defect, because the design promises it (`hilavitkutin-api/DESIGN.md.tmpl:267`). A correct B4 is copy then free, and since `build` has recorded column pointers into the bindings (`builder.rs:162-164`), the scheduler has to rebind after it. Nothing in the public surface does that today (0.1 item 4).

B2, reserved column. Sketched: `b3-paged-column/` (`Reserved<T>`, mmap `PROT_NONE` with `MAP_NORESERVE`, then mprotect to commit). It grows from 1000 to 10 million records and the base never moves. Committed memory goes from 8192 bytes at 1000 records to 80003072 bytes at 10M, of 512 MiB reserved (`test-output.txt:12`). For a 4-byte column at 1e7 records, that is exactly the bytes in use rounded up to a page. Nothing is copied and no pointer goes stale, so it is the zero-copy growth of api topic :663. Failing cases, not sketched: `MemoryProviderApi` has `allocate`, `deallocate` and `protect`, and no reserve or commit (`hilavitkutin-api/src/platform.rs`, the trait the sketch's `ZeroedHeap` implements). wasm linear memory has `memory.grow` and nothing else, and 32-bit targets run out of address space at a handful of reserved columns. So B2 is the os-tier shape and cannot be the only one.

B3, paged column. Sketched: `b3-paged-column/` (`Paged<T, P>`). It grows to 1 million records, record 5 never moves (passes), and windows that divide the page never straddle (passes). Red: the trunk's own window does straddle. For one 4-byte write column, the window is `24576 / 4 = 6144` records (`hilavitkutin/src/plan/steps/step9_morsel.rs:73-75`, `hilavitkutin-api/src/run_cfg.rs:113-122`). Over 4096-record pages, 170 of 171 morsels straddle a page. Over 8192-record pages, 85. Over 16384-record pages, 42 (`test-output.txt:15-20`). A straddling morsel has two bases, which breaks domain 04's one base per morsel (spec :397-398), so the window has to be split at the page boundary or chosen to divide the page. The L1 formula gives windows that are a multiple of 4 and almost never a power of two (`& !3`, `step9_morsel.rs:75`), so pages only work if the planner rounds the window down to a divisor of the page, or the page is sized as a multiple of every fiber's window. Unity's Entities is this arm with fixed 16 KiB chunks, and its per-chunk iteration is the split-at-boundary answer (Unity Entities 1.3, archetype concepts, fetched 2026-11-13, `/home/user/.data/fetched/96f6ecbdf6be9cf981c56d4249f2171e54d7755db3a0617ce02eda4887e210d6.html`).

B1, reserve the maximum. Not sketched, cited instead. The plan's total is the `record_count` given at build (`builder.rs:167-171`), and step 9 windows over it. A live count smaller than the capacity is not a plan parameter, so every unit walks the whole capacity: at 1000 live records in a 10M reserve, that is 10,000 times the work. This is op's "replaceable between passes up to a capacity fixed at build", and the trunk cannot express it. What it needs is a count that the plan band reads, beside `replace_resource` (`dirty.rs:48`), and R6 already lists record counts as adaptive (spec :2443).

B5, removal. Not sketched. A tombstone bitmask keeps positions, so every cross-record reference stays valid and the lanes stay wasted until compaction. Swap-remove keeps the column dense and moves positions, so identity has to live in a key column and not in a position. That is the workspace rule that a handle outside an access set is a red flag, turned into a design: the stable handle is a value in a column the unit declares. Generation counters for checked handles are prior art in skypjack's ECS series, part 3 (fetched 2026-11-13, `/home/user/.data/fetched/820007518b378b5041edfc7d6be763d29b5af92f6dda83a8b334c00a47fac301.html`). Bevy chooses per component between table storage, dense and swap-removed, and a sparse set (Bevy component storage, fetched 2026-11-13, `/home/user/.data/fetched/0b83d7f41cda9a6a95a22530020d2bf63a391e233101bf3c95e2e9b900dadf8a.html`). Per-column choice is the composition here too.

B6, spawn through a queue. Not sketched. The trunk already has half of it. `Accum<T>` appends (`hilavitkutin-api/DESIGN.md.tmpl:452-456`), and an append past capacity is refused and reported at the close with `wanted` and `had` (`hilavitkutin-api/src/accum_refused.rs:3-7`, `:12-19`). So the close knows exactly how far to grow. The missing half is the drain: at the close, append the queue to the target column (B2 or B3), bump the live count (B1's parameter), and replan. The failing case is the frame that overflowed: its excess spawns are refused, so either the accumulator is sized from last frame's `wanted`, or the drain runs in a second pass. Which of the two is unpriced.

### 2.4 Composition for B

- os tier: B2 for columns that grow, with B1's count as the plan parameter.
- wasm, no_os and 32-bit: B3, with the planner rounding windows to a divisor of the page.
- Spawning: B6 drained at the close into whichever of those the column uses.
- Removal: tombstones (R3) by default, with a swap-compaction pass at the close for columns that declare it.
- B4 only as the correct form of what `reserve` already promises, for columns that grow rarely.

Benches owed (`mock/benches/`): {B2, B3 rounded, B3 split, B4} at steady state and on the grow event, at 1e3 to 1e7 records, one and three fibers. And {B6 sized from last frame, B6 with a second pass} under a spawn burst. Op's "Bench both first" on record sets is a separate bench, and it is the one that decides the frame these sit in.

### 2.5 Op calls for B

B-1. Where the live count lives.
- a. A plan parameter replaced between frames up to the build capacity. This is op's "replaceable" half.
- b. Fixed at build, with growth by rebuilding the scheduler. Today that loses every column, because `build` takes the storage and nothing returns it.
- c. Both, declared per domain, as op answered, once the record-set bench settles what a domain is.

B-2. Fix `reserve` toward its design (copy, then free, then rebind), or strike the sentence from the design and say that re-reserve discards. Either is honest. The current state is neither.

B-3. Growth shape per platform tier: the composition in 2.4, or one shape everywhere (B3 is the only one that runs on every tier).

B-4. What a position means across a removal: stable (tombstones), dense (swap, with identity in a key column), or declared per column.

## 3. Part C: the consumer's cell and lane format

### 3.1 What the record says

- R3: "Stride = size_of::<T>() per column... Value constraint size_of::<T>() <= 16" (spec :2417-2418), and "Full 16 bytes usable. Clean power-of-two, fits u128, aligns with SIMD" (spec :388).
- R4: "Blanket impl: any `Copy + 'static` type ≤16 bytes is column-storable. Zero friction for consumers" (spec :448-449). "Consumers store whatever they want" (spec :458-459). "16-byte max... Enforced at compile time on the blanket impl's where clause. Forces interned IDs for String/PathBuf/Vec" (spec :460-461). "repr(C) on domain newtypes enforced by `#[derive(ColumnValue)]` macro or lint" (spec :462). Retained: "Compile-time size enforcement", "repr(C) on stored types" (spec :476-477).
- BIT_WIDTH for BitPacked layout (spec :454-457). Layout hints ByteAligned, Natural or BitPacked per column (spec :402-404).
- Morsel formula: `L1_usable / (Σ size_of::<T_i>())` (spec :407).
- Op on kaski's lattice lanes, 13 November: "Upstream on hila it should be entirely generic and configurable by user. Unless that's infeasible, but I swear I remember the research and panels there found ways and benched it okay with enough headroom to match or surpass as ceiling" (`/home/user/.data/op-responses/202611130448_answers-to-the-fix-up-calls-batch-1.md:159`).

Where it stops: the record fixes the ceiling (16 bytes), the floor (sub-byte by BIT_WIDTH) and the layout (`repr(C)`). It does not say what the planner must do so that a packed column survives a parallel split.

### 3.2 The arms

- C1. The type is the storage (R4): the consumer's `Copy` type is the cell.
- C2. Declared packing: BIT_WIDTH below 8, in BitPacked layout.
- C3. AoSoA: the fiber arena's co-located columns (spec :396-399), with a consumer's struct spread over columns.
- C4. Wide cells beyond 16 bytes.
- C5. Locus: kaski's lane width is kaski's format choice over C1 and C2.
- C6. A layout bound at the seam: `repr(C)` on any value that crosses an image boundary.

### 3.3 What each arm's failing case shows

C1 holds on the trunk, more than holds. A consumer type is a column value with no ceremony: every sketch here stores `i32` newtypes in the real engine. The failing case is the absent bound. Sketched: `c1-oversized-cell/`. A `[u64; 512]` cell, 4 KiB, is a `ColumnValue` and compiles (passes, `test-output.txt:81`). The planner then clamps its window to `MIN_MORSEL`, 64 records, and the morsel writes 262144 bytes against an L1 budget of 24576 (red, `test-output.txt:82-85`). That is 10.7 times the budget the morsel formula exists to respect, and the formula cannot fix it, because the floor clamp wins. The record's 16-byte bound is what keeps the formula meaningful. This is the trunk drift in 0.1 item 1, with its cost shown.

C2, declared packing. Sketched: `c2-packed-word-race/`. A 1-bit packed column over 64-bit words, written by two threads, each setting its one bit by loading the whole word, or-ing the bit in and storing the word back (a relaxed load and a relaxed store, which loses updates as a plain load and store would; `c2-packed-word-race/src/lib.rs:18-19`). Split at 68 of 136 records, a 4-aligned split like the trunk's `& !3`: 14977 of 200000 frames lost a write (red, `test-output.txt:11-16`; an earlier run of the same sketch lost 3312). Split at 64 of 128, word-aligned: 0 lost (`:25`). So packing is legal only if every window boundary and every per-core split falls on a multiple of `64 / BIT_WIDTH` records. The trunk guarantees neither: windows are `& !3` (`step9_morsel.rs:75`), and the per-core split is `ceil(total / ncores)` (`hilavitkutin/src/resource/bindings/per_core.rs:92-96`). Caveat: the sketch models the read-modify-write itself, because the engine has no packed path to race. It proves that the hazard exists, not that the engine has it.

C3, AoSoA. Not sketched. The record already has it: columns are independent, and the fiber arena co-locates them with static offsets (spec :396-399). A consumer struct is N columns, which is the "Columns, not tables" rule. Nothing to fail that I could find. Arrow's extension types are the prior art for a consumer-named logical type over a physical storage type (Arrow columnar format, extension types, fetched 2026-11-13, `/home/user/.data/fetched/a42a3f16641fab755745d8ba4524a4d27a7d54da7382942d2fffb6815f6e0624.html`).

C4, wide cells. The record forbids them (spec :460-461), and c1 shows why with numbers. One new option that is mine and therefore suspect: derive the bound from the run config instead of fixing it at 16, as `size_of::<T>() <= L1_USABLE / MIN_MORSEL`, which is 384 bytes at the defaults, checked at build. That keeps the morsel formula honest, and lets a consumer with a smaller `MIN_MORSEL` store wider cells. It loses R3's SIMD-lane argument for 16. It is op's call, not mine.

C5, locus. kaski's question, whether a lattice lane may be narrower than 64 bits, is answered upstream by C1 as the trunk stands (an `i16` or `i32` column already works) and by C2 once packing exists. Op's "entirely generic and configurable by user" is met by C1 plus C2 plus the split rule. The format set (16, 32, 64) is kaski's, over that.

C6, layout at the seam. Sketched: `c6-layout-across-seam/`. One source file, three compilations, plain and with `-Zrandomize-layout` at seeds 1 and 2 (rustc 1.98.0-nightly 57d06900f 2026-05-27, `rustc-version.txt`). The `repr(Rust)` cell came out at size 16, 32 and 24, with fields at different offsets in every build. The `repr(C)` cell was fill 0, discharge 4, heat 6, flags 7, tag 8, size 16 in all three (`output-plain.txt`, `output-seed1.txt`, `output-seed2.txt`). Randomisation exaggerates what a plain compiler does. But Rust promises no layout for a default-repr struct between two compilations, and A4's host and extension are two compilations, possibly with two compiler versions. The record's "repr(C) on stored types" (spec :477) is exactly the obligation a c ABI seam needs, and the trunk dropped it.

### 3.4 Composition for C

- Any `Copy` cell up to the bound: C1, as the record has it.
- Sub-byte cells: C2, with the planner rounding windows and per-core splits to `64 / BIT_WIDTH` records for packed columns only.
- Cells that cross an image seam: C6's `repr(C)`, which the record requires for every stored type. A narrower option is a marker required only on columns a slot unit exposes.
- Wide data: interned ids or a side column (spec :461), unless op takes C4's derived bound.

Benches owed (`mock/benches/`): {byte `bool`, packed 1-bit with rounded splits} at 1e5 to 1e7 records, for the bandwidth gain against the rounding constraint. And {cells of 4 to 512 bytes} against the window floor, to price the bound.

### 3.5 Op calls for C

C-1. The value bound.
- a. Restore 16 bytes as the record says.
- b. A bound derived from the run config (C4), checked at build.
- c. No bound, which is the trunk today and needs op to amend R3 and R4.

C-2. Packing.
- a. Not now. BIT_WIDTH stays an unread hook.
- b. BitPacked layout, with the planner rounding packed columns' windows and splits to the word multiple.
- c. Packed columns are single-writer: one core per packed column per phase.

C-3. `repr(C)`.
- a. Restore it on every stored type, by derive or lint, as the record says.
- b. Require it only on columns that cross a seam.
- c. None, with an extension reading only plain primitives.

C-4. Close `decide_the_storage_word_and_the_widest_cell_in_hilavitkutin` and `decide_how_a_consumer_gets_its_own_value_type_in_hilavitkuti` as malformed against `hilavitkutin-api` (0.3), and file C-1 to C-3 in their place. This is the dispatcher's or op's call, not mine.

## 4. What I settled, moved, and could not

Settled, as existence claims each shown once by a sketch, not as design:

- Load-time behaviour can run inside a static schedule without registering a unit, both as a program checked at load (A3) and as a foreign kernel (A4), on the real trunk engine.
- The trunk's resize loses records (b2).
- The trunk's window straddles power-of-two pages (b3).
- A packed column split at a non-word boundary loses writes (c2).
- The trunk accepts a cell that breaks the L1 formula (c1).
- A default-repr cell has no agreed layout across compilations (c6).
- `Library::load` cannot open an unmapped library on linux glibc (A4-1).
- Const names do not cross an image (A4-2, A4-3) and do not resolve on wasm (A4-4).

Moved:

- Part A's question from "may units arrive at load" to "what do compiled units need so that load-time behaviour is fast and correct". The answer is a window slice, the loader fix, a name story and a layout bound.
- Part B from "growth is impossible" to "growth is on the consumer as the record says, but the engine owes it a count parameter and a windowing rule, and the trunk's one resize is broken".
- Part C from "the cell is too narrow" to "the trunk is too loose".

Could not:

- Price anything. Every how-much question in this file is unpriced, and the benches are named in 1.3, 2.4 and 3.4.
- Sketch A2, A5, A6, A7, B1, B5, B6, C3 or C4. For each I gave the failing case from source or prior art, and none of those is a sketch.
- Reach the RimWorld compatibility page in usable form (`/home/user/.data/fetched/188f720f7e347d8c34c7d0d384c1ce6f12f5e00c224db302e3ac44bf5955eb19.bin`, possibly not the page). I did not use it.
- Fetch the Wasmtime call-cost discussion, Paradox's Jomini scripting docs or flecs's pipeline docs. I have search results only and cite them as such.

## 5. What I am least sure of

- Whether A3 per morsel is close enough to typed code for kaski's acts. Everything in part A leans on this, and it is unpriced. If the bench comes back at several times typed even over a window slice, A5 or A6 moves from optimisation to requirement.
- My reading of domain 16 against R6 (A-2). If op reads "changes WU registration" as permitting load-time registration, A8 stops being forbidden and the composition changes.
- c2's race is my own read-modify-write model, not an engine path, because none exists. A different packing implementation, for example atomic `fetch_or` on words, changes the failure from lost writes to contention. The split rule would still be required for determinism.
- c6 uses a nightly randomisation flag. A plain compiler at one version is deterministic in practice, and the risk is across versions, which I did not sketch.
- Whether kaski wants native mod code at all. The demoted tenet said no, and op's 9 October answer prefers a DSL. A4 may be an escape hatch nobody uses, in which case A4-1 to A4-4 matter for the plugin-host layer's other consumers and less for this question.
- The B1 cost figure, 10,000 times at 1e3 live in 1e7 reserved, is arithmetic on the plan's total, not a run.
