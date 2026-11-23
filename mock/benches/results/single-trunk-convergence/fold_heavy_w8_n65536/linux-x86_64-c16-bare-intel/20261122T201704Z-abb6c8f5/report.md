# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 50% faster than the next best (head_tail)

kernel_nway (77.82 us) leads head_tail (116.75 us) by 50%, a clear separation rather than a photo finish. CV 67.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 36% (significant)

kernel_nway is -41.94 us (36%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### nway is an outlier: 4.1x slower than the field

nway (321.25 us) is 4.1x the fastest (77.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.1x the fastest

Fastest kernel_nway (77.82 us) to slowest nway (321.25 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 4.3x its best-20%

nway's best 20% of batches run at 149.85 us but its worst 20% at 651.22 us (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 77819.3 ns median (-33.3% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 4.13x (fastest 77819.3 ns, slowest 321249.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112765502 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 159968ns | 117184ns | 115439ns | 118331ns | 244687ns | base |
| head_tail_dyn | 148772ns | 128726ns | 115920ns | 127379ns | 197288ns | -7.00% |
| kernel_nway | 110513ns | 78239ns | 67493ns | 75828ns | 184049ns | -30.92% |
| kernel_seq | 212108ns | 194398ns | 192108ns | 193758ns | 249633ns | +32.59% |
| nway | 390743ns | 322122ns | 150169ns | 296118ns | 652967ns | +144.26% |
| sequential | 194418ns | 193690ns | 193134ns | 193714ns | 196116ns | +21.54% |
| std_parallel | 322119ns | 251464ns | 148299ns | 227409ns | 551093ns | +101.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 159106ns | 115217ns | 242737ns | base | 0.412 |
| head_tail_dyn | 148140ns | 115587ns | 195945ns | -6.89% | 0.442 |
| kernel_nway | 109841ns | 67272ns | 182650ns | -30.96% | 0.597 |
| kernel_seq | 210983ns | 191832ns | 247145ns | +32.60% | 0.311 |
| nway | 389729ns | 149849ns | 651223ns | +144.95% | 0.168 |
| sequential | 194242ns | 192961ns | 195946ns | +22.08% | 0.337 |
| std_parallel | 320638ns | 147686ns | 548214ns | +101.52% | 0.204 |

## Performance model

- Peak throughput: **0.974 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.561 | 57.6% |
| head_tail_dyn | 0.510 | 52.4% |
| kernel_nway | 0.842 | 86.4% |
| kernel_seq | 0.338 | 34.7% |
| nway | 0.204 | 20.9% |
| sequential | 0.339 | 34.8% |
| std_parallel | 0.261 | 26.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 159968ns | 159968ns | base |
| head_tail_dyn | 148772ns | 148772ns | -7.00% |
| kernel_nway | 110513ns | 110513ns | -30.92% |
| kernel_seq | 212108ns | 212108ns | +32.59% |
| nway | 390743ns | 390743ns | +144.26% |
| sequential | 194418ns | 194418ns | +21.54% |
| std_parallel | 322119ns | 322119ns | +101.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 116755ns | base | --- | [115711, 159348] | --- | --- | --- | --- |
| head_tail_dyn | 128404ns | no significant difference | [-20587, +5908]ns | [121214, 152478] | no | 0.2379 | 0.2379 | 0 |
| kernel_nway | 77819ns | -41939.2ns (-35.9%) | [-48705, -36079]ns | [72579, 167004] | YES | 0.0090 | 0.0075 | 0 |
| kernel_seq | 193711ns | +75580.8ns (+64.7%) | [+62845, +77431]ns | [192540, 195289] | YES | 0.0039 | 0.0013 | 0 |
| nway | 321250ns | +166908.1ns (+143.0%) | [+34887, +478165]ns | [201103, 597432] | YES | 0.0090 | 0.0075 | 0 |
| sequential | 193514ns | +76281.6ns (+65.3%) | [+36253, +78912]ns | [193181, 195295] | YES | 0.0090 | 0.0075 | 0 |
| std_parallel | 250754ns | +77393.3ns (+66.3%) | [+54046, +127615]ns | [186577, 341606] | YES | 0.0009 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 115711ns | +5.1% | -32.7% | +66.3% | +182.4% | +68.9% | +126.7% |
| 1 | 2 | 118345ns | +2.6% | -42.9% | +63.5% | +89.0% | +63.5% | +94.9% |
| 1 | 3 | 123096ns | +23.9% | +0.7% | +58.4% | +322.0% | +56.8% | +94.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.018 | ok |
| head_tail_dyn | -0.170 | ok |
| kernel_nway | -0.274 | moderate- |
| kernel_seq | -0.010 | ok |
| nway | -0.392 | moderate- |
| sequential | -0.071 | ok |
| std_parallel | -0.335 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 160626.1ns | 159106.2ns | 101.0% | HIGH |
| head_tail_dyn | 151040.1ns | 148140.0ns | 102.0% | HIGH |
| kernel_nway | 111002.6ns | 109841.3ns | 101.1% | HIGH |
| kernel_seq | 210748.8ns | 210982.5ns | 99.9% | HIGH |
| nway | 391903.0ns | 389729.0ns | 100.6% | HIGH |
| sequential | 194131.3ns | 194241.8ns | 99.9% | HIGH |
| std_parallel | 343323.6ns | 320638.0ns | 107.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 115217.1-242737.2 ns)
  115217.1 |########################################
  121593.1 |
  127969.1 |########
  134345.1 |
  140721.1 |
  147097.2 |
  153473.2 |
  159849.2 |
  166225.2 |
  172601.2 |
  178977.2 |####
  185353.2 |
  191729.2 |####
  198105.2 |
  204481.2 |####
  210857.2 |
  217233.2 |
  223609.2 |
  229985.2 |
  236361.2 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 115586.6-195944.8 ns)
  115586.6 |
  119604.5 |########################################
  123622.4 |
  127640.4 |
  131658.3 |#####
  135676.2 |#####
  139694.1 |
  143712.0 |##########
  147729.9 |
  151747.8 |
  155765.7 |#####
  159783.6 |
  163801.5 |
  167819.4 |#####
  171837.3 |#####
  175855.2 |
  179873.1 |
  183891.0 |
  187908.9 |
  191926.8 |
  (1 below, 2 above range)

kernel_nway (n=18, range 67272.4-182649.8 ns)
  67272.4 |########################################
  73041.2 |########################################
  78810.1 |####################
  84579.0 |
  90347.8 |
  96116.7 |
  101885.6 |
  107654.5 |
  113423.3 |
  119192.2 |
  124961.1 |
  130729.9 |
  136498.8 |
  142267.7 |
  148036.5 |
  153805.4 |
  159574.3 |
  165343.2 |####################
  171112.0 |
  176880.9 |####################
  (2 below, 2 above range)

kernel_seq (n=18, range 191831.9-247144.7 ns)
  191831.9 |########################################
  194597.5 |##########
  197363.2 |
  200128.8 |
  202894.5 |
  205660.1 |
  208425.7 |
  211191.4 |
  213957.0 |
  216722.7 |
  219488.3 |
  222253.9 |
  225019.6 |
  227785.2 |
  230550.9 |
  233316.5 |
  236082.1 |
  238847.8 |
  241613.4 |
  244379.1 |
  (1 below, 3 above range)

nway (n=18, range 149848.7-651223.4 ns)
  149848.7 |##########################
  174917.4 |##########################
  199986.2 |########################################
  225054.9 |#############
  250123.6 |
  275192.4 |
  300261.1 |
  325329.8 |
  350398.6 |
  375467.3 |#############
  400536.1 |
  425604.8 |#############
  450673.5 |
  475742.3 |
  500811.0 |
  525879.7 |##########################
  550948.5 |
  576017.2 |
  601085.9 |
  626154.7 |#############
  (1 below, 4 above range)

sequential (n=18, range 192961.1-195945.7 ns)
  192961.1 |########################################
  193110.3 |##########
  193259.5 |####################
  193408.8 |##############################
  193558.0 |
  193707.2 |
  193856.5 |
  194005.7 |
  194154.9 |
  194304.2 |
  194453.4 |
  194602.6 |
  194751.9 |
  194901.1 |
  195050.3 |##########
  195199.6 |##########
  195348.8 |####################
  195498.0 |##########
  195647.3 |
  195796.5 |
  (1 below, 2 above range)

std_parallel (n=18, range 147685.8-548214.2 ns)
  147685.8 |########################################
  167712.3 |########################################
  187738.7 |########################################
  207765.1 |
  227791.5 |########################################
  247817.9 |####################
  267844.3 |########################################
  287870.8 |
  307897.2 |
  327923.6 |########################################
  347950.0 |####################
  367976.4 |
  388002.8 |
  408029.3 |
  428055.7 |
  448082.1 |
  468108.5 |
  488134.9 |
  508161.3 |
  528187.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=52.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=30.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=47.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=99.8% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=21.1% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: CV=54.6% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **nway**: bridge=98.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.0% of algo (FFI overhead may distort results)
- **std_parallel**: CV=63.5% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **std_parallel**: bridge=111.8% of algo (FFI overhead may distort results)
