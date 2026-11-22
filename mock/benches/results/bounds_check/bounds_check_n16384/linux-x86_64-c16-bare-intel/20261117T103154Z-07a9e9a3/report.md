# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bc_index) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bc_index has the worst median (2.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bc_iter at 1.88 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bc_iter is fastest but the noisiest (CV 43.4%)

bc_iter wins on median (1.88 us) yet has the highest variance (CV 43.4%), while bc_unchecked is the steadiest (CV 8.1%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (407 ns) is smaller than the fastest variant's own run-to-run std-dev (817 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bc_iter is inconsistent: worst-20% is 1.7x its best-20%

bc_iter's best 20% of batches run at 1.87 us but its worst 20% at 3.17 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bc_iter** at 1883.3 ns median (-17.8% vs baseline)
- Spread: 1.22x (fastest 1883.3 ns, slowest 2289.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 2112876524 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 4169ns | 4174ns | 3416ns | 3890ns | 5762ns | base |
| bc_iter | 4034ns | 3429ns | 3414ns | 3655ns | 5791ns | -3.25% |
| bc_unchecked | 4009ns | 4177ns | 3420ns | 4125ns | 4249ns | -3.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 2227ns | 1874ns | 2869ns | base | 7.358 |
| bc_iter | 2211ns | 1874ns | 3173ns | -0.71% | 7.411 |
| bc_unchecked | 2195ns | 1874ns | 2322ns | -1.40% | 7.463 |

## Performance model

- Peak throughput: **8.744 Gops/s** (bc_index; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 7.155 | 81.8% |
| bc_iter | 8.699 | 99.5% |
| bc_unchecked | 7.155 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 4169ns | 4169ns | base |
| bc_iter | 4034ns | 4034ns | -3.25% |
| bc_unchecked | 4009ns | 4009ns | -3.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 2290ns | base | --- | [1899, 2290] | --- | --- | --- | --- |
| bc_iter | 1883ns | no significant difference | [-112, +0]ns | [1874, 1886] | no | 0.6300 | 0.6300 | 5 |
| bc_unchecked | 2290ns | no significant difference | [-0, +389]ns | [2290, 2290] | no | 0.6300 | 0.5259 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 2551ns | -10.2% | -26.5% |
| 1 | 2 | 2290ns | -18.2% | -0.0% |
| 1 | 3 | 2290ns | -18.2% | -18.2% |
| 1 | 4 | 2185ns | +4.9% | +4.8% |
| 1 | 5 | 1874ns | +0.0% | +22.3% |
| 1 | 6 | 1874ns | +0.0% | +22.2% |
| 1 | 7 | 1874ns | +0.0% | +22.2% |
| 1 | 8 | 2290ns | +15.6% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.415 | moderate+ |
| bc_iter | -0.199 | ok |
| bc_unchecked | -0.207 | moderate- |

**Consistency summary:**

- **bc_iter**: won 3/8, lost 2/8
- **bc_unchecked**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 24.8ns | 2226.6ns | 1.1% |  |
| bc_iter | 23.5ns | 2210.8ns | 1.1% |  |
| bc_unchecked | 23.3ns | 2195.5ns | 1.1% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 1873.7-2868.7 ns)
   1873.7 |######################################
   1923.4 |
   1973.2 |
   2022.9 |
   2072.7 |
   2122.4 |
   2172.2 |
   2221.9 |
   2271.7 |########################################
   2321.4 |
   2371.2 |
   2420.9 |
   2470.7 |
   2520.4 |##########
   2570.2 |##
   2620.0 |
   2669.7 |
   2719.5 |
   2769.2 |
   2819.0 |
  (12 below, 7 above range)

bc_iter (n=160, range 1873.9-3172.8 ns)
   1873.9 |########################################
   1938.8 |
   2003.8 |
   2068.7 |
   2133.6 |
   2198.6 |
   2263.5 |####################
   2328.5 |
   2393.4 |
   2458.4 |
   2523.3 |
   2588.3 |
   2653.2 |
   2718.1 |
   2783.1 |
   2848.0 |
   2913.0 |
   2977.9 |
   3042.9 |
   3107.8 |
  (9 below, 9 above range)

bc_unchecked (n=160, range 1874.0-2322.2 ns)
   1874.0 |####
   1896.4 |
   1918.8 |
   1941.2 |
   1963.6 |
   1986.0 |
   2008.4 |
   2030.9 |
   2053.3 |
   2075.7 |
   2098.1 |
   2120.5 |
   2142.9 |
   2165.3 |
   2187.8 |
   2210.2 |
   2232.6 |
   2255.0 |
   2277.4 |########################################
   2299.8 |########
  (28 below, 4 above range)

```

## Diagnostics

- **bc_index**: CV=22.4% (high variance, measurements may be unstable)
- **bc_iter**: CV=36.9% (high variance, measurements may be unstable)
