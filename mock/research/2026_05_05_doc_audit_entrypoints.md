# Hilavitkutin entrypoint documentation audit, 2026-05-05

**Date:** 2026-05-05.
**Scope:** `README.md` (repo root, Tier 1), `mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl` and `mock/WORKFLOW.md.tmpl` (Tier 3, render to `docs/DESIGN.md` / `docs/PRINCIPLES.md`), and the rendered `docs/*` Tier 1 output. Companion to the per-crate audit at `mock/research/2026_05_05_doc_audit.md` (sub-agent).
**Method:** Manual read pass against documentation-writing + writing-style + vocabulary + ai-agent-framing workspace rules.

## Summary

Repo-root `README.md` is in good shape: stand-alone identity, no Tier 1 leakage, no banned vocabulary. The mock-root tmpls are clean of em-dashes after PR #62 (the cleanup sweep just landed), and the title's prior em-dash was replaced with a comma. The remaining surface is **Tier 3 leakage in the mock-root tmpls**: multiple `mock/design_rounds/` references, `cargo mock` mentions in `WORKFLOW.md.tmpl`, `.tmpl` extensions named explicitly, and round-id references. The rendered `docs/DESIGN.md` is **stale**: it still has em-dashes (lines 2, 10, 25) that the .tmpl no longer carries, indicating `cargo mock` regeneration has not run since PR #62 merged. A regen will clear those automatically. Title flow note: `# hilavitkutin, Architecture Design` (comma, post-em-dash-removal) reads stilted; consider `# Hilavitkutin: Architecture Design` (colon) for the headline.

## Per-file findings

### README.md (repo root)

Clean. Stand-alone identity, no leakage.

### mock/DESIGN.md.tmpl

- **Title flow, line 1.** `# hilavitkutin, Architecture Design`. Post-em-dash-replacement reads odd. Suggested: `# Hilavitkutin: Architecture Design` (colon properly introduces the qualifying phrase per writing-style).
- **Tier 3 leakage, line 8.** `The authoritative source is design_rounds/202603181200_topic.hilavitkutin-design-consolidation.md (22 domains, 9 cross-domain resolutions, 2480 lines). This doc is a map into that spec.` Replace the cross-reference with a public-friendly framing: state what the doc covers (22 domains, 9 cross-domain resolutions) without naming `design_rounds/` or the round id.
- **Round-number Tier 3 leakage, line 21.** `Renamed from prior hilavitkutin-extensions in round 202604240615`. The rename history belongs in a Tier 2 history doc, not the public crate-topology table. Suggested: drop the parenthetical or move to a sibling Tier 2 file.
- **`.tmpl` reference, line 62.** `Per-module design lives in crates/hilavitkutin/DESIGN.md.tmpl.` Names a `.tmpl` extension. Tier 3 leakage. Replace with `Per-module design lives in the per-crate documentation under each crate's directory.`
- **`.tmpl` reference, line 64.** `## Principles (see PRINCIPLES.md.tmpl)` heading. Replace with `## Principles (see PRINCIPLES.md)` since the rendered file is `PRINCIPLES.md`, not the template.
- **Changelist Tier 3 leakage, line 74.** `Consolidation topic is complete as of 2026-03-18; latest changelist (03-24) added arvo-bits insertion + strategy markers.` `changelist` is workspace-internal. Replace with `Consolidation is complete as of 2026-03-18; the most recent additions (03-24) were arvo-bits insertion and strategy markers.`
- **`design_rounds/` reference, line 81.** `These land as new design_rounds/ entries when they surface.` Replace with `These get filed as new design notes when they surface.` or remove entirely.

### mock/PRINCIPLES.md.tmpl

(Sub-agent audit will cover this in detail. Sample-checked: heavy round-id and design_rounds/ references throughout. Same cleanup pattern as DESIGN.md.tmpl.)

### mock/WORKFLOW.md.tmpl

- **Mockspace leakage, line 13-14.** `**Current phase: design.** The mockspace validates design via cargo check + doc generation.` Names mockspace. Tier 3 leakage if the file renders to a Tier 1 surface. If the file is contributor-only (verify), no fix needed. The first sentence after the heading sets the tone for the whole document.
- **`.tmpl` references, line 16-17.** `The goal is every DESIGN.md.tmpl agrees with every other and the types compile.` Names `.tmpl`.
- **Mockspace leakage, line 22-26 region.** `cargo mock`, `mock workspace` named throughout the development-order list. If this renders Tier 1, the entire ordered list reads as contributor-internal. Move the list to a Tier 2 contributor doc and replace the rendered Tier 1 prose with consumer-facing workflow (how to use the crate, how to bump versions, etc.).
- General observation: the WORKFLOW.md.tmpl reads as a contributor's workflow doc. Decide whether it should render Tier 1 at all. If yes, complete rewrite. If no, no fix needed; verify the rendering pipeline does not produce a Tier 1 file from this template.

### docs/DESIGN.md (rendered Tier 1, stale)

- **Em-dash, line 2.** `AUTO-GENERATED — DO NOT EDIT DIRECTLY` in the auto-gen header. Workspace-wide finding (mockspace renderer-generated; not local-fixable).
- **Em-dash, line 10.** `# hilavitkutin — Architecture Design`. The .tmpl source already replaced this with `,` post-PR #62; the rendered file is stale until `cargo mock` runs.
- **Em-dash, line 25.** `Standalone build-dep — no runtime deps.` in a table cell. Already swept in source per PR #62; stale rendered output.
- **Auto-gen header leakage, line 4 / 7.** `Generated by: mockspace (mock)` / `To regenerate: cargo mock`. Workspace-wide (mockspace renderer).
- **Stale changelist reference, line 93.** `latest changelist (03-24) added arvo-bits insertion`. Inherited from .tmpl; will fix after .tmpl line 74 fix lands.

A `cargo mock` regeneration after the .tmpl fixes above will resolve the body-content drift. The auto-gen-header em-dash and `mockspace`/`cargo mock` mentions on lines 2, 4, 7 are workspace-level findings that need a mockspace upstream fix.

### docs/PRINCIPLES.md, docs/HILAVITKUTIN_*_OVERVIEW.md (rendered Tier 1)

(Sub-agent audit will cover the per-crate OVERVIEW files. Sample-checked: same auto-gen header pattern. Per-crate findings will surface specifics.)

## Cross-cutting patterns

1. **Tier 3 leakage in mock-root tmpls.** Multiple sites in `mock/DESIGN.md.tmpl` name `design_rounds/` paths, `changelist`, `.tmpl` extensions, and round ids. Same pattern as arvo's mock-root tmpls.
2. **Title em-dash replaced with comma reads stilted.** The PR #62 sweep replaced `# hilavitkutin — Architecture Design` with `# hilavitkutin, Architecture Design`; the comma is grammatically defensible but reads slightly off. Consider colon.
3. **Stale rendered docs/.** Same as arvo: `docs/DESIGN.md` still has em-dashes that source .tmpl removed. `cargo mock` regen needed after the .tmpl fixes land.
4. **WORKFLOW.md.tmpl is Tier 2 in shape but renders Tier 1.** Decide whether to rewrite the Tier 1 portion or strip the file from the Tier 1 render entirely.
5. **Auto-gen header is a workspace-wide finding.** Same as arvo: `Generated by: mockspace (mock) ... To regenerate: cargo mock` plus the em-dash on line 2 are produced by the mockspace renderer. Workspace-level fix.

## Suggested topic-file scope

Hilavitkutin entrypoint cleanup wants:

1. **Strip mock-root tmpl Tier 3 leakage.** ~6 sites in `mock/DESIGN.md.tmpl` (round ids, changelist, design_rounds, .tmpl). Replace each with public-friendly framing or drop.
2. **Title flow fix.** `# hilavitkutin, Architecture Design` → `# Hilavitkutin: Architecture Design` (or another wording the maintainer prefers).
3. **`mock/PRINCIPLES.md.tmpl` cleanup** (sub-agent will detail).
4. **`mock/WORKFLOW.md.tmpl` audience decision.** Pure contributor-only doc, or Tier-1-rendered? If Tier-1-rendered, full rewrite of the body to consumer focus.
5. **Stale docs/ regeneration via `cargo mock` after .tmpl fixes land.** Mechanical, not a discrete topic item.
6. **Workspace-wide auto-gen-header issue.** Surface to mockspace upstream as a separate finding; not addressed in hilavitkutin's doc round.

The hilavitkutin doc round wants one topic file with one doc CL covering items 1-4. No src CL needed unless the per-crate audit surfaces rustdoc drift.
