# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 4.0x slower than the field

ctz_loop (2.47 us) is 4.0x the fastest (620 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_intrinsic is fastest but the noisiest (CV 66.3%)

ctz_intrinsic wins on median (620 ns) yet has the highest variance (CV 66.3%), while ctz_loop is the steadiest (CV 22.3%, 2.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ctz_intrinsic, ctz_debruijn) are a dead heat (<1%)

ctz_intrinsic (620 ns) and ctz_debruijn (621 ns) differ by 0.15%, inside the noise, even though the wider field spreads 298.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 4.0x the fastest

Fastest ctz_intrinsic (620 ns) to slowest ctz_loop (2.47 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### ctz_intrinsic is inconsistent: worst-20% is 2.6x its best-20%

ctz_intrinsic's best 20% of batches run at 561 ns but its worst 20% at 1.45 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: ctz_intrinsic** at 620.3 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 3.98x (fastest 620.3 ns, slowest 2472.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 2112792693 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 3190ns | 2500ns | 2269ns | 2565ns | 5985ns | base |
| ctz_intrinsic | 3126ns | 2499ns | 2266ns | 2501ns | 5861ns | -2.01% |
| ctz_loop | 4303ns | 4374ns | 3577ns | 4329ns | 4952ns | +34.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 779ns | 561ns | 1436ns | base | 21.032 |
| ctz_intrinsic | 776ns | 561ns | 1454ns | -0.44% | 21.125 |
| ctz_loop | 2458ns | 2026ns | 2911ns | +215.57% | 6.665 |

## Performance model

- Peak throughput: **29.225 Gops/s** (ctz_debruijn; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 26.373 | 90.2% |
| ctz_intrinsic | 26.411 | 90.4% |
| ctz_loop | 6.628 | 22.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 3190ns | 3190ns | base |
| ctz_intrinsic | 3126ns | 3126ns | -2.01% |
| ctz_loop | 4303ns | 4303ns | +34.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 621ns | base | --- | [620, 623] | --- | --- | --- | --- |
| ctz_intrinsic | 620ns | no significant difference | [-3, +2]ns | [619, 622] | no | 0.6928 | 0.6928 | 0 |
| ctz_loop | 2472ns | +1834.8ns (+295.3%) | [+1779, +1843]ns | [2457, 2498] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 852ns | -27.2% | +134.5% |
| 1 | 2 | 508ns | +22.1% | +306.7% |
| 1 | 3 | 617ns | +0.4% | +321.5% |
| 1 | 4 | 622ns | -0.1% | +295.4% |
| 1 | 5 | 624ns | -0.1% | +300.6% |
| 1 | 6 | 621ns | -17.8% | +284.2% |
| 1 | 7 | 621ns | +0.2% | +317.7% |
| 1 | 8 | 620ns | +0.2% | +310.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.377 | moderate- |
| ctz_intrinsic | -0.190 | ok |
| ctz_loop | 0.283 | moderate+ |

**Consistency summary:**

- **ctz_intrinsic**: won 3/8, lost 4/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 31.0ns | 779.0ns | 4.0% |  |
| ctz_intrinsic | 30.3ns | 775.6ns | 3.9% |  |
| ctz_loop | 24.4ns | 2458.3ns | 1.0% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 560.6-1436.4 ns)
    560.6 |
    604.4 |########################################
    648.2 |#
    692.0 |
    735.8 |
    779.6 |
    823.4 |#####
    867.1 |
    910.9 |
    954.7 |
    998.5 |
   1042.3 |
   1086.1 |
   1129.9 |
   1173.7 |
   1217.5 |
   1261.2 |
   1305.0 |
   1348.8 |
   1392.6 |
  (16 below, 18 above range)

ctz_intrinsic (n=160, range 560.7-1453.9 ns)
    560.7 |
    605.4 |########################################
    650.1 |
    694.7 |
    739.4 |
    784.0 |
    828.7 |
    873.3 |
    918.0 |
    962.6 |
   1007.3 |
   1052.0 |
   1096.6 |
   1141.3 |#
   1185.9 |
   1230.6 |
   1275.2 |
   1319.9 |
   1364.5 |
   1409.2 |
  (16 below, 18 above range)

ctz_loop (n=160, range 2025.8-2910.6 ns)
   2025.8 |##############################
   2070.0 |###
   2114.2 |
   2158.5 |
   2202.7 |
   2247.0 |
   2291.2 |
   2335.5 |############
   2379.7 |#########################
   2423.9 |##################################
   2468.2 |########################################
   2512.4 |######################################
   2556.7 |########################################
   2600.9 |#######################
   2645.2 |
   2689.4 |
   2733.6 |
   2777.9 |
   2822.1 |
   2866.4 |
  (19 below, 4 above range)

```

## Diagnostics

- **ctz_debruijn**: CV=51.8% (high variance, measurements may be unstable)
- **ctz_intrinsic**: CV=53.0% (high variance, measurements may be unstable)
- **ctz_loop**: CV=22.5% (high variance, measurements may be unstable)
