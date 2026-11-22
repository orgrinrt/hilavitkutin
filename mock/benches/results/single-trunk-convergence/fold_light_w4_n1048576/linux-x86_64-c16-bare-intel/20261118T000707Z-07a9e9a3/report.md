# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway dominates: 47% faster than the next best (head_tail)

nway (67.05 us) leads head_tail (98.27 us) by 47%, a clear separation rather than a photo finish. CV 50.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### nway beats baseline by 38% (significant)

nway is -37.44 us (38%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### head_tail_dyn is an outlier: 2.9x slower than the field

head_tail_dyn (193.47 us) is 2.9x the fastest (67.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 7.0x its best-20%

kernel_nway's best 20% of batches run at 56.13 us but its worst 20% at 391.71 us (7.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: nway** at 67048.1 ns median (-31.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.89x (fastest 67048.1 ns, slowest 193472.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
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
| head_tail | 115303ns | 98480ns | 96411ns | 98780ns | 149534ns | base |
| head_tail_dyn | 352253ns | 193839ns | 159697ns | 186412ns | 697292ns | +205.50% |
| kernel_nway | 188445ns | 112867ns | 56302ns | 95188ns | 394402ns | +63.43% |
| kernel_seq | 137877ns | 115924ns | 109078ns | 114276ns | 187679ns | +19.58% |
| nway | 85528ns | 67328ns | 53415ns | 67228ns | 129034ns | -25.82% |
| sequential | 103094ns | 102088ns | 99876ns | 101693ns | 106805ns | -10.59% |
| std_parallel | 205920ns | 181580ns | 93596ns | 166016ns | 321939ns | +78.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 114790ns | 96250ns | 148439ns | base | 9.135 |
| head_tail_dyn | 350827ns | 159246ns | 694262ns | +205.62% | 2.989 |
| kernel_nway | 187181ns | 56130ns | 391713ns | +63.06% | 5.602 |
| kernel_seq | 137107ns | 108793ns | 185952ns | +19.44% | 7.648 |
| nway | 85190ns | 53128ns | 128554ns | -25.79% | 12.309 |
| sequential | 102820ns | 99584ns | 106549ns | -10.43% | 10.198 |
| std_parallel | 204670ns | 93354ns | 319563ns | +78.30% | 5.123 |

## Performance model

- Peak throughput: **19.737 Gops/s** (nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.670 | 54.1% |
| head_tail_dyn | 5.420 | 27.5% |
| kernel_nway | 9.366 | 47.5% |
| kernel_seq | 9.066 | 45.9% |
| nway | 15.639 | 79.2% |
| sequential | 10.299 | 52.2% |
| std_parallel | 5.804 | 29.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 115303ns | 115303ns | base |
| head_tail_dyn | 352253ns | 352253ns | +205.50% |
| kernel_nway | 188445ns | 188445ns | +63.43% |
| kernel_seq | 137877ns | 137877ns | +19.58% |
| nway | 85528ns | 85528ns | -25.82% |
| sequential | 103094ns | 103094ns | -10.59% |
| std_parallel | 205920ns | 205920ns | +78.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 98269ns | base | --- | [97232, 111096] | --- | --- | --- | --- |
| head_tail_dyn | 193472ns | +76242.4ns (+77.6%) | [+61945, +100341]ns | [171356, 240116] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 111961ns | no significant difference | [-32864, +171687]ns | [67341, 278260] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 115660ns | +15692.5ns (+16.0%) | [+8511, +22343]ns | [110963, 128025] | YES | 0.0039 | 0.0013 | 0 |
| nway | 67048ns | -37438.2ns (-38.1%) | [-44993, -16330]ns | [60256, 105651] | YES | 0.0463 | 0.0309 | 0 |
| sequential | 101815ns | no significant difference | [-6515, +3932]ns | [100782, 105639] | no | 0.1155 | 0.0963 | 0 |
| std_parallel | 180673ns | +70920.4ns (+72.2%) | [+31470, +105463]ns | [139634, 244775] | YES | 0.0151 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 97018ns | +103.3% | -30.6% | +23.0% | -2.6% | +3.1% | +20.1% |
| 1 | 2 | 105634ns | +62.2% | +220.3% | +5.0% | -42.6% | +0.2% | +107.5% |
| 1 | 3 | 97990ns | +102.5% | +10.2% | +16.0% | -38.5% | +3.9% | +90.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.660 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.666 | HIGH- (thermal bounce) |
| kernel_nway | -0.654 | HIGH- (thermal bounce) |
| kernel_seq | -0.374 | moderate- |
| nway | -0.161 | ok |
| sequential | -0.611 | HIGH- (thermal bounce) |
| std_parallel | -0.369 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 114768.5ns | 114790.2ns | 100.0% | HIGH |
| head_tail_dyn | 330685.2ns | 350827.1ns | 94.3% | HIGH |
| kernel_nway | 191275.4ns | 187180.7ns | 102.2% | HIGH |
| kernel_seq | 140044.8ns | 137107.2ns | 102.1% | HIGH |
| nway | 88110.4ns | 85189.5ns | 103.4% | HIGH |
| sequential | 103626.7ns | 102820.2ns | 100.8% | HIGH |
| std_parallel | 203830.5ns | 204670.2ns | 99.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 96250.1-148439.1 ns)
  96250.1 |########################################
  98859.6 |####
  101469.0 |####
  104078.5 |
  106687.9 |####
  109297.4 |
  111906.8 |
  114516.3 |####
  117125.7 |
  119735.2 |
  122344.6 |
  124954.1 |
  127563.5 |
  130173.0 |
  132782.4 |
  135391.9 |
  138001.3 |
  140610.8 |
  143220.2 |####
  145829.7 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 159245.6-694262.4 ns)
  159245.6 |########################################
  185996.5 |####################
  212747.3 |#############
  239498.1 |######
  266249.0 |
  292999.8 |
  319750.6 |
  346501.5 |
  373252.3 |
  400003.2 |
  426754.0 |
  453504.8 |
  480255.7 |
  507006.5 |
  533757.3 |
  560508.2 |
  587259.0 |
  614009.8 |
  640760.7 |
  667511.5 |
  (2 below, 4 above range)

kernel_nway (n=18, range 56129.5-391712.9 ns)
  56129.5 |########################################
  72908.7 |##########
  89687.8 |####################
  106467.0 |####################
  123246.2 |##########
  140025.3 |
  156804.5 |
  173583.7 |
  190362.8 |
  207142.0 |
  223921.2 |
  240700.4 |
  257479.5 |
  274258.7 |##########
  291037.9 |
  307817.0 |
  324596.2 |
  341375.4 |
  358154.5 |##########
  374933.7 |
  (2 below, 4 above range)

kernel_seq (n=18, range 108792.8-185952.2 ns)
  108792.8 |########################################
  112650.8 |#############
  116508.7 |##########################
  120366.7 |
  124224.7 |
  128082.6 |
  131940.6 |
  135798.6 |######
  139656.5 |
  143514.5 |######
  147372.5 |
  151230.5 |######
  155088.4 |
  158946.4 |
  162804.4 |
  166662.3 |
  170520.3 |
  174378.3 |
  178236.2 |
  182094.2 |
  (1 below, 2 above range)

nway (n=18, range 53127.6-128554.3 ns)
  53127.6 |##########
  56898.9 |########################################
  60670.3 |##########
  64441.6 |##########
  68212.9 |##########
  71984.3 |
  75755.6 |
  79527.0 |####################
  83298.3 |
  87069.6 |
  90841.0 |
  94612.3 |
  98383.6 |
  102155.0 |####################
  105926.3 |
  109697.7 |##########
  113469.0 |
  117240.3 |
  121011.7 |
  124783.0 |
  (2 below, 3 above range)

sequential (n=18, range 99584.4-106548.6 ns)
  99584.4 |
  99932.6 |####################
  100280.9 |##########
  100629.1 |
  100977.3 |####################
  101325.5 |
  101673.7 |##############################
  102021.9 |####################
  102370.1 |
  102718.3 |
  103066.5 |
  103414.7 |
  103762.9 |
  104111.2 |
  104459.4 |
  104807.6 |
  105155.8 |
  105504.0 |########################################
  105852.2 |
  106200.4 |
  (2 below, 2 above range)

std_parallel (n=18, range 93354.2-319562.9 ns)
  93354.2 |########################################
  104664.7 |####################
  115975.1 |
  127285.5 |####################
  138596.0 |####################
  149906.4 |####################
  161216.8 |####################
  172527.3 |####################
  183837.7 |####################
  195148.1 |####################
  206458.6 |####################
  217769.0 |
  229079.5 |####################
  240389.9 |####################
  251700.3 |####################
  263010.8 |
  274321.2 |####################
  285631.6 |
  296942.1 |
  308252.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=26.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=88.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **head_tail_dyn**: bridge=97.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=80.2% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 7.0x (possible bimodal distribution)
- **kernel_nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=38.4% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.6% of algo (FFI overhead may distort results)
- **nway**: CV=40.0% (high variance, measurements may be unstable)
- **nway**: bridge=116.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: CV=51.0% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **std_parallel**: bridge=100.1% of algo (FFI overhead may distort results)
