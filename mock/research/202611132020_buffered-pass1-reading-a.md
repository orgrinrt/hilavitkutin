# Reading A: what a Buffered column reads in the first pass

Ref read: origin/work/api-revision at eeb7f5557a12 ("chore: close design round 202611131924"). Read-only. Nothing in the clone was edited, checked out or committed. Line numbers below are of the files at that ref.

## 0. Gates

Canon gate: ambiguous, handed back. The governing text says what pass 1 reads in one sense and is silent in the sense that matters. hilavitkutin-api DESIGN.md.tmpl:159-161 says every read "sees the frozen side, which is the store as it stood when the pass began". :650-656 says the boundary runs after the close and "a buffered column a unit of the first pass fills is read from the second pass on". :251-252 says "Both of its sides start as a column starts." So pass 1 reads the starting state, and the starting state is defined by reference to how a column starts. Neither design says how a column starts. That is the gap, and it is not specific to Buffered (section 3). Every shape below is a new position. None is licensed by the governing text, and the call belongs to the dispatcher reflecting on the canon, or to op. I am the first reader. A second independent read is owed before anything is stamped.

Test gate: the whole suite does not build. I exported eeb7f555 with `git archive` into the scratchpad and ran `CARGO_INCREMENTAL=0 cargo test --offline --workspace --no-fail-fast`. Two test targets fail to link and take the run down with them: hilavitkutin-persistence test `archive_str` and hilavitkutin-providers test `smoke`, both on `rust-lld: undefined symbol: __start_hilavitkutin_strings` / `__stop_hilavitkutin_strings` under `--gc-sections`. With those two packages excluded the rest gives 548 passed, 0 failed, 20 ignored. I did not chase the link failure. It is outside this question and may come from the environment, but it is red and has to be recorded as red.

The tests on this surface are setup that helps, and the setup is exactly the hole asked about. mock/crates/hilavitkutin/tests/buffered_column.rs writes the frozen side through the hidden accessor before the first `run` in every test that reads a Buffered: :249, :291, :320, :352, :383, :408, :429, :459, :496. No test runs pass 1 over a frozen side the engine left alone. Two of them misstate their own bodies. :357 asserts `"pass 1 reads what the build left"`, but the build left nothing; the test wrote 10 at :352. :485 is named `a_store_filled_in_the_first_pass_is_read_from_the_second` and asserts at :500 `"pass 1 reads the unfilled store"` equal to 0, after :496 filled it with 0 by hand. That assertion is green because the test wrote the value it checks for. It says nothing about the engine. Every test also runs over StableProvider (tests/stable_provider/mod.rs:29, `vec![0u8; cap]`), which hands out zeroed memory, so even an unseeded test would see 0 and pass by luck of the provider. Under the gate rules I would refuse adjacent work on this footing. I am delivering this reading anyway because the reading is the diagnosis of that defect, and it cannot be fixed from a read-only clone.

## 1. The reviewer's claims, checked

Claim 1, `drain_column` reserves both sides and initialises neither. Holds. mock/crates/hilavitkutin/src/resource/bindings/column.rs:399-400 says it outright ("the drain reserves, it does not initialise"). :401 reserves the first side through `reserve_records` (:30-54), which calls `ColumnStorage::reserve` and takes a pointer, nothing more. :405 reserves the second through `K::reserve_side`, which is `reserve_records` again for Snapshot (:230-236) and Exchange (:280-294). `ArenaColumnStorage::reserve` (hilavitkutin-providers/src/storage.rs:64 onward) calls `provider.allocate` and does not write. `MemoryProviderApi::allocate` (hilavitkutin-api/src/platform.rs:29-36) promises nothing about contents. The api design's `ColumnStorage::reserve` (api DESIGN.md.tmpl:716-720) says reserve and align, and says nothing about initialisation.

Claim 2, `map.rs:59-62` sends every read of a Buffered in a pass to the frozen side. Holds, with the routing done one file over. map.rs:59-62 is `record`, which reads `col.read`. The read impls for `Buffered<T, M>` and `Reach<Buffered<T, M>, N>` (:83-89, :99-105) go through `record` and `window`. `col.read` is filled in by the column ColSelector as `read: self.__frozen()` (dispatch/engine_ctx/col_select.rs:103), and `__frozen` is `K::frozen` (column.rs:353-355). That is `side` for Snapshot (:244-246), and `other` while `turned` is false for Exchange (:305-307). `turned` starts false (:289). So in pass 1 every Buffered read reaches the second reservation, which nothing has written.

The reviewer missed a third read. The scatter reads the frozen side too. `land_compared` (map.rs:182-195) compares the returned value against `col.read` at the record for every buffered write under Differs, and a bare Buffered member in `map` is Differs (api DESIGN.md.tmpl:524-527). So in pass 1 the writer's change mark is decided by comparing against unwritten memory. That mark decides whether the boundary runs (resource/bindings/close.rs:115-125) and whether the store's readers are seeded dirty for pass 2. Pass 1 therefore makes the unit's outputs and the engine's own scheduling decision depend on memory nobody wrote.

Claim 3, no supported API seeds the frozen side between `build` and the first `run`. Holds. The only routes are `#[doc(hidden)]` accessors documented as "Not supported surface": `__frozen` (column.rs:350-355), `__live` (:357-362), `__ptr` (:342-348). `Buffered` registers with `Init = ()` and `HasTrivialCtor` (hilavitkutin-api/src/store.rs:211-216), so registration cannot carry a value either. The positional accessors do not reach a Buffered (api DESIGN.md.tmpl:670; trybuild fixtures `read_buffered_by_position.rs` and `write_buffered_by_position.rs`).

Claim 4, `DESIGN.md.tmpl:648-657` does not say what pass 1 reads. Half right. :650-656 says what pass 1's writes become ("read from the second pass on"). Combined with :159-161 the design does say pass 1 reads the store as it stood when the pass began. What it never says is what that state is. The only sentence about it is :251-252, "Both of its sides start as a column starts", and neither design says how a column starts. The engine design's account of the drain (hilavitkutin DESIGN.md.tmpl:1268-1273, :1328-1330) covers reservation and leaves contents out. So the claim is correct in substance. The design pins pass 1 to the starting state and leaves the starting state undefined.

What this costs in practice, observed. I wrote an uncommitted spike in the scratchpad export (tree/mock/crates/hilavitkutin/tests/spike_pass1_buffered.rs, outside the repository, so not citable evidence under the repo's evidence rule; commit it into a probe directory before anyone cites it). It registers a Buffered with a total writer (`Fill`, writes 7) and a reader (`Watch`), with no seeding, in both modes. Over StableProvider, pass 1 reads 0x0. Over a bump provider whose buffer was pre-filled with 0xA5, pass 1 reads 0xa5a5a5a5. Pass 2 reads 7 in all four cases. So pass 1's output is whatever the memory provider left. Over a provider handing out genuinely uninitialised memory, `ptr::read::<T>` of those bytes is undefined behaviour for every `T`, integers included. Under the current `ColumnValue` that `T` can be a reference, a `NonZero`, a `bool` or an enum (api DESIGN.md.tmpl:289-295, any `Copy + 'static`), where some bit patterns are invalid values.

A shipped artifact already does this. The column swap bench carrier, mock/benches/variants/swap_common/src/column.rs:139-176, builds over `HeapBump` (re-exported from rcm_common, mock/benches/variants/rcm_common/src/lib.rs:124-129). That buffer is `Box<[MaybeUninit<u8>]>`, so it is really uninitialised. The carrier seeds the plain `Column<Slot>` through the hidden `__ptr` (:155-166), seeds nothing on the Buffered middle column, and runs a cold pass at :176. In the Buffered arms, `Consume` reads the frozen middle column in that pass (:130-135) and `Produce`'s Differs scatter compares against it. That is a read of uninitialised memory in a bench the trunk measures. The output settles after a few passes, which is why nobody saw it.

Two smaller drifts in the same code. The safety precondition of `ColKind::bring_in` (column.rs:176-177) says "Both sides hold `count` initialised records". Nothing establishes that at the first boundary: after the drain the frozen side has never been written. The Snapshot copy happens to read only the live side, so the stated precondition is false while the operation is sound. That is accidentally-correct, and the next edit that reads the frozen side in `bring_in` (a per-morsel dirty copy, FIXME 8e2e2326d588 at :221) turns it into UB. Separately, api DESIGN.md.tmpl:166-172 says that under Snapshot "the live side always holds the whole store" and that Snapshot "is right under any writer". That is true only if the live side starts initialised. A partial writer in pass 1 (api :661 allows one) leaves the rest of the live side unwritten, and the Snapshot boundary copies that unwritten memory over the frozen side. Writes through the value function only, which are total over the unit's range, hide this today. The design sentence still claims more than the mechanism gives.

## 2. What the governing documents and op's words say

Governing (stand-in canon, per the brief): the round 202611130617-h6mb doc changelist and the two crate designs.

- api :159-161, :622-627: every read of the pass, by any unit of any rank, sees the store as it stood when the pass began.
- api :650-656: the boundary is the engine's and no unit's; "none sees one early"; "with no exception for rank: a buffered column a unit of the first pass fills is read from the second pass on".
- api :251-252: "Both of its sides start as a column starts."
- api :629-631, engine :48-52: a reader and a writer of a Buffered take no edge, and one unit may be both.
- engine :120-125: the first frame after build is cold and every root is dirty, so every reader of a Buffered runs in pass 1.
- api :676-679: a column whose every writer is at the close holds, for every ordinary unit of a pass, the value it had when the pass began. That is the same pass-1 question for a plain column (section 3).
- api :236-245: `Resource::new(value)` is the existing precedent for a store whose registration carries its starting value, and `Column`/`Virtual` are `HasTrivialCtor` because they carry none.
- Changelist 202611130617-h6mb_changelist.doc.lock.md, section 1 (the frozen and live split): it builds the Buffered from loimu's founding research and gives hilavitkutin "the pair of sides, the frozen read and the live write, ... the swap at the boundary with its dirty mark". The topic file's closing paragraph (202611130617-h6mb_topic.value-form-domains-and-the-buffered-kind.md, last section) says "hilavitkutin holds the buffer pair, its swap and its seed". From context, "seed" there is the dirty seed of the next pass and not an initial value. I read it that way. If it meant the initial value, the changelist never carried it into the design.

Founding record (cited by the changelist as the record it builds on):
- Spec, mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1462-1475: "hilavitkutin does not load data"; the consumer "Materialise[s] external data into hilavitkutin columns before execution" and defines "loader WorkUnits (`type Read = ()`, `type Write = (Column<A>, ...)`)", which are "root nodes". For a plain column the RAW edge puts the loader before its readers in the same pass, so pass 1 is defined. For a Buffered that edge is deliberately absent (api :629-631), and api :655-656 says the loader's fill is seen from pass 2. So the founding mechanism closes pass 1 for Column and does not close it for Buffered.
- Core design topic 202603141800_topic.hilavitkutin-core-design.md:677-680: consumers materialise data into columns "before processing", and the bridge "is saalis's concern, not hilavitkutin's". :684-685 has storage "mmap-backed (with linked-page fallback for no-mmap platforms)". Anonymous mmap is zero-filled on Linux, which may be why nobody noticed. The record does not promise zero, and the fallback path has no such property.
- Founding research (research tier, below the topics), 202603141800_research.loimu-storage.md: loimu's records declare a default per field (`mass: Coord = Coord::ONE`, `velocity: Vec3<Coord> = Vec3::ZERO`, :799-806), "Default record data for R is inserted into the appropriate view" when a node gains a marker (:819), and "hilavitkutin should adopt the record model concept: a trait describing field layout, default values, and type metadata" (:1140-1142). This is the same document the changelist quotes as the source of the frozen and live split (:191-197, :244-251, :826-828). In the source the Buffered was taken from, the buffer pair never existed without a starting value. The port dropped it.

Op's words: silent on the starting state. In the round's verbatim answers (topic file, sections dated 13 November) op pushes for the double buffering to be built from the record ("if the double buffering is in fact in there, then tell the fuckers to focus", 13:35) and asks for the mechanism behind the frozen and live split (13:20). He says nothing about what the frozen side holds before the first boundary. I also grepped the workspace op-responses captures (/root/.local/vibecheck/worktrees/user/.data/op-responses, 1232 files, through 2026-11-12) for initialisation, uninit, zeroing, default values and first frame in a hilavitkutin context, and found nothing that bears on it. The `briefs/fixup/op-answers-*.md` files the round cites are "on the channel" and not in this repository. I could not read them beyond the quotes the round carries.

So the documents establish three things: pass 1 reads the starting state; a Buffered's starting state is a column's; the founding record's way of giving a column data is a loader unit inside the pass. The third does not reach a Buffered's pass-1 reads, and that leaves the gap.

## 3. Scope: the hole is a column's, and the Buffered inherits it

api :251-252 defines a Buffered's start by a column's start, so any Buffered-only answer creates a second definition of "start" with no dependency between the two. The plain column has the same hole wherever a read in pass 1 is not preceded by a writer in that pass:
- api :676-679: a Column written only at the close and read by ordinary units. The rank rule puts the reader before the writer (engine :44-48), so pass 1 reads an unwritten column.
- A Column with no writer at all (a pure input the consumer fills), which today can only be filled through the hidden `__ptr` (swap_common/src/column.rs:155-166 does exactly that for `Column<Slot>`).
The question as posed presumes the fix is about Buffered. By api :251-252 it is about how a column starts, and the Buffered pass-1 read is the most visible case.

## 4. Candidate shapes

For each: what the documents say, runtime cost, type surface, determinism, no_std / no-alloc / const-first, and what breaks downstream.

A. Starting value at registration, written into every reservation at the drain.
`Buffered::<T, M>::new(start: T)` as a `const fn`, with `Init = T`, mirroring `Resource::new(value)` (api :236-245). The drain writes `start` into both sides, or into one side and then a `copy_nonoverlapping` into the other.
For: precedent at api :236-245; the founding research's per-field defaults (loimu-storage.md:799-806, :819, :1140-1142), from the same source as the split itself; it makes api :251-252 mean something; it establishes the `bring_in` precondition (column.rs:176-177) and the Snapshot "live side always holds the whole store" claim (api :166) from build onward; the pass-1 Differs compare becomes defined.
Against: nothing in the governing text. Spec :1462 "hilavitkutin does not load data" is not violated, because one uniform value is no more loading than `Resource::new` is. A per-record generator (B below) gets closer to that line.
Runtime: count stores per reservation once, at build, never in a pass. Zero per-pass cost. On a zero-filled mmap provider it duplicates work the OS did; that is a build-time memset and not worth a special path.
Type surface: one constructor argument. `Buffered` loses `HasTrivialCtor` (store.rs:216), as `Resource` lacks it. Illegal state ("a Buffered with no start") cannot be constructed, because there is no zero-argument constructor.
Determinism: pass 1 becomes a pure function of the registrations and the loaders. The pass-1 change mark becomes deterministic.
no_std / no-alloc / const-first: all held. `const fn new(start: T)`, the value is `Copy`, and the write goes through the existing `ColumnPtr`.
Breaks: every `.with(Buffered::<T>::new())` / `Col::new()` site: hilavitkutin/tests/buffered_column.rs, plan_inputs_from_bundle.rs, plan_inputs_projection.rs, hilavitkutin-api/tests/store_types.rs, access_set.rs, contains.rs, the trybuild fixtures and their `.stderr`, swap_common/src/column.rs:143-150; api :249-252 and the `HasTrivialCtor` sentence at :241-243; engine :1328-1330 (the drain now writes); `StoreDispatch` and `drain_column` must carry `Init` into the drain (column.rs:393-422). Every `__frozen().fill(...)` in the tests gets replaced by the registration value, and the two misnamed assertions become true by construction.
Limit: a uniform value only. A field that starts non-uniform (a heightmap) still enters through a loader unit and is visible from pass 2 (api :655-656).

B. A per-record generator at registration.
`Buffered::from_fn(f: fn(USize) -> T)`. The drain calls `f(i)` for each record and writes both sides.
For: what A gives, plus non-uniform starts, with no host write API.
Against: this edges toward hilavitkutin loading data (spec :1462), and the founding loader-unit path already exists for that. Nothing in the record asks for it.
Runtime: n indirect calls at build. A `fn` pointer is not inlined at the drain, which does not matter at build time.
Type surface: a second constructor.
Determinism: deterministic only if `f` is pure, and the type cannot enforce that (a `fn` pointer can read a static `AtomicU32`). That is a weaker guarantee than A.
no_std / const-first: a `fn` pointer is const-storable. Fine.
Breaks: as A.
Residue: worth having only as a companion to A, if a consumer shows a non-uniform start that a loader plus one cold pass cannot serve.

C. Both sides initialised at the drain from a contract on `T` (zero bytes, `Default`, or an associated const such as `const START: Self`).
For: no registration change. A const on the type fits const-first.
Against: `ColumnValue` is any `Copy + 'static` (api :289-301, "Zero friction for consumers"). Zero bytes are not a valid value of `&'static U`, `NonZero*`, most enums or `bool` beyond 0/1. A zero-fill contract would be unsound for those types unless `Buffered<T>` gets a `Zeroable`-style unsafe marker, which is the `Plain` limit the data-bound sketch already hit (mock/research/sketches/202611130728_data-bound-registration/FINDINGS.md, the `Plain` bound). `Default` is not const-callable on stable Rust, so it breaks const-first. An associated-const trait adds a bound and an impl per value type, and allows one start per type rather than per store. Stores are keyed by type, so the expressiveness difference from A is small, but A's value can be computed at run time (from config) and this cannot.
Runtime: as A.
Determinism: deterministic.
Breaks: a new bound on `Buffered`'s `T`, and on `Column`'s too if api :251-252 is kept. Pushing zeroing into `ColumnStorage::reserve` (api :716-720) would change every column, the plan's CSR pools, and each accumulator, a much larger blast radius for one store kind.
Verdict: dominated by A.

D. Pass 1 typed so that a read is not expressible.
A schedule is built once and runs every pass (engine :1284-1290, schedule-once-reuse), so a pass-1-only restriction cannot live on a unit's read list. It could only live on the scheduler: `build` returns a cold scheduler whose `run` requires every Buffered to be seeded first, which is shape E in typestate clothing. On its own D has no mechanism. Against it: api :622-623 ("read ... by every unit of the pass whatever its rank") and engine :124-125 (cold first frame, every root dirty). Not a candidate alone.

E. A public seeding step between `build` and the first `run`.
`scheduler.seed::<Buffered<T, M>>(...)`, possibly typestated so that `run` exists only once every Buffered in the registration has been seeded (a type-level list of unseeded stores, each `seed` removing one, `run` bounded on the empty list).
For: it turns the tests' `__frozen` seeding into a supported surface; non-uniform data; `replace_value` (api :685-688, engine :1339-1341) is a precedent for a host call between passes.
Against: spec :1462 "hilavitkutin does not load data" and :1467-1469 (loader WorkUnits are the bridge). A host-side bulk write into a column goes straight against the founding data-loading boundary. `replace_value` replaces a resource, never a column, and the swap spec's S3 keeps columns "sized at plan time and never resized". Nothing ratified gives the host a write into a column.
Runtime: n writes, outside any pass.
Type surface: heavy. A second scheduler typestate parameter threaded through `run`, `run_fused` and `run_parallel`, plus a seed accessor per store kind. The typestated form adds trait-solver depth on long registrations (api :1398 area, recursion_limit notes).
Determinism: deterministic if the host is.
no_std: fine.
Breaks: the scheduler's public type and every run entry. Benches would need to call it.
Verdict: the strongest typestate story and the weakest canon standing. I would not stamp it.

F. Pass-1 reads routed elsewhere (to the live side, or treated as plain).
Against: api :629-631 and engine :48-52. No edge between a Buffered's reader and writer means pass-1 reads of the live side race with writes, and a unit that both reaches over and writes the store (api :636-646, `SpreadHeat`) would read its own output mid-sweep. Giving pass 1 RAW edges that later passes lack means two plans. It also breaks "what a unit writes in one pass is what every unit reads in the next" (api :654-655). Rejected.

G. Leading-band writes (PassStart / plan-stage) to a Buffered land on the frozen side, or the engine runs an extra boundary after the leading bands of pass 1.
For: it reuses the founding loader idea and the leading band "is where a pass is set up" (api :550-552); `meta::PlanStage` fires at startup (core design topic :723).
Against: api :653-656 forbids it in so many words: "No unit of the consumer's brings a live side in, and none sees one early ... with no exception for rank". Rejected unless op reopens that sentence.

H. Reads of a Buffered return `Maybe<T>` until the store has been brought in once.
Either per record (a validity bitset) or per store (one "warm" flag, checked once per morsel).
For: no initial value is invented; the type says honestly that pass 1 may have nothing.
Against: it changes `ReadVals` for every pass to cover one pass. api :373 says a Buffered read yields "its `T`, the record's value as the pass began". Per record it costs a bit test per read and blocks vectorisation of the step, which is exactly the case api :504-507 names Buffered for. Per store it does not solve partial seeding. And Differs still needs something to compare against in pass 1 (it would have to treat "absent" as "differs", one more rule).
Verdict: priced against the perf stance for a one-pass problem. Rejected as the primary shape.

I. Leave it as a consumer obligation, documented ("a schedule must not read a Buffered before its first boundary").
Against: `run` is safe, and api :159-161 promises a value. A safe API whose first call can read uninitialised memory is unsound, not under-documented. This is the current state, and the swap bench already violates the obligation nobody wrote down. Rejected.

J. Discard pass 1: the first `run` after `build` is a warm-up whose outputs are unspecified and whose only defined effect is the boundary.
Against: engine :124-125 (the cold first frame runs every root), api :654-656 (pass 1 is a real pass whose fill is read in pass 2), and it leaves the pass-1 Differs compare and mark undefined. It also makes every consumer pay one wasted pass. Rejected.

## 5. What I would stamp, marked as my reading and not a decision

A, applied to every record-count store that is read before any write in its first pass, which by api :251-252 means Column as well as Buffered. One definition of "how a store starts": a value given at registration and written into every reservation at the drain. Specifically:
- `Buffered::<T, M>::new(start)` and `Column::<T>::new(start)`, both `const fn`, `Init = T`. The no-value constructors go, so an unstarted store cannot be constructed. If op wants `Column` to keep its zero-argument form for columns that only loaders write, the honest alternative is a type-level split (`Column<T>` with a start, and a store whose every read is RAW-preceded by a writer, which the plan can check). The split costs a new marker. Silently keeping `Column::new()` keeps the hole at api :676-679.
- The drain writes `start` into both Buffered sides. That establishes the `bring_in` precondition at column.rs:176-177 from build onward, and a one-line invariant at that line should say so.
- Non-uniform starting data stays on the founding path: a loader unit (spec :1467-1469), read from pass 2 for a Buffered as api :655-656 already says, with pass 1 reading the registered start. B is held back as a companion only if a real consumer shows a need.
- Tests: delete the `__frozen().fill(...)` seeding, register the start instead, and add the missing ones: an unseeded pass 1 over a provider that hands out non-zero memory (in the shape of my spike) in both modes; the pass-1 Differs mark equal and unequal to the start; the swap bench's cold pass over its uninitialised HeapBump.
- Fix the drift whatever is decided: the swap bench carrier at swap_common/src/column.rs:139-176 reads uninitialised memory today.

Why A and not E: A has a ratified precedent in the same design (api :236-245) and a founding-research pedigree from the same document the split was taken from (loimu-storage.md:799-806, :819, :1140-1142). It costs nothing per pass, stays const-first and no_std, and puts the invariant in the type, since no constructor makes an unstarted store. E's typestate is attractive, but it gives the host a bulk write into a column against spec :1462-1469, and nothing ratified licenses that.

## 6. What I am unsure of

- Whether op wants the starting state in hilavitkutin at all, or wants it to stay entirely a loader's business (spec :1462 "hilavitkutin does not load data"). If the latter, the only shape left that the governing text admits is none of the above. The design would then have to say that a Buffered read in pass 1 is unspecified, which contradicts api :159-161 and the safety of `run`. That contradiction is op's to resolve.
- Whether extending A to `Column` is in scope. api :251-252 says yes by reference, but it changes a store marker this round did not touch and is a design change with its own blast radius.
- Whether "its seed" in the topic's closing paragraph meant the initial value. I read it as the dirty seed. If it meant the initial value, the round intended something here and did not write it into the design, and that is drift between the round and its own design.
- Whether the swap bench's figures for the Buffered arms are affected. The cold pass is outside the timed region and the carrier settles before timing, so probably not. It is still UB in a measured artifact and has to be fixed before the numbers are cited.
- The link failures in the suite (`__start_hilavitkutin_strings`): environment or real. Not investigated.
- My spike is uncommitted scratch and is not evidence under the repo's rule. Everything above that matters rests on the file:line reading. The spike only shows how the hole looks.

A second independent reading is owed before any of this is stamped. If the two readings disagree, the call goes to op, not to a third reader.
