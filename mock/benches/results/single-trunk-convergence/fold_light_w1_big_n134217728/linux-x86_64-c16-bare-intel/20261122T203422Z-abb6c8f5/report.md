# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential shows alternating (throttle bounce) (autocorr -0.65)

sequential's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sequential** at 27846841.4 ns median (-9.8% vs baseline)
- Spread: 1.12x (fastest 27846841.4 ns, slowest 31242798.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112823136 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 30807172ns | 30864174ns | 30587132ns | 30800533ns | 30927152ns | base |
| head_tail_dyn | 30044583ns | 30885713ns | 27368444ns | 30316041ns | 30975464ns | -2.48% |
| kernel_nway | 30287828ns | 30763720ns | 28558508ns | 30351212ns | 31057412ns | -1.69% |
| kernel_seq | 30796097ns | 31248742ns | 26287389ns | 30864027ns | 32948555ns | -0.04% |
| nway | 29420441ns | 30573491ns | 26397387ns | 29231902ns | 31214775ns | -4.50% |
| sequential | 28686864ns | 27848445ns | 26312762ns | 27837393ns | 31148123ns | -6.88% |
| std_parallel | 30145602ns | 29778594ns | 29119793ns | 29674675ns | 31364895ns | -2.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 30802926ns | 30585995ns | 30918956ns | base | 4.357 |
| head_tail_dyn | 30039456ns | 27365917ns | 30967056ns | -2.48% | 4.468 |
| kernel_nway | 30280903ns | 28552142ns | 31046201ns | -1.69% | 4.432 |
| kernel_seq | 30790373ns | 26286045ns | 32943002ns | -0.04% | 4.359 |
| nway | 29419008ns | 26396132ns | 31213572ns | -4.49% | 4.562 |
| sequential | 28685305ns | 26311330ns | 31146604ns | -6.87% | 4.679 |
| std_parallel | 30141054ns | 29113557ns | 31362935ns | -2.15% | 4.453 |

## Performance model

- Peak throughput: **5.106 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 4.349 | 85.2% |
| head_tail_dyn | 4.346 | 85.1% |
| kernel_nway | 4.364 | 85.5% |
| kernel_seq | 4.296 | 84.1% |
| nway | 4.390 | 86.0% |
| sequential | 4.820 | 94.4% |
| std_parallel | 4.508 | 88.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 30807172ns | 30807172ns | base |
| head_tail_dyn | 30044583ns | 30044583ns | -2.48% |
| kernel_nway | 30287828ns | 30287828ns | -1.69% |
| kernel_seq | 30796097ns | 30796097ns | -0.04% |
| nway | 29420441ns | 29420441ns | -4.50% |
| sequential | 28686864ns | 28686864ns | -6.88% |
| std_parallel | 30145602ns | 30145602ns | -2.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 30862845ns | base | --- | [30626975, 30918956] | --- | --- | --- | --- |
| head_tail_dyn | 30884388ns | no significant difference | [-2360050, +53332]ns | [28266925, 30967056] | no | 1.0000 | 0.6875 | 0 |
| kernel_nway | 30758961ns | no significant difference | [-1698913, +134794]ns | [29037546, 31046201] | no | 1.0000 | 0.6875 | 0 |
| kernel_seq | 31242798ns | no significant difference | [-2441656, +2066929]ns | [28185319, 32943002] | no | 1.0000 | 1.0000 | 0 |
| nway | 30571692ns | no significant difference | [-4428391, +331921]ns | [26471760, 31213572] | no | 1.0000 | 1.0000 | 0 |
| sequential | 27846841ns | no significant difference | [-3687219, +264954]ns | [27062468, 31146604] | no | 1.0000 | 0.6875 | 0 |
| std_parallel | 29773813ns | no significant difference | [-1581192, +626476]ns | [29286415, 31362935] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 30900151ns | +0.1% | -1.8% | +2.6% | -14.3% | -10.0% | -1.3% |
| 1 | 2 | 30881651ns | +0.1% | -0.0% | +5.1% | +1.1% | +0.9% | -4.2% |
| 1 | 3 | 30626975ns | -7.7% | -3.3% | -8.0% | -0.2% | -11.5% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.133 | ok |
| head_tail_dyn | -0.162 | ok |
| kernel_nway | -0.446 | moderate- |
| kernel_seq | -0.270 | moderate- |
| nway | -0.243 | moderate- |
| sequential | -0.648 | HIGH- (thermal bounce) |
| std_parallel | -0.647 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 30863866.8ns | 30802925.5ns | 100.2% | HIGH |
| head_tail_dyn | 30210320.2ns | 30039456.4ns | 100.6% | HIGH |
| kernel_nway | 30167788.5ns | 30280902.6ns | 99.6% | HIGH |
| kernel_seq | 30703666.4ns | 30790373.2ns | 99.7% | HIGH |
| nway | 29507581.0ns | 29419007.9ns | 100.3% | HIGH |
| sequential | 28815266.1ns | 28685304.6ns | 100.5% | HIGH |
| std_parallel | 30996583.6ns | 30141054.2ns | 102.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 30585995.1-30918956.0 ns)
  30585995.1 |########################################
  30602643.1 |
  30619291.2 |
  30635939.2 |
  30652587.3 |########################################
  30669235.3 |
  30685883.4 |
  30702531.4 |
  30719179.5 |
  30735827.5 |
  30752475.6 |
  30769123.6 |
  30785771.7 |
  30802419.7 |
  30819067.8 |
  30835715.8 |########################################
  30852363.9 |
  30869011.9 |
  30885660.0 |########################################
  30902308.0 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 27365917.2-30967055.7 ns)
  27365917.2 |#############
  27545974.1 |
  27726031.1 |
  27906088.0 |
  28086144.9 |
  28266201.8 |
  28446258.8 |
  28626315.7 |
  28806372.6 |
  28986429.5 |
  29166486.4 |#############
  29346543.4 |
  29526600.3 |
  29706657.2 |
  29886714.1 |
  30066771.1 |
  30246828.0 |
  30426884.9 |
  30606941.8 |
  30786998.8 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 28552142.1-31046200.9 ns)
  28552142.1 |########################################
  28676845.0 |
  28801548.0 |
  28926250.9 |
  29050953.9 |
  29175656.8 |
  29300359.7 |
  29425062.7 |########################################
  29549765.6 |
  29674468.6 |
  29799171.5 |
  29923874.4 |
  30048577.4 |
  30173280.3 |
  30297983.3 |
  30422686.2 |
  30547389.1 |
  30672092.1 |########################################
  30796795.0 |########################################
  30921498.0 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 26286045.0-32943002.4 ns)
  26286045.0 |####################
  26618892.9 |
  26951740.7 |
  27284588.6 |
  27617436.5 |
  27950284.3 |
  28283132.2 |
  28615980.1 |
  28948827.9 |
  29281675.8 |
  29614523.7 |
  29947371.5 |####################
  30280219.4 |
  30613067.3 |####################
  30945915.1 |
  31278763.0 |
  31611610.9 |########################################
  31944458.7 |
  32277306.6 |
  32610154.5 |
  (0 below, 1 above range)

nway (n=6, range 26396131.5-31213571.7 ns)
  26396131.5 |########################################
  26637003.5 |
  26877875.5 |
  27118747.5 |
  27359619.5 |
  27600491.6 |
  27841363.6 |
  28082235.6 |
  28323107.6 |
  28563979.6 |
  28804851.6 |
  29045723.6 |
  29286595.6 |
  29527467.6 |
  29768339.6 |
  30009211.7 |
  30250083.7 |####################
  30490955.7 |####################
  30731827.7 |
  30972699.7 |####################
  (0 below, 1 above range)

sequential (n=6, range 26311330.5-31146604.1 ns)
  26311330.5 |#############
  26553094.2 |
  26794857.9 |
  27036621.5 |
  27278385.2 |
  27520148.9 |
  27761912.6 |########################################
  28003676.2 |
  28245439.9 |
  28487203.6 |
  28728967.3 |
  28970731.0 |
  29212494.6 |
  29454258.3 |
  29696022.0 |
  29937785.7 |
  30179549.3 |
  30421313.0 |
  30663076.7 |
  30904840.4 |#############
  (0 below, 1 above range)

std_parallel (n=6, range 29113557.2-31362934.6 ns)
  29113557.2 |####################
  29226026.1 |
  29338494.9 |
  29450963.8 |########################################
  29563432.7 |
  29675901.5 |
  29788370.4 |
  29900839.3 |
  30013308.1 |####################
  30125777.0 |
  30238245.9 |
  30350714.7 |
  30463183.6 |
  30575652.5 |
  30688121.3 |
  30800590.2 |
  30913059.1 |
  31025527.9 |
  31137996.8 |####################
  31250465.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=98.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.1% of algo (FFI overhead may distort results)
- **nway**: bridge=101.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=104.6% of algo (FFI overhead may distort results)
