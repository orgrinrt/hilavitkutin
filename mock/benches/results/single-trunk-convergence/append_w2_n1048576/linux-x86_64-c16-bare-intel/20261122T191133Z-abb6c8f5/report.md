# append, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_nway** at 435468.0 ns median (-19.0% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.88x (fastest 435468.0 ns, slowest 820636.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112781387 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 536690ns | 539010ns | 518875ns | 534411ns | 549017ns | base |
| head_tail_dyn | 549345ns | 543527ns | 501659ns | 535401ns | 594104ns | +2.36% |
| kernel_nway | 435129ns | 436831ns | 400668ns | 429235ns | 461202ns | -18.92% |
| kernel_seq | 549299ns | 531815ns | 501521ns | 526590ns | 607252ns | +2.35% |
| nway | 683122ns | 678344ns | 664101ns | 676612ns | 702398ns | +27.28% |
| sequential | 823391ns | 821182ns | 805398ns | 818433ns | 839825ns | +53.42% |
| std_parallel | 480725ns | 468994ns | 456973ns | 468543ns | 510873ns | -10.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 535423ns | 517457ns | 547648ns | base | 1.958 |
| head_tail_dyn | 548018ns | 500456ns | 592649ns | +2.35% | 1.913 |
| kernel_nway | 433702ns | 399344ns | 459695ns | -19.00% | 2.418 |
| kernel_seq | 548260ns | 500631ns | 606051ns | +2.40% | 1.913 |
| nway | 682041ns | 663018ns | 701334ns | +27.38% | 1.537 |
| sequential | 822921ns | 805022ns | 839320ns | +53.70% | 1.274 |
| std_parallel | 479428ns | 455846ns | 509490ns | -10.46% | 2.187 |

## Performance model

- Peak throughput: **2.626 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.949 | 74.2% |
| head_tail_dyn | 1.933 | 73.6% |
| kernel_nway | 2.408 | 91.7% |
| kernel_seq | 1.976 | 75.3% |
| nway | 1.548 | 59.0% |
| sequential | 1.278 | 48.7% |
| std_parallel | 2.241 | 85.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 536690ns | 536690ns | base |
| head_tail_dyn | 549345ns | 549345ns | +2.36% |
| kernel_nway | 435129ns | 435129ns | -18.92% |
| kernel_seq | 549299ns | 549299ns | +2.35% |
| nway | 683122ns | 683122ns | +27.28% |
| sequential | 823391ns | 823391ns | +53.42% |
| std_parallel | 480725ns | 480725ns | -10.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 537893ns | base | --- | [526583, 541778] | --- | --- | --- | --- |
| head_tail_dyn | 542386ns | no significant difference | [-10663, +18876]ns | [523470, 551234] | no | 0.5768 | 0.4807 | 0 |
| kernel_nway | 435468ns | -98131.9ns (-18.2%) | [-115559, -82591]ns | [417372, 450642] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 530638ns | no significant difference | [-23127, +46504]ns | [513632, 590075] | no | 1.0000 | 1.0000 | 0 |
| nway | 677370ns | +138682.6ns (+25.8%) | [+131979, +152798]ns | [670081, 690363] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 820637ns | +289670.0ns (+53.9%) | [+270720, +302216]ns | [805723, 835908] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 467897ns | -67460.2ns (-12.5%) | [-76440, -47674]ns | [462536, 483869] | YES | 0.0002 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 539444ns | +0.5% | -20.3% | -4.2% | +25.8% | +49.3% | -14.4% |
| 1 | 2 | 541021ns | -4.6% | -15.4% | +11.8% | +25.2% | +52.4% | -13.1% |
| 1 | 3 | 532185ns | +5.7% | -22.9% | -3.2% | +27.3% | +57.1% | -10.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.271 | moderate- |
| head_tail_dyn | -0.544 | HIGH- (thermal bounce) |
| kernel_nway | -0.551 | HIGH- (thermal bounce) |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.253 | moderate- |
| sequential | -0.013 | ok |
| std_parallel | -0.002 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4515925.1ns | 535423.2ns | 843.4% | HIGH |
| head_tail_dyn | 4532311.1ns | 548018.2ns | 827.0% | HIGH |
| kernel_nway | 4426271.9ns | 433702.2ns | 1020.6% | HIGH |
| kernel_seq | 4526428.5ns | 548259.9ns | 825.6% | HIGH |
| nway | 4647717.7ns | 682040.7ns | 681.4% | HIGH |
| sequential | 4681157.9ns | 822921.3ns | 568.8% | HIGH |
| std_parallel | 4545819.4ns | 479427.8ns | 948.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 517457.1-547647.9 ns)
  517457.1 |
  518966.6 |
  520476.2 |
  521985.7 |########################################
  523495.2 |
  525004.8 |
  526514.3 |
  528023.9 |
  529533.4 |#############
  531042.9 |
  532552.5 |#############
  534062.0 |#############
  535571.6 |
  537081.1 |##########################
  538590.7 |########################################
  540100.2 |
  541609.7 |
  543119.3 |#############
  544628.8 |
  546138.4 |
  (2 below, 4 above range)

head_tail_dyn (n=18, range 500455.9-592648.5 ns)
  500455.9 |
  505065.5 |#############
  509675.2 |
  514284.8 |
  518894.4 |########################################
  523504.1 |
  528113.7 |#############
  532723.3 |
  537333.0 |##########################
  541942.6 |########################################
  546552.2 |#############
  551161.8 |#############
  555771.5 |
  560381.1 |
  564990.7 |
  569600.4 |#############
  574210.0 |
  578819.6 |
  583429.3 |
  588038.9 |
  (2 below, 3 above range)

kernel_nway (n=18, range 399344.2-459695.3 ns)
  399344.2 |####################
  402361.7 |####################
  405379.3 |####################
  408396.8 |
  411414.4 |
  414432.0 |####################
  417449.5 |####################
  420467.1 |####################
  423484.6 |
  426502.2 |####################
  429519.7 |####################
  432537.3 |
  435554.9 |
  438572.4 |########################################
  441590.0 |
  444607.5 |
  447625.1 |########################################
  450642.6 |####################
  453660.2 |####################
  456677.8 |
  (1 below, 3 above range)

kernel_seq (n=18, range 500630.6-606050.7 ns)
  500630.6 |
  505901.6 |#############
  511172.6 |########################################
  516443.6 |##########################
  521714.6 |
  526985.6 |##########################
  532256.6 |#############
  537527.6 |
  542798.6 |
  548069.6 |
  553340.6 |#############
  558611.7 |
  563882.7 |
  569153.7 |
  574424.7 |
  579695.7 |
  584966.7 |#############
  590237.7 |#############
  595508.7 |
  600779.7 |#############
  (2 below, 3 above range)

nway (n=18, range 663018.1-701334.2 ns)
  663018.1 |
  664933.9 |
  666849.7 |########################################
  668765.5 |
  670681.3 |#############
  672597.1 |
  674512.9 |########################################
  676428.7 |
  678344.5 |##########################
  680260.3 |
  682176.1 |
  684091.9 |#############
  686007.7 |
  687923.5 |
  689839.3 |########################################
  691755.2 |
  693671.0 |#############
  695586.8 |
  697502.6 |
  699418.4 |#############
  (2 below, 1 above range)

sequential (n=18, range 805021.9-839319.8 ns)
  805021.9 |########################################
  806736.8 |
  808451.7 |
  810166.6 |
  811881.5 |
  813596.4 |
  815311.3 |
  817026.2 |
  818741.1 |####################
  820456.0 |####################
  822170.8 |
  823885.7 |
  825600.6 |
  827315.5 |##########
  829030.4 |
  830745.3 |
  832460.2 |
  834175.1 |##########
  835890.0 |##############################
  837604.9 |
  (2 below, 3 above range)

std_parallel (n=18, range 455846.4-509489.8 ns)
  455846.4 |
  458528.5 |#############
  461210.7 |########################################
  463892.9 |##########################
  466575.1 |##########################
  469257.2 |
  471939.4 |#############
  474621.6 |
  477303.8 |#############
  479985.9 |#############
  482668.1 |
  485350.3 |#############
  488032.5 |#############
  490714.6 |#############
  493396.8 |
  496079.0 |
  498761.2 |
  501443.3 |
  504125.5 |
  506807.7 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=839.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=833.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1015.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=849.2% of algo (FFI overhead may distort results)
- **nway**: bridge=686.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=570.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=968.5% of algo (FFI overhead may distort results)
