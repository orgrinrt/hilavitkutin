# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v2_decomposed shows alternating (throttle bounce) (autocorr -0.55)

v2_decomposed's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (27 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (v0_blob) is the fastest** at 27.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1.11x (fastest 27.4 ns, slowest 30.4 ns)

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
| v0_blob | 4729ns | 4712ns | 4703ns | 4716ns | 4796ns | base |
| v1_snapshot | 4752ns | 4718ns | 4704ns | 4729ns | 4867ns | +0.47% |
| v2_decomposed | 4730ns | 4712ns | 4702ns | 4715ns | 4804ns | +0.02% |
| v3_shapebound | 4732ns | 4715ns | 4704ns | 4719ns | 4799ns | +0.06% |
| v4_erased | 4733ns | 4717ns | 4706ns | 4722ns | 4792ns | +0.08% |
| v5_handletable | 4732ns | 4715ns | 4700ns | 4716ns | 4808ns | +0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 27ns | 26ns | 29ns | base | 37.337 |
| v1_snapshot | 28ns | 26ns | 30ns | +1.52% | 36.779 |
| v2_decomposed | 28ns | 26ns | 30ns | +1.96% | 36.618 |
| v3_shapebound | 28ns | 26ns | 29ns | +1.31% | 36.854 |
| v4_erased | 28ns | 27ns | 30ns | +3.56% | 36.052 |
| v5_handletable | 30ns | 29ns | 32ns | +10.90% | 33.668 |

## Performance model

- Peak throughput: **40.046 Gops/s** (v0_blob; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 37.372 | 93.3% |
| v1_snapshot | 36.835 | 92.0% |
| v2_decomposed | 36.571 | 91.3% |
| v3_shapebound | 36.901 | 92.1% |
| v4_erased | 36.184 | 90.4% |
| v5_handletable | 33.684 | 84.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 4729ns | 4729ns | base |
| v1_snapshot | 4752ns | 4752ns | +0.47% |
| v2_decomposed | 4730ns | 4730ns | +0.02% |
| v3_shapebound | 4732ns | 4732ns | +0.06% |
| v4_erased | 4733ns | 4733ns | +0.08% |
| v5_handletable | 4732ns | 4732ns | +0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 27ns | base | --- | [27, 28] | --- | --- | --- | --- |
| v1_snapshot | 28ns | no significant difference | [+0, +1]ns | [27, 28] | no | 0.0609 | 0.0398 | 5 |
| v2_decomposed | 28ns | no significant difference | [+0, +1]ns | [28, 28] | no | 0.0900 | 0.0900 | 7 |
| v3_shapebound | 28ns | no significant difference | [+0, +1]ns | [28, 28] | no | 0.0609 | 0.0487 | 6 |
| v4_erased | 28ns | +1.0ns (+3.6%) | [+0, +1]ns | [28, 29] | YES | 0.0000 | 0.0000 | 5 |
| v5_handletable | 30ns | +3.3ns (+12.0%) | [+3, +4]ns | [30, 31] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 27ns | +5.3% | +2.2% | -1.1% | +2.0% | +13.2% |
| 1 | 2 | 27ns | +0.4% | +7.6% | +5.4% | +5.4% | +9.9% |
| 1 | 3 | 27ns | +3.1% | -0.5% | +2.9% | +4.9% | +11.0% |
| 1 | 4 | 28ns | +4.5% | +6.5% | +3.6% | +4.2% | +7.6% |
| 1 | 5 | 27ns | +3.7% | +0.9% | +2.6% | +3.5% | +12.9% |
| 2 | 1 | 28ns | +0.0% | +0.5% | +2.0% | +2.7% | +10.4% |
| 2 | 2 | 28ns | +0.9% | +1.6% | +1.4% | +4.0% | +8.3% |
| 2 | 3 | 29ns | -1.6% | -2.4% | -2.6% | -2.6% | +7.2% |
| 2 | 4 | 27ns | +3.0% | +0.9% | +3.1% | +7.4% | +12.8% |
| 2 | 5 | 28ns | -0.9% | +0.7% | -0.5% | +4.3% | +9.4% |
| 3 | 1 | 28ns | +2.7% | -0.5% | -0.9% | +4.9% | +10.2% |
| 3 | 2 | 27ns | +2.4% | +2.9% | -0.2% | +8.8% | +12.1% |
| 3 | 3 | 27ns | -0.4% | +0.7% | +3.7% | +1.5% | +14.9% |
| 3 | 4 | 26ns | +6.4% | +7.8% | +4.9% | +7.6% | +21.0% |
| 3 | 5 | 28ns | -1.1% | -3.2% | -5.3% | -0.2% | +5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.174 | ok |
| v1_snapshot | -0.277 | moderate- |
| v2_decomposed | -0.550 | HIGH- (thermal bounce) |
| v3_shapebound | 0.146 | ok |
| v4_erased | -0.179 | ok |
| v5_handletable | -0.022 | ok |

**Consistency summary:**

- **v1_snapshot**: won 4/15, lost 10/15
- **v2_decomposed**: won 4/15, lost 11/15
- **v3_shapebound**: won 6/15, lost 9/15
- **v4_erased**: won 2/15, lost 13/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 1994.6ns | 27.4ns | 7272.8% | HIGH |
| v1_snapshot | 2003.5ns | 27.8ns | 7196.2% | HIGH |
| v2_decomposed | 1998.2ns | 28.0ns | 7145.4% | HIGH |
| v3_shapebound | 1992.0ns | 27.8ns | 7169.5% | HIGH |
| v4_erased | 1991.7ns | 28.4ns | 7012.1% | HIGH |
| v5_handletable | 2064.8ns | 30.4ns | 6788.9% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 25.6-29.3 ns)
     25.6 |####
     25.8 |####################
     25.9 |########
     26.1 |####################################
     26.3 |############
     26.5 |
     26.7 |########################################
     26.9 |####################
     27.1 |####################################
     27.2 |####################
     27.4 |################################
     27.6 |############################
     27.8 |
     28.0 |####################################
     28.2 |########################################
     28.4 |####
     28.5 |####################
     28.7 |####################
     28.9 |############
     29.1 |
  (11 below, 12 above range)

v1_snapshot (n=120, range 26.1-29.6 ns)
     26.1 |####
     26.3 |######
     26.5 |###########
     26.6 |######################
     26.8 |
     27.0 |###########
     27.2 |###############################
     27.3 |
     27.5 |########
     27.7 |#################
     27.8 |######
     28.0 |
     28.2 |########################################
     28.4 |######
     28.5 |########
     28.7 |###########
     28.9 |########
     29.1 |###########
     29.2 |###########
     29.4 |####
  (10 below, 10 above range)

v2_decomposed (n=120, range 26.2-29.9 ns)
     26.2 |################
     26.4 |########
     26.6 |############
     26.8 |####################################
     27.0 |################
     27.1 |####################################
     27.3 |
     27.5 |####################################
     27.7 |########################################
     27.9 |####################################
     28.0 |############################
     28.2 |############
     28.4 |################################
     28.6 |############
     28.8 |########
     29.0 |####################
     29.1 |############################
     29.3 |
     29.5 |####
     29.7 |############################
  (9 below, 9 above range)

v3_shapebound (n=120, range 26.2-29.4 ns)
     26.2 |#######
     26.4 |##########
     26.6 |##########
     26.7 |#################
     26.9 |######################
     27.0 |
     27.2 |#########################
     27.4 |###########################
     27.5 |
     27.7 |######################
     27.8 |
     28.0 |#################
     28.2 |########################################
     28.3 |
     28.5 |###############
     28.6 |###############
     28.8 |#######
     29.0 |##
     29.1 |#####
     29.3 |#######
  (8 below, 11 above range)

v4_erased (n=120, range 26.6-30.2 ns)
     26.6 |
     26.7 |##########
     26.9 |#############
     27.1 |################
     27.3 |######
     27.5 |####################
     27.7 |#######################
     27.8 |################
     28.0 |########################################
     28.2 |#######################
     28.4 |#############
     28.6 |####################
     28.8 |#############
     28.9 |#######################
     29.1 |####################################
     29.3 |
     29.5 |#############
     29.7 |####################
     29.9 |######
     30.0 |#############
  (12 below, 9 above range)

v5_handletable (n=120, range 28.7-32.1 ns)
     28.7 |#################
     28.9 |#############
     29.1 |###############################
     29.2 |###############################
     29.4 |######################
     29.6 |###############################
     29.7 |######################
     29.9 |######################
     30.1 |########
     30.2 |###################################
     30.4 |########
     30.6 |###################################
     30.7 |#############
     30.9 |###################################
     31.1 |########################################
     31.3 |###################################
     31.4 |###############################
     31.6 |#############
     31.8 |#############
     31.9 |####
  (7 below, 8 above range)

```

## Diagnostics

- **v0_blob**: bridge=7244.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7145.1% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7097.0% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7152.4% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=7014.8% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6759.0% of algo (FFI overhead may distort results)
