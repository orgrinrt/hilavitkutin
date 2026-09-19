# Data Model: Columns, Values, Stores, Determinism

Data in a hilavitkutin app lives in columns and resources the scheduler owns.
This chapter states the column memory layout, the value-type contract, the
three store types, the storage contract, and the ordering model that falls
out of data access. The resource storage model has its own chapter,
[[storage]].

## Column memory layout

Stride is type-native: `size_of::<T>()` per column, each column with its own
stride (spec resolution R3). There is no per-entry flags byte; null and
tombstone semantics use a separate bitmask per column. The value constraint
is `size_of::<T>() <= 16`: the full 16 bytes usable, a clean power of two,
fits `u128`, aligns with SIMD lanes. This superseded the earlier universal
128-bit tagged-stride designs after benching showed type-native stride
outperforms them.

Columnar storage is contiguous, with 64-byte alignment on column array base
addresses and sequential prefetcher-optimal access. Within a fiber, all
columns share one co-located arena allocation with known static offsets: one
base pointer per morsel, each column at `base + col_offset + i * stride`.
Columns in different fibers are separate allocations. Column layout per
column is selected from scheduling hints (ByteAligned, Natural, BitPacked).

Arena addressing needs no size constraint: benched at 2 to 30 columns
(morsel arenas 24 KB to 248 KB, far past the aarch64 12-bit scaled immediate
range), performance varies within about 17 percent with no addressing-mode
cliff, because LLVM pre-computes column pointers from the arena base at
morsel boundaries and uses pointer-add in the inner loop. Co-location wins
where it matters: at 2 columns the arena is 33 percent faster than separate
pointers, and at 14 or more columns it still wins through shared TLB entries
and cross-column prefetch. The dispatcher resolves column pointers from the
arena base at each morsel boundary; the inner loop is pointer arithmetic.

## Column value types

```rust
pub trait ColumnValue: Copy + 'static {
    const BIT_WIDTH: USize = USize(core::mem::size_of::<Self>() * 8);
}
```

Any `Copy + 'static` type of at most 16 bytes is column-storable, enforced at
compile time. `BIT_WIDTH` informs the storage engine for bitpacking: the
default is byte-aligned, and sub-byte types override it (a 1-bit boolean, a
4-bit nibble) so the BitPacked layout can densify them. There is no pack or
unpack step and no slot-size intermediary: the type is the storage.

The founding spec expressed the default `BIT_WIDTH` through a
`min_specialization` blanket impl; that was de-specialized (round-level
amendment, task #631): the default lives in the trait body, and overriding
impls state their width explicitly. The 16-byte cap forces interned IDs in
place of heap-shaped values (String, PathBuf, Vec never enter columns).
Domain newtypes and the arvo-types-only discipline apply to our own crates;
consumers store whatever fits the contract. `repr(C)` on domain newtypes is
enforced by the `#[derive(ColumnValue)]` surface (macro or lint), not by
the trait itself (domain 05).

## Store types

Three store types, one scheduling mechanism:

```rust
pub struct Resource<T>(PhantomData<T>);  // singleton, one value
pub struct Column<T>(PhantomData<T>);    // collection, N records, morsel-chunked
pub struct Virtual<T>(PhantomData<T>);   // zero data, DAG edge only
```

The scheduler sees store-id pairs for edge construction and does not care
about cardinality: a write-to-read overlap on a Resource builds the same DAG
edge as one on a Column. `Column<T>` requires `T: ColumnValue` and carries no
schema parameter; schema-as-traits wrappers are consumer-side (spec
resolution R1), and translation happens in the consumer's registration
layer. Resources are accessed through the Context's resource accessors,
columns through morsel-scoped raw-pointer access, virtuals are fire-only.

`StoreId` is a dense index assigned at plan time. `AccessMask` is an
arvo-bitmask type sized to the store count; every scheduler set operation is
single-instruction bitwise (AND for conflict, OR for union, zero-test for
empty). `AccessSet` is a trait on tuples of store markers; the plan phase
lowers the tuple types to runtime masks through monomorphised generic
functions. The monomorphised function is the type identity: no TypeId, no
`std::any`. Store marker types are ZSTs that constrain the DAG, enforce type
safety, and determine morsel sizes at compile time; they are erased to plain
indices before execution begins.

## Column storage contract

`ColumnStorage` hands out raw pointers, never slices: slices assert noalias
that fused WorkUnit access violates (established as undefined behaviour in
the founding bench work, the cross-domain resolution both dispatch and
resources build on). Stride is type-native; column arrays are 64-byte
aligned. The consumer provides the backing memory; the library never calls
an allocator, and the `MemoryProvider` platform trait is the valve for any
allocation strategy. A separate arena carries resource `Seq`/`Map`
allocations (attached to the handle store, see [[storage]]).

Column lifetime uses the release-advisory consumer-count model: the schedule
pre-computes each column's reader count, decrements on fiber completion, and
release fires at zero. The evict/dump and inject/import APIs (spec resolution
R2) let consumers persist column data out and load it back in without copies
or retained references; hilavitkutin owns generation counters and skip
logic, the consumer owns persistence (see [[scheduler]]).

Every column is reserved once at plan time and never reallocated while the
scheduler lives: a reallocation would invalidate every recorded pointer with
no mid-life re-resolution mechanism.

## Determinism and ordering

Ordering is the data flow. DAG edges come from write-to-read column overlap;
there are no triggers and no emits, because data access patterns fully
capture ordering. Write-write ambiguity on the same store resolves by
scheduling-hint priority (Urgency, then Divisibility, then Significance),
tie-broken by most downstream dependents first, then a deterministic
fallback.

Commutativity is declared per WorkUnit (`COMMUTATIVE: bool`); a fiber is
commutative only if all its constituent WUs are. Commutative fibers are
eligible for front-back processing, multi-core morsel dispatch, and
deterministic segment assignment; non-commutative fibers process in a single
direction. Ordering is guaranteed by construction: non-commutative work is
trivially ordered on one core, and commutative deterministic segments are
always in record order, so no lineariser mechanism exists or is needed.

Records within a fiber are independent; execution order of records does not
affect correctness. The one exception is resource accumulation with
non-commutative operations, which skips head+tail convergence
(see [[execution]]).

There are no partial writes: WUs buffer internally and write on return,
downstream does not run until completion, and version stamps increment only
on completion. Correctness is independent of atomicity.
