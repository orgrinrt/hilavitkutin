# doc changelist

Round topic `202611192054-xugn_topic.every-build-choice-is-a-pragma.md`, the only flat topic. It holds op's answers with their options, what the canon says, and the four calls this changelist asks the stack coordinator's stamp on.

The design of `hilavitkutin-build` says a pragma is a type with requirements that are checked, and its code has an enum value and a function that repairs a set. This round writes the design so the crate can hold it, and adds the seven build choices op answered on as pragmas of the same kind. Everything below is in `mock/crates/hilavitkutin-build/`.

## Changes proposed

### `DESIGN.md.tmpl`, section "Pragma system"

Rewritten whole. A pragma is a unit type implementing `Pragma`: `NAME`, `BIT`, `REACH`, `UNMET`, `NEEDS`, `type Group`, `type Requires`, `admitted`, and three writers into a `core::fmt::Write`, one per route its text can take (`write_rustflags`, `write_cfgs`, `write_cargo_config`), each writing nothing by default. A linker's argument is a rustc flag, `-Clink-arg=`, and goes through the wrapper with the rest: the script prints no `cargo::rustc-link-arg`. A writer is handed a `Written`: the profile's name, the target triple and the path of the pragma's tool. No function returns a collection.

`Written` also carries a `Shared`, whether the keys the pragma's group shares across profiles are owed or already written. The section says the one form of text each route has: flags separated by single spaces, cfgs one to a line, and cargo configuration as whole lines in dotted keys with no table header.

`Requirement` is read as three masks. `All<(A, B)>` asks for every one, `Any<(A, B)>` for at least one, `Not<(A, B)>` for none, and `()` for nothing. `Group` is a choice a set holds one value of at most, a type that carries its own sentence for a set holding two; `Free` is the group of a pragma that is one value of no choice.

A group whose keys belong to no profile says so in `Group::SHARED`.

`Set` is implemented for flat tuples of pragmas, arities zero through sixteen. It folds its members into `MASK`, `GROUPS`, `SHARED`, `REACH` and `VERDICT`, the first thing wrong with the set or the empty string. `Set::each` hands every member to a `Visit`, and a walk is handed a `Site`, the profile, the triple and a `ToolPath` per tool found. `checked::<S>()` is the `const fn` that asserts over the verdict, and `crate_reach::<S>()` the one that refuses a pragma of workspace reach. `PragmaSet` stays as the folded value of a set, a mask built by `PragmaSet::of::<S>()`, read by `contains::<P>()`, with nothing that changes it; `PragmaSet::coherent`, `with`, `without`, `PragmaIter` and the `Pragma` enum go.

The thirteen pragmas the crate has become types under their names. `ParallelCodegen` takes its count as a const parameter. `MathPeephole` requires `All<(FastMath,)>` and `Profiling` requires `Any<(Pgo, Bolt)>`, which is what `coherent` repaired by hand.

The pragma table gains the rows of the seven groups:

| Group | Pragmas | Reach | Takes effect by |
|---|---|---|---|
| `LinkerGroup` | `Mold`, `Wild`, `Bfd` | crate | through the wrapper: `-Clink-arg=-fuse-ld=mold`, `-Clink-arg=-fuse-ld=bfd`, and for `Wild` the pair `-Clinker=clang -Clink-arg=--ld-path=<its path>` |
| `EdgeGroup` | `DylibDomain` | crate | `-Cprefer-dynamic` through the wrapper |
| `LtoGroup` | `ThinLto`, `ExpandedLto` | workspace | profile keys in the generated cargo file |
| `IncrementalGroup` | `Incremental`, `NoIncremental` | workspace | profile key in the generated cargo file |
| `BackendGroup` | `Cranelift` | workspace | `[unstable] codegen-backend` and the profile key |
| `DriverGroup` | `GccDriver`, `ClangDriver` | workspace | `[target.<triple>] linker` |
| `CcGroup` | `GccCc`, `ClangCc` | workspace | `CC`, `CXX` and `AR` under `[env]` |

Every linker's flags go by one route, the wrapper. This changelist had it otherwise twice. First `Wild`'s path was a link argument of the script beside a flag of the wrapper, and the argument reached the first build of a workspace, which runs without the wrapper, where gcc refused `--ld-path`: found when the source was written. Then `Mold` and `Bfd` kept the script's `cargo::rustc-link-arg` while `Wild` went through the wrapper, and the first review of the source read that directive as a fourth mechanism beside the three the canon's row names. So the first build of a workspace links as the toolchain does by itself, like every flag of that build, and the ones after it link with the linker the set names. A tool's path that holds a space is refused, since the wrapper splits on spaces.

The verdict of a set also names a pragma named twice. `Mask` and `Words` are the aliases the constants are written in. `each_tool` hands over a set's tools, and `writes_flags` asks a set at its site. A prototype of the script's own file is named by a path, `script!(crate::Sim)`. `tools_home` answers a `Home`. The claiming script asks to be run again when the generated file changes, and the include line is the constant `INCLUDE_LINE`.

A group with no pragma in the set leaves the choice to the toolchain. lld is the pinned toolchain's own linker on x86_64 Linux and has no pragma; gold links and rustc warns against it on every build, so it has none either. `Cranelift` requires `Not<(ThinLto, ExpandedLto)>`. `zig cc`, as a driver and as a C compiler, waits in the backlog on the prebuilt source.

### `DESIGN.md.tmpl`, new section "Reach"

`Reach::Crate` is the crate whose script names the set: the wrapper's flags, the linker's among them, and the cfg. `Reach::Workspace` is cargo configuration, which reaches every crate cargo builds, the dependencies and the build scripts among them. A pragma of workspace reach is named by one script of the workspace, the one written `script!(<prototype>, workspace)`. Every other script asserts `crate_reach` over its sets, so naming such a pragma there does not compile.

The workspace's script writes `.cargo/hilavitkutin-build.toml` beside the wrapper, with the text of `write_generated`. Its first line names the crate that wrote it, which `claim` reads back as a `Claim`. A script that claims the workspace and finds the file written by another crate writes nothing and fails the build with a `cargo::error` naming both crates. `config.toml` takes the file by `include = [{ path = "hilavitkutin-build.toml", optional = true }]`: `write_cargo_config` writes that line beside the wrapper's, and a `config.toml` that exists without it is left alone and the notice names the line, as the wrapper's line is handled today. The build that writes the file runs before cargo has read it, and the next one takes the keys. The tables are written for `dev` and `release`; the three custom profiles stay design until cargo tells a script which one it builds.

The file is dotted keys with no table header. A key of no profile is written once, the pragma being told through `Shared`, and `claimed::<P>()` refuses at check a claiming prototype whose sets hold different pragmas of a shared group, since one key twice is a file cargo cannot read. `Notice` gains the two things said about the file, and `includes_generated` reads the include line from a config file's text.

Three things the first review of the source asked of this section. Where there was no file the script places its own by a hard link, which refuses where a file is already there, so two scripts that both found none cannot both write: one places its file and the other reads whose it is and fails the build. The file is written only where the build can carry every key in it: before writing, the claiming script looks at every pragma of workspace reach in the `dev` and the `release` set, and `write_uncarried` writes a `cargo::error` for each the target does not admit or whose tool the machine lacks, and where it wrote any the file is left as it was. And the section says that a link driver's key is for the target of the build that wrote the file, so builds for two targets in turn write the file at every turn.

What is not checked: a requirement between a pragma of one script and a pragma of another. `DylibDomain` in one crate and an LTO pragma in the workspace's set are each accepted, and the design says so.

### `DESIGN.md.tmpl`, new section "The target and what it admits"

`Target` is the target's facts, read from `CARGO_CFG_TARGET_OS`, `CARGO_CFG_TARGET_ENV`, `CARGO_CFG_TARGET_ARCH` and `CARGO_CFG_TARGET_FEATURE` through the script's closure: `Os`, `Env`, `Arch`, the `ObjectFormat` that follows from them, and `Isa`, which is the instruction-set enum `TargetAxis` under the name of what it is. `Pragma::admitted` takes the target and answers with the sentence of a refusal or with nothing. `Mold`, `Wild` and `Bfd` admit ELF only; `Cranelift` and `DylibDomain` do not admit wasm. `Bolt`, ELF only as well, is not held to it yet: it sits in every prototype's release set and writes nothing, and refusing it would fail every release build on macOS.

A pragma the target does not admit fails the build: `configure` writes `cargo::error=` with the pragma's name, the fact and the remedy, and the same for a pragma whose tool is a local build with no path in its `Site`. The environment enum is reached as `target::Env`, the root having the closure's trait under that name. The script that claims the workspace does the same for a pragma whose tool the machine lacks.

`TierAxis` and `PassesAxis` are deleted. The section "Four-part build configuration model" loses its three axes and says the flags of a crate follow from its set.

### `DESIGN.md.tmpl`, new section "Tools"

`TOOLS` is one table of `Tool` rows: name, pinned version, `Source` and `Proof`. `Source` is `Package` (with the package's name), `Component` (with the rustup component's name), `LocalBuild` (with the crate cargo installs) or `Awaited`, the plugin and profile tools of the thirteen older pragmas, which have no source yet and whose pragmas keep writing what they can without them. `Proof` is the tool's version line and, for a linker, the mark it leaves in `.comment`. The rows: mold a package, wild a local build pinned at 0.10.0, clang and gcc packages, cranelift the component `rustc-codegen-cranelift-preview`. `REQUIREMENTS`, `Requirement` (the enum of external tools), `PragmaRequirement` and `requirements_for` go; a pragma names its tools in `Pragma::NEEDS`.

`write_tool_dir` writes the directory a tool has under a tools home, `<home>/<host triple>/<name>-<version>`. The home is `HILAVITKUTIN_BUILD_TOOLS_HOME` where the builder sets it, and the per-user cache where not (`$XDG_CACHE_HOME/hilavitkutin-build/tools`, else `$HOME/.cache/hilavitkutin-build/tools`, on macOS `$HOME/Library/Caches/hilavitkutin-build/tools`), decided by `tools_home`.

`provision` answers what the script does about one tool from what the script found, as a `Provision`: `Use` the one on the path where its version line holds the pin, `Use` the one in the home, `Build` it into the home, or `Refuse` with the command that brings a package or a component. Where it builds, the `script!` expansion runs `cargo install --locked --version <pin> --root <dir> <crate>`, says so in a `cargo::warning` first, and removes every `<name>-<version>` directory beside the new one whose version `older` says is below the pin. A newer one is another clone's pin and stays. A pragma's writer takes the path of its tool, so a locally built linker is named by its full path and needs nothing on `PATH`. A `Proof` holds a `Line`. The script that builds holds the operating system's lock on a file beside the tool's directory, which goes with the process, so an interrupted build leaves nothing to clear. It gives the child cargo a target directory of its own and none of the outer build's wrapper or flags, and a refusal is the text of `write_refusal`.

What the second review of the source asked of this section. A pragma names every tool it needs, in `Pragma::NEEDS`, rows of `TOOLS`, where it named one in `Pragma::TOOL`. `Wild` writes `-Clinker=clang` and waited on wild alone, so a machine without clang got rustc's own error; `ClangCc` puts `clang++` and `llvm-ar` under `[env]` and was asked about clang alone, so the generated file could hold a key the machine cannot follow. `Wild` needs wild and clang, `GccCc` gcc, g++ and ar, `ClangCc` clang, clang++ and llvm-ar. The table gains the four rows, each a package. The first of a pragma's needs is the one whose path its writer is handed.

What the third review of the source asked of this section. An older version is removed only where the script takes that version's lock at once: a held lock is a clone that pins the version and is building it now, so its directory stays. A lock file is never removed, since a script that removed one could take it from under a script waiting on it; the files are empty and stay beside the directories. A `Proof` is read off the first two lines a tool prints when asked for its version, joined by a space, where it was read off the first alone: an llvm-ar built with no vendor name prints `LLVM (http://llvm.org/):` first and its `LLVM version` on the second line, and was refused as missing while installed.

### `DESIGN.md.tmpl`, section "Entry point"

`BuildConfig::from_env` reads through the closure and `BuildConfig::from_cargo` takes the values already read. `write_directives` is generic over the set and takes its `Site`. `Profile::name` is the profile as cargo spells it.

### `DESIGN.md.tmpl`, section "Prototypes and the wiring a workspace takes"

`Prototype` is a trait with one set per profile (`Dev`, `DevOpt`, `Release`, `Profiling`, `Ci`). `Default` and `Deterministic` are unit types implementing it with the sets the profiles have today, less `ExpandedLto`, which is of workspace reach. `Deterministic` has neither `FastMath` nor `MathPeephole`, which stands on it and which `coherent` took out before. `Workspace` is `Default` with `ExpandedLto` in `Release` and `Ci`, for the script that claims the workspace. The `Ci` set is selected by no build yet, like the other custom profiles' sets, and no key is written for it; the design says so beside the sets. `script!(Default)` names one of the crate's own by its bare name, and `script!(my_build::Sim)` a consumer's by its path. The expansion holds one `const _` per profile set, so a broken set is refused by a check at the script's line.

`configure` keeps its signature and takes `Default`. `configure_as::<P, _, _>` is the same for a named prototype, and replaces `configure_with` and its closure over a set. `Tree::of` and `plan` take enums where they took `bool` (`Unpacked`, `Placed`, `Host`, `Flags`), which the review of the round before this asked for.

In this repository the engine crate `hilavitkutin` claims the workspace with `script!(Workspace, workspace)`, and `mock/.cargo/config.toml` gains the include line. `mock/Cargo.toml` already sets fat LTO and one codegen unit for `release`, so the generated file changes nothing a release build does.

### `DESIGN.md.tmpl`, sections "Default pragma sets per profile", "Five profiles" and "Config files"

The default sets table is split in two, the crate's sets and the workspace's. The "ci, same as release" line and the profile table's thin LTO for ci are made one statement: ci is `ExpandedLto` as the code has it. The profile table's line that dev uses cranelift for workspace crates is taken out: no profile's default set holds a pragma of the seven groups other than `ExpandedLto`, and a default changes later, per target, by a change that cites its bench run directory where the default is declared. The config files table gains `.cargo/hilavitkutin-build.toml` and loses the file under `target/` that nothing wrote or read; `write_config_file` goes.

### `DESIGN.md.tmpl`, sections "Flag ownership by pragma" and "Plugin and container sources"

The paragraph on `PragmaSet::coherent` goes. The two mentions of `configure().run()` go; the crate never had that call.

### `README.md.tmpl`

One sentence added: the seven build choices are pragmas, and one script of a workspace names the ones that reach all of it.

### `BACKLOG.md.tmpl`

Entries added, names unbackticked: zig cc as a driver and a C compiler with a prebuilt source checked against a digest; an lld pragma for targets whose toolchain does not link with it by default; the three custom profiles reaching the generated cargo file; a requirement checked between two scripts' sets.

## What the mechanism rests on

Sketches, each with the compiler's or cargo's output in its `FINDINGS.md` under `mock/research/sketches/`:

- `202611192215_dotted-keys-and-an-error-directive`: the generated file in dotted keys, one key twice refused by cargo before any script runs, and `cargo::error` failing a build after the script ran to its end.
- `202611192110_a-flat-tuple-set-refused-under-cargo-check`: the set, the three requirement forms, the group and the reach refusal, each seen refused under `cargo check`.
- `202611192125_cargo-takes-a-generated-config-by-include`: the include line with no file, then with the file.
- `202611192150_each-linker-by-its-link-arguments`: mold, GNU ld and wild each linking and leaving its mark, wild refused under gcc.
- `202611191615_cargo-keys-for-driver-backend-and-cc` and `202611191735_lto-and-incremental-are-profile-keys`: the keys of the workspace's groups.
- `202611191600_a-link-flag-reaches-the-final-crate-alone` and `202611191830_where-prefer-dynamic-has-to-reach`: the two groups of crate reach.

## The points to look at

The canon's row on mechanisms names three, and the generated cargo file is read here as the first of them, generated configuration, and every flag of a linker goes by the second, the wrapper. The same row puts `FastMath` under the wrapper, and the crate sets its cfg by a directive of the script, which is older than this round and is not changed in it; that goes to op with the question below. The canon's row on the builder names `configure().profile(..).run()`; the round keeps what that row states, sets enabled per profile as tuples of types, and has a prototype where the row's mechanism has a closure. Neither row is edited in this round. That the row's mechanism names a call the crate never had goes to op as a question with this round's landing.

Two public contracts change for a consumer: `Pragma` the enum, `PragmaSet`'s mutators and `configure_with` are gone, and `BuildConfig` loses `tier` and `passes` and has `Target` for its target. kaski has twenty-eight build scripts that call `configure_with` and take FastMath out of the set; they move to `configure_as::<Deterministic, _, _>`, whose sets are the ones those scripts built, on a kaski branch that lands straight after this round. Its other scripts call `script!` or `configure`, which keep their form. The frozen script of `hilavitkutin-frozen-authors` calls `configure` and is not touched.

A build script that runs `cargo install` is slow once and reaches the network. It is the only place that is in every build and needs no step by hand, which is op's condition, and it happens only where a set names a local-build pragma and the tool is on the machine nowhere.

## What waits outside this changelist

The bench's `build-call` and `build-fixture` reading the target's facts, the linker marks and the profiles from this crate, and the `link-static` arm as a type over its linker pragma, are source under `mock/benches/support/` and belong to the source changelist. The move of `build-call` waits there on the job `build-bench-rerun` landing its rows.
