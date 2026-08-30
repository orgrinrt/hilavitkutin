# Sketch: the install arm for replace_resource / replace_value

**Date:** 2026-07-19
**Hypothesis:** the install is `Selector<T, Index>::get(&self.bindings)` then a write through the
pointer, reusing the `Index` the signature already carries for `Stores: Locate<T, Index>`.
**Outcome:** **FAILS, and the failure is larger than the hypothesis.** The current signature cannot
express an install at all, because its value parameter carries no data.

## What was tried

Both bodies were given the install, with the added bound
`<Vals as BindingsFor>::Bindings: Selector<T, Index>`:

```rust
unsafe { *Selector::<T, Index>::get(&self.bindings).as_ptr() = new; }
```

`cargo check -p hilavitkutin` passes. That is not evidence of anything: a where-clause bound is
assumed inside the function and only checked at call sites. The library compiling means the body is
consistent with an assumption, not that the assumption holds.

The call site is where it resolves, and it does not:

```
error[E0277]: the trait bound `ResourceBinding<Ra, BindingNil>: Selector<Resource<Ra>, Here>`
              is not satisfied
error[E0308]: mismatched types: expected `Resource<Ra>`, found `Ra`
```

## Why: two different type spaces, not two different index spaces

The round expected a possible mismatch between index witnesses. The mismatch is in the type.

`Locate<T, Index>` ranges over the **Stores** access-set list, whose members are store *markers*:
`Resource<Ra>`, `Column<X>`, `Accum<Y>`. So `T` unifies with `Resource<Ra>`.

`Selector<T, Index>` ranges over the **bindings** list, whose nodes are `ResourceBinding<Ra, _>`,
keyed by the *value* type. So it would need `ResourceBinding<Resource<Ra>, _>`, which does not exist
and should not.

There is no marker-to-value mapping to bridge them: `Resource<T>` is
`pub struct Resource<T>(PhantomData<T>)` (`hilavitkutin-api/src/store.rs:18`) with no associated
type naming its value.

## The larger finding

`Resource::new(value)` does not return a `Resource<T>`. It returns `StagedResource<T>`
(`store.rs:46`), a distinct type that owns the value until `build()` moves it into scheduler storage.
`Resource<T>` itself is a zero-sized marker.

So in `replace_value<T: Replaceable, Index>(&mut self, new: T)`, with `T` unifying to `Resource<Ra>`,
the argument is a ZST. **There is no value to install.** The signature is not merely unimplemented;
it cannot be implemented as written, and no body could have satisfied its own doc comment.

That reframes the defect. The audit called it "the argument is discarded". It is worse: the argument
never carried anything to discard, and the doc comment promises a swap the type signature makes
impossible.

## The contract fork, which is op's

Three shapes, all requiring a public signature change:

**A. Take the value.** `replace_value<V, Index>(&mut self, new: V)` with
`Stores: Locate<Resource<V>, Index>` and `Bindings: Selector<V, Index>`. Reads naturally at the call
site (`scheduler.replace_value(Ra(7))`) and matches what the bindings are keyed by. Changes the type
parameter's meaning from marker to value, so `Replaceable` would be implemented on `Ra` rather than
on `Resource<Ra>`.

**B. Take the staged carrier.** `replace_value(&mut self, staged: StagedResource<V>)`, mirroring the
builder's `.with(Resource::new(v))` spelling exactly. Most symmetric with registration; slightly more
ceremony at the call site.

**C. Keep the marker, add the mapping.** Give store markers an associated value type
(`trait StoreMarker { type Value; }`, `impl<T> StoreMarker for Resource<T> { type Value = T; }`) and
take `new: T::Value`. `Replaceable` stays on the marker, which preserves the current opt-in
granularity and its diagnostic message.

## Status

**Source reverted.** Shipping a function whose bound no call site can satisfy would be worse than the
current stub: the stub at least compiles for consumers. The finding stands on its own and the
contract choice goes to op, which is what gating this item on a sketch was for.

## Second pass: canon's "member-by-member" describes a model that has not shipped

A reading of `202606210600_expert-architect-storage-model.md` suggested the mechanism was already
settled against the pointer write:

> "under the handle model a swap is a handle update ... plus a member-by-member copy, **not an
> in-place blob memcpy**. Swap semantics need explicit spec."

> "**E. Replaceable swap:** member-by-member copy of each leaf into its column slot ... Same path as
> initial drain."

**That reading was wrong, and the error is the same one this arc keeps producing.** Both quotations
are scoped to "the handle model", the per-member decomposition the document *proposes*. It has not
shipped. The same document argues against it for the current workload and recommends the opposite:

> "**A. blob + separate-provenance stack-snapshot (minimal change).** ... Delivers the measured
> 1.28-1.40x win with NO decomposition ... Viable + simpler for the current small-singleton
> workload."

I read the swap language without checking whether the storage model it presupposes was live. That is
the fifth instance of reading a claim without establishing what it applies to.

## What the shipped drain actually does

`bindings.rs:335-375`, the resource arm:

```rust
match cs.reserve::<T>(id, USize(1)) { ... }        // one record of T
let typed = unsafe { cs.column_ptr_mut::<T>(id) };  // *mut T
unsafe { core::ptr::write(typed, value); }          // one whole-T write
```

A resource is one contiguous `T` in one reserved record. "Member-by-member copy" for a blob-model
resource **is** the single whole-`T` write. So the pointer write is not a violation of canon's
mechanism; under the shipped storage model it *is* canon's mechanism, and it mirrors the drain
exactly. The only difference is that the drain uses `ptr::write` into an uninitialised slot while a
swap assigns, so the previous value drops rather than leaks.

Canon's member-by-member requirement becomes binding if and when per-member decomposition ships
(#654). At that point the install follows the decomposition, like every other write path.

## Resolution

- **Mechanism: settled, and it is the pointer write.** Confirmed against the shipped drain rather
  than inferred.
- **Signature: genuinely open.** Canon says so in as many words: "Swap semantics need explicit
  spec." The A/B/C fork above is a real design choice, not a lookup.
