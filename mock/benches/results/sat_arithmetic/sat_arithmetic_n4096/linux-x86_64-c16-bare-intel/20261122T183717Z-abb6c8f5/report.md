# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 41% faster than the next best (sat_checked)

sat_wrap (1.15 us) leads sat_checked (1.63 us) by 41%, a clear separation rather than a photo finish. CV 39.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 41% (significant)

sat_wrap is -666 ns (41%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 39.9%)

sat_wrap wins on median (1.15 us) yet has the highest variance (CV 39.9%), while sat_saturate is the steadiest (CV 19.1%, 1.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sat_wrap is inconsistent: worst-20% is 1.7x its best-20%

sat_wrap's best 20% of batches run at 1.04 us but its worst 20% at 1.72 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### sat_saturate's edge over baseline is significant but tiny (17 ns, 1.06%)

sat_saturate differs from baseline sat_checked by 17 ns (1.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 1150.1 ns median (-29.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.71x (fastest 1150.1 ns, slowest 1967.1 ns)

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
| Counter frequency | 2112769626 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 3677ns | 3169ns | 3145ns | 3442ns | 4913ns | base |
| sat_saturate | 3715ns | 3851ns | 3149ns | 3682ns | 4382ns | +1.05% |
| sat_wrap | 3281ns | 3033ns | 2739ns | 3035ns | 4565ns | -10.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 1875ns | 1605ns | 2497ns | base | 2.184 |
| sat_saturate | 1897ns | 1609ns | 2228ns | +1.13% | 2.160 |
| sat_wrap | 1242ns | 1039ns | 1720ns | -33.77% | 3.298 |

## Performance model

- Peak throughput: **3.942 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.520 | 63.9% |
| sat_saturate | 2.082 | 52.8% |
| sat_wrap | 3.561 | 90.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 3677ns | 3677ns | base |
| sat_saturate | 3715ns | 3715ns | +1.05% |
| sat_wrap | 3281ns | 3281ns | -10.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 1625ns | base | --- | [1620, 1960] | --- | --- | --- | --- |
| sat_saturate | 1967ns | +17.2ns (+1.1%) | [+4, +339]ns | [1962, 1970] | YES | 0.0325 | 0.0325 | 0 |
| sat_wrap | 1150ns | -666.2ns (-41.0%) | [-684, -475]ns | [1150, 1150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 1982ns | -18.7% | -42.0% |
| 1 | 2 | 1972ns | -0.0% | -41.7% |
| 1 | 3 | 1610ns | +0.3% | -28.6% |
| 1 | 4 | 1613ns | +22.4% | -28.7% |
| 1 | 5 | 1612ns | +22.3% | -41.6% |
| 1 | 6 | 1631ns | +21.2% | -29.5% |
| 1 | 7 | 1976ns | -18.1% | -41.8% |
| 1 | 8 | 1613ns | +22.2% | -28.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | 0.071 | ok |
| sat_saturate | -0.471 | moderate- |
| sat_wrap | -0.161 | ok |

**Consistency summary:**

- **sat_saturate**: won 2/8, lost 5/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 22.8ns | 1875.3ns | 1.2% |  |
| sat_saturate | 22.7ns | 1896.5ns | 1.2% |  |
| sat_wrap | 25.7ns | 1242.0ns | 2.1% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 1605.0-2496.5 ns)
   1605.0 |########################################
   1649.6 |#
   1694.2 |
   1738.7 |
   1783.3 |
   1827.9 |
   1872.5 |
   1917.1 |##
   1961.6 |############################
   2006.2 |
   2050.8 |
   2095.4 |
   2139.9 |
   2184.5 |
   2229.1 |
   2273.7 |
   2318.2 |
   2362.8 |
   2407.4 |
   2452.0 |
  (12 below, 9 above range)

sat_saturate (n=160, range 1608.9-2227.8 ns)
   1608.9 |####################
   1639.9 |
   1670.8 |
   1701.8 |
   1732.7 |
   1763.7 |
   1794.6 |
   1825.6 |
   1856.5 |
   1887.4 |
   1918.4 |
   1949.3 |########################################
   1980.3 |##########
   2011.2 |
   2042.2 |
   2073.1 |
   2104.0 |
   2135.0 |
   2165.9 |
   2196.9 |
  (16 below, 5 above range)

sat_wrap (n=160, range 1039.2-1720.5 ns)
   1039.2 |
   1073.3 |
   1107.3 |
   1141.4 |########################################
   1175.5 |
   1209.5 |
   1243.6 |
   1277.6 |
   1311.7 |
   1345.8 |
   1379.8 |
   1413.9 |
   1448.0 |
   1482.0 |
   1516.1 |
   1550.2 |
   1584.2 |
   1618.3 |
   1652.4 |
   1686.4 |
  (17 below, 10 above range)

```

## Diagnostics

- **sat_checked**: CV=27.7% (high variance, measurements may be unstable)
- **sat_wrap**: CV=37.0% (high variance, measurements may be unstable)
