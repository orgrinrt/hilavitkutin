# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 236% faster than the next best (fpt_table_16)

fpt_match_16 (590 ns) leads fpt_table_16 (1.99 us) by 236%, a clear separation rather than a photo finish. CV 59.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_match_16 is fastest but the noisiest (CV 59.8%)

fpt_match_16 wins on median (590 ns) yet has the highest variance (CV 59.8%), while fpt_table_16 is the steadiest (CV 16.0%, 1.99 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (590 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest fpt_match_16 (590 ns) to slowest fpt_table_16 (1.99 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fpt_match_16 is inconsistent: worst-20% is 2.4x its best-20%

fpt_match_16's best 20% of batches run at 485 ns but its worst 20% at 1.17 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 590.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.36x (fastest 590.4 ns, slowest 1985.0 ns)

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
| Counter frequency | 2112822230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2859ns | 2473ns | 2025ns | 2455ns | 4904ns | base |
| fpt_table_16 | 3702ns | 3874ns | 3045ns | 3763ns | 4176ns | +29.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 684ns | 485ns | 1173ns | base | 5.988 |
| fpt_table_16 | 1893ns | 1359ns | 2272ns | +176.79% | 2.164 |

## Performance model

- Peak throughput: **8.442 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 6.938 | 82.2% |
| fpt_table_16 | 2.063 | 24.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2859ns | 2859ns | base |
| fpt_table_16 | 3702ns | 3702ns | +29.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 590ns | base | --- | [590, 591] | --- | --- | --- | --- |
| fpt_table_16 | 1985ns | +1318.1ns (+223.3%) | [+1252, +1392]ns | [1944, 1996] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 483ns | +186.0% |
| 1 | 2 | 602ns | +217.4% |
| 1 | 3 | 593ns | +241.7% |
| 1 | 4 | 590ns | +247.8% |
| 1 | 5 | 590ns | +211.9% |
| 1 | 6 | 490ns | +177.0% |
| 1 | 7 | 589ns | +277.7% |
| 1 | 8 | 594ns | +235.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.223 | moderate- |
| fpt_table_16 | -0.161 | ok |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 28.2ns | 684.0ns | 4.1% |  |
| fpt_table_16 | 23.1ns | 1893.2ns | 1.2% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 485.2-1172.7 ns)
    485.2 |######
    519.6 |
    554.0 |####
    588.3 |########################################
    622.7 |
    657.1 |
    691.5 |
    725.8 |
    760.2 |
    794.6 |
    829.0 |
    863.3 |
    897.7 |
    932.1 |
    966.5 |
   1000.8 |
   1035.2 |
   1069.6 |
   1104.0 |
   1138.3 |
  (20 below, 14 above range)

fpt_table_16 (n=160, range 1359.1-2272.4 ns)
   1359.1 |#############
   1404.8 |#######
   1450.5 |
   1496.1 |##
   1541.8 |#####
   1587.4 |#
   1633.1 |#
   1678.8 |#
   1724.4 |#
   1770.1 |#######
   1815.8 |####################
   1861.4 |########
   1907.1 |#####
   1952.8 |########################################
   1998.4 |#######################
   2044.1 |########
   2089.8 |#############
   2135.4 |#############
   2181.1 |########
   2226.8 |########
  (15 below, 13 above range)

```

## Diagnostics

- **fpt_match_16**: CV=51.6% (high variance, measurements may be unstable)
