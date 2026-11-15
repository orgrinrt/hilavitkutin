# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (2.08 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 289 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 602% faster than the next best (popcount_swar)

popcount_std (289 ns) leads popcount_swar (2.03 us) by 602%, a clear separation rather than a photo finish. CV 23.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 85% (significant)

popcount_std is -1.78 us (85%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 7.2x slower than the field

popcount_handrolled (2.08 us) is 7.2x the fastest (289 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 23.7%)

popcount_std wins on median (289 ns) yet has the highest variance (CV 23.7%), while popcount_handrolled is the steadiest (CV 4.3%, 2.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_std shows alternating (throttle bounce) (autocorr -0.67)

popcount_std's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.2x the fastest

Fastest popcount_std (289 ns) to slowest popcount_handrolled (2.08 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar's edge over baseline is significant but tiny (-29 ns, 1.39%)

popcount_swar differs from baseline popcount_handrolled by -29 ns (1.39%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 288.8 ns median (-86.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.20x (fastest 288.8 ns, slowest 2080.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 4225ns | 4297ns | 4019ns | 4213ns | 4468ns | base |
| popcount_std | 2580ns | 2503ns | 2339ns | 2479ns | 3123ns | -38.95% |
| popcount_swar | 4217ns | 4180ns | 4013ns | 4192ns | 4494ns | -0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 2049ns | 1948ns | 2172ns | base | 7.995 |
| popcount_std | 298ns | 270ns | 357ns | -85.46% | 54.991 |
| popcount_swar | 2047ns | 1946ns | 2187ns | -0.13% | 8.006 |

## Performance model

- Peak throughput: **60.757 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 7.875 | 13.0% |
| popcount_std | 56.731 | 93.4% |
| popcount_swar | 8.080 | 13.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 4225ns | 4225ns | base |
| popcount_std | 2580ns | 2580ns | -38.95% |
| popcount_swar | 4217ns | 4217ns | -0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 2081ns | base | --- | [2017, 2085] | --- | --- | --- | --- |
| popcount_std | 289ns | -1777.9ns (-85.5%) | [-1795, -1728]ns | [287, 290] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 2028ns | -28.9ns (-1.4%) | [-43, -5]ns | [2012, 2060] | YES | 0.0089 | 0.0089 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 1986ns | -86.2% | -1.9% |
| 1 | 2 | 2009ns | -84.9% | +3.7% |
| 1 | 3 | 2087ns | -86.9% | -2.8% |
| 1 | 4 | 2088ns | -86.0% | -6.5% |
| 1 | 5 | 2083ns | -86.6% | -4.1% |
| 1 | 6 | 1950ns | -84.7% | +7.6% |
| 1 | 7 | 2092ns | -86.1% | +0.5% |
| 1 | 8 | 2082ns | -86.3% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.106 | ok |
| popcount_std | -0.672 | HIGH- (thermal bounce) |
| popcount_swar | 0.012 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.5ns | 2049.3ns | 0.2% |  |
| popcount_std | 3.1ns | 297.9ns | 1.0% |  |
| popcount_swar | 3.2ns | 2046.5ns | 0.2% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 1948.2-2172.2 ns)
   1948.2 |######################
   1959.4 |######
   1970.6 |
   1981.8 |##############
   1993.0 |
   2004.2 |#
   2015.4 |#
   2026.6 |
   2037.8 |###
   2049.0 |#
   2060.2 |#
   2071.4 |########
   2082.6 |########################################
   2093.8 |###########
   2105.0 |###
   2116.2 |
   2127.4 |
   2138.6 |#
   2149.8 |
   2161.0 |
  (17 below, 14 above range)

popcount_std (n=160, range 269.7-357.4 ns)
    269.7 |#####################
    274.0 |######################
    278.4 |###########
    282.8 |###########################
    287.2 |########################################
    291.6 |##############################
    296.0 |###############
    300.4 |##########
    304.7 |#####
    309.1 |####
    313.5 |####
    317.9 |##
    322.3 |#
    326.7 |#
    331.1 |
    335.4 |#
    339.8 |#
    344.2 |
    348.6 |
    353.0 |
  (14 below, 5 above range)

popcount_swar (n=160, range 1945.7-2187.3 ns)
   1945.7 |########################################
   1957.7 |####
   1969.8 |####
   1981.9 |##############
   1994.0 |###########
   2006.1 |##########
   2018.2 |#############
   2030.3 |########
   2042.3 |#####
   2054.4 |##
   2066.5 |####
   2078.6 |#########################
   2090.7 |######################################
   2102.8 |##############
   2114.8 |####
   2126.9 |
   2139.0 |
   2151.1 |
   2163.2 |#
   2175.3 |#
  (14 below, 6 above range)

```

## Diagnostics

- **popcount_std**: CV=22.9% (high variance, measurements may be unstable)
