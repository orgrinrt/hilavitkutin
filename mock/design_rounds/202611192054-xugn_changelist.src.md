# src changelist

Round topic `202611192054-xugn_topic.every-build-choice-is-a-pragma.md`, the only flat topic, and doc changelist `202611192054-xugn_changelist.doc.lock.md`.

This changelist replaces `202611192054-xugn_changelist.src.deprecated.md`. The source that one lists was written, at commit `66388cfc`, and writing it showed one defect of the design and a handful of names the design did not have. The doc changelist was reopened for them, and this text is the source as it stands. The last section says what differs from the deprecated text.

The crate's pragmas become types, a set a tuple of them, and the seven build choices pragmas among the rest. Everything below is under `mock/crates/hilavitkutin-build/` unless a path says otherwise. The mechanism follows the sketches the doc changelist names; the set and its refusal under check are `202611192110_a-flat-tuple-set-refused-under-cargo-check`, the generated file's form is `202611192215_dotted-keys-and-an-error-directive`, and wild's pair of flags is `202611192150_each-linker-by-its-link-arguments`.

## src/pragma.rs (rewritten)

- `Pragma`, the trait of the design: `NAME`, `BIT`, `REACH`, `UNMET`, `TOOL`, `type Group`, `type Requires`, `admitted`, and the four writers, each writing nothing by default.
- `Reach`, `Crate` or `Workspace`.
- `Written`, what a writer is handed, with its accessors, and `Shared`, `Owed` or `Written`.
- `Group` with `BIT`, `CLASH` and `SHARED`, and `Free`.
- `Requirement` with `ALL`, `ANY` and `NONE`, implemented for `()`, `All<L>`, `Any<L>` and `Not<L>` over a list that is a `Set`.
- `PragmaSet`, a mask with `PragmaSet::of::<S>()` and `contains::<P>()` and nothing that changes it.
- `Mask` and `Words`, the aliases a mask and a name or a sentence are written in.
- The place of every pragma and of every group in a mask, two private enums, one list each.
- The enum `Pragma`, `PragmaIter`, `with`, `without`, `coherent`, `iter` and `parallel_codegen_units` are deleted.

## src/set.rs (new)

- `Set`, implemented by one macro for the flat tuples of arity zero through sixteen: `MASK`, `GROUPS`, `SHARED`, `REACH`, `VERDICT`, and `each`, which hands every member to a `Visit`.
- `Visit`, one method generic over the member.
- `Site`, what a walk is handed, and `ToolPath`. `each_tool` is the walk over a set's tools.
- `checked::<S>()` and `crate_reach::<S>()`, the two `const fn` a script's `const` items call.
- The fold of a verdict is a private `const fn` over one row per member: a pragma named twice, then a group held twice, then a requirement unmet.

## src/roster.rs (new)

- The thirteen older pragmas as unit types: `LoopOptimization`, `Polly`, `MathPeephole`, `FastMath`, `ExpandedLto`, `Pgo`, `Bolt`, `Profiling`, `BuildStd`, `ParallelCodegen<const N: u8>`, `SharedGenerics`, `LoopFusion`, `MimallocAllocator`.
- `LoopOptimization` writes its five options, `ParallelCodegen` `-Zthreads=N`, `SharedGenerics` its flag and `FastMath` its cfg, as `flags.rs` and `bootstrap.rs` wrote them. `MathPeephole` requires `All<(FastMath,)>` and `Profiling` `Any<(Pgo, Bolt)>`.
- `ExpandedLto` is of workspace reach, in `LtoGroup`, and writes its two profile keys.

## src/choice.rs (new)

- The groups `LinkerGroup`, `EdgeGroup`, `LtoGroup`, `IncrementalGroup`, `BackendGroup`, `DriverGroup` and `CcGroup`, the last two shared.
- The pragmas `Mold`, `Wild`, `Bfd`, `DylibDomain`, `ThinLto`, `Incremental`, `NoIncremental`, `Cranelift`, `GccDriver`, `ClangDriver`, `GccCc` and `ClangCc`, each writing what the design's table of the seven choices says, and each admitting the targets the design says.
- `Wild` writes its driver and its path as one pair of rustc flags, and nothing without a path.

## src/target.rs (new)

- `Target` with its `Os`, `Env`, `Arch`, `ObjectFormat` and `Isa`, and `Target::from_cargo` over the four values a script read.

## src/tools.rs (new)

- `Tool`, `Source`, `Proof`, `Line`, the rows as constants and `TOOLS` over them.
- `tools_home` with `Home`, `write_tool_dir`, `provision` with `Provision`, and `write_refusal`.

## src/generated.rs (new)

- `GENERATED_NAME`, the generated file's name.
- `write_generated::<P, _>`, the file's text for a prototype: the line that names the crate, then the keys of the `dev` set and of the `release` set, a shared key once.
- `Claim` and `claim`, and `includes_generated`.

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

- `write_directives::<S, _>` takes the `BuildConfig` and the set's `Site`. After the lines it wrote before it writes one `cargo::rustc-link-arg` per link argument, and one `cargo::error` per pragma the target does not admit, per local-build tool the site has no path for, and per such tool whose path holds a space.
- `configure` is `configure_as::<Default, _, _>`. `configure_as` asserts `checked` and `crate_reach` over the prototype's five sets in an inline `const`, picks the set of the profile, writes its directives and keeps the warning for dropped flags.
- `configure_with`, `resolve` and `write_config_file` are deleted.

## src/wiring.rs

- `Unpacked`, `Placed`, `Host` and `Flags`, which `Tree::of` and `plan` take where they took `bool`.
- `INCLUDE_LINE`, and `write_cargo_config` writes it above the `[build]` table.
- `Notice` gains `Generated` and `AddInclude`, and `write_notice` their lines.

## src/script.rs, src/script_tools.rs (new)

- `script!` takes a prototype by bare name or by path, and either with `, workspace` after it.
- The expansion holds a `const` item per set that calls `checked` and `crate_reach`, or one that calls `claimed` in the workspace's script.
- It resolves the build, picks the set of the profile, and for every tool of the set does what `provision` answers: nothing, a `cargo::error` with `write_refusal`, or the local build under a lock directory, with a warning before it and the older versions removed after it. That half is a second macro the first expands, in `src/script_tools.rs`, so neither file passes the size limit.
- It writes the directives with the tools it found, and the wiring as before.
- The workspace's script then reads the generated file and `config.toml`, and writes the file where there is none or where it holds another text for this crate, under a name of its own first. A file claimed by another crate is left and a `cargo::error` names both. It asks to be run again when the file changes.
- The expansion names no `Vec`: the tools of a set are at most sixteen and sit in arrays.

## src/lib.rs

- The modules above are declared, `axis` and `requirements` are gone, and the exports follow. The crate doc's layout and roster say what the crate has.

## src/axis.rs, src/requirements.rs (deleted)

## Outside the crate

- `mock/crates/hilavitkutin/build.rs` is `script!(Workspace, workspace)`.
- `mock/.cargo/config.toml` gains the include line, and `mock/.cargo/hilavitkutin-build.toml` is the file the engine crate's script writes, committed.
- `mock/Cargo.toml`: the comment over the release profile says where `ExpandedLto` is written now.
- `mock/crates/hilavitkutin-frozen-authors/build.rs` is not touched.

## Tests

- `tests/common/mod.rs` (new): the list of every pragma, named once, which the binaries below share.
- `tests/harness/mod.rs` (new): the scratch workspace and the spying rustc, moved out of `tests/script.rs` for the two binaries below that also build one.
- `tests/pragma.rs` (rewritten): every two pragma bits and every two group bits apart, by a list that names each type once and is held to the count; the group, the reach and the tool of every pragma against a table; `PragmaSet::of` and `contains` over sets that hold and do not hold a pragma; the three requirement forms as literal masks.
- `tests/set.rs` (new): `VERDICT` of sets that break each rule is that rule's own sentence and of sets that keep it the empty string, both read as constants; the folded constants; the walk visits every member once in order, to arity sixteen, and stops where its visit refuses; the space between two pragmas' flags and none where one wrote nothing; what a `Site` hands a member.
- `tests/refused.rs` (new): scratch crates under the pinned cargo, each one line of a script, refused by `cargo check` with the sentence of its rule: a requirement of each of the three forms, a group held twice, a pragma named twice, a set of a profile not being built, a pragma of workspace reach in a script that does not claim, and a claiming prototype whose sets differ in a shared group. Beside each the same crate with the set that keeps the rule, which checks.
- `tests/choice.rs` (new): what each pragma of the seven choices writes on each route, as literal text, with and without a tool's path, and what every pragma admits over one target of each object format.
- `tests/target.rs` (new): `Target::from_cargo` over a table of values, an unknown and an absent one among them.
- `tests/tools.rs` (new, replacing `tests/requirements.rs`): the table; a proof over the version lines the four programs print and lines that only look like them; `provision` over source, what is on the path and what is in the home; `tools_home` over each variable set, unset and empty on both kinds of host; `write_tool_dir` and `write_refusal` as literal text.
- `tests/generated.rs` (new): `write_generated` as literal text for `Workspace`, `Default` and prototypes of the test's own that hold every workspace pragma, a shared key once and no key twice; the committed file is the written one; `claim` over a file of this crate, of another, of a name that only starts the same, and of no claim; `includes_generated` over the forms of the line.
- `tests/config.rs`, `tests/profile.rs`, `tests/prototype.rs`, `tests/bootstrap.rs`, `tests/wiring.rs`: follow the new signatures. The sets of the three prototypes are literal lists. The directives of `Default` and `Deterministic` are the literal text they were. `write_directives` and `configure_as` write the error line for a pragma the target does not admit and for a local-build tool with no path or a path with a space. The table of `plan` is over the enums, and the two new notices are literal text.
- `tests/script.rs`: the config file a first build writes has the include line.
- `tests/script_pragmas.rs` (new): a claiming script writes the generated file and a later release build's link takes its keys, beside a script that claims nothing; a second crate that claims fails the build and leaves the file; a file edited by hand is written again and a current one is left; a file somebody else wrote fails the build and is left; a config file without the include line is left and the line named; a set that names mold links with it, or fails with the package's name where the machine has none; a set that names cranelift fails with the rustup command where the toolchain lacks the component; a set that names wild takes it from a tools home the test fills with a stand-in, by its full path, and an older version beside it is left where nothing was built.
- Not held by a build: the local build itself, which fetches from the registry and takes minutes. Its decision is `provision`, held in `tests/tools.rs`, and the command was run by hand for wild 0.10.0 on this machine, which is where the tools home the linker sketch used came from. The lock directory and the removal of older versions are in that same path and are not run by a test either.

## The bench's configuration

`mock/benches/support/build-call` reads the target's facts, the linker marks and the cargo flags from this crate, `build-fixture` its profiles, and a `link-static` arm is a type over its linker pragma (tickets `build_call_s_target_spec_reading_linker_mark_and_cargo_flags`, `a_link_static_arm_carries_its_linker_and_quantity_in_a_type`). That source is not in this changelist. The job `build-bench-rerun` is writing its rows in the same files and has not landed them, so the two tickets carry the move to the round after this one.

## What a consumer sees

`script!(Default)`, `script!(Deterministic)` and `configure` keep their form and their text. Gone: `Pragma` the enum, `PragmaIter`, the mutators of `PragmaSet`, `configure_with`, `write_config_file`, `Prototype` the enum, `TargetAxis`, `TierAxis`, `PassesAxis`, `REQUIREMENTS` and its types, and `BuildConfig`'s `tier` and `passes`. `write_directives`, `write_rustflags`, `Tree::of` and `plan` change their signatures.

## Comparison to deprecated changelist

### src/pragma.rs (rewritten)

`Mask` and `Words` are added, and the two private lists of places are said to be here, where the deprecated text had them under `roster.rs`. Unchanged: the trait `Pragma` with `NAME`, `BIT`, `REACH`, `UNMET` and `TOOL`; `Reach` with `Crate` and `Workspace`; `Written`; `Shared` with `Owed`; `Group` with `CLASH` and `SHARED`; `Free`; `Requirement` with `ALL`, `ANY` and `NONE`; `PragmaSet` with `PragmaSet::of`; and the deletion of the enum and of `PragmaIter`.

### src/set.rs (new)

`each_tool` is added, and the fold names a pragma named twice before the two rules the deprecated text had. Unchanged: `Set` with `MASK`, `GROUPS`, `VERDICT`, `Visit`, `Site` and `ToolPath`, and the two assertions.

### src/roster.rs (new)

Unchanged: `LoopOptimization`, `Polly`, `MathPeephole`, `FastMath`, `ExpandedLto`, `Pgo`, `Bolt`, `Profiling`, `BuildStd`, `ParallelCodegen`, `SharedGenerics`, `LoopFusion` and `MimallocAllocator`, with what each writes and requires. The line about the lists of bits moved to `pragma.rs`.

### src/choice.rs (new)

`Wild` writes one pair of rustc flags, where the deprecated text followed a design that had its path as a link argument. Unchanged: `LinkerGroup`, `EdgeGroup`, `LtoGroup`, `IncrementalGroup`, `BackendGroup`, `DriverGroup`, `CcGroup`, and `Mold`, `Bfd`, `DylibDomain`, `ThinLto`, `Incremental`, `NoIncremental`, `Cranelift`, `GccDriver`, `ClangDriver`, `GccCc` and `ClangCc`.

### src/target.rs (new)

Unchanged: `Target`, `Os`, `Env`, `Arch`, `ObjectFormat`, `Isa` and `Target::from_cargo`.

### src/tools.rs (new)

`Home` is named. Unchanged: `Tool`, `Source`, `Proof`, `Line`, `TOOLS`, `Provision` and the four functions.

### src/generated.rs (new)

`write_generated` has two type parameters, the prototype and the writer, where the deprecated text wrote three. Unchanged: `GENERATED_NAME`, `Claim` and the two readers.

### src/prototype.rs (rewritten)

Unchanged: `Prototype` with `Dev`, `DevOpt`, `Release` and `Ci`, the three unit types `Default`, `Deterministic` and `Workspace`, the claiming assertion, and the deletion of the enum.

### src/profile.rs

Unchanged: `Profile::default_pragmas` is deleted and `Profile::name` added.

### src/config.rs

Unchanged: `BuildConfig` and its two constructors.

### src/flags.rs

`writes_flags` takes the set's site, since a pragma may write its flag only with its tool's path. Unchanged otherwise, and it answers a `Flags`.

### src/bootstrap.rs

One more error line, for a tool's path that holds a space. Unchanged otherwise.

### src/wiring.rs

`INCLUDE_LINE` is named. Unchanged: `Unpacked`, `Placed`, `Host`, `Tree::of`, and `Notice` with `Generated` and `AddInclude`.

### src/script.rs

The tools half of the expansion is a second macro in a file of its own, `script!` asks to be run again when the generated file changes, and the expansion holds arrays where a `Vec` would be refused. Unchanged otherwise.

### src/lib.rs

Unchanged.

### src/axis.rs, src/requirements.rs (deleted)

Unchanged: both are deleted, and with them `TargetAxis`, `TierAxis`, `PassesAxis` and `REQUIREMENTS`.

### outside the crate

Unchanged.

### tests

`tests/common/mod.rs` and `tests/harness/mod.rs` are named, the new cases of `tests/script.rs` are a binary of their own, `tests/script_pragmas.rs`, and the wild case uses a stand-in in the tools home. The cases added: a pragma named twice, a set of a profile not being built, the tool of every pragma, a path with a space, a file somebody else wrote, and the committed generated file. The lock directory and the removal of older versions are named among what no build holds.

### the bench's configuration

The deprecated text left open whether the bench's source would join this changelist. It does not: the job that holds those files has not landed, and the two tickets carry it.

### what a consumer sees

Unchanged.
