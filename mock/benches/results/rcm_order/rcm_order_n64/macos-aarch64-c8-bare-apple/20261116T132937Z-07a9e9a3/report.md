# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rcm_adj shows alternating (throttle bounce) (autocorr -0.66)

rcm_adj's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.7% of the fastest

All 4 variants sit between 80.33 us and 80.90 us - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: rcm_rev** at 80329.1 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 80329.1 ns, slowest 80900.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 84548ns | 84517ns | 83464ns | 84413ns | 85294ns | base |
| rcm_half | 84820ns | 84808ns | 84094ns | 84638ns | 85456ns | +0.32% |
| rcm_rev | 84537ns | 84650ns | 83549ns | 84307ns | 85376ns | -0.01% |
| rcm_scr | 84581ns | 84596ns | 83735ns | 84456ns | 85190ns | +0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 80533ns | 80197ns | 80956ns | base | 0.001 |
| rcm_half | 80870ns | 80283ns | 81406ns | +0.42% | 0.001 |
| rcm_rev | 80499ns | 80068ns | 81043ns | -0.04% | 0.001 |
| rcm_scr | 80573ns | 80285ns | 80916ns | +0.05% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 99.6% |
| rcm_half | 0.001 | 99.0% |
| rcm_rev | 0.001 | 99.7% |
| rcm_scr | 0.001 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 84548ns | 84548ns | base |
| rcm_half | 84820ns | 84820ns | +0.32% |
| rcm_rev | 84537ns | 84537ns | -0.01% |
| rcm_scr | 84581ns | 84581ns | +0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 80388ns | base | --- | [80315, 80569] | --- | --- | --- | --- |
| rcm_half | 80900ns | +350.0ns (+0.4%) | [+98, +725]ns | [80469, 81238] | YES | 0.0226 | 0.0075 | 0 |
| rcm_rev | 80329ns | no significant difference | [-298, +85]ns | [80202, 80473] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 80462ns | no significant difference | [-165, +227]ns | [80402, 80552] | no | 0.7210 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 80429ns | +0.3% | -0.1% | +0.0% |
| 1 | 2 | 80254ns | +1.5% | +0.0% | +0.3% |
| 1 | 3 | 80558ns | -0.1% | -0.4% | -0.1% |
| 2 | 1 | 80367ns | -0.2% | -0.2% | +0.2% |
| 2 | 2 | 80579ns | +0.6% | +1.1% | -0.3% |
| 2 | 3 | 80325ns | +0.2% | +0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.660 | HIGH- (thermal bounce) |
| rcm_half | -0.371 | moderate- |
| rcm_rev | -0.185 | ok |
| rcm_scr | 0.036 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 2/6, lost 1/6
- **rcm_scr**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 2786895.8ns | 80532.9ns | 3460.6% | HIGH |
| rcm_half | 2789161.3ns | 80870.4ns | 3448.9% | HIGH |
| rcm_rev | 2786319.2ns | 80498.6ns | 3461.3% | HIGH |
| rcm_scr | 2785573.4ns | 80572.9ns | 3457.2% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 80197.2-80956.2 ns)
  80197.2 |
  80235.2 |#############
  80273.1 |##########################
  80311.1 |########################################
  80349.0 |#############
  80387.0 |#############
  80424.9 |#############
  80462.9 |#############
  80500.8 |
  80538.8 |#############
  80576.7 |#############
  80614.7 |#############
  80652.6 |#############
  80690.6 |
  80728.5 |
  80766.5 |
  80804.4 |
  80842.4 |
  80880.3 |
  80918.3 |
  (2 below, 2 above range)

rcm_half (n=18, range 80283.3-81405.6 ns)
  80283.3 |
  80339.4 |
  80395.6 |########################################
  80451.7 |##########################
  80507.8 |
  80563.9 |
  80620.0 |#############
  80676.1 |
  80732.2 |
  80788.3 |
  80844.4 |#############
  80900.6 |#############
  80956.7 |#############
  81012.8 |
  81068.9 |#############
  81125.0 |
  81181.1 |#############
  81237.2 |#############
  81293.3 |#############
  81349.5 |
  (2 below, 3 above range)

rcm_rev (n=18, range 80068.0-81043.1 ns)
  80068.0 |##########
  80116.8 |
  80165.5 |########################################
  80214.3 |####################
  80263.0 |##########
  80311.8 |
  80360.5 |##############################
  80409.3 |##########
  80458.0 |
  80506.8 |##########
  80555.5 |
  80604.3 |
  80653.0 |
  80701.8 |
  80750.5 |
  80799.3 |
  80848.0 |##########
  80896.8 |
  80945.5 |
  80994.3 |
  (1 below, 3 above range)

rcm_scr (n=18, range 80284.7-80916.0 ns)
  80284.7 |#############
  80316.3 |
  80347.9 |##########################
  80379.4 |########################################
  80411.0 |
  80442.5 |########################################
  80474.1 |##########################
  80505.7 |
  80537.2 |##########################
  80568.8 |#############
  80600.4 |
  80631.9 |
  80663.5 |
  80695.0 |#############
  80726.6 |
  80758.2 |
  80789.7 |
  80821.3 |
  80852.8 |
  80884.4 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3464.9% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3444.7% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=3468.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3461.9% of algo (FFI overhead may distort results)
