# append, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (kernel_nway, head_tail) are a dead heat (<1%)

kernel_nway (583.99 us) and head_tail (586.75 us) differ by 0.47%, inside the noise, even though the wider field spreads 68.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.66)

std_parallel's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_nway, head_tail, head_tail_dyn, kernel_seq} vs {sequential, std_parallel, nway} (31% apart)

The field splits into a fast tier {kernel_nway, head_tail, head_tail_dyn, kernel_seq} and a slow tier {sequential, std_parallel, nway} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 1.7x its best-20%

nway's best 20% of batches run at 953.24 us but its worst 20% at 1.63 ms (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 583994.2 ns median (-0.5% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.69x (fastest 583994.2 ns, slowest 985122.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 590550ns | 588581ns | 527311ns | 570574ns | 652134ns | base |
| head_tail_dyn | 598801ns | 617103ns | 502643ns | 579811ns | 675366ns | +1.40% |
| kernel_nway | 585890ns | 585120ns | 511826ns | 573382ns | 641686ns | -0.79% |
| kernel_seq | 626766ns | 626199ns | 531967ns | 603591ns | 708928ns | +6.13% |
| nway | 1193127ns | 986362ns | 954242ns | 979597ns | 1632865ns | +102.04% |
| sequential | 850627ns | 815921ns | 808375ns | 813741ns | 927081ns | +44.04% |
| std_parallel | 909049ns | 892537ns | 853987ns | 887263ns | 969261ns | +53.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 589144ns | 526002ns | 650642ns | base | 1.780 |
| head_tail_dyn | 597512ns | 501411ns | 673971ns | +1.42% | 1.755 |
| kernel_nway | 584420ns | 510469ns | 639997ns | -0.80% | 1.794 |
| kernel_seq | 625230ns | 530662ns | 707183ns | +6.13% | 1.677 |
| nway | 1191971ns | 953236ns | 1631682ns | +102.32% | 0.880 |
| sequential | 850149ns | 807980ns | 926572ns | +44.30% | 1.233 |
| std_parallel | 907582ns | 852937ns | 967602ns | +54.05% | 1.155 |

## Performance model

- Peak throughput: **2.091 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.787 | 85.5% |
| head_tail_dyn | 1.703 | 81.4% |
| kernel_nway | 1.796 | 85.9% |
| kernel_seq | 1.679 | 80.3% |
| nway | 1.064 | 50.9% |
| sequential | 1.286 | 61.5% |
| std_parallel | 1.177 | 56.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 590550ns | 590550ns | base |
| head_tail_dyn | 598801ns | 598801ns | +1.40% |
| kernel_nway | 585890ns | 585890ns | -0.79% |
| kernel_seq | 626766ns | 626766ns | +6.13% |
| nway | 1193127ns | 1193127ns | +102.04% |
| sequential | 850627ns | 850627ns | +44.04% |
| std_parallel | 909049ns | 909049ns | +53.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 586747ns | base | --- | [546982, 607908] | --- | --- | --- | --- |
| head_tail_dyn | 615796ns | no significant difference | [-53721, +34377]ns | [537642, 641189] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 583994ns | no significant difference | [-49265, +41988]ns | [550499, 618602] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 624524ns | no significant difference | [-53252, +82620]ns | [574890, 652698] | no | 0.9774 | 0.8145 | 0 |
| nway | 985122ns | +416126.7ns (+70.9%) | [+358795, +740569]ns | [961378, 1367704] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 815493ns | +235339.2ns (+40.1%) | [+217333, +275195]ns | [810352, 818543] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 891207ns | +321267.7ns (+54.8%) | [+290210, +342028]ns | [877679, 906149] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 596744ns | +7.6% | -9.5% | -0.9% | +61.1% | +35.8% | +50.5% |
| 1 | 2 | 530789ns | +2.0% | +13.3% | +23.7% | +92.7% | +54.2% | +65.7% |
| 1 | 3 | 657360ns | -13.4% | -4.4% | -9.3% | +52.0% | +23.8% | +36.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.514 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.339 | moderate- |
| kernel_nway | -0.032 | ok |
| kernel_seq | -0.664 | HIGH- (thermal bounce) |
| nway | -0.418 | moderate- |
| sequential | -0.536 | HIGH- (thermal bounce) |
| std_parallel | -0.665 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4572391.6ns | 589143.6ns | 776.1% | HIGH |
| head_tail_dyn | 4620207.5ns | 597511.7ns | 773.2% | HIGH |
| kernel_nway | 4594535.4ns | 584420.1ns | 786.2% | HIGH |
| kernel_seq | 4767379.0ns | 625229.7ns | 762.5% | HIGH |
| nway | 5124878.3ns | 1191971.0ns | 429.9% | HIGH |
| sequential | 4717640.4ns | 850148.5ns | 554.9% | HIGH |
| std_parallel | 5010319.7ns | 907582.3ns | 552.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 526002.2-650642.1 ns)
  526002.2 |########################################
  532234.2 |####################
  538466.2 |####################
  544698.2 |
  550930.2 |########################################
  557162.2 |
  563394.2 |
  569626.2 |####################
  575858.2 |
  582090.2 |####################
  588322.1 |########################################
  594554.1 |####################
  600786.1 |####################
  607018.1 |####################
  613250.1 |
  619482.1 |
  625714.1 |
  631946.1 |
  638178.1 |
  644410.1 |####################
  (1 below, 3 above range)

head_tail_dyn (n=18, range 501410.8-673971.1 ns)
  501410.8 |
  510038.8 |#############
  518666.9 |
  527294.9 |##########################
  535922.9 |##########################
  544550.9 |
  553178.9 |
  561806.9 |#############
  570434.9 |
  579062.9 |
  587691.0 |
  596319.0 |
  604947.0 |#############
  613575.0 |
  622203.0 |##########################
  630831.0 |##########################
  639459.0 |########################################
  648087.0 |#############
  656715.1 |
  665343.1 |
  (2 below, 1 above range)

kernel_nway (n=18, range 510469.5-639996.7 ns)
  510469.5 |
  516945.8 |
  523422.2 |
  529898.6 |
  536374.9 |#############
  542851.3 |##########################
  549327.6 |
  555804.0 |########################################
  562280.4 |
  568756.7 |#############
  575233.1 |
  581709.5 |
  588185.8 |
  594662.2 |#############
  601138.5 |#############
  607614.9 |#############
  614091.3 |##########################
  620567.6 |
  627044.0 |
  633520.4 |#############
  (2 below, 3 above range)

kernel_seq (n=18, range 530661.7-707182.8 ns)
  530661.7 |
  539487.8 |####################
  548313.8 |########################################
  557139.9 |
  565965.9 |
  574792.0 |
  583618.0 |
  592444.1 |########################################
  601270.1 |####################
  610096.2 |
  618922.2 |########################################
  627748.3 |########################################
  636574.3 |
  645400.4 |########################################
  654226.4 |####################
  663052.5 |
  671878.5 |####################
  680704.6 |####################
  689530.6 |
  698356.7 |
  (2 below, 1 above range)

nway (n=18, range 953235.9-1631682.0 ns)
  953235.9 |########################################
  987158.2 |########
  1021080.5 |
  1055002.8 |####
  1088925.1 |
  1122847.4 |
  1156769.7 |
  1190692.0 |
  1224614.3 |
  1258536.7 |
  1292459.0 |
  1326381.3 |
  1360303.6 |
  1394225.9 |
  1428148.2 |
  1462070.5 |
  1495992.8 |
  1529915.1 |
  1563837.4 |
  1597759.7 |
  (1 below, 5 above range)

sequential (n=18, range 807980.5-926572.3 ns)
  807980.5 |########################################
  813910.1 |##################################
  819839.7 |#####
  825769.3 |
  831698.9 |
  837628.5 |
  843558.0 |
  849487.6 |
  855417.2 |
  861346.8 |
  867276.4 |
  873206.0 |
  879135.6 |
  885065.2 |
  890994.8 |
  896924.4 |#####
  902854.0 |
  908783.5 |
  914713.1 |
  920642.7 |
  (2 below, 1 above range)

std_parallel (n=18, range 852937.3-967602.4 ns)
  852937.3 |
  858670.6 |#############
  864403.8 |
  870137.1 |#############
  875870.3 |########################################
  881603.6 |#############
  887336.9 |#############
  893070.1 |########################################
  898803.4 |#############
  904536.6 |##########################
  910269.9 |##########################
  916003.1 |
  921736.4 |
  927469.6 |
  933202.9 |
  938936.1 |
  944669.4 |
  950402.6 |
  956135.9 |
  961869.1 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=780.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=743.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=785.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=741.7% of algo (FFI overhead may distort results)
- **nway**: CV=28.9% (high variance, measurements may be unstable)
- **nway**: bridge=502.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=572.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=548.3% of algo (FFI overhead may distort results)
