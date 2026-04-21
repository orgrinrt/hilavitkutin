# Progressive Disclosure UX Meets Compile-Time Type-Safe Schema: The Saalis Experience

**Date:** 2026-03-14
**Context:** Synthesis of data classification UX research, universal entity model analysis, DataView architecture, and presentation layer design. Defines how saalis bridges the gap between Notion-like flexible UX and Rust's compile-time type safety.

---

## The Central Tension

Saalis occupies an unusual position in the design space. On one side sits a compile-time type-safe schema: ZST column markers, `SupportsEntity<E>` trait bounds, registered metadata tables, and a `#[register]` macro system that makes invalid metadata access a compile error. On the other side sits a user-facing ambition: Notion-like flexibility where users browse, filter, customise, and build views over their content library without ever touching code.

These two forces are usually in opposition. Type-safe systems produce rigid UIs; flexible UIs sacrifice type safety. Saalis resolves this by making the type system generative rather than restrictive. The compiler enforces correctness at the schema boundary. The UI layer reads the registered schema and generates its presentation automatically. Users interact with the generated surface --- they never see the types, but the types ensure that everything they see is correct.

This document synthesises how that works across the full progressive disclosure stack, from zero-configuration browsing to extension-bundled view presets.

---

## 1. Auto-Generated Default Views From Registered Metadata

When a new entity type is registered --- say `Game` with `HasMetadata<GameMetadata>`, `HasMetadata<Rating>`, `HasMetadata<Media>`, `HasMetadata<Classifications>` --- the system already knows everything it needs to produce a working UI. Each metadata table is a `MetadataTable` implementor with typed columns expressed as `Column<In<M>, As<V>>` ZSTs. Each column carries its value type (`As<Text>`, `As<Integer>`, `As<Url>`). The `#[register]` macro collects these at compile time via `inventory`.

The presentation layer walks this registry at startup and constructs default views:

**Gallery view.** The system looks for a metadata table with an image-typed column (a `Column<In<Media>, As<ImageUrl>>` or similar). If found, that column becomes the gallery card's hero image. The first text column from the primary metadata table (`GameMetadata.title`) becomes the card heading. Badge columns (`Classifications.genre`) become tag pills on the card. The gallery view is assembled without any view-specific code --- it emerges from the column types.

**Table view.** Every column from every metadata table registered for the entity type becomes a potential table column. The default table view shows columns from the primary metadata table (the one named after the entity type, by convention) and hides columns from auxiliary tables. Users can toggle column visibility. Column types determine sort behaviour: `As<Integer>` sorts numerically, `As<Text>` sorts lexicographically, `As<Date>` sorts chronologically.

**Board view.** The system looks for enum-typed or badge-typed columns with a small number of distinct values. For games, `DownloadStatus.status` (Queued, Downloading, Downloaded, Verified, Failed) is a natural board-column candidate. If no suitable column exists, the board view is omitted from the defaults rather than showing something nonsensical.

**List view.** A compact fallback: entity name, one or two key metadata values, and available actions. Always generated, always available.

The critical property of this system is that default views are correct by construction. The column types constrain what the UI can do: you cannot sort an image column, you cannot use a free-text column as a board grouper, you cannot filter a blob column. These constraints are not runtime checks --- they are consequences of the type metadata baked into the registry at compile time.

No entity type ever presents an empty page. The moment it is registered with at least one metadata table, it has views.

---

## 2. Extensions Adding Metadata Tables Auto-Surface in the UI

Consider a third-party extension: `saalis-ext-howlongtobeat`. It registers a new metadata table `HltbMetadata` with columns for `main_story_hours`, `completionist_hours`, and `play_style`. It implements `SupportsEntity<Game>` to declare that this table applies to game entities.

When the extension is loaded, the `inventory` registry now includes `HltbMetadata` and its columns. The presentation layer's startup walk discovers the new table. Without any UI code changes:

- The **Table view** gains three new columns (hidden by default, available in the column visibility toggle).
- The **Filter builder** gains new filter options: "Main Story Hours > X", "Play Style is any of [...]".
- The **Sort options** include "Sort by Completionist Hours".
- The **Gallery view** can optionally show `main_story_hours` as a metadata line on cards, if the user enables it.

This works because the view generation logic operates over the registry, not over a hardcoded list of tables. The registry is append-only at compile time (or at plugin load time for dynamic extensions). Every metadata table that declares `SupportsEntity<E>` for a given entity type automatically participates in that entity type's views.

The mechanism is the same one that makes `DataViewRead::fields()` work: the trait implementation on each metadata table describes its columns, types, and display properties. The generic `dataview_page` handler does not know about HltbMetadata specifically --- it queries the registry, finds all metadata tables for the entity type, and constructs the DataView from their combined column schemas.

This is the payoff of the ZST column system. `Column<In<HltbMetadata>, As<Float>>` is a type, not a runtime value. The compiler verifies that the extension's query code only accesses columns that exist in its own table. The UI layer reads the column metadata (type, label, visibility defaults) and renders accordingly. The extension author writes zero UI code. The host application writes zero extension-aware UI code. The registry mediates.

---

## 3. The Five-Layer Progressive Disclosure Model

Progressive disclosure applied to saalis follows five layers, each revealing more capability while keeping the previous layer fully functional.

### Layer 1: Browse

The user opens saalis and sees their game library in Gallery view. Cover art tiles fill the screen. A search bar sits at the top. Clicking a game opens its detail page. This is the Steam Library / Playnite experience. No configuration was required. The system populated default views from the registered metadata, ran the Curator subsystem to catalogue content, and the Enricher filled in cover art from connected metadata providers.

At this layer, the user's mental model is simple: "This is my collection. I can look at it and search it." The entire type-safe schema, the metadata table joins, the composite caching --- all invisible. The composites table pre-joins the fields needed for gallery rendering (title, cover URL, platform badge) so the gallery query is a single-table scan, not an N-join operation.

### Layer 2: Filter

The user notices the filter bar. Platform chips appear above the gallery (auto-generated from distinct values in `Classifications.platform`). Genre chips follow. A "Status" dropdown offers "Downloaded", "Queued", "Not Downloaded". Clicking a chip applies the filter; the gallery updates via HTMX partial swap. Active filters show as dismissible chips --- the user always knows what is filtered.

The filter bar is generated from `FieldMeta` entries. Each filterable column from each registered metadata table contributes a filter control. The control type depends on the column type: enum columns become chip selectors, numeric columns become range sliders (hidden under "More filters"), boolean columns become toggles. The filter builder maps directly to the typed query API: selecting "Platform = SNES" generates the equivalent of `ctx.query::<Game>().filter::<Classifications>(platform.eq("SNES"))`. The user clicks; the system generates a type-safe query.

### Layer 3: Customise

The user switches to Table view and discovers column visibility toggles. They hide "Developer" and "Publisher", add "Completionist Hours" from the HltbMetadata extension. They drag columns to reorder them. They save this configuration as a named view: "My RPG Tracker" with filters for genre = RPG, sorted by completionist hours descending.

Saved views are persisted as metadata on the User entity --- specifically, in a `UserViews` metadata table where each row stores a serialised view configuration (column set, filter state, sort state, view mode, entity type). This means views participate in the same entity system as everything else. They can be queried, backed up, and (later) shared.

At this layer, the user has moved from consumer to configurator. They are not writing code, but they are making structural decisions about how data is presented. The system supports this by making every view configuration reversible (undo via "Reset to default") and by always providing the auto-generated defaults as a fallback.

### Layer 4: Build

The user creates a new view from scratch. They pick an entity type (Game), choose a view mode (Board), select the grouping column (a custom "Backlog Status" property they added: Wishlist, Backlog, Playing, Completed, Dropped), configure which metadata columns appear on the board cards, and set default filters. They create a second view: a Table filtered to "Downloaded = true" with columns for file size, checksum status, and last verified date --- an audit view.

This layer exposes the full DataView builder surface through GUI controls. The visual builder maps one-to-one with the DataView builder API. Every configuration the GUI offers corresponds to a valid builder method call. The user is, in effect, using a visual programming environment where the "program" is a DataView configuration and the "compiler" is the view renderer.

The compound filter builder appears at this layer: AND/OR groups, nested conditions, "any of" / "none of" operators. The visual builder renders these as nested rows with connectors, similar to Notion's advanced filter UI but constrained by column types. You cannot apply "greater than" to a text column --- the builder simply does not offer numeric operators for text columns. Type safety surfaces as UX constraint, not as error messages.

### Layer 5: Automate

The user installs extensions that add automation: "When download status changes to Verified, move to Completed in my Backlog board." This layer is where the scheduler's event system meets the view system. Work units can emit events; views can be configured to respond to events by refreshing, re-sorting, or highlighting changed rows.

View templates --- pre-configured view bundles shipped by extensions --- also live at this layer. A "Steam Connector" extension ships with a "Steam Library" view preset: gallery mode, filtered to Steam-sourced games, sorted by last played, with Steam-specific metadata columns (achievements, playtime) visible by default. The user installs the extension and the view appears in their view picker, ready to use.

This layer is explicitly opt-in. A user who never reaches Layer 5 has a complete, satisfying experience at Layer 3 or even Layer 1. The layers are additive, not gated.

---

## 4. Gallery, Table, Board: One DataView System, Three Renderers

The DataView system separates data from presentation through the `DataViewRead` trait and the `DataViewBuilder<R>` generic builder. The same `Vec<R>` (where `R: DataViewRead`) feeds all three renderers. Filters, sorts, and pagination apply before rendering --- the renderer receives a pre-processed slice of items and displays them in its format.

### Gallery View: Cover Art From Media Metadata

The gallery renderer looks for specific field roles:

- **Hero image:** The first `FieldType::Image` field becomes the card's background or thumbnail. For games, this comes from `Media` metadata (cover art URL). The composites table pre-joins this, so the gallery query does not touch the `Media` metadata table directly.
- **Title:** The first `FieldType::Text` field becomes the card heading.
- **Badges:** All `FieldType::Badge` fields render as coloured pills below the title. Platform, genre, and download status appear as compact visual indicators.
- **Secondary text:** Remaining visible text and number fields render as small metadata lines.

The gallery is the default view for entity types that have image metadata. If an entity type registers no image columns, the default switches to List view. This heuristic is computed at startup from the column registry.

### Table View: Dense Metadata Comparison

The table renderer maps directly to the `FieldMeta` schema. Each field becomes a column header. Sort indicators appear on sortable columns. Inline editing (for `DataViewWrite` implementors) renders the appropriate control: toggles for booleans, dropdowns for enums, text inputs for strings. Column widths are computed from field type: images get fixed narrow widths, text fields get flexible widths, numbers get compact fixed widths.

The table is the power user's primary interface. Bulk selection checkboxes, batch actions, and column reordering make it suitable for metadata auditing and mass operations ("Mark all selected as verified", "Add tag 'retro' to selection").

### Board View: Download Status Tracking

The board renderer groups items by a designated enum or badge field. Each distinct value becomes a column. Items render as cards within their column. Dragging a card between columns updates the grouping field --- the Kanban status-update pattern from Obsidian's Kanban plugin, implemented natively.

For download tracking, the natural grouping field is `DownloadStatus.status`. Columns: Queued, Downloading, Downloaded, Verified, Failed. Each card shows the game title, a progress indicator (for Downloading), and contextual actions (Retry for Failed, Verify for Downloaded). Moving a card from Downloaded to Verified triggers a checksum verification work unit through the scheduler.

The board view is the most opinionated renderer: it requires a grouping field with a small number of distinct values. The view builder validates this at configuration time, not at render time. If the user selects a text field with 500 distinct values as the grouping column, the builder refuses with an explanation ("This field has too many distinct values for a board view. Try filtering first, or choose a field with fewer categories.").

### View Switching

A toolbar above the data area offers view mode buttons: Gallery, Table, Board, List. Switching preserves filters, sorts, and pagination state --- only the renderer changes. The URL encodes the current view mode (`?mode=gallery`), so bookmarks and back-button navigation work across mode switches.

This "one source, many views" principle is the single most important UX concept adopted from the research. Notion, Fibery, Coda, and Org-mode all validate it. Saalis's implementation is distinct because the view modes are not arbitrary --- they are constrained by the registered column types. The type system ensures that every view mode that appears in the toolbar is valid for the current data shape.

---

## 5. Visual Filter Builder Mapped to the Typed Query API

The filter builder is the bridge between user intent and the typed query system. Users interact with visual controls; the system translates clicks into type-safe query operations.

### The Mapping

When the user selects "Platform" from the filter property dropdown, the system knows this is a `Column<In<Classifications>, As<Enum>>`. The operator dropdown shows only enum-compatible operators: "is", "is any of", "is none of". The value picker shows a chip selector with all distinct platform values, auto-populated from the data.

When the user selects "Rating" from the property dropdown, the system knows this is a `Column<In<Rating>, As<Float>>`. The operator dropdown shows numeric operators: "equals", "greater than", "less than", "between". The value input renders as a number field with appropriate constraints.

This is not a general-purpose query language. It is a constrained visual interface where every possible interaction produces a valid query. The constraints come from the column type metadata in the registry. The filter builder cannot produce an invalid query because the available operators and value inputs are determined by the column type at render time.

### URL Encoding

Applied filters encode in URL query parameters: `?platform=SNES&rating_min=8&sort=title&dir=asc`. The route handler deserialises these into a `ViewState`, which the `DataViewBuilder` uses to drive the filter-sort-paginate pipeline. HTMX partial swaps update the URL via `hx-push-url`, so the browser's back button navigates filter history.

### Compound Filters

At Layer 4 (Build), the filter builder exposes AND/OR grouping. The visual representation is nested rows with logical connectors:

```
WHERE
  Platform is any of [SNES, Genesis]
  AND (
    Rating > 8
    OR Genre is "RPG"
  )
```

Each row is a single filter condition with property, operator, and value controls. Groups are indented blocks with AND/OR toggles. The visual builder prevents impossible combinations: you cannot AND two mutually exclusive values of the same enum field (the system collapses it to "is any of").

### What This Replaces

In a raw query API, the equivalent would be:

```rust
ctx.query::<Game>()
    .filter::<Classifications>(platform.is_any_of(&["SNES", "Genesis"]))
    .and(
        ctx.filter::<Rating>(score.gt(8.0))
            .or(ctx.filter::<Classifications>(genre.eq("RPG")))
    )
```

The user never sees this. They click dropdowns and chips. The system generates it. The types guarantee it is valid. This is the core promise: compile-time safety for the developer, visual simplicity for the user, no impedance mismatch between the two.

---

## 6. Smart Defaults: Sensible Views From Metadata Registration

When a new entity type is registered, the system must decide what views to create as defaults. This decision is algorithmic, driven by the registered metadata tables and their column types.

### The Heuristic

1. **Does the entity type have an image column?** If yes, default to Gallery view. If no, default to List view.
2. **Does the entity type have an enum/badge column with 2-8 distinct values?** If yes, generate a Board view grouped by that column.
3. **Always generate a Table view** with all primary metadata columns visible and auxiliary columns hidden.
4. **Always generate a List view** as the compact fallback.

### Column Visibility Defaults

- Columns from the primary metadata table (the one conventionally named after the entity type) are visible by default.
- Columns from auxiliary metadata tables (ratings, media, credits) are hidden by default but available in the column picker.
- Extension-added columns are hidden by default.
- Image columns are visible only in Gallery and detail views, not in Table or List.

### Sort Defaults

- Gallery: sorted by title (alphabetical).
- Table: sorted by the first sortable column (usually title).
- Board: sorted by the column's natural order within each group.
- List: sorted by most-recently-added (descending creation date).

### Filter Defaults

No filters applied by default. Suggested filters appear as ghost chips above the view: "By platform", "By genre", "By status". Clicking a suggestion opens the filter builder pre-populated with that property.

These defaults mean that a connector extension that introduces a new entity type (say `Movie` for a future movie library connector) gets a complete, working UI the moment it registers `Movie` as an entity type with `MovieMetadata`, `Rating`, and `Media` tables. The extension author's effort is zero on the presentation side. The host application's effort is also zero. The defaults emerge from the type registry.

---

## 7. Novel Ideas

### 7.1 Extension-Bundled View Templates

Extensions can register view presets alongside their metadata tables and connector logic. A "Steam Connector" extension ships:

- **"Steam Library" view:** Gallery mode, filtered to entities catalogued by the Steam connector, sorted by last played (from `Stats` metadata), with Steam-specific columns visible (achievements, playtime from a `SteamStats` metadata table the extension registers).
- **"Steam Wishlist" view:** List mode, filtered to entities on the user's Steam wishlist (from `Watchlist` metadata cross-referenced with Steam identity), sorted by discount percentage (if the extension tracks pricing).

View templates are registered via `#[register]` like everything else. They are descriptors: a name, a target entity type, a serialised view configuration, and an optional icon. The view picker in the UI shows extension-provided presets alongside user-created views, distinguished by a small badge indicating their source.

The user can use an extension preset as-is, clone it and modify the clone, or ignore it entirely. Presets are read-only --- modifying a preset's filters creates a user-owned copy, leaving the original intact. If the extension updates and ships a revised preset, the user's copies are unaffected but the original updates.

### 7.2 User-Created Views as Entity Metadata

Saved views are rows in a `UserViews` metadata table on the `User` entity. Each row stores:

- `view_name: Text`
- `entity_type: EntityTypeId`
- `view_config: Json` (serialised view mode, column set, filters, sorts)
- `is_pinned: Boolean` (appears in the sidebar)
- `is_shared: Boolean` (visible to other users)

Because views are metadata on the User entity, they participate in all the systems that metadata participates in: backup, export, sync, and --- crucially --- they can be queried. "Show me all views created by User X" is a standard metadata query. "How many users have a board view for download status?" is an analytics query over the same table.

### 7.3 View Sharing Between Users

When `is_shared` is true, a view appears in other users' view pickers under a "Shared Views" section. The view configuration is read-only for non-owners. Other users can clone a shared view to customise it.

In a multi-user household scenario (the Batocera appliance serving a family), a parent might create a "Kid-Friendly Games" view: Gallery mode, filtered to `Classifications.age_rating <= "E10+"`, with download and delete actions hidden. This view is shared with the `Child` role. The child user sees the curated gallery without the ability to modify the filters or access restricted actions. The `AdminGuard` and role-based `SetAccess` system enforce this at the action level, not just at the view level.

### 7.4 Responsive View Adaptation

The HTMX + Maud template system renders server-side, but CSS media queries and the view system can cooperate for responsive adaptation:

- **Wide screens (>1200px):** Gallery view renders 4-6 cards per row. Table view shows all visible columns.
- **Medium screens (768-1200px):** Gallery renders 2-3 cards per row. Table collapses low-priority columns (those added by extensions, or explicitly marked as collapsible).
- **Narrow screens (<768px):** Gallery renders 1-2 cards per row. Table switches to List view automatically. Board collapses to a single-column vertical list with group headers.

The view mode stored in the URL is the user's preference for wide screens. On narrow screens, the server can detect the viewport (via a cookie set by a small client-side script on first load, or via `Sec-CH-Viewport-Width` client hints) and override the view mode in the template. The user's saved preference is not changed --- only the rendered output adapts.

The action budget system already supports this: `ItemAction` has a weight, and the component has a budget for inline display that shrinks on narrow screens. High-priority actions (Download, Play) remain visible; low-priority actions (Edit Metadata, View History) move to the overflow menu.

### 7.5 View Inheritance and Composition

A more speculative idea: views that compose with other views. A "Dashboard" page could embed multiple DataViews as tiles:

- Top-left: Board view of active downloads (filtered to `DownloadStatus.status IN (Queued, Downloading)`), compact mode.
- Top-right: Gallery of recently added games (sorted by creation date, limit 8), no filter bar.
- Bottom: Table of failed downloads with retry actions.

Each tile is an independent DataView with its own query, filters, and rendering. The dashboard is a layout of DataView references, not a single monolithic view. This is analogous to Notion's linked databases and Fibery's Smart Folders --- the same data source, different filtered lenses, composed on a single page.

Dashboard layouts could themselves be saved as user metadata, extending the view system to support multi-view compositions.

---

## 8. Avoiding the Blank Page Problem

The research identifies the blank-page problem as the primary UX failure mode for flexible tools. Notion solved it with templates and onboarding questions. Obsidian mitigated it with starter vaults. Saalis must never show an empty page.

### The Principle: Always Show Something Actionable

Every screen in saalis must present either content or a clear path to content. There are three scenarios to handle:

**Scenario 1: First launch, no content.** The user has installed saalis on a fresh Batocera system. No games are catalogued yet. The default Gallery view would show zero items.

Instead of an empty gallery, the system shows a guided setup flow:

1. "Welcome to saalis. Let us find your games." --- with a scan button that triggers the Curator's filesystem scanner.
2. If ROM directories are detected (Batocera has standard paths), show what was found: "Found 847 files across 12 systems. Catalogue now?" --- with a progress indicator.
3. If no ROM directories are detected, show connector setup: "Connect a source to start building your library" --- with cards for available connectors (local filesystem, Steam, etc.).

The key: the first screen a new user sees is never empty. It is always a call to action with a clear next step.

**Scenario 2: Entity type with no entries.** The user has games but installs a future "Movies" extension. The Movie entity type has zero entries. Instead of an empty gallery, the view shows:

- A header explaining the entity type: "Movies --- Your film collection."
- A call to action: "Add movies manually" (form link) or "Connect a movie source" (connector setup link).
- If a movie connector is installed but unconfigured: "Configure [ConnectorName] to start cataloguing movies."

**Scenario 3: View with filters that match nothing.** The user applies filters that exclude all items. Instead of a blank area, the system shows:

- The active filter chips (so the user sees what they filtered by).
- A message: "No games match these filters."
- A suggestion: "Try removing [filter with fewest matches] to see more results." --- The system computes which filter, if removed, would yield the most results, and suggests removing it.
- A "Clear all filters" button.

**Scenario 4: Extension loaded but no data yet.** The HltbMetadata extension is installed, but the enricher has not run yet. The "Completionist Hours" column exists but all values are empty. Instead of showing a column of blanks:

- The column shows a subtle "Pending enrichment" indicator in the header.
- A tooltip on the indicator explains: "This data will be populated when the HowLongToBeat enricher runs."
- If the enricher has never run, a one-time inline prompt offers: "Run now?" --- which triggers a `ManualTrigger` event through the scheduler.

### The Implementation Pattern

Every view renderer checks its item count after filtering. If zero, it delegates to a "zero state" component that receives the entity type, the active filters, and the available connectors. The zero state component selects the appropriate message and actions from a decision tree:

1. No items of this entity type exist at all? --> Show onboarding/setup.
2. Items exist but filters exclude all? --> Show filter adjustment suggestions.
3. Items exist and pass filters but a specific metadata column is entirely empty? --> Show enrichment prompt.

This zero-state logic is generic over entity types. It reads the registry to determine what connectors, enrichers, and metadata tables are available for the current entity type, and constructs its suggestions accordingly. A new entity type introduced by an extension gets appropriate zero-state messaging without any extension-specific UI code.

### Never Require Configuration Before Showing Value

The most important design constraint: **Layer 1 (Browse) must work with zero user configuration.** The system must catalogue content, generate views, and present a functional gallery before the user makes any choices. This means:

- Default connectors (filesystem scanner) run automatically on first launch.
- Default views are generated from registered metadata without user input.
- Default enrichers run in the background after cataloguing, populating cover art and basic metadata.
- The user's first experience is opening saalis and seeing their game collection with cover art, searchable and browsable.

Every subsequent layer (Filter, Customise, Build, Automate) adds capability. No layer removes capability from the layers below it. A user who never leaves Layer 1 has a complete, useful application. This is the progressive disclosure contract: each layer is self-sufficient, and deeper layers are always optional.

---

## Architectural Implications

### The Registry as UI Schema

The `inventory`-based registry is not just a plugin system --- it is the UI's schema source. Every `MetadataTable` registration, every `Column` type, every `SupportsEntity` bound contributes to the UI's structure. This means the registry must expose enough metadata for view generation: column labels, display hints (image vs text vs badge), default visibility, sort behaviour, and filter type.

The `FieldMeta` struct in the DataView system already captures most of this. The bridge is a function (or trait method) on each metadata table type that produces `Vec<FieldMeta>` from its column declarations. The `#[register]` macro can generate this automatically from the column ZSTs, since each `As<V>` type maps to a `FieldType`: `As<Text>` maps to `FieldType::Text`, `As<ImageUrl>` maps to `FieldType::Image`, `As<Float>` maps to `FieldType::Number`, and so on.

### Composites as the View Performance Layer

The composites table is not an optimisation detail --- it is a core part of the view architecture. Gallery and List views should never query raw metadata tables directly. The composites table pre-joins the fields needed for list rendering: title, primary image URL, platform badge, download status, rating score. This keeps gallery queries as single-table scans regardless of how many metadata tables are registered.

When an extension adds a new metadata table, the composite does not automatically include its columns. Extension columns appear in Table view (via direct metadata table queries for visible rows only) and in detail views, but not in the gallery composite unless explicitly added. This is a deliberate performance boundary: the gallery must remain fast even with dozens of extension metadata tables registered.

### View Configurations as Portable Data

Because view configurations are serialised JSON stored as entity metadata, they are inherently portable. A user can export their views, share them as files, or include them in extension packages. The serialisation format references column IDs and metadata table names (registered `&'static str` values), not internal database column names or ORM model fields. This means view configurations survive schema migrations, metadata table renames, and extension version changes --- as long as the referenced column names remain stable.

The extension registration system already enforces name stability through the `Named` trait (`const NAME: &'static str`). A metadata table's name is its identity. Changing it is a breaking change that requires a migration. This stability guarantee extends to view configurations: a saved view referencing `HltbMetadata.main_story_hours` will continue to work as long as the extension maintains that column name.

---

## Summary

The saalis UX strategy rests on a single architectural insight: the compile-time type registry that enforces schema correctness also contains all the information needed to generate a complete, working UI. Types are not just constraints --- they are UI metadata. Every `Column<In<M>, As<V>>` is simultaneously a compile-time safety bound and a rendering instruction.

Progressive disclosure layers this capability:

1. **Browse** --- auto-generated views from registered metadata, zero configuration.
2. **Filter** --- visual filter builder constrained by column types, producing type-safe queries.
3. **Customise** --- column visibility, saved views stored as user entity metadata.
4. **Build** --- compound filters, custom view creation, full DataView builder surface through GUI.
5. **Automate** --- extension-bundled view presets, event-driven view updates, dashboard composition.

Each layer is self-sufficient. Each layer builds on the one below. No layer requires the one above. The blank page never appears. The type system ensures correctness. The user never sees a type. That is the saalis experience.
