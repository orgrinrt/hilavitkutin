# FINDINGS: E4 slice-1 schedule-gated walk

**Round:** 202606081200 (GATE-2 E4 slice 1 — virtual firing + On-gating)
**Hypothesis:** the const-gated cons walk can recover each element's `Schedule` (Always vs On<V>) and branch the run-gate at compile time (Always run; On<V> run iff V fired), over a carrier mixing both, with no runtime type dispatch.

## Outcome: WORKS

`cargo run` prints `producer=1 ontick=1 ontock=0` then `WORKS`. A carrier of `[Producer (Always, fires Tick), OnTick (On<Tick>), OnTock (On<Tock>)]` dispatches correctly: the Always WU runs, the On<Tick> WU runs (Producer fired Tick earlier in the walk), the On<Tock> WU does not (Tock never fired).

## What it proved

1. **Schedule recovery is via a companion associated-type trait, not a free param.** `W: WorkUnit<S>` with `S` inferred is ambiguous (a type could carry several `WorkUnit<S>` impls, and the walk has no way to pick one). The clean, unambiguous shape is a companion `trait Scheduled { type Sched: ScheduleGate; }` impl'd once per WU. The walk bound becomes `W: Wu + Scheduled` and it reads `<W as Scheduled>::Sched`. rustc resolves it with no ambiguity.

2. **The gate dispatches at compile time.** `ScheduleGate` has `fn should_run(&FiredSet) -> bool` with `impl ScheduleGate for Always` (returns `true`, const-folds, the `if` vanishes) and `impl<V: Virtual> ScheduleGate for On<V>` (a flag read). `<<W as Scheduled>::Sched as ScheduleGate>::should_run(fired)` is a static monomorphised call, no vtable, so an Always WU pays nothing and an On<V> WU pays one flag read. This composes with the existing const-gated member walk exactly as the dirty-skip bit does (a per-member predicated branch).

3. **Mixed-schedule carriers compile.** A single `RunGated` impl over `WuCons<W, Tail>` handles elements of differing `Sched` (Always and On<V> and On<Tock>) in one cons-list; each element's gate is its own monomorphisation. This is the bound generalisation the engine's `RunFiber for WuCons<W, Tail>` needs (it currently bounds `W: WorkUnit` = `WorkUnit<Always>`, which rejects On<V> WUs).

## What it deliberately did NOT settle (src-CL design, not mechanism)

- The exact domain-10 firing semantics: per-(virtual, consumer) bits vs per-virtual; epoch-based reset (flag == epoch, increment to clear) vs explicit clear; clear-on-dispatch; same-pass (firer ordered before consumer, as here) vs next-pass (flag persists across the pass boundary). The API doc says On<V> consumers "run next pass"; the meta kernel fires + consumes within one invocation. These are layered on top of the proven trait dispatch and are a src-CL decision grounded in spec `:685-714` (+ source-topic T5 §Q1/§Q3 if the bullets are ambiguous). The sketch used a minimal per-virtual bool set and same-pass ordering (Producer precedes OnTick) purely to exercise the gate.
- Devirt under fat LTO (the D6 ASM concern): the call is static by construction, but the actual zero-`blr` confirmation is a src-CL ASM-gate check, not this sketch's scope.

## Unblocks

The src CL for slice 1:
1. Add `Scheduled` (companion trait naming each WU's Schedule) + `ScheduleGate` (the Always / On<V> compile-time gate) to hilavitkutin-api.
2. Generalise `RunFiber` / `RunGatedTrunk` from `W: WorkUnit` to recover `<W as Scheduled>::Sched` and gate on it (the Always path stays identical to today; On<V> adds the flag read).
3. Real fired-flag store on the bindings (domain-10 epoch shape) + the engine `VirtualFirerApi` impl setting it (replacing the B3 no-op at `engine_ctx.rs:1047`).
4. TDD: a firer WU fires `Virtual<V>`; an `On<V>` consumer runs + observes; absent the fire it does not; full suite bit-identical; D6 ASM bit-test not blr.
