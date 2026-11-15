# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_rev, rcm_adj) are a dead heat (<1%)

rcm_rev (4.56 ms) and rcm_adj (4.59 ms) differ by 0.52%, inside the noise, even though the wider field spreads 8.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader rcm_rev vs stability leader rcm_half (+9% speed for 3.2x steadier)

rcm_rev is fastest (4.56 ms, CV 4.9%); rcm_half gives up 8.6% median for 3.2x lower variance (CV 1.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_rev** at 4563931.8 ns median (-0.5% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.09x (fastest 4563931.8 ns, slowest 4955446.9 ns)

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
| Counter frequency | 2112746224 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 4672551ns | 4591808ns | 4525806ns | 4595252ns | 4861872ns | base |
| rcm_half | 4943603ns | 4959732ns | 4853079ns | 4919644ns | 5024803ns | +5.80% |
| rcm_rev | 4621332ns | 4567994ns | 4338280ns | 4529862ns | 4900064ns | -1.10% |
| rcm_scr | 4923271ns | 4893133ns | 4796308ns | 4875009ns | 5059146ns | +5.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 4668516ns | 4521788ns | 4857787ns | base | 0.000 |
| rcm_half | 4939557ns | 4849029ns | 5020787ns | +5.81% | 0.000 |
| rcm_rev | 4617418ns | 4334584ns | 4896014ns | -1.09% | 0.000 |
| rcm_scr | 4919369ns | 4792382ns | 5055164ns | +5.37% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 94.5% |
| rcm_half | 0.000 | 87.5% |
| rcm_rev | 0.000 | 95.0% |
| rcm_scr | 0.000 | 88.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 4672551ns | 4672551ns | base |
| rcm_half | 4943603ns | 4943603ns | +5.80% |
| rcm_rev | 4621332ns | 4621332ns | -1.10% |
| rcm_scr | 4923271ns | 4923271ns | +5.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 4587772ns | base | --- | [4569790, 4750150] | --- | --- | --- | --- |
| rcm_half | 4955447ns | +296466.3ns (+6.5%) | [+239203, +337677]ns | [4866239, 4988263] | YES | 0.0004 | 0.0001 | 0 |
| rcm_rev | 4563932ns | no significant difference | [-208133, +135431]ns | [4447825, 4780502] | no | 0.2379 | 0.2379 | 0 |
| rcm_scr | 4889372ns | +248676.9ns (+5.4%) | [+119080, +416961]ns | [4823030, 5003888] | YES | 0.0113 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 4834681ns | +3.2% | -5.4% | +1.7% |
| 1 | 2 | 4654037ns | +6.9% | +6.3% | +3.5% |
| 1 | 3 | 4647511ns | +8.3% | -2.2% | +3.7% |
| 2 | 1 | 4579262ns | +5.9% | -3.1% | +6.7% |
| 2 | 2 | 4551409ns | +6.9% | +6.5% | +9.2% |
| 2 | 3 | 4524373ns | +8.1% | -4.1% | +13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.270 | moderate+ |
| rcm_half | 0.226 | moderate+ |
| rcm_rev | -0.468 | moderate- |
| rcm_scr | 0.353 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 4/6, lost 2/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 30750935.4ns | 4668515.7ns | 658.7% | HIGH |
| rcm_half | 30749180.3ns | 4939556.7ns | 622.5% | HIGH |
| rcm_rev | 30887119.8ns | 4617417.6ns | 668.9% | HIGH |
| rcm_scr | 30976304.9ns | 4919369.2ns | 629.7% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 4521788.3-4857786.8 ns)
  4521788.3 |##########
  4538588.2 |####################
  4555388.1 |##########
  4572188.1 |########################################
  4588988.0 |##########
  4605787.9 |
  4622587.8 |
  4639387.8 |####################
  4656187.7 |
  4672987.6 |##########
  4689787.5 |
  4706587.5 |
  4723387.4 |
  4740187.3 |
  4756987.2 |
  4773787.2 |
  4790587.1 |
  4807387.0 |
  4824186.9 |####################
  4840986.8 |
  (1 below, 3 above range)

rcm_half (n=18, range 4849028.6-5020787.3 ns)
  4849028.6 |####################
  4857616.5 |########################################
  4866204.5 |####################
  4874792.4 |
  4883380.3 |####################
  4891968.3 |####################
  4900556.2 |
  4909144.2 |
  4917732.1 |
  4926320.0 |
  4934908.0 |
  4943495.9 |####################
  4952083.8 |
  4960671.8 |####################
  4969259.7 |####################
  4977847.7 |########################################
  4986435.6 |########################################
  4995023.5 |
  5003611.5 |
  5012199.4 |####################
  (2 below, 2 above range)

rcm_rev (n=18, range 4334584.3-4896013.9 ns)
  4334584.3 |####################
  4362655.7 |####################
  4390727.2 |####################
  4418798.7 |####################
  4446870.2 |####################
  4474941.7 |####################
  4503013.1 |
  4531084.6 |########################################
  4559156.1 |########################################
  4587227.6 |
  4615299.1 |
  4643370.5 |
  4671442.0 |####################
  4699513.5 |
  4727585.0 |
  4755656.5 |####################
  4783727.9 |
  4811799.4 |
  4839870.9 |########################################
  4867942.4 |
  (1 below, 3 above range)

rcm_scr (n=18, range 4792382.2-5055163.9 ns)
  4792382.2 |#############
  4805521.3 |##########################
  4818660.4 |########################################
  4831799.5 |
  4844938.6 |
  4858077.6 |
  4871216.7 |#############
  4884355.8 |##########################
  4897494.9 |
  4910634.0 |#############
  4923773.1 |
  4936912.1 |
  4950051.2 |
  4963190.3 |#############
  4976329.4 |
  4989468.5 |#############
  5002607.6 |##########################
  5015746.6 |
  5028885.7 |
  5042024.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=668.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=618.8% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=672.8% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=628.1% of algo (FFI overhead may distort results)
