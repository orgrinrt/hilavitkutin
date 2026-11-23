# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 234% faster than the next best (seq_snapshot)

seq_live (1.10 ms) leads seq_snapshot (3.67 ms) by 234%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (1.10 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest seq_live (1.10 ms) to slowest seq_snapshot (3.67 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (seq_live) is the fastest** at 1098161.9 ns median
- 1 variant significantly slower than baseline
- Spread: 3.34x (fastest 1098161.9 ns, slowest 3668587.5 ns)

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
| Counter frequency | 2112730567 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 1105319ns | 1098672ns | 1082909ns | 1099230ns | 1145997ns | base |
| seq_snapshot | 3655695ns | 3669713ns | 3556625ns | 3666798ns | 3721453ns | +230.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 1104744ns | 1082356ns | 1145410ns | base | 3.797 |
| seq_snapshot | 3654533ns | 3555466ns | 3720267ns | +230.80% | 1.148 |

## Performance model

- Peak throughput: **3.875 Gops/s** (seq_live; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 3.819 | 98.6% |
| seq_snapshot | 1.143 | 29.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 1105319ns | 1105319ns | base |
| seq_snapshot | 3655695ns | 3655695ns | +230.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 1098162ns | base | --- | [1096357, 1099735] | --- | --- | --- | --- |
| seq_snapshot | 3668587ns | +2568418.7ns (+233.9%) | [+2563493, +2572946]ns | [3660426, 3671734] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 1103869ns | +234.2% |
| 1 | 2 | 1093550ns | +234.2% |
| 1 | 3 | 1096286ns | +233.4% |
| 1 | 4 | 1101294ns | +230.8% |
| 1 | 5 | 1097087ns | +235.3% |
| 2 | 1 | 1098747ns | +231.8% |
| 2 | 2 | 1124278ns | +227.3% |
| 2 | 3 | 1068124ns | +241.3% |
| 2 | 4 | 1091480ns | +237.4% |
| 2 | 5 | 1094774ns | +235.2% |
| 3 | 1 | 1097624ns | +235.0% |
| 3 | 2 | 1099338ns | +234.9% |
| 3 | 3 | 1096643ns | +208.4% |
| 3 | 4 | 1107143ns | +228.8% |
| 3 | 5 | 1091117ns | +236.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | -0.359 | moderate- |
| seq_snapshot | -0.064 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 5331702.6ns | 1104744.0ns | 482.6% | HIGH |
| seq_snapshot | 7032409.7ns | 3654533.2ns | 192.4% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 1082355.9-1145409.8 ns)
  1082355.9 |
  1085508.6 |######
  1088661.3 |####################
  1091814.0 |########################################
  1094966.7 |######################################
  1098119.4 |##################################
  1101272.1 |################
  1104424.8 |########################
  1107577.5 |######
  1110730.2 |##########
  1113882.9 |##
  1117035.6 |
  1120188.2 |##
  1123340.9 |####
  1126493.6 |
  1129646.3 |####
  1132799.0 |
  1135951.7 |##
  1139104.4 |
  1142257.1 |
  (8 below, 8 above range)

seq_snapshot (n=120, range 3555466.0-3720267.0 ns)
  3555466.0 |
  3563706.1 |
  3571946.1 |
  3580186.2 |
  3588426.2 |
  3596666.3 |
  3604906.3 |
  3613146.4 |
  3621386.4 |##
  3629626.5 |####
  3637866.5 |###################################
  3646106.6 |#################################
  3654346.6 |####################
  3662586.7 |#####################################
  3670826.7 |########################################
  3679066.8 |#####################################
  3687306.8 |###############
  3695546.9 |##
  3703786.9 |
  3712027.0 |######
  (8 below, 6 above range)

```

## Diagnostics

- **seq_live**: bridge=484.8% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=191.4% of algo (FFI overhead may distort results)
