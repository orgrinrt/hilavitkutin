# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 20.2x slower than the field

dep_graph_dense (7.49 us) is 20.2x the fastest (372 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_hybrid is fastest but the noisiest (CV 6.0%)

dep_graph_hybrid wins on median (372 ns) yet has the highest variance (CV 6.0%), while dep_graph_dense is the steadiest (CV 3.3%, 7.49 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 20.2x the fastest

Fastest dep_graph_hybrid (372 ns) to slowest dep_graph_dense (7.49 us): 20.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_hybrid's edge over baseline is significant but tiny (-5 ns, 1.31%)

dep_graph_hybrid differs from baseline dep_graph_csr by -5 ns (1.31%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dep_graph_hybrid** at 371.7 ns median (-2.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 20.16x (fastest 371.7 ns, slowest 7492.9 ns)

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
| dep_graph_csr | 2626ns | 2598ns | 2446ns | 2626ns | 2806ns | base |
| dep_graph_dense | 9588ns | 9723ns | 9143ns | 9607ns | 9977ns | +265.12% |
| dep_graph_hybrid | 2527ns | 2475ns | 2435ns | 2495ns | 2717ns | -3.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 387ns | 358ns | 417ns | base | 0.166 |
| dep_graph_dense | 7394ns | 7056ns | 7697ns | +1812.74% | 0.009 |
| dep_graph_hybrid | 381ns | 367ns | 413ns | -1.45% | 0.168 |

## Performance model

- Peak throughput: **0.179 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.167 | 93.8% |
| dep_graph_dense | 0.009 | 4.8% |
| dep_graph_hybrid | 0.172 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2626ns | 2626ns | base |
| dep_graph_dense | 9588ns | 9588ns | +265.12% |
| dep_graph_hybrid | 2527ns | 2527ns | -3.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 382ns | base | --- | [381, 383] | --- | --- | --- | --- |
| dep_graph_dense | 7493ns | +7086.8ns (+1854.7%) | [+7004, +7100]ns | [7381, 7498] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 372ns | -5.0ns (-1.3%) | [-12, -0]ns | [370, 376] | YES | 0.0376 | 0.0376 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 369ns | +1931.6% | +8.6% |
| 1 | 2 | 411ns | +1725.1% | -10.0% |
| 1 | 3 | 383ns | +1763.9% | +0.3% |
| 1 | 4 | 367ns | +1944.6% | +0.4% |
| 1 | 5 | 384ns | +1818.6% | +2.8% |
| 1 | 6 | 385ns | +1848.3% | -3.6% |
| 1 | 7 | 410ns | +1729.4% | -10.2% |
| 1 | 8 | 383ns | +1858.0% | -3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.273 | moderate- |
| dep_graph_dense | -0.271 | moderate- |
| dep_graph_hybrid | -0.357 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 69.2ns | 386.6ns | 17.9% | HIGH |
| dep_graph_dense | 8.5ns | 7393.7ns | 0.1% |  |
| dep_graph_hybrid | 70.7ns | 380.9ns | 18.6% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 358.4-416.9 ns)
    358.4 |#######
    361.4 |#
    364.3 |######
    367.2 |#############
    370.1 |##########
    373.1 |####
    376.0 |
    378.9 |#############################
    381.8 |##########################
    384.8 |##
    387.7 |#
    390.6 |###
    393.5 |
    396.5 |#
    399.4 |
    402.3 |#
    405.2 |
    408.2 |################
    411.1 |########################################
    414.0 |##
  (19 below, 2 above range)

dep_graph_dense (n=160, range 7056.1-7696.9 ns)
   7056.1 |##########
   7088.1 |#
   7120.2 |####
   7152.2 |###
   7184.3 |#####
   7216.3 |###############
   7248.3 |########
   7280.4 |##
   7312.4 |#
   7344.5 |####
   7376.5 |###
   7408.6 |
   7440.6 |##
   7472.6 |########################################
   7504.7 |####################################
   7536.7 |#####
   7568.8 |####
   7600.8 |####
   7632.8 |
   7664.9 |
  (18 below, 10 above range)

dep_graph_hybrid (n=160, range 366.6-413.3 ns)
    366.6 |########################################
    368.9 |#####################
    371.2 |######
    373.6 |###
    375.9 |#####
    378.3 |######
    380.6 |##########
    382.9 |###
    385.3 |#
    387.6 |####
    390.0 |####
    392.3 |###
    394.6 |########
    397.0 |#
    399.3 |
    401.6 |
    404.0 |
    406.3 |
    408.7 |
    411.0 |
  (11 below, 14 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=17.8% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=18.9% of algo (FFI overhead may distort results)
