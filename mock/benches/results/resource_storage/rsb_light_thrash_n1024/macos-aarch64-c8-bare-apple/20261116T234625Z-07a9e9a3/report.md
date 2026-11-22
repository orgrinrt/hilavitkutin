# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v0_blob is fastest but the noisiest (CV 29.2%)

v0_blob wins on median (28 ns) yet has the highest variance (CV 29.2%), while v4_erased is the steadiest (CV 4.3%, 29 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (v0_blob, v3_shapebound) are a dead heat (<1%)

v0_blob (28 ns) and v3_shapebound (28 ns) differ by 0.00%, inside the noise, even though the wider field spreads 10.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (8 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (28 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader v0_blob vs stability leader v4_erased (+3% speed for 6.8x steadier)

v0_blob is fastest (28 ns, CV 29.2%); v4_erased gives up 3.2% median for 6.8x lower variance (CV 4.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (v0_blob) is the fastest** at 27.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.10x (fastest 27.8 ns, slowest 30.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under light surrounding work |
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
| v0_blob | 159ns | 158ns | 154ns | 158ns | 169ns | base |
| v1_snapshot | 161ns | 160ns | 154ns | 160ns | 169ns | +0.86% |
| v2_decomposed | 161ns | 161ns | 157ns | 160ns | 166ns | +0.94% |
| v3_shapebound | 157ns | 157ns | 154ns | 157ns | 162ns | -1.18% |
| v4_erased | 160ns | 158ns | 154ns | 159ns | 168ns | +0.21% |
| v5_handletable | 161ns | 159ns | 155ns | 159ns | 170ns | +0.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 29ns | 26ns | 33ns | base | 35.767 |
| v1_snapshot | 28ns | 26ns | 32ns | -0.54% | 35.961 |
| v2_decomposed | 28ns | 26ns | 30ns | -1.92% | 36.466 |
| v3_shapebound | 28ns | 26ns | 30ns | -2.71% | 36.764 |
| v4_erased | 28ns | 27ns | 30ns | -0.72% | 36.026 |
| v5_handletable | 31ns | 29ns | 34ns | +8.30% | 33.025 |

## Performance model

- Peak throughput: **39.340 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 36.835 | 93.6% |
| v1_snapshot | 36.312 | 92.3% |
| v2_decomposed | 36.571 | 93.0% |
| v3_shapebound | 36.835 | 93.6% |
| v4_erased | 35.679 | 90.7% |
| v5_handletable | 33.355 | 84.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 159ns | 159ns | base |
| v1_snapshot | 161ns | 161ns | +0.86% |
| v2_decomposed | 161ns | 161ns | +0.94% |
| v3_shapebound | 157ns | 157ns | -1.18% |
| v4_erased | 160ns | 160ns | +0.21% |
| v5_handletable | 161ns | 161ns | +0.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 28ns | base | --- | [28, 28] | --- | --- | --- | --- |
| v1_snapshot | 28ns | no significant difference | [-0, +1]ns | [28, 28] | no | 0.1331 | 0.0798 | 2 |
| v2_decomposed | 28ns | no significant difference | [-0, +1]ns | [28, 28] | no | 0.2442 | 0.1953 | 3 |
| v3_shapebound | 28ns | no significant difference | [-0, +0]ns | [28, 28] | no | 1.0000 | 1.0000 | 7 |
| v4_erased | 29ns | +0.8ns (+2.7%) | [+0, +1]ns | [28, 29] | YES | 0.0067 | 0.0027 | 5 |
| v5_handletable | 31ns | +2.8ns (+10.1%) | [+2, +3]ns | [30, 31] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 28ns | +5.6% | +1.8% | -0.4% | +0.0% | +8.3% |
| 1 | 2 | 28ns | +0.5% | -3.2% | +0.4% | -2.3% | +6.2% |
| 1 | 3 | 28ns | -1.1% | -2.0% | -1.3% | +3.8% | +8.2% |
| 1 | 4 | 28ns | -2.3% | +1.8% | -2.3% | +4.6% | +12.7% |
| 1 | 5 | 28ns | -5.3% | +3.2% | -1.6% | +3.4% | +9.9% |
| 2 | 1 | 28ns | +3.6% | +1.1% | -2.0% | +4.2% | +7.3% |
| 2 | 2 | 28ns | +0.9% | +1.6% | +0.4% | +6.1% | +7.6% |
| 2 | 3 | 29ns | -4.1% | -4.4% | -4.8% | -4.3% | +6.1% |
| 2 | 4 | 27ns | +5.1% | +1.5% | +5.1% | +2.7% | +11.9% |
| 2 | 5 | 27ns | +4.2% | +5.7% | +4.0% | +4.0% | +20.2% |
| 3 | 1 | 28ns | -3.1% | +2.2% | -0.5% | +2.0% | +11.0% |
| 3 | 2 | 28ns | -0.4% | -1.4% | +0.7% | +2.7% | +16.5% |
| 3 | 3 | 28ns | +0.4% | -1.3% | -1.8% | +1.3% | +6.8% |
| 3 | 4 | 28ns | +3.1% | +2.9% | +1.5% | +4.4% | +10.9% |
| 3 | 5 | 27ns | +3.3% | +3.3% | -2.0% | +3.8% | +13.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.154 | ok |
| v1_snapshot | 0.157 | ok |
| v2_decomposed | 0.090 | ok |
| v3_shapebound | 0.099 | ok |
| v4_erased | 0.337 | moderate+ |
| v5_handletable | 0.020 | ok |

**Consistency summary:**

- **v1_snapshot**: won 6/15, lost 9/15
- **v2_decomposed**: won 5/15, lost 10/15
- **v3_shapebound**: won 9/15, lost 6/15
- **v4_erased**: won 2/15, lost 12/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 1982.6ns | 28.6ns | 6924.7% | HIGH |
| v1_snapshot | 1986.6ns | 28.5ns | 6976.5% | HIGH |
| v2_decomposed | 2000.1ns | 28.1ns | 7122.6% | HIGH |
| v3_shapebound | 1978.2ns | 27.9ns | 7102.3% | HIGH |
| v4_erased | 1976.9ns | 28.4ns | 6954.9% | HIGH |
| v5_handletable | 2055.0ns | 31.0ns | 6627.5% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 26.2-33.4 ns)
     26.2 |#####################
     26.6 |##########
     26.9 |########################################
     27.3 |################
     27.7 |########################################
     28.0 |#####################
     28.4 |################
     28.8 |##############
     29.1 |################
     29.5 |##########
     29.8 |####
     30.2 |##########
     30.6 |####
     30.9 |
     31.3 |
     31.6 |##
     32.0 |
     32.4 |
     32.7 |
     33.1 |
  (10 below, 1 above range)

v1_snapshot (n=120, range 26.1-31.9 ns)
     26.1 |##############
     26.4 |######################
     26.7 |#################
     27.0 |###########
     27.3 |#########################
     27.6 |#########################
     27.9 |###########
     28.2 |##################################
     28.5 |########################################
     28.7 |########################################
     29.0 |######################
     29.3 |#####
     29.6 |##############
     29.9 |########
     30.2 |##
     30.5 |##
     30.8 |
     31.1 |##
     31.4 |##
     31.7 |#####
  (10 below, 1 above range)

v2_decomposed (n=120, range 26.3-29.8 ns)
     26.3 |
     26.5 |##########
     26.7 |#######################
     26.8 |
     27.0 |
     27.2 |########################################
     27.4 |####################
     27.5 |################
     27.7 |##########################
     27.9 |##############################
     28.1 |##########
     28.2 |#################################
     28.4 |##############################
     28.6 |######
     28.8 |####################
     28.9 |####################
     29.1 |#############
     29.3 |###
     29.5 |###
     29.6 |##############################
  (11 below, 8 above range)

v3_shapebound (n=120, range 26.0-29.8 ns)
     26.0 |##################
     26.2 |#########################
     26.4 |#######
     26.6 |#######
     26.8 |##############
     27.0 |##########
     27.1 |########################################
     27.3 |#############################
     27.5 |#####################
     27.7 |#############################
     27.9 |################################
     28.1 |#############################
     28.3 |#######
     28.5 |#####################
     28.6 |##################
     28.8 |##################
     29.0 |
     29.2 |##################
     29.4 |##############
     29.6 |#######
  (8 below, 10 above range)

v4_erased (n=120, range 26.6-30.0 ns)
     26.6 |#########
     26.7 |#######
     26.9 |#########
     27.1 |####
     27.3 |##############
     27.4 |####
     27.6 |##############
     27.8 |###########
     28.0 |###########
     28.1 |################
     28.3 |
     28.5 |###########
     28.7 |########################################
     28.8 |
     29.0 |###########
     29.2 |#########################
     29.3 |##################
     29.5 |
     29.7 |##################
     29.9 |#######
  (9 below, 10 above range)

v5_handletable (n=120, range 28.9-34.2 ns)
     28.9 |########
     29.1 |############################
     29.4 |#################
     29.7 |############################
     29.9 |###############################
     30.2 |#####
     30.5 |############################
     30.7 |########################################
     31.0 |######################
     31.3 |####################
     31.6 |####################
     31.8 |###########
     32.1 |#################
     32.4 |########
     32.6 |#####
     32.9 |#####
     33.2 |#####
     33.4 |##
     33.7 |#####
     34.0 |
  (9 below, 1 above range)

```

## Diagnostics

- **v0_blob**: CV=28.4% (high variance, measurements may be unstable)
- **v0_blob**: bridge=7072.7% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6973.0% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7027.5% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7074.1% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6851.2% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6629.0% of algo (FFI overhead may distort results)
