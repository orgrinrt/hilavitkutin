# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.3x slower than the field

std_parallel (264.21 us) is 2.3x the fastest (115.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail, kernel_nway) are a dead heat (<1%)

head_tail (115.52 us) and kernel_nway (116.65 us) differ by 0.98%, inside the noise, even though the wider field spreads 128.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (115.52 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### head_tail is inconsistent: worst-20% is 2.1x its best-20%

head_tail's best 20% of batches run at 97.54 us but its worst 20% at 203.05 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 115515.9 ns median
- 5 variants significantly slower than baseline
- Spread: 2.29x (fastest 115515.9 ns, slowest 264209.9 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 141502ns | 115716ns | 97821ns | 113812ns | 204878ns | base |
| head_tail_dyn | 143753ns | 121871ns | 120063ns | 121795ns | 188537ns | +1.59% |
| kernel_nway | 147717ns | 116906ns | 115061ns | 116301ns | 211168ns | +4.39% |
| kernel_seq | 221303ns | 195771ns | 192685ns | 195043ns | 275000ns | +56.40% |
| nway | 183698ns | 153745ns | 126510ns | 153736ns | 257235ns | +29.82% |
| sequential | 237769ns | 195580ns | 193385ns | 195242ns | 323753ns | +68.03% |
| std_parallel | 271283ns | 264455ns | 253605ns | 261826ns | 294309ns | +91.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 140720ns | 97537ns | 203046ns | base | 0.466 |
| head_tail_dyn | 143043ns | 119834ns | 186907ns | +1.65% | 0.458 |
| kernel_nway | 147033ns | 114873ns | 209661ns | +4.49% | 0.446 |
| kernel_seq | 220340ns | 192384ns | 273104ns | +56.58% | 0.297 |
| nway | 183379ns | 126307ns | 256705ns | +30.31% | 0.357 |
| sequential | 237574ns | 193224ns | 323508ns | +68.83% | 0.276 |
| std_parallel | 270666ns | 253306ns | 292971ns | +92.34% | 0.242 |

## Performance model

- Peak throughput: **0.672 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.567 | 84.4% |
| head_tail_dyn | 0.539 | 80.2% |
| kernel_nway | 0.562 | 83.6% |
| kernel_seq | 0.336 | 50.0% |
| nway | 0.427 | 63.5% |
| sequential | 0.335 | 49.9% |
| std_parallel | 0.248 | 36.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 141502ns | 141502ns | base |
| head_tail_dyn | 143753ns | 143753ns | +1.59% |
| kernel_nway | 147717ns | 147717ns | +4.39% |
| kernel_seq | 221303ns | 221303ns | +56.40% |
| nway | 183698ns | 183698ns | +29.82% |
| sequential | 237769ns | 237769ns | +68.03% |
| std_parallel | 271283ns | 271283ns | +91.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 115516ns | base | --- | [114915, 127591] | --- | --- | --- | --- |
| head_tail_dyn | 121652ns | +5784.5ns (+5.0%) | [+3944, +15602]ns | [121289, 127756] | YES | 0.0090 | 0.0075 | 0 |
| kernel_nway | 116647ns | no significant difference | [-355, +18474]ns | [115208, 137187] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 195202ns | +79020.1ns (+68.4%) | [+74362, +88630]ns | [193159, 197955] | YES | 0.0000 | 0.0000 | 0 |
| nway | 153513ns | +37204.5ns (+32.2%) | [+29623, +39952]ns | [152655, 173157] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 195410ns | +78150.5ns (+67.7%) | [+70526, +88672]ns | [194914, 195624] | YES | 0.0090 | 0.0075 | 0 |
| std_parallel | 264210ns | +148661.0ns (+128.7%) | [+137913, +156616]ns | [255865, 274077] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 115842ns | +3.6% | +0.3% | +66.7% | +32.8% | +68.9% | +133.8% |
| 1 | 2 | 115549ns | +5.8% | -0.3% | +69.5% | +32.2% | +67.2% | +130.2% |
| 1 | 3 | 98199ns | +24.0% | +19.2% | +98.5% | +55.6% | +99.0% | +160.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.158 | ok |
| head_tail_dyn | -0.284 | moderate- |
| kernel_nway | -0.508 | HIGH- (thermal bounce) |
| kernel_seq | -0.394 | moderate- |
| nway | -0.208 | moderate- |
| sequential | -0.659 | HIGH- (thermal bounce) |
| std_parallel | -0.028 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 146190.6ns | 140720.4ns | 103.9% | HIGH |
| head_tail_dyn | 146044.6ns | 143043.4ns | 102.1% | HIGH |
| kernel_nway | 148384.8ns | 147032.6ns | 100.9% | HIGH |
| kernel_seq | 220306.5ns | 220340.4ns | 100.0% | HIGH |
| nway | 186082.8ns | 183378.8ns | 101.5% | HIGH |
| sequential | 228302.2ns | 237573.6ns | 96.1% | HIGH |
| std_parallel | 273668.4ns | 270665.6ns | 101.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 97537.5-203045.6 ns)
  97537.5 |##########
  102812.9 |
  108088.3 |
  113363.7 |########################################
  118639.1 |
  123914.5 |#####
  129189.9 |#####
  134465.3 |#####
  139740.7 |
  145016.1 |
  150291.5 |
  155566.9 |
  160842.3 |
  166117.7 |
  171393.1 |
  176668.6 |
  181944.0 |
  187219.4 |
  192494.8 |
  197770.2 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 119834.1-186906.8 ns)
  119834.1 |########################################
  123187.7 |####
  126541.3 |
  129895.0 |####
  133248.6 |
  136602.3 |
  139955.9 |
  143309.5 |
  146663.2 |
  150016.8 |
  153370.5 |
  156724.1 |
  160077.7 |
  163431.4 |
  166785.0 |
  170138.6 |
  173492.3 |####
  176845.9 |
  180199.6 |
  183553.2 |
  (2 below, 3 above range)

kernel_nway (n=18, range 114872.6-209661.4 ns)
  114872.6 |########################################
  119612.1 |
  124351.5 |
  129091.0 |
  133830.4 |###
  138569.8 |
  143309.3 |###
  148048.7 |
  152788.1 |###
  157527.6 |
  162267.0 |
  167006.5 |
  171745.9 |
  176485.3 |
  181224.8 |
  185964.2 |
  190703.7 |
  195443.1 |
  200182.5 |
  204922.0 |
  (1 below, 3 above range)

kernel_seq (n=18, range 192383.8-273103.7 ns)
  192383.8 |########################################
  196419.8 |##########
  200455.8 |
  204491.8 |
  208527.8 |
  212563.8 |
  216599.8 |
  220635.8 |
  224671.8 |
  228707.8 |
  232743.8 |
  236779.8 |
  240815.7 |
  244851.7 |
  248887.7 |
  252923.7 |
  256959.7 |
  260995.7 |
  265031.7 |
  269067.7 |
  (1 below, 3 above range)

nway (n=18, range 126306.6-256704.7 ns)
  126306.6 |####
  132826.5 |
  139346.4 |
  145866.3 |
  152386.2 |########################################
  158906.1 |
  165426.0 |
  171945.9 |####
  178465.8 |########
  184985.7 |
  191505.6 |
  198025.5 |
  204545.4 |
  211065.4 |
  217585.3 |
  224105.2 |
  230625.1 |
  237145.0 |
  243664.9 |
  250184.8 |####
  (2 below, 2 above range)

sequential (n=18, range 193224.3-323507.8 ns)
  193224.3 |########################################
  199738.5 |
  206252.7 |
  212766.9 |
  219281.0 |
  225795.2 |
  232309.4 |
  238823.6 |
  245337.7 |
  251851.9 |
  258366.1 |
  264880.3 |
  271394.4 |
  277908.6 |
  284422.8 |
  290937.0 |
  297451.1 |
  303965.3 |
  310479.5 |
  316993.7 |
  (1 below, 1 above range)

std_parallel (n=18, range 253306.2-292971.5 ns)
  253306.2 |##########################
  255289.4 |##########################
  257272.7 |
  259256.0 |
  261239.2 |########################################
  263222.5 |
  265205.8 |##########################
  267189.0 |#############
  269172.3 |
  271155.6 |#############
  273138.8 |
  275122.1 |#############
  277105.4 |
  279088.6 |#############
  281071.9 |
  283055.2 |
  285038.4 |
  287021.7 |
  289005.0 |
  290988.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=44.5% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=28.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=41.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=29.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=99.4% of algo (FFI overhead may distort results)
- **nway**: CV=40.9% (high variance, measurements may be unstable)
- **nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **sequential**: CV=73.7% (high variance, measurements may be unstable)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.8% of algo (FFI overhead may distort results)
