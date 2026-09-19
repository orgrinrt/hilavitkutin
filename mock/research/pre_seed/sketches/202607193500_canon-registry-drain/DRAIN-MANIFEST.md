# Drain Manifest: Every Source Passage, Where It Went

The losslessness proof. Each passage of the drained sources maps to a row (and
field) or carries an explicit reason it is deliberately not a row. A passage
missing from this table is a drain defect.

## Storage addendum (202606210600_resource-storage-model-canonical-addendum.md)

| Passage | Destination |
|---|---|
| Header: dates, revision, oracle refs | provenance metadata on every row citing it |
| Header: original asserted decomposed layout, bench refuted | `ruling::per_member_shape_bound_columns_original_addendum` (kept as superseded row) |
| Item 1: one-record blob, V2/V3 refutation with numbers | `ruling::resource_value_layout_one_record_blob` + `bench::storage_six_variant` |
| Item 1: Decompose scoped to size fold + ptr+len | same ruling, `ruled` field, final clause |
| Item 2: scalar snapshot, wall-clock-neutral M1, codegen-real | `ruling::scalar_members_snapshot_before_morsel_loop` |
| Item 2: DrainStores lacks the snapshot, that absence is the drift | `mechanism::scalar_snapshot` (status absent, note) |
| Item 3: handle store, separate provenance, noalias substrate | `ruling::resource_is_handle_not_inline` + `invariant::handle_store_never_aliases_value_columns` |
| Item 4: live-stream, ptr+len, consecutive elements, 2.5x / 4 MiB | `ruling::collections_live_streamed_never_copied` + bench row |
| Item 5: noalias architectural guarantee; 1.28-1.40x is a distillation not reproduced on M1 | invariant row (`breaks`) + snapshot ruling (`because`, the caveat verbatim in substance) |
| Item 6: erased addressing, op hybrid 2026-07-02, parity numbers, global-capable | `ruling::erased_static_shape_addressing_global` |
| Item 6: loimu as the interop motivation | carried by the provenance ref to the hybrid topic, which states it; not restated. Judged acceptable: motivation, not a decision |
| "Why the handle model is what R5 says" (three spec readings) | the spec refs in the corresponding rulings' provenance |
| Retraction section | the superseded row + `ruling::seq_map_arena_attaches_to_handle_store` |
| "Shipped impl is drift only in lacking the snapshot" | `mechanism::resource_drain` (wired) + `mechanism::scalar_snapshot` (absent) |
| Dependent work: A3b downstream, CollectionBytes #163/#164, drift-fix arc | NOT rows: task-tracker material. EXCEPT one orphan fact, see FINDINGS friction 1 |

## Consolidation spec R5 (:535-566) and storage model (:1682-1710)

| Passage | Destination |
|---|---|
| Three field types, ColumnValue limit, elements exempt | `ruling::resource_model_three_field_types` + `constant::column_value_size_limit` |
| No dynamic collections, const generic sizes | same ruling (`ruled`, `because`) |
| Morsel budget: write collections count, read-only ride L2, formula | `ruling::collection_write_budget_rule` |
| Raw pointers not slices, type-native stride, T6 aliasing UB | `ruling::columnstorage_raw_pointers_not_slices` |
| Consumer provides backing memory, library never allocates | `ruling::library_never_allocates` (refusal, valve = MemoryProvider) |
| 64-byte column alignment | `constant::column_alignment` |
| release(column) advisory + consumer-count model | `mechanism::column_release_consumer_count` (absent) |
| Separate arena for Seq/Map | `ruling::seq_map_arena_attaches_to_handle_store` |
| Pointer indirection, &self inline writes are UB | `ruling::resource_is_handle_not_inline` + `invariant::no_inline_writes_through_shared_ref` |
| Provenance separation, 1.28-1.40x measured figure | invariant row + snapshot ruling caveat |
| "The fiber dispatcher handles this automatically, knows the resource set from WU declarations" | `mechanism::scalar_snapshot` (`what`) |
| "NOT the same issue as the cu/cw noalias finding (domain 09)" | cross-domain boundary note: belongs to the domain-09 drain, deferred, not lost |

## This week's rulings (A2)

| Passage | Destination |
|---|---|
| A2-5 mis-citation correction + swap-absence + commissioned round | `ruling::member_by_member_swap_language_is_not_canon` + `mechanism::replace_value_install` |
| A2-3 PlanAffecting unsealing | `mechanism::replace_resource_plan_dirty` (note) |
| Exclusivity sketch outcome | `sketch::planaffecting_replaceable_exclusivity` |

Not in this drain's scope (other domains or other artifacts): A2-1 precedence,
A2-4 deviation dispositions (those become mechanism rows of their own domains),
the r8 status ledger outside storage.
