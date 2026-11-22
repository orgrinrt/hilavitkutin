# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 14.2x slower than the field

dep_graph_dense (2.13 us) is 14.2x the fastest (150 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 14.2x the fastest

Fastest dep_graph_csr (150 ns) to slowest dep_graph_dense (2.13 us): 14.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 149.6 ns median
- 2 variants significantly slower than baseline
- Spread: 14.24x (fastest 149.6 ns, slowest 2130.6 ns)

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
| dep_graph_csr | 2234ns | 2221ns | 2209ns | 2225ns | 2286ns | base |
| dep_graph_dense | 4246ns | 4210ns | 4191ns | 4218ns | 4386ns | +90.08% |
| dep_graph_hybrid | 2261ns | 2261ns | 2219ns | 2253ns | 2328ns | +1.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 150ns | 146ns | 154ns | base | 0.214 |
| dep_graph_dense | 2147ns | 2113ns | 2225ns | +1332.75% | 0.015 |
| dep_graph_hybrid | 156ns | 152ns | 162ns | +3.92% | 0.205 |

## Performance model

- Peak throughput: **0.218 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.214 | 97.9% |
| dep_graph_dense | 0.015 | 6.9% |
| dep_graph_hybrid | 0.206 | 94.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2234ns | 2234ns | base |
| dep_graph_dense | 4246ns | 4246ns | +90.08% |
| dep_graph_hybrid | 2261ns | 2261ns | +1.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 150ns | base | --- | [149, 150] | --- | --- | --- | --- |
| dep_graph_dense | 2131ns | +1980.9ns (+1324.1%) | [+1978, +1985]ns | [2128, 2134] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 155ns | +5.4ns (+3.6%) | [+5, +6]ns | [154, 155] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 149ns | +1321.4% | +2.8% |
| 1 | 2 | 148ns | +1330.3% | +3.6% |
| 1 | 3 | 151ns | +1311.0% | +6.2% |
| 1 | 4 | 150ns | +1323.3% | +2.4% |
| 1 | 5 | 153ns | +1289.8% | +2.3% |
| 1 | 6 | 150ns | +1322.4% | +4.3% |
| 1 | 7 | 149ns | +1362.7% | +2.6% |
| 1 | 8 | 149ns | +1347.9% | +3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.143 | ok |
| dep_graph_dense | 0.309 | moderate+ |
| dep_graph_hybrid | -0.399 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 69.2ns | 149.9ns | 46.2% | HIGH |
| dep_graph_dense | 9.2ns | 2147.0ns | 0.4% |  |
| dep_graph_hybrid | 75.8ns | 155.7ns | 48.7% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 146.5-153.5 ns)
    146.5 |#################
    146.8 |############################
    147.2 |#####
    147.5 |######################
    147.9 |
    148.3 |#####################################
    148.6 |#####################################
    149.0 |######################
    149.3 |########################################
    149.7 |####################
    150.0 |
    150.4 |######################
    150.7 |####################
    151.1 |###############################
    151.4 |###########
    151.8 |########################################
    152.1 |
    152.5 |###########
    152.8 |########
    153.2 |#################
  (12 below, 10 above range)

dep_graph_dense (n=160, range 2112.8-2225.5 ns)
   2112.8 |##########################
   2118.4 |##########################
   2124.1 |########################################
   2129.7 |############################
   2135.3 |##########################
   2141.0 |#######
   2146.6 |####
   2152.2 |#####
   2157.9 |#####
   2163.5 |##
   2169.1 |#
   2174.8 |####
   2180.4 |##
   2186.0 |##
   2191.7 |#
   2197.3 |##
   2202.9 |##
   2208.6 |#
   2214.2 |#
   2219.8 |#
  (15 below, 11 above range)

dep_graph_hybrid (n=160, range 151.7-161.7 ns)
    151.7 |#####################################
    152.2 |####################
    152.7 |############
    153.2 |######################
    153.7 |###########################
    154.2 |########################################
    154.7 |###########################
    155.2 |######################
    155.7 |#################
    156.2 |############
    156.7 |################################
    157.2 |##########
    157.7 |###############
    158.2 |############
    158.7 |##########
    159.2 |############
    159.7 |##
    160.2 |##########
    160.7 |
    161.2 |##
  (11 below, 10 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=46.8% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=49.7% of algo (FFI overhead may distort results)
