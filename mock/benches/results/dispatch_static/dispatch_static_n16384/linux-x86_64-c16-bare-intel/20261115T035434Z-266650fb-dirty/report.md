# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (2.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_direct at 2.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### dispatch_direct is fastest but the noisiest (CV 16.1%)

dispatch_direct wins on median (2.29 us) yet has the highest variance (CV 16.1%), while dispatch_const_ptr is the steadiest (CV 0.2%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (368 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_trait's comparison is tie-heavy (14% tied pairs)

14% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_direct vs stability leader dispatch_const_ptr (+0% speed for 72.4x steadier)

dispatch_direct is fastest (2.29 us, CV 16.1%); dispatch_const_ptr gives up 0.0% median for 72.4x lower variance (CV 0.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.0% of the fastest

All 3 variants sit between 2.29 us and 2.29 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_direct's edge over baseline is significant but tiny (-0 ns, 0.00%)

dispatch_direct differs from baseline dispatch_const_ptr by -0 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_direct** at 2287.2 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 2287.2 ns, slowest 2287.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 2112812250 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 4174ns | 4167ns | 4159ns | 4168ns | 4205ns | base |
| dispatch_direct | 4051ns | 4164ns | 3408ns | 4111ns | 4514ns | -2.93% |
| dispatch_trait | 3983ns | 4163ns | 3407ns | 4105ns | 4194ns | -4.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2290ns | 2287ns | 2299ns | base | 7.156 |
| dispatch_direct | 2223ns | 1872ns | 2468ns | -2.93% | 7.372 |
| dispatch_trait | 2186ns | 1872ns | 2298ns | -4.52% | 7.494 |

## Performance model

- Peak throughput: **8.754 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 7.163 | 81.8% |
| dispatch_direct | 7.163 | 81.8% |
| dispatch_trait | 7.163 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 4174ns | 4174ns | base |
| dispatch_direct | 4051ns | 4051ns | -2.93% |
| dispatch_trait | 3983ns | 3983ns | -4.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2287ns | base | --- | [2287, 2287] | --- | --- | --- | --- |
| dispatch_direct | 2287ns | -0.1ns (-0.0%) | [-0, -0]ns | [2287, 2287] | YES | 0.0001 | 0.0001 | **20** (12%, HIGH) |
| dispatch_trait | 2287ns | no significant difference | [-0, +0]ns | [2287, 2287] | no (adj: YES) | 0.0212 | 0.0212 | **22** (14%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2287ns | +0.0% | +0.0% |
| 1 | 2 | 2287ns | -0.0% | -0.0% |
| 1 | 3 | 2287ns | -18.2% | -18.2% |
| 1 | 4 | 2287ns | +0.0% | -18.2% |
| 1 | 5 | 2287ns | +0.0% | +0.0% |
| 1 | 6 | 2287ns | -0.0% | +0.0% |
| 1 | 7 | 2287ns | -0.0% | +0.0% |
| 1 | 8 | 2287ns | -18.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.031 | ok |
| dispatch_direct | -0.208 | moderate- |
| dispatch_trait | 0.292 | moderate+ |

**Consistency summary:**

- **dispatch_direct**: won 2/8, lost 0/8
- **dispatch_trait**: won 2/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 23.8ns | 2289.7ns | 1.0% |  |
| dispatch_direct | 22.9ns | 2222.6ns | 1.0% |  |
| dispatch_trait | 22.4ns | 2186.1ns | 1.0% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 2287.0-2299.5 ns)
   2287.0 |########################################
   2287.7 |
   2288.3 |
   2288.9 |
   2289.5 |
   2290.2 |
   2290.8 |
   2291.4 |
   2292.0 |
   2292.6 |
   2293.3 |
   2293.9 |
   2294.5 |
   2295.1 |
   2295.8 |
   2296.4 |
   2297.0 |
   2297.6 |#
   2298.2 |##
   2298.9 |##
  (16 below, 13 above range)

dispatch_direct (n=160, range 1871.7-2467.7 ns)
   1871.7 |###
   1901.5 |
   1931.3 |
   1961.1 |
   1990.9 |
   2020.7 |
   2050.5 |
   2080.3 |
   2110.1 |
   2139.9 |
   2169.7 |
   2199.5 |
   2229.3 |
   2259.1 |########################################
   2288.9 |#########
   2318.7 |
   2348.5 |
   2378.3 |
   2408.1 |
   2437.9 |
  (31 below, 3 above range)

dispatch_trait (n=160, range 1872.2-2298.0 ns)
   1872.2 |####
   1893.5 |
   1914.8 |
   1936.1 |
   1957.4 |
   1978.6 |
   1999.9 |
   2021.2 |
   2042.5 |
   2063.8 |
   2085.1 |
   2106.4 |
   2127.7 |
   2148.9 |
   2170.2 |
   2191.5 |
   2212.8 |
   2234.1 |
   2255.4 |
   2276.7 |########################################
  (29 below, 28 above range)

```
