# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway dominates: 18% faster than the next best (kernel_seq)

nway (86.56 us) leads kernel_seq (102.38 us) by 18%, a clear separation rather than a photo finish. CV 43.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 2.1x slower than the field

std_parallel (179.68 us) is 2.1x the fastest (86.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.63)

head_tail's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {nway, kernel_seq, kernel_nway, head_tail, sequential} vs {head_tail_dyn, std_parallel} (51% apart)

The field splits into a fast tier {nway, kernel_seq, kernel_nway, head_tail, sequential} and a slow tier {head_tail_dyn, std_parallel} with a 51% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### kernel_nway is inconsistent: worst-20% is 4.1x its best-20%

kernel_nway's best 20% of batches run at 63.43 us but its worst 20% at 260.94 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: nway** at 86559.6 ns median (-16.7% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 2.08x (fastest 86559.6 ns, slowest 179684.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
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
| head_tail | 117557ns | 104122ns | 84995ns | 97847ns | 163403ns | base |
| head_tail_dyn | 175402ns | 160480ns | 156609ns | 161569ns | 205546ns | +49.21% |
| kernel_nway | 141322ns | 102887ns | 63805ns | 86474ns | 262353ns | +20.22% |
| kernel_seq | 126838ns | 102660ns | 89667ns | 100706ns | 184622ns | +7.90% |
| nway | 104595ns | 86782ns | 74565ns | 87226ns | 145663ns | -11.03% |
| sequential | 107665ns | 106450ns | 102541ns | 105421ns | 113593ns | -8.41% |
| std_parallel | 214302ns | 180609ns | 117902ns | 168649ns | 330981ns | +82.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 117099ns | 84814ns | 162478ns | base | 8.955 |
| head_tail_dyn | 174607ns | 156362ns | 203893ns | +49.11% | 6.005 |
| kernel_nway | 140645ns | 63427ns | 260940ns | +20.11% | 7.455 |
| kernel_seq | 126185ns | 89522ns | 183078ns | +7.76% | 8.310 |
| nway | 104310ns | 74314ns | 145282ns | -10.92% | 10.053 |
| sequential | 107413ns | 102293ns | 113342ns | -8.27% | 9.762 |
| std_parallel | 213296ns | 117669ns | 329105ns | +82.15% | 4.916 |

## Performance model

- Peak throughput: **16.532 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.091 | 61.0% |
| head_tail_dyn | 6.549 | 39.6% |
| kernel_nway | 10.221 | 61.8% |
| kernel_seq | 10.242 | 62.0% |
| nway | 12.114 | 73.3% |
| sequential | 9.876 | 59.7% |
| std_parallel | 5.836 | 35.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 117557ns | 117557ns | base |
| head_tail_dyn | 175402ns | 175402ns | +49.21% |
| kernel_nway | 141322ns | 141322ns | +20.22% |
| kernel_seq | 126838ns | 126838ns | +7.90% |
| nway | 104595ns | 104595ns | -11.03% |
| sequential | 107665ns | 107665ns | -8.41% |
| std_parallel | 214302ns | 214302ns | +82.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 103912ns | base | --- | [90815, 119095] | --- | --- | --- | --- |
| head_tail_dyn | 160100ns | +57726.4ns (+55.6%) | [+51965, +72253]ns | [157216, 180323] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 102587ns | no significant difference | [-39939, +12808]ns | [67306, 141995] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 102377ns | no significant difference | [-14178, +16137]ns | [100180, 118098] | no | 0.7210 | 0.4807 | 0 |
| nway | 86560ns | no significant difference | [-30081, +4338]ns | [78425, 109317] | no | 0.7210 | 0.4807 | 0 |
| sequential | 106173ns | no significant difference | [-8456, +17350]ns | [103187, 110017] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 179684ns | +61687.1ns (+59.4%) | [+42319, +87232]ns | [147177, 205580] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 110656ns | +63.0% | +14.0% | -6.9% | -4.7% | +2.3% | +73.4% |
| 1 | 2 | 85253ns | +85.3% | +11.8% | +18.3% | +18.2% | +21.1% | +133.2% |
| 1 | 3 | 104762ns | +50.1% | -39.0% | -13.0% | -28.3% | -0.6% | +16.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.634 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.151 | ok |
| kernel_nway | -0.000 | ok |
| kernel_seq | -0.079 | ok |
| nway | -0.091 | ok |
| sequential | -0.216 | moderate- |
| std_parallel | -0.215 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 116536.3ns | 117099.0ns | 99.5% | HIGH |
| head_tail_dyn | 179866.1ns | 174607.4ns | 103.0% | HIGH |
| kernel_nway | 139175.8ns | 140644.8ns | 99.0% | HIGH |
| kernel_seq | 129226.5ns | 126185.0ns | 102.4% | HIGH |
| nway | 105150.6ns | 104309.7ns | 100.8% | HIGH |
| sequential | 110778.5ns | 107412.7ns | 103.1% | HIGH |
| std_parallel | 212029.0ns | 213295.6ns | 99.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 84813.9-162478.4 ns)
  84813.9 |########################################
  88697.1 |
  92580.3 |#############
  96463.6 |##########################
  100346.8 |##########################
  104230.0 |##########################
  108113.2 |
  111996.5 |#############
  115879.7 |
  119762.9 |
  123646.1 |#############
  127529.3 |
  131412.6 |#############
  135295.8 |
  139179.0 |
  143062.2 |
  146945.5 |
  150828.7 |
  154711.9 |
  158595.1 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 156361.6-203892.9 ns)
  156361.6 |########################################
  158738.2 |#####
  161114.8 |#####
  163491.3 |#####
  165867.9 |
  168244.4 |
  170621.0 |
  172997.6 |
  175374.1 |
  177750.7 |###########
  180127.2 |#####
  182503.8 |#####
  184880.4 |
  187256.9 |
  189633.5 |#####
  192010.1 |
  194386.6 |
  196763.2 |
  199139.7 |
  201516.3 |
  (1 below, 2 above range)

kernel_nway (n=18, range 63426.8-260940.5 ns)
  63426.8 |########################################
  73302.5 |######
  83178.2 |
  93053.9 |######
  102929.6 |####################
  112805.2 |
  122680.9 |
  132556.6 |######
  142432.3 |
  152308.0 |
  162183.7 |######
  172059.3 |######
  181935.0 |
  191810.7 |
  201686.4 |
  211562.1 |
  221437.7 |######
  231313.4 |######
  241189.1 |
  251064.8 |
  (1 below, 1 above range)

kernel_seq (n=18, range 89522.4-183078.0 ns)
  89522.4 |##########
  94200.2 |
  98878.0 |########################################
  103555.7 |#####
  108233.5 |
  112911.3 |
  117589.1 |
  122266.8 |
  126944.6 |#####
  131622.4 |#####
  136300.2 |
  140978.0 |
  145655.7 |
  150333.5 |
  155011.3 |
  159689.1 |#####
  164366.9 |
  169044.6 |
  173722.4 |
  178400.2 |
  (2 below, 2 above range)

nway (n=18, range 74314.1-145282.5 ns)
  74314.1 |##########################
  77862.6 |########################################
  81411.0 |
  84959.4 |########################################
  88507.8 |
  92056.2 |
  95604.6 |
  99153.1 |
  102701.5 |##########################
  106249.9 |#############
  109798.3 |#############
  113346.7 |#############
  116895.2 |
  120443.6 |
  123992.0 |
  127540.4 |
  131088.8 |
  134637.2 |
  138185.7 |
  141734.1 |
  (2 below, 3 above range)

sequential (n=18, range 102292.9-113342.1 ns)
  102292.9 |##########
  102845.4 |########################################
  103397.9 |##########
  103950.3 |
  104502.8 |
  105055.2 |
  105607.7 |##########
  106160.1 |##########
  106712.6 |
  107265.1 |
  107817.5 |
  108370.0 |##########
  108922.4 |##########
  109474.9 |##########
  110027.3 |##########
  110579.8 |
  111132.3 |
  111684.7 |
  112237.2 |
  112789.6 |##########
  (2 below, 3 above range)

std_parallel (n=18, range 117669.0-329104.7 ns)
  117669.0 |########################################
  128240.8 |
  138812.6 |########################################
  149384.4 |
  159956.2 |####################
  170527.9 |########################################
  181099.7 |####################
  191671.5 |########################################
  202243.3 |########################################
  212815.1 |
  223386.9 |
  233958.6 |
  244530.4 |####################
  255102.2 |########################################
  265674.0 |
  276245.8 |
  286817.5 |
  297389.3 |
  307961.1 |
  318532.9 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=34.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=97.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=91.1% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **kernel_nway**: bridge=96.5% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=47.4% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.5% of algo (FFI overhead may distort results)
- **nway**: CV=36.2% (high variance, measurements may be unstable)
- **nway**: bridge=101.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=104.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=68.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=95.8% of algo (FFI overhead may distort results)
