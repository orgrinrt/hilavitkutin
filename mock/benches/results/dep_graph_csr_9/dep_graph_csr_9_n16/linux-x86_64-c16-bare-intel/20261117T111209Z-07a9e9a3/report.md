# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 394% faster than the next best (dep_graph_dense)

dep_graph_csr (89 ns) leads dep_graph_dense (439 ns) by 394%, a clear separation rather than a photo finish. CV 86.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_hybrid is an outlier: 5.0x slower than the field

dep_graph_hybrid (447 ns) is 5.0x the fastest (89 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 86.1%)

dep_graph_csr wins on median (89 ns) yet has the highest variance (CV 86.1%), while dep_graph_hybrid is the steadiest (CV 24.3%, 447 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dep_graph_csr shows alternating (throttle bounce) (autocorr -0.57)

dep_graph_csr's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (89 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest dep_graph_csr (89 ns) to slowest dep_graph_hybrid (447 ns): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 3.6x its best-20%

dep_graph_csr's best 20% of batches run at 73 ns but its worst 20% at 265 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 88.9 ns median
- 2 variants significantly slower than baseline
- Spread: 5.02x (fastest 88.9 ns, slowest 446.7 ns)

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
| Counter frequency | 2112765565 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dep_graph_csr | 2635ns | 1974ns | 1614ns | 1882ns | 5912ns | base |
| dep_graph_dense | 2421ns | 2344ns | 2090ns | 2345ns | 2981ns | -8.10% |
| dep_graph_hybrid | 2251ns | 2348ns | 1924ns | 2163ns | 2842ns | -14.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 118ns | 73ns | 265ns | base | 0.135 |
| dep_graph_dense | 453ns | 391ns | 557ns | +282.52% | 0.035 |
| dep_graph_hybrid | 429ns | 366ns | 540ns | +262.23% | 0.037 |

## Performance model

- Peak throughput: **0.220 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.180 | 81.8% |
| dep_graph_dense | 0.036 | 16.6% |
| dep_graph_hybrid | 0.036 | 16.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2635ns | 2635ns | base |
| dep_graph_dense | 2421ns | 2421ns | -8.10% |
| dep_graph_hybrid | 2251ns | 2251ns | -14.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 89ns | base | --- | [89, 89] | --- | --- | --- | --- |
| dep_graph_dense | 439ns | +350.6ns (+394.4%) | [+349, +359]ns | [438, 439] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 447ns | +300.4ns (+337.9%) | [+294, +316]ns | [374, 447] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 89ns | +303.1% | +402.8% |
| 1 | 2 | 73ns | +504.2% | +412.8% |
| 1 | 3 | 89ns | +392.6% | +410.9% |
| 1 | 4 | 76ns | +480.4% | +384.3% |
| 1 | 5 | 89ns | +392.7% | +312.5% |
| 1 | 6 | 73ns | +505.7% | +404.4% |
| 1 | 7 | 73ns | +499.6% | +527.5% |
| 1 | 8 | 89ns | +394.7% | +403.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.569 | HIGH- (thermal bounce) |
| dep_graph_dense | -0.036 | ok |
| dep_graph_hybrid | -0.104 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 65.1ns | 118.4ns | 55.0% | HIGH |
| dep_graph_dense | 1020.4ns | 452.7ns | 225.4% | HIGH |
| dep_graph_hybrid | 942.6ns | 428.7ns | 219.9% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 72.7-264.9 ns)
     72.7 |############################
     82.3 |########################################
     91.9 |
    101.5 |
    111.1 |
    120.7 |
    130.4 |
    140.0 |
    149.6 |
    159.2 |
    168.8 |
    178.4 |
    188.0 |
    197.6 |
    207.2 |#
    216.8 |#
    226.5 |
    236.1 |##
    245.7 |#
    255.3 |#
  (7 below, 15 above range)

dep_graph_dense (n=160, range 390.9-557.1 ns)
    390.9 |
    399.3 |
    407.6 |
    415.9 |
    424.2 |
    432.5 |########################################
    440.8 |##
    449.1 |##
    457.4 |
    465.7 |
    474.0 |
    482.4 |
    490.7 |
    499.0 |
    507.3 |
    515.6 |
    523.9 |
    532.2 |
    540.5 |
    548.8 |
  (19 below, 8 above range)

dep_graph_hybrid (n=160, range 366.4-539.6 ns)
    366.4 |########################################
    375.1 |#
    383.7 |
    392.4 |
    401.0 |
    409.7 |
    418.4 |
    427.0 |
    435.7 |
    444.3 |##############################
    453.0 |##############
    461.7 |#
    470.3 |#
    479.0 |
    487.6 |
    496.3 |
    505.0 |
    513.6 |
    522.3 |
    530.9 |
  (11 below, 6 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=64.7% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dep_graph_csr**: bridge=55.6% of algo (FFI overhead may distort results)
- **dep_graph_dense**: CV=25.1% (high variance, measurements may be unstable)
- **dep_graph_dense**: bridge=225.5% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=25.4% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=218.4% of algo (FFI overhead may distort results)
