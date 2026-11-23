# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 142% faster than the next best (fold_paired)

fold_quad (475 ns) leads fold_paired (1.15 us) by 142%, a clear separation rather than a photo finish. CV 71.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 59% (significant)

fold_quad is -674 ns (59%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 4.8x slower than the field

fold_sequential (2.29 us) is 4.8x the fastest (475 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 71.7%)

fold_quad wins on median (475 ns) yet has the highest variance (CV 71.7%), while fold_sequential is the steadiest (CV 11.9%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fold_sequential shows alternating (throttle bounce) (autocorr -0.68)

fold_sequential's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.8x the fastest

Fastest fold_quad (475 ns) to slowest fold_sequential (2.29 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fold_quad is inconsistent: worst-20% is 2.5x its best-20%

fold_quad's best 20% of batches run at 475 ns but its worst 20% at 1.18 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 474.8 ns median (-58.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.82x (fastest 474.8 ns, slowest 2287.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x239fb80547ae61c2 |
| Counter frequency | 2112775296 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 3255ns | 3027ns | 2478ns | 2994ns | 4812ns | base |
| fold_quad | 2706ns | 2016ns | 2008ns | 2150ns | 5072ns | -16.86% |
| fold_sequential | 3931ns | 4164ns | 3407ns | 3959ns | 4369ns | +20.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 1232ns | 940ns | 1812ns | base | 13.302 |
| fold_quad | 635ns | 475ns | 1181ns | -48.41% | 25.786 |
| fold_sequential | 2155ns | 1872ns | 2389ns | +74.96% | 7.603 |

## Performance model

- Peak throughput: **34.520 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 14.266 | 41.3% |
| fold_quad | 34.507 | 100.0% |
| fold_sequential | 7.163 | 20.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 3255ns | 3255ns | base |
| fold_quad | 2706ns | 2706ns | -16.86% |
| fold_sequential | 3931ns | 3931ns | +20.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| fold_quad | 475ns | -673.6ns (-58.7%) | [-674, -615]ns | [475, 475] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 2287ns | +1138.6ns (+99.1%) | [+944, +1139]ns | [2287, 2287] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -58.7% | +99.2% |
| 1 | 2 | 1149ns | -58.7% | +99.2% |
| 1 | 3 | 940ns | -38.3% | +99.2% |
| 1 | 4 | 1149ns | -58.7% | +99.2% |
| 1 | 5 | 1149ns | -58.7% | +63.0% |
| 1 | 6 | 1148ns | -49.5% | +99.2% |
| 1 | 7 | 1149ns | -58.7% | +63.0% |
| 1 | 8 | 940ns | -49.5% | +144.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.208 | moderate- |
| fold_quad | -0.375 | moderate- |
| fold_sequential | -0.675 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 25.6ns | 1231.7ns | 2.1% |  |
| fold_quad | 26.2ns | 635.4ns | 4.1% |  |
| fold_sequential | 22.5ns | 2155.0ns | 1.0% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 939.7-1811.6 ns)
    939.7 |##########
    983.3 |
   1026.9 |
   1070.5 |
   1114.1 |########################################
   1157.6 |#####
   1201.2 |
   1244.8 |
   1288.4 |
   1332.0 |
   1375.6 |
   1419.2 |
   1462.8 |
   1506.4 |
   1550.0 |
   1593.6 |
   1637.2 |
   1680.8 |
   1724.4 |
   1768.0 |
  (13 below, 10 above range)

fold_quad (n=160, range 474.6-1180.8 ns)
    474.6 |########################################
    509.9 |
    545.2 |##################
    580.5 |#
    615.9 |
    651.2 |
    686.5 |
    721.8 |
    757.1 |
    792.4 |
    827.7 |
    863.0 |
    898.3 |
    933.6 |
    968.9 |
   1004.2 |
   1039.5 |
   1074.8 |
   1110.2 |
   1145.5 |
  (21 below, 19 above range)

fold_sequential (n=160, range 1871.6-2388.6 ns)
   1871.6 |##################
   1897.5 |
   1923.3 |
   1949.2 |
   1975.0 |
   2000.9 |
   2026.7 |
   2052.6 |
   2078.4 |
   2104.3 |
   2130.1 |
   2156.0 |
   2181.8 |
   2207.7 |
   2233.5 |
   2259.4 |
   2285.2 |########################################
   2311.1 |
   2336.9 |#
   2362.8 |
  (17 below, 4 above range)

```

## Diagnostics

- **fold_paired**: CV=41.1% (high variance, measurements may be unstable)
- **fold_quad**: CV=53.6% (high variance, measurements may be unstable)
