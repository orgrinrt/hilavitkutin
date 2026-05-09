# Builder + Provider shape sketch

**Hypothesis:** the unified `.with(value)` builder shape from round
202605091700 topic 2 compiles and resolves correctly under current
nightly (1.96.0-nightly) with the features hilavitkutin-api already
pins (`adt_const_params`, `const_trait_impl`, `generic_const_exprs`,
`marker_trait_attr`).

The shape is:

- `pub trait Provider` with `type Init` associated type.
- `pub const trait Marker: Provider<Init = ()>` requiring `const fn new()`.
- Library-side wrappers (`Column<T>`, `Virtual<T>`, `LinkedBin<T: ?Sized>`)
  impl `Marker` so `Type::<T>::new()` works.
- User-side unit structs impl `Provider` directly (no `Marker` needed
  because the user writes the bare name).
- Stateful providers (`Resource<T>`) impl `Provider<Init = T>` and ship
  an inherent `::new(value)` constructor.
- Platform impls (memory, threads, clock) impl their respective API
  traits; a blanket impl wires them as providers.
- Builder method: `.with<P: Provider>(p: P) -> Self::NextState` where
  the typestate updates via per-kind dispatch traits.

**Concerns to validate:**

1. Does the trait dispatch resolve unambiguously when the same
   `.with<P>` method accepts WUs, Resources, Columns, Virtuals,
   LinkedBins, Kits, and platform providers?
2. Does `const Marker` compile under the workspace's nightly pin?
3. Does typestate accumulation (Cons-list grow) work through one
   uniform method?
4. Does the LinkedBin path with `dyn TraitFamily` survive (since
   `dyn Trait` is unsized)?

**Outcome:** see `OUTCOME.md`.
