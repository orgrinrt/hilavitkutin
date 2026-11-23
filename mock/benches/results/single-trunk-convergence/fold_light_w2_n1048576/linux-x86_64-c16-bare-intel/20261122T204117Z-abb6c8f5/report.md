# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 31% faster than the next best (head_tail)

kernel_nway (75.37 us) leads head_tail (98.83 us) by 31%, a clear separation rather than a photo finish. CV 75.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 28% (significant)

kernel_nway is -28.05 us (28%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### head_tail_dyn is an outlier: 2.4x slower than the field

head_tail_dyn (182.31 us) is 2.4x the fastest (75.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.65)

kernel_seq's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### std_parallel is inconsistent: worst-20% is 4.8x its best-20%

std_parallel's best 20% of batches run at 115.83 us but its worst 20% at 560.02 us (4.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 75373.2 ns median (-23.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.42x (fastest 75373.2 ns, slowest 182310.3 ns)

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
| Counter frequency | 2112779539 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 130912ns | 99046ns | 95972ns | 98871ns | 196442ns | base |
| head_tail_dyn | 210806ns | 182655ns | 172528ns | 180415ns | 275532ns | +61.03% |
| kernel_nway | 109465ns | 75698ns | 65266ns | 76805ns | 180554ns | -16.38% |
| kernel_seq | 160125ns | 133516ns | 104973ns | 132846ns | 228618ns | +22.31% |
| nway | 124493ns | 106492ns | 81376ns | 98736ns | 184687ns | -4.90% |
| sequential | 105551ns | 105856ns | 100740ns | 105676ns | 107768ns | -19.37% |
| std_parallel | 280358ns | 153416ns | 116023ns | 146854ns | 562781ns | +114.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 130301ns | 95798ns | 195031ns | base | 8.047 |
| head_tail_dyn | 209610ns | 172138ns | 272620ns | +60.87% | 5.003 |
| kernel_nway | 108803ns | 64952ns | 179180ns | -16.50% | 9.637 |
| kernel_seq | 159301ns | 104679ns | 226815ns | +22.26% | 6.582 |
| nway | 124190ns | 81188ns | 184212ns | -4.69% | 8.443 |
| sequential | 105248ns | 100440ns | 107460ns | -19.23% | 9.963 |
| std_parallel | 279151ns | 115830ns | 560022ns | +114.24% | 3.756 |

## Performance model

- Peak throughput: **16.144 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.610 | 65.7% |
| head_tail_dyn | 5.752 | 35.6% |
| kernel_nway | 13.912 | 86.2% |
| kernel_seq | 7.877 | 48.8% |
| nway | 9.866 | 61.1% |
| sequential | 9.929 | 61.5% |
| std_parallel | 6.858 | 42.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 130912ns | 130912ns | base |
| head_tail_dyn | 210806ns | 210806ns | +61.03% |
| kernel_nway | 109465ns | 109465ns | -16.38% |
| kernel_seq | 160125ns | 160125ns | +22.31% |
| nway | 124493ns | 124493ns | -4.90% |
| sequential | 105551ns | 105551ns | -19.37% |
| std_parallel | 280358ns | 280358ns | +114.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 98830ns | base | --- | [97435, 148715] | --- | --- | --- | --- |
| head_tail_dyn | 182310ns | +78456.9ns (+79.4%) | [+64013, +84069]ns | [176430, 188420] | YES | 0.0004 | 0.0001 | 0 |
| kernel_nway | 75373ns | -28047.3ns (-28.4%) | [-32073, -17241]ns | [70065, 125066] | YES | 0.0151 | 0.0075 | 0 |
| kernel_seq | 133112ns | +12715.4ns (+12.9%) | [+3536, +57908]ns | [116534, 188359] | YES | 0.0463 | 0.0309 | 0 |
| nway | 106286ns | no significant difference | [-16233, +10033]ns | [86026, 149396] | no | 0.2855 | 0.2379 | 0 |
| sequential | 105609ns | no significant difference | [-37966, +7571]ns | [104756, 107174] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 152889ns | +30790.1ns (+31.2%) | [+20682, +64264]ns | [121609, 270994] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 124444ns | +50.9% | -28.3% | -0.8% | -11.5% | -13.8% | +22.7% |
| 1 | 2 | 98157ns | +81.0% | -28.2% | +91.9% | -16.4% | +7.6% | +23.6% |
| 1 | 3 | 97788ns | +77.0% | -0.5% | +11.1% | +27.6% | +7.1% | +148.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.160 | ok |
| head_tail_dyn | -0.031 | ok |
| kernel_nway | -0.609 | HIGH- (thermal bounce) |
| kernel_seq | -0.646 | HIGH- (thermal bounce) |
| nway | -0.591 | HIGH- (thermal bounce) |
| sequential | -0.021 | ok |
| std_parallel | -0.327 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 132430.5ns | 130301.0ns | 101.6% | HIGH |
| head_tail_dyn | 213046.3ns | 209609.6ns | 101.6% | HIGH |
| kernel_nway | 109645.5ns | 108802.7ns | 100.8% | HIGH |
| kernel_seq | 161816.7ns | 159300.8ns | 101.6% | HIGH |
| nway | 125080.5ns | 124189.7ns | 100.7% | HIGH |
| sequential | 106200.4ns | 105248.0ns | 100.9% | HIGH |
| std_parallel | 271185.2ns | 279150.9ns | 97.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 95798.2-195031.2 ns)
  95798.2 |########################################
  100759.9 |####
  105721.5 |
  110683.2 |
  115644.8 |
  120606.5 |
  125568.1 |
  130529.8 |
  135491.4 |
  140453.1 |####
  145414.7 |
  150376.3 |########
  155338.0 |
  160299.6 |
  165261.3 |
  170222.9 |####
  175184.6 |
  180146.2 |
  185107.9 |
  190069.5 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 172137.7-272620.5 ns)
  172137.7 |########################################
  177161.9 |########################
  182186.0 |################
  187210.1 |########################
  192234.3 |
  197258.4 |
  202282.6 |
  207306.7 |########
  212330.8 |
  217355.0 |
  222379.1 |
  227403.2 |
  232427.4 |########
  237451.5 |
  242475.7 |
  247499.8 |
  252523.9 |
  257548.1 |
  262572.2 |
  267596.3 |
  (1 below, 2 above range)

kernel_nway (n=18, range 64952.3-179180.2 ns)
  64952.3 |########################################
  70663.7 |################################
  76375.1 |########
  82086.4 |
  87797.8 |
  93509.2 |
  99220.6 |########
  104932.0 |
  110643.4 |########
  116354.8 |
  122066.2 |
  127777.6 |
  133489.0 |########
  139200.4 |
  144911.8 |
  150623.2 |
  156334.6 |
  162046.0 |
  167757.4 |########
  173468.8 |
  (1 below, 3 above range)

kernel_seq (n=18, range 104679.4-226814.8 ns)
  104679.4 |########################################
  110786.1 |
  116892.9 |##########
  122999.7 |####################
  129106.5 |##########
  135213.2 |##########
  141320.0 |
  147426.8 |##########
  153533.6 |
  159640.3 |
  165747.1 |
  171853.9 |
  177960.6 |
  184067.4 |########################################
  190174.2 |
  196281.0 |
  202387.7 |
  208494.5 |
  214601.3 |
  220708.1 |
  (1 below, 3 above range)

nway (n=18, range 81188.0-184211.9 ns)
  81188.0 |########################################
  86339.2 |####################
  91490.4 |##########
  96641.6 |
  101792.8 |####################
  106944.0 |##########
  112095.2 |
  117246.4 |
  122397.6 |
  127548.8 |##########
  132700.0 |
  137851.1 |##########
  143002.3 |
  148153.5 |
  153304.7 |
  158455.9 |##########
  163607.1 |
  168758.3 |##########
  173909.5 |##########
  179060.7 |
  (1 below, 2 above range)

sequential (n=18, range 100440.4-107460.3 ns)
  100440.4 |
  100791.4 |
  101142.4 |
  101493.4 |
  101844.4 |
  102195.4 |
  102546.4 |
  102897.4 |#############
  103248.4 |
  103599.4 |
  103950.4 |#############
  104301.3 |#############
  104652.3 |##########################
  105003.3 |#############
  105354.3 |########################################
  105705.3 |#############
  106056.3 |
  106407.3 |#############
  106758.3 |
  107109.3 |########################################
  (1 below, 3 above range)

std_parallel (n=18, range 115830.2-560022.4 ns)
  115830.2 |########################################
  138039.8 |
  160249.4 |##########
  182459.1 |
  204668.7 |#####
  226878.3 |
  249087.9 |#####
  271297.5 |#####
  293507.1 |
  315716.7 |#####
  337926.3 |
  360135.9 |
  382345.5 |
  404555.1 |
  426764.7 |
  448974.3 |
  471183.9 |
  493393.5 |
  515603.2 |
  537812.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=43.0% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=36.5% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.4% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=52.1% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=104.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=35.6% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=103.9% of algo (FFI overhead may distort results)
- **nway**: CV=39.0% (high variance, measurements may be unstable)
- **nway**: bridge=99.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=91.3% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **std_parallel**: bridge=104.9% of algo (FFI overhead may distort results)
