# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (v0_blob, v1_snapshot) are a dead heat (<1%)

v0_blob (28 ns) and v1_snapshot (28 ns) differ by 0.72%, inside the noise, even though the wider field spreads 9.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (28 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader v0_blob vs stability leader v3_shapebound (+1% speed for 1.2x steadier)

v0_blob is fastest (28 ns, CV 5.3%); v3_shapebound gives up 1.1% median for 1.2x lower variance (CV 4.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### v2_decomposed's edge over baseline is significant but tiny (0 ns, 1.79%)

v2_decomposed differs from baseline v0_blob by 0 ns (1.79%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (v0_blob) is the fastest** at 27.9 ns median
- 3 variants significantly slower than baseline
- Spread: 1.09x (fastest 27.9 ns, slowest 30.5 ns)

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
| v0_blob | 157ns | 157ns | 154ns | 157ns | 161ns | base |
| v1_snapshot | 159ns | 158ns | 154ns | 158ns | 169ns | +1.45% |
| v2_decomposed | 159ns | 159ns | 155ns | 159ns | 165ns | +1.29% |
| v3_shapebound | 159ns | 158ns | 155ns | 158ns | 165ns | +0.94% |
| v4_erased | 160ns | 160ns | 155ns | 160ns | 166ns | +1.83% |
| v5_handletable | 160ns | 159ns | 155ns | 159ns | 167ns | +1.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 28ns | 26ns | 30ns | base | 36.912 |
| v1_snapshot | 29ns | 26ns | 35ns | +4.81% | 35.219 |
| v2_decomposed | 28ns | 27ns | 30ns | +2.28% | 36.088 |
| v3_shapebound | 28ns | 26ns | 30ns | +1.14% | 36.494 |
| v4_erased | 28ns | 26ns | 31ns | +2.31% | 36.080 |
| v5_handletable | 31ns | 29ns | 34ns | +11.06% | 33.237 |

## Performance model

- Peak throughput: **39.870 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 36.703 | 92.1% |
| v1_snapshot | 36.441 | 91.4% |
| v2_decomposed | 36.184 | 90.8% |
| v3_shapebound | 36.312 | 91.1% |
| v4_erased | 36.184 | 90.8% |
| v5_handletable | 33.574 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 157ns | 157ns | base |
| v1_snapshot | 159ns | 159ns | +1.45% |
| v2_decomposed | 159ns | 159ns | +1.29% |
| v3_shapebound | 159ns | 159ns | +0.94% |
| v4_erased | 160ns | 160ns | +1.83% |
| v5_handletable | 160ns | 160ns | +1.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 28ns | base | --- | [28, 28] | --- | --- | --- | --- |
| v1_snapshot | 28ns | no significant difference | [-0, +1]ns | [28, 28] | no | 0.2672 | 0.2672 | 3 |
| v2_decomposed | 28ns | +0.5ns (+1.8%) | [+0, +1]ns | [28, 29] | YES | 0.0435 | 0.0199 | 4 |
| v3_shapebound | 28ns | +0.3ns (+1.1%) | [+0, +1]ns | [28, 28] | YES | 0.0435 | 0.0261 | 3 |
| v4_erased | 28ns | no significant difference | [+0, +1]ns | [28, 29] | no | 0.0966 | 0.0773 | 4 |
| v5_handletable | 30ns | +2.8ns (+10.0%) | [+2, +3]ns | [30, 31] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 28ns | -1.1% | -1.1% | -2.8% | +1.1% | +8.1% |
| 1 | 2 | 28ns | -3.0% | -0.9% | +0.2% | +0.5% | +10.5% |
| 1 | 3 | 28ns | -1.6% | -0.4% | +0.4% | -0.5% | +9.2% |
| 1 | 4 | 28ns | -0.4% | -1.6% | -1.6% | -1.6% | +6.9% |
| 1 | 5 | 29ns | -2.7% | -2.6% | -6.3% | -4.5% | +4.3% |
| 2 | 1 | 27ns | +3.8% | +3.7% | +4.2% | +4.2% | +12.2% |
| 2 | 2 | 27ns | +6.1% | +3.3% | +2.4% | +6.9% | +11.5% |
| 2 | 3 | 28ns | +1.8% | -1.6% | +0.9% | +2.0% | +6.9% |
| 2 | 4 | 27ns | +1.3% | +5.0% | +1.8% | +1.3% | +11.8% |
| 2 | 5 | 28ns | -2.2% | +7.2% | +2.5% | +0.5% | +10.1% |
| 3 | 1 | 27ns | +2.0% | +4.7% | +3.1% | +4.0% | +8.9% |
| 3 | 2 | 28ns | +1.8% | +0.0% | +2.5% | +8.4% | +7.3% |
| 3 | 3 | 26ns | +7.2% | +7.2% | +5.7% | +12.6% | +16.2% |
| 3 | 4 | 28ns | +2.2% | +2.3% | +1.4% | +0.0% | +8.7% |
| 3 | 5 | 27ns | +5.7% | +4.4% | +0.9% | +4.8% | +12.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.169 | ok |
| v1_snapshot | 0.383 | moderate+ |
| v2_decomposed | 0.187 | ok |
| v3_shapebound | -0.450 | moderate- |
| v4_erased | 0.233 | moderate+ |
| v5_handletable | 0.059 | ok |

**Consistency summary:**

- **v1_snapshot**: won 6/15, lost 9/15
- **v2_decomposed**: won 6/15, lost 8/15
- **v3_shapebound**: won 3/15, lost 12/15
- **v4_erased**: won 3/15, lost 11/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 1975.0ns | 27.7ns | 7119.4% | HIGH |
| v1_snapshot | 1988.8ns | 29.1ns | 6840.3% | HIGH |
| v2_decomposed | 1981.9ns | 28.4ns | 6984.5% | HIGH |
| v3_shapebound | 1978.0ns | 28.1ns | 7049.3% | HIGH |
| v4_erased | 1979.5ns | 28.4ns | 6974.6% | HIGH |
| v5_handletable | 2044.0ns | 30.8ns | 6634.2% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 25.7-29.7 ns)
     25.7 |##################
     25.9 |#######
     26.1 |##########
     26.3 |#####################
     26.5 |##############
     26.7 |################################
     26.9 |##########
     27.1 |##############
     27.3 |################################
     27.5 |#####################
     27.7 |#######
     27.9 |####################################
     28.1 |########################################
     28.3 |##########
     28.5 |################################
     28.7 |##################
     28.9 |##############
     29.1 |#####################
     29.3 |#######
     29.5 |
  (7 below, 10 above range)

v1_snapshot (n=120, range 26.2-35.0 ns)
     26.2 |############
     26.6 |##################
     27.0 |##################
     27.5 |########################################
     27.9 |################################
     28.4 |####################################
     28.8 |##############
     29.2 |############
     29.7 |########################
     30.1 |######
     30.6 |##
     31.0 |
     31.4 |
     31.9 |
     32.3 |
     32.8 |
     33.2 |
     33.7 |
     34.1 |
     34.5 |
  (12 below, 1 above range)

v2_decomposed (n=120, range 26.5-30.3 ns)
     26.5 |################
     26.7 |############
     26.9 |########
     27.1 |################################
     27.3 |################
     27.5 |########
     27.6 |########################################
     27.8 |########################
     28.0 |####################################
     28.2 |################
     28.4 |####################################
     28.6 |############################
     28.8 |########
     29.0 |########
     29.1 |########################################
     29.3 |####################
     29.5 |############
     29.7 |
     29.9 |############
     30.1 |####################
  (10 below, 12 above range)

v3_shapebound (n=120, range 26.3-29.7 ns)
     26.3 |####
     26.5 |
     26.7 |#############
     26.8 |######
     27.0 |
     27.2 |##########################
     27.3 |########
     27.5 |
     27.7 |#################################
     27.9 |########
     28.0 |
     28.2 |########################################
     28.4 |########
     28.5 |####
     28.7 |#############
     28.9 |###########
     29.0 |#############
     29.2 |######
     29.4 |#############
     29.6 |####
  (13 below, 9 above range)

v4_erased (n=120, range 26.3-30.5 ns)
     26.3 |########
     26.6 |##
     26.8 |#####
     27.0 |##
     27.2 |##############
     27.4 |##############
     27.6 |########################################
     27.8 |#########################
     28.0 |##############
     28.2 |##############
     28.4 |####################
     28.6 |###############################
     28.8 |###########
     29.1 |#################
     29.3 |####################
     29.5 |#####
     29.7 |########
     29.9 |#####
     30.1 |########
     30.3 |##
  (13 below, 11 above range)

v5_handletable (n=120, range 28.9-33.9 ns)
     28.9 |######
     29.1 |##################
     29.4 |####
     29.6 |########################
     29.9 |##################
     30.1 |##############################
     30.4 |##############
     30.6 |########################################
     30.9 |##########
     31.1 |########
     31.4 |####
     31.6 |############
     31.9 |######
     32.1 |########
     32.4 |########
     32.6 |##
     32.9 |
     33.1 |####
     33.4 |
     33.6 |####
  (9 below, 1 above range)

```

## Diagnostics

- **v0_blob**: bridge=7045.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: CV=39.8% (high variance, measurements may be unstable)
- **v1_snapshot**: bridge=7001.8% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6956.2% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=6973.4% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6952.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6671.8% of algo (FFI overhead may distort results)
