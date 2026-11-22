# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 16% faster than the next best (std_parallel)

sequential (26.47 ms) leads std_parallel (30.65 ms) by 16%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.63)

kernel_nway's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sequential** at 26469371.9 ns median (-14.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.17x (fastest 26469371.9 ns, slowest 31090194.0 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 30147478ns | 30943823ns | 26145760ns | 30486446ns | 31639885ns | base |
| head_tail_dyn | 30562097ns | 30800216ns | 29375423ns | 30412653ns | 31379600ns | +1.38% |
| kernel_nway | 30742070ns | 30835413ns | 26709683ns | 30824508ns | 32634607ns | +1.97% |
| kernel_seq | 30727576ns | 30861668ns | 29797149ns | 30849986ns | 31009174ns | +1.92% |
| nway | 30819096ns | 31091502ns | 26205464ns | 31025425ns | 32816417ns | +2.23% |
| sequential | 27018716ns | 26470518ns | 26194694ns | 26415162ns | 28336057ns | -10.38% |
| std_parallel | 30567441ns | 30658470ns | 29383770ns | 30280510ns | 31589675ns | +1.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 30141352ns | 26144400ns | 31632088ns | base | 4.453 |
| head_tail_dyn | 30556096ns | 29373675ns | 31372218ns | +1.38% | 4.393 |
| kernel_nway | 30736184ns | 26703282ns | 32625235ns | +1.97% | 4.367 |
| kernel_seq | 30721527ns | 29794004ns | 31003362ns | +1.92% | 4.369 |
| nway | 30817826ns | 26204678ns | 32814798ns | +2.24% | 4.355 |
| sequential | 27017503ns | 26193619ns | 28334768ns | -10.36% | 4.968 |
| std_parallel | 30564397ns | 29382000ns | 31587709ns | +1.40% | 4.391 |

## Performance model

- Peak throughput: **5.134 Gops/s** (head_tail; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 4.338 | 84.5% |
| head_tail_dyn | 4.359 | 84.9% |
| kernel_nway | 4.353 | 84.8% |
| kernel_seq | 4.350 | 84.7% |
| nway | 4.317 | 84.1% |
| sequential | 5.071 | 98.8% |
| std_parallel | 4.378 | 85.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 30147478ns | 30147478ns | base |
| head_tail_dyn | 30562097ns | 30562097ns | +1.38% |
| kernel_nway | 30742070ns | 30742070ns | +1.97% |
| kernel_seq | 30727576ns | 30727576ns | +1.92% |
| nway | 30819096ns | 30819096ns | +2.23% |
| sequential | 27018716ns | 27018716ns | -10.38% |
| std_parallel | 30567441ns | 30567441ns | +1.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 30937414ns | base | --- | [27854553, 31632088] | --- | --- | --- | --- |
| head_tail_dyn | 30793189ns | no significant difference | [-1607998, +2743231]ns | [29502882, 31372218] | no | 1.0000 | 0.6875 | 0 |
| kernel_nway | 30830904ns | no significant difference | [-2485985, +3616983]ns | [28752412, 32625235] | no | 1.0000 | 0.6875 | 0 |
| kernel_seq | 30853027ns | no significant difference | [-803268, +2477846]ns | [30308191, 31003362] | no | 1.0000 | 1.0000 | 0 |
| nway | 31090194ns | no significant difference | [-2711932, +3897317]ns | [28548485, 32814798] | no | 1.0000 | 0.6875 | 0 |
| sequential | 26469372ns | -2971159.9ns (-9.6%) | [-5359769, -1040617]ns | [26248369, 28334768] | YES (adj: no) | 1.0000 | 0.2188 | 0 |
| std_parallel | 30653862ns | no significant difference | [-655223, +1859722]ns | [29451620, 31587709] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 27854553ns | +8.7% | +10.6% | +8.9% | +12.8% | -5.1% | +5.7% |
| 1 | 2 | 31632088ns | -5.1% | -3.6% | -2.5% | -4.9% | -16.9% | -0.6% |
| 1 | 3 | 30937414ns | +1.4% | -0.1% | +0.2% | +0.0% | -8.4% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.275 | moderate- |
| head_tail_dyn | -0.275 | moderate- |
| kernel_nway | -0.630 | HIGH- (thermal bounce) |
| kernel_seq | -0.047 | ok |
| nway | -0.583 | HIGH- (thermal bounce) |
| sequential | -0.212 | moderate- |
| std_parallel | -0.380 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 1/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 30218723.8ns | 30141351.6ns | 100.3% | HIGH |
| head_tail_dyn | 31510668.1ns | 30556096.0ns | 103.1% | HIGH |
| kernel_nway | 30585079.7ns | 30736183.6ns | 99.5% | HIGH |
| kernel_seq | 30909804.0ns | 30721526.8ns | 100.6% | HIGH |
| nway | 29596246.2ns | 30817825.6ns | 96.0% | HIGH |
| sequential | 27197019.9ns | 27017502.9ns | 100.7% | HIGH |
| std_parallel | 31039188.5ns | 30564396.8ns | 101.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 26144399.8-31632088.3 ns)
  26144399.8 |####################
  26418784.2 |
  26693168.7 |
  26967553.1 |
  27241937.5 |
  27516321.9 |
  27790706.4 |
  28065090.8 |
  28339475.2 |
  28613859.6 |
  28888244.1 |
  29162628.5 |
  29437012.9 |####################
  29711397.3 |
  29985781.8 |
  30260166.2 |
  30534550.6 |
  30808935.0 |########################################
  31083319.4 |
  31357703.9 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 29373674.6-31372217.5 ns)
  29373674.6 |########################################
  29473601.7 |
  29573528.9 |########################################
  29673456.0 |
  29773383.2 |
  29873310.3 |
  29973237.5 |
  30073164.6 |
  30173091.8 |
  30273018.9 |
  30372946.1 |
  30472873.2 |
  30572800.4 |
  30672727.5 |########################################
  30772654.7 |
  30872581.8 |########################################
  30972509.0 |
  31072436.1 |########################################
  31172363.3 |
  31272290.4 |
  (0 below, 1 above range)

kernel_nway (n=6, range 26703282.0-32625235.2 ns)
  26703282.0 |#############
  26999379.7 |
  27295477.3 |
  27591575.0 |
  27887672.6 |
  28183770.3 |
  28479868.0 |
  28775965.6 |
  29072063.3 |
  29368161.0 |
  29664258.6 |
  29960356.3 |
  30256453.9 |
  30552551.6 |########################################
  30848649.3 |#############
  31144746.9 |
  31440844.6 |
  31736942.3 |
  32033039.9 |
  32329137.6 |
  (0 below, 1 above range)

kernel_seq (n=6, range 29794003.8-31003362.0 ns)
  29794003.8 |#############
  29854471.7 |
  29914939.6 |
  29975407.5 |
  30035875.4 |
  30096343.4 |
  30156811.3 |
  30217279.2 |
  30277747.1 |
  30338215.0 |
  30398682.9 |
  30459150.8 |
  30519618.8 |
  30580086.7 |
  30640554.6 |
  30701022.5 |
  30761490.4 |
  30821958.3 |########################################
  30882426.2 |#############
  30942894.1 |
  (0 below, 1 above range)

nway (n=6, range 26204677.8-32814797.6 ns)
  26204677.8 |####################
  26535183.8 |
  26865689.8 |
  27196195.8 |
  27526701.8 |
  27857207.7 |
  28187713.7 |
  28518219.7 |
  28848725.7 |
  29179231.7 |
  29509737.7 |
  29840243.7 |
  30170749.6 |
  30501255.6 |
  30831761.6 |########################################
  31162267.6 |####################
  31492773.6 |####################
  31823279.6 |
  32153785.6 |
  32484291.6 |
  (0 below, 1 above range)

sequential (n=6, range 26193619.2-28334767.9 ns)
  26193619.2 |####################
  26300676.6 |########################################
  26407734.1 |
  26514791.5 |####################
  26621848.9 |
  26728906.4 |
  26835963.8 |
  26943021.3 |
  27050078.7 |
  27157136.1 |
  27264193.6 |
  27371251.0 |
  27478308.4 |
  27585365.9 |
  27692423.3 |
  27799480.8 |
  27906538.2 |
  28013595.6 |
  28120653.1 |
  28227710.5 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 29381999.7-31587709.1 ns)
  29381999.7 |########################################
  29492285.2 |########################################
  29602570.6 |
  29712856.1 |
  29823141.6 |
  29933427.1 |
  30043712.5 |########################################
  30153998.0 |
  30264283.5 |
  30374568.9 |
  30484854.4 |
  30595139.9 |
  30705425.3 |
  30815710.8 |
  30925996.3 |
  31036281.8 |
  31146567.2 |########################################
  31256852.7 |
  31367138.2 |########################################
  31477423.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=99.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=102.1% of algo (FFI overhead may distort results)
