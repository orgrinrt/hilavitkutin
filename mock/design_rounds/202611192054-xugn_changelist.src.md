# src changelist

Round topic `202611192054-xugn_topic.every-build-choice-is-a-pragma.md`, the only flat topic, and doc changelist `202611192054-xugn_changelist.doc.lock.md`.

The crate's pragmas become types, a set a tuple of them, and the seven build choices pragmas among the rest. Everything below is under `mock/crates/hilavitkutin-build/` unless a path says otherwise. The mechanism follows the sketches the doc changelist names; the set and its refusal under check are `202611192110_a-flat-tuple-set-refused-under-cargo-check`, and the generated file's form is `202611192215_dotted-keys-and-an-error-directive`.

## src/pragma.rs (rewritten)

- `Pragma`, the trait of the design: `NAME`, `BIT`, `REACH`, `UNMET`, `TOOL`, `type Group`, `type Requires`, `admitted`, and the four writers, each writing nothing by default.
- `Reach`, `Crate` or `Workspace`.
- `Written`, what a writer is handed, with its accessors, and `Shared`, `Owed` or `Written`.
- `Group` with `BIT`, `CLASH` and `SHARED`, and `Free`.
- `Requirement` with `ALL`, `ANY` and `NONE`, implemented for `()`, `All<L>`, `Any<L>` and `Not<L>` over a list that is a `Set`.
- `PragmaSet`, a mask with `of::<S>()` and `contains::<P>()` and nothing that changes it.
- The enum `Pragma`, `PragmaIter`, `with`, `without`, `coherent`, `iter` and `parallel_codegen_units` are deleted.

## src/set.rs (new)

- `Set`, implemented by one macro for the flat tuples of arity zero through sixteen: `MASK`, `GROUPS`, `SHARED`, `REACH`, `VERDICT`, and `each`, which hands every member to a `Visit`.
- `Visit`, one method generic over the member.
- `Site`, what a walk is handed, and `ToolPath`.
- `checked::<S>()` and `crate_reach::<S>()`, the two `const fn` a script's `const` items call.
- The fold of a verdict is a private `const fn` over one row per member.

## src/roster.rs (new)

- The thirteen older pragmas as unit types: `LoopOptimization`, `Polly`, `MathPeephole`, `FastMath`, `ExpandedLto`, `Pgo`, `Bolt`, `Profiling`, `BuildStd`, `ParallelCodegen<const N: u8>`, `SharedGenerics`, `LoopFusion`, `MimallocAllocator`.
- `LoopOptimization` writes its five options, `ParallelCodegen` `-Zthreads=N`, `SharedGenerics` its flag and `FastMath` its cfg, as `flags.rs` and `bootstrap.rs` wrote them. `MathPeephole` requires `All<(FastMath,)>` and `Profiling` `Any<(Pgo, Bolt)>`.
- `ExpandedLto` is of workspace reach, in `LtoGroup`, and writes its two profile keys.
- The bits of every pragma and of every group are two private enums in this file's sibling `pragma.rs`, one list each.

## src/choice.rs (new)

- The groups `LinkerGroup`, `EdgeGroup`, `LtoGroup`, `IncrementalGroup`, `BackendGroup`, `DriverGroup` and `CcGroup`, the last two shared.
- The pragmas `Mold`, `Wild`, `Bfd`, `DylibDomain`, `ThinLto`, `Incremental`, `NoIncremental`, `Cranelift`, `GccDriver`, `ClangDriver`, `GccCc` and `ClangCc`, each writing what the design's table of the seven choices says, and each admitting the targets the design says.

## src/target.rs (new)

- `Target` with its `Os`, `Env`, `Arch`, `ObjectFormat` and `Isa`, and `Target::from_cargo` over the four values a script read.

## src/tools.rs (new)

- `Tool`, `Source`, `Proof`, `Line`, the rows as constants and `TOOLS` over them.
- `tools_home`, `write_tool_dir`, `provision` with `Provision`, and `write_refusal`.

## src/generated.rs (new)

- `GENERATED_NAME`, the generated file's name.
- `write_generated::<P, _, _>`, the file's text for a prototype: the line that names the crate, then the keys of the `dev` set and of the `release` set, a shared key once.
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

- `write_rustflags::<S, _>` and `writes_flags::<S>` are over a set's type, the second answering a `Flags`. The list of flags by `match` is deleted.

## src/bootstrap.rs

- `write_directives::<S, _>` takes the `BuildConfig` and the set's `Site`. After the lines it wrote before it writes one `cargo::rustc-link-arg` per link argument, and one `cargo::error` per pragma the target does not admit and per local-build tool the site has no path for.
- `configure` is `configure_as::<Default, _, _>`. `configure_as` asserts `checked` and `crate_reach` over the prototype's five sets in an inline `const`, picks the set of the profile, writes its directives and keeps the warning for dropped flags.
- `configure_with`, `resolve` and `write_config_file` are deleted.

## src/wiring.rs

- `Unpacked`, `Placed`, `Host` and `Flags`, which `Tree::of` and `plan` take where they took `bool`.
- `write_cargo_config` writes the include line above the `[build]` table.
- `Notice` gains `Generated` and `AddInclude`, and `write_notice` their lines.

## src/script.rs

- `script!` takes a prototype by bare name or by path, and either with `, workspace` after it.
- The expansion holds a `const` item that calls `checked` and `crate_reach` over each of the five sets, or `claimed` in the workspace's script.
- It resolves the build, picks the set of the profile, and for every tool of the set does what `provision` answers: nothing, a `cargo::error` with `write_refusal`, or the local build under a lock directory, with a warning before it and the older versions removed after it.
- It writes the directives with the tools it found, and the wiring as before.
- The workspace's script then reads the generated file and `config.toml`, and writes the file where there is none or where it holds another text for this crate, under a name of its own first. A file claimed by another crate is left and a `cargo::error` names both.

## src/lib.rs

- The modules above are declared, `axis` and `requirements` are gone, and the exports follow. The crate doc's layout and roster say what the crate has.

## src/axis.rs, src/requirements.rs (deleted)

## Outside the crate

- `mock/crates/hilavitkutin/build.rs` is `script!(Workspace, workspace)`.
- `mock/.cargo/config.toml` gains the include line, and `mock/.cargo/hilavitkutin-build.toml` is the file the engine crate's script writes, committed.
- `mock/Cargo.toml`: the comment over the release profile says where `ExpandedLto` is written now.
- `mock/crates/hilavitkutin-frozen-authors/build.rs` is not touched.

## Tests

- `tests/pragma.rs` (rewritten): every two pragma bits and every two group bits apart, by a list that names each type once and is held to the count; `PragmaSet::of` and `contains` over sets that hold and do not hold each pragma; the three requirement forms and the group and the shared constants as literal masks.
- `tests/set.rs` (new): `VERDICT` of sets that break each rule is that rule's own sentence and of sets that keep it the empty string, both read as constants; `REACH`; the walk visits every member once in order, to arity sixteen; the space between two pragmas' flags and none where one wrote nothing.
- `tests/refused.rs` (new): scratch crates under the pinned cargo, each one line of a script, refused by `cargo check` with the sentence of its rule: a requirement of each of the three forms, a group held twice, a pragma of workspace reach in a script that does not claim, and a claiming prototype whose sets differ in a shared group. Beside each the same crate with the set that keeps the rule, which checks.
- `tests/choice.rs` (new): what each pragma of the seven choices writes on each route, as literal text, with and without a tool's path, and what each admits over a table of targets.
- `tests/target.rs` (new): `Target::from_cargo` over a table of values, an unknown and an absent one among them.
- `tests/tools.rs` (new, replacing `tests/requirements.rs`): `provision` over the whole product of source, what is on the path and what is in the home; a version line that only looks like the pin; `tools_home` over each variable set and unset on both kinds of host; `write_tool_dir` and `write_refusal` as literal text.
- `tests/generated.rs` (new): `write_generated` as literal text for `Workspace` and for prototypes of the test's own that hold every workspace pragma, a shared key once; `claim` over a file of this crate, of another, of a name that only starts the same, and of no first line; `includes_generated` over the forms of the line.
- `tests/config.rs`, `tests/profile.rs`, `tests/prototype.rs`, `tests/bootstrap.rs`, `tests/wiring.rs`: follow the new signatures. The directives of `Default` and `Deterministic` are the literal text they were. `configure_as` writes the error line for a pragma the target does not admit and for a local-build tool with no path. The table of `plan` is over the enums, and the two new notices are literal text.
- `tests/script.rs`: the scratch workspace's first build writes the include line; a claiming script writes the generated file and the next build's rustc takes its key; a second crate that claims fails the build and leaves the file; a file edited by hand is written again; a set that names a tool the machine lacks fails the build with the command; a set that names wild takes it from a tools home the test fills, by its full path, and a tools home with an older version beside it is left as it is where nothing was built.
- Not held by a build: the local build itself, which fetches from the registry and takes minutes. Its decision is `provision`, held in `tests/tools.rs`, and the command was run by hand for wild 0.10.0 on this machine, which is where the tools home the linker sketch used came from.

## The bench's configuration

`mock/benches/support/build-call` reads the target's facts, the linker marks and the cargo flags from this crate, `build-fixture` its profiles, and a `link-static` arm is a type over its linker pragma (tickets `build_call_s_target_spec_reading_linker_mark_and_cargo_flags`, `a_link_static_arm_carries_its_linker_and_quantity_in_a_type`). That source is written once the job `build-bench-rerun` has its rows on `dev`, since it edits the same files. This changelist is reopened to list it then, or the two tickets carry it to the round after, whichever the job's landing allows.

## What a consumer sees

`script!(Default)`, `script!(Deterministic)` and `configure` keep their form and their text. Gone: `Pragma` the enum, `PragmaIter`, the mutators of `PragmaSet`, `configure_with`, `write_config_file`, `Prototype` the enum, `TargetAxis`, `TierAxis`, `PassesAxis`, `REQUIREMENTS` and its types, and `BuildConfig`'s `tier` and `passes`. `write_directives`, `write_rustflags`, `Tree::of` and `plan` change their signatures.
