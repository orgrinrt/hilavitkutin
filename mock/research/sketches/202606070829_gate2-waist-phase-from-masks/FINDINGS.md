# FINDINGS: gate2-waist-phase-from-masks (GATE-2 chart S2 / R2-pre)

**Hypothesis.** The engine can derive the CANONICAL waist-bounded phase axis at
const time from per-unit access masks, via arvo's `waist_detect_const` (R1b), on
the pinned nightly. This is the chart's S2 / R2-pre gate: the genuinely-new R2
chain (mask overlaps -> unit adjacency -> const waist detection -> canonical
phase mapping), proven before wiring it into `plan/grouping.rs`.

**Outcome: WORKS.**

```
waist flags  = [false, false, false, true, false, false]
waist phases = [0, 0, 0, 0, 1, 1]
depth phases = [0, 1, 1, 2, 3, 3]
S2 SKETCH: WORKS
```

All three (`ADJ`, `FLAGS`, `PHASES`) are computed in `const` items, so the whole
chain const-evaluates.

**What it proves, concretely.**

1. **Mask -> unit-adjacency const build.** A `const fn` builds the unit x unit
   RAW adjacency `[Adj; N]` (`Adj = Bits<64, Hot, Unsigned>`, the engine's
   `D::AdjRow` default) from per-unit access masks: row `i` bit `j` set iff
   `reads[j]` overlaps `writes[i]`. Overlap is the const `BitLogic::bitand` +
   `BitSequence::is_zero` composition (the same the shipped `AccessMask::overlaps`
   uses); the bit-set is const `BitAccess::with_bit_set`. No `BitMatrix`/`Mask`
   inherent method touched.

2. **`waist_detect_const` over the engine's adjacency + identity topo order.**
   `waist_detect_const::<Dim<N>, Adj>(&ADJ, &ORDER)` const-evaluates and returns
   the waist flags. `ORDER` is the identity `[NodeId(0)..NodeId(N-1)]`, which is a
   valid topo order because the build rejects anti-topological registration
   (registration == topo at dispatch). `Adj = Bits<64>` is the exact `W: [const]
   BitAccess` word R1b proved, so no new bit-word risk.

3. **Canonical waist -> phase mapping.** `phase[k] = count of waist flags
   strictly before k`, the const analog of the runtime `compute_waists`
   (steps.rs:314-326: phase 0 starts at position 0; each waist position opens a
   new phase at the next position; the waist unit is the last of its phase).

4. **The axis genuinely changed.** The fixture is an hourglass (wide-narrow-wide)
   where the waist-bounded phase `[0,0,0,0,1,1]` DIFFERS from the depth axis the
   shipped grouping computes today `[0,1,1,2,3,3]`. The `assert_ne!(PHASES,
   DEPTH)` makes the sketch falsifiable: it proves the course-correction (phase =
   waist-bounded, not depth) actually takes effect, not that the two coincide.

**What it does NOT prove (deferred to R2 itself, low risk).**

- It models per-unit masks as `Bits<8>` directly rather than the engine's
  `AccessMask<CS>`. `AccessMask`'s own const surface (`empty`/`set`/`contains`/
  `overlaps`) is already shipped + const (used in the shipped const
  `compute_phases`), so the substitution is faithful; R2 folds the real
  `AccessMask` arrays the existing `BundleMasks` fold produces.
- It uses a fixed `N = 6` fixture, not the generic `GATE2_MAX_UNITS`-sized
  scratch. R2 sizes by the existing `GATE2_MAX_UNITS` constant (the shipped
  grouping already does this); the adjacency loop bounds by the actual unit count.
- Width cap: the adjacency row `Adj = Bits<64>` holds up to 64 units, matching
  the runtime `D::AdjRow` default. A consumer needing > 64 units sets a wider
  `D::AdjRow` (the same constraint the runtime waist path already has); a wider
  `Bits` const-`BitAccess` would be its own arvo round (R1c) if/when needed, not
  on the R2 path.

**Unblocks R2.** The full R2 mechanism (mask fold -> unit adjacency ->
`waist_detect_const` -> canonical section-index phases) is proven const over real
arvo types. R2 in `plan/grouping.rs`: add a const `compute_phases_waist` that
builds the unit adjacency from the `BundleMasks`-folded `AccessMask` arrays, calls
`waist_detect_const`, prefix-counts to phases, and replaces the depth
`compute_phases` as the phase axis. `compute_trunks` (within-phase column-conflict
union-find) stays, keyed on the new waist-phase. Update
`tests/gate2_const_grouping.rs` to the waist-based expectations; a producer ->
consumer chain has no interior waist -> one phase -> `morsel_outer.rs` stays green.
