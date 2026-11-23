# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 250% faster than the next best (fpt_table_16)

fpt_match_16 (591 ns) leads fpt_table_16 (2.07 us) by 250%, a clear separation rather than a photo finish. CV 107.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_match_16 is fastest but the noisiest (CV 107.0%)

fpt_match_16 wins on median (591 ns) yet has the highest variance (CV 107.0%), while fpt_table_16 is the steadiest (CV 19.7%, 2.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (591 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.5x the fastest

Fastest fpt_match_16 (591 ns) to slowest fpt_table_16 (2.07 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fpt_match_16 is inconsistent: worst-20% is 3.9x its best-20%

fpt_match_16's best 20% of batches run at 501 ns but its worst 20% at 1.97 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 590.9 ns median
- 1 variant significantly slower than baseline
- Spread: 3.50x (fastest 590.9 ns, slowest 2067.1 ns)

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
| Counter frequency | 2112768962 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 3089ns | 2474ns | 2073ns | 2483ns | 5922ns | base |
| fpt_table_16 | 3848ns | 3954ns | 3100ns | 3871ns | 4525ns | +24.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 850ns | 501ns | 1971ns | base | 4.820 |
| fpt_table_16 | 2065ns | 1542ns | 2554ns | +142.97% | 1.984 |

## Performance model

- Peak throughput: **8.175 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 6.932 | 84.8% |
| fpt_table_16 | 1.982 | 24.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 3089ns | 3089ns | base |
| fpt_table_16 | 3848ns | 3848ns | +24.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 591ns | base | --- | [590, 591] | --- | --- | --- | --- |
| fpt_table_16 | 2067ns | +1351.3ns (+228.7%) | [+1278, +1525]ns | [1988, 2186] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 592ns | +220.3% |
| 1 | 2 | 502ns | +179.5% |
| 1 | 3 | 589ns | +279.3% |
| 1 | 4 | 2516ns | -25.2% |
| 1 | 5 | 483ns | +343.2% |
| 1 | 6 | 591ns | +217.1% |
| 1 | 7 | 585ns | +297.3% |
| 1 | 8 | 593ns | +312.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.187 | ok |
| fpt_table_16 | -0.041 | ok |

**Consistency summary:**

- **fpt_table_16**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 28.6ns | 849.9ns | 3.4% |  |
| fpt_table_16 | 22.8ns | 2065.0ns | 1.1% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 501.0-1971.3 ns)
    501.0 |######
    574.6 |########################################
    648.1 |
    721.6 |
    795.1 |
    868.6 |
    942.1 |
   1015.6 |
   1089.2 |
   1162.7 |
   1236.2 |#
   1309.7 |
   1383.2 |
   1456.7 |
   1530.2 |
   1603.7 |
   1677.3 |
   1750.8 |
   1824.3 |
   1897.8 |
  (14 below, 18 above range)

fpt_table_16 (n=160, range 1542.5-2553.9 ns)
   1542.5 |#######
   1593.0 |##
   1643.6 |####
   1694.2 |#########
   1744.8 |#########
   1795.3 |################
   1845.9 |########################################
   1896.5 |#########################
   1947.0 |###########
   1997.6 |################
   2048.2 |###########
   2098.7 |################
   2149.3 |###########
   2199.9 |########################################
   2250.4 |##############
   2301.0 |#####################
   2351.6 |#########
   2402.2 |############################
   2452.7 |############################
   2503.3 |##
  (16 below, 5 above range)

```

## Diagnostics

- **fpt_match_16**: CV=74.4% (high variance, measurements may be unstable)
- **fpt_match_16**: worst_20/best_20 = 3.9x (possible bimodal distribution)
