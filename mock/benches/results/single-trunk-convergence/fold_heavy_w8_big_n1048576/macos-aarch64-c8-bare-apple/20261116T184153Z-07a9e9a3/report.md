# fold_heavy, frames of tens of milliseconds, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 46% (significant)

kernel_nway is -200.79 us (46%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.3x slower than the field

sequential (770.47 us) is 3.3x the fastest (236.38 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 9.7%)

kernel_nway wins on median (236.38 us) yet has the highest variance (CV 9.7%), while head_tail is the steadiest (CV 0.6%, 435.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.3x the fastest

Fastest kernel_nway (236.38 us) to slowest sequential (770.47 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 236375.0 ns median (-45.7% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.26x (fastest 236375.0 ns, slowest 770468.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 436286ns | 435760ns | 432573ns | 435556ns | 439240ns | base |
| head_tail_dyn | 447658ns | 443271ns | 437865ns | 441861ns | 461250ns | +2.61% |
| kernel_nway | 244944ns | 237271ns | 221052ns | 234274ns | 272896ns | -43.86% |
| kernel_seq | 766359ns | 761625ns | 759260ns | 761024ns | 777911ns | +75.66% |
| nway | 248439ns | 243068ns | 229052ns | 240236ns | 270438ns | -43.06% |
| sequential | 773089ns | 770849ns | 762104ns | 768771ns | 785057ns | +77.20% |
| std_parallel | 269155ns | 266073ns | 255750ns | 265281ns | 281667ns | -38.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 435438ns | 431896ns | 438417ns | base | 2.408 |
| head_tail_dyn | 446858ns | 437333ns | 459943ns | +2.62% | 2.347 |
| kernel_nway | 244135ns | 220365ns | 272052ns | -43.93% | 4.295 |
| kernel_seq | 765585ns | 758990ns | 776427ns | +75.82% | 1.370 |
| nway | 247606ns | 228396ns | 269339ns | -43.14% | 4.235 |
| sequential | 772071ns | 761781ns | 782656ns | +77.31% | 1.358 |
| std_parallel | 268627ns | 255302ns | 281167ns | -38.31% | 3.903 |

## Performance model

- Peak throughput: **4.758 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.410 | 50.7% |
| head_tail_dyn | 2.368 | 49.8% |
| kernel_nway | 4.436 | 93.2% |
| kernel_seq | 1.378 | 29.0% |
| nway | 4.327 | 90.9% |
| sequential | 1.361 | 28.6% |
| std_parallel | 3.949 | 83.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 436286ns | 436286ns | base |
| head_tail_dyn | 447658ns | 447658ns | +2.61% |
| kernel_nway | 244944ns | 244944ns | -43.86% |
| kernel_seq | 766359ns | 766359ns | +75.66% |
| nway | 248439ns | 248439ns | -43.06% |
| sequential | 773089ns | 773089ns | +77.20% |
| std_parallel | 269155ns | 269155ns | -38.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 435073ns | base | --- | [432823, 438417] | --- | --- | --- | --- |
| head_tail_dyn | 442802ns | +9119.8ns (+2.1%) | [+661, +24479]ns | [437828, 459943] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| kernel_nway | 236375ns | -200791.6ns (-46.2%) | [-209703, -163412]ns | [223979, 272052] | YES | 0.0375 | 0.0313 | 0 |
| kernel_seq | 761036ns | +325968.8ns (+74.9%) | [+322260, +342214]ns | [759292, 776427] | YES | 0.0375 | 0.0313 | 0 |
| nway | 242307ns | -190979.1ns (-43.9%) | [-204688, -167828]ns | [231172, 269339] | YES | 0.0375 | 0.0313 | 0 |
| sequential | 770469ns | +333979.2ns (+76.8%) | [+328016, +347906]ns | [763088, 782656] | YES | 0.0375 | 0.0313 | 0 |
| std_parallel | 265505ns | -169958.4ns (-39.1%) | [-177031, -153443]ns | [259208, 281167] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 436240ns | +1.1% | -47.8% | +74.4% | -40.4% | +76.6% | -40.6% |
| 1 | 2 | 435464ns | +5.6% | -37.5% | +77.4% | -46.1% | +76.4% | -39.0% |
| 1 | 3 | 434609ns | +1.1% | -46.4% | +75.8% | -42.8% | +79.0% | -35.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.001 | ok |
| head_tail_dyn | -0.664 | HIGH- (thermal bounce) |
| kernel_nway | -0.659 | HIGH- (thermal bounce) |
| kernel_seq | -0.618 | HIGH- (thermal bounce) |
| nway | -0.533 | HIGH- (thermal bounce) |
| sequential | -0.323 | moderate- |
| std_parallel | -0.038 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 434724.0ns | 435437.5ns | 99.8% | HIGH |
| head_tail_dyn | 450618.1ns | 446857.6ns | 100.8% | HIGH |
| kernel_nway | 245776.1ns | 244135.4ns | 100.7% | HIGH |
| kernel_seq | 762786.5ns | 765585.1ns | 99.6% | HIGH |
| nway | 248937.5ns | 247605.9ns | 100.5% | HIGH |
| sequential | 770793.4ns | 772071.2ns | 99.8% | HIGH |
| std_parallel | 268901.0ns | 268626.7ns | 100.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 431895.8-438416.7 ns)
  431895.8 |########################################
  432221.8 |
  432547.9 |
  432873.9 |
  433200.0 |
  433526.0 |########################################
  433852.1 |
  434178.1 |
  434504.1 |########################################
  434830.2 |
  435156.2 |########################################
  435482.3 |
  435808.3 |
  436134.4 |########################################
  436460.4 |
  436786.4 |
  437112.5 |
  437438.5 |
  437764.6 |
  438090.6 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 437333.3-459942.7 ns)
  437333.3 |########################################
  438463.8 |
  439594.2 |
  440724.7 |####################
  441855.2 |
  442985.6 |
  444116.1 |####################
  445246.6 |
  446377.1 |
  447507.5 |
  448638.0 |
  449768.5 |
  450898.9 |
  452029.4 |
  453159.9 |
  454290.3 |
  455420.8 |
  456551.3 |
  457681.8 |
  458812.2 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 220364.6-272052.0 ns)
  220364.6 |########################################
  222949.0 |
  225533.3 |########################################
  228117.7 |
  230702.1 |
  233286.5 |########################################
  235870.8 |########################################
  238455.2 |
  241039.6 |
  243624.0 |
  246208.3 |
  248792.7 |
  251377.1 |
  253961.4 |########################################
  256545.8 |
  259130.2 |
  261714.6 |
  264298.9 |
  266883.3 |
  269467.7 |
  (0 below, 1 above range)

kernel_seq (n=6, range 758989.6-776427.1 ns)
  758989.6 |########################################
  759861.5 |####################
  760733.3 |####################
  761605.2 |
  762477.1 |
  763349.0 |
  764220.8 |
  765092.7 |
  765964.6 |
  766836.5 |
  767708.3 |
  768580.2 |####################
  769452.1 |
  770323.9 |
  771195.8 |
  772067.7 |
  772939.6 |
  773811.4 |
  774683.3 |
  775555.2 |
  (0 below, 1 above range)

nway (n=6, range 228395.8-269338.5 ns)
  228395.8 |########################################
  230442.9 |
  232490.1 |########################################
  234537.2 |########################################
  236584.3 |
  238631.5 |
  240678.6 |
  242725.8 |
  244772.9 |
  246820.0 |
  248867.2 |########################################
  250914.3 |
  252961.4 |
  255008.6 |
  257055.7 |
  259102.9 |
  261150.0 |
  263197.1 |
  265244.3 |
  267291.4 |########################################
  (0 below, 1 above range)

sequential (n=6, range 761781.2-782656.2 ns)
  761781.2 |####################
  762825.0 |
  763868.7 |####################
  764912.5 |
  765956.2 |
  767000.0 |
  768043.7 |
  769087.5 |
  770131.2 |########################################
  771175.0 |
  772218.7 |
  773262.5 |####################
  774306.2 |
  775350.0 |
  776393.7 |
  777437.5 |
  778481.2 |
  779525.0 |
  780568.7 |
  781612.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 255302.1-281166.7 ns)
  255302.1 |####################
  256595.3 |
  257888.6 |
  259181.8 |
  260475.0 |
  261768.2 |
  263061.5 |########################################
  264354.7 |
  265647.9 |
  266941.2 |####################
  268234.4 |
  269527.6 |
  270820.9 |
  272114.1 |
  273407.3 |####################
  274700.5 |
  275993.8 |
  277287.0 |
  278580.2 |
  279873.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=98.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=100.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.2% of algo (FFI overhead may distort results)
