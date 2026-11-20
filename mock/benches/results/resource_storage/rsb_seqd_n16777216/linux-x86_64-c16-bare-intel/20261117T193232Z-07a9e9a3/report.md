# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 210% faster than the next best (seq_snapshot)

seq_live (4.59 ms) leads seq_snapshot (14.22 ms) by 210%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (4.59 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest seq_live (4.59 ms) to slowest seq_snapshot (14.22 ms): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (seq_live) is the fastest** at 4585676.1 ns median
- 1 variant significantly slower than baseline
- Spread: 3.10x (fastest 4585676.1 ns, slowest 14224089.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Seq collection member: live-stream vs snapshot-copy across payload size |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 4596109ns | 4586898ns | 4554405ns | 4587644ns | 4663206ns | base |
| seq_snapshot | 14235123ns | 14225390ns | 14145539ns | 14224453ns | 14356717ns | +209.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 4594789ns | 4553137ns | 4661865ns | base | 3.651 |
| seq_snapshot | 14233745ns | 14144219ns | 14355297ns | +209.78% | 1.179 |

## Performance model

- Peak throughput: **3.685 Gops/s** (seq_live; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 3.659 | 99.3% |
| seq_snapshot | 1.179 | 32.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 4596109ns | 4596109ns | base |
| seq_snapshot | 14235123ns | 14235123ns | +209.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 4585676ns | base | --- | [4580804, 4590625] | --- | --- | --- | --- |
| seq_snapshot | 14224089ns | +9646558.2ns (+210.4%) | [+9619110, +9655150]ns | [14209686, 14237986] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 4574479ns | +211.9% |
| 1 | 2 | 4610052ns | +209.1% |
| 1 | 3 | 4580949ns | +209.1% |
| 1 | 4 | 4552248ns | +212.4% |
| 1 | 5 | 4606258ns | +209.3% |
| 2 | 1 | 4588992ns | +210.8% |
| 2 | 2 | 4554979ns | +210.7% |
| 2 | 3 | 4586417ns | +210.4% |
| 2 | 4 | 4602204ns | +208.9% |
| 2 | 5 | 4576387ns | +210.9% |
| 3 | 1 | 4595919ns | +209.2% |
| 3 | 2 | 4610226ns | +207.6% |
| 3 | 3 | 4597580ns | +208.3% |
| 3 | 4 | 4563751ns | +211.5% |
| 3 | 5 | 4565891ns | +212.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.091 | ok |
| seq_snapshot | -0.085 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 27198406.1ns | 4594788.5ns | 591.9% | HIGH |
| seq_snapshot | 27349091.2ns | 14233744.7ns | 192.1% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 4553137.3-4661865.3 ns)
  4553137.3 |###############
  4558573.7 |###############
  4564010.1 |#####################
  4569446.5 |########################################
  4574882.9 |#####################
  4580319.3 |########################################
  4585755.7 |#################################
  4591192.1 |###############
  4596628.5 |####################################
  4602064.9 |#########
  4607501.3 |#####################
  4612937.7 |#########
  4618374.1 |#########
  4623810.5 |
  4629246.9 |#########
  4634683.3 |######
  4640119.7 |######
  4645556.1 |
  4650992.5 |
  4656428.9 |
  (11 below, 8 above range)

seq_snapshot (n=120, range 14144219.2-14355296.9 ns)
  14144219.2 |################
  14154773.1 |########################
  14165327.0 |####################
  14175880.9 |############################
  14186434.7 |########################################
  14196988.6 |############
  14207542.5 |########################################
  14218096.4 |####################
  14228650.3 |########################################
  14239204.2 |############################
  14249758.1 |########################
  14260311.9 |################################
  14270865.8 |################################
  14281419.7 |############
  14291973.6 |########
  14302527.5 |
  14313081.4 |############
  14323635.2 |
  14334189.1 |
  14344743.0 |####
  (12 below, 10 above range)

```

## Diagnostics

- **seq_live**: bridge=593.7% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=192.3% of algo (FFI overhead may distort results)
