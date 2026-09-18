# GATE-2 Deviation 4: Pointer-Size Spawn and Exit-Counter Join

**Date:** 2026-07-19
**Status:** the design-memo evidence for deviation 4 under
evidence-then-bless (A2-4); seed governance item 5. This deviation's
channel is analysis, not a bench: the question is contract shape, not
speed.

## The shipped shape

`ThreadPoolApi::spawn<F: FnOnce() + Send + 'static>` is the contract;
the shipped `OsThreadPool` realises it by smuggling `F` BY VALUE through
the pthread `*mut c_void` argument, with a runtime assertion that
`size_of::<F>() <= size_of::<*mut c_void>()` (`platform/os.rs:115`,
message: "closure must be pointer-sized (no alloc to box a fatter
closure)"). Join is an exit counter (`PoolFrame.exited`, awaited with a
real Acquire in `thread/frame.rs`), not per-thread `JoinHandle`s.

## Why it is sound, and where the limit bites

Soundness: the engine's one spawn site captures exactly one
`SendCtxPtr` (a pointer), so the size assertion holds by construction;
the transmute-copy reconstruction reads exactly `size_of::<F>()` bytes;
and the exit counter is a genuine happens-before join (the #689 audit
confirmed the Release/Acquire pair), with `Drop` awaiting it before any
field drops. Within the engine there is no hole.

The limit is a CONTRACT gap, not an engine bug: the api trait promises
arbitrary `FnOnce`, the shipped impl accepts only pointer-sized ones,
and the failure is a runtime assertion rather than a compile refusal. A
consumer bringing `OsThreadPool` to its own fatter closure hits a panic
the trait signature never warned about. Under the no-alloc identity the
impl cannot box, so the honest choices are narrowing the contract or
widening the mechanism.

## The alternatives, priced

Widening (canonical-flavoured): an inline-storage spawn (a fixed
N-byte closure slot per worker, the closure written into pool-owned
storage rather than the pthread argument) admits fatter closures with
no alloc. Cost: a per-worker slot in the worker-visible plane and one
indirection at thread start (cold path, once per pool lifetime). Under
the deviation 1+5+6 plane relocation this slot has a natural home, so
the widening is nearly free WHEN that round lands. Narrowing
(honest-contract): keep pointer-size but move the check to compile
time (a `const` assertion on `size_of::<F>()` in the impl, which the
existing engine call site already satisfies), so misuse is a compile
error naming the rule instead of a runtime panic.

## Proposed ruling (op's call)

Bless the mechanism, fix the contract: keep pointer-size spawn and the
exit-counter join as the engine's shape (sound, alloc-free, and the
only spawn site satisfies it by construction), lift the runtime size
assertion to a compile-time refusal now, and fold the inline-storage
widening into the plane relocation round as its natural extension for
arbitrary consumer pools. The exit-counter join needs no change; it is
the part of this deviation that was canonical all along.
