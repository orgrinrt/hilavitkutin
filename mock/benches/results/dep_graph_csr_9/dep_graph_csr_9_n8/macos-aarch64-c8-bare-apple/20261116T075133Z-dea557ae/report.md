# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 300% faster than the next best (dep_graph_hybrid)

dep_graph_csr (25 ns) leads dep_graph_hybrid (102 ns) by 300%, a clear separation rather than a photo finish. CV 20.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 21.8x slower than the field

dep_graph_dense (554 ns) is 21.8x the fastest (25 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 20.2%)

dep_graph_csr wins on median (25 ns) yet has the highest variance (CV 20.2%), while dep_graph_hybrid is the steadiest (CV 2.9%, 102 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dep_graph_csr shows alternating (throttle bounce) (autocorr -0.68)

dep_graph_csr's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (25 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 21.8x the fastest

Fastest dep_graph_csr (25 ns) to slowest dep_graph_dense (554 ns): 21.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 25.4 ns median
- 2 variants significantly slower than baseline
- Spread: 21.80x (fastest 25.4 ns, slowest 553.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dep_graph_csr | 2105ns | 2092ns | 2076ns | 2101ns | 2146ns | base |
| dep_graph_dense | 2671ns | 2665ns | 2615ns | 2665ns | 2744ns | +26.90% |
| dep_graph_hybrid | 2213ns | 2215ns | 2175ns | 2207ns | 2270ns | +5.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 26ns | 23ns | 30ns | base | 0.312 |
| dep_graph_dense | 556ns | 538ns | 582ns | +2067.79% | 0.014 |
| dep_graph_hybrid | 102ns | 98ns | 106ns | +296.90% | 0.079 |

## Performance model

- Peak throughput: **0.353 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.315 | 89.2% |
| dep_graph_dense | 0.014 | 4.1% |
| dep_graph_hybrid | 0.079 | 22.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2105ns | 2105ns | base |
| dep_graph_dense | 2671ns | 2671ns | +26.90% |
| dep_graph_hybrid | 2213ns | 2213ns | +5.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 25ns | base | --- | [25, 25] | --- | --- | --- | --- |
| dep_graph_dense | 554ns | +528.3ns (+2079.9%) | [+526, +531]ns | [551, 556] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 102ns | +76.3ns (+300.4%) | [+76, +77]ns | [101, 102] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 25ns | +2105.6% | +301.8% |
| 1 | 2 | 25ns | +2171.3% | +303.5% |
| 1 | 3 | 25ns | +2103.2% | +308.3% |
| 1 | 4 | 26ns | +2005.0% | +299.0% |
| 1 | 5 | 25ns | +2080.0% | +305.8% |
| 1 | 6 | 26ns | +1994.5% | +281.7% |
| 1 | 7 | 25ns | +2119.0% | +307.5% |
| 1 | 8 | 25ns | +2062.2% | +285.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.680 | HIGH- (thermal bounce) |
| dep_graph_dense | 0.145 | ok |
| dep_graph_hybrid | -0.083 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 72.9ns | 25.7ns | 284.0% | HIGH |
| dep_graph_dense | 9.3ns | 556.3ns | 1.7% |  |
| dep_graph_hybrid | 458.1ns | 101.8ns | 449.8% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 22.6-29.8 ns)
     22.6 |#############
     23.0 |###########
     23.4 |###################
     23.7 |
     24.1 |##############################
     24.4 |######################
     24.8 |###########
     25.2 |########################################
     25.5 |####################
     25.9 |###################
     26.2 |
     26.6 |#################
     27.0 |######################
     27.3 |#########
     27.7 |###########
     28.0 |#######
     28.4 |#####
     28.8 |
     29.1 |###
     29.5 |#
  (16 below, 3 above range)

dep_graph_dense (n=160, range 537.6-582.0 ns)
    537.6 |#########
    539.8 |##############
    542.0 |#########################
    544.3 |################
    546.5 |##################
    548.7 |########################################
    550.9 |##############
    553.1 |################################
    555.4 |###################################
    557.6 |################
    559.8 |#######
    562.0 |##############################
    564.2 |##############
    566.5 |#######
    568.7 |####
    570.9 |#######
    573.1 |#######
    575.3 |#######
    577.6 |####
    579.8 |
  (15 below, 12 above range)

dep_graph_hybrid (n=160, range 98.0-106.1 ns)
     98.0 |########
     98.4 |
     98.8 |#############
     99.2 |#############################
     99.6 |##################################
    100.0 |########################
    100.4 |##################
    100.8 |##################
    101.2 |
    101.6 |################################
    102.0 |################
    102.5 |##########################
    102.9 |########################################
    103.3 |##################
    103.7 |##########
    104.1 |################
    104.5 |#####
    104.9 |#############
    105.3 |#####
    105.7 |################
  (17 below, 13 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=20.0% (high variance, measurements may be unstable)
- **dep_graph_csr**: bridge=290.6% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=447.2% of algo (FFI overhead may distort results)
