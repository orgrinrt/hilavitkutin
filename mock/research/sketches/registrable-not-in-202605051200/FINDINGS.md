# Findings: NotIn<H> compile-time diamond detection

**Date:** 2026-05-05.
**Round:** 202605042200.
**Outcome:** FAILS — `feature(negative_impls)` (with or without `with_negative_coherence`) cannot soundly encode type-list disequality on cons-list shape with parameterised K.

## Setup

The proposal commits to compile-time error on duplicate-marker registration via `Stores: NotIn<H>` bound at the engine's leaf Registrable impl. Encoding sketched in `sketch.rs`:

```rust
#[marker]
pub trait NotIn<H> {}

impl<H> NotIn<H> for () {}
impl<H, K, R> NotIn<H> for (K, R) where R: NotIn<H> {}
impl<H, R> !NotIn<H> for (H, R) {}
```

The intent: positive blanket on `(K, R)` covers the K != H case; negative impl on `(H, R)` carves out K == H.

## Observed result

```
$ rustup run nightly rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata
error[E0751]: found both positive and negative implementation of trait `NotIn<_>` for type `(_, _)`:
  --> sketch.rs:71:1
   |
66 | impl<H, K, R> NotIn<H> for (K, R) where R: NotIn<H> {}
   | --------------------------------------------------- positive implementation here
...
71 | impl<H, R> !NotIn<H> for (H, R) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ negative implementation here
```

Tested on rustc nightly. Same failure with and without `feature(with_negative_coherence)` enabled. The positive blanket's universally-quantified `K` includes the case `K = H`, which conflicts with the negative impl regardless of which negative-coherence feature is active.

## Why it fails

Rust does not have a "K is not H" predicate at the type level. The positive blanket `impl<H, K, R> NotIn<H> for (K, R)` admits ALL K including K = H. The negative impl `impl<H, R> !NotIn<H> for (H, R)` is correct semantically (the case K = H), but the trait coherence checker sees both impls applying when the substitution `K = H` happens, and rejects.

The encoding would require either:

a) **Subtractive blanket** with an explicit "K not equal H" bound — Rust doesn't have it.
b) **Specialisation** with `default impl` on the blanket and override on `(H, R)` — `feature(specialization)` is much more unstable than `negative_impls` and the workspace forbids unstable specialisation.
c) **An auxiliary `TypeNeq<A, B>` witness trait** — equivalent to `TypeId` semantically; workspace forbids `TypeId`.

No viable encoding survives the workspace's nightly-feature constraints.

## Implication for the proposal

The third revision's "compile-time error preferred; runtime first-wins fallback" is an unsubstantiated promise. Compile-time error via `NotIn<H>` cannot be delivered with the sketched encoding under workspace constraints. The "fallback" was named but its semantics were left vague (the proposal said "engine-side check on Stores cons-list at runtime", which is wrong: Stores is a type, not a runtime value).

## Path forward (recommendations for round 4)

The proposal must commit to ONE of:

1. **Silent shadowing, Bevy-style.** Accept that two `Resource<Foo>` entries in `Stores` cons-list are tolerated; `Contains<Resource<Foo>>` proofs match the first occurrence; consumer is responsible for not creating diamonds. Document loudly. Add `#[diagnostic::on_unimplemented]` hint when consumers hit the related "Resource registered twice but only first is read" debugging surface.

2. **Runtime panic at engine `.resource(t)`.** Engine maintains a runtime registry of registered marker TypeId-equivalents (without using `TypeId`; possibly via `core::any::type_name::<T>()` string equality, or a sealed enum-of-markers pattern). Detect duplicate at registration call; panic. Costs one runtime check per registration; trade for explicit failure mode.

3. **`.replace::<M>(value)` explicit operator** on the builder + silent-shadow as default. Consumer writing `.with(MyResource).with(MyResource)` shadows; consumer writing `.replace::<Resource<Foo>>(new)` explicitly overwrites. The `.replace` operator gates on `Stores: Contains<M>` (must already be present). This is the cleanest design and matches Bevy's `init_resource` vs `insert_resource` distinction.

4. **Drop diamond detection entirely** for v0; track as a future follow-up when `feature(negative_impls)` matures or a new approach surfaces.

The sketch's negative-impl path is dead. Decision point for the user.

## Cross-references

- `mock/design_rounds/202605042200_topic_markers_as_registrables_final.md` — proposal under audit
- Round 3 type-system audit (memory; see resume) — predicted this failure
- Round 3 architectural audit — recommended `.replace` + silent shadowing
- workspace rule `arvo-toolbox-not-policer.md` — relevant principle
