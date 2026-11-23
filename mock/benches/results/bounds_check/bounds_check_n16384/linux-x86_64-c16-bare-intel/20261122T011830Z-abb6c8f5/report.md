# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_unchecked is fastest but the noisiest (CV 19.5%)

bc_unchecked wins on median (2.29 us) yet has the highest variance (CV 19.5%), while bc_index is the steadiest (CV 10.2%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (446 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bc_unchecked vs stability leader bc_index (+0% speed for 1.9x steadier)

bc_unchecked is fastest (2.29 us, CV 19.5%); bc_index gives up 0.0% median for 1.9x lower variance (CV 10.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.0% of the fastest

All 3 variants sit between 2.29 us and 2.29 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_iter's edge over baseline is significant but tiny (0 ns, 0.02%)

bc_iter differs from baseline bc_index by 0 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_unchecked** at 2289.6 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 2289.6 ns, slowest 2290.6 ns)

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
| Counter frequency | 2112744983 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 4039ns | 4170ns | 3414ns | 4126ns | 4406ns | base |
| bc_iter | 4162ns | 4175ns | 3746ns | 4177ns | 4533ns | +3.03% |
| bc_unchecked | 4027ns | 4169ns | 3409ns | 3991ns | 4754ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 2215ns | 1875ns | 2402ns | base | 7.398 |
| bc_iter | 2282ns | 2058ns | 2477ns | +3.04% | 7.180 |
| bc_unchecked | 2208ns | 1873ns | 2594ns | -0.30% | 7.420 |

## Performance model

- Peak throughput: **8.746 Gops/s** (bc_unchecked; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 7.155 | 81.8% |
| bc_iter | 7.153 | 81.8% |
| bc_unchecked | 7.156 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 4039ns | 4039ns | base |
| bc_iter | 4162ns | 4162ns | +3.03% |
| bc_unchecked | 4027ns | 4027ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 2290ns | base | --- | [2290, 2290] | --- | --- | --- | --- |
| bc_iter | 2291ns | +0.5ns (+0.0%) | [+0, +1]ns | [2290, 2291] | YES | 0.0002 | 0.0002 | 4 |
| bc_unchecked | 2290ns | -0.5ns (-0.0%) | [-1, -0]ns | [2290, 2290] | YES | 0.0000 | 0.0000 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 2290ns | -18.2% | -0.0% |
| 1 | 2 | 2290ns | +0.0% | -0.0% |
| 1 | 3 | 1874ns | +22.2% | -0.0% |
| 1 | 4 | 2290ns | +0.0% | -0.0% |
| 1 | 5 | 2290ns | +0.0% | -18.2% |
| 1 | 6 | 2290ns | +0.0% | -18.2% |
| 1 | 7 | 1874ns | +22.3% | +22.2% |
| 1 | 8 | 2290ns | +0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.375 | moderate- |
| bc_iter | -0.017 | ok |
| bc_unchecked | -0.142 | ok |

**Consistency summary:**

- **bc_iter**: won 1/8, lost 2/8
- **bc_unchecked**: won 2/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 23.4ns | 2214.6ns | 1.1% |  |
| bc_iter | 24.2ns | 2282.0ns | 1.1% |  |
| bc_unchecked | 23.6ns | 2208.0ns | 1.1% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 1874.7-2402.1 ns)
   1874.7 |####
   1901.0 |
   1927.4 |
   1953.8 |
   1980.2 |
   2006.5 |
   2032.9 |
   2059.3 |
   2085.7 |
   2112.0 |
   2138.4 |
   2164.8 |
   2191.2 |
   2217.5 |
   2243.9 |
   2270.3 |########################################
   2296.6 |##############
   2323.0 |
   2349.4 |
   2375.8 |
  (28 below, 5 above range)

bc_iter (n=160, range 2058.1-2477.2 ns)
   2058.1 |
   2079.1 |
   2100.0 |
   2121.0 |
   2141.9 |
   2162.9 |
   2183.9 |
   2204.8 |
   2225.8 |
   2246.7 |
   2267.7 |
   2288.6 |########################################
   2309.6 |
   2330.5 |
   2351.5 |
   2372.5 |
   2393.4 |
   2414.4 |
   2435.3 |
   2456.3 |
  (18 below, 3 above range)

bc_unchecked (n=160, range 1873.3-2593.9 ns)
   1873.3 |##############
   1909.4 |
   1945.4 |
   1981.4 |
   2017.4 |
   2053.5 |
   2089.5 |
   2125.5 |
   2161.5 |
   2197.6 |
   2233.6 |
   2269.6 |########################################
   2305.7 |
   2341.7 |
   2377.7 |
   2413.7 |
   2449.8 |
   2485.8 |
   2521.8 |
   2557.8 |
  (19 below, 5 above range)

```

## Diagnostics

- **bc_unchecked**: CV=20.2% (high variance, measurements may be unstable)
