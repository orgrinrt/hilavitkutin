# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### llvm_non_incremental shows alternating (throttle bounce) (autocorr -0.50)

llvm_non_incremental's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (68956.43 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 68956431705.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 68956431705.4 ns, slowest 72764162413.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2112792157 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 72800387857ns | 72764167084ns | 72482997548ns | 72714475863ns | 73087951001ns | +5.57% |
| llvm_non_incremental | 68959579945ns | 68956434194ns | 68765735628ns | 68911231366ns | 69129024972ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 72800385139ns | 72482995424ns | 73087949428ns | +5.57% |
| llvm_non_incremental | 68959577775ns | 68765733726ns | 69129022867ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 72800387857ns | 72800387857ns | +5.57% |
| llvm_non_incremental | 68959579945ns | 68959579945ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 68956431705ns | base | --- | [68793278751, 69129022867] | --- | --- | --- | --- |
| llvm_incremental | 72764162414ns | +3841522170.8ns (+5.6%) | [+3518076437, +4162823485]ns | [72549043575, 73087949428] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 68959728744ns | +5.1% |
| 1 | 2 | 68765733726ns | +6.0% |
| 1 | 3 | 69155840202ns | +5.6% |
| 1 | 4 | 68953134667ns | +6.1% |
| 1 | 5 | 68820823776ns | +5.5% |
| 1 | 6 | 69102205532ns | +5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | 0.124 | ok |
| llvm_non_incremental | -0.503 | HIGH- (thermal bounce) |

**Consistency summary:**

- **llvm_incremental**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 285236768.7ns | 72800385138.7ns | 0.4% |  |
| llvm_non_incremental | 183905895.1ns | 68959577774.6ns | 0.3% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 72482995423.6-73087949427.5 ns)
  72482995423.6 |########################################
  72513243123.8 |
  72543490824.0 |
  72573738524.2 |
  72603986224.4 |########################################
  72634233924.6 |########################################
  72664481624.8 |
  72694729325.0 |
  72724977025.2 |
  72755224725.4 |
  72785472425.6 |
  72815720125.7 |
  72845967825.9 |
  72876215526.1 |########################################
  72906463226.3 |
  72936710926.5 |
  72966958626.7 |
  72997206326.9 |########################################
  73027454027.1 |
  73057701727.3 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 68765733726.5-69129022867.3 ns)
  68765733726.5 |####################
  68783898183.5 |
  68802062640.6 |
  68820227097.6 |####################
  68838391554.7 |
  68856556011.7 |
  68874720468.7 |
  68892884925.8 |
  68911049382.8 |
  68929213839.9 |
  68947378296.9 |########################################
  68965542753.9 |
  68983707211.0 |
  69001871668.0 |
  69020036125.1 |
  69038200582.1 |
  69056365039.1 |
  69074529496.2 |
  69092693953.2 |####################
  69110858410.3 |
  (0 below, 1 above range)

```
