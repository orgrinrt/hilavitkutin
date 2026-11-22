# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_exch is fastest but the noisiest (CV 13.6%)

cs_exch wins on median (205.06 us) yet has the highest variance (CV 13.6%), while cs_plain is the steadiest (CV 0.9%, 205.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (cs_exch, cs_plain) are a dead heat (<1%)

cs_exch (205.06 us) and cs_plain (205.91 us) differ by 0.42%, inside the noise, even though the wider field spreads 42.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {cs_exch, cs_plain} vs {cs_snap_twin, cs_snap} (41% apart)

The field splits into a fast tier {cs_exch, cs_plain} and a slow tier {cs_snap_twin, cs_snap} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader cs_exch vs stability leader cs_plain (+0% speed for 14.6x steadier)

cs_exch is fastest (205.06 us, CV 13.6%); cs_plain gives up 0.4% median for 14.6x lower variance (CV 0.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: cs_exch** at 205056.2 ns median (-0.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.42x (fastest 205056.2 ns, slowest 291800.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 225595ns | 209483ns | 205018ns | 208562ns | 261431ns | +7.11% |
| cs_plain | 210625ns | 210406ns | 206922ns | 210076ns | 213299ns | base |
| cs_snap | 307460ns | 296069ns | 286682ns | 293737ns | 338433ns | +45.98% |
| cs_snap_twin | 300366ns | 296004ns | 294056ns | 295606ns | 310660ns | +42.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 221254ns | 200575ns | 256987ns | +7.58% | 0.005 |
| cs_plain | 205670ns | 202878ns | 207497ns | base | 0.005 |
| cs_snap | 303112ns | 281978ns | 333797ns | +47.38% | 0.003 |
| cs_snap_twin | 295897ns | 289568ns | 306126ns | +43.87% | 0.003 |

## Performance model

- Peak throughput: **0.005 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.005 | 97.8% |
| cs_plain | 0.005 | 97.4% |
| cs_snap | 0.004 | 68.7% |
| cs_snap_twin | 0.004 | 68.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 225595ns | 225595ns | +7.11% |
| cs_plain | 210625ns | 210625ns | base |
| cs_snap | 307460ns | 307460ns | +45.98% |
| cs_snap_twin | 300366ns | 300366ns | +42.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 205915ns | base | --- | [204744, 206469] | --- | --- | --- | --- |
| cs_exch | 205056ns | no significant difference | [-2608, +27421]ns | [202573, 235910] | no | 0.4807 | 0.4807 | 0 |
| cs_snap | 291800ns | +85720.9ns (+41.6%) | [+82090, +92450]ns | [285998, 298156] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 291352ns | +85118.7ns (+41.3%) | [+84123, +87658]ns | [290742, 292423] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 206375ns | +0.2% | +51.4% | +40.7% |
| 1 | 2 | 204696ns | -1.4% | +52.6% | +42.3% |
| 1 | 3 | 205800ns | -1.8% | +36.8% | +42.9% |
| 2 | 1 | 205596ns | +20.2% | +42.2% | +41.1% |
| 2 | 2 | 206621ns | -0.6% | +42.1% | +41.1% |
| 2 | 3 | 203725ns | +26.1% | +40.5% | +43.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.273 | moderate- |
| cs_plain | -0.470 | moderate- |
| cs_snap | 0.141 | ok |
| cs_snap_twin | -0.370 | moderate- |

**Consistency summary:**

- **cs_exch**: won 3/6, lost 3/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 5604244.4ns | 221254.2ns | 2532.9% | HIGH |
| cs_plain | 5574505.3ns | 205670.1ns | 2710.4% | HIGH |
| cs_snap | 5593394.7ns | 303111.6ns | 1845.3% | HIGH |
| cs_snap_twin | 5577198.8ns | 295897.2ns | 1884.8% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 200575.0-256986.8 ns)
  200575.0 |########################################
  203395.6 |########################################
  206216.2 |####################
  209036.8 |
  211857.4 |
  214678.0 |
  217498.5 |
  220319.1 |
  223139.7 |
  225960.3 |
  228780.9 |##########
  231601.5 |
  234422.1 |
  237242.7 |
  240063.3 |##########
  242883.9 |
  245704.5 |##########
  248525.0 |
  251345.6 |
  254166.2 |##########
  (2 below, 2 above range)

cs_plain (n=18, range 202877.8-207497.2 ns)
  202877.8 |
  203108.7 |
  203339.7 |#############
  203570.7 |#############
  203801.7 |#############
  204032.6 |
  204263.6 |
  204494.6 |#############
  204725.5 |#############
  204956.5 |
  205187.5 |
  205418.5 |##########################
  205649.4 |#############
  205880.4 |########################################
  206111.4 |
  206342.4 |##########################
  206573.3 |#############
  206804.3 |
  207035.3 |#############
  207266.2 |
  (1 below, 2 above range)

cs_snap (n=18, range 281977.8-333797.2 ns)
  281977.8 |#############
  284568.8 |########################################
  287159.7 |#############
  289750.7 |##########################
  292341.7 |########################################
  294932.7 |#############
  297523.6 |
  300114.6 |#############
  302705.6 |
  305296.5 |
  307887.5 |
  310478.5 |##########################
  313069.5 |
  315660.4 |
  318251.4 |
  320842.4 |
  323433.3 |
  326024.3 |
  328615.3 |
  331206.3 |
  (2 below, 2 above range)

cs_snap_twin (n=18, range 289568.0-306125.7 ns)
  289568.0 |##############################
  290395.9 |########################################
  291223.8 |##############################
  292051.7 |##############################
  292879.6 |####################
  293707.5 |##########
  294535.3 |
  295363.2 |
  296191.1 |
  297019.0 |
  297846.9 |
  298674.8 |
  299502.6 |
  300330.5 |
  301158.4 |
  301986.3 |
  302814.2 |
  303642.1 |
  304469.9 |
  305297.8 |
  (1 below, 1 above range)

```

## Diagnostics

- **cs_exch**: bridge=2719.5% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=2706.3% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1910.2% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1913.1% of algo (FFI overhead may distort results)
