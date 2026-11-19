# Sketch findings: a set of pragma types is refused at compile time when a requirement on it breaks

**Date:** 2026-11-19
**Outcome:** WORKS for all three requirements as constants asserted in a const block; as trait bounds WORKS for all-of and for one per group, and FAILS WITH E0283 for any-of once two of the asked pragmas are in the set
**Unblocks:** the pragma round can state a pragma as a unit type and a set as a list of them, with the requirement check written once over the list

## Hypothesis

A pragma can be a unit type implementing one trait, a set can be a cons list of those types, and the three requirements the round names can each refuse a set before the build script runs: all of a list is present, one of a list is present, and a group holds one member at most. `no_std`, no allocation, no feature gate. Two routes were built side by side: the requirements as trait bounds over the list (`src/bounds.rs`), and the list folded into constants that a const block asserts over (`src/mask.rs`).

The pragmas are five the crate has today, with the two requirements `PragmaSet::coherent` states (MathPeephole on FastMath, Profiling on Pgo or Bolt), and two linkers and a backend standing in for the round's groups.

Built under `nightly-2026-05-28` (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`), from this directory, one example per set.

## What was tried, in order

### 1. A coherent set, bounds route: compiles and runs

`cargo run --quiet --example ok`

```
-Zllvm-plugins=math-peephole -Zfast-math -Cprofile-generate -Cprofile-use -Clink-arg=-fuse-ld=mold -Zcodegen-backend=cranelift
```

### 2. Two linkers, bounds route: refused

`cargo build --quiet --example breaks_exclusion`

```
error[E0277]: the pragma set holds two pragmas of one group: `(LinkerGroup, LinkerGroup)`
   --> examples/breaks_exclusion.rs:8:17
    |
  8 |     configure::<Set, _, _>(&mut text).unwrap();
    |                 ^^^ a set holds one pragma of a group
    |
    = help: the trait `Distinct` is not implemented for `(LinkerGroup, LinkerGroup)`
    = help: the following other types implement trait `Distinct`:
              (BackendGroup, Free)
              (BackendGroup, LinkerGroup)
              (Free, G)
              (LinkerGroup, BackendGroup)
              (LinkerGroup, Free)
    = note: required for `Mold` to implement `Apart<(Wild, ())>`
    = note: required for `(Mold, (Wild, ()))` to implement `OnePerGroup`
    = note: required for `(Mold, (Wild, ()))` to implement `Coherent<((), NoneAsked, ((), NoneAsked, ()))>`
```

The bound is one blanket impl over the pragmas' `Group` types and one `Distinct` impl per ordered pair of groups that differ. Two impls of the shape "a linker beside a backend" and "a linker beside a driver" would overlap, since nothing tells the compiler the two group traits are disjoint; the associated type with the pairs written out does not.

### 3. MathPeephole without FastMath, bounds route: refused

`cargo build --quiet --example breaks_all`

```
error[E0277]: the pragma set does not hold `FastMath`
   --> examples/breaks_all.rs:8:17
    |
  8 |     configure::<Set, _, _>(&mut text).unwrap();
    |                 ^^^ a pragma in this set stands on `FastMath`
    |
    = help: the trait `Has<FastMath, _>` is not implemented for `()`
```

### 4. Profiling with neither Pgo nor Bolt, bounds route: refused, and the message is the compiler's own

`cargo build --quiet --example breaks_any`

```
error[E0277]: the trait bound `(Profiling, ()): Coherent<((), _, ())>` is not satisfied
   --> examples/breaks_any.rs:8:17
    |
  8 |     configure::<Set, _, _>(&mut text).unwrap();
    |                 ^^^ the trait `HasAny<(Pgo, (Bolt, ())), _>` is not implemented for `(Profiling, ())`
```

The `on_unimplemented` text on `HasAny` is not used here.

### 5. Profiling with both Pgo and Bolt, bounds route: a coherent set is refused

`cargo build --quiet --example any_two_present`

```
error[E0283]: type annotations needed
   --> examples/any_two_present.rs:9:5
    |
  9 |     configure::<Set, _, _>(&mut text).unwrap();
    |     ^^^^^^^^^^^^^^^^^^^^^^ cannot infer type of the type parameter `I` declared on the function `configure`
    |
    = note: multiple `impl`s satisfying `(Profiling, (Pgo, (Bolt, ()))): HasAny<(Pgo, (Bolt, ())), _>` found in the `pragma_requirements_as_bounds` crate:
            - impl<S, H, T, I> HasAny<(H, T), First<I>> for S
              where S: Has<H, I>;
            - impl<S, H, T, I> HasAny<(H, T), Later<I>> for S
              where S: HasAny<T, I>;
```

This is the wall of the bounds route. Membership in a list needs the position as an inferred type, since a bound cannot ask whether the head of a list equals a type. All-of has one position per asked pragma. Any-of has one per asked pragma that is present, and with two present the compiler has two proofs and takes neither. Taking the first that holds needs a bound that says the earlier ones are absent, which is a negative bound. No attempt past this was made.

### 6. The same sets, mask route: the coherent one runs and each broken one is refused in the pragma's own words

`cargo run --quiet --example mask_ok`, with Profiling, Pgo and Bolt all in the set:

```
-Zllvm-plugins=math-peephole -Zfast-math -Cprofile-generate -Cprofile-use  -Clink-arg=-fuse-ld=mold -Zcodegen-backend=cranelift
```

`cargo build --quiet --example mask_breaks_exclusion`

```
error[E0080]: evaluation panicked: the set holds two linkers
...
note: the above error was encountered while instantiating `fn pragma_requirements_as_bounds::mask::configure::<(Mold, (Wild, ())), String>`
 --> examples/mask_breaks_exclusion.rs:8:5
```

`cargo build --quiet --example mask_breaks_all`

```
error[E0080]: evaluation panicked: MathPeephole stands on FastMath, which the set does not hold
...
note: the above error was encountered while instantiating `fn pragma_requirements_as_bounds::mask::configure::<(MathPeephole, ()), String>`
 --> examples/mask_breaks_all.rs:8:5
```

`cargo build --quiet --example mask_breaks_any`

```
error[E0080]: evaluation panicked: Profiling stands on Pgo or Bolt, and the set holds neither
...
note: the above error was encountered while instantiating `fn pragma_requirements_as_bounds::mask::configure::<(Profiling, ()), String>`
 --> examples/mask_breaks_any.rs:8:5
```

### 7. The mask route under `cargo check`: not refused

`cargo check --example mask_breaks_any`

```
    Checking pragma-requirements-as-bounds v0.0.0 (...)
    Finished `dev` profile [unoptimized] target(s) in 0.10s
```

The const block is evaluated when `configure` is instantiated, and a check instantiates nothing.

## The result

A pragma as a unit type and a set as a cons list of them holds, with no allocation and no feature gate, and the flags are written through `core::fmt::Write` by walking the list.

As constants, all three requirements refuse a broken set at build time with a sentence the pragma supplies, and a set that meets an any-of twice over is accepted. The refusal arrives when the function is instantiated, so a build reports it and a check does not.

As trait bounds, all-of and one per group refuse a broken set at type check, a check included. Any-of refuses a set that holds none of the asked pragmas and also refuses a set that holds two of them, so the bounds route cannot carry any-of as written.

The attempts 2 through 5 and the three broken sets of 6 are the cases that must fail, and each was seen failing; attempts 1 and the first of 6 are the same code accepting a coherent set.

## What is NOT established

The list here is a bare tuple pair. `notko-hlist` was not linked, so whether its list type takes these impls from another crate under the orphan rule is not shown.

Whether a `const _: () = <Set as Checked>::OK;` beside the call makes a check report the mask route's refusal was not tried.

A pragma with a value (`ParallelCodegen` and its count) is not in the sketch.

The mask route gives each pragma one refusal text for both its group and its requirements, which was enough here because no sketched pragma has both.

Nothing here touches target admission or whether a tool is on the machine, which the round's note leaves to a refusal when the script runs.
