# src changelist

Round topic `202611192054-xugn_topic.every-build-choice-is-a-pragma.md`, the only flat topic, and doc changelist `202611192054-xugn_changelist.doc.lock.md`.

This changelist replaces `202611192054-xugn_changelist.src.deprecated.md`, and it is the fifth text of the source. The first is the deprecated one. Its source was written, at commit `66388cfc`, and writing it showed one defect of the design and a handful of names the design did not have; the second text, at commit `4e9435de`, was the source as it then stood. The first review of that source, `mock/research/202611192300_review-of-the-pragma-round.md`, found a route the canon does not name, two races and an unheld path, and the doc changelist was reopened for them; the third text, at commit `ff154907`, was the source after those. The second review, in the same record, found that a pragma was asked about one tool where its text names several, and the doc changelist was reopened once more; the fourth text, at commit `5e79882c`, was the source after that. The third review, in the same record, found the source keeping an older tool whose lock is held where the design removed every older one, and a proof of llvm-ar that only a vendor's build passes, and the doc changelist was reopened for the two. This text is the source after that. The last section says what differs from the deprecated text.

The crate's pragmas become types, a set a tuple of them, and the seven build choices pragmas among the rest. Everything below is under `mock/crates/hilavitkutin-build/` unless a path says otherwise. The mechanism follows the sketches the doc changelist names; the set and its refusal under check are `202611192110_a-flat-tuple-set-refused-under-cargo-check`, the generated file's form is `202611192215_dotted-keys-and-an-error-directive`, and each linker's flags are `202611192150_each-linker-by-its-link-arguments`.

## src/pragma.rs (rewritten)

- `Pragma`, the trait of the design: `NAME`, `BIT`, `REACH`, `UNMET`, `NEEDS`, `type Group`, `type Requires`, `admitted`, and the three writers, `write_rustflags`, `write_cfgs` and `write_cargo_config`, each writing nothing by default. There is no writer of link arguments. `NEEDS` is a list of rows of `TOOLS`, empty by default.
- `Reach`, `Crate` or `Workspace`.
- `Written`, what a writer is handed, with its accessors, and `Shared`, `Owed` or `Written`. The tool's path it holds is the path of the first of the pragma's needs.
- `Group` with `BIT`, `CLASH` and `SHARED`, and `Free`.
- `Requirement` with `ALL`, `ANY` and `NONE`, implemented for `()`, `All<L>`, `Any<L>` and `Not<L>` over a list that is a `Set`.
- `PragmaSet`, a mask with `PragmaSet::of::<S>()` and `contains::<P>()` and nothing that changes it.
- `Mask` and `Words`, the aliases a mask and a name or a sentence are written in.
- The place of every pragma and of every group in a mask, two private enums, one list each.
- The enum `Pragma`, `PragmaIter`, `with`, `without`, `coherent`, `iter` and `parallel_codegen_units` are deleted.

## src/set.rs (new)

- `Set`, implemented by one macro for the flat tuples of arity zero through sixteen: `MASK`, `GROUPS`, `SHARED`, `REACH`, `VERDICT`, and `each`, which hands every member to a `Visit`.
- `Visit`, one method generic over the member.
- `Site`, what a walk is handed, and `ToolPath`. `each_tool` is the walk over a set's tools, every need of every member.
- `checked::<S>()` and `crate_reach::<S>()`, the two `const fn` a script's `const` items call.
- The fold of a verdict is a private `const fn` over one row per member: a pragma named twice, then a group held twice, then a requirement unmet.

## src/roster.rs (new)

- The thirteen older pragmas as unit types: `LoopOptimization`, `Polly`, `MathPeephole`, `FastMath`, `ExpandedLto`, `Pgo`, `Bolt`, `Profiling`, `BuildStd`, `ParallelCodegen<const N: u8>`, `SharedGenerics`, `LoopFusion`, `MimallocAllocator`.
- `LoopOptimization` writes its five options, `ParallelCodegen` `-Zthreads=N`, `SharedGenerics` its flag and `FastMath` its cfg, as `flags.rs` and `bootstrap.rs` wrote them. `MathPeephole` requires `All<(FastMath,)>` and `Profiling` `Any<(Pgo, Bolt)>`.
- `ExpandedLto` is of workspace reach, in `LtoGroup`, and writes its two profile keys.

## src/choice.rs (new)

- The groups `LinkerGroup`, `EdgeGroup`, `LtoGroup`, `IncrementalGroup`, `BackendGroup`, `DriverGroup` and `CcGroup`, the last two shared.
- The pragmas `Mold`, `Wild`, `Bfd`, `DylibDomain`, `ThinLto`, `Incremental`, `NoIncremental`, `Cranelift`, `GccDriver`, `ClangDriver`, `GccCc` and `ClangCc`, each writing what the design's table of the seven choices says, and each admitting the targets the design says.
- The three linkers write rustc flags and nothing else: `Mold` `-Clink-arg=-fuse-ld=mold`, `Bfd` `-Clink-arg=-fuse-ld=bfd`, and `Wild` its driver and its path as one pair, and nothing without a path.
- `Wild` needs wild and clang, `GccCc` gcc, g++ and ar, and `ClangCc` clang, clang++ and llvm-ar. Every other pragma of the file needs one tool or none.

## src/target.rs (new)

- `Target` with its `Os`, `Env`, `Arch`, `ObjectFormat` and `Isa`, and `Target::from_cargo` over the four values a script read.

## src/tools.rs (new)

- `Tool`, `Source`, `Proof`, `Line`, the rows as constants and `TOOLS` over them. The rows of clang++, llvm-ar, g++ and ar are among them, each a package. A `Proof` is asked about a tool's version line, which the script reads as the first two lines the tool prints, joined by a space.
- `tools_home` with `Home`, `write_tool_dir`, `provision` with `Provision`, and `write_refusal`.
- `older`, whether one dotted version is below another, which is what the script asks before it removes a directory beside a tool it built. Text that is not dotted numbers is older than nothing.

## src/generated.rs (new)

- `GENERATED_NAME`, the generated file's name.
- `write_generated::<P, _>`, the file's text for a prototype: the line that names the crate, then the keys of the `dev` set and of the `release` set, a shared key once.
- `write_uncarried::<P, _>`, one `cargo::error` line for each pragma of workspace reach in the `dev` and the `release` set that the target does not admit, and one for each of its needs the script says the machine lacks, with the profile it is in. A tool with no source yet is lacked by nobody.
- `Claim` and `claim`, and `includes_generated`, which takes the file's name alone in quotes: a path that ends in the name is another file.

## src/prototype.rs (rewritten)

- `Prototype`, the trait with `Dev`, `DevOpt`, `Release`, `Profiling` and `Ci`.
- `Default`, `Deterministic` and `Workspace`, with the sets of the design's table. A set lists its pragmas in the order of their bits, so the flags of a crate are the text they were.
- `claimed::<P>()`, the `const fn` of the claiming script: every set checked, and the same pragmas of the shared groups in each.
- The enum `Prototype` with `tune` and `configure` is deleted.

## src/profile.rs

- `Profile::default_pragmas` is deleted; a prototype has the sets. `Profile::name` is added.

## src/config.rs

- `BuildConfig` is `profile`, `pragmas` and `target`, the last a `Target`; `tier` and `passes` are gone.
- `BuildConfig::from_cargo::<P>` takes the profile's value and a `Target`. `BuildConfig::from_env::<P, _>` reads both through the closure.
- `fast_math` reads the mask by `contains::<FastMath>()`.

## src/flags.rs

- `write_rustflags::<S, _>` and `writes_flags::<S>` are over a set's type and take its `Site`, the second answering a `Flags`. The list of flags by `match` is deleted.

## src/bootstrap.rs

- `write_directives::<S, _>` takes the `BuildConfig` and the set's `Site`. After the lines it wrote before it writes one `cargo::error` per pragma the target does not admit, per local-build tool among a pragma's needs that the site has no path for, and per such tool whose path holds a space. The line for a missing path says that a `script!` with none says why in its own error and that no other script can name the pragma. It writes no `cargo::rustc-link-arg`.
- `configure` is `configure_as::<Default, _, _>`. `configure_as` asserts `checked` and `crate_reach` over the prototype's five sets in an inline `const`, picks the set of the profile, writes its directives and keeps the warning for dropped flags.
- `configure_with`, `resolve` and `write_config_file` are deleted.

## src/wiring.rs

- `Unpacked`, `Placed`, `Host` and `Flags`, which `Tree::of` and `plan` take where they took `bool`.
- `INCLUDE_LINE`, and `write_cargo_config` writes it above the `[build]` table.
- `Notice` gains `Generated` and `AddInclude`, and `write_notice` their lines.

## src/script.rs, src/script_tools.rs (new)

- `script!` takes a prototype by bare name or by path, and either with `, workspace` after it.
- The expansion holds a `const` item per set that calls `checked` and `crate_reach`, or one that calls `claimed` in the workspace's script.
- It resolves the build, picks the set of the profile, and for every tool of the set does what `provision` answers: nothing, a `cargo::error` with `write_refusal`, said once for a tool however often it is asked for, or the local build. That half is a second macro the first expands, in `src/script_tools.rs`, so neither file passes the size limit.
- The local build holds the operating system's lock on the file `<name>-<version>.lock` beside the tool's directory for as long as it runs, and a script that waited for the lock looks for the tool again before it builds. There is no lock directory and no wait with a limit. After a build it removes the `<name>-<version>` directories beside the new one that `older` says are below the pin, and no other, and of those it leaves one whose own lock is held, which a clone that pins that version is building. It removes no lock file.
- The tools it has refused sit in an array of one place per row of `TOOLS`.
- It writes the directives with the tools it found, and the wiring as before.
- The workspace's script then reads the generated file and `config.toml`. It prints what `write_uncarried` writes, asking for each tool as it does for the set's own, and where that is anything it leaves the file. Otherwise it writes the file where there is none or where it holds another text for this crate, under a name of its own first. Where there was none it places the file by a hard link, and where the link finds a file already there it reads whose that one is. A file claimed by another crate, found before the write or at the link, is left and a `cargo::error` names both. It asks to be run again when the file changes.
- The expansion names no `Vec`: the tools of a set sit in arrays, forty-eight places, three needs for each of sixteen members.

## src/lib.rs

- The modules above are declared, `axis` and `requirements` are gone, and the exports follow. The crate doc's layout and roster say what the crate has, and say of the three linkers that each writes rustc flags the wrapper appends.

## src/axis.rs, src/requirements.rs (deleted)

## Outside the crate

- `mock/crates/hilavitkutin/build.rs` is `script!(Workspace, workspace)`.
- `mock/.cargo/config.toml` gains the include line, and `mock/.cargo/hilavitkutin-build.toml` is the file the engine crate's script writes, committed.
- `mock/Cargo.toml`: the comment over the release profile says where `ExpandedLto` is written now. The profile keeps `lto` and `codegen-units` beside the generated file. Cargo reads `.cargo/config.toml` from the directory it is run in, so a build started elsewhere against this manifest takes no key of the generated file, and the benches of this repository are builds whose LTO may not depend on where they were started.
- The root `README.md`: the paragraphs on this crate say what a consumer writes now, `script!` with a prototype, `, workspace` in one script, the include line, and `configure_as` for a script written by hand. `configure_with` and the sentence that `ExpandedLto` writes nothing go.
- `mock/crates/hilavitkutin-frozen-authors/build.rs` is not touched.

## Tests

- `tests/common/mod.rs` (new): the list of every pragma, named once, which the binaries below share.
- `tests/harness/mod.rs` (new): the scratch workspace and the spying rustc, moved out of `tests/script.rs` for the two binaries below that also build one. The spy can be told a sysroot to answer with, so a test decides whether the toolchain has a component, and a build can be given a path of the test's own that leaves named programs out, so a test decides whether the machine has a tool.
- `tests/pragma.rs` (rewritten): every two pragma bits and every two group bits apart, by a list that names each type once and is held to the count; the group, the reach and the needs of every pragma against a table; `PragmaSet::of` and `contains` over sets that hold and do not hold a pragma; the three requirement forms as literal masks.
- `tests/set.rs` (new): `VERDICT` of sets that break each rule is that rule's own sentence and of sets that keep it the empty string, both read as constants; the folded constants; the walk visits every member once in order, to arity sixteen, and stops where its visit refuses; the space between two pragmas' flags and none where one wrote nothing; what a `Site` hands a member; `each_tool` hands every need of a member.
- `tests/refused.rs` (new): scratch crates under the pinned cargo, each one line of a script, refused by `cargo check` with the sentence of its rule: a requirement of each of the three forms, a group held twice, a pragma named twice, a set of a profile not being built, a pragma of workspace reach in a script that does not claim, and a claiming prototype whose sets differ in a shared group. Beside each the same crate with the set that keeps the rule, which checks. The kept case of the claiming prototype holds one link driver in every profile's set.
- `tests/choice.rs` (new): what each pragma of the seven choices writes on each route, as literal text, with and without a tool's path, and what every pragma admits over one target of each object format.
- `tests/target.rs` (new): `Target::from_cargo` over a table of values, an unknown and an absent one among them.
- `tests/tools.rs` (new, replacing `tests/requirements.rs`): the table; a proof over the version lines the eight programs print, llvm-ar's with and without a vendor name, and lines that only look like them; `provision` over source, what is on the path and what is in the home; `tools_home` over each variable set, unset and empty on both kinds of host; `write_tool_dir` and `write_refusal` as literal text; `older` over a table of pairs, equal, longer, shorter and not numbers among them.
- `tests/generated.rs` (new): `write_generated` as literal text for `Workspace`, `Default` and prototypes of the test's own that `claimed` accepts, asserted in the test, and that between them hold every workspace pragma, a shared key once and no key twice; the committed file is the written one; `write_uncarried` as literal text over a target that does not admit a pragma and over a tool said to be lacked, in one set and in both, a second need of a pragma among them, and nothing for a prototype the build carries or for a tool with no source; `claim` over a file of this crate, of another, of a name that only starts the same, and of no claim; `includes_generated` over the forms of the line, a path that ends in the name among the ones it does not take.
- `tests/config.rs`, `tests/profile.rs`, `tests/prototype.rs`, `tests/bootstrap.rs`, `tests/wiring.rs`: follow the new signatures. The sets of the three prototypes are literal lists. The directives of `Default` and `Deterministic` are the literal text they were. `write_directives` and `configure_as` write the error line for a pragma the target does not admit and for a local-build tool with no path or a path with a space, and no link-argument line for a set that names a linker. The table of `plan` is over the enums, and the two new notices are literal text.
- `tests/script.rs`: the config file a first build writes has the include line.
- `tests/script_pragmas.rs` (new): a claiming script writes the generated file and a later release build's link takes its keys, beside a script that claims nothing; a second crate that claims fails the build and leaves the file, in a build that runs the two scripts side by side; a file edited by hand is written again and a current one is left; a file somebody else wrote fails the build and is left; a config file without the include line is left and the line named; a tool whose version is on the second line it prints is found; a set that names mold fails with the package's name under a path without mold, on every machine, and where the machine has mold links with it from the second build on, by the wrapper, and leaves mold's mark in the artifact; a set that names cranelift, under a sysroot the test makes with the backend and under one without, writes the key in the first and in the second fails with the rustup command and writes no file; a set that names wild fails with clang's package under a path without clang, on every machine, and where the machine has clang takes wild from a tools home the test fills with a stand-in, by its full path, and an older version beside it is left where nothing was built; a claiming set that names the clang C compiler, under a path without llvm-ar, fails with its package and writes no file.
- `tests/script_build.rs` (new): the local build, run whole. The build runs under a cargo home of the test's own, whose configuration replaces the registry with a directory the test writes, and in it a crate under wild's name and pin whose program answers the version question as wild does, so `cargo install` runs as it does for a consumer and reaches no network. The first build says it is building, leaves the program in the home and no scratch directory, removes a directory of an older version, leaves an older one whose lock the test holds, and leaves a newer one, and leaves the lock file of the version it removed. A lock file left by a process that is gone does not hold the build up. The second build builds nothing.

## The bench's configuration

`mock/benches/support/build-call` reads the target's facts, the linker marks and the cargo flags from this crate, `build-fixture` its profiles, and a `link-static` arm is a type over its linker pragma (tickets `build_call_s_target_spec_reading_linker_mark_and_cargo_flags`, `a_link_static_arm_carries_its_linker_and_quantity_in_a_type`). That source is not in this changelist. The job `build-bench-rerun` is writing its rows in the same files and has not landed them, so the two tickets carry the move to the round after this one.

## What a consumer sees

`script!(Default)`, `script!(Deterministic)` and `configure` keep their form and their text. Gone: `Pragma` the enum, `PragmaIter`, the mutators of `PragmaSet`, `configure_with`, `write_config_file`, `Prototype` the enum, `TargetAxis`, `TierAxis`, `PassesAxis`, `REQUIREMENTS` and its types, and `BuildConfig`'s `tier` and `passes`. `write_directives`, `write_rustflags`, `Tree::of` and `plan` change their signatures.

kaski is the one consumer of a part that goes. Twenty-eight of its build scripts call `configure_with` and take FastMath out of the profile's set: `kaski-engine`, the `kaski-engine-*` crates, `kaski-sim`, and the two configured arms of its `build-config` bench. They become `configure_as::<Deterministic, _, _>` in the same work, on the kaski branch `work/hilavitkutin-build-pragma-migration`, which lands by kaski's own flow straight after this round, with no ratchet. kaski takes this crate by git from `dev`, so its scripts cannot build in the new form before this round is there, and the branch is written and waits for it. The sets of `Deterministic` are the sets those scripts built, the defaults without FastMath and without MathPeephole, which the old coherence pass dropped with it, so the two bench arms measure what they measured.

## Comparison to deprecated changelist

### src/pragma.rs (rewritten)

The trait has three writers where the deprecated text had four: the writer of link arguments is gone. `TOOL`, one row, is `NEEDS`, a list of rows. `Mask` and `Words` are added, and the two private lists of places are said to be here, where the deprecated text had them under `roster.rs`. Unchanged: the trait `Pragma` with `NAME`, `BIT`, `REACH` and `UNMET`; `Reach` with `Crate` and `Workspace`; `Written`; `Shared` with `Owed`; `Group` with `CLASH` and `SHARED`; `Free`; `Requirement` with `ALL`, `ANY` and `NONE`; `PragmaSet` with `PragmaSet::of`; and the deletion of the enum and of `PragmaIter`.

### src/set.rs (new)

`each_tool` is added, and the fold names a pragma named twice before the two rules the deprecated text had. Unchanged: `Set` with `MASK`, `GROUPS`, `VERDICT`, `Visit`, `Site` and `ToolPath`, and the two assertions.

### src/roster.rs (new)

Unchanged: `LoopOptimization`, `Polly`, `MathPeephole`, `FastMath`, `ExpandedLto`, `Pgo`, `Bolt`, `Profiling`, `BuildStd`, `ParallelCodegen`, `SharedGenerics`, `LoopFusion` and `MimallocAllocator`, with what each writes and requires. The line about the lists of bits moved to `pragma.rs`.

### src/choice.rs (new)

`Mold`, `Bfd` and `Wild` write rustc flags, where the deprecated text followed a design that had a linker's text as a link argument of the script, and `Wild`, `GccCc` and `ClangCc` need more than one tool. Unchanged: `LinkerGroup`, `EdgeGroup`, `LtoGroup`, `IncrementalGroup`, `BackendGroup`, `DriverGroup`, `CcGroup`, and `DylibDomain`, `ThinLto`, `Incremental`, `NoIncremental`, `Cranelift`, `GccDriver` and `ClangDriver`.

### src/target.rs (new)

Unchanged: `Target`, `Os`, `Env`, `Arch`, `ObjectFormat`, `Isa` and `Target::from_cargo`.

### src/tools.rs (new)

`Home` is named, `older` is added, and `TOOLS` has four rows more. Unchanged: `Tool`, `Source`, `Proof`, `Line`, `Provision` and the four functions.

### src/generated.rs (new)

`write_generated` has two type parameters, the prototype and the writer, where the deprecated text wrote three, `write_uncarried` is added, and `includes_generated` takes the bare name alone. Unchanged: `GENERATED_NAME`, `Claim` and the two readers.

### src/prototype.rs (rewritten)

Unchanged: `Prototype` with `Dev`, `DevOpt`, `Release` and `Ci`, the three unit types `Default`, `Deterministic` and `Workspace`, the claiming assertion, and the deletion of the enum.

### src/profile.rs

Unchanged: `Profile::default_pragmas` is deleted and `Profile::name` added.

### src/config.rs

Unchanged: `BuildConfig` and its two constructors.

### src/flags.rs

`writes_flags` takes the set's site, since a pragma may write its flag only with its tool's path. Unchanged otherwise, and it answers a `Flags`.

### src/bootstrap.rs

No `cargo::rustc-link-arg` line, which the deprecated text had after the lines written before, and one more error line, for a tool's path that holds a space. Unchanged otherwise.

### src/wiring.rs

`INCLUDE_LINE` is named. Unchanged: `Unpacked`, `Placed`, `Host`, `Tree::of`, and `Notice` with `Generated` and `AddInclude`.

### src/script.rs

The tools half of the expansion is a second macro in a file of its own, `script!` asks to be run again when the generated file changes, and the expansion holds arrays where a `Vec` would be refused. The local build holds a lock on a file where the deprecated text had a lock directory, and removes the older versions whose lock nobody holds where the deprecated text removed every other. The generated file is placed by a hard link where there was none, and is left where `write_uncarried` writes anything. Unchanged otherwise.

### src/lib.rs

Unchanged.

### src/axis.rs, src/requirements.rs (deleted)

Unchanged: both are deleted, and with them `TargetAxis`, `TierAxis`, `PassesAxis` and `REQUIREMENTS`.

### outside the crate

The root `README.md` is named, and why `mock/Cargo.toml` keeps its two keys. Unchanged otherwise.

### tests

`tests/common/mod.rs` and `tests/harness/mod.rs` are named, the new cases of `tests/script.rs` are a binary of their own, `tests/script_pragmas.rs`, and the wild case uses a stand-in in the tools home. The cases added: a pragma named twice, a set of a profile not being built, the needs of every pragma, a path with a space, a file somebody else wrote, the committed generated file, `older`, `write_uncarried`, two claims side by side, cranelift under a sysroot of the test's own, and a tool the machine lacks under a path of the test's own. The local build, which the deprecated text left to a run by hand, is a binary, `tests/script_build.rs`.

### the bench's configuration

The deprecated text left open whether the bench's source would join this changelist. It does not: the job that holds those files has not landed, and the two tickets carry it.

### what a consumer sees

The lists are unchanged. The deprecated text named no consumer of `configure_with`. This one names kaski's twenty-eight scripts and where they move.
