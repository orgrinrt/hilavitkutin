# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_rev, rcm_adj) are a dead heat (<1%)

rcm_rev (124.70 us) and rcm_adj (125.81 us) differ by 0.89%, inside the noise, even though the wider field spreads 8.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader rcm_rev vs stability leader rcm_adj (+1% speed for 5.0x steadier)

rcm_rev is fastest (124.70 us, CV 4.3%); rcm_adj gives up 0.9% median for 5.0x lower variance (CV 0.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_rev** at 124697.5 ns median (-0.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 124697.5 ns, slowest 134840.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 2112757625 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 128147ns | 127610ns | 127035ns | 127624ns | 129486ns | base |
| rcm_half | 129221ns | 130658ns | 116611ns | 126464ns | 139661ns | +0.84% |
| rcm_rev | 123764ns | 126471ns | 115928ns | 123212ns | 128508ns | -3.42% |
| rcm_scr | 136667ns | 136620ns | 129885ns | 135856ns | 141273ns | +6.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 126286ns | 125123ns | 127635ns | base | 0.001 |
| rcm_half | 127358ns | 114554ns | 137821ns | +0.85% | 0.001 |
| rcm_rev | 121936ns | 114122ns | 126675ns | -3.45% | 0.001 |
| rcm_scr | 134837ns | 128075ns | 139401ns | +6.77% | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 90.7% |
| rcm_half | 0.000 | 88.6% |
| rcm_rev | 0.001 | 91.5% |
| rcm_scr | 0.000 | 84.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 128147ns | 128147ns | base |
| rcm_half | 129221ns | 129221ns | +0.84% |
| rcm_rev | 123764ns | 123764ns | -3.42% |
| rcm_scr | 136667ns | 136667ns | +6.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 125806ns | base | --- | [125356, 127374] | --- | --- | --- | --- |
| rcm_half | 128836ns | no significant difference | [-10489, +4277]ns | [116237, 131042] | no | 0.2379 | 0.2379 | 0 |
| rcm_rev | 124698ns | -1648.0ns (-1.3%) | [-10560, -668]ns | [115107, 125564] | YES | 0.0113 | 0.0075 | 0 |
| rcm_scr | 134841ns | +8623.7ns (+6.9%) | [+6370, +10130]ns | [133017, 135370] | YES | 0.0039 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 125520ns | +2.4% | -0.9% | +8.9% |
| 1 | 2 | 125377ns | +5.0% | +2.0% | +7.5% |
| 1 | 3 | 127772ns | +2.8% | -10.9% | -0.7% |
| 2 | 1 | 125902ns | +2.4% | -8.8% | +5.1% |
| 2 | 2 | 127374ns | -9.3% | -1.4% | +5.1% |
| 2 | 3 | 125190ns | -8.4% | -0.2% | +8.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.446 | moderate- |
| rcm_half | 0.488 | moderate+ |
| rcm_rev | 0.051 | ok |
| rcm_scr | 0.038 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 5/6, lost 1/6
- **rcm_scr**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 1914289.6ns | 126286.4ns | 1515.8% | HIGH |
| rcm_half | 1915415.8ns | 127357.7ns | 1504.0% | HIGH |
| rcm_rev | 1914548.1ns | 121935.8ns | 1570.1% | HIGH |
| rcm_scr | 1915975.8ns | 134837.3ns | 1421.0% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 125122.7-127634.6 ns)
  125122.7 |#############
  125248.3 |########################################
  125373.9 |#############
  125499.5 |#############
  125625.1 |#############
  125750.7 |
  125876.3 |#############
  126001.9 |
  126127.5 |
  126253.1 |#############
  126378.7 |
  126504.3 |
  126629.9 |
  126755.4 |
  126881.0 |
  127006.6 |
  127132.2 |#############
  127257.8 |##########################
  127383.4 |#############
  127509.0 |#############
  (2 below, 2 above range)

rcm_half (n=18, range 114553.9-137820.8 ns)
  114553.9 |########################################
  115717.2 |##########
  116880.6 |
  118043.9 |
  119207.3 |
  120370.6 |
  121534.0 |
  122697.3 |
  123860.6 |
  125024.0 |
  126187.3 |
  127350.7 |##########
  128514.0 |##############################
  129677.4 |##############################
  130840.7 |########################################
  132004.0 |
  133167.4 |
  134330.7 |
  135494.1 |
  136657.4 |
  (1 below, 1 above range)

rcm_rev (n=18, range 114121.9-126675.4 ns)
  114121.9 |#############
  114749.6 |########################################
  115377.2 |
  116004.9 |
  116632.6 |
  117260.3 |
  117887.9 |
  118515.6 |
  119143.3 |
  119771.0 |
  120398.6 |
  121026.3 |
  121654.0 |
  122281.7 |
  122909.3 |
  123537.0 |#############
  124164.7 |##########################
  124792.4 |########################################
  125420.0 |##########################
  126047.7 |##########################
  (2 below, 2 above range)

rcm_scr (n=18, range 128075.1-139400.8 ns)
  128075.1 |
  128641.4 |
  129207.6 |
  129773.9 |
  130340.2 |
  130906.5 |
  131472.8 |##########
  132039.1 |####################
  132605.4 |
  133171.7 |##########
  133738.0 |##########
  134304.2 |##############################
  134870.5 |########################################
  135436.8 |
  136003.1 |##########
  136569.4 |##########
  137135.7 |
  137702.0 |
  138268.3 |##########
  138834.6 |
  (2 below, 1 above range)

```

## Diagnostics

- **rcm_adj**: bridge=1520.9% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=1485.3% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=1534.7% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=1419.4% of algo (FFI overhead may distort results)
