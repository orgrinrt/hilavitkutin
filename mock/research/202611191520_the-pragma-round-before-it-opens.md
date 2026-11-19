# The pragma round, before it opens

19 November 2026. Holds for: hilavitkutin `dev` at `4acfbc7a` merged into `work/hilavitkutin-build-sets-a-consumer-up-by-itself-and-an`, the crate `hilavitkutin-build` as it is there, toolchain nightly-2026-05-28. Nothing here was built or run; every statement about the crate is a reading of its source or its design and says which.

The round this note prepares is the one the job ticket `hilavitkutin_build_carries_linker_backend_edge_form_c` asks for: every build choice of a consumer, per target, is carried by `hilavitkutin-build`. It cannot open yet. The consumer-setup round `202611151157` is open in this tree at its doc changelist and waits on op's view (question ticket `may_the_doc_changelist_of_the_consumer_setup_round_of`), and a tree holds one round at a time. This note is the material the topic is cut from once that round closes, so that the wait costs nothing.

It is a proposal by the session that will run the round. It binds nothing, and its changelist goes by op before it locks.

## What op said

Each answer is copied from the workspace record, `.shared/research/202611150830_stack-path/62-op-calls-on-the-build-axes-round.md` in clause-dev, with the options it was chosen among. Where that file and the plan before it (file 61) disagree, that file holds.

On how the crate models linker, link driver, backend, edge form, C compiler, LTO level and incremental, asked with three options ("Axes, LTO leaves pragmas", "Axes, keep ExpandedLto", "All as pragmas"), op: "All as pragmas". The intent, as read here: one mechanism. A choice with flags and requirements is a pragma, and no second kind of thing stands beside it.

On where locally built tools live, asked with two options (a per-user cache, or under `target/`), op, in his own words: "Configurable by the builder, default to per user cache and reuse existing where available. On upgrades, remove old versions too automstically". The intent, as read here, in four parts: the place is a setting of whoever builds, with the per-user cache as its default; a tool already on the machine that passes its proof is used and nothing is built; when a pin moves the old version's directory is removed by the tool; none of it is done by hand.

On defaults at landing, asked with two options, op: "Toolchain's own until benched (Recommended)", the option reading "every default equals today's behaviour at landing, and a default changes per target only when a bench cell shows the other value ahead, with its run directory cited where the default is declared". The intent: no profile's default set gains a linker, backend, LTO or C compiler pragma in this round.

On the bench's own configuration (file 63 of the same record), asked whether the bench branch may land with its configuration moved later, op: "Land, move config in the round (Recommended)". The intent: this round owes the move of every bench-held configuration site into the crate.

Earlier the same day (file 58), on what the crate is for: "We'll use hilavitkutin-build to set up the build and link harness per target, no ad hoc hacky shit in there. Proper, no duplication, reuse reuse reuse." and "hilavitkutin-build gets upstream abstractions and paths for all this configuration too". The intent: one definition of the build and link configuration, in the crate, read by the bench and by a real build alike.

## What the crate is today, and where its design and its code part

The design and the code disagree about what a pragma is, and the round has to start from that.

The design says a pragma is a type. `DESIGN.md.tmpl`, "Pragma system", declares a trait `Pragma` with a name, an associated `Requires`, and functions returning `Vec<String>`, and the combinators `All<(A, B)>` and `Any<(A, B)>`, "Checked at `configure().run()` time". The ratified tenet `build_behaviour_is_set_through_pragmas_enabled_per_profile_by_a_builder` records the same shape as its mechanism: `configure().profile("release", |p| p.enable::<(LoopOptimization, FastMath, ExpandedLto, Bolt, Pgo)>()).run()`.

The code says a pragma is a value. `src/pragma.rs` has one enum of thirteen variants and a bit mask over them. The two requirements between pragmas the design names are written out by hand in `PragmaSet::coherent` (MathPeephole without FastMath, Profiling without Pgo or Bolt), which removes the dependent pragma where the design says the set is refused. `src/requirements.rs` holds a second thing under nearly the same name, a table from a pragma to the external tool it waits on, and its module comment says the requirements between pragmas "belong to the pragma-resolution stage, which does not exist yet".

The design's own trait cannot be written as it stands either. The crate is `no_std` with no allocation (the provisional tenet `the_build_crate_is_no_std_takes_the_profile_and_target_strings_as_parameters_and_writes_its_text_into_a_formatting_writer`), and `Vec<String>` is neither. So the round does not choose between the design and the code. It writes the design's intent, typed pragmas with requirements that are checked, in a form the crate can hold, and the code under it is rewritten from that.

The three axis enums of `src/axis.rs` are data nothing computes from. The module says the mapping from a triple of them to a rustc call "is not computed here yet", and `BuildConfig::from_cargo` leaves the tier and the passes at their defaults in every build. The tier axis has four values that are each a combination of two pragmas already in the set (static, static with Bolt, Pgo, Pgo with Bolt), and the passes axis names which extension points the pass plugin registers at, which follows from the loop pragmas. Op's answer left these to the round: whether they fold into pragmas as well "is a question the round raises and does not assume" (the coordinator's reading in file 62). It is raised below.

On LTO the design states three things that cannot all hold (readings of `DESIGN.md.tmpl`): the pragma table has `ExpandedLto` as fat LTO with one codegen unit, the default sets say ci is the same as release, and the profile table gives ci thin LTO. The code gives ci `ExpandedLto` (`src/profile.rs`). A pragma for thin LTO beside it, the two excluding each other, lets the design say one thing.

## The proposal

### A pragma is a type, and a requirement is a bound

Every pragma is a unit type implementing one trait of the crate. The trait gives its name, the group it belongs to where it is one value of a choice, what it requires, the mechanism it takes effect by, and a function that writes its flags or its configuration lines into a `core::fmt::Write`, as every writer of the crate does now. Nothing is returned in a collection.

A profile's set is a type-level list of pragmas, which is what the tenet's `enable::<(A, B, C)>()` already spells. The workspace has the list this needs: `notko`'s hlist is a dependency of the workspace for the access sets, and the crate already depends on `notko`. Whether the set is that list or a const bit mask indexed by a constant of the trait is a call the round's sketches make by which of the two gives a compile error a reader can act on; the list is tried first.

Requirements between pragmas are then bounds, checked where the set is named, before any build script runs:

| Requirement | Meaning | Where it fails |
|---|---|---|
| `All<(A, B)>` | every one is in the set | compile error at the set |
| `Any<(A, B)>` | at least one is in the set | compile error at the set |
| exclusion within a group | at most one pragma of a group is in the set | compile error at the set |
| admission by the target | the target's facts allow the pragma | `configure`, as a refusal naming the pragma, the fact and the remedy |
| a tool the machine has | the tool of the pragma's row passes its proof | `configure`, the same way |

The first three are settled before the program runs, so they are types. The last two cannot be: a build script is compiled for the host and learns its target from cargo when it runs, and what the machine has is found by the caller on disk. They are checked in `configure` from what the closure hands over, and a pragma that fails one is refused in words. The record's reading of op's answer is that a pragma a target cannot carry "fails its requirement the same way a pragma with a missing plugin does"; in the code today a pragma waiting on a plugin writes nothing and says nothing (`src/flags.rs`), so the round gives both cases the one refusal. Nothing falls back in silence to another value. This differs from file 61, which proposed a fall to the target's default with a warning; op's answer puts admission among the requirements, and a failed requirement refuses.

`PragmaSet::coherent` goes, since a set that would need repairing no longer compiles.

### The seven choices as groups

One pragma per value that says something other than the toolchain's own. A group with no pragma in the set leaves that choice to the toolchain, which is what "toolchain's own until benched" needs: the default costs no pragma and no code.

| Group | Pragmas to start with | Takes effect by |
|---|---|---|
| linker | the bundled rust-lld, the system lld, mold, wild, gold, GNU ld | the wrapper, on the final crate alone |
| link driver | gcc, clang, zig cc, the linker called directly | cargo configuration the tool writes, per target |
| backend | cranelift | cargo configuration the tool writes, per profile |
| edge form | dylib domains | the facade manifest the tool writes, and `prefer-dynamic` through the wrapper on every crate |
| C compiler | gcc, clang, zig cc | the `CC` family of variables in the cargo configuration the tool writes |
| LTO | thin, and the existing `ExpandedLto` for fat | the generated profile config |
| incremental | on, off | the generated profile config |

Incremental has a pragma for each of its two values because cargo's own default differs by profile, so neither value is the same as saying nothing. Every other group has the toolchain's value as its absence.

Each row uses a mechanism the tenet `a_pragma_takes_effect_by_generated_config_by_the_compiler_wrapper_or_by_a_post_build_hook` already names, or the cargo configuration the maker tool of the consumer-setup round writes. The second is a fourth route beside the tenet's three and is a design change the changelist states as one: a driver, a backend and the `CC` variables are read by cargo before any build script runs, as the wrapper's path is, so a build script cannot set them and the tool must.

What a target admits is declared per pragma as a constant over the target's facts: mold, gold and wild on ELF only, cranelift not on wasm and with no LTO pragma beside it, dylib domains not on wasm and with no LTO across the edge, zig cc only where zig has the target. These are claims from the bench plan (file 59, section 4), each to be shown refused in a small cell before the declaration is believed.

### The target as facts

The crate reads the target as its facts, operating system, environment, architecture, object format and feature set, from the `CARGO_CFG_TARGET_*` values the closure already carries. The instruction-set enum `TargetAxis` becomes one field of that. The bench's `build-call` reads the same facts today from the target spec JSON by hand (ticket `build_call_s_target_spec_reading_linker_mark_and_cargo_flags`), which is the copy this replaces.

### Tools: one table, a home the builder sets

`REQUIREMENTS` is widened and stays one table. A tool has a name, a pinned version, a source and a proof. The source is one of four: a package of a trusted manager, a toolchain component, a prebuilt release checked against a digest, or a local build from a named source. The proof is the tool's version line and, for a linker, the mark it leaves in what it links.

The crate writes the path a tool of a given name and version has under a tools home for a host, as text. The home is a setting of the builder, read through the closure, and where it is unset the crate names the per-user cache directory. The pack's maker tool does the work on disk, in this sequence: where a tool already on the machine passes the proof it is used and nothing is built; else the tool is taken from its source into the path the crate names; and where the directory of another version of the same tool is found under the home it is removed. That is op's answer in full, and the tool's tests state each of the three as a case with the one that must fail.

By op's rule on packages (file 60 of the record) mold's row is a package, wild's is a local build pinned at 0.10.0 until the nightly pin moves and 0.11.0 after, cranelift's is a toolchain component, and zig's is a prebuilt release.

### Defaults

No profile's default set gains one of the new pragmas. The design's line that dev uses cranelift for workspace crates has no code and no cell behind it, and is taken out of the profile table until a cell earns it. Ci keeps what the code gives it today. A default changes later, per target, by a change that cites its bench run directory where the default is declared.

### The bench reads the crate

`mock/benches/support/build-call` and `build-fixture` take the target's facts, the default linker's mark, cargo's flags, the pinned toolchain and the fixture's profiles from the crate, and their own copies are deleted (ticket `build_call_s_target_spec_reading_linker_mark_and_cargo_flags`). This part waits until the job `build-bench-rerun` has landed its rows in `routine.rs`; the rest of the round does not wait on it.

A `link-static` arm is described today three times and its linker and quantity are parsed back out of its name (ticket `a_link_static_arm_carries_its_linker_and_quantity_in_a_type`). With a linker being a pragma type, an arm is a type over its linker pragma and its quantity, and the name is derived from the type.

The crate also gives the enumeration of what a target admits, so the bench's sweep lists are checked against it by a test and never written from memory.

## The questions the round raises

For op, in the changelist:

1. Whether the tier and passes axes go. Read here, both restate what the pragma set already says and neither is computed from anything, so the proposal is that they are deleted and the target axis becomes the target's facts. Deleting them changes a public type of the crate.
2. Whether a pragma that a target does not admit is refused, as proposed above, or dropped with a warning as the wrapper's dropped flags are today.

For the round's own sketches, each committed before the doc changelist locks:

1. A set of pragma types with `All`, `Any` and exclusion as bounds, with the compiler's output for a set that breaks each.
2. A linker's flags reaching the final crate alone through the wrapper, with the fingerprints of the other crates shown unmoved. File 59 names this as a trap and nothing has shown it yet.
3. The cargo configuration keys the tool would write for a driver, a backend and the `CC` variables, read by a build that takes them.

## What the sketches found

All three are committed under `mock/research/sketches/`, each with the compiler's or cargo's output in its `FINDINGS.md`.

`202611191545_pragma-requirements-as-bounds` holds a pragma as a unit type and a set as a list of them, `no_std` and with no feature gate. As trait bounds, all-of and one per group refuse a broken set at type check. Any-of as a bound refuses a set that holds two of the asked pragmas (E0283, two proofs and no way to prefer one), so it cannot be a bound. As constants folded from the list and asserted in a const block, all three refuse a broken set with a sentence the pragma supplies, and that refusal comes at a build and not at a check. What the round takes from it: the list is the set, and the requirement check is the const one, since one mechanism then carries all three and its message is ours to write. Whether the two bounds that do work are kept beside it, for the error a check shows, is the changelist's to weigh.

`202611191600_a-link-flag-reaches-the-final-crate-alone` shows the trap of file 59 does not bite where the final crate's script alone prints the flag: the binary is relinked with it and the library under it stays fresh. A link flag printed by a library's script changes nothing in the binary and rebuilds the library and all that stands on it. So a linker pragma belongs to the set of a crate that links, and the tool or the lint refuses one in a library's set. A plain link argument also goes by `cargo::rustc-link-arg-bins` with no wrapper. Open after it: a build script cannot tell from its environment that its crate links, so the prototype or the crate's metadata has to say so.

`202611191615_cargo-keys-for-driver-backend-and-cc` shows each of the three as one table of cargo configuration that a build takes: `[target.<triple>] linker`, `[unstable] codegen-backend` with `[profile.<name>] codegen-backend`, and `[env]`. None of them stops at the workspace's own crates as the wrapper does: they reach build scripts and dependencies, and `[env]` yields to the caller's environment. So a driver, a backend and a C compiler are workspace-wide choices the maker tool writes into `.cargo/config.toml`, and are no part of one crate's set. This splits the groups in two: a linker goes per linking crate through the script, and a driver, a backend and a C compiler go per workspace through the tool. LTO and incremental fall with the workspace choices (sketch `sketches/202611191735_lto-and-incremental-are-profile-keys/`): cargo turns the one profile key into different arguments for a library and a binary, and `-Clto=thin` appended to a library alone is refused, "options `-C embed-bitcode=no` and `-C lto` are incompatible". The edge form's `prefer-dynamic` falls with the crate that is the dylib (sketch `sketches/202611191830_where-prefer-dynamic-has-to-reach/`): a whole-workspace release build without it fails, "cannot satisfy dependencies so `std` only shows up once", and with it on the dylib crate alone the build links and runs. One domain is shown; two side by side are not. A nested workspace under `mock/` reads `mock/.cargo/config.toml`, so whatever the tool writes there applies to the bench and sketch workspaces too.

## Who else builds on this

The `Pragma` enum, `PragmaSet` and `BuildConfig` are public and change shape. loru builds on hilavitkutin, so the change is posted to the stack coordinator before it lands, as the brief orders. kaski's build scripts name no pragma (a reading of the topic of round `202611151157`, which counts them as copies of two scripts calling `configure` and `configure_with`), so the consumer-setup round's prototypes are the only callers to move.
