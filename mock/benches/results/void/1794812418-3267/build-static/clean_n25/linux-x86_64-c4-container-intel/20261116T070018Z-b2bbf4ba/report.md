# build-static: every crate built into an empty target directory

2 variants, 3 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### llvm_non_incremental shows alternating (throttle bounce) (autocorr -0.64)

llvm_non_incremental's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (16426.10 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 16426098303.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 16426098303.0 ns, slowest 17371481895.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 3 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2300280178 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 17010370538ns | 17371504811ns | 15705556015ns | -0ns | 17010370538ns | +4.74% |
| llvm_non_incremental | 16240569981ns | 16426101573ns | 15656099579ns | -0ns | 16240569981ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 17010360205ns | 15705552002ns | 17010360205ns | +4.74% |
| llvm_non_incremental | 16240566402ns | 15656095643ns | 16240566402ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 17010370538ns | 17010370538ns | +4.74% |
| llvm_non_incremental | 16240569981ns | 16240569981ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 16426098303ns | base | --- | [15656095643, 16639505260] | --- | --- | --- | --- |
| llvm_incremental | 17371481896ns | +945383592.9ns (+5.8%) | [+49456358, +1314541458]ns | [15705552002, 17954046718] | YES (adj: no) | 0.2500 | 0.2500 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 16426098303ns | +5.8% |
| 1 | 2 | 15656095643ns | +0.3% |
| 1 | 3 | 16639505260ns | +7.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.625 | HIGH- (thermal bounce) |
| llvm_non_incremental | -0.638 | HIGH- (thermal bounce) |

**Consistency summary:**

- **llvm_incremental**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 155492944.5ns | 17010360205.0ns | 0.9% |  |
| llvm_non_incremental | 33257277.5ns | 16240566402.0ns | 0.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=3, range 15705552001.7-17010360205.0 ns)
  15705552001.7 |########################################
  15770792411.9 |
  15836032822.0 |
  15901273232.2 |
  15966513642.4 |
  16031754052.5 |
  16096994462.7 |
  16162234872.9 |
  16227475283.0 |
  16292715693.2 |
  16357956103.4 |
  16423196513.5 |
  16488436923.7 |
  16553677333.9 |
  16618917744.0 |
  16684158154.2 |
  16749398564.4 |
  16814638974.5 |
  16879879384.7 |
  16945119794.9 |
  (0 below, 2 above range)

llvm_non_incremental (n=3, range 15656095643.4-16240566402.0 ns)
  15656095643.4 |########################################
  15685319181.3 |
  15714542719.3 |
  15743766257.2 |
  15772989795.1 |
  15802213333.0 |
  15831436871.0 |
  15860660408.9 |
  15889883946.8 |
  15919107484.8 |
  15948331022.7 |
  15977554560.6 |
  16006778098.5 |
  16036001636.5 |
  16065225174.4 |
  16094448712.3 |
  16123672250.3 |
  16152895788.2 |
  16182119326.1 |
  16211342864.0 |
  (0 below, 2 above range)

```
