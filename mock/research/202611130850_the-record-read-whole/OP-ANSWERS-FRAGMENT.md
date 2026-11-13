# Op's words on the points of the shared principles fragment

This note holds the maintainer's own words found on points that the principles template states in the block ported from the workspace's shared principles fragment by `8e63891` and `04e1cb9` of 2026-04-20. Each section below the line is copied line for line, by command, from a capture kept in the workspace repository, with the file and the line range named in its heading paragraph, and nothing in it is reworded. The registry's rows that rest on these words, or whose stamps name them, cite this note, so that a clone of hilavitkutin on its own can open them. No capture holds his words about the fragment itself.

---

## Contracts first, 2026-09-04 09:31

Copied from `.data/op-responses/202609040931_contracts-first-is-a-rule-in-shared-rules.md`, lines 23 to 29.

## Said unprompted

> Make that a rule in fact, if not already. Always trait contracts first, with
> the typestate well designed prior with zsts, markers, and all of the logic
> working through generics with real bounds, assoc types similarly, and the
> impls just slot in then at the use site via generics or assoc types. The
> usual deal for all of our rust and kind of on ts stack too.

## No alloc means no transient heap, 2026-11-12 17:58

Copied from `.data/op-responses/202611121758_op-on-notko-no-alloc-and-his-stack-under-hilavitkutin.md`, lines 34 to 36.

## Said unprompted at 17:58

> You could do some surface level reading and grepping on hilavitkutin and notko specifically. Notko has a contract api for explicit mem allocation lending and filling. No alloc only means no transient heap, everything accounted for statically. Still can have large and even dynamic allocations, just needs statically typed backing to be accepted and made sure it's sound. Hilavitkutin comes with a bunch of things beyond just scheduling, too. Or you could wait. Whatever. My own stack is explicitly mostly conformant to no_std and if we went hilavitkutin, there's already some work done on similar no_std and my stack for input (Tassu) and potentially huurre (windows a subset, don't need the full app framework) once it lands no_std window api. Kirjo, riimu, lato, etc already exist. Arvo, too, is same principle. Minimal deps, no_std, principally no alloc if notko in the stack which makes things entirely controllable and pleasant to work with

## The tracked line of the lint allow marker, 2026-09-02 20:36

Copied from `.data/op-responses/202609022036_huurre-wayland-lint-allow.md`, lines 50 to 82.

## Asked

> huurre-wayland's bare primitives are down from 110 to 57, all in four
> foreign-ABI declaration files: ffi.rs 22 (libwayland), xkb.rs 15
> (libxkbcommon), wire.rs 11 (the seam conversions), sys.rs 9 (libc). Every
> other file in the crate now names only wire newtypes, arvo types or domain
> aliases. Those four declare C signatures and compute nothing. What mechanism
> covers them?

Options offered:

1. **Per-file exclude in mockspace.toml.** Four narrow entries, each stating
   the file declares a named foreign ABI and computes nothing. It's the
   mechanism already used for handle.rs, flags.rs, measure.rs and arena.rs in
   this same file. Not a lint:allow and not a severity change. But it is the
   same shape as the eight src/imp/ groups you had me delete.
2. **lint:allow per line, with a task.** What no-bare-primitives.md actually
   names for FFI repr(C) boundary fields: tracked via lint:allow with a linked
   task, explicitly "not a free pass". It is the canon's stated form, and it is
   ~57 markers, which is the sprinkling you told me not to do.
3. **A category in the lint pack.** Add a `foreign-abi` primitive-introduction
   category beside `numeric` and `fallibility`, declared per crate in
   mockspace.toml. Honest at the concept level, but the granularity is the
   crate rather than the file, so it would except huurre-wayland's logic files
   too, which is wider than the claim.
4. **Leave it red for now.** The gate stays blocked, nothing commits in
   huurre, and the call waits. I keep working in the other repos in the
   meantime.

## Answered

> The upstream needs to remove the tracked line, because we've not landed on
> any standard there that would be useful. But option 2 then.
