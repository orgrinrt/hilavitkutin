# Deriving the Canon Registry Schema From the Documentation It Must Generate

**Date:** 2026-07-19
**Status:** derivation for op review; nothing here is adopted until ruled
**Method:** op's criterion (2026-07-19): the registry's ambition is procedural documentation, query
and template with loops and branches, cohesive single-source-of-truth docs with no duplicated
prose. So the schema is derived by normalising the documentation surface the project must sustain,
database-style: walk every doc section, classify it authored-narrative versus enumerable-fact,
collect the row-sets the enumerable sections would query, and merge those requirements into a
schema. A kind earns its place by the sections that query it, not by resemblance to the historical
pile (the earlier artifact-shaped taxonomy, rejected) and not by mimicking ikiuni_renderer's
render-domain kinds.

## 1. The documentation surface walked

Root `mock/DESIGN.md.tmpl` (architecture): crate layout and dependency graph (already generated
from crate metadata; no new rows), the runtime subsystem sections (plan chain, dispatch, adapt,
thread pool, morsel loop, resource resolution, plan caching), vocabulary, the 22-domain map, a
status section, consumers.

Engine `crates/hilavitkutin/DESIGN.md.tmpl` (the 1300-line shipping contract): its own domain map
(today a hand-maintained duplicate of the root's, the exact conflicting-copy failure op named),
per-domain contract sections that restate settled decisions in prose (the replace_resource
section that carried this week's mis-grounded language is one), mechanism descriptions with
embedded constants, stub and follow-up notes that restate status, the soundness-relevant
obligations scattered through threading and dispatch sections.

Beyond the two DESIGNs: the r8 roadmap's five-state ledger (wired, substrate-only, absent,
deviated, defect) per mechanism; the GATE-2 deviation ledger's ten prose entries; the bench
findings appendices; the sketch corpus; the budgets and thresholds named across sections; agent
rules fragments (already template-generated).

## 2. What each enumerable section needs, and the normalisation

Walking the sections yields recurring queries of the shape "for each domain, for each mechanism in
it, state its contract, status, obligations, and numbers." That loop names the spine the earlier
flat taxonomy lacked:

**Spine entities (what the docs loop over):**

- `domain`: the 22 domains (id, name, dependency level, one-line summary). Generates BOTH domain
  maps from one source, deleting an existing live duplication.
- `mechanism`: a designed thing the engine contains (name, owning domain, canonical-position
  provenance into the frozen sources, status wired / substrate-only / absent / deviated, and,
  when deviated: the shipped position, disposition tag, revisit trigger, evidence refs). This
  single kind absorbs BOTH prose ledgers: r8's state ledger and the GATE-2 deviation ledger are
  the same table wearing two documents. Deviation is a state of a mechanism, not a kind of its
  own; that demotion is what normalising forced, and it is the structural correction to the
  rejected taxonomy. The 12-step plan chain, the parking tiers, the barrier, the swap surface:
  each is a mechanism row. The status sections of both DESIGNs and the whole ledger view become
  queries.

**Leaf kinds (what attaches to the spine by ref):**

- `ruling`: a settled question (question, ruled, because, alternatives, supersedes, authority
  tier, provenance; refusal variant carries valve and instead). Contract sections interpolate the
  ruled text instead of restating it; the canon view per domain is a query. The absence of a row
  is the machine-visible "canon has no answer."
- `invariant`: an obligation plus its enforcement surface (who upholds it, what breaks if
  violated, where the test or assert lives). Generates a reviewer-facing soundness-contract
  section and interpolates into the owning mechanism's section. The parked-between-frames and
  provenance-separation obligations stop living as scattered sentences.
- `constant`: value, why, consumer-tunable flag, deciding-bench ref. Generates the budgets table;
  sections interpolate values instead of hardcoding numbers that drift.
- `bench`: hypothesis, arms, result, what it decided (ruling ref), harness path. Generates the
  bench appendix; rulings cite rows instead of prose findings.
- `sketch`: hypothesis, outcome, what it unblocks. Indexes the corpus the sketch discipline
  already governs; the chart-the-path routine's "what is proven" becomes a query.
- `vocab`: term, meaning, dead-term mapping. Generates the vocabulary sections; the dead-term
  lint reads the same source.

**Not rows, deliberately:** narrative prose (why-the-architecture-is-shaped-so) stays authored in
the templates; the gap catalogue stays in tests (the executable registry); tasks stay in the
tracker; crate structure stays generated from crate metadata as today. A fact appearing in two or
more documents must be a row; a passage appearing in exactly one document and carrying judgment
rather than facts stays prose. That is the whole authored-versus-generated law.

## 3. The seed

The existing canon pile (consolidation spec, unified-engine amendment, A1, storage addendum, A2,
the deviation ledger, the frozen memos) becomes frozen reference roots, cited by `provenance`
fields, per the seed pattern. The A2-1 precedence algebra is applied once per decision at
extraction time; the row records the resolved outcome; nobody applies precedence twice.

## 4. What this buys, against this week's failures

The domain-map duplication dies. The mis-citation class dies (a CL cites a ruling row or it cites
nothing; expert memos have no rows). The deviation ledger's "pending op review" stops rotting in
prose and becomes six queryable mechanism rows with dispositions and triggers. The r8 roadmap's
verification method (construction sites, not doc comments) gets a durable home: mechanism status
is a maintained field, re-checked as rounds move it, not re-derived each roadmap. And the future
query DSL gets a schema already shaped for loops and branches: domains contain mechanisms;
mechanisms carry status; leaves attach by ref.
