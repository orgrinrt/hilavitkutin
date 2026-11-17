# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v0_blob is fastest but the noisiest (CV 15.4%)

v0_blob wins on median (28 ns) yet has the highest variance (CV 15.4%), while v3_shapebound is the steadiest (CV 6.1%, 28 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (28 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader v0_blob vs stability leader v3_shapebound (+2% speed for 2.5x steadier)

v0_blob is fastest (28 ns, CV 15.4%); v3_shapebound gives up 1.6% median for 2.5x lower variance (CV 6.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### v1_snapshot's edge over baseline is significant but tiny (0 ns, 1.80%)

v1_snapshot differs from baseline v0_blob by 0 ns (1.80%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (v0_blob) is the fastest** at 27.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.10x (fastest 27.8 ns, slowest 30.5 ns)

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
| v0_blob | 76ns | 75ns | 73ns | 75ns | 82ns | base |
| v1_snapshot | 77ns | 76ns | 73ns | 77ns | 84ns | +1.58% |
| v2_decomposed | 75ns | 75ns | 67ns | 76ns | 81ns | -1.59% |
| v3_shapebound | 76ns | 75ns | 73ns | 75ns | 80ns | -0.27% |
| v4_erased | 76ns | 75ns | 73ns | 75ns | 81ns | -0.23% |
| v5_handletable | 77ns | 76ns | 73ns | 76ns | 82ns | +0.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 28ns | 26ns | 33ns | base | 35.966 |
| v1_snapshot | 29ns | 26ns | 31ns | +0.51% | 35.784 |
| v2_decomposed | 28ns | 26ns | 31ns | -0.18% | 36.029 |
| v3_shapebound | 28ns | 26ns | 31ns | -0.56% | 36.167 |
| v4_erased | 29ns | 26ns | 31ns | +0.44% | 35.809 |
| v5_handletable | 31ns | 29ns | 33ns | +7.95% | 33.317 |

## Performance model

- Peak throughput: **40.007 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 36.835 | 92.1% |
| v1_snapshot | 35.679 | 89.2% |
| v2_decomposed | 36.184 | 90.4% |
| v3_shapebound | 36.248 | 90.6% |
| v4_erased | 36.184 | 90.4% |
| v5_handletable | 33.574 | 83.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 76ns | 76ns | base |
| v1_snapshot | 77ns | 77ns | +1.58% |
| v2_decomposed | 75ns | 75ns | -1.59% |
| v3_shapebound | 76ns | 76ns | -0.27% |
| v4_erased | 76ns | 76ns | -0.23% |
| v5_handletable | 77ns | 77ns | +0.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 28ns | base | --- | [28, 28] | --- | --- | --- | --- |
| v1_snapshot | 29ns | +0.5ns (+1.8%) | [+0, +1]ns | [28, 29] | YES (adj: no) | 0.0513 | 0.0210 | 2 |
| v2_decomposed | 28ns | no significant difference | [-1, +1]ns | [28, 29] | no | 0.7826 | 0.7826 | 2 |
| v3_shapebound | 28ns | no significant difference | [-0, +1]ns | [28, 29] | no | 0.7826 | 0.7826 | 2 |
| v4_erased | 28ns | no significant difference | [+0, +1]ns | [28, 29] | no | 0.0513 | 0.0308 | 6 |
| v5_handletable | 30ns | +2.6ns (+9.2%) | [+2, +3]ns | [30, 31] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 28ns | +5.9% | +0.7% | +4.1% | +2.7% | +8.8% |
| 1 | 2 | 27ns | +9.2% | +11.6% | +7.0% | +2.8% | +11.3% |
| 1 | 3 | 28ns | +2.7% | +7.1% | +5.5% | +0.9% | +8.2% |
| 1 | 4 | 29ns | -4.4% | -7.6% | -4.9% | +1.4% | +10.0% |
| 1 | 5 | 28ns | +4.9% | -2.1% | -1.2% | -0.4% | +16.6% |
| 2 | 1 | 27ns | +2.0% | +4.6% | +3.6% | +4.0% | +8.6% |
| 2 | 2 | 29ns | +3.1% | -1.6% | -6.4% | -2.8% | +3.3% |
| 2 | 3 | 28ns | +1.8% | +0.2% | -2.5% | -0.7% | +6.3% |
| 2 | 4 | 28ns | +2.5% | -0.9% | -2.9% | +5.8% | +9.8% |
| 2 | 5 | 28ns | +1.2% | -3.0% | +0.9% | +2.8% | +10.3% |
| 3 | 1 | 28ns | -2.6% | +2.1% | -0.4% | -1.4% | +4.7% |
| 3 | 2 | 27ns | +1.1% | +3.7% | +3.7% | +3.1% | +10.6% |
| 3 | 3 | 29ns | -1.1% | -3.0% | +1.2% | -3.2% | +9.3% |
| 3 | 4 | 28ns | +3.4% | +7.4% | +2.4% | +3.1% | +9.8% |
| 3 | 5 | 28ns | +0.0% | -1.2% | +0.0% | -1.6% | +9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.249 | moderate- |
| v1_snapshot | 0.070 | ok |
| v2_decomposed | 0.010 | ok |
| v3_shapebound | 0.308 | moderate+ |
| v4_erased | -0.123 | ok |
| v5_handletable | 0.114 | ok |

**Consistency summary:**

- **v1_snapshot**: won 3/15, lost 11/15
- **v2_decomposed**: won 7/15, lost 8/15
- **v3_shapebound**: won 6/15, lost 8/15
- **v4_erased**: won 6/15, lost 9/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 1978.4ns | 28.5ns | 6948.7% | HIGH |
| v1_snapshot | 2005.5ns | 28.6ns | 7008.2% | HIGH |
| v2_decomposed | 1995.0ns | 28.4ns | 7019.4% | HIGH |
| v3_shapebound | 1977.4ns | 28.3ns | 6984.0% | HIGH |
| v4_erased | 1978.9ns | 28.6ns | 6920.3% | HIGH |
| v5_handletable | 2048.0ns | 30.7ns | 6663.3% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 25.6-32.7 ns)
     25.6 |##
     26.0 |#################
     26.3 |####################
     26.7 |####################
     27.0 |########################################
     27.4 |###############################
     27.7 |############################
     28.1 |######################
     28.4 |#################
     28.8 |###############################
     29.2 |########################################
     29.5 |#################
     29.9 |########
     30.2 |#####
     30.6 |##
     30.9 |##
     31.3 |##
     31.6 |
     32.0 |
     32.4 |
  (8 below, 3 above range)

v1_snapshot (n=120, range 26.0-31.1 ns)
     26.0 |#######
     26.3 |##########
     26.5 |#############################
     26.8 |###
     27.1 |#############################
     27.3 |###
     27.6 |#########################
     27.8 |##################
     28.1 |################################
     28.3 |##################
     28.6 |#############################
     28.8 |##############
     29.1 |########################################
     29.4 |#####################
     29.6 |########################################
     29.9 |#######
     30.1 |#############################
     30.4 |#######
     30.6 |##########
     30.9 |###
  (9 below, 6 above range)

v2_decomposed (n=120, range 25.7-31.3 ns)
     25.7 |##############
     26.0 |##################
     26.3 |##########
     26.6 |##################
     26.8 |##############
     27.1 |##################
     27.4 |##############
     27.7 |##################
     28.0 |########################################
     28.2 |####################################
     28.5 |##########
     28.8 |################################
     29.1 |####################################
     29.3 |##############
     29.6 |##################
     29.9 |##########
     30.2 |################################
     30.5 |##########
     30.7 |#######
     31.0 |
  (10 below, 6 above range)

v3_shapebound (n=120, range 26.0-30.8 ns)
     26.0 |#####
     26.3 |###############
     26.5 |#######
     26.8 |#################
     27.0 |############
     27.2 |##########
     27.5 |####################
     27.7 |#####
     28.0 |##########
     28.2 |########################################
     28.4 |##########
     28.7 |######################
     28.9 |###############
     29.1 |###############
     29.4 |
     29.6 |####################
     29.9 |#######
     30.1 |############
     30.3 |##
     30.6 |#####
  (10 below, 9 above range)

v4_erased (n=120, range 26.3-31.1 ns)
     26.3 |##########
     26.5 |
     26.8 |#################################
     27.0 |######
     27.3 |#################################
     27.5 |#############
     27.7 |################
     28.0 |########################################
     28.2 |####################
     28.5 |########################################
     28.7 |
     29.0 |####################
     29.2 |####################
     29.4 |#############
     29.7 |#############
     29.9 |
     30.2 |################
     30.4 |####################
     30.7 |################
     30.9 |######
  (9 below, 9 above range)

v5_handletable (n=120, range 29.0-33.1 ns)
     29.0 |###
     29.2 |######
     29.5 |########################
     29.7 |########################################
     29.9 |#####################
     30.1 |##################
     30.3 |#####################
     30.5 |########################
     30.7 |##############################
     30.9 |########################
     31.1 |#########
     31.3 |##############################
     31.5 |#########
     31.7 |###############
     31.9 |#########
     32.1 |######
     32.3 |###
     32.5 |############
     32.7 |###
     32.9 |
  (13 below, 5 above range)

```

## Diagnostics

- **v0_blob**: bridge=7064.0% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6848.8% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6942.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=6952.2% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6937.5% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6637.7% of algo (FFI overhead may distort results)
