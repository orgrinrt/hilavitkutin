# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (142 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.0% of the fastest

All 3 variants sit between 2.29 us and 2.29 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_unchecked's edge over baseline is significant but tiny (-0 ns, 0.02%)

bc_unchecked differs from baseline bc_index by -0 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_unchecked** at 2289.7 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 2289.7 ns, slowest 2290.4 ns)

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
| Counter frequency | 2112761440 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 4090ns | 4172ns | 3698ns | 4172ns | 4237ns | base |
| bc_iter | 3998ns | 4171ns | 3413ns | 4112ns | 4240ns | -2.26% |
| bc_unchecked | 4089ns | 4170ns | 3696ns | 4172ns | 4234ns | -0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 2244ns | 2031ns | 2319ns | base | 7.300 |
| bc_iter | 2194ns | 1875ns | 2324ns | -2.24% | 7.467 |
| bc_unchecked | 2243ns | 2030ns | 2314ns | -0.07% | 7.306 |

## Performance model

- Peak throughput: **8.740 Gops/s** (bc_iter; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 7.154 | 81.9% |
| bc_iter | 7.153 | 81.8% |
| bc_unchecked | 7.156 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 4090ns | 4090ns | base |
| bc_iter | 3998ns | 3998ns | -2.26% |
| bc_unchecked | 4089ns | 4089ns | -0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 2290ns | base | --- | [2290, 2290] | --- | --- | --- | --- |
| bc_iter | 2290ns | no significant difference | [-0, +0]ns | [2290, 2290] | no | 0.6322 | 0.6322 | 3 |
| bc_unchecked | 2290ns | -0.4ns (-0.0%) | [-0, -0]ns | [2290, 2290] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 1874ns | +22.4% | -0.0% |
| 1 | 2 | 2290ns | +0.0% | -0.0% |
| 1 | 3 | 2290ns | +0.0% | -0.0% |
| 1 | 4 | 2290ns | -18.2% | -0.0% |
| 1 | 5 | 2290ns | -18.2% | -0.0% |
| 1 | 6 | 2290ns | +0.0% | -0.0% |
| 1 | 7 | 2290ns | +0.0% | -0.0% |
| 1 | 8 | 2290ns | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.018 | ok |
| bc_iter | 0.292 | moderate+ |
| bc_unchecked | -0.018 | ok |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 1/8
- **bc_unchecked**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 23.6ns | 2244.4ns | 1.1% |  |
| bc_iter | 23.1ns | 2194.1ns | 1.1% |  |
| bc_unchecked | 23.9ns | 2242.7ns | 1.1% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 2030.7-2319.1 ns)
   2030.7 |
   2045.2 |
   2059.6 |
   2074.0 |
   2088.4 |
   2102.8 |
   2117.3 |
   2131.7 |
   2146.1 |
   2160.5 |
   2174.9 |
   2189.4 |
   2203.8 |
   2218.2 |
   2232.6 |
   2247.0 |
   2261.5 |
   2275.9 |########################################
   2290.3 |##########################
   2304.7 |####
  (20 below, 3 above range)

bc_iter (n=160, range 1874.6-2323.6 ns)
   1874.6 |#####
   1897.0 |
   1919.5 |
   1941.9 |
   1964.4 |
   1986.9 |
   2009.3 |
   2031.8 |
   2054.2 |
   2076.7 |
   2099.1 |
   2121.6 |
   2144.0 |
   2166.5 |
   2188.9 |
   2211.4 |
   2233.8 |
   2256.3 |
   2278.7 |########################################
   2301.2 |##############
  (28 below, 1 above range)

bc_unchecked (n=160, range 2030.2-2313.6 ns)
   2030.2 |
   2044.4 |
   2058.6 |
   2072.7 |
   2086.9 |
   2101.1 |
   2115.2 |
   2129.4 |
   2143.6 |
   2157.7 |
   2171.9 |
   2186.1 |
   2200.2 |
   2214.4 |
   2228.6 |
   2242.7 |
   2256.9 |
   2271.1 |
   2285.2 |########################################
   2299.4 |###########
  (20 below, 2 above range)

```
