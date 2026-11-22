# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v0_blob is fastest but the noisiest (CV 15.7%)

v0_blob wins on median (28 ns) yet has the highest variance (CV 15.7%), while v5_handletable is the steadiest (CV 4.2%, 31 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (v0_blob, v1_snapshot) are a dead heat (<1%)

v0_blob (28 ns) and v1_snapshot (28 ns) differ by 0.00%, inside the noise, even though the wider field spreads 10.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (28 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### v2_decomposed's edge over baseline is significant but tiny (0 ns, 1.80%)

v2_decomposed differs from baseline v0_blob by 0 ns (1.80%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (v0_blob) is the fastest** at 27.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.10x (fastest 27.8 ns, slowest 30.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under L1-evicting heavy-memory mix-in |
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
| v0_blob | 4745ns | 4716ns | 4705ns | 4718ns | 4865ns | base |
| v1_snapshot | 4771ns | 4716ns | 4703ns | 4734ns | 4951ns | +0.55% |
| v2_decomposed | 4758ns | 4715ns | 4704ns | 4723ns | 4915ns | +0.28% |
| v3_shapebound | 4742ns | 4714ns | 4703ns | 4715ns | 4862ns | -0.06% |
| v4_erased | 4748ns | 4712ns | 4704ns | 4715ns | 4892ns | +0.07% |
| v5_handletable | 4762ns | 4716ns | 4703ns | 4722ns | 4941ns | +0.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 28ns | 26ns | 32ns | base | 36.300 |
| v1_snapshot | 28ns | 26ns | 31ns | -0.18% | 36.367 |
| v2_decomposed | 28ns | 27ns | 30ns | +0.48% | 36.127 |
| v3_shapebound | 28ns | 26ns | 30ns | -1.33% | 36.789 |
| v4_erased | 29ns | 27ns | 30ns | +1.22% | 35.862 |
| v5_handletable | 31ns | 29ns | 32ns | +8.45% | 33.472 |

## Performance model

- Peak throughput: **39.696 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 36.835 | 92.8% |
| v1_snapshot | 36.835 | 92.8% |
| v2_decomposed | 36.056 | 90.8% |
| v3_shapebound | 36.835 | 92.8% |
| v4_erased | 35.930 | 90.5% |
| v5_handletable | 33.355 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 4745ns | 4745ns | base |
| v1_snapshot | 4771ns | 4771ns | +0.55% |
| v2_decomposed | 4758ns | 4758ns | +0.28% |
| v3_shapebound | 4742ns | 4742ns | -0.06% |
| v4_erased | 4748ns | 4748ns | +0.07% |
| v5_handletable | 4762ns | 4762ns | +0.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 28ns | base | --- | [28, 28] | --- | --- | --- | --- |
| v1_snapshot | 28ns | no significant difference | [-0, +0]ns | [28, 28] | no | 0.6403 | 0.5123 | 6 |
| v2_decomposed | 28ns | +0.5ns (+1.8%) | [+0, +1]ns | [28, 29] | YES | 0.0435 | 0.0261 | 3 |
| v3_shapebound | 28ns | no significant difference | [-0, +0]ns | [28, 28] | no | 0.7770 | 0.7770 | 8 |
| v4_erased | 28ns | +0.6ns (+2.0%) | [+0, +1]ns | [28, 29] | YES | 0.0159 | 0.0064 | 6 |
| v5_handletable | 31ns | +2.8ns (+10.1%) | [+2, +3]ns | [30, 31] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 28ns | -2.6% | +3.0% | +1.1% | -1.8% | +8.5% |
| 1 | 2 | 28ns | -1.6% | +5.6% | -2.4% | +3.8% | +11.6% |
| 1 | 3 | 27ns | +3.2% | +0.9% | +1.3% | +8.0% | +11.3% |
| 1 | 4 | 28ns | +1.3% | -1.8% | -0.4% | +3.2% | +7.1% |
| 1 | 5 | 30ns | -6.6% | -7.7% | -6.1% | +1.0% | -0.8% |
| 2 | 1 | 28ns | -0.7% | +3.8% | -3.1% | +4.4% | +10.0% |
| 2 | 2 | 27ns | +4.6% | +8.7% | +1.7% | +2.0% | +16.8% |
| 2 | 3 | 27ns | +1.8% | +2.8% | +2.9% | +7.0% | +12.9% |
| 2 | 4 | 29ns | -2.6% | -0.3% | -1.9% | -0.5% | +9.2% |
| 2 | 5 | 28ns | -0.2% | +3.9% | +0.7% | +3.0% | +12.4% |
| 3 | 1 | 28ns | +0.5% | +4.5% | -1.1% | +1.8% | +11.5% |
| 3 | 2 | 28ns | +6.1% | +2.2% | +1.3% | +5.4% | +10.3% |
| 3 | 3 | 28ns | -0.7% | +5.5% | +0.0% | +2.4% | +12.2% |
| 3 | 4 | 27ns | +2.8% | +0.9% | +5.5% | +0.6% | +8.3% |
| 3 | 5 | 28ns | -0.9% | +0.9% | +1.3% | -0.5% | +6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.021 | ok |
| v1_snapshot | -0.319 | moderate- |
| v2_decomposed | 0.135 | ok |
| v3_shapebound | -0.085 | ok |
| v4_erased | 0.184 | ok |
| v5_handletable | 0.379 | moderate+ |

**Consistency summary:**

- **v1_snapshot**: won 8/15, lost 7/15
- **v2_decomposed**: won 3/15, lost 12/15
- **v3_shapebound**: won 6/15, lost 8/15
- **v4_erased**: won 3/15, lost 12/15
- **v5_handletable**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 1998.9ns | 28.2ns | 7086.0% | HIGH |
| v1_snapshot | 2008.3ns | 28.2ns | 7132.4% | HIGH |
| v2_decomposed | 2015.6ns | 28.3ns | 7111.2% | HIGH |
| v3_shapebound | 2000.4ns | 27.8ns | 7186.9% | HIGH |
| v4_erased | 2000.3ns | 28.6ns | 7005.4% | HIGH |
| v5_handletable | 2075.9ns | 30.6ns | 6785.7% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 25.8-31.7 ns)
     25.8 |#############
     26.1 |##########
     26.4 |################
     26.7 |##################
     27.0 |##################################
     27.3 |##################
     27.6 |########################################
     27.9 |##########
     28.2 |################
     28.5 |#############################
     28.8 |#####################
     29.1 |#############
     29.4 |#############
     29.7 |#####################
     29.9 |##
     30.2 |##
     30.5 |##
     30.8 |##
     31.1 |##
     31.4 |
  (9 below, 2 above range)

v1_snapshot (n=120, range 26.2-31.2 ns)
     26.2 |##########################
     26.4 |##
     26.7 |##################
     26.9 |#############
     27.2 |########################################
     27.4 |##################
     27.7 |##########################
     27.9 |##########################
     28.2 |#############################
     28.4 |##########
     28.7 |#####################
     28.9 |#####
     29.2 |#####################
     29.4 |########
     29.7 |################
     29.9 |#####
     30.2 |##
     30.4 |##
     30.7 |
     30.9 |
  (8 below, 1 above range)

v2_decomposed (n=120, range 26.5-30.2 ns)
     26.5 |
     26.7 |##############################
     26.9 |###############
     27.1 |###################################
     27.2 |####################
     27.4 |###################################
     27.6 |#########################
     27.8 |#########################
     28.0 |########################################
     28.2 |#########################
     28.4 |#########################
     28.5 |#########################
     28.7 |########################################
     28.9 |########################################
     29.1 |##############################
     29.3 |#####
     29.5 |###################################
     29.6 |####################
     29.8 |##########
     30.0 |
  (10 below, 14 above range)

v3_shapebound (n=120, range 26.1-29.7 ns)
     26.1 |#######
     26.3 |##########
     26.5 |##################
     26.6 |################################
     26.8 |##############
     27.0 |
     27.2 |########################################
     27.4 |##########
     27.5 |#############################
     27.7 |#############################
     27.9 |####################################
     28.1 |####################################
     28.3 |#####################
     28.4 |##########
     28.6 |#####################
     28.8 |
     29.0 |##################
     29.2 |#####################
     29.3 |
     29.5 |#######
  (10 below, 9 above range)

v4_erased (n=120, range 26.7-30.5 ns)
     26.7 |######
     26.9 |###############
     27.1 |###############
     27.3 |###############
     27.5 |############
     27.7 |#####################
     27.9 |#########
     28.0 |#####################
     28.2 |###############
     28.4 |###########################
     28.6 |########################################
     28.8 |
     29.0 |#########
     29.2 |##############################
     29.4 |###############
     29.6 |###############
     29.7 |#########
     29.9 |############
     30.1 |######
     30.3 |
  (11 below, 12 above range)

v5_handletable (n=120, range 28.8-32.4 ns)
     28.8 |
     29.0 |############
     29.2 |########################
     29.4 |###############
     29.5 |#####################
     29.7 |######
     29.9 |###############
     30.1 |############
     30.3 |############
     30.4 |#####################
     30.6 |########################################
     30.8 |
     31.0 |##################
     31.2 |###########################
     31.4 |#####################
     31.5 |######
     31.7 |#########
     31.9 |#####################
     32.1 |######
     32.3 |#########
  (13 below, 9 above range)

```

## Diagnostics

- **v0_blob**: bridge=7139.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7143.0% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7074.1% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7141.4% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6965.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6700.8% of algo (FFI overhead may distort results)
