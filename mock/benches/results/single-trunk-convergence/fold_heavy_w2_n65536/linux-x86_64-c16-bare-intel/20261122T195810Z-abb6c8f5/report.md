# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.2x slower than the field

std_parallel (267.89 us) is 2.2x the fastest (119.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail is fastest but the noisiest (CV 87.6%)

head_tail wins on median (119.36 us) yet has the highest variance (CV 87.6%), while sequential is the steadiest (CV 0.1%, 195.44 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (119.36 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, head_tail_dyn} vs {kernel_seq, sequential, nway, kernel_nway, std_parallel} (56% apart)

The field splits into a fast tier {head_tail, head_tail_dyn} and a slow tier {kernel_seq, sequential, nway, kernel_nway, std_parallel} with a 56% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### kernel_nway is inconsistent: worst-20% is 3.2x its best-20%

kernel_nway's best 20% of batches run at 97.67 us but its worst 20% at 307.97 us (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 119362.4 ns median
- 5 variants significantly slower than baseline
- Spread: 2.24x (fastest 119362.4 ns, slowest 267886.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112746474 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 165379ns | 119610ns | 114977ns | 119796ns | 258954ns | base |
| head_tail_dyn | 146150ns | 124618ns | 114898ns | 125933ns | 192101ns | -11.63% |
| kernel_nway | 224775ns | 250494ns | 97963ns | 210199ns | 310045ns | +35.92% |
| kernel_seq | 205482ns | 194093ns | 191954ns | 193772ns | 229811ns | +24.25% |
| nway | 224888ns | 197525ns | 143539ns | 187576ns | 321530ns | +35.98% |
| sequential | 195659ns | 195619ns | 195382ns | 195570ns | 195932ns | +18.31% |
| std_parallel | 267862ns | 268130ns | 240018ns | 260624ns | 292643ns | +61.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 164519ns | 114788ns | 256864ns | base | 0.398 |
| head_tail_dyn | 145577ns | 114678ns | 190937ns | -11.51% | 0.450 |
| kernel_nway | 223825ns | 97674ns | 307974ns | +36.05% | 0.293 |
| kernel_seq | 204668ns | 191695ns | 228228ns | +24.40% | 0.320 |
| nway | 224447ns | 143342ns | 320802ns | +36.43% | 0.292 |
| sequential | 195486ns | 195212ns | 195755ns | +18.82% | 0.335 |
| std_parallel | 267378ns | 239809ns | 291696ns | +62.52% | 0.245 |

## Performance model

- Peak throughput: **0.671 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.549 | 81.8% |
| head_tail_dyn | 0.528 | 78.6% |
| kernel_nway | 0.262 | 39.0% |
| kernel_seq | 0.339 | 50.5% |
| nway | 0.332 | 49.5% |
| sequential | 0.335 | 50.0% |
| std_parallel | 0.245 | 36.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 165379ns | 165379ns | base |
| head_tail_dyn | 146150ns | 146150ns | -11.63% |
| kernel_nway | 224775ns | 224775ns | +35.92% |
| kernel_seq | 205482ns | 205482ns | +24.25% |
| nway | 224888ns | 224888ns | +35.98% |
| sequential | 195659ns | 195659ns | +18.31% |
| std_parallel | 267862ns | 267862ns | +61.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 119362ns | base | --- | [115377, 157378] | --- | --- | --- | --- |
| head_tail_dyn | 124197ns | no significant difference | [-13872, +8369]ns | [121143, 146701] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 250174ns | +70615.9ns (+59.2%) | [+2320, +128115]ns | [153151, 255594] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| kernel_seq | 193506ns | +76097.8ns (+63.8%) | [+55315, +77634]ns | [192874, 196350] | YES | 0.0026 | 0.0013 | 0 |
| nway | 197153ns | +37299.5ns (+31.2%) | [+30038, +100685]ns | [153279, 266363] | YES | 0.0113 | 0.0075 | 0 |
| sequential | 195438ns | +75868.3ns (+63.6%) | [+38708, +80049]ns | [195342, 195616] | YES | 0.0026 | 0.0013 | 0 |
| std_parallel | 267887ns | +134939.1ns (+113.0%) | [+124773, +144922]ns | [252562, 271170] | YES | 0.0009 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 115485ns | +5.7% | +110.8% | +67.4% | +130.3% | +69.2% | +118.7% |
| 1 | 2 | 129900ns | -6.7% | +96.5% | +50.2% | +69.8% | +50.5% | +107.5% |
| 1 | 3 | 115542ns | +26.1% | -14.5% | +67.0% | +31.9% | +69.3% | +132.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.185 | ok |
| kernel_nway | -0.207 | moderate- |
| kernel_seq | -0.646 | HIGH- (thermal bounce) |
| nway | -0.009 | ok |
| sequential | -0.011 | ok |
| std_parallel | -0.191 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 170015.8ns | 164519.1ns | 103.3% | HIGH |
| head_tail_dyn | 147892.0ns | 145576.8ns | 101.6% | HIGH |
| kernel_nway | 221910.1ns | 223825.3ns | 99.1% | HIGH |
| kernel_seq | 206496.4ns | 204668.5ns | 100.9% | HIGH |
| nway | 225943.1ns | 224446.6ns | 100.7% | HIGH |
| sequential | 195606.9ns | 195485.8ns | 100.1% | HIGH |
| std_parallel | 266242.5ns | 267378.5ns | 99.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 114788.2-256864.1 ns)
  114788.2 |########################################
  121892.0 |####
  128995.8 |
  136099.6 |########
  143203.4 |
  150307.2 |
  157410.9 |
  164514.7 |
  171618.5 |####
  178722.3 |####
  185826.1 |
  192929.9 |####
  200033.7 |
  207137.5 |
  214241.3 |
  221345.1 |
  228448.9 |
  235552.7 |
  242656.5 |
  249760.3 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 114678.3-190937.2 ns)
  114678.3 |
  118491.3 |########################################
  122304.2 |
  126117.2 |#####
  129930.1 |#####
  133743.0 |
  137556.0 |
  141368.9 |
  145181.9 |###############
  148994.8 |
  152807.8 |
  156620.7 |
  160433.7 |
  164246.6 |#####
  168059.5 |
  171872.5 |
  175685.4 |
  179498.4 |
  183311.3 |
  187124.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 97674.4-307973.7 ns)
  97674.4 |####################
  108189.3 |
  118704.3 |
  129219.3 |
  139734.2 |##########
  150249.2 |
  160764.2 |##########
  171279.1 |
  181794.1 |
  192309.1 |
  202824.0 |
  213339.0 |
  223854.0 |##########
  234368.9 |##########
  244883.9 |########################################
  255398.9 |##############################
  265913.8 |
  276428.8 |
  286943.8 |
  297458.7 |
  (2 below, 3 above range)

kernel_seq (n=18, range 191695.2-228228.2 ns)
  191695.2 |########################################
  193521.8 |######################
  195348.5 |
  197175.1 |###########
  199001.8 |
  200828.4 |
  202655.1 |
  204481.7 |
  206308.4 |
  208135.0 |
  209961.7 |
  211788.3 |
  213615.0 |
  215441.6 |
  217268.3 |
  219094.9 |
  220921.6 |
  222748.2 |
  224574.9 |#####
  226401.5 |
  (2 below, 2 above range)

nway (n=18, range 143341.8-320802.4 ns)
  143341.8 |##########################
  152214.8 |########################################
  161087.9 |##########################
  169960.9 |
  178833.9 |
  187707.0 |#############
  196580.0 |#############
  205453.0 |
  214326.1 |
  223199.1 |
  232072.1 |#############
  240945.1 |
  249818.2 |
  258691.2 |##########################
  267564.2 |########################################
  276437.3 |
  285310.3 |
  294183.3 |
  303056.4 |
  311929.4 |
  (1 below, 2 above range)

sequential (n=18, range 195212.1-195755.3 ns)
  195212.1 |##########################
  195239.3 |#############
  195266.5 |
  195293.6 |#############
  195320.8 |
  195347.9 |#############
  195375.1 |##########################
  195402.2 |
  195429.4 |########################################
  195456.6 |
  195483.7 |
  195510.9 |#############
  195538.0 |#############
  195565.2 |
  195592.4 |
  195619.5 |
  195646.7 |
  195673.8 |#############
  195701.0 |
  195728.1 |#############
  (1 below, 3 above range)

std_parallel (n=18, range 239809.0-291695.5 ns)
  239809.0 |
  242403.4 |
  244997.7 |#############
  247592.0 |#############
  250186.3 |#############
  252780.7 |#############
  255375.0 |#############
  257969.3 |#############
  260563.6 |
  263158.0 |
  265752.3 |##########################
  268346.6 |########################################
  270940.9 |#############
  273535.2 |#############
  276129.6 |
  278723.9 |
  281318.2 |
  283912.5 |
  286506.9 |
  289101.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=63.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=28.0% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=39.2% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **kernel_nway**: bridge=99.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: CV=39.6% (high variance, measurements may be unstable)
- **nway**: bridge=106.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.8% of algo (FFI overhead may distort results)
