# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 234% faster than the next best (seq_snapshot)

seq_live (1.10 ms) leads seq_snapshot (3.68 ms) by 234%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (1.10 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest seq_live (1.10 ms) to slowest seq_snapshot (3.68 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (seq_live) is the fastest** at 1101306.2 ns median
- 1 variant significantly slower than baseline
- Spread: 3.34x (fastest 1101306.2 ns, slowest 3676609.9 ns)

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
| seq_live | 1109987ns | 1101884ns | 1080128ns | 1107638ns | 1146892ns | base |
| seq_snapshot | 3670482ns | 3677774ns | 3539108ns | 3679051ns | 3776151ns | +230.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 1109408ns | 1079621ns | 1146320ns | base | 3.781 |
| seq_snapshot | 3669355ns | 3537962ns | 3775018ns | +230.75% | 1.143 |

## Performance model

- Peak throughput: **3.885 Gops/s** (seq_live; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 3.808 | 98.0% |
| seq_snapshot | 1.141 | 29.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 1109987ns | 1109987ns | base |
| seq_snapshot | 3670482ns | 3670482ns | +230.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 1101306ns | base | --- | [1098604, 1110328] | --- | --- | --- | --- |
| seq_snapshot | 3676610ns | +2563378.0ns (+232.8%) | [+2556976, +2568940]ns | [3671579, 3685380] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 1081647ns | +238.0% |
| 1 | 2 | 1087091ns | +240.1% |
| 1 | 3 | 1126760ns | +226.6% |
| 1 | 4 | 1117377ns | +229.0% |
| 1 | 5 | 1133530ns | +224.9% |
| 2 | 1 | 1098739ns | +235.5% |
| 2 | 2 | 1083283ns | +208.0% |
| 2 | 3 | 1084788ns | +243.3% |
| 2 | 4 | 1136584ns | +225.3% |
| 2 | 5 | 1099136ns | +230.6% |
| 3 | 1 | 1115295ns | +232.5% |
| 3 | 2 | 1099695ns | +232.7% |
| 3 | 3 | 1127744ns | +225.5% |
| 3 | 4 | 1094035ns | +235.4% |
| 3 | 5 | 1101715ns | +233.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.051 | ok |
| seq_snapshot | -0.233 | moderate- |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 5339182.7ns | 1109407.5ns | 481.3% | HIGH |
| seq_snapshot | 7010196.5ns | 3669354.5ns | 191.0% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 1079620.8-1146320.1 ns)
  1079620.8 |########
  1082955.7 |########
  1086290.7 |#######################
  1089625.7 |##########
  1092960.6 |########
  1096295.6 |########################################
  1099630.6 |############
  1102965.5 |######
  1106300.5 |##########
  1109635.5 |####
  1112970.4 |############
  1116305.4 |##
  1119640.4 |##
  1122975.3 |##########
  1126310.3 |##########
  1129645.3 |################
  1132980.2 |########
  1136315.2 |############
  1139650.2 |########
  1142985.2 |
  (9 below, 8 above range)

seq_snapshot (n=120, range 3537962.4-3775017.5 ns)
  3537962.4 |
  3549815.2 |
  3561667.9 |
  3573520.7 |
  3585373.4 |
  3597226.2 |
  3609078.9 |#
  3620931.7 |##########
  3632784.5 |######
  3644637.2 |###############
  3656490.0 |#####################
  3668342.7 |########################################
  3680195.5 |##############################
  3692048.2 |##################
  3703901.0 |#############
  3715753.8 |##########
  3727606.5 |#
  3739459.3 |###
  3751312.0 |#
  3763164.8 |
  (8 below, 8 above range)

```

## Diagnostics

- **seq_live**: bridge=484.0% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=190.7% of algo (FFI overhead may distort results)
