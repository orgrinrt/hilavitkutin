# Flexible Schema Done Right: Synthesis Strategy for Saalis

**Date:** 2026-03-14
**Synthesizes:** `2026-03-14-research.universal-entity-model-gotchas.md` + `2026-03-14-research.data-classification-ux.md`
**Context:** Saalis uses a universal entity model where everything — games, users, downloads — is an entity in a shared `entities` anchor table. Metadata lives in typed tables (`Rating`, `Description`, `Media`, `Composites`, etc.) with compile-time ZST column types (`Column<In<M>, As<V>>`). The web UI uses DataViews as typed query projections over this model.

---

## 1. How Typed Metadata Tables Avoid the Classic EAV N-Join Problem — and Where They Don't

The EAV research establishes that pure EAV's fundamental sin is the pivot: reconstructing a single logical row from N attribute rows requires N self-joins against a single polymorphic table. Saalis's typed metadata tables eliminate the *self*-join entirely. Each metadata table has proper columns, proper types, proper indexes. Querying "all games released after 2020" is a single-table scan against `titles` or `game_metadata` with a B-tree index on the relevant column. No pivot. No text-to-number casting. No composite index on `(entity_id, attribute_name)` because attribute names are columns, not rows.

But the N-join problem does not vanish — it *transforms*. Instead of N self-joins within one table, Saalis performs N cross-table joins when reconstructing a full entity profile. Loading a game's detail page touches `titles`, `ratings`, `media`, `credits`, `classifications`, `descriptions`, `links`, and `stats` — eight LEFT JOINs against the `entities` anchor. For a single entity with indexed foreign keys, this is sub-millisecond on SQLite. The danger zone is the list view: "show 50 games sorted by rating with thumbnails and genre badges" requires joining `ratings` (for sort), `media` (for thumbnails), `classifications` (for badges), and `titles` (for display names) across 50 entities. Four joins times 50 rows is manageable with covering indexes, but it is the exact query shape that tripped up Magento's flat catalog and WordPress's wp_postmeta.

The critical insight from the EAV research is that the N-join problem has *two* distinct manifestations, and our architecture handles them differently:

**Single-entity reconstruction** (detail pages, edit forms): N joins where N is the number of metadata tables, but each join returns at most a handful of rows for one entity_id. SQLite's index lookups are O(log N) per join. With 10 metadata tables and proper `entity_id` indexes, this is ~10 index lookups. Fast at any realistic scale.

**Multi-entity projection** (list views, galleries, search results): The same N joins, but the query planner must plan across all tables simultaneously, applying filters from one table (ratings > 8), sorts from another (order by title), and projections from a third (select media.url). This is where SQLite's single-index-per-table limitation bites — it will pick one index per table in the join and scan the rest. Without the composites table, every list view is a multi-table join that grows more fragile as data grows.

The typed metadata approach preserves everything the database engine is good at (column types, constraints, B-tree indexes, covering indexes) while paying a controlled cost in join complexity that scales with *the number of metadata tables*, not with *the number of entities* — a much more favourable growth axis, since metadata tables are a compile-time-fixed set while entities grow unboundedly.

---

## 2. Fibery as the Closest Analogue — What We Can Adopt Directly

The UX research identifies Fibery as the closest product analogue to Saalis's architecture. Both systems define custom entity types connected by typed relations, with views as lenses over the same underlying data. This is not a superficial similarity — Fibery's core insight is that **relations are the primary organisational mechanism**, not an afterthought. The same principle animates Saalis's entity model, where hierarchy (franchise -> work -> edition -> variant) and cross-cutting concerns (user -> game ownership, download -> target entity) all emerge from metadata relationships between entities in the same anchor table.

Three Fibery patterns are directly adoptable:

**Bidirectional relation visibility.** Fibery shows relations from both ends: if Game A relates to Platform B, both the game page and the platform page surface the connection. In Saalis, this means the `References<T>` trait (§1.2 of the architecture decisions) should generate inverse navigation automatically. When a `Media` metadata row references an entity, the entity's detail view should show its media. When a `Credits` row references a person entity and a game entity, both the person and the game should display the credit. The DataView builder should support `.col::<Credits>().show_inverse()` to render "games this person worked on" without a second query definition.

**Many-to-many as a first-class concern.** Unlike Notion, which requires workaround junction databases for M:N relations, Fibery handles them natively. Saalis's metadata tables already support this naturally: the `classifications` table has one row per (entity, label) pair, making a game with five genres five rows. But the DataView builder needs explicit support for rendering multi-valued columns — displaying five genre badges in a gallery card, not five separate rows. The `.col::<Genre>()` call should aggregate multi-valued metadata into a single cell by default, with a configurable display limit.

**Hierarchies from relations, not hardcoded structure.** Fibery's philosophy that "hierarchies emerge from relation configuration" aligns with Saalis's §4.3 decision that hierarchy is metadata relationships. The practical implication is that the DataView system needs a way to express hierarchical grouping: "show franchises, then works within each franchise, then editions within each work" — all from the same entity table, grouped by relationship metadata. This is a tree-structured DataView, which neither Notion nor Fibery handles particularly well in their table views but which would be a differentiator for a content library where franchise/series organisation is a core use case.

---

## 3. The Blank-Page Problem Applied to Saalis — Default Views from Registered Metadata

The UX research documents a consistent finding across Notion, Obsidian, and Logseq: when users face a blank canvas, they freeze. Notion solved this with template-driven onboarding; Obsidian's community invented a dozen competing organisational methodologies because the tool did not choose one; Logseq users who go beyond simple journals hit a wall of Datalog syntax.

Saalis has a structural advantage that none of these tools possess: **the set of metadata tables is known at compile time.** When a user installs Saalis (or when Saalis first encounters a Game entity), the system knows — through `#[register]` and `inventory` — exactly which metadata tables are registered for `Game`, exactly which columns each table has, and exactly what types those columns carry. This is not runtime discovery; it is compiled-in knowledge.

This means default views can be generated automatically, not just provided as templates:

**Auto-generated Gallery view.** For any entity type `E` where `E: HasMetadata<Media>` and `E: HasMetadata<Titles>`, the system can generate a gallery view with cover art from `Media` (filtered to `MediaType::CoverArt`) and the primary title from `Titles` (filtered to `IsPrimary = true`). No configuration required. The DataView builder code for this is:

```rust
DataView::for_entity::<E>("default_gallery")
    .col::<Title>()
    .col::<CoverArt>().render_as(Image)
    .col::<Rating>()
    .card_mode()
    .page_size(48)
```

But the key insight is that this DataView definition itself can be *derived from trait bounds*. If `Game: HasMetadata<Titles> + HasMetadata<Media> + HasMetadata<Rating>`, the system has enough information to construct the default gallery without any hand-written view definition.

**Auto-generated Table view.** Enumerate all `Column<In<M>, As<V>>` for all `M` where `M: SupportsEntity<E>`. Each column becomes a table column with type-appropriate rendering (text, number, badge, image). The table view shows all available metadata by default, with columns togglable.

**Auto-generated Board view.** For entity types that have a status-like metadata field (detectable via a marker trait like `StatusField` or by convention on columns with enum-like values), generate a Kanban board grouped by that status.

The user never sees "No views configured." They see their games in a gallery from the moment the first entity is catalogued. Customisation is additive — editing a working default, not building from nothing.

---

## 4. The Composites Table as Materialised View — What to Precompute, When to Refresh

The architecture defines `Composites` as a host-managed, plugin-read-only metadata table containing cached aggregates. The EAV research identifies this as the canonical mitigation for the N-join problem in list views. But Magento's flat catalog experience warns us: denormalisation caches that fall out of sync with source tables during high-write periods are worse than no cache at all, because they silently serve stale data.

### What to Precompute

The composites table should cache exactly the fields that appear in the default list/gallery views — the fields that every paginated query touches:

| Field | Source table | Why |
|-------|-------------|-----|
| `primary_title` | `titles` (where `is_primary = true`) | Every view needs a name. Avoids a join + filter on every list query. |
| `cover_art_url` | `media` (where `media_type = 'cover'`) | Gallery view renders this for every card. Without caching, it is a join + filter per entity. |
| `primary_rating` | `ratings` (preferred source or average) | Sort-by-rating is the most common sort. Caching avoids a join for the sort column. |
| `primary_genre` | `classifications` (first genre label) | Badge display and genre filtering in galleries. |
| `platform` | `classifications` (where scheme = 'platform') | Platform filtering is fundamental to a game library. |
| `entity_type_name` | `entity_types.name` (via anchor join) | Multi-type views need this without joining the anchor. |

Fields that should NOT be in composites:
- Infrequently accessed metadata (credits, compat info, full descriptions) — these are detail-view-only and the single-entity join cost is acceptable.
- Volatile, high-frequency-update fields (download progress) — the refresh cost would exceed the read benefit.
- Fields that are only meaningful in specific contexts (download status is irrelevant in a game browse view).

### When to Refresh

The fire-and-forget scheduler with work units provides the natural refresh mechanism. Three trigger patterns:

**Immediate (synchronous) refresh** for user-initiated writes. When a user manually edits a title or rating through the web UI, the composites row should update within the same request cycle. Staleness here is perceptible and frustrating. This is a single row update — fast even synchronously.

**Deferred (async) refresh** for bulk enrichment. When a connector enriches 500 games with ratings from a metadata API, the composites updates are batched as a single work unit emitted after `EntityEnriched`. The scheduler processes this in the background. Brief staleness (seconds to minutes) is acceptable because the user did not trigger the write and may not be watching.

**Full rebuild** as a Housekeeper task. A periodic sweep (daily or on-demand) that recomputes all composites rows from source tables. This is the safety net against drift. On a library of 100k entities, a full rebuild is a set of aggregate queries — expensive but bounded, and safe to run during idle periods.

### Detecting Composite Drift

Add a `composite_version` column to composites and a `last_modified` timestamp to each source metadata table. A lightweight Housekeeper work unit can sample N random entities, compare their composite row against source data, and flag drift rate. If drift exceeds a threshold (e.g., >5% of sampled rows are stale), trigger a full rebuild. This is the detection mechanism the Magento flat catalog lacked.

---

## 5. The Compile-Time ZST Advantage — How `SupportsEntity<E>` + `HasMetadata<M>` Prevents the Schema Awareness Burden

The EAV research identifies "schema awareness" as a medium-severity risk: every piece of code that handles entities generically must know the full set of metadata tables. Adding a new metadata table means updating every place that reconstructs full entities. This is the maintenance burden that plagues Magento (PHP runtime checks, attribute-name typos as common bugs), Salesforce (massive metadata cache layer), and WordPress (ad-hoc meta_key strings scattered through plugins).

Saalis eliminates this burden through a mechanism that none of the surveyed systems possess: **compile-time trait bounds that make invalid metadata access a compiler error.**

The dual-trait pattern (`SupportsEntity<E>` on the table side, `HasMetadata<M>` on the entity side) with `min_specialization` blanket bridging means:

1. **A DataView that requests a column from a table not registered for that entity type will not compile.** `DataView::for_entity::<Game>("x").col::<UserProfile>()` fails at compile time if `Game` does not implement `HasMetadata<UserProfile>`. No runtime "column not found" errors. No silent empty results.

2. **A plugin that adds a new metadata table automatically participates in the system.** The `#[register]` macro generates the descriptor; the host discovers it via `inventory`; the DataView builder can reference it. No manual update to a central registry. No "don't forget to add your table to the list" comments in code.

3. **Multi-entity union views enforce intersection semantics automatically.** `DataView::new("browse").entities::<Any<(Game, Movie)>>().col::<Rating>()` compiles only if BOTH `Game: HasMetadata<Rating>` AND `Movie: HasMetadata<Rating>`. This prevents the polymorphic query nightmare identified in the EAV research (§2.2): "all entities sorted by name requires knowing that games store names in game_metadata.title and users store names in user_metadata.username." In Saalis, if you want a cross-type view, you can only use metadata types that all included entity types share — and the compiler enforces it.

4. **The schema awareness problem becomes a non-issue for application code.** Instead of code that must enumerate all possible metadata tables, code operates on trait bounds: "give me any entity type E where E: HasMetadata<Titles> + HasMetadata<Rating>." The compiler resolves which concrete types satisfy this. Adding a new metadata table updates only the code that specifically opts into it.

This is Saalis's single most significant architectural advantage over every EAV-like system surveyed. Magento uses PHP strings. Notion uses runtime type fields. Salesforce uses a metadata cache. Datomic uses transaction-time schema enforcement. Only Saalis has compile-time trait bounds that make schema violations impossible to ship.

The cost is real: Rust compile times, `min_specialization` is nightly-only, and the ZST pattern has a learning curve. But for an appliance binary that compiles once and deploys, the compile-time cost is paid once and the runtime benefit is permanent.

---

## 6. Gallery View as Primary + Performance Implications of Media Metadata Joins

The UX research is unambiguous: for visual content libraries (games, movies), the gallery view is the primary browsing experience. Playnite, LaunchBox, Steam, and every game library tool confirms this. Users expect large cover art thumbnails with title and status overlay. The table view is a power-user tool; the gallery is the front door.

This creates a direct performance tension with the EAV model. A gallery view for 48 games per page requires, at minimum:

- `titles` join (for the card title)
- `media` join (for cover art URL)
- `ratings` join (for the rating badge, optional)
- `classifications` join (for genre/platform badges, optional)

Four joins across 48 entities. Without the composites table, this is the query that runs on every page load, every scroll, every filter change. With covering indexes on each metadata table, SQLite handles this in low single-digit milliseconds at 10k entities. At 100k entities, the query planner's index selection becomes more variable. At 1M entities (unlikely for a personal library, but possible for a shared catalogue), it becomes the dominant performance bottleneck.

The composites table is specifically designed for this: `primary_title`, `cover_art_url`, `primary_rating`, and `primary_genre` are all composite fields. The gallery query becomes:

```sql
SELECT c.primary_title, c.cover_art_url, c.primary_rating, c.primary_genre
FROM composites c
JOIN entities e ON c.entity_id = e.id
WHERE e.type_id = ?
ORDER BY c.primary_title
LIMIT 48 OFFSET ?
```

One join (composites to entities), one WHERE clause, one ORDER BY — all on indexed columns in a single table. This is the same optimisation that WooCommerce's HPOS migration achieved: collapsing a multi-table pivot into a single denormalised read path.

But the gallery view has a subtlety that list/table views do not: **image loading is the real bottleneck, not SQL**. The database can return 48 cover art URLs in 2ms, but the browser must then fetch 48 images. The performance strategy for galleries must therefore extend beyond SQL:

- **Precomputed thumbnails.** The `Media` table stores original URLs. A Housekeeper work unit should generate and cache thumbnail-sized versions (e.g., 300x400px WebP) on disk or in a `media_cache` table. The gallery DataView references the thumbnail URL, not the original.
- **Lazy loading with intersection observer.** The first viewport of cards (typically 8-12) loads immediately; the rest load as the user scrolls. The DataView builder should support a `.lazy_images()` modifier.
- **Placeholder aspect ratio.** CLS (cumulative layout shift) is the gallery-view UX killer. The composites table should cache cover art dimensions (`cover_width`, `cover_height`) so the HTML can set explicit aspect ratios before images load.

---

## 7. Novel Ideas: Auto-Generated DataViews and Progressive Schema Disclosure

### 7.1 Auto-Generated DataView Definitions from Registered Metadata Tables

The compile-time registration system knows which metadata tables exist for each entity type. This enables a capability that no surveyed tool offers: **DataView definitions that are generated from the type system, not manually constructed.**

The concept: a `DefaultViews` trait that any entity type can implement, with a derive macro that inspects `HasMetadata<M>` bounds:

```rust
#[derive(DefaultViews)]
struct Game;
```

The derive macro examines which metadata tables `Game` has registered (via `HasMetadata`) and generates view definitions based on metadata table traits:

- If `HasMetadata<Media>`: include cover art in gallery view; mark entity as "gallery-capable."
- If `HasMetadata<Titles>`: include title column in all views; use primary title as card label.
- If `HasMetadata<Rating>`: include rating in gallery overlay and as a default sort option.
- If `HasMetadata<Classifications>`: include genre/platform as filterable badges.
- If any metadata table implements a `StatusLike` marker trait: generate a Board view grouped by that status.

This means a plugin author who defines a new entity type and implements `HasMetadata` for several tables gets default views for free. The plugin does not need to define any presentation logic — the type system provides enough information.

For first-party entity types, the auto-generated defaults can be overridden with explicit DataView definitions. The override mechanism is simple: if a named view already exists in the explicit definitions, the auto-generated one is suppressed.

### 7.2 Progressive Schema Disclosure — Simple Properties First, Power Features on Demand

The UX research distills a four-layer progressive disclosure model (§10). The synthesis insight is that this model can be *driven by the type system*, not just by UI design patterns.

**Layer 0 (Zero-config): Composites only.** The default gallery and list views read exclusively from the composites table. The user sees title, cover art, rating, and genre. No configuration. No awareness of metadata tables. This is the Steam/Playnite experience.

**Layer 1 (View customisation): Column toggling.** The user opens the Table view and sees a column picker. The available columns are enumerated from all `Column<In<M>, As<V>>` for all `M: SupportsEntity<E>`. Each column shows its human-readable name (from `Display` impl) and type icon (text, number, date, image — from `As<V>` type). The user toggles columns on and off. They never see SQL. They never see "metadata table." They see "Show Rating," "Show Platform," "Show Release Year."

**Layer 2 (Advanced filtering): Compound filters with AND/OR.** The visual filter builder (adopted from Fibery/Notion UX patterns) lets users compose filter groups. The available filter operators are determined by the column's `As<V>` type: text columns get "contains/starts with/equals," number columns get ">/</between," enum-like columns get "is any of." The type system constrains what the filter builder offers, so invalid filters cannot be constructed.

**Layer 3 (Custom properties): User-defined metadata.** The architecture supports plugin-defined metadata tables. For advanced users who want to track custom properties (e.g., "personal difficulty rating," "estimated play time"), a user-metadata table with a flexible schema (closer to key-value, but scoped per user) can overlay the typed metadata. This is the escape hatch — it sacrifices compile-time type safety for user flexibility, but it is explicitly gated behind an "Advanced" interaction that Layer 0-2 users never encounter.

The key design principle: **each layer uses the type system to constrain what the UI offers, so the user cannot make invalid choices at any layer.** Layer 0 shows composites (always valid). Layer 1 shows registered columns (always valid by construction). Layer 2 offers type-appropriate operators (always valid by `As<V>` type). Only Layer 3 introduces user-defined schema, and even there, the system validates types on input.

### 7.3 View Recommendation Engine

A novel combination of the EAV research (which identifies which query patterns are expensive) and the UX research (which identifies which views are appropriate for which data shapes): the DataView system could recommend views based on the registered metadata.

- Entity type with `HasMetadata<Media>` and >80% cover art fill rate? Recommend Gallery as default.
- Entity type with a `StatusLike` column and >3 distinct status values? Recommend Board.
- Entity type with a date column (release date, creation date)? Recommend Calendar as a secondary view.
- Entity type with >10 metadata columns? Recommend Table as a secondary view for power users.

These recommendations surface during the progressive disclosure: when a user clicks "Add view," the system pre-suggests the most appropriate view types based on the entity's metadata shape, rather than presenting an undifferentiated list of all view types.

### 7.4 Metadata Fill-Rate Dashboard

One concern from the EAV research is the "ghost entity" problem — entities that exist in the anchor table but lack expected metadata. A related concern is metadata sparseness: if 60% of games lack ratings, sorting by rating produces confusing results (nulls at the top or bottom).

A fill-rate dashboard, generated automatically from the registration system, shows for each entity type and each metadata table: how many entities have at least one row, the percentage fill rate, and which sources contributed data. This serves both as a data quality tool and as a composites health monitor — if the composites fill rate diverges from the source tables' fill rate, something is wrong.

The DataView builder can use fill-rate data to make smart defaults: if `Rating` has <10% fill for a given entity type, do not include it in the default gallery overlay (it would be empty for most cards). If `Media` has <50% cover art fill, fall back to a list view as the default instead of gallery (empty gallery cards look broken).

---

## 8. The WordPress HPOS Migration Lesson — When Flexible Models Need Denormalisation

The EAV research dedicates significant attention to WooCommerce's HPOS migration: order data moved from the generic `wp_postmeta` (key-value EAV) to dedicated custom tables with proper columns. The result was 5x faster order creation and 40x faster backend filtering. This is the most dramatic real-world validation of the principle that Saalis's architecture is built on: typed metadata tables over generic key-value stores.

But HPOS also teaches a subtler lesson: **even well-designed flexible models eventually need targeted denormalisation, and the trigger is always a specific query pattern that crosses the performance threshold.**

### Detecting the Denormalisation Threshold

WooCommerce's HPOS migration was triggered by a specific symptom: the orders admin page, which filtered and sorted wp_postmeta rows, degraded to 30+ second page loads at scale. The root cause was a query pattern (multi-attribute filtering on a key-value table) that crossed a threshold where index-assisted lookups became full table scans.

For Saalis, the analogous query patterns to monitor are:

1. **Gallery page load time.** The primary composites query. If this exceeds 100ms consistently, the composites table needs restructuring — either additional precomputed fields, or a different index strategy.
2. **Multi-filter list view.** "Games on SNES with rating > 8, sorted by title." This crosses three metadata tables. If it exceeds 200ms, the filter columns need to migrate into composites.
3. **Search results.** Full-text search across titles, descriptions, and classifications. If the FTS5 index becomes stale or slow, the search index table needs restructuring.

### The Detection Mechanism

Rather than waiting for user complaints (as WordPress did), Saalis can build threshold detection into the Doctor subsystem:

- **Query timing instrumentation.** The DataView builder's `.execute()` method records query duration. Queries exceeding a configurable threshold (e.g., 200ms) emit a `QueryPerformanceExceeded` event.
- **The Doctor subsystem correlates slow queries with their metadata table joins** and recommends denormalisation: "The gallery view for Game entities averages 350ms. The `classifications` join accounts for 60% of execution time. Consider adding `primary_platform` to the composites table."
- **Automatic ANALYZE scheduling.** After bulk imports, the Housekeeper runs `ANALYZE` to update `sqlite_stat1`. This prevents the common SQLite pitfall where the query planner makes bad decisions because statistics are stale.

### Staged Denormalisation Strategy

The WordPress/HPOS lesson suggests a staged approach rather than a big-bang migration:

**Stage 0: No denormalisation.** Source metadata tables serve all queries directly. Composites table is empty. This is the correct starting state for development and small libraries. It proves the normalised model works and establishes baseline query performance.

**Stage 1: Gallery composites.** The first denormalisation step: precompute the gallery card fields (title, cover art URL, rating, genre). This is triggered when gallery page load exceeds 50ms consistently — likely at 5k-10k entities. The composites table starts small and targeted.

**Stage 2: Filter composites.** When multi-filter queries become slow, add the most-filtered columns to composites. The Doctor subsystem identifies these automatically from query logs. Platform and genre are the likely first candidates.

**Stage 3: Hilavitkutin columnar cache.** For analytics and bulk queries (e.g., "distribution of ratings across all games," "platform popularity over time"), the columnar store sidesteps the join problem entirely. This is CQRS: normalised writes, columnar reads for analytics.

The critical discipline: **every composites field must have a documented source query and refresh trigger.** "primary_title comes from `SELECT name FROM titles WHERE entity_id = ? AND is_primary = true ORDER BY updated_at DESC LIMIT 1`, refreshed on `EntityEnriched` events targeting the `Titles` table." This prevents the Magento failure mode where flat indexes drifted from source because the refresh logic was incomplete.

---

## 9. Putting It Together — The Unified Strategy

The two research documents converge on a single architectural truth: **type safety and UX flexibility are not opposing forces — they are complementary, and the bridge between them is the registration system.**

The EAV research shows that the risks of a universal entity model — join explosion, schema awareness burden, inner platform effect, query performance degradation — are all mitigable through compile-time type constraints and strategic denormalisation. The UX research shows that user flexibility — multiple views, visual filters, progressive disclosure, default-that-just-works — requires knowing the data shape at system design time, not discovering it at runtime.

Saalis's `#[register]` + ZST + trait bound architecture provides both. The metadata tables are known at compile time, which means:

1. **Default views are generated, not configured** — because the type system knows what metadata each entity type carries.
2. **The composites table's schema is derived, not designed** — because the gallery/list views' column requirements are known from the DataView builder's type parameters.
3. **Invalid queries are compile errors, not runtime surprises** — because `SupportsEntity<E>` and `HasMetadata<M>` enforce valid metadata access.
4. **Progressive disclosure layers are type-driven** — because each layer exposes a subset of the registered columns, constrained by the column's `As<V>` type.
5. **Denormalisation decisions are data-driven** — because the Doctor subsystem can correlate query patterns with metadata table joins and recommend specific composites additions.
6. **Plugin-defined entity types get first-class views for free** — because the auto-generation inspects trait bounds, not hard-coded entity type lists.

This is the synthesis: the EAV research tells us *what to fear* (joins, staleness, schema awareness, performance cliffs). The UX research tells us *what to build* (gallery-first, default views, visual filters, progressive disclosure). The type system tells us *how to connect them safely*. No other system in the survey — not Notion, not Fibery, not Salesforce, not Datomic — has this particular combination. They all solve the flexibility problem at runtime (type fields, metadata caches, transaction-time validation). Saalis solves it at compile time, and that changes the entire risk profile.

The strategy, in summary:

| Concern | Solution | Mechanism |
|---------|----------|-----------|
| N-join list view performance | Composites table | Denormalised read cache with staged growth |
| Default views for any entity type | Auto-generated DataViews | `HasMetadata` trait bound inspection at derive time |
| Gallery as primary view | Composites + thumbnail cache | `primary_title`, `cover_art_url`, `cover_width`, `cover_height` |
| Schema awareness burden | Compile-time ZST bounds | `SupportsEntity<E>` + `HasMetadata<M>` prevent invalid access |
| Blank-page problem | Generated defaults + override | Entity types ship with views; users customise, never build from zero |
| Bidirectional relations | `References<T>` with inverse navigation | DataView `.show_inverse()` modifier |
| Filter UX | Type-driven visual filter builder | `As<V>` type constrains available operators per column |
| Denormalisation timing | Doctor subsystem query instrumentation | Automatic threshold detection + composites recommendations |
| Composite staleness | Scheduler-driven refresh + drift sampling | Immediate for user writes, deferred for enrichment, periodic full rebuild |
| Progressive disclosure | Type-system-driven layers | Each layer exposes a subset of registered capabilities |

---

## Sources

All sources from both input research documents apply. Key sources for this synthesis:

- WooCommerce HPOS migration: [Developer Docs](https://developer.woocommerce.com/docs/features/high-performance-order-storage/)
- Magento EAV flat catalog: [Magento Academy](https://meetmagentoacademy.github.io/magento2-training-resources/backend/performance/eav.html)
- Fibery entity-relationship model: [Custom Domain](https://fibery.com/features/custom-domain), [Database Design](https://fibery.com/blog/guides/how-to-design-a-database-in-fibery/)
- Notion blank-page problem: [Onboarding Analysis](https://onboardme.substack.com/p/how-notion-solved-the-blank-page-product-strategy-deepdive)
- Progressive disclosure: [NN/G](https://www.nngroup.com/articles/progressive-disclosure/)
- SQLite query planner: [SQLite Docs](https://sqlite.org/queryplanner.html), [Optimizer Overview](https://sqlite.org/optoverview.html)
- CQRS pattern: [Azure Architecture Center](https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs)
- Saalis architecture: `docs/plans/2026-03-10-final-architecture-decisions.md` §1.2, §4, §15
