# The mockspace.toml fragment this sketch's rows validate against

Frozen roots (all targets immutable: locked rounds and frozen memos, which is
what makes line citations honest per the reference-syntax contract):

```toml
# ONE declared root: the seed, the frozen consolidated design. Everything
# else provenance needs is either a registry row ref or a git commit ref
# (git::commit::<hash>, citing the trunk commit that landed the research),
# so the roots table never grows again.
#
# The lifecycle: the seed is consolidated once (per domain, riding the drain
# rounds), then frozen. After the freeze, ALL canon changes happen in the
# registry; where a row conflicts with the seed, the row is the later canon.
# Research documents are paper trail: anything meaningful from them lands as
# row additions or edits, cited by their trunk commit; the prose is read only
# for full reasoning depth. Anchor refs during assembly; frozen = true and
# line refs legal once the last domain lands.

[ref.roots.seed]
path = "mock/research/seed"
# frozen = true  # set when the consolidation completes

Namespaces (fields shown as name, required?, one-line intent):

```toml
[[registry.namespace]]
key = "ruling"
# question (req): what was at stake, as the question. ruled (req): what was
# settled; for a refusal, what will not be done. because: the reasoning,
# load-bearing for re-evaluation. alternatives[]: what else was on the table.
# kind (req): ruling | refusal. valve / instead: refusal-only. supersedes[]:
# rows this replaces; superseded rows stay. provenance[] (req): refs in
# precedence order, first is the one to follow.

[[registry.namespace]]
key = "invariant"
# holds (req): the condition. upheld_by (req): who maintains it.
# breaks (req): what goes wrong on violation. enforced (req): where the
# enforcement lives (test, assert, structure, or honestly "discipline").
# provenance[] (req).

[[registry.namespace]]
key = "mechanism"
# what (req): the designed thing. domain (req). status (req): wired |
# substrate_only | absent | deviated. deviated adds: shipped, disposition,
# trigger. provenance[] (req): ONE field for every edge; the ref's root
# carries its kind (ruling:: and invariant:: rows govern, seed:: is the
# consolidated design, git::commit:: is research trail), ordered
# governing-first. A mechanism with no ruling:: ref in provenance is the
# pending-extraction signal, derived rather than declared.
# lives_at: source location when built. note.

[[registry.namespace]]
key = "bench"
# hypothesis (req), arms[] (req), result (req, with the numbers),
# decided[]: ruling rows this evidence settled. runs, provenance[] (req).

[[registry.namespace]]
key = "constant"
# value (req), why (req), tunable: consumer-adjustable default per the
# caps-are-defaults principle. provenance[] (req).

[[registry.namespace]]
key = "epoch"
# goal (req): the band's coherent aim. after[]: epoch refs this one orders
# behind (empty = ready or standing). provenance[].

[[registry.namespace]]
key = "task"
# what (req). epoch (req): owning epoch ref. status (req): pending |
# in_progress | done | superseded. delivers[]: mechanism refs this task's
# landing flips to wired; the completeness check (any absent/deviated
# mechanism no task delivers is a roadmap hole) hangs on this edge.
# blocked_by[]: task refs. tracker: external task-list cross-ref while that
# list exists. provenance[].

[[registry.namespace]]
key = "sketch"
# hypothesis (req), outcome (req): works | fails | inconclusive | superseded.
# detail, unblocks, provenance[] (req).
```

Filing: one TOML per SUBJECT with kinds mixed (registry/storage.toml carries
[[ruling]], [[invariant]], [[mechanism]], [[bench]], [[constant]], [[sketch]]
side by side); the namespace is the array-of-tables key, never the path.

Deliberately absent from the pilot: `domain` rows (the pilot marks domain as a
field; full adoption promotes it to a namespace generating the two domain
maps), `vocab` (nothing in the storage drain demanded it), and every
render-domain kind from ikiuni_renderer.
