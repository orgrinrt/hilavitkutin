# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (v0_blob, v2_decomposed) are a dead heat (<1%)

v0_blob (28 ns) and v2_decomposed (28 ns) differ by 0.35%, inside the noise, even though the wider field spreads 9.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (28 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader v0_blob vs stability leader v1_snapshot (+1% speed for 1.1x steadier)

v0_blob is fastest (28 ns, CV 4.7%); v1_snapshot gives up 0.7% median for 1.1x lower variance (CV 4.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (v0_blob) is the fastest** at 28.3 ns median
- 2 variants significantly slower than baseline
- Spread: 1.10x (fastest 28.3 ns, slowest 31.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under clean morsel loop (algo only) |
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
| v0_blob | 77ns | 76ns | 73ns | 77ns | 81ns | base |
| v1_snapshot | 77ns | 77ns | 73ns | 77ns | 81ns | +0.04% |
| v2_decomposed | 76ns | 76ns | 70ns | 76ns | 80ns | -1.49% |
| v3_shapebound | 77ns | 77ns | 73ns | 76ns | 80ns | -0.30% |
| v4_erased | 78ns | 77ns | 74ns | 77ns | 84ns | +1.50% |
| v5_handletable | 77ns | 77ns | 74ns | 77ns | 81ns | +0.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 28ns | 27ns | 30ns | base | 36.058 |
| v1_snapshot | 29ns | 27ns | 30ns | +0.59% | 35.847 |
| v2_decomposed | 28ns | 27ns | 30ns | +0.21% | 35.985 |
| v3_shapebound | 28ns | 26ns | 30ns | -0.27% | 36.155 |
| v4_erased | 30ns | 27ns | 34ns | +4.56% | 34.486 |
| v5_handletable | 31ns | 29ns | 33ns | +8.70% | 33.173 |

## Performance model

- Peak throughput: **38.739 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 36.184 | 93.4% |
| v1_snapshot | 35.930 | 92.7% |
| v2_decomposed | 36.056 | 93.1% |
| v3_shapebound | 35.930 | 92.7% |
| v4_erased | 35.068 | 90.5% |
| v5_handletable | 33.032 | 85.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 77ns | 77ns | base |
| v1_snapshot | 77ns | 77ns | +0.04% |
| v2_decomposed | 76ns | 76ns | -1.49% |
| v3_shapebound | 77ns | 77ns | -0.30% |
| v4_erased | 78ns | 78ns | +1.50% |
| v5_handletable | 77ns | 77ns | +0.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 28ns | base | --- | [28, 29] | --- | --- | --- | --- |
| v1_snapshot | 28ns | no significant difference | [-0, +0]ns | [28, 29] | no | 0.3127 | 0.1876 | 7 |
| v2_decomposed | 28ns | no significant difference | [-0, +0]ns | [28, 29] | no | 0.3678 | 0.2942 | 10 |
| v3_shapebound | 28ns | no significant difference | [-0, +0]ns | [28, 29] | no | 0.8522 | 0.8522 | 5 |
| v4_erased | 29ns | +0.8ns (+3.0%) | [+0, +1]ns | [29, 29] | YES | 0.0003 | 0.0001 | 1 |
| v5_handletable | 31ns | +2.8ns (+9.7%) | [+2, +3]ns | [30, 31] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 29ns | -0.7% | -3.1% | -3.4% | +0.3% | +7.9% |
| 1 | 2 | 29ns | -1.4% | -3.5% | -1.7% | +3.0% | +5.7% |
| 1 | 3 | 28ns | +2.0% | +3.0% | +0.2% | +3.0% | +8.9% |
| 1 | 4 | 29ns | -3.1% | -2.1% | -4.3% | -3.2% | +3.4% |
| 1 | 5 | 28ns | +2.1% | +0.4% | +2.3% | +4.1% | +8.4% |
| 2 | 1 | 28ns | -0.2% | -4.7% | -2.3% | +4.0% | +6.5% |
| 2 | 2 | 28ns | +1.8% | +0.0% | +3.2% | +6.7% | +9.4% |
| 2 | 3 | 29ns | +2.4% | +1.2% | +2.3% | +1.9% | +3.1% |
| 2 | 4 | 28ns | +1.1% | +3.7% | -0.4% | +3.2% | +11.9% |
| 2 | 5 | 28ns | +1.8% | +0.5% | -1.2% | +3.9% | +10.4% |
| 3 | 1 | 29ns | +0.0% | -1.6% | -0.3% | +0.7% | +15.1% |
| 3 | 2 | 28ns | +0.7% | +1.2% | +1.6% | +1.2% | +11.0% |
| 3 | 3 | 28ns | +1.8% | +3.2% | +0.2% | +3.2% | +11.2% |
| 3 | 4 | 28ns | +2.3% | -0.5% | +4.2% | +5.8% | +10.3% |
| 3 | 5 | 28ns | +1.1% | +3.2% | +4.1% | +3.8% | +8.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.305 | moderate- |
| v1_snapshot | -0.372 | moderate- |
| v2_decomposed | 0.199 | ok |
| v3_shapebound | 0.083 | ok |
| v4_erased | 0.122 | ok |
| v5_handletable | 0.271 | moderate+ |

**Consistency summary:**

- **v1_snapshot**: won 4/15, lost 10/15
- **v2_decomposed**: won 6/15, lost 8/15
- **v3_shapebound**: won 7/15, lost 8/15
- **v4_erased**: won 1/15, lost 14/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 2006.3ns | 28.4ns | 7065.0% | HIGH |
| v1_snapshot | 2010.1ns | 28.6ns | 7036.8% | HIGH |
| v2_decomposed | 2005.4ns | 28.5ns | 7047.1% | HIGH |
| v3_shapebound | 2006.0ns | 28.3ns | 7082.6% | HIGH |
| v4_erased | 2016.7ns | 29.7ns | 6791.7% | HIGH |
| v5_handletable | 2071.2ns | 30.9ns | 6709.9% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 26.6-30.3 ns)
     26.6 |#####
     26.8 |########
     27.0 |##############
     27.2 |############################
     27.4 |#####
     27.5 |#################
     27.7 |##############
     27.9 |########################################
     28.1 |##
     28.3 |####################
     28.5 |#####
     28.7 |#####################################
     28.8 |######################
     29.0 |########
     29.2 |###########
     29.4 |########
     29.6 |########
     29.8 |###########
     30.0 |########
     30.1 |###########
  (9 below, 9 above range)

v1_snapshot (n=120, range 26.9-30.2 ns)
     26.9 |######
     27.1 |###
     27.2 |#########
     27.4 |###############
     27.6 |##################
     27.7 |###############
     27.9 |###
     28.0 |###########################
     28.2 |#####################
     28.4 |########################################
     28.5 |############
     28.7 |#####################
     28.9 |#####################
     29.0 |
     29.2 |###########################
     29.4 |#####################
     29.5 |
     29.7 |#################################
     29.9 |###
     30.0 |
  (12 below, 10 above range)

v2_decomposed (n=120, range 26.7-30.3 ns)
     26.7 |##########
     26.9 |#############
     27.1 |#####
     27.3 |#####
     27.5 |#############
     27.6 |########################################
     27.8 |
     28.0 |################################
     28.2 |################
     28.3 |##########
     28.5 |##################
     28.7 |################
     28.9 |########################
     29.1 |##########
     29.2 |#############
     29.4 |#####
     29.6 |#####
     29.8 |##########
     30.0 |#############
     30.1 |########
  (9 below, 9 above range)

v3_shapebound (n=120, range 26.4-30.0 ns)
     26.4 |######
     26.6 |##################
     26.8 |######
     27.0 |#########
     27.1 |###############
     27.3 |#########
     27.5 |
     27.7 |########################################
     27.9 |#####################
     28.0 |#########
     28.2 |######
     28.4 |###########################
     28.6 |############
     28.8 |#################################
     28.9 |#####################
     29.1 |#########
     29.3 |############
     29.5 |###############
     29.6 |########################################
     29.8 |
  (10 below, 8 above range)

v4_erased (n=120, range 27.3-33.7 ns)
     27.3 |######
     27.6 |###########
     27.9 |###############
     28.3 |##########################
     28.6 |####################
     28.9 |########################################
     29.2 |#################################
     29.5 |##########################
     29.9 |###############
     30.2 |######################
     30.5 |########
     30.8 |##
     31.1 |####
     31.5 |##
     31.8 |######
     32.1 |
     32.4 |
     32.8 |
     33.1 |
     33.4 |
  (10 below, 1 above range)

v5_handletable (n=120, range 29.0-32.8 ns)
     29.0 |################
     29.2 |####################
     29.4 |####################
     29.6 |#############
     29.8 |######
     30.0 |####################
     30.1 |#############
     30.3 |#################################
     30.5 |#############
     30.7 |##########
     30.9 |#######################
     31.1 |################
     31.3 |####################
     31.5 |####################
     31.6 |########################################
     31.8 |##########
     32.0 |####################
     32.2 |###
     32.4 |
     32.6 |#############
  (9 below, 11 above range)

```

## Diagnostics

- **v0_blob**: bridge=7072.4% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7024.2% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7048.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7026.7% of algo (FFI overhead may distort results)
- **v4_erased**: CV=22.9% (high variance, measurements may be unstable)
- **v4_erased**: bridge=6858.2% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6656.8% of algo (FFI overhead may distort results)
