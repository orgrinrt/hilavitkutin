# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: rcm_half** at 340327.0 ns median (-3.9% vs baseline)
- Spread: 1.05x (fastest 340327.0 ns, slowest 357023.0 ns)

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
| rcm_adj | 365483ns | 360723ns | 329965ns | 352228ns | 403124ns | base |
| rcm_half | 353011ns | 346054ns | 334225ns | 345402ns | 373818ns | -3.41% |
| rcm_rev | 362229ns | 362565ns | 335919ns | 356878ns | 383411ns | -0.89% |
| rcm_scr | 358569ns | 356427ns | 334178ns | 351524ns | 381331ns | -1.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 360108ns | 324933ns | 398352ns | base | 0.001 |
| rcm_half | 347442ns | 329089ns | 367745ns | -3.52% | 0.001 |
| rcm_rev | 356625ns | 331068ns | 377640ns | -0.97% | 0.001 |
| rcm_scr | 353460ns | 329503ns | 376220ns | -1.85% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 91.7% |
| rcm_half | 0.001 | 95.5% |
| rcm_rev | 0.001 | 91.0% |
| rcm_scr | 0.001 | 92.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 365483ns | 365483ns | base |
| rcm_half | 353011ns | 353011ns | -3.41% |
| rcm_rev | 362229ns | 362229ns | -0.89% |
| rcm_scr | 358569ns | 358569ns | -1.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 354181ns | base | --- | [335692, 364921] | --- | --- | --- | --- |
| rcm_half | 340327ns | no significant difference | [-18727, +4754]ns | [333581, 362048] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 357023ns | no significant difference | [-9888, +7873]ns | [343723, 362958] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 350788ns | no significant difference | [-7458, +10852]ns | [340071, 363135] | no | 0.7137 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 353288ns | -4.5% | -0.2% | +0.6% |
| 1 | 2 | 336250ns | +6.5% | +5.0% | +3.1% |
| 1 | 3 | 431679ns | -19.0% | -17.2% | -18.5% |
| 2 | 1 | 361596ns | -5.7% | -1.4% | +3.2% |
| 2 | 2 | 351192ns | -3.4% | +1.6% | -1.9% |
| 2 | 3 | 337350ns | +0.7% | +11.0% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.209 | moderate- |
| rcm_half | 0.011 | ok |
| rcm_rev | 0.050 | ok |
| rcm_scr | -0.332 | moderate- |

**Consistency summary:**

- **rcm_half**: won 4/6, lost 2/6
- **rcm_rev**: won 3/6, lost 3/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 11309121.5ns | 360108.3ns | 3140.5% | HIGH |
| rcm_half | 11195514.6ns | 347441.9ns | 3222.3% | HIGH |
| rcm_rev | 11217728.5ns | 356625.0ns | 3145.5% | HIGH |
| rcm_scr | 11192800.2ns | 353460.2ns | 3166.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 324933.3-398352.1 ns)
  324933.3 |########################################
  328604.3 |
  332275.2 |#############
  335946.1 |##########################
  339617.1 |
  343288.0 |
  346959.0 |
  350629.9 |##########################
  354300.8 |#############
  357971.8 |##########################
  361642.7 |#############
  365313.6 |#############
  368984.6 |
  372655.5 |#############
  376326.5 |#############
  379997.4 |
  383668.3 |
  387339.3 |
  391010.2 |
  394681.1 |
  (1 below, 2 above range)

rcm_half (n=18, range 329088.9-367745.1 ns)
  329088.9 |########################################
  331021.7 |
  332954.5 |##########
  334887.3 |
  336820.1 |##########
  338753.0 |####################
  340685.8 |##########
  342618.6 |
  344551.4 |
  346484.2 |
  348417.0 |##########
  350349.8 |
  352282.6 |
  354215.5 |
  356148.3 |
  358081.1 |##########
  360013.9 |##########
  361946.7 |##########
  363879.5 |##########
  365812.3 |
  (1 below, 3 above range)

rcm_rev (n=18, range 331068.1-377639.6 ns)
  331068.1 |##########
  333396.6 |####################
  335725.2 |
  338053.8 |
  340382.4 |
  342710.9 |
  345039.5 |
  347368.1 |
  349696.7 |
  352025.2 |####################
  354353.8 |##########
  356682.4 |########################################
  359011.0 |
  361339.6 |##########
  363668.1 |
  365996.7 |####################
  368325.3 |
  370653.9 |
  372982.4 |##########
  375311.0 |
  (2 below, 2 above range)

rcm_scr (n=18, range 329502.8-376220.1 ns)
  329502.8 |####################
  331838.7 |####################
  334174.5 |####################
  336510.4 |
  338846.3 |
  341182.1 |
  343518.0 |####################
  345853.9 |########################################
  348189.7 |####################
  350525.6 |########################################
  352861.5 |
  355197.3 |########################################
  357533.2 |
  359869.1 |
  362204.9 |
  364540.8 |
  366876.7 |####################
  369212.5 |
  371548.4 |####################
  373884.3 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3161.5% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3289.7% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=3136.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3190.4% of algo (FFI overhead may distort results)
