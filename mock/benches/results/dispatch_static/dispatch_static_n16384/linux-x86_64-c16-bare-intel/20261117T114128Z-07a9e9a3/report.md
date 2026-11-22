# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (2.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_direct at 1.88 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### dispatch_direct dominates: 22% faster than the next best (dispatch_trait)

dispatch_direct (1.88 us) leads dispatch_trait (2.29 us) by 22%, a clear separation rather than a photo finish. CV 23.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dispatch_direct is fastest but the noisiest (CV 23.8%)

dispatch_direct wins on median (1.88 us) yet has the highest variance (CV 23.8%), while dispatch_trait is the steadiest (CV 18.1%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dispatch_const_ptr shows alternating (throttle bounce) (autocorr -0.63)

dispatch_const_ptr's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (406 ns) is smaller than the fastest variant's own run-to-run std-dev (447 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_direct's edge over baseline is significant but tiny (0 ns, 0.00%)

dispatch_direct differs from baseline dispatch_const_ptr by 0 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_direct** at 1881.0 ns median (-17.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 1881.0 ns, slowest 2287.1 ns)

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
| Counter frequency | 2112808726 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 3930ns | 4168ns | 3411ns | 3836ns | 4730ns | base |
| dispatch_direct | 3819ns | 3425ns | 3412ns | 3666ns | 4685ns | -2.82% |
| dispatch_trait | 3905ns | 4167ns | 3412ns | 3829ns | 4626ns | -0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2152ns | 1871ns | 2580ns | base | 7.614 |
| dispatch_direct | 2091ns | 1871ns | 2555ns | -2.83% | 7.836 |
| dispatch_trait | 2138ns | 1871ns | 2524ns | -0.66% | 7.665 |

## Performance model

- Peak throughput: **8.755 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 7.164 | 81.8% |
| dispatch_direct | 8.710 | 99.5% |
| dispatch_trait | 7.164 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 3930ns | 3930ns | base |
| dispatch_direct | 3819ns | 3819ns | -2.82% |
| dispatch_trait | 3905ns | 3905ns | -0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2287ns | base | --- | [1882, 2287] | --- | --- | --- | --- |
| dispatch_direct | 1881ns | +0.1ns (+0.0%) | [+0, +0]ns | [1872, 1883] | YES | 0.0038 | 0.0019 | 9 |
| dispatch_trait | 2287ns | no significant difference | [+0, +0]ns | [1882, 2287] | no | 0.1033 | 0.1033 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2287ns | +0.0% | +0.0% |
| 1 | 2 | 1871ns | +0.0% | +0.0% |
| 1 | 3 | 2287ns | -18.2% | -18.2% |
| 1 | 4 | 1871ns | +0.0% | +22.2% |
| 1 | 5 | 1871ns | +0.0% | +0.0% |
| 1 | 6 | 2287ns | +0.0% | +0.0% |
| 1 | 7 | 1871ns | +0.0% | +22.2% |
| 1 | 8 | 2287ns | +0.0% | -18.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.625 | HIGH- (thermal bounce) |
| dispatch_direct | -0.275 | moderate- |
| dispatch_trait | -0.375 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 0/8
- **dispatch_trait**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 22.3ns | 2151.8ns | 1.0% |  |
| dispatch_direct | 21.6ns | 2091.0ns | 1.0% |  |
| dispatch_trait | 22.3ns | 2137.6ns | 1.0% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 1871.3-2580.4 ns)
   1871.3 |#######################################
   1906.8 |
   1942.2 |
   1977.7 |
   2013.1 |
   2048.6 |
   2084.0 |
   2119.5 |
   2154.9 |
   2190.4 |
   2225.8 |
   2261.3 |########################################
   2296.8 |################
   2332.2 |
   2367.7 |
   2403.1 |
   2438.6 |
   2474.0 |
   2509.5 |
   2544.9 |
  (20 below, 6 above range)

dispatch_direct (n=160, range 1871.4-2555.5 ns)
   1871.4 |########################################
   1905.6 |
   1939.8 |
   1974.0 |
   2008.2 |
   2042.4 |
   2076.6 |
   2110.8 |
   2145.0 |
   2179.2 |
   2213.4 |
   2247.6 |
   2281.8 |##########################
   2316.0 |
   2350.2 |
   2384.5 |
   2418.7 |
   2452.9 |
   2487.1 |
   2521.3 |
  (8 below, 4 above range)

dispatch_trait (n=160, range 1871.3-2523.6 ns)
   1871.3 |##################################
   1903.9 |
   1936.5 |
   1969.1 |
   2001.8 |
   2034.4 |
   2067.0 |
   2099.6 |
   2132.2 |
   2164.8 |
   2197.4 |
   2230.0 |
   2262.7 |########################################
   2295.3 |############
   2327.9 |
   2360.5 |
   2393.1 |
   2425.7 |
   2458.3 |
   2490.9 |
  (25 below, 4 above range)

```

## Diagnostics

- **dispatch_direct**: CV=21.4% (high variance, measurements may be unstable)
