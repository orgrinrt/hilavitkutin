# replace_value blob install cost across value sizes

3 variants, 160 samples per variant.
Baseline: **sc_v1k**

## Highlights

Baseline for all deltas below: **sc_v1k**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sc_v64b dominates: 3918% faster than the next best (sc_v1k)

sc_v64b (2 ns) leads sc_v1k (68 ns) by 3918%, a clear separation rather than a photo finish. CV 53.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sc_v64b beats baseline by 97% (significant)

sc_v64b is -66 ns (97%) faster than baseline sc_v1k, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sc_v64k is an outlier: 1010.6x slower than the field

sc_v64k (1.72 us) is 1010.6x the fastest (2 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sc_v64b is fastest but the noisiest (CV 53.9%)

sc_v64b wins on median (2 ns) yet has the highest variance (CV 53.9%), while sc_v64k is the steadiest (CV 7.2%, 1.72 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sc_v64k shows alternating (throttle bounce) (autocorr -0.54)

sc_v64k's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 1010.6x the fastest

Fastest sc_v64b (2 ns) to slowest sc_v64k (1.72 us): 1010.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sc_v64b is inconsistent: worst-20% is 4.9x its best-20%

sc_v64b's best 20% of batches run at 1 ns but its worst 20% at 3 ns (4.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sc_v64b** at 1.7 ns median (-97.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1010.56x (fastest 1.7 ns, slowest 1718.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | replace_value blob install cost across value sizes |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sc_v1k | 2197ns | 2197ns | 2136ns | 2192ns | 2271ns | base |
| sc_v64b | 2106ns | 2097ns | 2088ns | 2097ns | 2151ns | -4.12% |
| sc_v64k | 3899ns | 3905ns | 3714ns | 3890ns | 4109ns | +77.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sc_v1k | 69ns | 65ns | 76ns | base | 0.926 |
| sc_v64b | 2ns | 1ns | 3ns | -97.31% | 34.374 |
| sc_v64k | 1725ns | 1625ns | 1850ns | +2395.52% | 0.037 |

## Performance model

- Peak throughput: **98.462 Gops/s** (sc_v64b; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sc_v1k | 0.937 | 1.0% |
| sc_v64b | 37.647 | 38.2% |
| sc_v64k | 0.037 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sc_v1k | 2197ns | 2197ns | base |
| sc_v64b | 2106ns | 2106ns | -4.12% |
| sc_v64k | 3899ns | 3899ns | +77.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sc_v1k | 68ns | base | --- | [68, 69] | --- | --- | --- | --- |
| sc_v64b | 2ns | -66.3ns (-97.1%) | [-67, -66]ns | [2, 2] | YES | 0.0000 | 0.0000 | 0 |
| sc_v64k | 1718ns | +1648.8ns (+2414.1%) | [+1624, +1688]ns | [1691, 1756] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sc_v1k | sc_v64b | sc_v64k |
|---|---|---|---|---|
| 1 | 1 | 68ns | -97.2% | +2326.5% |
| 1 | 2 | 68ns | -96.9% | +2519.5% |
| 1 | 3 | 68ns | -97.5% | +2307.7% |
| 1 | 4 | 71ns | -97.3% | +2255.9% |
| 1 | 5 | 70ns | -97.9% | +2445.5% |
| 1 | 6 | 67ns | -97.2% | +2392.9% |
| 1 | 7 | 69ns | -97.5% | +2458.8% |
| 1 | 8 | 67ns | -97.5% | +2426.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sc_v1k | -0.048 | ok |
| sc_v64b | -0.295 | moderate- |
| sc_v64k | -0.539 | HIGH- (thermal bounce) |

**Consistency summary:**

- **sc_v64b**: won 8/8, lost 0/8
- **sc_v64k**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sc_v1k | 42.1ns | 69.1ns | 60.9% | HIGH |
| sc_v64b | 44.9ns | 1.9ns | 2410.3% | HIGH |
| sc_v64k | 42.7ns | 1724.9ns | 2.5% |  |

## Distribution (algo ns)

```
sc_v1k (n=160, range 65.0-75.7 ns)
     65.0 |##########
     65.5 |#####################
     66.0 |#############################
     66.6 |########################################
     67.1 |##########
     67.6 |##################
     68.2 |###################################
     68.7 |##########
     69.3 |#############################
     69.8 |################
     70.3 |#########################
     70.9 |########
     71.4 |########
     72.0 |########
     72.5 |####
     73.0 |######
     73.6 |####
     74.1 |##########
     74.7 |##
     75.2 |##
  (14 below, 2 above range)

sc_v64b (n=160, range 0.7-3.2 ns)
      0.7 |
      0.8 |################################
      0.9 |
      1.0 |
      1.2 |####################
      1.3 |
      1.4 |
      1.5 |
      1.7 |########################################
      1.8 |
      1.9 |
      2.0 |######################################
      2.2 |
      2.3 |
      2.4 |###########################
      2.5 |
      2.7 |
      2.8 |###############
      2.9 |
      3.0 |
  (11 below, 14 above range)

sc_v64k (n=160, range 1624.8-1850.5 ns)
   1624.8 |########################################
   1636.1 |#
   1647.4 |#############
   1658.6 |##########
   1669.9 |###############
   1681.2 |############
   1692.5 |########
   1703.8 |########
   1715.1 |###
   1726.3 |###
   1737.6 |#####
   1748.9 |###################
   1760.2 |########################################
   1771.5 |##########################
   1782.8 |##########
   1794.0 |######
   1805.3 |#
   1816.6 |##########
   1827.9 |#
   1839.2 |###
  (15 below, 5 above range)

```

## Diagnostics

- **sc_v1k**: bridge=61.1% of algo (FFI overhead may distort results)
- **sc_v64b**: CV=49.2% (high variance, measurements may be unstable)
- **sc_v64b**: worst_20/best_20 = 4.9x (possible bimodal distribution)
- **sc_v64b**: bridge=2600.0% of algo (FFI overhead may distort results)
