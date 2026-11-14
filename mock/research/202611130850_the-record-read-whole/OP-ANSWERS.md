# What op answered on 13 November 2026 that bears on hilavitkutin

This note holds the questions put to the maintainer on 13 November 2026 that
bear on hilavitkutin, each with the options it was asked with and the answer
he gave, and the remarks he made unprompted on the same day about the engine.
The text under each heading is copied line for line from the captures kept in
the workspace repository, under `.data/op-responses/`, and nothing in it is
reworded. Each section names the capture and the lines it was copied from.

The registry's question rows quote this note, so that a row's passage can be
checked against a file in this repository. The times are the ones the captures
carry.

## The new crates beside the api, 05:12 and 05:16

Copied from `.data/op-responses/202611130508_the-canon-goes-provisional-and-hilavitkutin-is-revised-in.md`, lines 30 to 64.

### Said unprompted at 05:12

> Alaina want you to present in ask user tool the new crates and what "engine" in hilavitkutin means when you also mention "its own as another runner" ?
> I am wary and uncomfortable if that means you are adding something entirely out of place or too specialized or too parallel to what hilavitkutin is now, in there, since you separate it from the current stuff in hila
> Also you should use fable more daringly, I notice a lot of the work is on opus now, and this is all complex enough and needs bright, sharp calls and decisions, that fable should be used

### Asked, answered at 05:16

#### New crates

  ```text
  What the words mean. 'The engine' in hilavitkutin = the existing `hilavitkutin` crate: the static scheduler that turns WorkUnit read/write access sets into per-core morsel dispatch, with `hilavitkutin-api` as its contract. The loop-core branch (not landed, in review) adds four crates BESIDE that: `-contracts` (authors name a record domain, columns and resources and write value kernels; implementors write a store), `-reference` (one-thread runner kept as oracle), `-laws` (laws over recorded runs), `-frozen-authors` (authors that must never need editing). 'Ported later as another runner' means the existing scheduler would be rewritten to run those new contracts and the old api deleted (step HV18). It was put beside, not in place, because `hilavitkutin-api` sits on old arvo and moving it stops the existing engine building. So yes: until HV18 it is a second, parallel vocabulary for the same thing, and one expert flagged exactly that. How do you want it?
  ```

Options offered:

1. Hold, map it first (Recommended)

   Loop-core does not land yet. A fable seat maps every new item onto hilavitkutin's own design (WorkUnit, Column/Resource/Virtual, access sets, morsels) and says per item: already exists under its own name, extends it, or is new and why. You get that map, then decide.

2. Fold into the existing api

   No second contract crate. The value-kernel form, store trait and laws become a revision of `hilavitkutin-api` and its design in place; the reference runner and laws are the oracle for the existing scheduler. Costs: the existing engine is pinned at a tag or ported sooner.

3. Oracle and laws only

   Keep only the one-thread reference runner and the laws, written against today's `hilavitkutin-api`, as the test oracle the existing scheduler is held to. No new author or store vocabulary at all.

4. As planned, beside

   Land the four crates beside the old api, mark the old api legacy, converge at HV18 when the scheduler is ported onto the new contracts.

Chosen:

- Hold, map it first (Recommended)

## The registry and the tools, 05:26

Copied from `.data/op-responses/202611130508_the-canon-goes-provisional-and-hilavitkutin-is-revised-in.md`, lines 107 to 109.

### Said unprompted at 05:26

> Also we should start using the newer mockspace features like the registry and tools in both arvo and hilavitkutin. Worth it to start writing up the canon where it is missing and filling in even where it exists, and start using tickets. If tickets aren't upstreamed to mockstream yet as we use them in kaski, approx our shape for now and copy over some of the related tools that aren't upstreamed but useful and needed etc. or rather perhaps upstream the tools in full, the ones that don't depend on kaski specifically, to the mocksoace extra lints for now. I think the mockspace extensions as crates (deps in its toml) already support tools too along with lints, and perhaps also agent stuff too and maybe even registry schemas?

## The shape after the map and the data-bound table, 05:32

Copied from `.data/op-responses/202611130508_the-canon-goes-provisional-and-hilavitkutin-is-revised-in.md`, lines 133 to 185.

### Asked, answered at 05:32

#### Hila shape

  ```text
  Hilavitkutin, after the map: which shape for what the loop-core branch tried to add? (The branch has not landed. In every option the 20 duplicate items go and nothing calls `hilavitkutin-api` legacy.)
  ```

Options offered:

1. Instruments now, api round with you (Recommended)

   Now: only the one-thread oracle, the laws, the frozen crates, the ledger and lints, rewritten against today's `hilavitkutin-api`. Then a design round on `hilavitkutin-api` itself with you in it: per-domain record count, a frame-buffered column kind beside `Accum`, `RecordOp` widened into the primary value form, declared neighbour reads, safe exclusive writes. The mapping seat's pick.

2. Revise the api in place now

   Same destination without waiting for you in the round: the needed forms go into `hilavitkutin-api` and its design now, drafted by a fable seat from the map, reviewed, landed; you overturn afterwards what you dislike.

3. Instruments only

   Oracle, laws, frozen crates and lints against today's api, and nothing else. Contract changes wait until you raise them yourself.

4. Drop the branch whole

   Nothing of loop-core lands. hilavitkutin continues from its own roadmap r6, and kaski's needs are filed as tickets on it.

Chosen:

- Revise the api in place now

#### Data bind

  ```text
  The branch added a run-time binding table: content rows say which stored column fills which column of which kernel, so content can add columns with no Rust. Today hilavitkutin registers by type at compile time. Do you want the data-bound way in hilavitkutin?
  ```

Options offered:

1. Typed only for now

   Registration stays by type as designed. Kaski's content-defined channels are solved above it (lane index inside a typed block column) and a data-bound table is revisited only if that fails.

2. Both, typed first

   The typed registration stays primary; a data-bound table is a second, opt-in way to register, designed in the api round with the erased shape-described addressing you chose for resources on 2 July as its base.

3. Decide in the api round

   Leave it open; the round brings a sketch and a bench of both and you pick there.

Chosen:

- Decide in the api round

## The recovered rounds and the convergence call, 05:39

Copied from `.data/op-responses/202611130539_the-canon-walk-and-arvo-is-claimed-then-reworked.md`, lines 44 to 90.

#### PR #4

  ```text
  hilavitkutin has an open PR #4, `recover/unlanded-rounds-and-benches`: 226 files that `dev` lacks (recovered design rounds and benches from before the split). It was not opened tonight. What happens to it?
  ```

Options offered:

1. Review and land it

   A fable reviewer reads it against `dev` as recovered history (rounds, research, benches); it lands if nothing in it contradicts what landed since. The api revision can then cite the recovered research.

2. Triage first

   A helper lists what is in it by kind and age and what `dev` already superseded; you get the list and decide.

3. Leave it

   It stays open and untouched; nothing reads it.

Chosen:

- Triage first

#### r6 call

  ```text
  hilavitkutin's roadmap r6 records a supersession as 'the agent's call rather than op's': head-and-tail convergence replaced by an N-way record-order partition. No agent can ratify it. What do you want done with it?
  ```

Options offered:

1. Bring me both designs

   A one-page side by side of head-and-tail convergence and the N-way partition, with what each costs and what the code does today, as a question ticket for you. Nothing depends on it meanwhile.

2. Reverse it

   Head-and-tail convergence is the design again; the N-way partition is recorded as a proposal.

3. Ratify it

   The N-way record-order partition stands as the design.

Chosen:

- Bring me both designs

## The entity system, 05:05

Copied from `.data/op-responses/202611130448_answers-to-the-fix-up-calls-batch-1.md`, lines 89 to 111.

#### Lattice

  ```text
  How is the lattice held? Both experts lean to the first option; upstream block addressing fits the first two either way.
  ```

Options offered:

1. Tile is an entity (Recommended)

   A tile is an entity with one component pointing at its block in a kaski-owned store, found through the place index and read by a declared query.

2. Outside the entity system

   Tiles are keys of the store; the lattice does not touch entities at all.

3. Decide after benches

   Wait for lattice-floor and thing-tables to report, with an entity-cells arm and an entity-handle arm added to the bench.

Answered in their own words:

> Option 3 but I would be interested in reducing the ecs role to even zero if hilavitkutin proves capable and we can implement realistically the things we need under its mantle

## The lane width upstream, 05:05

Copied from `.data/op-responses/202611130448_answers-to-the-fix-up-calls-batch-1.md`, lines 137 to 159.

#### Lane width

  ```text
  May a lattice lane be narrower than 64 bits? (Your open ticket.) Both experts picked the first option.
  ```

Options offered:

1. Closed set 16/32/64 (Recommended)

   Formats exist as members; a channel's assignment waits for the lattice-floor bench and matching digests on native and wasm.

2. 64-bit everywhere

   One 64-bit lane with a 128-bit intermediate. The narrow formats stay unused.

3. Everything after the bench

   No format set is fixed until lattice-floor reports per width.

Answered in their own words:

> Upstream on hila it should be entirely generic and configurable by user. Unless that’s infeasible, but I swear I remember the research and panels there found ways and benched it okay with enough headroom to match or surpass as ceiling

## Four calls on the api revision, 05:50

Copied from `.data/op-responses/202611130550_four-calls-that-shape-the-hilavitkutin-api-revision.md`, lines 18 to 114.

### Asked, answered at 05:50

#### Overflow

  ```text
  hilavitkutin today asserts (panics) when a unit appends past an accumulator's capacity. A branch about to land makes that a refusal returned as a value from `run` (the frame fails closed, nothing is silently dropped, same result at every worker count). Both experts picked the first option. Do you want it?
  ```

Options offered:

1. Refusal as a value (Recommended)

   `run` returns the refusal; no panic path on a worker of a no_std engine that spawns nothing. If someone wants an abort it is the memory provider's property.

2. Assert stays the default

   The panic stays; a refusal value is something a provider opts into.

3. Configurable by the consumer

   A const, generic or trait parameter on the run configuration picks refuse or assert, default refuse.

Chosen:

- Configurable by the consumer

#### Changed

  ```text
  When hilavitkutin tracks which records 'changed' in a frame (so only readers of changed data are woken), what does changed mean for a computed column? Both experts picked the first option; they call it a derivation of yours to confirm, not canon text.
  ```

Options offered:

1. Differs from the frame before (Recommended)

   A value compared with the committed one, at a granularity fixed in the contract. Equal rewrites wake nobody and the plumbing has no discretion.

2. Written this frame

   Any write marks it changed, whatever the value. Cheaper to track, wakes more.

3. Declared per column

   The column (or the unit writing it) declares which of the two holds, as a trait or const; default differs.

Chosen:

- Declared per column

#### Implementors

  ```text
  Your sentence: 'the work demands no change on implementors, it happens in the plumbing side of hilavitkutin'. Who counts as an implementor that must never need an edit when the plumbing grows?
  ```

Options offered:

1. Units, stores, providers, hosts (Recommended)

   Whoever writes units, and also whoever implements a store, a memory provider or a host against the contracts. Runners and the planner are plumbing and may change. Both experts read your 'Impls are trivial. And interchangeable' this way.

2. Unit authors only

   Only code written as units against the api. Stores and providers may need edits as the plumbing grows.

3. Also outside runners

   The above, plus anyone who writes their own runner or planner against the contracts.

Answered in their own words:

> Okay so that is the ideal. At this point we can break it, and in future too, so let’s rephrase it so that we’d prefer not to break the public facing api. Mostly thinking about unit authors and data structure authors and such, not so much runners, but not forgetting them

#### Freeze

  ```text
  The 'frozen' test units and implementations (kept unedited to prove your property: if growing the plumbing forces an edit to one, a lint fails). When do they freeze?
  ```

Options offered:

1. When their form locks (Recommended)

   Edits are allowed but each one is logged in a ledger until the api form they use is locked; the check asks that the last two slices added no ledger row; append-only after. Both experts picked this.

2. At first landing

   Append-only from the first commit; any first-night mistake becomes a second form beside the old.

3. When kaski's first unit lands

   Ledgered until a real consumer exists, then frozen.

Chosen:

- When their form locks (Recommended)

## The pillars, 05:52

Copied from `.data/op-responses/202611130552_the-pillars-of-hilavitkutin-and-arvo-and-old-arvo-returns.md`, lines 36 to 43.

### Said unprompted at 05:52

> Or you know, viable also to just simply upstream properly while doing something with it anyway.
> And do a txt file catalogue of stale and removable branches on those repos too so when console works again or when the agent api path for clearing the branches and creating tags comes online, those just clear along with the existing ones from the drakn
> Also I think you should start the next question round when the last one is still pushing, since it takes time. Hopefully once answered, the push has gone through in meanwhile
> And things can be added and extended and changed in hilavitkutin as long as the main principles and paradigm itself, the intent, doesn't change with it and it doesn't compromise on its main pillars of full, exhaustive typestate, contracts first, all machinery and logic runs on contracts alone, people impl its parts and the engine runs it via the contracts. And optimisations first and foremost, always, though as said, we can start simpler and later go deeper, incrementally. And whatever gets added, matches in taste and execution, is not a drop looser in typestate or strictness with no std, no alloc, unless behind a feature that is entirely opt in and not required for things to work
> Same for arvo of course
> If you look at the old arvo, the one hila pins for example. It used to have a whole lot going, and we'd like to bring it all back, too, just using the more sound and expressible, configurable, explicit framework and fundamentals

## Convergence and the lost round, 06:02

Copied from `.data/op-responses/202611130602_convergence-is-measured-first.md`, lines 36 to 88.

### Asked, answered at 06:02

#### Convergence

  ```text
  hilavitkutin, single-trunk phases: head-and-tail convergence or the N-way record split? The records were reversed back to head-and-tail in July (r7, r8, and the seed says 'Never an N-way record or morsel partition'), but the N-way path is what runs on `dev` and your design text carries both. Which is the design?
  ```

Options offered:

1. Head-and-tail, finish the deletion

   As r8 and the seed say. The N-way path in `worker.rs` is removed, the head-and-tail types get driven, the design's ceil-slice passages are corrected, a commutativity test is added.

2. N-way, retire head-and-tail

   The N-way record-order split is the design; the unused head-and-tail types and the seed sentence go.

3. Both, chosen at plan time

   Two inhabitants the planner picks between per fiber (commutative single-trunk: head-and-tail; ordered appends: N-way forward slices), stated in the contract.

4. Measure first

   A bench under the harness with both arms on single-trunk workloads; the question returns with the numbers. The code stays as it is meanwhile.

Chosen:

- Measure first

#### Lost round

  ```text
  The commit that r8 says deleted the N-way path ('round 202607192410') is not on `dev` and may be in PR #4's 226 recovered files. Should the PR #4 triage look for it specifically and, if it is there, bring that round's source change back as its own branch for review?
  ```

Options offered:

1. Yes, recover it for review

   The triage looks for round 202607192410's source change; if found it is put on its own branch and reviewed against today's `dev`, landing only on your answer to the first question.

2. Only list it

   The triage says whether it is there and what it changes; nothing is branched.

3. No, redo from the design

   Whatever the first answer is gets implemented fresh against today's `dev`; old source is not recovered.

Answered in their own words:

> Treat equal to rest in triage. Might be useful others and this too

## Resource read, changed, two writers and extent, 07:59

Copied from `.data/op-responses/202611130726_does-anyone-here-know-what-hilavitkutin-is.md`, lines 202 to 302.

### Asked, answered at 07:59

#### Resource read

  ```text
  hilavitkutin api revision. The value form is a unit written as a plain function: it receives the values of its declared reads and returns the values of its declared writes (your `RecordOp`, widened to lists). When such a unit reads a resource, how does it get it? This fixes the signature every unit author writes.
  ```

Options offered:

1. By reference

   The read product carries a lifetime and hands `&T`, so any resource works, including `Seq` and `Map` collections, as `resource() -> &T` does today. Signatures carry a lifetime.

2. By value, collections via block form

   Only `Copy` resources are read in the value form, by value, with no lifetime in the signature; a unit that needs a collection resource is written in the block form.

3. Both, by the resource's kind

   A `Copy` resource arrives by value and a collection by reference, chosen in the types from the resource's declared kind, so an author writes one form and the signature follows the declaration.

Answered in their own words:

> Yes. Use typestate and statically branch accordingly

#### Changed scope

  ```text
  You answered that what 'changed' means is declared per column (differs from the committed frame, or written this frame). The changelist applies that only to the new frame-buffered column kind, because a column written in place has no committed copy to compare with. Which stores declare it?
  ```

Options offered:

1. Every per-record kind, sensible defaults

   Every per-record store kind declares it; the default is differs for the frame-buffered kind and written for a column written in place. Declaring differs on an in-place column asks its values to be comparable and costs a compare.

2. Frame-buffered only

   Only the frame-buffered kind declares it; an in-place column counts as changed whenever its writer ran.

3. Every kind, default differs

   Every per-record kind declares it and the default is differs everywhere, so every column value must be comparable.

4. The writing unit declares

   The unit that writes says which meaning holds for its write, not the store.

Answered in their own words:

> Last option. Pretty sure this is somewhere in the research and proposed canon…

#### Two writers

  ```text
  Two units write the same frame-buffered column in one frame. Today, for other store kinds, your design resolves write-write by scheduling priority. For the frame-buffered kind there is no order between two writers of the staged side. What should happen?
  ```

Options offered:

1. Refuse now, disjoint sets later

   Refused at build (compile time) now; a later form lets two writers declare disjoint record sets and both write.

2. Refuse at build

   Always a compile-time refusal; one writer per frame-buffered column, full stop.

3. Priority, as other kinds

   Resolved by scheduling priority like every other kind; the lower-priority writer's work on overlapping records is dead.

Answered in their own words:

> Schedule should exist for this specifically. Are we sure that you and the agents working on this actually know what hilavitkutin is and is intended to be?

#### Extent

  ```text
  A record domain is a set of records with its own count (replacing the scheduler's single record count). Is a domain's extent fixed for the life of a built scheduler, or can it be replaced between passes (as a value is swapped with `replace_value`)? Kaski's world grows, loads tiles and has births and deaths.
  ```

Options offered:

1. Replaceable between passes

   The extent is a value the consumer may replace between passes up to a capacity fixed at build; the plan is built for the capacity and passes run over the current extent.

2. Fixed at build

   The extent is fixed when the scheduler is built; a world that grows rebuilds the scheduler.

3. Both, declared per domain

   A domain declares in its type whether its extent is fixed or replaceable up to a capacity; fixed domains let the planner assume more.

Chosen:

- Both, declared per domain

## Write order, last frame, counts and geometry, 08:13

Copied from `.data/op-responses/202611130808_read-the-whole-hilavitkutin-spec-and-port-it-to-a-registry.md`, lines 88 to 188.

### Asked, answered at 08:13

#### Write order

  ```text
  hilavitkutin: the founding spec and the seed say two writers of one store are ordered by hint priority, then by most downstream dependents, then a fixed fallback. The engine's design template and the code order them by registration position alone and read no hints. Which is the design?
  ```

Options offered:

1. Hints first, as founded

   Hint priority, then most dependents, then a fixed fallback. The code's registration-position order is a deviation to fix, and the engine design line is corrected.

2. Hints, registration as fallback

   Hint priority, then most dependents, with registration position named as the fixed deterministic fallback.

3. Registration position

   Registration position is the rule, as the engine design and code have it; the founding rule is retired.

Chosen:

- Hints, registration as fallback

#### Last frame

  ```text
  hilavitkutin: a unit needs the value a store held at the start of the frame while the same or another unit produces the next value (kaski's fields step this way). The record is silent on it. How should hilavitkutin say that?
  ```

Options offered:

1. Read marker, engine buffers

   An ordinary column plus a read marker meaning 'as of frame start'; the engine derives the buffering from the access declarations. Writers stay ordinary writers, ordered like any others.

2. Two-sided store, meta swap

   One store kind with a committed and a staged side, swapped by a meta unit on the scheduler's own pipeline; its writers are ordered like any writers.

3. Two stores and a carry unit

   Two ordinary stores and a unit on the frame-end meta band that carries next to current. Nothing new in the api.

4. As the revision wrote

   A kind with a committed and staged side, one writer only, swapped by the engine outside the meta band.

Answered in their own words:

> I am almost 100% we have these counters/increments already designed exactly for something like this. Hilavitkutin has a spec and known downstream users (not us at least not yet) that do triple buffering even and the spec should answer that already at least in intent… have you not read the whole thing? Perhaps you better increase your efforts on porting the canon and design in hila to a registry so canon tools and such can be used on it, as unearth and already said too and whatever else we are upstrwaming

#### Counts

  ```text
  hilavitkutin: columns in one scheduler may need different record counts (kaski: tiles, beings, items). Today there is one record count. Where does the count live?
  ```

Options offered:

1. On the column in storage

   Each column carries its count in storage; a fiber takes the count of its columns, and a unit over columns of unequal count is refused at build.

2. On the column's type

   A group parameter on the column type names which record set it belongs to; columns of one group share a count.

3. Consumer wrappers name groups

   The engine sees per-column counts only; grouping columns into record sets is a consumer-side wrapper.

Answered in their own words:

> Okay so i am fairway confident this is also answered. Isn’t the intent a contiguous per prototype and indices to process needed windows to it? Pretty sure the design has answers to these things and more

#### Geometry

  ```text
  hilavitkutin: does it own geometry (axes, neighbour offsets, the value past an edge)? Your text says it 'does not own the data model'. Kaski's field kernels read neighbours east, north and up.
  ```

Options offered:

1. No, only a declared reach

   The engine knows only a declared cross-record read with a reach in records, so it can plan and fuse around it; axes, offsets and edge values are the consumer's (kaski's) to map onto record positions.

2. Opt-in crate over the api

   Geometry lives in an opt-in hilavitkutin crate built on the api's declared cross-record read; the core api stays free of it.

3. Yes, in the api

   Axes, offsets and boundary values are api concepts.

Chosen:

- Opt-in crate over the api
