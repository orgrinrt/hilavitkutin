# `hilavitkutin`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/hilavitkutin.svg)](https://github.com/orgrinrt/hilavitkutin/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/hilavitkutin.svg)](https://github.com/orgrinrt/hilavitkutin/issues)
![License](https://img.shields.io/github/license/orgrinrt/hilavitkutin?color=%23009689)

> A morsel-driven pipeline engine for `no_std` rust, with its kernels and stores written against one small set of contracts.

</div>

hilavitkutin runs kernels over columns of records, morsel by morsel, with no allocator, no spawn and no `dyn`. New work is written against `hilavitkutin-contracts`, which is where a kernel author and a store implementor meet without seeing each other: the author writes value kernels over a domain of records, the implementor writes the store the records live in and the order they are visited in, and a table of plain data binds the two, checked once by `bind` before anything runs.

A value kernel has no receiver and is handed no position, only the values of what it reads at one record, and it returns the values of what it writes there. Which stored column fills which of its columns, which records it runs over and in what morsels, belong to the table, the store and the runner, so the same kernel runs over any store and under any runner as it is. A step reads a committed view and writes staged cells, and the checked table cuts the staged side into one part per unit holding only the columns that unit writes, which a runner can cut further by position and hand to its threads with nothing more asked of the author or the implementor.

`hilavitkutin-reference` is the one-thread runner every other runner is held to by digest, and `hilavitkutin-laws` holds the laws a run is checked against, as plain functions over what the run produced, so a store or a runner written elsewhere can be held to the same laws. The scheduler in `hilavitkutin` and its older contract crate `hilavitkutin-api` predate the contracts and are being ported onto them, after which `hilavitkutin-api` goes.

## Usage

A kernel names its domain, what it reads, the columns it touches and what it writes, and an identity that stays the same across builds. Here `Plane` is a two-axis domain with `East` one of its offsets, and `Heat` and `Flux` are columns of it holding the lane `L`:

```rust
use hilavitkutin_contracts::domain::{Near, Own};
use hilavitkutin_contracts::kernel::{KernelId, ValueKernel};
use hilavitkutin_contracts::list::{Cons, Empty};
use hilavitkutin_contracts::num::{Lane, USize};

pub struct Mirror;

impl ValueKernel for Mirror {
    type Domain = Plane;
    type Reads = Cons<Own<Heat>, Cons<Near<Heat, East>, Empty>>;
    type Roles = Cons<Heat, Cons<Flux, Empty>>;
    type Writes = Cons<Own<Flux>, Empty>;

    const ID: KernelId = KernelId::new(USize::from_host(2));

    // the flux at a place is its heat less the heat east of it
    fn at((here, (east, ())): (L, (L, ()))) -> (L, ()) {
        (L::narrow(L::wide_sub(here.widen(), east.widen())), ())
    }
}
```

The table gives each unit a stored column and its format for every column in `Roles`, in that order. `bind` checks it against the kernel list and the store, and holds the store for as long as the checked table lives, so the table cannot be run over anything it was not checked against:

```rust
let roles = [
    Role { column: ColumnNo::from_host(1), format: L::FORMAT },
    Role { column: ColumnNo::from_host(2), format: L::FORMAT },
];
let table = [Binding {
    unit:   UnitId::new(USize::from_host(0)),
    kernel: Mirror::ID,
    roles:  &roles,
    params: &[],
}];

let mut bound = match bind::<Plane, Cons<Mirror, Empty>, _, _>(&table, &mut store) {
    Outcome::Ok(bound) => bound,
    Outcome::Err(refusal) => panic!("{refusal:?}"),
};
let mut digests = [Digest::new(); 8];
let mut out = Run::new(&mut digests);
let done = run(&mut bound, &visit, Morsel::ONE, Count::from_host(8), &mut out);
```

A refusal is a value either way, from `bind` for a table that does not hold and from `run` for a frame that could not be kept or a visiting order that broke its contract.

## Testing

`cargo +nightly test --workspace` runs the test suites across every crate. Verification of the engine's atomic-ordering guarantees uses MIRI under strict-provenance and tree-borrows; see [`MIRI.md`](./MIRI.md) at the repo root for the flag invocation and the test files that exercise the atomic-ordering paths.

## Status

It's early days. The contracts are not locked, five of their shapes are still open, and names change without deprecation aliases. The plugin-host layer (`hilavitkutin-linking`, `hilavitkutin-extensions` and its macros) is implemented, the ecosystem crates partly, and the engine's runtime is designed and waits on its port onto the contracts. It builds on nightly only: the contracts use no feature gate of their own, but the `notko` paths they turn on and the engine both need unstable rustc features.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/hilavitkutin/blob/dev/LICENSE)
