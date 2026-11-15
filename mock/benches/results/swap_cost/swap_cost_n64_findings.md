# replace_value blob install cost across value sizes

3 variants, 160 samples per variant.
Baseline: **sc_v1k**

## Highlights

Baseline for all deltas below: **sc_v1k**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sc_v64b dominates: 3281% faster than the next best (sc_v1k)

sc_v64b (2 ns) leads sc_v1k (71 ns) by 3281%, a clear separation rather than a photo finish. CV 47.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sc_v64b beats baseline by 97% (significant)

sc_v64b is -69 ns (97%) faster than baseline sc_v1k, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sc_v64k is an outlier: 815.2x slower than the field

sc_v64k (1.71 us) is 815.2x the fastest (2 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sc_v64b is fastest but the noisiest (CV 47.4%)

sc_v64b wins on median (2 ns) yet has the highest variance (CV 47.4%), while sc_v64k is the steadiest (CV 5.8%, 1.71 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 815.2x the fastest

Fastest sc_v64b (2 ns) to slowest sc_v64k (1.71 us): 815.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sc_v64b is inconsistent: worst-20% is 3.8x its best-20%

sc_v64b's best 20% of batches run at 1 ns but its worst 20% at 4 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sc_v64b** at 2.1 ns median (-97.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 815.19x (fastest 2.1 ns, slowest 1711.9 ns)

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
| sc_v1k | 2297ns | 2281ns | 2156ns | 2285ns | 2473ns | base |
| sc_v64b | 2232ns | 2237ns | 2180ns | 2236ns | 2270ns | -2.82% |
| sc_v64k | 3956ns | 3931ns | 3820ns | 3934ns | 4159ns | +72.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sc_v1k | 71ns | 64ns | 77ns | base | 0.907 |
| sc_v64b | 2ns | 1ns | 4ns | -96.86% | 28.845 |
| sc_v64k | 1739ns | 1673ns | 1874ns | +2363.77% | 0.037 |

## Performance model

- Peak throughput: **67.368 Gops/s** (sc_v64b; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sc_v1k | 0.901 | 1.3% |
| sc_v64b | 30.476 | 45.2% |
| sc_v64k | 0.037 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sc_v1k | 2297ns | 2297ns | base |
| sc_v64b | 2232ns | 2232ns | -2.82% |
| sc_v64k | 3956ns | 3956ns | +72.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sc_v1k | 71ns | base | --- | [70, 72] | --- | --- | --- | --- |
| sc_v64b | 2ns | -68.7ns (-96.8%) | [-69, -68]ns | [2, 2] | YES | 0.0000 | 0.0000 | 0 |
| sc_v64k | 1712ns | +1640.4ns (+2310.4%) | [+1632, +1650]ns | [1704, 1721] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sc_v1k | sc_v64b | sc_v64k |
|---|---|---|---|---|
| 1 | 1 | 66ns | -96.2% | +2456.8% |
| 1 | 2 | 64ns | -96.1% | +2673.4% |
| 1 | 3 | 74ns | -97.7% | +2249.5% |
| 1 | 4 | 71ns | -96.8% | +2267.9% |
| 1 | 5 | 73ns | -97.4% | +2238.5% |
| 1 | 6 | 71ns | -97.0% | +2332.3% |
| 1 | 7 | 70ns | -97.0% | +2318.4% |
| 1 | 8 | 73ns | -96.6% | +2237.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sc_v1k | 0.126 | ok |
| sc_v64b | -0.203 | moderate- |
| sc_v64k | -0.080 | ok |

**Consistency summary:**

- **sc_v64b**: won 8/8, lost 0/8
- **sc_v64k**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sc_v1k | 45.5ns | 70.6ns | 64.5% | HIGH |
| sc_v64b | 48.2ns | 2.2ns | 2173.7% | HIGH |
| sc_v64k | 46.5ns | 1739.0ns | 2.7% |  |

## Distribution (algo ns)

```
sc_v1k (n=160, range 63.8-76.6 ns)
     63.8 |#####
     64.5 |#################
     65.1 |###########
     65.8 |##############
     66.4 |###########
     67.0 |#################
     67.7 |###############################
     68.3 |#####
     69.0 |##############
     69.6 |####################
     70.2 |########################################
     70.9 |#########################
     71.5 |##################################
     72.2 |######################
     72.8 |######################
     73.4 |#################
     74.1 |###############################
     74.7 |##
     75.4 |############################
     76.0 |##
  (14 below, 14 above range)

sc_v64b (n=160, range 0.9-3.6 ns)
      0.9 |
      1.1 |########################################
      1.2 |
      1.4 |
      1.5 |
      1.6 |#################################
      1.8 |
      1.9 |
      2.0 |#################################
      2.2 |
      2.3 |
      2.4 |####################################
      2.6 |
      2.7 |
      2.8 |######################################
      3.0 |
      3.1 |
      3.2 |###################
      3.4 |
      3.5 |
  (18 below, 16 above range)

sc_v64k (n=160, range 1673.2-1874.3 ns)
   1673.2 |#############################
   1683.2 |#############################
   1693.3 |########################################
   1703.4 |################################
   1713.4 |##################
   1723.5 |#######################
   1733.5 |##############
   1743.6 |##################
   1753.6 |##########
   1763.7 |##########
   1773.7 |#######
   1783.8 |#######
   1793.8 |#
   1803.9 |#####
   1814.0 |#
   1824.0 |
   1834.1 |
   1844.1 |
   1854.2 |#
   1864.2 |
  (11 below, 10 above range)

```

## Diagnostics

- **sc_v1k**: bridge=63.7% of algo (FFI overhead may distort results)
- **sc_v64b**: CV=44.8% (high variance, measurements may be unstable)
- **sc_v64b**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sc_v64b**: bridge=2290.5% of algo (FFI overhead may distort results)
