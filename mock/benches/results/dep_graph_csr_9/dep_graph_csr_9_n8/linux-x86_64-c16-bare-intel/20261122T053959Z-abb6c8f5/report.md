# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 182% faster than the next best (dep_graph_dense)

dep_graph_csr (36 ns) leads dep_graph_dense (103 ns) by 182%, a clear separation rather than a photo finish. CV 105.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_hybrid is an outlier: 3.4x slower than the field

dep_graph_hybrid (124 ns) is 3.4x the fastest (36 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 105.5%)

dep_graph_csr wins on median (36 ns) yet has the highest variance (CV 105.5%), while dep_graph_hybrid is the steadiest (CV 43.0%, 124 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dep_graph_hybrid shows alternating (throttle bounce) (autocorr -0.64)

dep_graph_hybrid's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (36 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest dep_graph_csr (36 ns) to slowest dep_graph_hybrid (124 ns): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 3.6x its best-20%

dep_graph_csr's best 20% of batches run at 35 ns but its worst 20% at 128 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 36.4 ns median
- 2 variants significantly slower than baseline
- Spread: 3.41x (fastest 36.4 ns, slowest 124.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x13579bdf2468ace0 |
| Counter frequency | 2112878130 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dep_graph_csr | 2469ns | 1580ns | 1568ns | 1700ns | 5677ns | base |
| dep_graph_dense | 2052ns | 1661ns | 1652ns | 1740ns | 3389ns | -16.88% |
| dep_graph_hybrid | 2135ns | 2022ns | 1654ns | 1938ns | 3204ns | -13.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 56ns | 35ns | 128ns | base | 0.143 |
| dep_graph_dense | 125ns | 100ns | 207ns | +124.28% | 0.064 |
| dep_graph_hybrid | 131ns | 101ns | 195ns | +133.93% | 0.061 |

## Performance model

- Peak throughput: **0.227 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.220 | 96.9% |
| dep_graph_dense | 0.078 | 34.4% |
| dep_graph_hybrid | 0.064 | 28.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2469ns | 2469ns | base |
| dep_graph_dense | 2052ns | 2052ns | -16.88% |
| dep_graph_hybrid | 2135ns | 2135ns | -13.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 36ns | base | --- | [36, 40] | --- | --- | --- | --- |
| dep_graph_dense | 103ns | +65.5ns (+179.9%) | [+65, +67]ns | [102, 103] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 124ns | +80.7ns (+221.6%) | [+79, +82]ns | [124, 124] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 35ns | +190.0% | +251.0% |
| 1 | 2 | 35ns | +184.3% | +187.4% |
| 1 | 3 | 37ns | +182.4% | +238.2% |
| 1 | 4 | 35ns | +183.6% | +187.9% |
| 1 | 5 | 36ns | +182.6% | +248.9% |
| 1 | 6 | 35ns | +254.7% | +185.3% |
| 1 | 7 | 45ns | +175.3% | +183.4% |
| 1 | 8 | 45ns | +126.5% | +180.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.424 | moderate+ |
| dep_graph_dense | 0.242 | moderate+ |
| dep_graph_hybrid | -0.638 | HIGH- (thermal bounce) |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 63.9ns | 55.9ns | 114.3% | HIGH |
| dep_graph_dense | 1001.3ns | 125.4ns | 798.3% | HIGH |
| dep_graph_hybrid | 1041.1ns | 130.8ns | 795.7% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 35.3-127.6 ns)
     35.3 |########################################
     39.9 |#######
     44.5 |########
     49.1 |
     53.7 |
     58.4 |
     63.0 |
     67.6 |
     72.2 |
     76.8 |
     81.4 |
     86.0 |
     90.7 |
     95.3 |
     99.9 |
    104.5 |
    109.1 |
    113.7 |
    118.3 |
    122.9 |#
  (4 below, 19 above range)

dep_graph_dense (n=160, range 100.4-207.1 ns)
    100.4 |########################################
    105.7 |
    111.1 |
    116.4 |
    121.7 |##############
    127.1 |
    132.4 |
    137.7 |
    143.1 |
    148.4 |
    153.7 |
    159.1 |
    164.4 |
    169.7 |
    175.1 |
    180.4 |
    185.7 |
    191.1 |
    196.4 |
    201.7 |
  (10 below, 12 above range)

dep_graph_hybrid (n=160, range 101.3-195.4 ns)
    101.3 |##########################
    106.0 |
    110.7 |
    115.4 |
    120.1 |########################################
    124.8 |######################
    129.5 |
    134.2 |
    138.9 |
    143.6 |
    148.3 |
    153.1 |
    157.8 |
    162.5 |
    167.2 |
    171.9 |
    176.6 |
    181.3 |
    186.0 |
    190.7 |
  (16 below, 13 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=68.7% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dep_graph_csr**: bridge=113.5% of algo (FFI overhead may distort results)
- **dep_graph_dense**: CV=48.4% (high variance, measurements may be unstable)
- **dep_graph_dense**: bridge=791.7% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=40.9% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=796.3% of algo (FFI overhead may distort results)
