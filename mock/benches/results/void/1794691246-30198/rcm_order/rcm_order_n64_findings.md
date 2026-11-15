# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.6% (fastest 82.02 us to slowest 83.35 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.33 us) is smaller than the fastest variant's own run-to-run std-dev (13.28 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader rcm_scr vs stability leader rcm_adj (+0% speed for 5.3x steadier)

rcm_scr is fastest (82.02 us, CV 16.2%); rcm_adj gives up 0.3% median for 5.3x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### rcm_rev is inconsistent: worst-20% is 1.5x its best-20%

rcm_rev's best 20% of batches run at 80.45 us but its worst 20% at 121.34 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rcm_scr** at 82022.9 ns median (-0.3% vs baseline)
- Spread: 1.02x (fastest 82022.9 ns, slowest 83347.9 ns)

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
| rcm_adj | 86029ns | 85331ns | 82867ns | 85212ns | 88834ns | base |
| rcm_half | 87590ns | 84831ns | 83347ns | 84792ns | 93908ns | +1.81% |
| rcm_rev | 98321ns | 85779ns | 82965ns | 85314ns | 125508ns | +14.29% |
| rcm_scr | 90812ns | 84929ns | 83017ns | 84552ns | 104101ns | +5.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 83140ns | 80493ns | 86018ns | base | 0.001 |
| rcm_half | 84501ns | 80749ns | 90019ns | +1.64% | 0.001 |
| rcm_rev | 95101ns | 80454ns | 121336ns | +14.39% | 0.001 |
| rcm_scr | 87663ns | 80653ns | 99922ns | +5.44% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 97.8% |
| rcm_half | 0.001 | 97.8% |
| rcm_rev | 0.001 | 96.5% |
| rcm_scr | 0.001 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 86029ns | 86029ns | base |
| rcm_half | 87590ns | 87590ns | +1.81% |
| rcm_rev | 98321ns | 98321ns | +14.29% |
| rcm_scr | 90812ns | 90812ns | +5.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 82229ns | base | --- | [81304, 84350] | --- | --- | --- | --- |
| rcm_half | 82260ns | no significant difference | [-552, +1310]ns | [81577, 83696] | no | 1.0000 | 1.0000 | 0 |
| rcm_rev | 83348ns | no significant difference | [-387, +1696]ns | [81235, 84458] | no | 1.0000 | 1.0000 | 0 |
| rcm_scr | 82023ns | no significant difference | [-367, +2594]ns | [81367, 84400] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 80625ns | +2.0% | +1.0% | +0.3% |
| 1 | 2 | 84571ns | +2.0% | -0.2% | +31.5% |
| 1 | 3 | 83750ns | -0.0% | +0.8% | +0.5% |
| 2 | 1 | 82125ns | -0.8% | -1.8% | -1.1% |
| 2 | 2 | 81050ns | +1.5% | +1.2% | +1.0% |
| 2 | 3 | 81558ns | -0.6% | +3.6% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.035 | ok |
| rcm_half | 0.089 | ok |
| rcm_rev | -0.185 | ok |
| rcm_scr | -0.203 | moderate- |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 3/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 2825096.8ns | 83139.6ns | 3398.0% | HIGH |
| rcm_half | 2830091.2ns | 84500.7ns | 3349.2% | HIGH |
| rcm_rev | 3047881.2ns | 95101.2ns | 3204.9% | HIGH |
| rcm_scr | 2847340.7ns | 87663.0ns | 3248.1% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 80493.0-86018.0 ns)
  80493.0 |########################################
  80769.3 |
  81045.5 |#############
  81321.8 |#############
  81598.0 |
  81874.3 |##########################
  82150.5 |##########################
  82426.8 |
  82703.0 |
  82979.3 |
  83255.5 |#############
  83531.8 |#############
  83808.0 |
  84084.3 |#############
  84360.5 |##########################
  84636.8 |
  84913.0 |#############
  85189.3 |
  85465.5 |
  85741.8 |
  (1 below, 2 above range)

rcm_half (n=18, range 80748.6-90018.7 ns)
  80748.6 |####################
  81212.1 |####################
  81675.6 |####################
  82139.1 |########################################
  82602.6 |
  83066.1 |##########
  83529.6 |##############################
  83993.1 |##########
  84456.6 |
  84920.2 |
  85383.7 |
  85847.2 |##########
  86310.7 |
  86774.2 |
  87237.7 |
  87701.2 |
  88164.7 |
  88628.2 |
  89091.7 |
  89555.2 |
  (1 below, 1 above range)

rcm_rev (n=18, range 80454.2-121336.1 ns)
  80454.2 |########################################
  82498.3 |########################################
  84542.4 |
  86586.5 |######
  88630.6 |######
  90674.7 |
  92718.7 |
  94762.8 |
  96806.9 |######
  98851.0 |
  100895.1 |
  102939.2 |
  104983.3 |
  107027.4 |
  109071.5 |
  111115.6 |
  113159.7 |
  115203.8 |
  117247.9 |
  119292.0 |
  (2 below, 1 above range)

rcm_scr (n=18, range 80652.8-99922.2 ns)
  80652.8 |########################################
  81616.2 |#################
  82579.7 |#####
  83543.2 |#####
  84506.7 |#####
  85470.1 |
  86433.6 |
  87397.1 |
  88360.5 |
  89324.0 |
  90287.5 |
  91251.0 |
  92214.4 |#####
  93177.9 |#####
  94141.4 |
  95104.9 |
  96068.3 |
  97031.8 |
  97995.3 |
  98958.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3414.7% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3405.6% of algo (FFI overhead may distort results)
- **rcm_rev**: CV=48.5% (high variance, measurements may be unstable)
- **rcm_rev**: bridge=3420.2% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3420.7% of algo (FFI overhead may distort results)
