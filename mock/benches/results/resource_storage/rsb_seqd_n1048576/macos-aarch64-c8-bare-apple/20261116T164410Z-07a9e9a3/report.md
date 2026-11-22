# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 22% faster than the next best (seq_snapshot)

seq_live (330.49 us) leads seq_snapshot (404.20 us) by 22%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (330.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 330486.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 330486.6 ns, slowest 404204.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 333162ns | 331901ns | 330674ns | 332068ns | 338932ns | base |
| seq_snapshot | 404348ns | 404283ns | 400929ns | 404187ns | 408251ns | +21.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 331683ns | 329203ns | 337653ns | base | 3.161 |
| seq_snapshot | 404277ns | 400862ns | 408180ns | +21.89% | 2.594 |

## Performance model

- Peak throughput: **3.185 Gops/s** (seq_live; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 3.173 | 99.6% |
| seq_snapshot | 2.594 | 81.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 333162ns | 333162ns | base |
| seq_snapshot | 404348ns | 404348ns | +21.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 330487ns | base | --- | [329956, 330829] | --- | --- | --- | --- |
| seq_snapshot | 404204ns | +72763.2ns (+22.0%) | [+72002, +74078]ns | [402704, 405327] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 330218ns | +23.4% |
| 1 | 2 | 335388ns | +21.3% |
| 1 | 3 | 329612ns | +21.7% |
| 1 | 4 | 330433ns | +23.4% |
| 1 | 5 | 330036ns | +21.8% |
| 2 | 1 | 329628ns | +21.8% |
| 2 | 2 | 330834ns | +22.1% |
| 2 | 3 | 330581ns | +22.6% |
| 2 | 4 | 329929ns | +21.8% |
| 2 | 5 | 329948ns | +23.4% |
| 3 | 1 | 330266ns | +21.9% |
| 3 | 2 | 341281ns | +17.7% |
| 3 | 3 | 330850ns | +23.0% |
| 3 | 4 | 332540ns | +20.6% |
| 3 | 5 | 329467ns | +21.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.143 | ok |
| seq_snapshot | -0.284 | moderate- |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 1719642.5ns | 331682.5ns | 518.5% | HIGH |
| seq_snapshot | 1719420.5ns | 404277.4ns | 425.3% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 329202.5-337653.4 ns)
  329202.5 |########################################
  329625.1 |####################################
  330047.6 |###########
  330470.2 |################################
  330892.7 |#############
  331315.2 |###########
  331737.8 |#########
  332160.3 |###
  332582.9 |#######
  333005.4 |#####
  333428.0 |###
  333850.5 |#
  334273.1 |#
  334695.6 |
  335118.2 |#######
  335540.7 |###
  335963.3 |
  336385.8 |
  336808.3 |
  337230.9 |
  (13 below, 7 above range)

seq_snapshot (n=120, range 400861.7-408179.7 ns)
  400861.7 |########################################
  401227.6 |###########################
  401593.5 |##########
  401959.4 |####
  402325.3 |####
  402691.2 |####
  403057.1 |########
  403423.0 |##
  403788.9 |########
  404154.8 |########
  404520.7 |########
  404886.6 |######
  405252.5 |
  405618.4 |####
  405984.3 |########
  406350.2 |########
  406716.1 |##############
  407082.0 |###########################
  407447.9 |#######################
  407813.8 |########
  (8 below, 4 above range)

```

## Diagnostics

- **seq_live**: bridge=517.6% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=425.6% of algo (FFI overhead may distort results)
