# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (10428.93 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 10428927674.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 10428927674.0 ns, slowest 10947567236.4 ns)

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
| llvm_incremental | 10949705528ns | 10947569962ns | 10844275248ns | 10917221804ns | 11051146254ns | +5.07% |
| llvm_non_incremental | 10421343082ns | 10428930356ns | 10239379689ns | 10417957182ns | 10517403627ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 10949702838ns | 10844273042ns | 11051143566ns | +5.07% |
| llvm_non_incremental | 10421340202ns | 10239377033ns | 10517400510ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 10949705528ns | 10949705528ns | +5.07% |
| llvm_non_incremental | 10421343082ns | 10421343082ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 10428927674ns | base | --- | [10317692422, 10517400510] | --- | --- | --- | --- |
| llvm_incremental | 10947567236ns | +500501309.3ns (+4.8%) | [+364166402, +720420197]ns | [10850397712, 11051143566] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 10568772980ns | +3.2% |
| 1 | 2 | 10396007811ns | +4.3% |
| 1 | 3 | 10435785642ns | +5.3% |
| 1 | 4 | 10466028040ns | +3.7% |
| 1 | 5 | 10422069706ns | +5.9% |
| 1 | 6 | 10239377033ns | +8.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.025 | ok |
| llvm_non_incremental | -0.062 | ok |

**Consistency summary:**

- **llvm_incremental**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 45349829.2ns | 10949702838.2ns | 0.4% |  |
| llvm_non_incremental | 28767372.4ns | 10421340202.0ns | 0.3% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 10844273041.7-11051143566.3 ns)
  10844273041.7 |########################################
  10854616567.9 |########################################
  10864960094.2 |
  10875303620.4 |
  10885647146.6 |
  10895990672.9 |
  10906334199.1 |########################################
  10916677725.3 |
  10927021251.5 |
  10937364777.8 |
  10947708304.0 |
  10958051830.2 |
  10968395356.5 |
  10978738882.7 |########################################
  10989082408.9 |
  10999425935.1 |
  11009769461.4 |
  11020112987.6 |
  11030456513.8 |########################################
  11040800040.1 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 10239377032.6-10517400510.0 ns)
  10239377032.6 |########################################
  10253278206.5 |
  10267179380.3 |
  10281080554.2 |
  10294981728.1 |
  10308882901.9 |
  10322784075.8 |
  10336685249.7 |
  10350586423.5 |
  10364487597.4 |
  10378388771.3 |
  10392289945.1 |########################################
  10406191119.0 |
  10420092292.9 |########################################
  10433993466.7 |########################################
  10447894640.6 |
  10461795814.5 |########################################
  10475696988.3 |
  10489598162.2 |
  10503499336.1 |
  (0 below, 1 above range)

```
