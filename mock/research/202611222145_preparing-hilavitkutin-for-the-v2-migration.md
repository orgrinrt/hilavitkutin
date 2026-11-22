# Preparing hilavitkutin for the v2 migration

Read on 22 November 2026, after a fetch of every remote branch, against `origin/dev` at abb6c8f5. Nothing here migrates: the migration waits for mockspace v2 on its `dev`. This note records what was brought level before it, what was found, and the `include` patterns a migration of this repository would declare.

## The trunk

`cargo mock` exits 0 on `dev` abb6c8f5 and leaves the tree as it found it.

## The engine pin

Two tool crates, `cross-target` and `old-arvo-names` under `mock/tools/`, pinned mockspace at `rev = f42844c`. They follow the `dev` branch now, like the rest of the stack, and resolve 6eb94229 (the head the launcher builds the lints against on the day). Both build against it with `cargo check`.

The lockfiles also pin two revisions of arvo (1b2796e and 6def8831), by the `rev` of each dependency. Both exist in arvo's history, so neither is a dead revision. They were left where they are: moving them to arvo's current `dev` changes the numeric surface the engine compiles against, which is work for a design round and no preparation. vehje was not attached to this job and was not read.

## The lint-rules skill template

The ticket `the_lint_rules_skill_templates_of_hilavitkutin_and_vehje_des` asked for `mock/agent/skills/lint-rules/SKILL.md.tmpl` to be checked against the one-pass lint harness. The template already follows it: it writes lints through the context (`ctx.source()`, `ctx.all_sources()`, `ctx.tree()`, `ctx.view()`) and names the `message_lint`, `repo_lint` and `cross_lint` entry points and the `testkit` fixture, all of which exist in mockspace `dev` 6eb94229 (`lint-rules/src/context.rs`, `lint-rules/src/lib.rs`, `src/bootstrap/lints.rs`). One line was wrong: `ctx.registry()` is on the repo lint context only. It now says so. The same ticket names vehje's template, which was not read here.

## The branches

65 remote branches, sorted by `branch-classes` on the content of what a merge into `dev` would change.

| Class | Count | What was done |
|---|---|---|
| already on the stale list with their evidence | 42 | nothing here; they wait for `forge-sweep`, which deletes through the console |
| landed | 5 | `research/load-time-units-seat02`, `work/canon-registry`, `work/hila-records-and-red`, `work/sketch-accumulator-grouped-by-data`, `work/sketch-derived-context-block-form`: queued for deletion through `forge-route delete-branch` |
| record | 3 | landed, see below |
| source | 4 | one landed, see below |
| conflicting | 8 | see below |

Landed by this work, each a merge commit of the whole branch:

| Branch | What it adds |
|---|---|
| `work/reports-home` | 18 files of reports |
| `research/load-time-units-seat01` and `research/load-time-units-seat01-reply` | the sketches and notes of the load-time units panel, seat 01 (the reply branch contains the first) |
| `bench/macos-aarch64-apple-m1/20261116T0720Z`, `...T1220Z`, `...T1929Z` | the m1 run directories |
| `bench/linux-x86_64-12th-gen-intel-core-i5-1240p/20261117T0943Z` | the linux run directories |

Two decisions inside those merges. The four files of the `c6-layout-across-seam` sketch exist on `dev` in a later edit that holds more than the branch's copy (a round-trip comment and its output line), so `dev`'s text is kept and the branch's two `allow(dead_code)` attributes were added. And `mock/benches/results/INDEX.md` is generated from the run directories and each bench branch regenerated it from its own set, so the three later merges conflicted in 191 places; it stays at the text `dev` had, and the next bench run regenerates it from all the directories. Until then the index does not list the runs these merges brought.

Retired: `docs/root-readme`. It rewrites the generated `README.md` by hand; `dev` has regenerated that file several times since (the last on 20 November).

Left, with what each still holds. The last column counts commits of the branch whose subject has no equal on `dev` (a squash or a rebase leaves other subjects, so a low number is evidence that most of the branch landed in another form, not proof).

| Branch | Ahead | Behind | Last commit | Subject not on dev |
|---|---|---|---|---|
| `work/hilavitkutin-stranded-and-placement` | 25 | 18 | 2026-11-19 | 24 |
| `work/hv-suite-examples` | 15 | 33 | 2026-11-14 | 14 |
| `work/hv-suite-reads` | 4 | 33 | 2026-11-14 | 4 |
| `work/hv-suite-words` | 16 | 33 | 2026-11-14 | 15 |
| `work/loop-core` | 30 | 52 | 2026-11-13 | 30 |
| `work/record-the-97-crate-point-of-build-static` | 21 | 9 | 2026-11-19 | 19 |
| `work/commit-subject-config` | 4 | 20 | 2026-11-16 | 3 |

`work/record-the-97-crate-point-of-build-static` is mostly on `dev` already: its 65 new run directories are present and byte-identical. What it still holds is the correction of `mock/benches/support/build-call/tests/measured.rs` and one note after three reviews, which conflict with the version `dev` took (`docs/BENCHES.md` is generated and regenerates). The other branches carry source and design rounds that edit files `dev` has changed since, so each needs its own owner to merge or retire it; the branches that carry open work migrate with the repository afterwards.

`work/commit-subject-config` cannot land yet, for the reason given in arvo's note of the same day: it needs template variables and a lint key that mockspace `dev` does not have.

## The `include` patterns

The listing of `mock/` on `dev` (11506 tracked files):

| Directory | Files | Goes to |
|---|---|---|
| `registry/` | 107 | the namespace `registry` |
| `research/` | 1949 | the namespace `mock` |
| `design_rounds/` | 1780 | the namespace `mock` |
| `benches/results/` | 6491 | the namespace `mock` |
| `crates/**/*.md.tmpl` | 41 | the namespace `mock` |
| `agent/**/*.md.tmpl` | 12 | the namespace `mock` |
| `*.md.tmpl` and `catalogue.md` at the root of `mock/` | 4 | the namespace `mock` |
| `crates/` (the rest), `benches/` (arms and support crates), `lints/`, `tools/`, `agent/` (config and hooks), `.cargo/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | 1122 | stays on the trunk |

The declaration, in the shape muisti runs today (there is no `mock/test-history/` here, so that pattern is left out):

```toml
[[data_ref]]
namespace = "registry"
path = "registry"

[[data_ref]]
namespace = "mock"
path = "."
include = [
    "research/**",
    "design_rounds/**",
    "benches/results/**",
    "crates/**/*.md.tmpl",
    "agent/**/*.md.tmpl",
    "*.md.tmpl",
    "catalogue.md",
]
```

A dry run has to settle whether the second namespace, whose `path` is the whole of `mock/`, leaves out `registry/` by itself or needs the pattern `!registry/**`; this note could not run one. The bench arms and support crates stay on the trunk because `cargo mock bench` compiles them from the instance on disk; `benches/results/` is the bulk (6491 of 11506 files) and moves alone.
