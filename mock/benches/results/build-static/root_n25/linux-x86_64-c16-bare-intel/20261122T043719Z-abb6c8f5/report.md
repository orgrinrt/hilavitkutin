# build-static: rebuilt after an edit to one root contract crate

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (8852.41 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 666.02 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### llvm_incremental dominates: 1229% faster than the next best (llvm_non_incremental)

llvm_incremental (666.02 ms) leads llvm_non_incremental (8852.41 ms) by 1229%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### llvm_incremental beats baseline by 93% (significant)

llvm_incremental is -8189.48 ms (93%) faster than baseline llvm_non_incremental, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.78)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 13.3x the fastest

Fastest llvm_incremental (666.02 ms) to slowest llvm_non_incremental (8852.41 ms): 13.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: llvm_incremental** at 666020525.0 ns median (-92.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 13.29x (fastest 666020525.0 ns, slowest 8852411010.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: rebuilt after an edit to one root contract crate |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2112699331 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 664487122ns | 666022779ns | 654183925ns | 663142597ns | 671655509ns | -92.49% |
| llvm_non_incremental | 8848272611ns | 8852413347ns | 8573151040ns | 8808089073ns | 9046108705ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 664484934ns | 654182093ns | 671653071ns | -92.49% |
| llvm_non_incremental | 8848270449ns | 8573148909ns | 9046106517ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 664487122ns | 664487122ns | -92.49% |
| llvm_non_incremental | 8848272611ns | 8848272611ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 8852411011ns | base | --- | [8646293820, 9046106517] | --- | --- | --- | --- |
| llvm_incremental | 666020525ns | -8189478514.4ns (-92.5%) | [-8374526328, -7987351703]ns | [655781207, 671653071] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 8573148909ns | -92.3% |
| 1 | 2 | 9036923531ns | -92.6% |
| 1 | 3 | 8754410854ns | -92.5% |
| 1 | 4 | 9055289502ns | -92.6% |
| 1 | 5 | 8719438732ns | -92.4% |
| 1 | 6 | 8950411167ns | -92.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.780 | HIGH- (thermal bounce) |
| llvm_non_incremental | -0.678 | HIGH- (thermal bounce) |

**Consistency summary:**

- **llvm_incremental**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 46115908.0ns | 664484934.2ns | 6.9% | HIGH |
| llvm_non_incremental | 46270695.6ns | 8848270449.2ns | 0.5% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 654182092.7-671653070.8 ns)
  654182092.7 |####################
  655055641.6 |
  655929190.5 |
  656802739.4 |####################
  657676288.3 |
  658549837.2 |
  659423386.1 |
  660296935.0 |####################
  661170483.9 |
  662044032.8 |
  662917581.7 |
  663791130.6 |
  664664679.5 |
  665538228.4 |
  666411777.3 |
  667285326.2 |
  668158875.1 |
  669032424.0 |
  669905972.9 |
  670779521.8 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 8573148908.6-9046106516.7 ns)
  8573148908.6 |########################################
  8596796789.0 |
  8620444669.4 |
  8644092549.8 |
  8667740430.2 |
  8691388310.6 |
  8715036191.0 |########################################
  8738684071.4 |########################################
  8762331951.8 |
  8785979832.2 |
  8809627712.7 |
  8833275593.1 |
  8856923473.5 |
  8880571353.9 |
  8904219234.3 |
  8927867114.7 |########################################
  8951514995.1 |
  8975162875.5 |
  8998810755.9 |
  9022458636.3 |########################################
  (0 below, 1 above range)

```
