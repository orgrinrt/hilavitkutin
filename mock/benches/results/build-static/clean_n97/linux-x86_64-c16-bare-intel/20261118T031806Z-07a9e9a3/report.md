# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (68073.80 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 68073803987.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 68073803987.9 ns, slowest 71875078786.5 ns)

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
| Counter frequency | 2112687708 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 71837402921ns | 71875081113ns | 71509998905ns | 71787735164ns | 72075606564ns | +5.52% |
| llvm_non_incremental | 68078567416ns | 68073805944ns | 67671485572ns | 68046932617ns | 68329560535ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 71837400712ns | 71509997031ns | 72075604071ns | +5.52% |
| llvm_non_incremental | 68078565292ns | 67671483398ns | 68329558481ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 71837402921ns | 71837402921ns | +5.52% |
| llvm_non_incremental | 68078567416ns | 68078567416ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 68073803988ns | base | --- | [67832333408, 68329558481] | --- | --- | --- | --- |
| llvm_incremental | 71875078786ns | +3831162323.9ns (+5.6%) | [+3231960798, +4213383138]ns | [71561519279, 72075604071] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 67671483398ns | +6.7% |
| 1 | 2 | 68363253240ns | +4.8% |
| 1 | 3 | 68295863722ns | +4.7% |
| 1 | 4 | 68083889500ns | +5.6% |
| 1 | 5 | 67993183417ns | +5.7% |
| 1 | 6 | 68063718476ns | +5.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.073 | ok |
| llvm_non_incremental | -0.173 | ok |

**Consistency summary:**

- **llvm_incremental**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 299706119.3ns | 71837400712.2ns | 0.4% |  |
| llvm_non_incremental | 190629031.9ns | 68078565292.1ns | 0.3% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 71509997031.4-72075604071.0 ns)
  71509997031.4 |########################################
  71538277383.4 |
  71566557735.4 |
  71594838087.3 |########################################
  71623118439.3 |
  71651398791.3 |
  71679679143.3 |
  71707959495.3 |
  71736239847.2 |
  71764520199.2 |
  71792800551.2 |
  71821080903.2 |
  71849361255.2 |########################################
  71877641607.1 |########################################
  71905921959.1 |########################################
  71934202311.1 |
  71962482663.1 |
  71990763015.1 |
  72019043367.0 |
  72047323719.0 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 67671483398.3-68329558481.1 ns)
  67671483398.3 |########################################
  67704387152.4 |
  67737290906.6 |
  67770194660.7 |
  67803098414.9 |
  67836002169.0 |
  67868905923.1 |
  67901809677.3 |
  67934713431.4 |
  67967617185.5 |########################################
  68000520939.7 |
  68033424693.8 |########################################
  68066328448.0 |########################################
  68099232202.1 |
  68132135956.2 |
  68165039710.4 |
  68197943464.5 |
  68230847218.6 |
  68263750972.8 |########################################
  68296654726.9 |
  (0 below, 1 above range)

```
