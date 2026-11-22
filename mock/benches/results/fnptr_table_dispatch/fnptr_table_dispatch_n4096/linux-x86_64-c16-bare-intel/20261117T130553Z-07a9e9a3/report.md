# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 202% faster than the next best (fpt_table_16)

fpt_match_16 (590 ns) leads fpt_table_16 (1.78 us) by 202%, a clear separation rather than a photo finish. CV 61.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_match_16 is fastest but the noisiest (CV 61.4%)

fpt_match_16 wins on median (590 ns) yet has the highest variance (CV 61.4%), while fpt_table_16 is the steadiest (CV 29.4%, 1.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fpt_table_16 shows alternating (throttle bounce) (autocorr -0.51)

fpt_table_16's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (590 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.0x the fastest

Fastest fpt_match_16 (590 ns) to slowest fpt_table_16 (1.78 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fpt_match_16 is inconsistent: worst-20% is 2.2x its best-20%

fpt_match_16's best 20% of batches run at 526 ns but its worst 20% at 1.14 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 590.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.02x (fastest 590.5 ns, slowest 1784.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112762479 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2889ns | 2477ns | 2209ns | 2478ns | 4799ns | base |
| fpt_table_16 | 3642ns | 3570ns | 3040ns | 3522ns | 4605ns | +26.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 688ns | 526ns | 1139ns | base | 5.957 |
| fpt_table_16 | 1863ns | 1493ns | 2476ns | +171.03% | 2.198 |

## Performance model

- Peak throughput: **7.783 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 6.936 | 89.1% |
| fpt_table_16 | 2.295 | 29.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2889ns | 2889ns | base |
| fpt_table_16 | 3642ns | 3642ns | +26.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 590ns | base | --- | [590, 591] | --- | --- | --- | --- |
| fpt_table_16 | 1785ns | +1136.7ns (+192.5%) | [+1088, +1212]ns | [1710, 1868] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 591ns | +203.7% |
| 1 | 2 | 594ns | +174.5% |
| 1 | 3 | 588ns | +185.2% |
| 1 | 4 | 590ns | +225.8% |
| 1 | 5 | 591ns | +224.1% |
| 1 | 6 | 485ns | +205.3% |
| 1 | 7 | 586ns | +240.0% |
| 1 | 8 | 594ns | +183.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.123 | ok |
| fpt_table_16 | -0.508 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 28.0ns | 687.5ns | 4.1% |  |
| fpt_table_16 | 22.8ns | 1863.5ns | 1.2% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 526.3-1139.3 ns)
    526.3 |
    556.9 |#####
    587.6 |########################################
    618.2 |
    648.9 |#
    679.5 |
    710.2 |
    740.8 |
    771.5 |
    802.1 |
    832.8 |
    863.4 |
    894.1 |
    924.7 |
    955.4 |
    986.0 |
   1016.7 |
   1047.3 |
   1078.0 |
   1108.6 |
  (19 below, 14 above range)

fpt_table_16 (n=160, range 1492.7-2476.0 ns)
   1492.7 |##############################
   1541.9 |##################
   1591.1 |################
   1640.2 |######################################
   1689.4 |################
   1738.5 |################
   1787.7 |##############
   1836.9 |####################
   1886.0 |########################################
   1935.2 |################
   1984.4 |######################
   2033.5 |############
   2082.7 |######
   2131.9 |##
   2181.0 |####
   2230.2 |##
   2279.4 |##
   2328.5 |
   2377.7 |####
   2426.9 |
  (14 below, 7 above range)

```

## Diagnostics

- **fpt_match_16**: CV=52.7% (high variance, measurements may be unstable)
- **fpt_table_16**: CV=28.2% (high variance, measurements may be unstable)
