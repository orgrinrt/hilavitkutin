# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 71% faster than the next best (sat_checked)

sat_wrap (1.15 us) leads sat_checked (1.97 us) by 71%, a clear separation rather than a photo finish. CV 40.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 41% (significant)

sat_wrap is -816 ns (41%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 40.2%)

sat_wrap wins on median (1.15 us) yet has the highest variance (CV 40.2%), while sat_saturate is the steadiest (CV 23.4%, 1.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sat_wrap is inconsistent: worst-20% is 1.9x its best-20%

sat_wrap's best 20% of batches run at 942 ns but its worst 20% at 1.75 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### sat_saturate's edge over baseline is significant but tiny (10 ns, 0.53%)

sat_saturate differs from baseline sat_checked by 10 ns (0.53%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 1150.0 ns median (-41.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.72x (fastest 1150.0 ns, slowest 1973.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 2112797930 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 3799ns | 3848ns | 3151ns | 3684ns | 4794ns | base |
| sat_saturate | 3954ns | 3856ns | 3477ns | 3857ns | 4722ns | +4.06% |
| sat_wrap | 3235ns | 3030ns | 2482ns | 3014ns | 4653ns | -14.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 1941ns | 1611ns | 2439ns | base | 2.110 |
| sat_saturate | 2021ns | 1777ns | 2406ns | +4.09% | 2.027 |
| sat_wrap | 1225ns | 942ns | 1750ns | -36.93% | 3.345 |

## Performance model

- Peak throughput: **4.350 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.081 | 47.8% |
| sat_saturate | 2.076 | 47.7% |
| sat_wrap | 3.562 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 3799ns | 3799ns | base |
| sat_saturate | 3954ns | 3954ns | +4.06% |
| sat_wrap | 3235ns | 3235ns | -14.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 1968ns | base | --- | [1965, 1971] | --- | --- | --- | --- |
| sat_saturate | 1973ns | +10.4ns (+0.5%) | [+5, +17]ns | [1972, 1976] | YES | 0.0006 | 0.0006 | 0 |
| sat_wrap | 1150ns | -816.1ns (-41.5%) | [-821, -811]ns | [1150, 1150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 1973ns | -0.1% | -52.3% |
| 1 | 2 | 1975ns | -18.2% | -41.8% |
| 1 | 3 | 1973ns | +0.2% | -41.7% |
| 1 | 4 | 1614ns | +22.4% | -41.7% |
| 1 | 5 | 1619ns | +21.9% | -29.0% |
| 1 | 6 | 1978ns | -0.1% | -41.9% |
| 1 | 7 | 1974ns | +0.1% | -41.7% |
| 1 | 8 | 1617ns | +22.1% | -28.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | 0.058 | ok |
| sat_saturate | -0.156 | ok |
| sat_wrap | -0.209 | moderate- |

**Consistency summary:**

- **sat_saturate**: won 2/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 23.3ns | 1941.5ns | 1.2% |  |
| sat_saturate | 23.9ns | 2020.9ns | 1.2% |  |
| sat_wrap | 24.9ns | 1224.6ns | 2.0% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 1611.4-2439.4 ns)
   1611.4 |###################
   1652.8 |
   1694.2 |
   1735.6 |
   1777.0 |
   1818.4 |
   1859.8 |
   1901.2 |
   1942.6 |########################################
   1984.0 |#####
   2025.4 |
   2066.8 |
   2108.2 |
   2149.6 |
   2191.0 |
   2232.4 |
   2273.8 |
   2315.2 |
   2356.6 |
   2398.0 |
  (13 below, 7 above range)

sat_saturate (n=160, range 1777.1-2405.7 ns)
   1777.1 |
   1808.5 |
   1840.0 |
   1871.4 |
   1902.8 |
   1934.2 |######
   1965.7 |########################################
   1997.1 |
   2028.5 |
   2060.0 |
   2091.4 |
   2122.8 |
   2154.3 |
   2185.7 |
   2217.1 |
   2248.6 |
   2280.0 |
   2311.4 |
   2342.9 |
   2374.3 |
  (17 below, 6 above range)

sat_wrap (n=160, range 941.5-1749.7 ns)
    941.5 |#
    982.0 |
   1022.4 |
   1062.8 |
   1103.2 |
   1143.6 |########################################
   1184.0 |
   1224.4 |
   1264.8 |
   1305.2 |
   1345.6 |
   1386.1 |
   1426.5 |
   1466.9 |
   1507.3 |
   1547.7 |
   1588.1 |
   1628.5 |
   1668.9 |
   1709.3 |
  (30 below, 11 above range)

```

## Diagnostics

- **sat_checked**: CV=26.9% (high variance, measurements may be unstable)
- **sat_saturate**: CV=22.9% (high variance, measurements may be unstable)
- **sat_wrap**: CV=37.8% (high variance, measurements may be unstable)
