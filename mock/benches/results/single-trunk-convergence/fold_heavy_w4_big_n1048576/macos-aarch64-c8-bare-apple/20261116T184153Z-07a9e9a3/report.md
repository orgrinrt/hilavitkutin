# fold_heavy, frames of tens of milliseconds, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 44% (significant)

kernel_nway is -193.70 us (44%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.1x slower than the field

sequential (763.88 us) is 3.1x the fastest (242.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway is fastest but the noisiest (CV 21.1%)

kernel_nway wins on median (242.90 us) yet has the highest variance (CV 21.1%), while head_tail is the steadiest (CV 0.7%, 435.79 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### nway shows alternating (throttle bounce) (autocorr -0.66)

nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest kernel_nway (242.90 us) to slowest sequential (763.88 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 242901.0 ns median (-44.3% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.14x (fastest 242901.0 ns, slowest 763875.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 4 |
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
| head_tail | 436229ns | 436443ns | 430781ns | 436205ns | 438990ns | base |
| head_tail_dyn | 443495ns | 442094ns | 438771ns | 441694ns | 448557ns | +1.67% |
| kernel_nway | 263677ns | 243609ns | 225281ns | 240802ns | 317188ns | -39.56% |
| kernel_seq | 767521ns | 763656ns | 757260ns | 762378ns | 780365ns | +75.94% |
| nway | 258378ns | 250042ns | 247240ns | 249462ns | 277323ns | -40.77% |
| sequential | 770352ns | 764125ns | 762021ns | 764069ns | 783943ns | +76.59% |
| std_parallel | 280292ns | 261318ns | 249500ns | 258781ns | 327953ns | -35.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 435474ns | 429646ns | 438318ns | base | 2.408 |
| head_tail_dyn | 442710ns | 438177ns | 447302ns | +1.66% | 2.369 |
| kernel_nway | 262708ns | 224635ns | 315729ns | -39.67% | 3.991 |
| kernel_seq | 766981ns | 756948ns | 779302ns | +76.13% | 1.367 |
| nway | 257681ns | 246344ns | 276734ns | -40.83% | 4.069 |
| sequential | 769878ns | 761708ns | 783078ns | +76.79% | 1.362 |
| std_parallel | 279729ns | 248917ns | 327266ns | -35.76% | 3.749 |

## Performance model

- Peak throughput: **4.668 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.406 | 51.5% |
| head_tail_dyn | 2.374 | 50.9% |
| kernel_nway | 4.317 | 92.5% |
| kernel_seq | 1.374 | 29.4% |
| nway | 4.206 | 90.1% |
| sequential | 1.373 | 29.4% |
| std_parallel | 4.020 | 86.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 436229ns | 436229ns | base |
| head_tail_dyn | 443495ns | 443495ns | +1.67% |
| kernel_nway | 263677ns | 263677ns | -39.56% |
| kernel_seq | 767521ns | 767521ns | +75.94% |
| nway | 258378ns | 258378ns | -40.77% |
| sequential | 770352ns | 770352ns | +76.59% |
| std_parallel | 280292ns | 280292ns | -35.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 435786ns | base | --- | [432318, 438318] | --- | --- | --- | --- |
| head_tail_dyn | 441635ns | +6093.7ns (+1.4%) | [+3135, +12479]ns | [439193, 447302] | YES | 0.0313 | 0.0313 | 0 |
| kernel_nway | 242901ns | -193697.8ns (-44.4%) | [-204964, -119635]ns | [229495, 315729] | YES | 0.0313 | 0.0313 | 0 |
| kernel_seq | 763375ns | +328135.4ns (+75.3%) | [+319948, +346438]ns | [758266, 779302] | YES | 0.0313 | 0.0313 | 0 |
| nway | 249302ns | -185265.6ns (-42.5%) | [-189990, -158125]ns | [247005, 276734] | YES | 0.0313 | 0.0313 | 0 |
| sequential | 763875ns | +330093.8ns (+75.7%) | [+325406, +347714]ns | [762682, 783078] | YES | 0.0313 | 0.0313 | 0 |
| std_parallel | 260823ns | -175718.8ns (-40.3%) | [-181865, -109651]ns | [251099, 327266] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 438047ns | +1.4% | -29.6% | +73.3% | -43.0% | +77.5% | -25.3% |
| 1 | 2 | 435411ns | +1.7% | -44.8% | +79.0% | -36.6% | +76.7% | -40.1% |
| 1 | 3 | 432964ns | +1.9% | -44.7% | +76.1% | -43.0% | +76.2% | -42.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.000 | ok |
| head_tail_dyn | -0.000 | ok |
| kernel_nway | -0.158 | ok |
| kernel_seq | -0.653 | HIGH- (thermal bounce) |
| nway | -0.662 | HIGH- (thermal bounce) |
| sequential | -0.005 | ok |
| std_parallel | -0.104 | ok |

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
| head_tail | 436710.1ns | 435474.0ns | 100.3% | HIGH |
| head_tail_dyn | 448637.2ns | 442710.0ns | 101.3% | HIGH |
| kernel_nway | 256114.6ns | 262708.3ns | 97.5% | HIGH |
| kernel_seq | 762402.8ns | 766980.9ns | 99.4% | HIGH |
| nway | 261812.5ns | 257680.6ns | 101.6% | HIGH |
| sequential | 770975.7ns | 769878.5ns | 100.1% | HIGH |
| std_parallel | 269578.1ns | 279729.2ns | 96.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 429645.8-438317.7 ns)
  429645.8 |####################
  430079.4 |
  430513.0 |
  430946.6 |
  431380.2 |
  431813.8 |
  432247.4 |
  432681.0 |
  433114.6 |
  433548.2 |
  433981.8 |
  434415.3 |
  434848.9 |####################
  435282.5 |
  435716.1 |########################################
  436149.7 |####################
  436583.3 |
  437016.9 |
  437450.5 |
  437884.1 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 438177.1-447302.0 ns)
  438177.1 |########################################
  438633.3 |
  439089.6 |
  439545.8 |
  440002.1 |########################################
  440458.3 |########################################
  440914.6 |
  441370.8 |
  441827.1 |
  442283.3 |########################################
  442739.6 |
  443195.8 |
  443652.1 |
  444108.3 |
  444564.6 |
  445020.8 |
  445477.1 |
  445933.3 |
  446389.6 |
  446845.8 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 224635.4-315729.2 ns)
  224635.4 |########################################
  229190.1 |
  233744.8 |########################################
  238299.5 |########################################
  242854.1 |########################################
  247408.8 |
  251963.5 |########################################
  256518.2 |
  261072.9 |
  265627.6 |
  270182.3 |
  274737.0 |
  279291.7 |
  283846.3 |
  288401.0 |
  292955.7 |
  297510.4 |
  302065.1 |
  306619.8 |
  311174.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 756947.9-779302.1 ns)
  756947.9 |########################################
  758065.6 |
  759183.3 |########################################
  760301.0 |
  761418.7 |########################################
  762536.4 |
  763654.2 |
  764771.9 |########################################
  765889.6 |########################################
  767007.3 |
  768125.0 |
  769242.7 |
  770360.4 |
  771478.1 |
  772595.8 |
  773713.6 |
  774831.3 |
  775949.0 |
  777066.7 |
  778184.4 |
  (0 below, 1 above range)

nway (n=6, range 246343.8-276734.3 ns)
  246343.8 |########################################
  247863.3 |####################
  249382.9 |########################################
  250902.4 |
  252421.9 |
  253941.4 |
  255461.0 |
  256980.5 |
  258500.0 |
  260019.5 |
  261539.1 |
  263058.6 |
  264578.1 |
  266097.7 |
  267617.2 |
  269136.7 |
  270656.2 |
  272175.8 |
  273695.3 |
  275214.8 |
  (0 below, 1 above range)

sequential (n=6, range 761708.3-783078.1 ns)
  761708.3 |####################
  762776.8 |########################################
  763845.3 |####################
  764913.8 |
  765982.3 |
  767050.8 |
  768119.3 |
  769187.7 |
  770256.2 |
  771324.7 |
  772393.2 |
  773461.7 |####################
  774530.2 |
  775598.7 |
  776667.2 |
  777735.7 |
  778804.2 |
  779872.7 |
  780941.2 |
  782009.7 |
  (0 below, 1 above range)

std_parallel (n=6, range 248916.7-327265.6 ns)
  248916.7 |####################
  252834.1 |####################
  256751.6 |####################
  260669.0 |########################################
  264586.5 |
  268503.9 |
  272421.4 |
  276338.8 |
  280256.3 |
  284173.7 |
  288091.2 |
  292008.6 |
  295926.0 |
  299843.5 |
  303760.9 |
  307678.4 |
  311595.8 |
  315513.3 |
  319430.7 |
  323348.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=99.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: bridge=99.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.0% of algo (FFI overhead may distort results)
