# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### llvm_non_incremental dominates: 11% faster than the next best (llvm_incremental)

llvm_non_incremental (19350.54 ms) leads llvm_incremental (21564.30 ms) by 11%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (19350.54 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 19350543432.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.11x (fastest 19350543432.9 ns, slowest 21564295493.8 ns)

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
| Counter frequency | 2100505716 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 21040652267ns | 21564300513ns | 19044432393ns | 20868876469ns | 22296425900ns | +9.66% |
| llvm_non_incremental | 19187189476ns | 19350548255ns | 18402949077ns | 19134336427ns | 19658589247ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 21040647339ns | 19044427450ns | 22296420657ns | +9.66% |
| llvm_non_incremental | 19187184756ns | 18402944001ns | 19658584545ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 21040652267ns | 21040652267ns | +9.66% |
| llvm_non_incremental | 19187189476ns | 19187189476ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 19350543433ns | base | --- | [18552426291, 19658584545] | --- | --- | --- | --- |
| llvm_incremental | 21564295494ns | +2389267014.0ns (+12.3%) | [+225243511, +2945877224]ns | [19261225868, 22296420657] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 19470737630ns | +14.4% |
| 1 | 2 | 19669020712ns | -1.0% |
| 1 | 3 | 18402944001ns | +3.5% |
| 1 | 4 | 19230349236ns | +16.0% |
| 1 | 5 | 19648148378ns | +12.5% |
| 1 | 6 | 18701908581ns | +12.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.002 | ok |
| llvm_non_incremental | -0.348 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 85333516.9ns | 21040647339.4ns | 0.4% |  |
| llvm_non_incremental | 45096441.4ns | 19187184756.5ns | 0.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 19044427449.5-22296420656.7 ns)
  19044427449.5 |########################################
  19207027109.9 |
  19369626770.2 |########################################
  19532226430.6 |
  19694826090.9 |
  19857425751.3 |
  20020025411.6 |
  20182625072.0 |
  20345224732.4 |
  20507824392.7 |
  20670424053.1 |
  20833023713.4 |
  20995623373.8 |########################################
  21158223034.1 |
  21320822694.5 |
  21483422354.9 |
  21646022015.2 |
  21808621675.6 |
  21971221335.9 |########################################
  22133820996.3 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 18402944000.9-19658584545.3 ns)
  18402944000.9 |########################################
  18465726028.1 |
  18528508055.3 |
  18591290082.6 |
  18654072109.8 |########################################
  18716854137.0 |
  18779636164.2 |
  18842418191.4 |
  18905200218.7 |
  18967982245.9 |
  19030764273.1 |
  19093546300.3 |
  19156328327.5 |
  19219110354.8 |########################################
  19281892382.0 |
  19344674409.2 |
  19407456436.4 |
  19470238463.6 |########################################
  19533020490.9 |
  19595802518.1 |########################################
  (0 below, 1 above range)

```
