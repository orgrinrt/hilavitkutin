# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (1.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 1.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 41% faster than the next best (sat_saturate)

sat_wrap (1.15 us) leads sat_saturate (1.62 us) by 41%, a clear separation rather than a photo finish. CV 44.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -827 ns (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 44.9%)

sat_wrap wins on median (1.15 us) yet has the highest variance (CV 44.9%), while sat_checked is the steadiest (CV 14.9%, 1.97 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sat_wrap shows alternating (throttle bounce) (autocorr -0.62)

sat_wrap's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### sat_wrap is inconsistent: worst-20% is 1.9x its best-20%

sat_wrap's best 20% of batches run at 941 ns but its worst 20% at 1.83 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### sat_saturate's edge over baseline is significant but tiny (-10 ns, 0.51%)

sat_saturate differs from baseline sat_checked by -10 ns (0.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 1150.0 ns median (-41.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.72x (fastest 1150.0 ns, slowest 1972.8 ns)

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
| Counter frequency | 2112805263 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 3776ns | 3858ns | 3159ns | 3811ns | 4290ns | base |
| sat_saturate | 3603ns | 3170ns | 3150ns | 3410ns | 4636ns | -4.58% |
| sat_wrap | 3164ns | 3032ns | 2481ns | 2825ns | 4861ns | -16.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 1928ns | 1615ns | 2183ns | base | 2.125 |
| sat_saturate | 1839ns | 1608ns | 2359ns | -4.60% | 2.227 |
| sat_wrap | 1196ns | 941ns | 1828ns | -37.98% | 3.425 |

## Performance model

- Peak throughput: **4.353 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.076 | 47.7% |
| sat_saturate | 2.524 | 58.0% |
| sat_wrap | 3.562 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 3776ns | 3776ns | base |
| sat_saturate | 3603ns | 3603ns | -4.58% |
| sat_wrap | 3164ns | 3164ns | -16.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 1973ns | base | --- | [1969, 1975] | --- | --- | --- | --- |
| sat_saturate | 1623ns | -10.1ns (-0.5%) | [-18, -7]ns | [1619, 1632] | YES | 0.0000 | 0.0000 | 0 |
| sat_wrap | 1150ns | -826.9ns (-41.9%) | [-834, -824]ns | [956, 1150] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 1977ns | -0.4% | -41.8% |
| 1 | 2 | 1971ns | -18.1% | -52.3% |
| 1 | 3 | 1981ns | -18.3% | -42.0% |
| 1 | 4 | 1975ns | +0.1% | -52.4% |
| 1 | 5 | 1973ns | +0.3% | -52.3% |
| 1 | 6 | 1616ns | -0.2% | -28.8% |
| 1 | 7 | 1619ns | -0.4% | -41.9% |
| 1 | 8 | 1976ns | -18.3% | -41.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | 0.294 | moderate+ |
| sat_saturate | 0.067 | ok |
| sat_wrap | -0.625 | HIGH- (thermal bounce) |

**Consistency summary:**

- **sat_saturate**: won 6/8, lost 1/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 23.1ns | 1928.0ns | 1.2% |  |
| sat_saturate | 22.0ns | 1839.3ns | 1.2% |  |
| sat_wrap | 24.6ns | 1195.8ns | 2.1% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 1615.5-2182.6 ns)
   1615.5 |##########
   1643.8 |
   1672.2 |
   1700.5 |
   1728.9 |
   1757.3 |
   1785.6 |
   1814.0 |
   1842.3 |
   1870.7 |
   1899.0 |
   1927.4 |
   1955.7 |########################################
   1984.1 |######
   2012.4 |
   2040.8 |
   2069.2 |
   2097.5 |
   2125.9 |
   2154.2 |
  (15 below, 4 above range)

sat_saturate (n=160, range 1608.0-2359.4 ns)
   1608.0 |########################################
   1645.6 |
   1683.2 |
   1720.7 |
   1758.3 |
   1795.9 |
   1833.4 |
   1871.0 |
   1908.6 |
   1946.2 |######################
   1983.7 |####
   2021.3 |
   2058.9 |
   2096.4 |
   2134.0 |
   2171.6 |
   2209.2 |
   2246.7 |
   2284.3 |
   2321.9 |
  (12 below, 6 above range)

sat_wrap (n=160, range 940.9-1827.8 ns)
    940.9 |#####################################
    985.2 |
   1029.6 |
   1073.9 |
   1118.3 |########################################
   1162.6 |####
   1207.0 |
   1251.3 |
   1295.7 |
   1340.0 |
   1384.4 |
   1428.7 |
   1473.0 |
   1517.4 |
   1561.7 |
   1606.1 |
   1650.4 |
   1694.8 |
   1739.1 |
   1783.5 |
  (4 below, 12 above range)

```

## Diagnostics

- **sat_saturate**: CV=26.3% (high variance, measurements may be unstable)
- **sat_wrap**: CV=43.2% (high variance, measurements may be unstable)
