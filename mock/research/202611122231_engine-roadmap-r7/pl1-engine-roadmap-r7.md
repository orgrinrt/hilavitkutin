Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/pl1-engine-roadmap-r7.md, first posted 2026-11-12 22:31 UTC, read 2026-11-15.

# Engine roadmap r7: the contracts are hoisted, and a loop runs before anything is tuned

Date: 2026-11-12. Roadmap step PL1, revised the same night after two outside readers (`stamps-2-spj.md`, `stamps-2-torvalds.md`) read it blind of each other. Lands in hilavitkutin as `mock/research/202611122230_engine-roadmap-r7.md` on `work/roadmap-r7`. Read at hilavitkutin `af823b7`, arvo `d841c5e`, notko `0a13d99`, kaski `0905ce823`, all `dev`; every kaski path below is a path at that kaski commit and dies at kaski's regroup. Nothing was built or run. Every signature in this note is a sketch of a shape and is labelled so; none was compiled. The note is revised once more, when the sketches HV2 (kernels that are handed things) and HV3 (the frame view as types) report.

This note succeeds `202607181300_engine-roadmap-r6.md`. r6 ordered the engine's internals: per-fiber morsels, then the parallel model, the plan chain, dispatch order, adapt, and only then the consumer surfaces. The lead designer's words of 12 November turn that order round for the part a consumer writes against. Things may stay unoptimal, fusion or threading may be missing, as long as the contracts make it so that the later work demands no change on implementors and happens in the plumbing. So the contracts come first, in new crates beside the engine, with a one-thread reference runner under them that is kept as the oracle, and the first thing built is a loop that runs end to end.

What this note keeps of r6, what it hoists and what it leaves alone is said once here. It keeps r6's phase order A, G2C, B to H as the order of the existing engine's own internals, its evidence ledger (no leaf starts without an assigned resolution) and its sketch plan. Under stamp 14 those phases are frozen as they stand: none of r6's unshipped leaves starts until the bench `runner-arms` has the reference runner as an arm and shows that the leaf pays (roadmap step HV18). The fixes of roadmap steps HV1a and HV1b land on the existing engine now and are no r6 leaf. It hoists, ahead of every planner phase and into new crates: record domains, the two regimes of a frame, the committed view and the staged columns, emission with its order and merge law, change sets, identity, the read through a reference, and the close's order. r6 held the consumer surfaces in Phase E (E1 to E5, undecomposed) and change detection in D-gen and G2C-5; those leaves are not started on the engine while lane V designs the same forms, and they return at HV18 stated against the contracts. It leaves unratified the supersession r6 records at its lines 177 to 179, the N-way record-order partition in place of head and tail convergence, which r6 itself calls the agent's call and flags for the lead designer's review. Nothing in this note depends on that supersession, and G2C-0's deletions do not run under this note; the ruling is his.

hilavitkutin declares no canon (no `canon_paths` in its `mockspace.toml`), so its designs govern: `mock/DESIGN.md.tmpl` and `mock/crates/hilavitkutin-api/DESIGN.md.tmpl`. This note is a research note and supersedes no design sentence. Where the new crates say something the designs do not, or the opposite of what they say, the design round that births the crates has to say so by name; section 9 lists each such sentence for HL0a's doc changelist. Three design sentences shape what follows and are cited where they bite: every crate is `no_std` with no alloc (`mock/DESIGN.md.tmpl:138`, held by the forbidden-import row at `mockspace.toml:139`, whose scope `hilavitkutin*` covers the new crates by name); monomorphisation is the dispatch, no `dyn` and no `TypeId` (`mock/DESIGN.md.tmpl:139`); and there are three store types (`hilavitkutin-api/DESIGN.md.tmpl:140-148`). The vocabulary line at `mock/DESIGN.md.tmpl:114` retires "row" and "entry" in favour of "record" and "partition" in favour of "phase". So this note says record where the roadmap says row, "logged form" where it says journal entry, "member" for an item of a read list (the first page said "entry"), and "morsel split" where the first page's law B2 said "partition". A line of a content table that binds a kernel is a "binding".

Decisions that are the lead designer's are written for the default the roadmap takes, and each place where another answer would change the page is marked "Call n". Each place the revision changed says what the page said before and which reader asked.

## 1. Crates and module paths at the birth

Five crates are born by HL0a. Names are working names until the round's topic settles them; the default of Call 2 (a new crate beside `hilavitkutin-api`, taking that name at the port) is what the paths are chosen for. No new crate depends on `hilavitkutin-api` or `hilavitkutin`. From HL0a's landing until the port (HV18) the repository holds two contract crates; the legacy one says so in its own documents from that landing (section 8, HL0a item 10).

| Crate | Depends on | Holds |
|---|---|---|
| `hilavitkutin-contracts` | `notko`, `notko-hlist` | every trait an author or an implementor writes against, the plain data that crosses a border, and the checked binding table |
| `hilavitkutin-reference` | contracts; laws for its tests | the one-thread runner, mode Full; its test support; the mutant tests |
| `hilavitkutin-laws` | contracts | laws as functions over plain data returning a verdict; imports no runner and names no support trait |
| `hilavitkutin-frozen-authors` | contracts only | one file per author, each with its model |
| `hilavitkutin-frozen-implementors` | contracts only | one file per implementor |

Module paths of `hilavitkutin_contracts`, as HL0a to HL0c need them and no further. Taking the name `hilavitkutin-api` later is a rename of the crate; no path below names the crate inside itself.

| Path | Born | Items |
|---|---|---|
| `list` | HL0a | re-exports of `notko_hlist::{Empty, Cons, List, Here, There}`; `Select`, `Values` (local until notko step NK1 lands them, marked for removal) |
| `num` | HL0a | the proxy module: `USize`, `Bool`, `Lane`, `Identity`, `Additive`, by the names of `arvo/mock/research/202611122230_value-names.md`; and the contract crate's own `Index` and `Count`, two distinct private-field types over `USize` |
| `domain` | HL0a | `Domain`, `Column`, `Resource`, `Value`, `Cell`, the read-list members `Own`, `Near`, `Param` |
| `kernel` | HL0a | `ValueKernel`, `KernelId`, `Kernels` |
| `bind` | HL0a | `UnitId`, `Binding`, `BindRefusal`, the checked table `Bound`, the typed column handle `Col` |
| `store` | HL0a | implementor-facing: `Store`, `Read`, `Write`, `Visit`, `Params` |
| `digest` | HL0a | `Digest`, `Absorb` |
| `emit` | HL0b | `Kind`, `Band`, `Merge`, `Ordered`, `Emitter`, `OrderKey`, `Logged`, `Refusal`, the re-export of `notko::Push`; implementor-facing `Window` |
| `journal` | HL0c | `FORMAT` and nothing else; a logged form is already plain data in `emit` |

A "column" here is what the roadmap calls a lane of a domain. The word `Lane` is kept for the arithmetic vocabulary of the proxy, as kaski and the arvo name page use it, so the two never share a name.

`Index` and `Count`. What the first page said: both were aliases of the proxy `USize`. Both readers refused that as a default (SPJ, P0-1; Torvalds, section 4): a count passed where an index goes compiles in every implementor, and an alias turned into a distinct type later edits every frozen implementor, while the reverse edits nobody. They are two distinct types from birth. SPJ asked for a third, a column index; the typed handle `Col` of 2.1 is that type.

## 2. The trait shapes, as sketches

Bounds written `Copy + Send + Sync + 'static` are provisional: the bound list HV2 reports for its fused and threaded runners replaces them before HL0a starts, since a bound added to an author-facing associated type later is an edit to every author.

### 2.1 HL0a, the pure core

```
// domain: what an author names. Sketch.
pub trait Domain: 'static {
    type Key: Copy + Ord + Send + Sync + 'static;     // the canonical record key
    type Step: Copy + Send + Sync + 'static;          // an offset between keys, one component per axis; plain data
}
pub trait Cell: Copy + Send + Sync + 'static { /* sealed: one of a closed set of fixed widths, read from and
                                                  written to little-endian bytes by safe code */ }
pub trait Value: Copy + Absorb + Send + Sync + 'static {     // what a column holds: a format's value
    type Cell: Cell;                                  // its stored form
    const FORMAT: FormatId;                           // plain data; what the bind compares
    fn from_cell(cell: Self::Cell) -> Self;           // total: every bit pattern names a value
    fn cell(self) -> Self::Cell;
}
pub trait Column: 'static { type Domain: Domain; type Value: Value; const OUTSIDE: Self::Value; }
pub trait Resource: 'static { type Value: Value; }

pub struct Own<C>(..);                    // this record's value of column C
pub struct Near<C, O>(..);                // the value at offset O from this record's key; O in one of two spellings, below
pub struct Param<R>(..);                  // a resource's value, constant over the step
// Values: the projection from a list of members to a nested pair of their values,
// (A, (B, ())), which an author destructures and never names by position.
```

The neighbour member. What the first page said: `Near<C, const D: i32>`, "the value D records away in the domain's key order", locked at HL0c. Both readers found it one-dimensional and locked before any author with two axes existed (SPJ, P1-6 and correction C1; Torvalds, section 4). At head the shipped carry reads at `(east, north, up)` (`kaski-engine-lattice/src/tiles/block.rs:240-242`) and the diffusion folds over `sides` (`:250-255`), and a neighbour one line up in a plane is no constant distance in key order when the extent is a run-time value. So an offset is per axis from birth, the first author reads on more than one axis, and the member's spelling is carried as two candidates until HV2 reports its kernel (b) on a domain of at least two axes (correction C1). Neither reader designed the replacement, and SPJ says it should come from HV2 and not from this page, so the candidates are only that:

- N-const: `Near<C, const E: i32, const N: i32, const U: i32>`, three axes fixed in the contract, a domain with fewer refusing a non-zero component at build.
- N-type: `Near<C, O>` with `O: Offset<C::Domain>`, an offset type the domain's author declares, carrying its per-axis components as associated constants.

Under both, the store side takes the offset as plain data (`Read::near` below), so no const parameter sits on a trait method. A data-dependent offset inside a constant reach, as the shipped carry's gather has, is the window kernel's and arrives with V3. `Column::OUTSIDE` stays as the first boundary policy and is not a lock (Torvalds: it "hardcodes one boundary policy into the author's type"); section 5 says when it may lock.

```
// kernel: the author's form. Receiver-less, handed values, returns values. Sketch.
pub trait ValueKernel: 'static {
    type Domain: Domain;
    type Reads: List;       // Own, Near, Param members; the one read list
    type Writes: List;      // Own members of the same domain
    const ID: KernelId;
    fn at(reads: <Self::Reads as Values>::Of) -> <Self::Writes as Values>::Of;
}
```

The output type is computed from the declared write list, so a fallible return has no spelling. A column named in a kernel is a role: a formal parameter with a format. Which stored column fills the role is data, supplied by a binding (Call 7, default a). No frozen author names a channel.

```
// bind: a unit is a kernel under one binding. Sketch.
pub struct UnitId(..);                    // declared in the binding, plain data, private field
pub struct Binding<K: ValueKernel> { /* unit: UnitId; a stored-column number and a FormatId per role; a value per Param */ }
pub enum BindRefusal { DuplicateUnit, FormatMismatch, TwoWritersOfOneColumn, UnknownKernel }

pub struct Bound<..>(..);                 // the checked binding table. Abstract: no public field, no public constructor
pub fn bind(/* the bindings as data; the store's declared column formats */) -> Outcome<Bound<..>, BindRefusal>;
pub struct Col<D: Domain, V: Value>(..);  // a typed column handle. Private field; minted only from a Bound,
                                          // for a role whose FormatId the bind compared with V::FORMAT
```

The checked table is a witness. What the first page said: the bind returned refusals as values and each runner was left to rely on the result; the store's read took a bare column number. SPJ (P1-5, correction C5) asked that the refusal produce a witness, because a threaded runner that hands out staged columns relies on disjointness for memory safety and an invariant re-checked per runner has as many copies as runners. Torvalds (section 4, the HV3 correction) asked for a typed column handle minted at bind, because a store asked for any value type at a run-time column number must either check the format on every read or transmute on trust. The two fixes are one mechanism: `bind` is the only constructor of `Bound`, every runner takes a `Bound`, and the only source of a `Col<D, V>` and of a staged handle is a `Bound`. The format check and the two-writers check then happen once, in the contract crate.

```
// store: implementor-facing pieces the reference runner is assembled from. Sketch.
pub trait Store<D: Domain> {
    type Committed<'step>: Read<D> where Self: 'step;    // shared, lives for the step
    type Staged<'step>: Write<D> where Self: 'step;      // exclusive, has no read method
    fn extent(&self) -> Count;                           // run-time, owned by the store
    fn formats(&self /* , out */);                       // the FormatId of each stored column, for the bind
    fn step(&mut self) -> (Self::Committed<'_>, Self::Staged<'_>);
    fn swap(&mut self);                                  // the close only
}
pub trait Read<D: Domain> {
    fn key(&self, at: Index) -> D::Key;
    fn get<V: Value>(&self, column: Col<D, V>, at: Index) -> V;
    fn near(&self, at: Index, step: D::Step) -> Maybe<Index>;      // by key, never by position
}
pub trait Write<D: Domain> { fn put<V: Value>(&mut self, column: Col<D, V>, at: Index, v: V); }
pub trait Visit { /* yields morsels of positions for an extent and a morsel size, in its own order */ }
```

`Read::get`, restated so that it can be implemented. What the first page said: `get<V: Value>(&self, column: Index, at: Index) -> V` with `Value: Copy + Absorb + Send + Sync + 'static`. SPJ: nothing in that bound produces a `V`, since `Absorb` only consumes, so no store can implement it without a transmute or a `TypeId`, and `mock/DESIGN.md.tmpl:139` forbids the second. Torvalds: the format fact the bind checked is thrown away at the one place it is needed. The form above is implementable by safe code, as a prediction and not a compiled fact: a store holds each stored column as cells of a fixed width; `get` finds the column by the handle's number, reads `V::Cell` from its bytes by the sealed `Cell` trait's safe read, and returns `V::from_cell`. The store checks no format, because a `Col<D, V>` exists only for a column whose format the bind compared with `V::FORMAT`, and it trusts nothing through `unsafe`, because `from_cell` is total: a wrong cell would be a wrong value and never undefined behaviour. What remains at run time is the position's bound, which is not a format check and is the block kernel's and HV15's to remove. The alternative, if HV3 reports that the cell form does not build or costs a read it should not: the store is generic over the closed set of formats and the handle selects a typed column statically. Under both, `get` takes the handle, and under both the same pair of functions gives `Absorb` its bytes.

Under HV3's typed form `Committed` and `Staged` are the two regimes as types. Positions (`Index`) exist only on this side; no author form has a parameter through which one arrives.

```
// digest. Sketch.
pub struct Digest(..);
pub trait Absorb { fn absorb(self, into: &mut Digest); }   // fixed-width little-endian bytes, the same on every target
```

The digest is a fold over values in key order, column by column in binding order, by the function the sketch WD1 names in its findings. A count or an index is absorbed as 64 bits whatever the pointer width; a proxy number as its slot index in 16 bytes. The encoding is unlocked until the journal format locks.

The reference runner's entry is a library function in the front-end shape WD1 reports, a sketch: `run(bound, store, visit, morsel: Count, frames: Count, out: &mut Run)`, where `bound` is the checked table and `Run` holds a fixed-capacity digest list and, from HL0b, the log. It is never the body of a test. When a fixed list of `Run` is full the run returns a refusal as a value, never a shortened list (SPJ, correction C6).

### 2.2 HL0b, emission and the close

```
// emit. Sketch.
pub struct Band(..);
pub trait Merge {}                         // how forms of one kind from many morsels become one sequence
pub struct Ordered;                        // by order key; the only inhabitant at HL0b
pub trait Kind: Copy + Absorb + Send + Sync + 'static {
    type Target: Column;                   // first inhabitant: one target, one value. Not a lock; see below
    type Merge: Merge;
    const BAND: Band;
    fn land(held: <Self::Target as Column>::Value, form: Self) -> <Self::Target as Column>::Value;
}
pub struct OrderKey<K> { pub band: Band, pub unit: UnitId, pub key: K, pub seq: Count }   // ordered in this field order
pub struct Logged<F, K> { pub at: OrderKey<K>, pub frame: Count, pub form: F }
pub struct Refusal { pub band: Band, pub wanted: Count, pub had: Count }
pub trait Window<T>: Push<T> {             // implementor-facing; exclusive
    fn ceiling(&self) -> Count;
    fn refusal(&self) -> Maybe<Refusal>;   // sticky: once Just, every later push is counted and dropped
}
```

The order key is total on one condition, stated here because SPJ asked for it (P1-8): `seq` is assigned inside one invocation of one kernel on one record, counting that invocation's pushes from zero.

`Kind` has one target at HL0b and that is the first inhabitant, not the form's final arity. What the first page said: `Kind` with its band, `Ordered` and `land` locked at HL0c. Both readers refused the lock (SPJ, P1-6; Torvalds, section 4). At head kaski's shipped forms are two-target: the tally form "Moves an amount of one kind of good between two accounts" with `from` and `to` (`kaski-engine-contracts/src/mutation.rs:78-83`) and `Relocate` has `from` and `to` (`:127-130`). Torvalds adds the reason one target cannot simply be doubled: a debit that saturates while its credit does not breaks conservation if the two are independent forms. So the placement is kept (a declared fold on the kind, applied by the plumbing in order-key order, no landing code in a runner; both readers call the placement right) and the arity is open until an author of a two-target kind exists (V4).

The emitter is a value kernel that is also handed one emission handle. It is the first form that is handed a thing, so its shape is carried as two forks until HV2 reports (answer 1):

```
// fork E2, one small trait per handed thing; no runner trait in the contract. Sketch.
pub trait Emitter: 'static {
    type Domain: Domain; type Reads: List; type Writes: List; type Kind: Kind;
    const ID: KernelId;
    fn at(reads: <Self::Reads as Values>::Of, out: &mut impl Push<Self::Kind>) -> <Self::Writes as Values>::Of;
}
// fork E1, one runner trait whose associated context supplies every handle. Sketch.
pub trait Emitter: 'static {
    /* the same associated items */
    fn at<R: Runner>(reads: <Self::Reads as Values>::Of, ctx: &mut R::Ctx<'_, Self>) -> <Self::Writes as Values>::Of;
}
```

The handle under E2 is `notko::Push` (`notko/src/sink.rs:35-39`; "Infallible: overflow is the implementor's problem", `:28`): exclusive and infallible to the caller. An author cannot observe a refusal and cannot cause a different log by looking for one. The plumbing attaches the order key: the author supplies the form and nothing else. At HL0b a form lands on the emitting record's own key in the kind's target column; a target reached through a reference arrives with V4.

Which records an emitter runs on is final at HL0b. An emitter is an event on the change set of its declared reads (stamp 18), and the reference runner in mode Full defines that set by brute force: it keeps the committed values of the frame before and an emitter runs at a record exactly when a value among its declared reads at that record differs from the frame before. At the first frame every record counts as changed. The granularity at HL0b is one record. V1 then adds a mode, Tracked, that must equal this; it adds no meaning. This consumes Call 4 one landing earlier than the roadmap's wave table says (SPJ, call 4): HL0b builds on default a, "differs". If he answers b, "written", the Full definition, the models of B4 to B6 and V1 are restated. To keep that small, "changed" is a declared predicate of the contract with one inhabitant built, `Differs` at a stated granularity; "written" would be a second inhabitant and never a rewrite, and the plumbing has no discretion under either.

What this was and why it changed. The first page left the record set undefined until V1 and chose frozen author 2 as an edge-triggered emitter "So that V1 does not edit frozen author 2". Both readers refused that as a fixture picked so the first measurement of the lead designer's property cannot come out badly (SPJ, P1-7: "the setup helping"; Torvalds: "true by construction, and worthless"). The page no longer chooses an author for what V1 cannot do to it. Two emitter authors land at HL0b, one level-triggered and one edge-triggered through a state column of its own, both readers asking for the pair; V1's reading (how many edits to existing authors and implementors it needed) is taken over both and reported whatever it is.

The close, in the reference runner: merge every window by order key, land each form by `Kind::land` in that order, swap staged for committed, digest, seal. Its entry returns `Outcome<Seal, Refusal>` (the existing design returns a `notko::Outcome` from `run`, `mock/DESIGN.md.tmpl:42`). On a refusal the frame fails closed: nothing lands, nothing swaps, committed bytes are untouched. The window grows to the ceiling its provider declares and then refuses (stamp 6). In a `no_std`, no-alloc crate "grows" means it takes further lent chunks (`notko::Fill` over `Lend`, `notko/src/lend.rs:40`, `:93`) up to that ceiling.

### 2.3 HL0c, the loop closes

`journal::FORMAT` is a constant that reads "unfrozen" (stamp 12); a logged form and its order key are already plain data. Everything else HL0c adds is test support of the reference crate, under `hilavitkutin_reference::support`, a sketch: `Journal` (a fixed-capacity list of logged forms with a declared ceiling, which at its ceiling refuses as a value and never truncates; SPJ, P1-1: a model that truncates makes B8 and B9 pass on a shortened log), `Keyframe` (a plain copy of every committed column, every resource value, the frame number and the journal position), and three pluggable pieces local to that module, `Keyframer`, `Restorer` and `Applier`, so that the mutants of B7 to B9 are faulty pieces under an unmodified loop. Those three traits are named by the reference crate and its tests only. No law names them: section 4 says what a law takes. Host input is a list of forms per frame under the reserved `UnitId::HOST`, journaled like any other. Replay by re-execution restores a keyframe, feeds the recorded host forms and runs every kernel. Replay by application restores a keyframe and lands the journal's forms without running an emitter; value kernels are still stepped.

## 3. The six answers

1. The encoding of handed things. Carried as the two forks E1 and E2 of 2.2 until HV2 reports; HL0b does not start before the revision of this note picks one. No design sentence decides. HV2's growth probe is the evidence: the number of existing runner crates edited when a capability is added. The value kernel of HL0a is the same text under both forks, since it is handed nothing.

2. Where per-binding coefficients enter a receiver-less kernel, and which store kind that is. They are `Param<R>` members of the one read list, handed by value with the other reads. The store kind is the existing `Resource`. It rests on `hilavitkutin-api/DESIGN.md.tmpl:300-306`: a resource "holds persistent, infrequently-written state", has "cardinality one or more", and is "staged stack-local before the morsel loop", which is the constant-over-the-step read a coefficient needs. The binding supplies the value per unit. Both readers call this sound. Call 7: under answer b (channels are engine-fixed types) the binding becomes a type and the member is unchanged.

3. The type of a unit's declared identity, and the refusal of a collision. Two identities exist once a kernel is bound by data. `KernelId` is an associated constant of the kernel, declared by its author; two kernels with one id in a registered list `Kernels` are refused at build. `UnitId` is declared in the binding, is the "unit identity" of the order key, and a second binding with the same `UnitId` is refused at bind as the value `BindRefusal::DuplicateUnit`, before any frame runs. The existing design says the opposite of both, "No NAME const. WU identity is the type itself." (`hilavitkutin-api/DESIGN.md.tmpl:359`); section 9 lists it for HL0a's doc changelist. What is decided here is only that a kernel and a unit each have a declared identity. The representation is not: the first page's mask (bit `ID` of a 128 bit word per kernel, disjointness asserted in an inline `const` block where the list is instantiated; predicted class E0080; after the precedent for dense ids at `hilavitkutin-api/DESIGN.md.tmpl:154-157`) stays as the first sketch of the refusal and is marked unlocked, because both readers found that a ceiling of 128 kernels becomes an interface the day content names a kernel by it (SPJ, P1-11; Torvalds, section 4). Call 7: under answer b the unit collision moves to the build-time assertion as well.

4. The landing law of the first event kind. A declared fold on the kind, `Kind::land`, applied by the plumbing in order-key order; no landing code lives in a runner. No hilavitkutin design sentence decides. It rests on the lead designer's property and on the stamps' shaping rule: if the law were code in the runner, a consumer adding a kind would edit the plumbing, and a mutant of the law could not be a faulty implementor under an unmodified runner. One function serves the close and replay by application, so B9 cannot drift from the close.

5. The model journal, the keyframe and the restore are test support of the reference crate and no contract. Nothing lands upstream as a store kind of kaski's, so `kaski/mock/registry/tenet/engine.toml:669-670` is not touched. The contract carries only the logged form, the order key and the format constant. This holds only because the laws B7 to B9 are functions over plain data (section 4): were they written against `Keyframer`, `Restorer` and `Applier`, kaski could not run them on its own journal without implementing the reference crate's test support, which would make that support a contract by the back door (Torvalds, section 4). One correction to the roadmap follows from `mockspace.toml:139`: the model journal cannot be a growable vector, because the reference crate is named `hilavitkutin-*` and the row forbids `alloc` there. It is fixed-capacity with a declared ceiling, and of WD1's two routes only the fixed-capacity one is open unless that row is changed.

6. The ledger rule: an existing frozen file is edited before its form locks only with a row in `BREAKS.toml`; after the lock it is not edited. Written out in section 6. It is the default of Call 12, which both readers answer b; no design sentence decides.

Store kinds. A column of a domain is `Column`. A parameter is `Resource`. `Virtual` is used by nothing in HL0. Two things are classified by no text of this note and must be classified by no design text of the new crates until the lead designer says:

- What an emitted kind's log is. What the first page said: "read here as a column whose records only the plumbing appends". SPJ (P1-4) and Torvalds (section 4) both say not to decide it, and both name it the lead designer's. The design's sentence is "Three store types, one scheduling mechanism" (`hilavitkutin-api/DESIGN.md.tmpl:140`); the same design gives events to the third kind and not the second ("`Virtual<T>` ... zero-data, DAG edge only", `:147`; "`On<V>`: when virtual V fires", `:364`); the code at head has a fourth marker the sentence does not list, `Accum<T>` (`hilavitkutin-api/src/store.rs:119`), and the root design already speaks of "accumulator columns" (`mock/DESIGN.md.tmpl:94`); and the workspace's own statement is that state "reduces to exactly three things" (the workspace rule `hilavitkutin-workunit-mental-model`). The answers a reader can pick from: the log is a column only the plumbing appends; it is a virtual that carries a payload; it is a fourth kind, and the three-type sentence and the workspace statement change. Until he picks, the log is plumbing-owned state that no author names, like V1's change set: an author sees a push handle and never the log. The sentence that must change under the third answer is `:140` (with `:144-148` beside it), and the step that changes it is the doc changelist of the first design round after his answer, HL0b's if the answer is in hand by then. HL0a's and HL0b's doc changelists write no classification of the log.
- What identity is (V4). SPJ reads it as a domain whose keys the plumbing allocates, so a column (P1-3, "sound"); Torvalds says it may be a fourth kind and that the "exactly three" sentence is the lead designer's to change, "a call at V4, not tonight". One reader each way, so section 5 no longer classifies it. Same sentence, `:140`; the step is V4's doc changelist.

## 4. The nine birth laws, each with its mutant

A law is a function in `hilavitkutin-laws` over plain data that returns a verdict naming the law and a witness. Plain data here is what the contract crate already declares: digests, logged forms, counts, keys, and copies of column cells. No law takes a runner, a store or a support trait as a parameter. What the first page said: laws were functions in a crate that "imports no runner" while their mutants were "stores in the laws crate's tests" and B7 to B9 were stated over the support pieces. Torvalds found the two do not fit: a test that plugs a faulty store into the runner needs the runner. So the mutant tests live in the reference crate's tests, where the runner is; each plugs a faulty implementor, or for B7 to B9 a faulty support piece, into the unmodified reference runner, hands the runs' plain outputs to the law, and asserts the failing verdict. `#[should_panic]` is refused as a mutant test. Cross-target equality is an axis of the matrix and no law.

| Law | Born | The function takes | Statement | Mutant it must kill |
|---|---|---|---|---|
| B1 model | HL0a | the runner's columns per frame; the model's | equal on every input stream | `store_skips_last_record`: a store that skips the last record of a morsel |
| B2 morsel split and order | HL0a | digest streams of the runs of the matrix | equal across morsel size 1, a prime, whole; forward and seeded-permuted visiting; both layouts | `store_updates_in_place`: a store whose staged column is its committed one |
| B3 frame isolation | HL0a | committed cells at a step's start and at its end | equal, with staged storage poisoned before the step | `store_hands_staged_as_read`: a store that serves reads from the staged column |
| B4 order | HL0b | a log | ordered by (band, unit, record key, sequence), under every axis of B2 | `merge_keeps_visiting_order` |
| B5 multiplicity | HL0b | a log; the model's log | two identical forms on one target land twice | `merge_dedupes_by_target` |
| B6 refusal | HL0b | committed cells before and after; the returned outcome; the count of pushes after the refusal | past the ceiling the frame is refused, the refusal is a value and sticky, committed cells are untouched | `window_truncates`; `window_forgets_refusal` |
| B7 restore | HL0c | two digest streams and two logs: the uninterrupted run's, and for one keyframe the restored and continued run's; the keyframe's frame | equal from that frame on, for every keyframe | `keyframe_drops_pending`: a keyframer that omits the frame number and journal position |
| B8 re-execution | HL0c | the journal's log; the log of the replay by re-running | equal | `visit_repeats_a_record`: an emitter run on one record too many |
| B9 application | HL0c | the original digest stream; the stream of the replay by application; the count of emitter invocations in that replay | the streams are equal and the count is zero | `applier_runs_emitters` |

B7's mutant is only killable if the pending state reaches an output, so the digest absorbs the frame number and a logged form carries its frame. Stated this way, a consumer runs B7 to B9 on its own journal by producing two digest streams and two logs, and implements nothing of the reference crate.

## 5. Lane V: the order, the form each slice adds, and what locks when

Each slice lands the contract shape, the reference runner's implementation, one frozen author of the form with its model, and a test in Full and, from V1, Tracked.

| Slice | Form added | Store kind | Needs |
|---|---|---|---|
| V1 | a change set per domain, written by the plumbing at the close; mode Tracked, equal to what Full already defines as changed (2.2); the law stated at two granularities so a runner that hardcodes one is caught (Torvalds, call 4); the granularity a constant of the contract | none an author sees: plumbing-owned state | HL0c; Call 4 already consumed at HL0b |
| V2 | the order licence for a commutative kind, a second `Merge` inhabitant declared with a monoid; skewed emitters; extents that change between frames | unclassified, as the emitted kind's log (section 3) | V1 |
| V3 | the window kernel (const reach, access by value, the neighbourhood's final vocabulary); the fold kernel with a declared total monoid, which never accumulates in `Lane::Wide` (the arvo name page, form D item 4); the block kernel, the only form that sees a range | `Column` for all three | V2; sketch HV2b for the block kernel |
| V4 | identity; births and deaths landing at the close; the read through a reference, a fourth member naming the reference column and the target column; the wake rule; the first two-target kind with an author | identity is unclassified until his answer (section 3) | V3; reads arvo step AR8; Call 8 (default a, no copy) |
| V5 | the close's order from declared sets; `Clock` reachable from no kernel; `Park` as a capability a host may lack | none | V4 |

What locks, and at which landing. A lock is a line of the ledger turning `locked = true`; the landing named makes that commit.

What the first page said: at HL0c's landing it locked the value kernel, the read list with `Own`, `Near` and `Param`, the two regimes, `KernelId` and `UnitId`, and `Kind` with its band, `Ordered` and `land`; at V1 the emitter form and "changed"; at V3 the window and fold kernels; at V4 identity and the reference member; the rest at HV15. Both readers found the schedule too early (SPJ, call 12 and P1-6; Torvalds, call 12: "Locking before the first measurement means the measurement cannot report a failure"). The rule now: no form locks at HL0c, and a form locks no earlier than the landing of the first later slice that widens the plumbing under it, which is the first reading of the lead designer's property for that form, and only if that slice added no `BREAKS.toml` row for a file of the form. If it added one, the lock moves to the next slice that adds none.

| Form | Born | First measured by | Locks no earlier than |
|---|---|---|---|
| the value kernel; the read list with `Own` and `Param`; the two regimes; that a kernel and a unit each have a declared identity | HL0a | V1 (a mode added under them) | V1's landing |
| the emitter form; the definition of "changed" | HL0b | V1 (Tracked against Full, both emitter authors) | V1's landing |
| `Kind`: its band, the `Ordered` merge, the placement of `land` as a declared fold | HL0b | V2 (a second merge law) | V2's landing |
| the neighbour member in either spelling; `Column::OUTSIDE` | HL0a | V3 (the window kernel settles the neighbourhood's vocabulary) | V3's landing, with HV2's two-axis report in hand |
| `Kind`'s target arity and what a form may target | HL0b | V4 (targets through a reference, the first two-target kind) | V4's landing |
| the window kernel; the fold kernel | V3 | V4 | V4's landing |
| identity; the reference member | V4 | V5 | V5's landing |
| the close's order; `Clock`; `Park` | V5 | the plain threaded runner (HV16, check P12) | HV16's landing |
| the block kernel; storage addressing; the memory provider; the window's interior; change-set word ownership; the format a channel takes; the representation of `KernelId` and of `UnitId`; the cell encoding of `Value` and the representation of `Col`; the order key's representation, the digest encoding and `journal::FORMAT`, which lock together | as each is born | HV15's findings | roadmap step HV15, each on a committed findings file |

Line S5 of "sufficient shape" (no `BREAKS.toml` row from V4 and V5) is unchanged and stays the evidence. Under Call 12 answer b the later locks cost ledger rows and nothing else.

## 6. The ledger rule

Each frozen crate holds `LEDGER.toml` and `BREAKS.toml` at its root. A ledger line is one file: its path, the form it is an author or implementor of, the content hash, a count `breaks`, and `locked`. A `BREAKS.toml` row is one edit: the slice, the file, the form, the new hash, the reason.

- A new file is appended with its line; nothing else is needed.
- Before its form locks, an existing file is edited only in a commit that also updates its hash, raises `breaks` by one and adds the `BREAKS.toml` row.
- After the lock the file is not edited. A changed form arrives as a new form beside the old one, with new files.
- The check is a test in each frozen crate, `tests/ledger.rs`: it rebuilds each hash from the file, and is red on a mismatch, on a `breaks` count that differs from the number of rows for the file, and on a locked line whose `breaks` differs from the count recorded at its lock.
- Lane V appends the files its slice needs; lane W appends further files behind it. Neither edits a file the other wrote.
- "Sufficient shape" reads the file: no row from V4 or V5.

## 7. The two marked forks

Fork 1: HV2's kernels (b) or (c) fail under both arms. Then a kernel that is handed a thing is not portable across runners, HL0a does not start, and the consequence the first sitting named holds: the author surface is a kaski trait and hilavitkutin enters as one implementation of it. Of this note the following survive as written: the laws as verdict functions over plain data, the plain data of 2.2, the proxy module, the reference loop as the oracle. Sections 2.2's emitter, the lock table and the task lists are rewritten. HV2's kernel (b) runs on a domain with a key of at least two axes (correction C1); a WORKS on one axis does not let HL0a start.

Fork 2: HV3's exclusivity fixtures fail. Then `Committed` and `Staged` are two plain traits; each forbidden direction moves first to a `const` assertion and then to a debug-build checker in the reference runner (stamp 7's option c, never its option b), and the crate's documentation names the regimes unlocked. One part of this fork is already taken by Call 7's default: with columns bound by data, "two kernels of one step write the same stored column" cannot be a type error, so it is the bind-time value `BindRefusal::TwoWritersOfOneColumn` whatever HV3 reports, and the checked table `Bound` is its witness. HV3's fixture (6) applies to engine-fixed lists only; what the contract depends on is fixture (5) and the two pairs the readers added: a runner cannot obtain staged handles from an unchecked table (SPJ, C5), and a column handle minted for one value type is not read as another (Torvalds). The typed form applies to roles within one kernel, and to stored columns only under Call 7 answer b.

## 8. Task lists

Common to all three: repository `hilavitkutin`, base `dev`, one design round each (topic, doc changelist, lock, source changelist, lock, close), landed by a local merge and a push. Commands, from the repository root: `cargo mock check` (phase and lints), `cargo mock lock`, `cargo mock close`, `cargo mock test`. A compile-fail fixture is a pair: the failing file with its committed diagnostic and a passing twin that differs in the offending token only. Every new test is shown failing once against a named break. No `std`, no `alloc`, no `dyn`, no `generic_const_exprs`, no `specialization`; no feature line in a frozen crate.

### HL0a, branch `work/loop-core`

Needs: HV2 with kernels (b) and (c) working under an arm, (b) on a domain of at least two axes and reporting what the vocabulary needed to name an offset on an axis; HV3 reported, with the checked-table twin and the handle pair; WD1 reported; the arvo name page revised (it settles the form of the accumulator's term bound); this note revised; mockspace step MS0 merged (or: in the doc phase a new crate is a manifest and a `lib.rs` holding crate documentation only; post the refusing line if the lock is refused).

Create, in this order.
1. The five crates' manifests, `README.md.tmpl` and `DESIGN.md.tmpl`; the doc changelist carrying every line of section 9; rows in `mockspace.toml` under `[lints.forbidden-imports]`: the frozen crates import `hilavitkutin_contracts` only; laws import no runner; no new crate imports `hilavitkutin::*` or `hilavitkutin_api::*`. Bump notko in the lock if NK1 has landed.
2. `contracts`: `list`, `num` (against the name page; `Index` and `Count` are two distinct private-field types with `from_host` and `host`, no `raw`; `Lane` with its term bound), `domain` (with `Cell`, `Value`'s cell form, the neighbour member in the spelling HV2 picked), `digest`, `kernel`, `bind` (`bind` as the only constructor of `Bound`; `Col` minted from it), `store`.
3. `frozen-implementors`: store one (columns contiguous, forward `Visit`); store two (a different layout, a seeded-permutation `Visit`). Neither contains `unsafe` or a format comparison.
4. `frozen-authors`: author 1, a value kernel generic over `L: Lane` on a domain of at least two axes, reading `Own`, a neighbour on each side along each of two axes at the least (the shipped kernels read east, north and up) and one `Param`, using `widen`, `wide_add`, `wide_sub`, `narrow` only, its term count within the bound; its model, a plain function over arrays written without the contract.
5. `reference`: step, swap, digest; `run` as a library function taking a `Bound`.
6. `laws`: B1, B2, B3 as functions over plain data. The three mutants as stores in the reference crate's tests.
7. `LEDGER.toml`, `BREAKS.toml` (empty) and `tests/ledger.rs` in both frozen crates.
8. Lints under `mock/lints/` with rows at error: no `#![feature` line, no `recursion_limit`, and no `::<` on an item the file did not declare, in the frozen crates.
9. The cross-target command: WD1's script moved to `mock/tools/` and pointed at `run`.
10. The legacy contract crate marked: `hilavitkutin-api`'s `README.md.tmpl` and crate documentation say it is the legacy contract, superseded by the contracts crate and removed at the port (HV18), and a ticket owns the removal (Torvalds, call 2).

The two host doors of `num` are the only bare `usize` at a public position. `[lints.no-bare-numeric]` (`mockspace.toml:220`) is off at commit and at warn for build and push at head. If it reports them, post the line; the lint is not lowered and no exemption is added.

Checks: `cargo mock test` green with B1 to B3 over both stores, morsel sizes 1, a prime and whole, both visiting orders, each equal to the model; each mutant test asserting its failing verdict; the pairs under `tests/ui/` (an undeclared read; a kernel impl with a stricter bound; a kernel asking for its position or range, the twin being store-side code; a view kept past the step; two kernels with one `KernelId`; a `Count` passed where an `Index` goes; a `Col` minted for one value type read as another; a staged handle asked of an unchecked table); the bind refusals each returned as a value with no frame run and no `Bound` made; the surface fixture, one compile-pass file using every door and trait of `num`; `cargo mock check` green; the tool of item 9 exits zero.

Done means: landed on `dev`; digests equal on native and under node; the ledger test green; the sha posted. No emission, no journal. Nothing locks.

### HL0b, branch `work/loop-emission`

Needs: HL0a landed; this note's revision naming E1 or E2; Call 4 at its default a, or his answer.

Create, in this order.
1. `contracts::emit` in the chosen fork; the re-export of `notko::Push`; the predicate "changed" with its one inhabitant.
2. `frozen-implementors`: a window over lent chunks with a declared ceiling.
3. `reference`: the previous frame's committed values kept and the brute-force change set per emitter; per-morsel windows, the merge, the land, the close returning `Outcome<Seal, Refusal>`; the log added to `Run`.
4. `frozen-authors`: author 2, a level-triggered emitter (it emits at every record in its change set whose read is past the threshold); author 3, an edge-triggered emitter through a state column of its own; each with its model, the expected log as a plain function.
5. `laws`: B4, B5, B6 as functions over plain data; their four mutants in the reference crate's tests.

The doc changelist of this round writes no store-kind classification of an emitted kind's log (section 3), unless the lead designer's answer is in hand, in which case it carries the change to `hilavitkutin-api/DESIGN.md.tmpl:140` that the answer needs.

Checks: `cargo mock test` with B1 to B6 over the HL0a matrix, the log equal to each model's under every axis; a refusal test at a ceiling of one chunk showing committed cells untouched and the refusal returned; pairs: emitting a kind the emitter did not declare, a second merge law for one kind, a handle kept past the call; the ledger test green with no `BREAKS.toml` row, or each row explained in the commit; the cross-target tool comparing digests and the ordered log.

Done means: landed; the statement of the refusal law posted for notko step NK2. Nothing locks.

### HL0c, branch `work/loop-closes`

Needs: HL0b landed.

Create, in this order.
1. `contracts::journal::FORMAT`.
2. `reference::support`: `Journal` (refusing at its ceiling), `Keyframe`, `Keyframer`, `Restorer`, `Applier`, host input, both replays.
3. A binding table as data in the loop test: per binding a unit id, a kernel id, a stored-column number and a format per role, the coefficients; bound once by `bind`. Two bindings of author 1 with different coefficients, one of author 2 and one of author 3.
4. `laws`: B7, B8, B9 as functions over digest streams, logs and a count; their mutants as support pieces in the reference crate's tests.

Checks: `cargo mock test` with B1 to B9; the loop test (bind, step, emit, close, keyframe at every frame, restore, both replays) at two morsel sizes; the cross-target tool printing equal digest streams and equal logs on native and under node, which is line S1 of "sufficient shape"; `cargo mock check` green; a grep showing the laws crate names none of `Keyframer`, `Restorer`, `Applier`.

Done means: landed; the count of `BREAKS.toml` rows from HL0b and HL0c reported; lane W's first slice and V1 may start. No ledger line turns `locked = true` at this landing (the first page's item 5 did that; section 5 says why it went).

## 9. What HL0a's doc changelist must say

hilavitkutin's designs govern and this note supersedes none of them. The birth round's doc changelist carries each line below, by path and line as read at `af823b7`, saying that the sentence does not carry into the new crates, or what the new crates add beside it, and why. "Supersedes" here means for the five new crates only; the legacy crates keep their designs until the port.

Supersedes, for the new crates:

- `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:359-360`: "No NAME const. WU identity is the type itself. Naming is a consumer concern". The new crates give a kernel a declared `KernelId` and a unit a declared `UnitId`. Why: a kernel is bound by data, so one kernel type is many units, and an order key needs an identity that is the same across builds, which a type is not. (The code says the same as the design at `hilavitkutin-api/src/work_unit.rs:5`, and the legacy crate has its own `UnitId` with a public field at `src/dispatch_codegen.rs:191`; the new one shares the name and nothing else.)
- `hilavitkutin-api/DESIGN.md.tmpl:366-368`: "Write-write ambiguity resolved by scheduling hint priority across three dimensions. Tie-break: most deps first, then deterministic fallback." In the new crates two writers of one stored column in one step are refused at bind as `BindRefusal::TwoWritersOfOneColumn`, and no `Bound` is made. Why: in a two-regime step two writers of one staged column have no order a priority could resolve.
- `hilavitkutin-api/DESIGN.md.tmpl:322-337`, the `WorkUnit` trait, and `mock/DESIGN.md.tmpl:102-104`, the `Ctx` GAT: `fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>)`, `const COMMUTATIVE: bool = false`, `type Hint`. The value kernel has no receiver, is handed values and returns values, carries no `Ctx`, no hint and no `COMMUTATIVE` constant; commutativity is a merge law declared on a kind. Why: the lead designer's property; a context type is where a later capability becomes an edit to every author.
- `mock/DESIGN.md.tmpl:86`: "Per-morsel vs per-record WU dispatch inferred from AccessSet shape: column-shaped WUs receive a `Morsel<T>` window". No author form of the new crates but the block kernel (V3) is handed a window, a position or a range.

Adds to, without contradicting:

- `hilavitkutin-api/DESIGN.md.tmpl:140-148`: "Three store types, one scheduling mechanism" with `Resource<T>`, `Column<T>`, `Virtual<T>`. HL0a uses two of the three words: a column of a domain and a resource. It adds that a column named in a kernel is a role bound to a stored column by data. It says nothing on what an emitted kind's log or identity is, and it records that the code at head already holds a fourth marker the sentence does not list (`Accum<T>`, `hilavitkutin-api/src/store.rs:119`; "accumulator columns", `mock/DESIGN.md.tmpl:94`), a drift whose resolution is the lead designer's.
- `hilavitkutin-api/DESIGN.md.tmpl:300-308`: the resource sentence. Added: a per-binding coefficient is a resource value supplied by the binding and read as a `Param` member.
- `hilavitkutin-api/DESIGN.md.tmpl:150-157`: "The scheduler sees StoreId pairs for edge construction"; "`StoreId` is a dense index assigned at plan time"; `AccessMask` as `Mask64` or `Mask256`. Added: the new crates name a stored column by a typed handle minted by the bind, and the 128 bit word of the `KernelId` collision check is a stand-in for arvo's masks, marked so, its width no interface.
- `mock/DESIGN.md.tmpl:42`: the parameterless `run(&mut self) -> notko::Outcome<Cfg::Ok, Cfg::Err>`. Added: the reference runner's `run` is a library function over a checked table, a store and a visiting order; from HL0b the close returns `Outcome<Seal, Refusal>`.
- `mock/DESIGN.md.tmpl:114`: the vocabulary line. Added words: binding, role, logged form, member (of a read list), and `Lane` for the arithmetic vocabulary as distinct from a column. No retired word is used.
- `mock/DESIGN.md.tmpl:138-139`: principles 1 and 2. Kept without change and restated in each new crate's design; the model journal is fixed-capacity because of the first, and `Read::get` takes a typed handle because of the second.
- `mock/DESIGN.md.tmpl:12-34`: the crate layout table and the dependency graph. Added: five rows, the statement that no new crate depends on a legacy crate, and that `hilavitkutin-api` is the legacy contract until the port.

## 10. Raised by one reader

Applied above because it could be checked at head and costs nothing if wrong: the model journal refuses at its ceiling (SPJ); `seq` is assigned inside one invocation (SPJ); the mutant tests live with the runner (Torvalds); V1's law at two granularities (Torvalds); the legacy crate marked at HL0a (Torvalds); the bare-number lint note (SPJ); kaski paths read with their commit (Torvalds).

Not applied, with what would settle each:

- Torvalds would move every lock, the value kernel's included, to the "sufficient shape" checklist at the earliest; SPJ finds the value kernel, `Own`, `Param` and the two regimes sound to lock at HL0c if HV2's kernel (b) ran on two axes. Section 5 takes the rule the two share (no lock before the form's first measurement), which puts those at V1. Settled by V1's reading: if V1 adds a `BREAKS.toml` row for any of them, the rule already moves the lock on.
- Whether content names a kernel by a number or by an interned name (Torvalds, who calls it Call 7's sibling and the lead designer's). The representation of `KernelId` is unlocked until HV15, so nothing is built on either. Settled by a question ticket filed with Call 7.
- Whether the "changed" granularity belongs to the journal format as well as to the contract (Torvalds). The page puts the constant in the contract; the journal format is unfrozen until HV15. Settled when `journal::FORMAT` locks.
- SPJ's alternative for `Read::get`, a store generic per format, against Torvalds's typed handle over one store. The page takes the handle with a cell form and names the other as the fall-back. Settled by HV3's pair and its findings.
- That `Hash`-free `Absorb` is the only feed of a digest (SPJ, on the name page): stated on the name page, nothing to do here.
- HV16 as a duplicate mechanism (Torvalds checked and kept it), the two contract crates' end date behind six arvo crates (Torvalds), and the roadmap's wave-table row for Call 4 (SPJ): these are the roadmap's, not this note's. This note states its half: Call 4 is consumed at HL0b.

## 11. What this note could not establish

Whether any shape here compiles. Whether the `Values` projection, the cell form of `Value`, a `Col` minted generically over a kernel's read list, or the mask fold for `KernelId` resolve as sketched. Whether a `Col` or a staged handle can be tied to the one `Bound` that minted it (a brand); without one, a handle from another table reads a wrong value, never undefined behaviour, under the cell form. Which spelling of the neighbour member an author can write with no feature line and no turbofish; it comes from HV2. The right arity of `Kind` for a two-target form that conserves. Whether the forbidden-import row at `mockspace.toml:139` scans a crate's `tests/` (existing tests use `std`, `hilavitkutin/tests/test_executor.rs:68`), which matters more now that the mutant tests sit in the reference crate's tests. Whether `[lints.no-bare-numeric]` reports a door. What an emitted kind's log and identity are among the store kinds; both are the lead designer's. What the frozen implementors hold beyond a store, a visiting order and a window; it follows from each slice.
