# Template scrub: remove polka-dots / saalis / loimu / clause-dev leakage

**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** mock/crates/*/{README,DESIGN,BACKLOG}.md.tmpl + mock/{DESIGN,PRINCIPLES}.md.tmpl
**Source topics:** ecosystem polish audit (overnight 2026-05-04)

## Current state

Crate-level templates and the workspace-level DESIGN/PRINCIPLES templates carry references to the author's local sibling repositories: `polka-dots`, `saalis`, `loimu`, `clause-dev`. These propagate into `docs/` (auto-generated) and would propagate to crates.io via `[package] readme = "..."` once the crates publish.

The workspace's `readme-format.md` rule explicitly forbids these references in public crate surfaces:

> Local dev-routine leakage. The author's personal development setup is invisible to the reader. Zero mentions of:
> - Workspace-grouping directory names that exist only on the author's machine (e.g. `clause-dev`, `loimu`, private monorepo roots).
> - Sibling local-only repos that aren't public dependencies or consumers (e.g. `stellar-heritage`, `polka-dots`, `saalis` ...).

The rule is workspace-wide; it covers READMEs but the spirit applies equally to DESIGN and BACKLOG templates that downstream readers will see.

## Affected files (verified by grep)

- `mock/crates/hilavitkutin-ctx/README.md.tmpl:6` (saalis, polka-dots, loimu)
- `mock/crates/hilavitkutin-extensions/README.md.tmpl:9` (loimu-plugin-abi)
- `mock/crates/hilavitkutin/README.md.tmpl:5` (polka-dots, saalis, loimu)
- `mock/crates/hilavitkutin-persistence/DESIGN.md.tmpl:6,8,184` (saalis, polka-dots, loimu, .saalis)
- `mock/crates/hilavitkutin-ctx/DESIGN.md.tmpl:4` (saalis, polka-dots, loimu)
- `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:4` (polka-dots, saalis, loimu)
- `mock/crates/hilavitkutin-api/BACKLOG.md.tmpl:23` (polka-dots research)
- `mock/crates/hilavitkutin-linking/DESIGN.md.tmpl:123` (loimu / saalis / polka-dots synthesis)
- `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl:111,179` (loimu-plugin-abi, loimu)
- `mock/crates/hilavitkutin-str/DESIGN.md.tmpl:6` (saalis, polka-dots, loimu, saalis workspace)
- `mock/crates/hilavitkutin-persistence/BACKLOG.md.tmpl:42` (saalis's ZIP+zstd, polka's .polka)
- `mock/crates/hilavitkutin-str/BACKLOG.md.tmpl:43` (saalis / hilavitkutin supply)
- `mock/DESIGN.md.tmpl:42,95,97` (loimu, polka-dots, saalis listed as consumers)
- `mock/PRINCIPLES.md.tmpl:55` (saalis's)

19 specific line occurrences across 14 files.

## Decisions

1. **Replace pattern.** Each occurrence is rewritten to neutral framing that names the role rather than the local repo. `polka-dots / saalis / loimu` -> `downstream consumers`, `consumer ecosystems`, or `the hilavitkutin consumer surface`. Specific feature mentions (e.g. ".saalis ZIP+zstd") become abstract ("a consumer-chosen container format with framing and compression").

2. **`mock/DESIGN.md.tmpl` Consumers section.** Currently lists `loimu`, `clause`, `polka-dots`, `saalis` as named consumers. Drop the named list; replace with a paragraph describing the consumer pattern abstractly. The named consumer that DOES belong (vehje, formerly clause, the language compiler in the same workspace) stays named because it is a public sibling.

3. **`hilavitkutin-extensions` plugin-abi enumeration.** Currently names `viola-plugin-abi`, `clause-plugin-abi`, `loimu-plugin-abi` as consumer plugin-ABI examples. Drop `loimu-plugin-abi` (private); keep `viola-plugin-abi` (public, in workspace); keep `clause-plugin-abi` reference noting it as the planned vehje-side ABI (rename to `vehje-plugin-abi` for current naming).

4. **Research-input citation.** `hilavitkutin-linking/DESIGN.md.tmpl:123` mentions `EXTENSIONS_RESEARCH.md` as "loimu / saalis / polka-dots synthesis". The synthesis is real; replace with neutral "synthesis of dlopen patterns from prior consumer ecosystems" without naming.

5. **`hilavitkutin-extensions/DESIGN.md.tmpl:179`** mentions "loimu, viola-cli" as concrete examples of consumers that build extension discovery on top. Drop `loimu`; keep `viola-cli`.

6. **No source changes.** This round is DOC-only. Source files remain untouched.

7. **Em-dash sweep.** Templates also carry pervasive em-dashes that violate workspace writing-style rules. Defer to a separate round so this one stays focused on leakage; the em-dash work is mechanical sed-shape and easier to review separately.

8. **No new public surface.** No new types, traits, or methods. Pure prose / framing edits.

## Out of scope

- Em-dash sweep across templates (separate follow-up round).
- Changes to round-artefact files under `mock/design_rounds/*/` (these preserve historical state).
- Changes to research notes under `mock/research/` (preserved verbatim per project convention).
- Source files (`mock/crates/*/src/**`).
- crates.io publish prep (this round just removes the leakage; publish happens later).

## Frozen at lock

This topic file is frozen on the round's first commit. Decisions above bind the doc CL's scope.
