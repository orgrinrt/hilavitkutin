**Date:** 2026-05-08
**Phase:** TOPIC
**Scope:** hilavitkutin (two doc-CL slips)

# PR-review follow-up: persistence field-rename slip + extensions-macros truncation

PR-review follow-up to round 202605051400. Two bugs surfaced by the
reviewer:

1. `mock/crates/hilavitkutin-persistence/DESIGN.md.tmpl` field-rename
   slip: the audit's `RowCount → record_count` decision was applied to
   both the field name and the type identifier, producing
   `row_count: record_count` (ill-formed Rust). The intent was to
   rename the type from `RowCount` (the dead vocab term `row`) to a
   record-aligned form. Correct shape: field `record_count: USize`
   (typed via the substrate's USize newtype) or
   `record_count: RecordCount` (a domain alias). The struct currently
   pairs CamelCase types (`ContentHash`, `SchemaVersion`,
   `ColumnCount`) with snake_case fields (`name_hash`, `version`),
   so the type slot wants a CamelCase form. Use `USize` directly to
   stay vocabulary-neutral.

2. `mock/crates/hilavitkutin-extensions-macros/DESIGN.md.tmpl` line 79
   reads "The emitted shape locked in the src CL of. Follow-up rounds
   may extend...". The trailing "of." is a sentence-truncation slip
   from the round-id sweep. Rewrite as "The emitted shape is locked
   at v1." or drop the partial clause entirely.

## Decisions

### Decision 1: persistence struct field

Field name: `record_count`. Type: `USize`. The neighbouring `Cardinality`
and `BufferOffset` aliases (line 196 region) document `RecordCount` as
a `Cardinality`-shaped domain alias; if that domain alias is preferred,
use `record_count: RecordCount`. Either form satisfies the rename.

### Decision 2: extensions-macros sentence rewrite

Replace "The emitted shape locked in the src CL of." with "The emitted
shape is locked at v1.".
