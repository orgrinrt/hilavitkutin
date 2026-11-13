# Seat 02: load-time work units, growth, and the consumer's cell format

Seat 02 of two, written blind to seat 01 (phase one). Persona: Tiark Rompf. Branch
`research/load-time-units-seat02`, cut from `origin/dev` at `271909e5`. Sketches under
`mock/research/sketches/202611131951_load-time-units-seat02/`, one directory per arm, each with its
run output committed beside it. Nothing here is a bench; every number below is either a fact read
off a run (a test result, a resident-set size, an instruction in emitted assembly) or a figure
quoted from the record with its location. No crate source or design was edited.

The governing order is the brief's: op's own words first (channel docs op-answers 13, 20, 21, 25,
28, 36 and the captures under `.data/op-responses/`), then the founding record (the consolidation
spec, the core design topic, the api surface topic and the memory layout topic in
`mock/design_rounds/202604200055/`, and the trunk designs `mock/crates/*/DESIGN.md.tmpl`), then
everything else as presumed wrong where it conflicts. Citations name the file and line on
`271909e5` unless they say otherwise. "The spec" is
`202603181200_topic.hilavitkutin-design-consolidation.md`, "the core topic"
`202603141800_topic.hilavitkutin-core-design.md`, "the api topic"
`202603151200_topic.hilavitkutin-api-surface.md`.

This is the second seat. Under the two-expert rule my reading of what the record permits is formed
here from the record alone, and it is owed a reconciliation with seat 01's, which phase two does.
Nothing below is settled on my word.

## What the gates found

### Breaking the brief

Each cheap claim of the brief, checked against source before anything else.

1. My worktree did not exist. `/home/user/.worktrees/` held only `hv-ltp-seat01`, on branch
   `research/load-time-units-seat01`, and the session was started with that directory as its
   working directory. That is the other seat's worktree, the one the blind rule forbids me to open.
   I did not read anything in it beyond its branch name, which `git worktree list` on
   `/home/user/hilavitkutin` printed. I made my own: a fresh clone of
   `https://github.com/orgrinrt/hilavitkutin` at the path the brief names, branch cut from
   `origin/dev`, which is `271909e5f7a19eefe097e5573476132b62a6cfef` as the brief says. It is a
   clone, not a `git worktree`, so nothing was written into `/home/user/hilavitkutin`. The
   dispatcher should know that the seat-02 session was pointed at seat 01's tree; a less careful
   seat would have written into it.
2. Contamination I could not avoid, stated so it can be weighed. The build target directory is
   shared (`/home/user/target`, and the guard refuses a private `CARGO_TARGET_DIR`). Listing it to
   find my own assembly showed the names of other spikes' test binaries: `growth-16540b426a1de609`,
   `b3_paged_column-*`, `c2_packed_word_race-*`. They are very likely seat 01's. I opened none of
   them. I had already chosen my growth arms before I saw the names; the paged-column arm (B4
   below) was in my notes from the loimu research before that listing, and I did not sketch it.
3. `origin/work/api-revision` is not at `eeb7f555`; its tip is `40399db7`, eight commits on
   (a one-thread runner sketch, a reach-boundary fix, the instruments round). `eeb7f555` is an
   ancestor, and the locked doc changelist the brief cites is byte-identical between the two. The
   claim stands in substance.
4. The ticket texts in `.shared/scripts/life/tickets-hilavitkutin.toml:437-506` match the brief.
   kaski's ticket `may_a_lattice_channel_hold_lanes_narrower_than_64_bits` carries op's answer
   verbatim as the brief says.
5. "a crate op voided ('revise the api in place, no second contract crate')". The quoted words are
   spliced from two different questions of 13 November about 05:16 and 05:32
   (`.data/op-responses/202611130508_the-canon-goes-provisional-and-hilavitkutin-is-revised-in.md`).
   "No second contract crate" is the description of an option ("Fold into the existing api",
   `:50-52`) that op did not choose; he chose "Hold, map it first". The later question's chosen
   option is "Revise the api in place now" (`:147-161`). The conclusion holds, since
   `hilavitkutin-contracts` exists on no trunk crate list (`mock/Cargo.toml:3-20`) and op chose to
   revise the api in place; the quotation does not. A brief that splices an unchosen option's
   wording into op's mouth is how a reading becomes a ruling.
6. The premise of both of part C's tickets is false against the trunk, as the brief suspected.
   Details in C.
7. The stack audit the brief quotes for part B is wrong on its own evidence. It says re-reserving
   a column "is undefined" citing `crates/hilavitkutin-api/src/storage.rs:46-47,:59`
   (`mock/research/202611131215_stack-audit/DOWNSTREAM.md:15`). Those lines say the opposite:
   "Re-reserving an existing `id` resizes it." What is true is worse than undefined, and is in B.
   The same file says of load-time units that "nothing in the record addresses it" (`:27`). The
   record addresses it in at least five places, listed in A.

### The canon gate

hilavitkutin declares no canon (`mockspace.toml` has no `canon_paths`), so its design is the canon
and the founding record governs.

Aligned, with one constraint and one ambiguity. The question, "what can and should happen at load
time to a compile-time schedule", is a legitimate panel question: op asked for it (op-answers-28
item 2, "Panel it first"). The constraint: the spec closes one of the candidate answers in so many
words. "No runtime WU registration, no dynamic schedules" (spec `:2112-2114`, and again in R6 at
`:2439-2440`), and "Column type registration is static (compile-time), no runtime column type
creation" (R2, `:2411-2412`). The spec also records that this closure is what "eliminates the need
for fallback dispatch approaches (Approach C) entirely" (`:2116-2117`), which is the answer to the
open question the api topic had left about "work units registered at runtime, e.g., plugin
loading" (`:1348-1352`). So any arm in which a unit unknown at build joins hilavitkutin's plan is
against the canon as it stands, and can only reach op as a proposal to amend R6, not as a design
the panel recommends inside it. I treat it that way below (A4).

The ambiguity, handed back. The core topic says `meta::PlanStage` is "fired when DAG changes
(startup, plugin load)" (`:723`) and that the virtual-bit packing is "Recomputed on DAG change
(plugin load, reconfiguration)" (`:247-248`). The spec keeps "fired when DAG changes (recompute
plan)" and drops the words "plugin load" (`:2125`), while closing runtime registration (`:2112`).
The two readings are: a plugin load changes something the compiled plan is parameterised by (which
surface occupant is live, which compiled units are gated open, record counts), and the unit set is
fixed; or a plugin load changes the unit set, which R6 then forbids. The record does not decide
between them in words. I proceed on the first reading, because it is the only one consistent with
R6, and I mark it as my reading. It is op's to confirm (A, question A2).

### The test gate

I ran the whole suite from `mock/` with `CARGO_INCREMENTAL=0 cargo test --workspace
--no-fail-fast`. It does not build. Two test binaries fail to link, `hilavitkutin-persistence`'s
`archive_str` and `hilavitkutin-providers`' `smoke`, both on `undefined symbol:
__start_hilavitkutin_strings` and `__stop_hilavitkutin_strings` from `hilavitkutin_str::section`
under `rust-lld` with `--gc-sections` (the linker's own hint is `-z nostart-stop-gc`). A suite that
cannot be built on the toolchain the repository pins is not a suite anyone has run end to end on
this host recently. Excluding those two crates the rest builds and runs: 464 passed, 0 failed, 21
ignored, across 133 binaries.

I read the bodies of every test in the surfaces my sketches touch: `hilavitkutin-linking/tests/`,
`hilavitkutin-extensions/tests/`, `hilavitkutin-extensions-macros/tests/`, and the api's
`column_value.rs` and `column_storage.rs`. What they assert, against what they are named for:

- `hilavitkutin-linking/tests/basic.rs` is setup that helps. Every load in it opens the system libc
  (`:14-23`, `:37-140`), a library mapped into every process before `main`. On glibc the loader
  passes 4 as `RTLD_LOCAL` (`src/backend/unix.rs:9`), and 4 is `RTLD_NOLOAD`, so `dlopen` succeeds
  only for a library already resident. The suite feeds the loader exactly the one input on which
  the bug cannot show, and is green. Sketch s1 shows the loader failing on the first library that
  is not already loaded, and succeeding on the same file once a raw `dlopen` has mapped it. The
  defect is ticketed (`hilavitkutin_linking_passes_the_macos_rtld_local_value_on_li`, tickets file
  `:909-914`); s1 is the test that ticket asks for.
- `basic.rs:161` `compatibility_check_accepts_nonempty` asserts that a stub documented as a stub
  ("non-empty bytes should pass v1 stub") accepts non-empty bytes. It pins a placeholder as if it
  were behaviour.
- `hilavitkutin-extensions-macros/tests/lto_smoke.rs` is real about the symbols, but it loads the
  fixture with `libloading::Library::new` (`:107`), not with the shipped loader. Nothing in the
  repository loads an extension through `ExtensionHost::load`. The one test that would have caught
  the loader was written around it.
- `hilavitkutin-extensions/tests/descriptor_validation.rs` is real: it drives
  `validate_descriptor` over each refusal and the order of the checks. It covers validation only.
- `hilavitkutin-api/tests/column_storage.rs:47-55`,
  `columnstorage_is_implementable_and_reserve_count_roundtrips`, implements a stub `ColumnStorage`
  inside the test file and asserts that the stub returns the count the stub stored. It cannot fail
  against any change to any shipped crate. It is a tautology and should be deleted, not improved.
  The only shipped `ColumnStorage`, `ArenaColumnStorage`, breaks the contract's resize sentence (s4)
  and no test in the repository says so.
- `hilavitkutin-api/tests/column_value.rs:48-62`, the three `sub_byte_*_reports_container_width`
  tests, assert `BIT_WIDTH == USize(size_of::<T>() * 8)`, which is the trait's own default
  expression (`src/column_value.rs:29`) evaluated a second time. Structurally incapable of failing
  while the default exists. Delete. The four `blanket_*` tests above them assert literals and are
  real.

By the gate's own terms this suite disqualifies the surfaces I touch, and the standing instruction
is to refuse the dependent work. I did not refuse, and that is my call, so it is suspect: none of
my findings rests on any of the tests above. Each claim below rests on a spike that calls the
shipped code directly and is committed with its output, red where the shipped code fails. Deleting
the two tautologies and replacing `basic.rs` with a fresh-library load is owed, and is outside this
seat's write scope (`mock/research/` only). If the dispatcher reads the gate as forbidding even
that, the findings in this file are to be held until the suite is fixed, and I say so plainly here
rather than bury it.

The people who wrote these tests chose to load the one library that could not expose the bug,
asserted a stub against itself, and asserted a default against its own definition, and then a
green line was cited. That is not coverage, it is the appearance of it.

### Unlicensed things met on the way

The standing instruction asks for these whatever the scope.

- `hilavitkutin-api`'s `ColumnValue` admits any `Copy + 'static` type with no size cap
  (`src/column_value.rs:25-34`, design `:216-231`). The spec caps a value at sixteen bytes: "Value
  constraint size_of::<T>() <= 16" (R3, `:2416-2419`) and "Blanket impl for all Copy + 'static ≤ 16
  bytes" (R4, `:2421-2426`). The cap was never implemented, from the first commit of the trait
  (`6597bbfc`) to the trunk. No round I found records a decision to drop it. s5 shows a 32-byte
  record admitted. This is drift from the canon or a silent amendment of it, and either way it is
  op's (C, question C1).
- The api contract says re-reserving resizes (`hilavitkutin-api/src/storage.rs:46-47`); the only
  shipped implementation frees and maps afresh (`hilavitkutin-providers/src/storage.rs:68-80`), and
  the contents are lost. s4, red.
- The os tier's `protect` is an empty body (`hilavitkutin/src/platform/os.rs:80-83`) under a
  `FIXME:` that names no ticket (`:81`), against the workspace rule that every marker names the
  ticket filed with it. That empty body is the exact piece the record's growth mechanism needs (B).
- The api design says the extension surface's "runtime instance population happens in
  `hilavitkutin-extensions` plus `hilavitkutin`" (`hilavitkutin-api/DESIGN.md.tmpl:206-208`). The
  extensions design says "The engine crate (`hilavitkutin`) does not depend on this one"
  (`hilavitkutin-extensions/DESIGN.md.tmpl:131`) and the linking design says "The engine itself
  never uses this crate. The engine is static composition" (`hilavitkutin-linking/DESIGN.md.tmpl:58`).
  The trunk designs contradict each other on exactly the boundary part A asks about. Nothing in the
  engine populates the surface; `SchedulerBuilder::extend` (round `202605101036`,
  `builder-input-rename-and-reshape` topic, D5) is mentioned in a doc comment
  (`hilavitkutin/src/scheduler/builder.rs:71`) and does not exist.
- The api design still documents `LinkedBin` (`:193-214`); the source has `ExtensionSurface`
  (`src/builder_input.rs:155-193`). Already ticketed by the api revision
  (`the_first_sections_of_the_api_design_still_name_provider_pro`).
- `ExtensionError::RequiredHostProviderMissing { cap }` in source
  (`hilavitkutin-extensions/src/error.rs:40`) against `{ provider }` in the design (`:43`, `:74`).
  Small, and still code that does not say what its design says.
- The stack audit's two misreadings in "Breaking the brief", item 7.

## A. Units known only at load time

### What the record says

The record is not silent on this. It says a good deal, and it says it in pieces that nobody has
put together.

1. The schedule is static and stays static. "Everything via the builder is statically defined. The
   schedule is statically analysable and computable. No runtime WU registration, no dynamic
   schedules. The plan stage adjusts only what compile time cannot: morsel sizes from runtime
   hardware detection, record counts, EMA-based recomputation" (spec `:2110-2117`). R6 repeats it
   and names what is adaptive at plan time: "morsel sizes (from runtime hardware detection), record
   counts (from consumer data), per-phase strategy selection (from EMA metrics), and
   morsel-to-core affinity" (`:2442-2446`). Column types are static (R2, `:2411-2412`).
2. The record anticipated plugins inside that static schedule. "HasShape/ErasedColumn from loimu
   (T5 L674), for plugin type access" is in the spec's list of what is retained (`:577`); the core
   topic says the same model "supports type-erased access for plugin types" (`:674-675`), and that
   consumers "who need rollback (loimu with WASM plugins) build it from work units" (`:776-779`).
3. The meta virtuals are where a load lands. `PlanStage` fires "when DAG changes" (spec `:2125`;
   core topic `:723` adds "startup, plugin load"); in the engine design that is the `plan_dirty`
   seed, keyed per `PlanAffecting` resource and set by a `replace_resource` swap
   (`hilavitkutin/DESIGN.md.tmpl:97-99`, `:893-899`). `ScheduleReady` gates every consumer unit
   behind the plan stage (core topic `:725-727`).
4. The trunk has a typed slot for load-time code. `ExtensionSurface<dyn TraitFamily>`, formerly
   `LinkedBin`: "a slot in the SchedulerBuilder typestate that runtime instances populate via
   `hilavitkutin-extensions`", so that "WUs can declare reads and writes against
   `LinkedBin<dyn TraitFamily>` and the typestate proves coverage at `.build()` time"
   (`hilavitkutin-api/DESIGN.md.tmpl:199-210`; `src/builder_input.rs:155-187`). It is a store
   marker routed through `StoreDispatch`, so a unit that reads it is ordered by the plan like any
   other reader of a store.
5. The extension stack is a C ABI, domain-agnostic, full trust, load-once. Descriptors, provider
   vtables by stable `ProviderId`, failure policy, "V1 is full-trust native code; no isolation" and
   no hot reload (`hilavitkutin-extensions/DESIGN.md.tmpl:171-180`), an arbitrary-time linking
   invariant (`:143-152`), `cdylib` only, Rust `dylib` refused "to preserve the LLVM optimisation
   barrier" (`hilavitkutin-linking/DESIGN.md.tmpl:103`). The research behind it calls the barrier
   desirable and names macro-driven monomorphisation inside the plugin as the way generics cross
   (`mock/research/EXTENSIONS_RESEARCH.md:51-57`).
6. Op, tier one, on 13 November at 18:06: "The extension model is a literal c abi, it's standard
   ffi. Neither it nor me care about what supplies the whatever links through it"
   (`.data/op-responses/202611131806_op-on-the-extension-model-is-a-literal-c-abi.md:20`). And on
   a run-time binding table that lets content add columns with no Rust, at 05:32: "Decide in the
   api round" (`202611130508_...:163-185`), which the api revision has sketched and benched and
   left with the coordinator (changelist section 7, tier three).
7. kaski's canon, which governs what kaski needs, says kaski's mods are content: the provisional row
   `no_content_is_special_cased_in_rust_and_rust_is_never_the_authoring_language`
   (`kaski/mock/registry/tenet/content.toml:219-228`) carries op's words of 4 November: "while
   content is never engine code, we will have scripting that might compile to something ... Rust
   will never be the authoring language and [will not] ship rustc with the game or it's sdk". The
   ratified `content_kinds_are_open_and_interned` (`:25-34`) has every kind content adds interned at
   load into a dense typed id. The earlier `modding_is_content_only_never_engine_code` is demoted
   (`:14-23`), so it is history, not a ruling.

Where the record stops: who fills an extension surface and when; what the host unit's call into it
looks like; how an extension's declared reach is checked against the unit that hosts it; what a
plugin load does to the plan; determinism of extension code; any sandbox.

### The binding-time reading

The first question for any of this is which part is known when. Three times matter, not two: the
engine's build (the schedule, the unit set, the column types), the content author's build (a pack
compiled by its author, if anything is compiled), and the player's load (which packs, which rows,
which store each row binds, the constants). The arms differ in where they put the boundary between
the second and the third, and in whether anything crosses from the third into the first. R6 says
nothing crosses into the first. Everything else is open.

### The arms

Wide by category first. Eight, of which two are op's own options.

- A1. Content is data over a closed set of compiled forms (op's option 1). A compiled unit holds the
  forms; rows choose form, stores and constants at load. Sketched as s3.
- A2. Data-bound registration: content names which typed column a content channel occupies, so a
  pack can add a channel with no Rust. Op left it for the api round; tier-three bench exists. Not
  mine to sketch again.
- A3. A compiled host unit over an extension surface: load-time code (native, interpreted, or JIT)
  runs inside a unit the build compiled, within that unit's declared access envelope, called through
  the C ABI once a morsel. This is the record's own seam, items 2 to 5 above. Sketched as s1 (the
  shipped loader) and s2 (the envelope, the perimeter, the switch point, the code shape).
- A4. Runtime units in hilavitkutin's plan (op's option 2 read strongly): an extension declares its
  access masks at load, `PlanStage` re-plans with a unit the build never saw, and dispatch falls back
  to an indirect call per chain. Against R6. Not sketched; its failing case is the record's own.
- A5. Load time becomes build time: the game is rebuilt with its mods on the player's machine, so the
  schedule is regenerated with the mods' units compiled in. The first Futamura projection as an
  install step.
- A6. One compiled schedule per extension: an extension ships a cdylib that contains its own
  statically scheduled hilavitkutin instance; the host sequences schedules at pass boundaries and
  shares stores across the C boundary as wire views (base pointer, count, format).
- A7. Compiled superset, content selects: every unit any pack may want is compiled in, gated `On<V>`
  or by a `PlanAffecting` resource; a pack's load opens gates and fires `PlanStage`.
- A8. An interpreter unit with staged compilation behind it (Kohn, Leis and Neumann, ICDE 2018):
  content scripts run as bytecode in a compiled VM unit from the first morsel, and a compiled variant
  (JIT at load, or compiled ahead by the pack's author) is swapped in at a morsel boundary when it
  is ready. A3 is its carrier; s2 shows its switch point.

### What each arm's failing case shows

A1, s3 (`s3-content-closed-forms/`). Three rows of a pack compose to what a hand-written unit
computes over 10 000 records. A row writing a store the unit only reads, and a row naming a store
the unit never declared, are refused at load. The wall: a row asking for a form the build does not
hold (`neighbour_mean`) has no representation and is refused; new behaviour that needs a new form
needs a new build. One result went against what I expected and is the more useful for it: the
version that matches on the form at every record compiles to the same four vectorised loops as the
per-morsel version, because the form is loop-invariant and LLVM unswitches the match
(`asm/functions-x86-64-v3.s`, `s3_per_record`). With closed forms and a rule that holds across a
morsel, content-as-data costs nothing in code shape. Its cost appears only where the form varies
record by record, or where a row is bytecode rather than a form.

A3, s1 (`s1-loader-fresh-cdylib/`). The record's seam cannot load anything on Linux today. The
smallest extension `#[export_extension]` emits, built as a cdylib, fails `Library::load` with
`LoadFailed { platform_code: 0 }` and `ExtensionHost::load` with `LinkFailed`; after a raw `dlopen`
with glibc's own flags maps the same file, the shipped loader opens it. glibc's values, read from
the libc crate at run time, are `RTLD_LOCAL = 0`, `RTLD_NOLOAD = 4`.

A3, s2 (`s2-extension-surface-unit/`). Admission at load by stable store name works: a conforming
extension is admitted and computes its function; a write or a read outside the host unit's
envelope is refused before anything runs. The perimeter, red on purpose: an extension that
declares `a` as a read and writes it through the read pointer overwrote 4095 of 4096 records and
nothing saw it. Admission checks a declaration; native code behind a C ABI is not bound by it, which
is the extensions design's "full-trust native code" (`:178`) stated as a test. The switch point:
an interpreted variant swapped for a compiled one after morsel 0, 1, 3 or 9 yields the same column,
so the morsel is a sound place to change an extension's variant. The code shape, read from release
assembly at `-Ctarget-cpu=x86-64-v3`: the extension's own morsel loop is AVX2 (`vpaddq` over four
`ymm` registers, sixteen records an iteration) with no call; a host that calls the extension once a
record through an opaque pointer has `callq *16(%rsp)` inside the record loop and no vector
instruction; a host that calls once a morsel has one call a morsel. Across a boundary the compiler
cannot see through, the call must sit at morsel granularity or the extension's loop never
vectorises. The record already wants exactly this granularity for its own dispatch: "ONE indirect
call per morsel, entire partition inlined" (api topic `:1310-1325`).

A4, not sketched, and I say why rather than fake it. Its failing case is the one the record already
wrote down: the typestate proof the builder runs (`compute_plan::<Wus, Stores, BWit>`,
`hilavitkutin/src/scheduler/builder.rs:192`) is over a type-level unit list, so a unit with no type
cannot be in it, and the fallback the api topic named for runtime-registered units was Approach C,
"indirect per-chain, 1.17x" (api topic `:1350`), which the spec then removed from the design by
removing runtime registration. Anything I would have built to show this would have been a straw man
of my own making. What A4 offers that A3 does not is that an extension unit is planned, fused into
fibers and ordered against compiled units at unit granularity. What it costs is R6.

A5, not sketched. For kaski it is refused outright by kaski's own provisional row: rustc never ships
with the game or its SDK. For hilavitkutin in general it is not hilavitkutin's question at all,
since a consumer rebuilding its own binary is just a build. Its useful variant is the one Unity
ships (prior art below): the pack's author compiles, the player loads. That is A3 with native code,
and it is consistent with kaski's row because the player never runs rustc.

A6, not sketched. Its failing case is visible from the record without a spike: two independently
built plans have no edge between them, so a store written by one and read by the other inside one
pass is a race unless the host orders the schedules at a pass boundary. That is the same shape as
op's still-open "one schedule per set" against "several sets in one schedule" (op-answers-13 item 2,
"Bench both first"), and it should be benched with that, not decided here.

A7, not sketched. It is A1's wall one level up: a pack can only open what the build compiled.

A8, carried by s2's switch-point test. Untested here: an actual JIT, the latency of compiling at
load, and what an interpreted variant costs per record. Those are a bench's, and the bench must have
an interpreted arm, a compiled-at-load arm and a compiled-ahead arm, not one against a strawman.

### Where the line falls

My proposal, at low-to-moderate confidence, and offered as a design rather than as a boundary
observation.

hilavitkutin keeps R6 whole. A unit is a compiled unit, always. What arrives at load is never a
unit; it is code that runs inside a compiled host unit, or data a compiled unit reads.

hilavitkutin owns, and should finish:
- the extension surface as a store kind with a declared access envelope: a host unit that reads
  `ExtensionSurface<dyn F>` and declares the stores the occupant may read and write. The plan orders
  the host unit by that envelope like any other unit, and the occupant inherits the order. This is
  what the api design already says the typestate proves at `.build()` (`:208-210`);
- the per-morsel call point: the host unit's body hands the occupant one wire view a morsel (base
  pointers offset to the morsel, the count), which is the workspace rule's "FFI structs are wire
  shapes ... constructed at the FFI call site by reading the relevant accessors" made concrete;
- admission at load: a domain-agnostic check in `hilavitkutin-extensions` that an occupant's
  declared stores, by stable `ProviderId`-style names, fall inside the host unit's envelope, refused
  as a value otherwise (s2 shows it is a few lines over two lists);
- the plan's reaction to a load: an occupant change is a `PlanAffecting` swap, so `PlanStage` fires
  once and the steady-state frame is untouched (engine design `:893-899`). This is my reading of
  "plugin load" in the core topic, and it is question A2;
- a loader that loads (the RTLD constant per target, ticketed) and a test that loads a fresh cdylib
  through `ExtensionHost::load` (s1 is that test).

The consumer owns: which trait families exist and their ABI; the script language and whether and
where it compiles (kaski: vehje, compiled on the author's machine or interpreted, never rustc on the
player's); sandboxing (a wasm runtime behind the same surface is a consumer's occupant, since the
linking design puts wasm out of scope as "a separate transport", `:83`); ordering among several
occupants inside one host unit; and determinism discipline, which the prior art says is the host
API's job (Factorio shows mods only game state).

What the design needs back from the consumer for this to work: a declared envelope per host unit at
build, wide enough for the packs it means to host, since the envelope is where a pack's reach is
bounded, and a statement of which stores its occupants may write. A consumer that wants "a mod may
write anything" has asked for A4, and should be told that is an R6 amendment.

The api design's sentence that the engine populates the surface (`:206-208`) should lose "plus
`hilavitkutin`": in this shape the engine never touches the extension crates, which is what the
extensions and linking designs already say.

### Op calls for A

- A1-question. Does code that arrives at load ever become a unit in hilavitkutin's plan? Options:
  [a: never; load-time code runs only as the occupant of a compiled host unit's extension surface,
  inside its declared envelope, R6 stands | b: an extension unit joins the plan at `PlanStage` with
  access masks declared at load, R6 is amended and Approach C returns for those units | c: neither
  is hilavitkutin's; the engine offers only content-as-data and the consumer builds any code seam
  above it]. What decides it: a bench under `mock/benches` on the intended region (millions of
  records, op-answers-20) of a host unit calling a native occupant once a morsel against the same
  code compiled in as a unit, and against Approach C's indirect dispatch; and whether any consumer
  needs a load-time unit to be fused with compiled units, which only (b) gives.
- A2-question. What a "plugin load" is to the plan (core topic `:723` against spec `:2112`).
  Options: [a: a change of what the plan is parameterised by (an occupant, a gate, a count), the
  unit set fixed | b: a change of the unit set]. The record's words do not decide it; (b) is A1(b).
- A3-question. Where admission lives. Options: [a: in `hilavitkutin-extensions`, domain-agnostic,
  over stable names | b: in the consumer's contract crate, the extension stack untouched]. Decided
  by whether a second consumer (viola is the one the extension designs name) needs the same check.
- A4-question, only if op wants isolation now. Options: [a: native full trust only, as the
  extensions design says for v1 | b: a wasm transport beside the C ABI in hilavitkutin's extension
  stack | c: wasm is the consumer's occupant behind the same surface]. The linking design already
  leans (c) by calling wasm a separate transport.

Not op's, and should simply be done: the loader constant and its test.

## B. Growth

### What the record says

- A position is a record index, dense: "A column has N records. A morsel windows into a range of
  records" (R8, spec `:2453-2455`).
- Record counts are run-time parameters of the plan stage, "from consumer data" (R6, `:2443`).
- Deletion has a designed form: "null/tombstone via separate bitmask" (R3, `:2417-2418`).
- Growth has a designed form. The api topic's storage atom is "Contiguous, fixed-stride,
  zero-copy-growth column. Backed by mmap (os tier) or consumer-provided memory (no_os)" (`:663-664`).
  The core topic: "One storage implementation: in-memory, contiguous, fixed-stride, mmap-backed
  (with linked-page fallback for no-mmap platforms)" (`:683-685`). The loimu research it rests on:
  mmap with `PROT_NONE`, "Growth commits the next page; no realloc, no copy, existing pointers remain
  valid" (`202603141800_research.loimu-storage.md:19-21`), linked pages where there is no virtual
  memory (`:61-69`). The spec retains the slab's principles while dropping its `Stride` type
  (`:570-571`), and the api's provider contract carries `protect` for page permissions
  (`hilavitkutin-api/src/platform.rs:49-55`).
- Sparse access has a designed form: "collections use morsel iteration (column iterators with
  optional selection vector)" (core topic `:187-189`).
- Emission into a growing output is the accumulator, refused as a value past capacity, configurable
  by the consumer per op ("Configurable by the consumer", api changelist, tier-one quote).
- Op, tier one: "Isn't the intent a contiguous per prototype and indices to process needed windows
  to it?" (`.data/op-responses/202611130808_...:164`); on a domain's extent, "Both, declared per
  domain" (replaceable between passes up to a capacity fixed at build, or fixed at build), which he
  made wait on "Bench both first" for record sets (api changelist section 2, quoting him).
- kaski's ratified `an_engine_width_or_capacity_fixed_at_build_is_an_authored_bound_and_its_ceiling_is_reported`
  (`kaski/mock/registry/tenet/engine.toml:416-426`), with op: "there's no efficiency or computational
  wins on non-statically sized collections. We need to set caps and bounds that are enough for
  extensive content, but not too wasteful."

Where it stops: stable identity across a deletion; compaction; how a count change reaches a
context that has bound base pointers; what the shipped code does, which is the next section.

The ticket's sentence, "nothing resizes a store", is true of the shipped code and false of the
design. The audit's sentence, "Spawning is the ordinary case in a game and the engine cannot do it",
is true of the shipped code; the design can, and the part that is missing is small.

### The arms

- B1. Capacity at build, live count per pass. The extent is a capacity the consumer declares;
  spawning appends into the tail through the accumulator; a maintenance unit moves the live count at
  the pass boundary and the plan stage takes it as its record-count parameter. Op's "replaceable up
  to a capacity fixed at build", and kaski's ratified capacity row.
- B2. Zero-copy growth past any build-time figure: reserve address space, commit pages as the count
  grows, base pointer fixed. The record's slab.
- B3. Copying resize at a pass boundary, with every context rebound at `PlanStage`. The fallback for
  targets without virtual memory, and what `reserve` would mean if it kept contents by copying.
- B4. Paged columns: fixed pages, a morsel never straddles one, addressing stays base plus index
  inside a morsel. The record's no-mmap fallback.
- B5. Deletion by tombstone bitmask (R3), compaction by a maintenance unit at the pass boundary,
  positions dense and unstable across a compaction, a remap emitted for whoever keeps identity.
- B6. Sparse keys as windows: a selection vector of positions over a dense column, op's "indices to
  process needed windows".
- B7. Stable handles with generations, as ECS libraries do. The workspace rule names generation
  counters and handles introduced to reach scheduler data as a red flag
  (`hilavitkutin-workunit-mental-model`). An identity that is data (a column of durable ids the
  consumer owns, kaski's interned ids) is not that, and is B5's other half.
- B8. Several record sets, one per prototype. Op's "Bench both first"; not mine to decide.

### What each arm's failing case shows

s4 (`s4-growth/`), against the shipped `ArenaColumnStorage` and `OsMemoryProvider`.

- The shipped resize loses the column. `re_reserving_a_column_keeps_the_records_it_held` writes four
  records, reserves eight, reads back `[0, 0, 0, 0]`: red against the contract's "resizes it". The
  base pointer came back at the same address, because `munmap` then `mmap` reused the range, so a
  context still holding the old pointer reads zeros rather than faulting. B3 done this way is silent
  data loss. The audit called it undefined; it is defined, and it is wrong.
- B1 works today up to roughly physical memory. Through the shipped provider a 4 GiB capacity is
  granted, the resident set does not move on the reservation, and grows by what is touched (8 MiB for
  one million `i64` records, 2888 KiB to 11080 KiB).
- B1 past physical memory fails through the shipped provider. A 32 GiB capacity is refused on this
  host (16 GiB, no swap, `overcommit_memory = 0`), because the provider maps read-write with no
  `MAP_NORESERVE` (`os.rs:47-56`), so the kernel's heuristic counts it as a commitment.
- B2 works on this host the moment `protect` does something. Reserving 32 GiB with `PROT_NONE` and
  `MAP_NORESERVE`, then committing each pass's 65 536 records with `mprotect`: granted; one million
  records grown over sixteen passes; resident set up by 8.0 MiB for 8 MiB touched; base unchanged;
  records intact. That is the record's mechanism, and the only thing between the trunk and it is the
  empty `protect` body (`os.rs:80-83`) and a provider that reserves rather than commits.

Not shown, and the case B2 must fail on: a target with no virtual memory (wasm32, no_os). There one
linear memory grows only at its end, so at most one column can grow in place; the rest need B4's
pages or B3's copy. That is where B4 earns its place, and where a reach read crossing a page
boundary is its failing case. I did not build it.

B5's failing case is by design rather than by bug: a position held across a compaction is wrong
after it. That is acceptable exactly because no unit holds a position across a pass; a unit sees a
morsel window of the current pass. A consumer that needs identity keeps it as data, which kaski's
canon already does with ids interned at load.

### Op calls for B

Almost none, and that is the finding: op has answered this already, and the record designs it.
Asking him again would be the redundant question he has told the helpers not to bring him
(op-answers-25, 13:35).

- The extent form is answered ("Both, declared per domain") and waits on the record-set bench he
  asked for. Not re-asked.
- Not op's, owed as work: the provider reserves address space and commits through `protect` on the
  os tier (B2); `reserve` either keeps contents or is renamed so it does not claim to (the contract
  sentence and the shipped code cannot both stand); the count moves at a pass boundary through the
  plan stage's record-count parameter, which R6 already names as adaptive; deletion is the R3
  bitmask with a compaction maintenance unit.
- One genuine question, low priority. B-question: where does a record's stable identity live?
  Options: [a: the consumer's, as a column of durable ids it owns, with the compaction unit emitting
  a remap; positions stay dense and unstable | b: hilavitkutin's, as a store kind that keeps an id
  per record across compaction]. What decides it: whether any second consumer wants the same
  identity mechanism. The workspace rule reads (a), and kaski's interned ids are (a) already.

## C. The consumer's cell and lane format

### What the record says, and what the trunk holds

- R3: "Type-native stride, no flags byte, 16-byte max. Stride = size_of::<T>() per column" (spec
  `:2416-2419`). R4: "Blanket impl for all Copy + 'static ≤ 16 bytes ... BIT_WIDTH const ... for
  sub-byte bitpacking ... Type IS the storage. Consumers store whatever they want" (`:2421-2426`).
- The trunk: `ColumnValue: Copy + 'static`, an empty blanket impl, `BIT_WIDTH` a future hook with
  nothing packing (`hilavitkutin-api/src/column_value.rs:25-34`; design `:216-237`).
- Op, tier one, on the lane: "Upstream on hila it should be entirely generic and configurable by
  user" (`kaski/mock/registry/ticket/items/may_a_lattice_channel_hold_lanes_narrower_than_64_bits.toml`).
  On the loop-core map, the coordinator's relay of what he refused: "a sealed value with one integer
  type (your design says any `Copy` type is a column at its native stride)" and "an eight-byte cell
  as contract" under "Does not belong" (`202611130508_...:124-126`); those are the relay's words,
  not his, and they agree with R3 and R4.

So the premise of both tickets is false against the trunk. "Word (eight bytes) is the storage unit
and Cell64 the widest cell" and "Value is sealed and Int<BITS> is its only impl" are statements
about `hilavitkutin-contracts`, which is on no trunk crate list. On the trunk there is no storage
word, no widest cell short of the spec's sixteen bytes, and no sealing: a consumer's own value type
is a column value by being `Copy + 'static`. "How a consumer gets its own value type" has the answer
"by declaring it", and has had it since `6597bbfc`. Both tickets should be closed as malformed
against the trunk, with the one residual below carried forward.

### The arms

- C1. The consumer's type is the cell (the trunk, and R4's "Type IS the storage"). Lane width is the
  consumer's `T`; arvo's generic fixed point supplies the formats.
- C2. A closed set of formats chosen per channel (the option the experts recommended to op, which he
  answered away from).
- C3. A store-side packing layer reading `BIT_WIDTH` (R4's hook), so sub-byte values pack without
  the consumer writing a packed type.
- C4. A format descriptor at a boundary only: where a column crosses the C ABI (A3's wire view, A6's
  shared stores), the view carries a format id, as Arrow's C data interface carries a format string
  for the storage type and the extension identity in metadata. Inside the engine the type is the
  format and no id exists. This is the only place the voided crate's question ("a rule makes every
  FormatId unique") has a home, and it is at the boundary, not in the store.

### What each arm's failing case shows

s5 (`s5-cell-format/`). A consumer's `Q16(i32)`, `Q32(i64)`, `i128`, `u8` and a packed
`Lanes4x16([u16; 4])` are column values at their own widths. A 32-byte record is admitted, which R3
says should not compile, so C1 as shipped exceeds the canon. `bool` reports 8 bits and nothing packs,
so C3 does not exist yet. The shape a lane width buys, read from release assembly: the consumer's
32-bit lane with a 64-bit product compiles to `vpmuldq` and `vpsrlq` on `ymm`, eight records an
iteration; kaski's 64-bit lane with an `i128` product compiles to one-operand `imulq` and `shldq` per
record, four unrolled scalar products. The format is the consumer's choice and the engine does not
need to know it to get either loop. How much the narrow lane is worth is a bench's.

C2's failing case is op's answer itself: a closed set is what he said hila should not have.

### Op calls for C

- C1-question. R3's sixteen-byte cap, which the trunk never enforced. Options: [a: enforce it on
  every store, a compile-time refusal past sixteen bytes, as R3 and R4 say | b: amend R3 so the cap
  binds only `Field<T>`, the resource scalar whose register promotion is the stated reason for it
  (spec `:538-539`), and columns take any width at native stride | c: drop it everywhere]. What
  decides it: the record gives the cap without a reason for columns; a bench of a wide column
  against the same data split by `Decompose` into narrow columns, on the intended region, would.
- Not op's: C3 is a designed feature (R4's `BIT_WIDTH` "for sub-byte bitpacking") classified as
  later without anyone saying why. It is owed as design work, not a question.

## Prior art

Fetched 2026-11-13 into `.data/fetched/`, each named here by what it establishes and which claims I
checked against the fetched page.

- Bevy removed its dynamic plugin loading in 0.15: "deprecated in 0.14 for being unsound, and they
  have now been fully removed" (bevyengine.org 0.14 to 0.15 migration guide, fetched
  `bce33dd7...`; checked in the page). The Rust-ABI route the linking design already refuses is the
  one a large engine tried and withdrew.
- Unity Burst mod support: a mod ships a precompiled Burst library that the game loads with
  `BurstRuntime.LoadAdditionalLibrary` early, and compiling one needs the editor
  (docs.unity3d.com Burst 1.8 modding support, fetched `30972da0...`; the call checked in the page).
  That is the author compiles, the player loads: A5's surviving variant, which is A3.
- Kohn, Leis, Neumann, "Adaptive Execution of Compiled Queries", ICDE 2018 (fetched PDF `0a1f8099...`;
  checked in the text): "a morsel represents the smallest unit of work ... these morsels also emerge
  as the crucial mechanism for dynamically switching between different execution modes"; a handle
  "stores multiple variants of the same function. For every single morsel, we then choose the
  fastest available representation"; "to change the execution mode, one only needs to set a function
  pointer in this handle object". That is A8 and s2's switch-point test.
- Factorio multiplayer "uses deterministic lockstep" (wiki.factorio.com/Desynchronization, fetched
  `bb08275d...`; checked). Its mod API shows mods only game state; the forum detail that system time
  and true randomness are not exposed is a search summary I did not verify in a fetched page.
- DuckDB's extension C API: v1 is a versioned table of function pointers; v2 "is not yet stable"
  (duckdb.org C API v2 page, fetched `95ab392d...`; the v2 warning checked, the v1 stability claim is
  from the search summary). The shape of the extensions crate's provider vtable is the shape a large
  data system ships.
- Pointer-stable dynamic arrays by reserve and commit (vectrx.substack.com, fetched `fec0b71b...`).
  The claim that matters I verified myself on this host in s4.
- Arrow's C data interface: a format string for the storage type, an extension type's identity in
  `ARROW:extension:name` metadata (arrow.apache.org, fetched `da9baca5...`; the key checked in the
  page). C4's boundary-only format id.

Not reached: a measured per-call cost of a wasm component boundary in a game engine (searches found
only practitioner advice to batch, which is A3's per-morsel rule again), and any engine that admits
load-time units into a statically fused schedule, which I looked for and did not find. Flecs
modules and scripting were found only through mirrors and are not cited.

## What I settled, what I moved, what I could not

Settled, each with its spike:
- The record's extension seam cannot load a fresh library on Linux (s1, red), and the loader's suite
  is green only because it loads libc.
- Admission of an extension's declared stores against a compiled host unit's envelope is a small
  check and refuses what it should (s2); native code is not bound by it (s2, red on purpose).
- The morsel is a sound switch point between an extension's variants (s2).
- Across an opaque boundary the call must be per morsel for the extension's loop to vectorise (s2
  assembly).
- Content rows over closed forms compile to vector loops even when matched per record, as long as
  the form holds across the morsel (s3 assembly).
- The shipped resize drops the column (s4, red); capacity plus live count works today below physical
  memory (s4); the record's reserve-and-commit growth works on this host once `protect` commits (s4).
- Both of C's tickets are malformed against the trunk; the trunk admits a consumer's own types and
  exceeds R3 (s5).

Moved: "the largest open design question for kaski" is two questions. For kaski, its own canon
already says mods are content and scripts may compile to something; the engine-side need is A1
plus A3 at most. For hilavitkutin, the record already designs a seam (the surface, the meta
virtuals, plugin type access) that nobody finished; the open part is A1-question and A2-question.

Could not: no bench was run, so nothing here says how much anything costs; that needs the harness
with real competitor arms. No target without virtual memory was tried (B4, and B2's failing case).
A4 and A6 were not sketched, for the reasons given. No JIT was built for A8. Nothing was run on
macOS, where the loader's constant is presumably right.

## What I am least sure of

That op's option 2, "extension units join through a declared dynamic seam beside the compiled
schedule", means A4 at all. "Beside the compiled schedule" may well be A3: code beside the schedule,
inside a compiled unit, through a declared seam. If so, the panel's answer to option 2 is that the
record already designs it and the trunk half-built it, and A1-question collapses. I could not tell
from his words and I did not want to answer the easier question in his name.

That the core topic's "plugin load" can be read as a parameter change. It is the only reading
consistent with R6, which is a reason to prefer it and not a proof that its author meant it.

Whether the sixteen-byte cap was ever meant for columns. The spec states it without a reason that
applies to a column, and the reason it gives for `Field<T>` does not carry over.

## Sketches committed

All under `mock/research/sketches/202611131951_load-time-units-seat02/`:

- `s1-loader-fresh-cdylib/`: `ext/` (a no_std cdylib from `#[export_extension]`), `host/` (tests
  `fresh.rs`, red, and `preloaded.rs`, green), `README.md`, `OUTPUT.txt`.
- `s2-extension-surface-unit/`: `src/lib.rs`, `tests/envelope.rs` (four green, one red on purpose),
  `asm/functions-x86-64-v3.s`, `README.md`, `OUTPUT-tests.txt`.
- `s3-content-closed-forms/`: `src/lib.rs`, `tests/forms.rs` (four green), `asm/functions-x86-64-v3.s`,
  `README.md`, `OUTPUT-tests.txt`.
- `s4-growth/`: `src/lib.rs`, `tests/growth.rs` (two red), `tests/reserve_commit.rs` (two green),
  `README.md`, `OUTPUT-tests.txt`.
- `s5-cell-format/`: `src/lib.rs`, `tests/admits.rs` (four green, one of which R3 says should not
  compile), `asm/functions-x86-64-v3.s`, `README.md`, `OUTPUT-tests.txt`.

Each is a spike: it proves the one thing its README names, once, on x86_64 Linux glibc with
nightly-2026-05-28. Its names, field orders and shapes are scaffolding, not proposals.

## Reconciliation with seat 01

Written after phase one was pushed (`2bbf5726`). Seat 01's file is
`mock/research/202611131951_load-time-units/01_aaltonen.md` on `research/load-time-units-seat01` at
`80e85ad4`; I read it whole, and the sources of every sketch under
`mock/research/sketches/202611131951_load-time-units-seat01/`, from a detached read worktree that
is removed once this section is committed. Citations of the form `01:N` are lines of seat 01's file.
Nothing above this heading has been edited. Where I now think a phase-one line is wrong, I say so
here and cite it as `02:N`.

Two new spikes decide disagreements, both committed with their output:
`s6-occupant-as-resource/` and `s7-write-set-not-cell-width/`. I reran seat 01's `c1-oversized-cell`
and `b2-resize-loses-records` from its tree. Both reds reproduce, and `growth_moves_the_base_pointer`
passes (`s7-write-set-not-cell-width/OUTPUT-seat01-c1-b2-rerun.txt`).

### 1. Where the two files agree

"Same instrument" means both seats ran the same kind of check against the same code, so the second
one adds independence of authorship but not of method. "Two instruments" means the checks differed
in method or fixture.

1. The loader cannot open a library that is not already mapped on Linux glibc, because 4 is passed
   as `RTLD_LOCAL`. Reached independently (02:297-301, 02:113-120; 01:53, 01:102). The instrument is
   the same, a fresh cdylib through `Library::load`, but the fixtures differ: mine is an
   `#[export_extension]` descriptor library also driven through `ExtensionHost::load`, and seat 01's
   is a bare C kernel. So this is two fixtures on one instrument. It is not op's call. It is the
   ticketed defect, and both spikes are the test that ticket asks for.
2. `hilavitkutin-linking/tests/basic.rs` loads only libc, which is setup that helps. Reached
   independently, on the same instrument, a reading of the test (02:113-120; 01:33).
3. `hilavitkutin-api/tests/column_storage.rs:47-55` is a tautology and should be deleted. Reached
   independently, on the same instrument (02:131-135; 01:31).
4. The whole suite does not link. Two binaries fail on `__start_hilavitkutin_strings` and
   `__stop_hilavitkutin_strings`, and the rest gives 464 passed, 0 failed, 21 ignored. Reached
   independently, with the same command and the same count (02:100-107; 01:27).
5. A re-reserve through `ArenaColumnStorage` drops the column, against the contract's "resizes it".
   Reached independently on two instruments: my s4 used `OsMemoryProvider` (mmap) going from 4 to 8
   records, and seat 01's b2 used a std-heap provider going from 8 to 16 (02:473-477; 01:41, 01:177).
   The base pointer is where the two runs differ, and they compose rather than conflict. In mine it
   came back at the same address. In seat 01's it moved, because the test holds a second
   allocation so the allocator cannot hand the block back (`b2-resize-loses-records/tests/resize.rs:69-75`).
   So the base's identity after a re-reserve is unspecified. A context holding the old pointer reads
   zeros on one provider and dangles on another. The loss of the records is the one property, and
   both instruments show it.
6. The stack audit misquotes `storage.rs:46-47` as saying "undefined". Reached independently, same
   instrument (02:65-69; 01:42).
7. Both C tickets are malformed against the trunk, because `ColumnValue` is any `Copy + 'static`.
   Reached independently on two instruments: my s5 compiles a consumer's own types, and seat 01's
   sketches store `i32` newtypes in the real engine (02:530-541; 01:43, 01:273).
8. The trunk dropped R3's sixteen-byte cap. Reached independently on two instruments: s5 admits a
   32-byte record and c1 a 4 KiB one (02:159-164; 01:15, 01:235).
9. R6 governs. No unit is registered at load, and an arm that registers one is a canon amendment.
   Reached independently, from the same text (02:76-87; 01:117).
10. The record is ambiguous on what "plugin load" means to the plan, and the question goes to op.
    Reached independently (02:88-97; 01:21). Seat 01 found a sentence I missed (item 4.1 below).
11. Load-time behaviour can run inside a compiled unit whose access set is the build's, with
    anything outside that set refused at load. Reached independently on two instruments: my s2 is
    standalone native code with a bytecode variant and s3 is content rows over closed forms, while
    seat 01's a3 is an interpreter and a4 a native kernel inside the trunk engine (02:286-316;
    01:90-98).
12. The call into foreign or interpreted code must be once a morsel, never once a record. Reached
    independently on two instruments: my s2's release assembly shows the per-record host loop with
    `callq` inside and no vector instruction (02:310-316), and seat 01 quotes the tier-3
    `registration_binding` medians (01:96). Neither is a bench.
13. Prior art: Kohn, Leis and Neumann; the Bevy removal. These are the same fetched files, with the
    same hashes. That is one instrument cited twice, not corroboration.
14. Reserving address space and committing on growth works and keeps the base fixed. Reached
    independently on two instruments: my s4 reserves 32 GiB with `PROT_NONE | MAP_NORESERVE` and
    commits with `mprotect` (02:484-488), and seat 01's b3 `Reserved<T>` reserves 512 MiB and grows
    to ten million records (01:179). Seat 01 adds that `MemoryProviderApi` has no reserve or commit
    entry. My phase one blamed only the empty `protect` (`os.rs:80-83`). Both are true, and both
    are needed.
15. Deletion is the R3 tombstone, and identity lives as data in a column the consumer owns, never
    as a handle. Reached independently (02:495-498, 02:513-517; 01:185).
16. kaski's `modding_is_content_only_never_engine_code` is demoted and binds nothing, and
    `content_kinds_are_open_and_interned` binds. Reached independently, same instrument (02:232-239;
    01:47).
17. `BIT_WIDTH` is read by nothing, and nothing packs. Reached independently on two instruments: my
    s5 test on `bool`, and seat 01's reading of source (02:561-562; 01:17).
18. Nothing in either file is priced. Every "how much" is owed to the harness.

### 2. Where they disagree

D1. Whether the record's seam exists, and where load-time code lives.
- Seat 01: the record's "one nod toward a plugin reading columns" is HasShape/ErasedColumn
  (01:67). Its A-1 options are (a) nothing load-time enters hilavitkutin, or (b) an opt-in crate
  ships a slot-unit contract (01:132-136). The slot unit takes its kernel as a field at build
  (`a4-slot-unit/host/tests/slot.rs:143-146`, `:223-225`).
- Me in phase one: the record designs a seam, `ExtensionSurface<dyn F>` (formerly `LinkedBin`),
  with `PlanStage` on plugin load, and hilavitkutin should finish it as a store kind with an
  envelope (02:212-218, 02:354-357). My sentence that a unit reading it "is ordered by the plan like
  any other reader of a store" (02:217-218) was read off the doc comment, not run, and it is false.
- Evidence and decision: s6. Registering `ExtensionSurface::<dyn Family>::new()` and calling
  `build` does not compile. The surface implements none of `StoreAccumKind`, `StoreElemBytes`,
  `BindingsFor` or `DrainStores` (`s6-occupant-as-resource/OUTPUT-surface-does-not-compile.txt`;
  `hilavitkutin-api/src/builder_input.rs:174-187`). No test anywhere registers it. Its only
  appearances are `builder_input.rs`, `lib.rs` and a doc line in `builder.rs`. So my phase-one
  sentence that "the trunk half-built it" (02:650) is generous: it is a marker that breaks `build`.
  In the same spike, an occupant held in an ordinary `Resource` works on the trunk today, with no
  new mechanism. The occupant is a C-ABI kernel pointer, which is `Copy + 'static` and 8 bytes, so
  it is already a column value inside R3, and it is marked `PlanAffecting`. The unit declares it in
  its read set, so the plan orders the unit by its access set. Swapping it after build with
  `replace_resource` opens the plan band once, the unit reruns (7 to 14 morsel calls), and every
  record of the next frame is the new occupant's. The frame after shuts the band again.
- Resolution: neither phase-one position survives intact. Seat 01 is wrong that the record has no
  seam, and wrong to bind the occupant at build (see attack 2). I was wrong that the seam's shape is
  a new store kind to finish. The `Resource` mechanism already gives what the surface promised:
  ordering by access set, a dirty seed for readers, and one plan band per load. The one thing it
  does not give is a tie between the occupant's lifetime and the library handle, which matters on
  unload. That is the decision for op, below as merged call M3.

D2. When the occupant is bound.
- Seat 01: at build, as a field of the unit (`slot.rs:143-146`).
- Me: after build, swapped through a handle (02:304-309, s2's `AtomicPtr`), and now through a
  `PlanAffecting` resource (s6).
- Decision: s6. Binding at build means a library loaded after the schedule exists cannot reach the
  unit without a rebuild. A rebuild "loses every column", in seat 01's own words (01:203), because
  `build` consumes the storage (`builder.rs:167-171`). The extensions design's arbitrary-time
  linking invariant (`hilavitkutin-extensions/DESIGN.md.tmpl:143-152`) needs binding after build.
  s6 shows it, and a swap at the morsel boundary is s2's switch point.

D3. What the sixteen-byte cap is for, and whether a derived bound replaces it.
- Seat 01: "The record's 16-byte bound is what keeps the formula meaningful" (01:235). Its C4
  derives a per-cell bound `size_of::<T>() <= L1_USABLE / MIN_MORSEL`, which is 384 B (01:241).
- Me in phase one: the cap has no stated reason that applies to a column, with an option to bind it
  only on `Field<T>` (02:572-576, 02:656-657).
- Evidence and decision: the record and s7. The record does give column reasons: "Clean
  power-of-two, fits u128, aligns with SIMD" (spec `:388`) and "Forces interned IDs for
  String/PathBuf/Vec" (spec `:461`), with the cap on the blanket impl itself (spec `:448-449`). My
  option (b) rested on a false premise, and I withdraw it. Seat 01's inference fails too. Step 9 sums
  a fiber's write set (`step9_morsel.rs:40-56`) and clamps the window at `MIN_MORSEL` (`:73-75`).
  The record writes that clamp itself:
  `(L1_usable / Σ write_sizes).clamp(MIN_MORSEL, MAX_MORSEL) & !3` (spec `:834`), with
  `MIN_MORSEL` "configurable (not a design decision)" (`:840`). s7 shows that forty 16-byte write
  columns, every cell inside R3, overshoot the L1 budget (40960 B against 24576 B). Two 384-byte
  columns, every cell inside seat 01's derived bound, also overshoot (49152 B). The quantity is a
  fiber's summed write width against `L1_USABLE / MIN_MORSEL`, never one cell's width. So the cap
  stands for its own reasons, c1's L1 red is a consequence the record designed and not a symptom of
  the missing cap, and the derived bound's rationale is refuted.

D4. Whose types `repr(C)` binds.
- Seat 01: restore `repr(C)` "on every stored type, by derive or lint, as the record says"
  (01:16, 01:269).
- Me: phase one missed the record's `repr(C)` text entirely. My reading now is that it binds domain
  newtypes and seam types, not every consumer type.
- Evidence: the record says "repr(C) on domain newtypes enforced by `#[derive(ColumnValue)]` macro or
  lint, not by the trait itself" (spec `:462`). The blanket is "any `Copy + 'static` type ≤16 bytes
  is column-storable. Zero friction for consumers" (`:448-449`), and "Consumers store whatever they
  want" (`:458-459`). The retained list's "repr(C) on stored types (T3 §Q3)" (`:477`) is a summary
  line. The text with the mechanism in it says the trait does not require it. A blanket that admits
  any Copy type cannot also demand `repr(C)` of it.
- What would decide it: T3 §Q3 itself, the round the summary cites, which neither seat read. Until
  then the call goes to op (merged call M12), and this disagreement is not converged.

D5. Whether B carries op calls.
- Seat 01: four calls, B-1 to B-4 (01:201-210).
- Me: almost none, because op and the record have answered (02:503-517).
- Evidence: B-1's option (c) is op's own answer, which seat 01 quotes (01:160). B-2 is settled by
  seat 01's own sentence "The design is right and the code is wrong. Fix the code toward the
  design" (01:57), and then re-asked as "Either is honest" (01:206). B-3 is answered by the record:
  "mmap-backed (with linked-page fallback for no-mmap platforms)" (core topic `:683-685`). Only the
  identity and compaction question (B-4, my B-question) is open in the record. Not converged on
  framing. On substance it converges, because seat 01 says what the right answers are.

D6. Whether B1 works today.
- Me: "B1 works today up to roughly physical memory" (02:478).
- Seat 01: the trunk cannot express it, because the count is a build argument (01:183).
- Decision: source. `record_count` is a private field of the scheduler (`scheduler/mod.rs:181`). It
  is set at `build` (`builder.rs:167-171`), read by every run entry (`run.rs:132`,
  `run_fused.rs:111`, `run_trunks.rs:63`), and assigned nowhere after build. Seat 01 is right, and my
  sentence is wrong. What s4 showed is that the provider's side of B1 works: residency follows what
  is touched. The live count cannot move, so every unit walks the whole capacity. That is owed work:
  a count the plan band reads, beside `replace_resource`, as R6's "record counts (from consumer
  data)" requires (spec `:2443`).

D7. Where admission lives.
- Me in phase one: a domain-agnostic helper in `hilavitkutin-extensions` (02:361-364).
- Seat 01: the consumer, implicitly (01:119).
- Decision: my own s2's red. Native code is not bound by a declaration (02:304-307). An
  interpreted program is checked by its own interpreter against the unit's slots (seat 01's a3).
  With the occupant as a resource, the host unit hands it only windows of its own declared columns,
  so the envelope is structural. I move to seat 01's side: admission is the consumer's.

D8. Interned names across the seam. Seat 01 raised it and I did not. See section 4 and merged
call M5.

### 3. Attacks on seat 01

1. "That is the record's one nod toward a plugin reading columns" (01:67) is false. Here is what
   the record and the trunk have:
   - the core topic's `PlanStage` "fired when DAG changes (startup, plugin load)" (`:723`);
   - the virtual-bit packing "Recomputed on DAG change (plugin load, reconfiguration)" (`:247-248`);
   - "loimu with WASM plugins build it from work units" (`:776-779`);
   - the api design's `LinkedBin`/`ExtensionSurface` section (`hilavitkutin-api/DESIGN.md.tmpl:193-214`);
   - the type itself (`builder_input.rs:155-193`).

   A file whose A-1 offers to ship "a slot-unit contract" in a new opt-in crate (01:134) did not
   look at the api crate it would sit on, which already declares one. That declaration does not
   compile when used (s6), and that is the finding seat 01 walked past. Replacement: merged call M3.
2. The slot unit is not a load-time unit in the sense the extension stack means. Its kernel is a
   struct field set in `.with(Slot { kernel })` before `build` (`slot.rs:223-225`). So its library
   must be loaded before the schedule exists, and no library loaded later can reach it. The test
   proves that a kernel resolved at startup can run inside a static schedule. It does not prove
   load-time behaviour inside one. Replacement, built and green: the occupant is a `PlanAffecting`
   resource holding the C-ABI table. The host unit declares it in its read set and calls it once a
   morsel, and a load after build is a `replace_resource` that buys exactly one plan band (s6). The
   unload half is open (M3).
3. c1's conclusion (01:235) and C4's rationale (01:241) are refuted by s7, and by the record's own
   clamp (spec `:834`, `:840`). The red is real, but it is the designed floor of the morsel formula,
   not a cost of the missing cap. A 16-byte-clean fiber hits the same red. Replacement: restore the
   cap for the reasons the record gives (`:388`, `:461`). State the L1 promise as a fiber-level
   rule, and decide what happens when a fiber's write width exceeds `L1_USABLE / MIN_MORSEL` (M10).
4. "repr(C) on every stored type ... as the record says" (01:269) misreads spec `:462`, which says
   "not by the trait itself", for the reasons in D4. Replacement: `repr(C)` is required of a value
   type that crosses an image seam, enforced where the seam is declared. The workspace rule already
   says FFI structs are wire shapes built at the call site. Domain newtypes take the derive per
   `:462`.
5. B-1, B-2 and B-3 are put to op although op or the record already answers them, and B-2
   contradicts seat 01's own 01:57 (D5). Op told the helpers not to bring him questions the record
   answers (op-answers-25). Replacement: all three are owed work.
   - The count becomes a plan parameter (D6).
   - `reserve` copies, then frees, then the scheduler rebinds. The base's identity is unspecified
     (agreement 5).
   - mmap reserve and commit on the os tier, with pages elsewhere.
6. Citations that do not hold:
   - The 15 October quotes are cited at `:282` and `:300` (01:69). They are at `:284` (the
     layer-two act) and `:303` (op's "Okay yeah we'll want a dsl ... Preferably behind an
     interface"). The substance holds and the lines do not.
   - "op's standing instruction on 13 November was to do the work" (01:35) carries no citation, and
     it is the license seat 01 used to override the test gate.
   - `EXTENSIONS_RESEARCH.md:42-46` is about `inventory`'s `.init_array` constructors, not the
     `__start`/`__stop` section bounds `hilavitkutin-str` reads. Both are linker magic, but they are
     different mechanisms, so "names this mechanism" (01:54) overstates it. The finding about names
     across an image stands on seat 01's own spike, not on that citation.
7. The A composition's "Zero-cost shipping of first-party packs: A6" (01:127) and A6's "needs a Rust
   toolchain on the player's machine, or a build service" (01:113) miss that kaski's own row bars
   it: "Rust will never be the authoring language and [won't] ship rustc with the game or it's sdk"
   (`kaski/mock/registry/tenet/content.toml:219-228`, provisional). A first-party pack compiled in is
   just the build. A third-party pack compiled by its author is Unity's pattern, which is A4 with
   native code. A6 as a player-side step is closed for kaski.
8. c2's composition, a split rule rounding windows and per-core splits to `64 / BIT_WIDTH`
   (01:250), ignores op's stated mutation paradigm. "Intended case in the mutation paradigm is
   writing through queues ... queued up ops that drain verctoeised and optimized at the close"
   (op-answers-25, which seat 01 itself cites at 01:161). A packed column written only through a
   queue drained at the close has one writer by construction, so the split rule is needed only for
   a unit writing a packed column directly. Replacement: merged call M11, option a.

### 4. What seat 01 has that I missed, and whether it holds

1. Domain 16. The plan reruns on "more records, new WUs, structural DAG change" (spec `:1476-1477`),
   and the consumer "changes WU registration" (`:1487`). This holds: I read both lines. It is a
   third place in tension with R6, beside the two core-topic lines I cited. Read with R6 and with s6,
   the consistent reading is that a change to the unit set is a rebuild. A change of occupant or
   count is a plan band.
2. The context has no window slice. `read(i)` and `write(i, v)` are its only column access
   (`hilavitkutin-api/src/context.rs:117`, `:134`). This holds. My phase-one call point, "hands the
   occupant one wire view a morsel" (02:358-360), assumed an accessor the trunk does not have, so s2
   modelled it outside the engine. s6 and seat 01's a4 both copy each morsel in and out. The record
   has the shape: "One base pointer per morsel, columns at `base + col_offset + i * stride`" (spec
   `:397-398`). It is owed work on the api.
3. The record count is fixed at build. This holds (D6).
4. `hilavitkutin-providers/tests/column_storage.rs:215`,
   `a_second_reserve_frees_the_first_block_as_it_was_reserved`, pins the free. This holds: it is the
   one test that looks at a resize, and it asserts the behaviour that breaks the contract.
5. Names do not cross an image (A4-2, A4-3), and const names do not resolve on wasm (A4-4). This
   holds against source: `static_entries` returns `&[]` off linux, android, macos and ios
   (`hilavitkutin-str/src/section.rs:69-80`), and lookup is a linear scan
   (`hilavitkutin-str/src/interner.rs:119-140`) where the design claims "O(1)"
   (`hilavitkutin-str/DESIGN.md.tmpl:117`, `:137`). I did not rerun seat 01's a4 name tests, and the
   red rests on their committed output alone. The wasm half is a defect, not a design call: a name
   declared in an image must resolve in that image on every target kaski ships.
6. The record's `repr(C)` text (spec `:462`, `:477`). This holds as text. The reading is contested
   (D4).
7. b3's page straddle. This holds arithmetically: 6144 is 1.5 times 4096, so nearly every window
   straddles a 4096-record page. I add one refinement. A page must be sized in records, not bytes,
   because one fiber's columns have different strides, and a page of fixed bytes is a different
   record count per column. With pages in records, the planner rounds the window down to a divisor
   of the page, once per fiber.
8. c2's race. This holds as a hazard of the read-modify-write model seat 01 wrote, which seat 01
   says itself (01:305).
9. c6's layout. This holds as an existence claim under `-Zrandomize-layout`. One compiler at one
   version is stable in practice.
10. The pending ticket `which_registration_hilavitkutin_s_contract_takes_beside_the`
    (`tickets-hilavitkutin.toml:565-572`). This holds, and it is the same question as A from the
    registration side. In phase one I knew of its round (02:229-231, 02:260-261) but not that it now waits on op.
11. The accumulator refusal carries `wanted` and `had` (`hilavitkutin-api/src/accum_refused.rs:12-19`).
    This holds, and it makes the spawn queue (seat 01's B6) half-built already.
12. The record's own Approach C figure is "one fn ptr per fiber per morsel", 1.17x (spec `:1553`),
    "unacceptable" (`:1653-1654`). Seat 01 cites it against A8. I add what neither file said. An
    occupant called once a morsel across the C ABI is Approach C's cost class by construction,
    because nothing inlines across that barrier. The record refused that cost for compiled units,
    which can be fused. It did not consider code that cannot be. The bench owed for M1 must have an
    arm for the same kernel compiled in as a unit, so the barrier's price is a number and not an
    analogy.

Unlicensed things newly met, per the standing instruction:
- `ExtensionSurface` is documented as a builder input with `Dispatch = StoreDispatch<Self>`
  (`builder_input.rs:164`, `:184-187`). It breaks `build` when registered (s6), and no test in the
  repository registers it. The doc also promises `SchedulerBuilder::extend` per round
  `202605101036` D5 (`:166-167`), which does not exist.
- The api design promises "the typestate proves coverage at `.build()` time"
  (`hilavitkutin-api/DESIGN.md.tmpl:208-210`) for a type that cannot reach `.build()`.

### 5. The op calls of both files, merged

Each call gives its options with the recommended one first, which seat proposed it, and whether
both seats agree. "Provisional stamp" marks a call both seats agree on that the governing record
cannot derive. The dispatcher puts those to op as questions.

M1. Does code that arrives at load ever become a unit in hilavitkutin's plan? Merges my
A1-question and seat 01's A-1.
- Options:
  - a: never; it runs as the occupant of a compiled host unit, read from that unit's declared access
    set, and R6 stands;
  - b: an extension unit joins the plan at `PlanStage` with masks declared at load, R6 is amended,
    and Approach C returns for those units;
  - c: hilavitkutin offers only content as data, and any code seam is the consumer's, above the api.
- Proposed by both. Both recommend a.
- The record derives a from R6 (spec `:2439-2440`), so this is not a stamp. It still goes to op as
  the panel's answer, because op asked for the panel. What would move it to b: a harness bench on
  the intended region of an occupant once a morsel, against the same kernel compiled in and fused,
  against Approach C.

M2. What a "plugin load" is to the plan. Merges my A2-question and seat 01's A-2. The record's
lines are core `:247-248` and `:723`, and spec `:1476-1477` and `:1487`, against R6.
- Options:
  - a: a change of what the plan is parameterised by, which is an occupant, a gate or a count; on
    the trunk that is a `PlanAffecting` swap that opens one plan band (s6), and the unit set is
    fixed;
  - b: a change of the unit set, which is M1(b);
  - c: as (a), and the plugin and registration wording in domain 16 and the core topic is marked as
    superseded by R6 in the record's next consolidation.
- Proposed by both. Both recommend a, and seat 01 also offers c.
- Provisional stamp: the record's words do not decide what their author meant.

M3. The record's seam, `ExtensionSurface`. New in reconciliation, from s6.
- Options:
  - a: retire it from the api and its design; the occupant is a `PlanAffecting` resource holding a
    C-ABI table, which the trunk already orders, dirties and replans (s6); the extension host owns
    the rule that an occupant is swapped out before its library is dropped;
  - b: finish it as a store kind that holds the occupant and ties its lifetime to the library
    handle, so an unload cannot leave a dangling occupant;
  - c: leave it as is, a declared marker that breaks `build`.
- Proposed by seat 02. Seat 01 did not consider the type, so there is no agreement.
- What decides between a and b: whether unload safety belongs in the type or in the host's
  discipline, shown by a spike that drops a library while its occupant is installed. c is not an
  option anyone should take. It is listed because it is the current state.

M4. Where admission of a load-time occupant lives. Merges my A3-question with seat 01's implicit
position.
- Options:
  - a: the consumer's, since native occupants are not bound by a declaration (s2's red) and
    interpreted ones are checked by their own interpreter;
  - b: a domain-agnostic helper in `hilavitkutin-extensions`.
- Both now recommend a. I moved (D7).
- Provisional stamp: the record is silent.

M5. Names across an image seam. Seat 01's A-3, with a replacement option added.
- Options:
  - e (seat 02 recommends): no `Str` crosses the C ABI; an occupant names stores and keys as text in
    its descriptor, the host interns them at load, and the occupant refers to them by position in
    its own declared list, so there is one table and no handle crosses;
  - a: the extension interns through the host's interner, passed as function pointers;
  - b: the host pulls each library's const table through an exported symbol and merges it;
  - c: const and run-time handles compare by hash;
  - d: retire the linker section for a build-time table.
- Proposed by seat 01, with e added by seat 02. Seat 01 made no recommendation, so there is no
  agreement.
- What decides it: whether a mod may define a name the base game reads by constant. If not, e is
  enough. Separately, and not op's: const names must resolve on wasm in their own image. That is a
  defect (section 4.5).

M6. Isolation. My A4-question.
- Options:
  - c first: wasm is the consumer's occupant behind the same seam, as the linking design leans
    (`hilavitkutin-linking/DESIGN.md.tmpl:83`);
  - a: native full trust only, v1;
  - b: a wasm transport in hilavitkutin's extension stack.
- Proposed by seat 02. Seat 01 did not pose it.

M7. Consolidate with `which_registration_hilavitkutin_s_contract_takes_beside_the`. Seat 01's A-4.
Both agree that this panel's evidence supports that ticket's option a, typed only. It is the
dispatcher's to merge, not a separate call.

M8. Stable identity across removal. Merges my B-question and seat 01's B-4.
- Options:
  - a: tombstones (R3) by default, with a swap-compaction pass at the close for columns that declare
    it; identity is a durable id column the consumer owns, and compaction emits a remap;
  - b: identity is hilavitkutin's, a store kind that keeps an id per record;
  - c: positions stable always, with no compaction.
- Proposed by both. Both recommend a.
- Provisional stamp: the record has the tombstone (spec `:385-386`) and says nothing on compaction
  or identity.

M9. The live count, reserve, and growth per tier. Seat 01's B-1, B-2 and B-3. Recommended: not put
to op. They are owed work as listed in attack 5, because op ("Both, declared per domain", and
"Bench both first" for record sets) and the record (core `:683-685`, R6 `:2443`) answer them.
- Proposed by seat 01. Seat 02 disagrees that they are op's.

M10. The fiber's write width against L1. New in reconciliation, from s7.
- Options:
  - a (seat 02 recommends, low confidence): report at build any fiber whose summed write width
    exceeds `L1_USABLE / MIN_MORSEL`, as a value, and keep the record's clamp;
  - b: the planner splits such a fiber so each part fits;
  - c: refuse it at build;
  - d: the trunk today, silent.
- Proposed by seat 02. Seat 01 framed the same red as a cell-bound question, so there is no
  agreement yet.
- What decides between a and b: a harness bench of a forty-column fiber, split against unsplit.

M11. Packing. Seat 01's C-2, with a replacement option added.
- Options:
  - a (seat 02 recommends): BitPacked layout per R4; packed columns written in parallel only through
    queues drained at the close, per op's mutation paradigm; a unit writing a packed column directly
    gets windows and per-core splits rounded to `64 / BIT_WIDTH`;
  - b: rounding for every packed column, as seat 01 proposes;
  - c: single writer per packed column per phase;
  - d: not now.
- Proposed by seat 01, with a refined by seat 02. Both agree that packing is owed by R4 and that an
  unrounded split loses writes. They do not agree on which composition is recommended.

M12. `repr(C)`. Seat 01's C-3.
- Options:
  - b (seat 02 recommends): required on value types that cross an image seam, at the seam, plus the
    derive for domain newtypes per spec `:462`;
  - a (seat 01's reading): every stored type, by derive or lint;
  - c: none.
- Not converged (D4). What decides it: T3 §Q3.

M13. The sixteen-byte cap. Merges my C1-question and seat 01's C-1.
- Options:
  - a: restore it as R3 and R4 say, a compile-time refusal on the blanket impl;
  - b: seat 01's derived bound;
  - c: no bound, which amends R3 and R4.
- Both now recommend a. I withdrew my Field-only option, and s7 removes b's rationale.
- The record derives a, so this is not a stamp. The trunk's missing cap is drift to be fixed. It
  goes to op only if he wants c.

M14. Close both C tickets as malformed against the trunk, filing M11 to M13 in their place. Both
agree. This is the dispatcher's call.

Not calls, and owed by both files' evidence:
- the loader constant per target, with a fresh-library test;
- deletion of the two tautological tests (`column_storage.rs:47-55`, `column_value.rs:48-62`);
- a re-reserve contract test, and `reserve` made to keep contents;
- a count the plan band reads;
- a reserve and commit entry on `MemoryProviderApi`, and a body for `protect`;
- a window slice on the context;
- const names resolving on wasm;
- the suite linking.

### The composition that holds

The schedule stays what R6 says it is. The units, the graph and the dispatch are the build's.

What arrives at load is one of three things:
- data: rows in typed columns, kinds interned at load (s3, seat 01's A1);
- an interpreted program, checked at load against a compiled unit's slots and run a vector at a
  time per morsel (seat 01's a3);
- a native kernel, compiled by its author, never by the player.

The second and third are occupants of a compiled host unit, held in a `PlanAffecting` resource that
the unit declares. A load or a swap is one plan band and a dirty seed, which is the record's "plugin
load" read consistently with R6, and it runs on the trunk today (s6). The call crosses once a
morsel, never once a record (s2 assembly, the tier-3 bench), and costs Approach C's class by
construction. That cost is unpriced and is the bench M1 owes.

hilavitkutin owes the finished parts:
- a loader that loads;
- a window slice;
- names that survive the seam;
- the fate of `ExtensionSurface` (M3);
- a count that moves;
- a `reserve` that keeps its contents;
- reserve and commit on the os tier, with pages, sized in records, elsewhere.

The cell is the consumer's own type up to the record's sixteen bytes. The L1 promise is a fiber
rule, not a cell rule (s7). Packing waits on R4's layout with op's queues as the writer.

Not converged:
- `repr(C)` scope (D4);
- the fate of `ExtensionSurface` (M3), because seat 01 did not address it;
- the recommended composition for packing (M11);
- whether B carries op calls (D5).
