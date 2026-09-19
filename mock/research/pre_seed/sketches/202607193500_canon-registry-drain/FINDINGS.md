# Sketch: Draining the Storage Canon Into Registry Rows

**Date:** 2026-07-19
**Hypothesis:** the doc-derived schema (spine domain/mechanism, leaves ruling/
invariant/constant/bench/sketch) can express the storage/resource canon as a
stable registry, drained from the frozen sources without losing any detail,
reasoned about with the current simple queries and refs (no templating, no
query DSL).

**Outcome: WORKS**, with four schema corrections discovered during the drain,
three of them op's live catches. The corrections are the experiment's yield.

The drain: 29 rows in one subject dossier (`registry/storage.toml`: 12
rulings including one kept-superseded row and one refusal-with-valve, 4
invariants, 7 mechanisms, 1 bench, 3 constants, 2 sketches), from the storage
addendum, spec R5 plus the storage model section, the hybrid-decision topic,
and this week's A2. `DRAIN-MANIFEST.md` maps every source passage to its
destination row and field, or records the deliberate reason it is not a row;
that manifest is the losslessness proof and the pattern any future domain
drain owes.

## Corrections found

1. **Per-artifact roots, not generic roots** (op). `res::<timestamped
   filename>::lines` is untenable to read or write. One frozen root per seed
   artifact with a short semantic name (`storage_addendum`, `a2`, `spec`,
   `hybrid_topic`) makes the roots table the seed catalogue and every ref
   humane. Heading anchors preferred over line numbers where headings exist.
2. **`canonical` on mechanisms was a conflation** (op). It mixed two edge
   types: row-to-row (what governs this mechanism) and row-to-frozen-file
   (audit). Split into `governed_by` (ruling/invariant rows only, queryable
   both directions, supersession-aware) and the uniform `provenance`
   (frozen files only). An empty `governed_by` is the visible
   pending-extraction signal; `extraction` says why.
3. **File by subject, kinds mixed** (op; the registry doc said so all along).
   One `storage.toml` dossier reads as the domain's canon; per-kind subdirs
   fragmented it for no query benefit, since the namespace is the key, not
   the path.
4. **The superseded-row convention works.** The refuted original-addendum
   reading lives as its own row, `supersedes`-linked from the winner, so the
   record of changing course survives without polluting the active set: a
   query for active rulings simply skips rows whose id is in any supersedes
   list (or, once adopted, a status field per ikiuni's shape).

## Frictions logged, not resolved here

- One orphan fact: the addendum's dependent-work note that the per-store size
  fold runs over blob strides plus the CollectionBytes term (not decomposed
  columns). Too implementation-adjacent for a ruling today; it rides #344's
  implementing round and is flagged here so it is not lost.
- The domain-09 cross-reference in the spec's storage section ("not the same
  issue as cu/cw noalias") belongs to that domain's drain; deferred, tracked
  by the manifest.
- Live-source refs (a FIXME's location) cannot be provenance under the
  frozen-roots rule; status evidence must cite frozen artifacts (r8, A2) and
  name live locations in prose fields only. This held up fine in practice.

**Unblocks:** op's adoption ruling on the medium; if adopted, the real
`mock/registry/` + mockspace.toml land via a proper round with this sketch as
the template, and further domains drain as rounds touch them.

## Extension: the roadmap as data (op, same session)

Epochs and tasks joined the schema (`registry/roadmap.toml`: r8's bands as 7
epoch rows, the storage-relevant work as 4 task rows). The load-bearing edge
is `task.delivers -> mechanism`: a task's landing is what flips a mechanism's
status, so the roadmap document becomes a query over epochs, tasks, and
mechanism status, and the r6-to-r8 pattern (three hand-rewritten roadmaps in
three days re-deriving one state ledger) dies. Better: roadmap completeness
becomes checkable. The twelve-line query in this sketch's commit verifies
that every absent or deviated mechanism has a delivering task (pilot result:
no holes). ikiuni_renderer intends the same move after the query DSL
resolves; the data model needs nothing from the DSL and lands first.

One more consequence (op, same session): task rows make task references
durable. The session-local `#NNN` ids appear today inside durable repo
artifacts (FIXMEs, catalogued-test ignore reasons, memos: "tracked #654"),
and none of them resolve for any reader who is not this agent's session. A
`task::<slug>` ref resolves for anyone with the repo, forever. On adoption,
repo artifacts cite task slugs; `tracker` carries the session-local id as the
ephemeral cross-ref, and the existing #NNN citations get rewritten as their
files are next touched.

Filing refinement (op): epochs file alone; tasks file in the domain dossier
they belong to, or their own file when domain-free. Mixed epoch/task files
were hard to reason about, and filing is free since the namespace is the key.

## Final ref regime (op, fourth refinement round)

Three collapses, each removing schema rather than adding it. One declared
root: the seed (`mock/research/seed/`), the once-consolidated design, per
domain riding the drain rounds, frozen when complete; after the freeze all
canon changes happen in the registry only, and a row conflicting with the
seed is the later canon. Research becomes pure paper trail cited as
`git::commit::<hash>` trunk-commit refs, which are immutable by construction,
need no root declarations, and resolve to the memo plus its round context.
And `governed_by` merged back into `provenance`: the ref's root already
carries the edge's kind, so one ordered field (governing rows first) gives
the same queries by root-filtering with less schema; the pending-extraction
signal becomes "no ruling:: ref in provenance", derived rather than declared.
The dossier now demonstrates the final shape end to end: rows cite
`seed::storage::#anchors` (the retrofit chapter in seed/storage.md, written
from the drain's reading) plus git commit refs; no per-artifact roots remain.
