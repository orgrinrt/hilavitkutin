# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2.29 us to slowest 2.29 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### dispatch_const_ptr is fastest but the noisiest (CV 12.5%)

dispatch_const_ptr wins on median (2.29 us) yet has the highest variance (CV 12.5%), while dispatch_trait is the steadiest (CV 12.4%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (286 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (2.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (10% tied pairs)

10% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_trait (+0% speed for 1.0x steadier)

dispatch_const_ptr is fastest (2.29 us, CV 12.5%); dispatch_trait gives up 0.0% median for 1.0x lower variance (CV 12.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 2287.1 ns median
- Spread: 1.00x (fastest 2287.1 ns, slowest 2287.2 ns)

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
| Counter frequency | 2112810234 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 3848ns | 4159ns | 3406ns | 3818ns | 4381ns | base |
| dispatch_direct | 3924ns | 4164ns | 3407ns | 3959ns | 4339ns | +1.98% |
| dispatch_trait | 3940ns | 4163ns | 3407ns | 3965ns | 4398ns | +2.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2110ns | 1871ns | 2390ns | base | 7.766 |
| dispatch_direct | 2151ns | 1871ns | 2370ns | +1.95% | 7.617 |
| dispatch_trait | 2160ns | 1871ns | 2402ns | +2.39% | 7.584 |

## Performance model

- Peak throughput: **8.755 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 7.164 | 81.8% |
| dispatch_direct | 7.163 | 81.8% |
| dispatch_trait | 7.163 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 3848ns | 3848ns | base |
| dispatch_direct | 3924ns | 3924ns | +1.98% |
| dispatch_trait | 3940ns | 3940ns | +2.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2287ns | base | --- | [1882, 2287] | --- | --- | --- | --- |
| dispatch_direct | 2287ns | no significant difference | [+0, +0]ns | [2287, 2287] | no | 0.2819 | 0.2786 | 16 |
| dispatch_trait | 2287ns | no significant difference | [+0, +0]ns | [2287, 2287] | no | 0.2819 | 0.2819 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 1871ns | +0.0% | +22.2% |
| 1 | 2 | 2287ns | +0.0% | +0.0% |
| 1 | 3 | 2287ns | +0.0% | -0.0% |
| 1 | 4 | 2287ns | -0.0% | +0.0% |
| 1 | 5 | 2287ns | -18.2% | -18.2% |
| 1 | 6 | 1871ns | +22.2% | +22.2% |
| 1 | 7 | 1872ns | +22.2% | -0.0% |
| 1 | 8 | 1872ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.375 | moderate+ |
| dispatch_direct | -0.275 | moderate- |
| dispatch_trait | 0.059 | ok |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 2/8
- **dispatch_trait**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 21.7ns | 2109.8ns | 1.0% |  |
| dispatch_direct | 22.2ns | 2151.0ns | 1.0% |  |
| dispatch_trait | 22.3ns | 2160.3ns | 1.0% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 1871.4-2390.0 ns)
   1871.4 |#################################
   1897.3 |
   1923.2 |
   1949.2 |
   1975.1 |
   2001.0 |
   2027.0 |
   2052.9 |
   2078.8 |
   2104.8 |
   2130.7 |
   2156.6 |
   2182.6 |
   2208.5 |
   2234.4 |
   2260.4 |
   2286.3 |########################################
   2312.2 |
   2338.1 |
   2364.1 |
  (9 below, 3 above range)

dispatch_direct (n=160, range 1871.4-2370.2 ns)
   1871.4 |#######################
   1896.3 |
   1921.3 |
   1946.2 |
   1971.1 |
   1996.1 |
   2021.0 |
   2046.0 |
   2070.9 |
   2095.8 |
   2120.8 |
   2145.7 |
   2170.6 |
   2195.6 |
   2220.5 |
   2245.5 |
   2270.4 |########################################
   2295.3 |#########
   2320.3 |
   2345.2 |
  (11 below, 1 above range)

dispatch_trait (n=160, range 1871.4-2402.2 ns)
   1871.4 |#########################
   1897.9 |
   1924.5 |
   1951.0 |
   1977.5 |
   2004.1 |
   2030.6 |
   2057.2 |
   2083.7 |
   2110.3 |
   2136.8 |
   2163.3 |
   2189.9 |
   2216.4 |
   2243.0 |
   2269.5 |########################################
   2296.1 |###########
   2322.6 |
   2349.2 |
   2375.7 |
  (9 below, 2 above range)

```
