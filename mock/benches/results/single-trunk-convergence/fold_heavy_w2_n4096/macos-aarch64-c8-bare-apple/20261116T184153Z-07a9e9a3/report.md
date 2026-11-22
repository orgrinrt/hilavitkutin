# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq dominates: 15% faster than the next best (head_tail)

kernel_seq (5.87 us) leads head_tail (6.75 us) by 15%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 5.9x slower than the field

std_parallel (34.88 us) is 5.9x the fastest (5.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.9x the fastest

Fastest kernel_seq (5.87 us) to slowest std_parallel (34.88 us): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 3.3x its best-20%

kernel_nway's best 20% of batches run at 6.54 us but its worst 20% at 21.88 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 5872.9 ns median (-12.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 5.94x (fastest 5872.9 ns, slowest 34881.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 7492ns | 7423ns | 7201ns | 7360ns | 7835ns | base |
| head_tail_dyn | 7608ns | 7650ns | 7022ns | 7523ns | 8029ns | +1.55% |
| kernel_nway | 13018ns | 8404ns | 7139ns | 8339ns | 22975ns | +73.76% |
| kernel_seq | 6452ns | 6385ns | 6065ns | 6349ns | 6801ns | -13.88% |
| nway | 22414ns | 22227ns | 21671ns | 22155ns | 23174ns | +199.17% |
| sequential | 13631ns | 13629ns | 12876ns | 13441ns | 14294ns | +81.95% |
| std_parallel | 36844ns | 35342ns | 31758ns | 34342ns | 43139ns | +391.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 6834ns | 6538ns | 7184ns | base | 0.599 |
| head_tail_dyn | 6981ns | 6467ns | 7385ns | +2.15% | 0.587 |
| kernel_nway | 12243ns | 6538ns | 21877ns | +79.15% | 0.335 |
| kernel_seq | 5936ns | 5626ns | 6277ns | -13.14% | 0.690 |
| nway | 21864ns | 21147ns | 22616ns | +219.94% | 0.187 |
| sequential | 13480ns | 12739ns | 14146ns | +97.25% | 0.304 |
| std_parallel | 36452ns | 31374ns | 42743ns | +433.41% | 0.112 |

## Performance model

- Peak throughput: **0.728 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.607 | 83.4% |
| head_tail_dyn | 0.589 | 80.9% |
| kernel_nway | 0.526 | 72.2% |
| kernel_seq | 0.697 | 95.8% |
| nway | 0.189 | 26.0% |
| sequential | 0.305 | 41.8% |
| std_parallel | 0.117 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 7492ns | 7492ns | base |
| head_tail_dyn | 7608ns | 7608ns | +1.55% |
| kernel_nway | 13018ns | 13018ns | +73.76% |
| kernel_seq | 6452ns | 6452ns | -13.88% |
| nway | 22414ns | 22414ns | +199.17% |
| sequential | 13631ns | 13631ns | +81.95% |
| std_parallel | 36844ns | 36844ns | +391.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 6746ns | base | --- | [6623, 6890] | --- | --- | --- | --- |
| head_tail_dyn | 6956ns | no significant difference | [-156, +435]ns | [6777, 7133] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 7792ns | +1083.3ns (+16.1%) | [+371, +9869]ns | [6996, 17079] | YES | 0.0016 | 0.0013 | 0 |
| kernel_seq | 5873ns | -864.6ns (-12.8%) | [-1060, -738]ns | [5721, 6048] | YES | 0.0002 | 0.0001 | 0 |
| nway | 21658ns | +14768.8ns (+218.9%) | [+14652, +15354]ns | [21385, 22150] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 13446ns | +6670.9ns (+98.9%) | [+6371, +6906]ns | [12938, 13740] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 34881ns | +28216.7ns (+418.3%) | [+25600, +29802]ns | [32500, 36494] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 6838ns | +2.0% | +164.5% | -14.7% | +224.4% | +108.8% | +411.6% |
| 1 | 2 | 6706ns | +3.4% | +4.3% | -7.9% | +220.9% | +99.4% | +382.2% |
| 1 | 3 | 6648ns | +4.9% | +11.8% | -12.9% | +222.7% | +98.1% | +459.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.031 | ok |
| head_tail_dyn | -0.665 | HIGH- (thermal bounce) |
| kernel_nway | -0.187 | ok |
| kernel_seq | -0.661 | HIGH- (thermal bounce) |
| nway | -0.123 | ok |
| sequential | -0.078 | ok |
| std_parallel | -0.529 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 87694.4ns | 6833.8ns | 1283.2% | HIGH |
| head_tail_dyn | 88045.4ns | 6980.8ns | 1261.3% | HIGH |
| kernel_nway | 93975.2ns | 12242.8ns | 767.6% | HIGH |
| kernel_seq | 90689.1ns | 5935.6ns | 1527.9% | HIGH |
| nway | 100583.8ns | 21864.1ns | 460.0% | HIGH |
| sequential | 89624.5ns | 13479.9ns | 664.9% | HIGH |
| std_parallel | 108073.8ns | 36452.1ns | 296.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 6537.5-7184.0 ns)
   6537.5 |
   6569.9 |##########################
   6602.2 |########################################
   6634.5 |
   6666.8 |
   6699.2 |##########################
   6731.5 |
   6763.8 |#############
   6796.1 |##########################
   6828.5 |
   6860.8 |#############
   6893.1 |#############
   6925.4 |
   6957.8 |#############
   6990.1 |
   7022.4 |
   7054.7 |
   7087.1 |#############
   7119.4 |
   7151.7 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 6466.7-7384.7 ns)
   6466.7 |
   6512.6 |####################
   6558.5 |
   6604.4 |
   6650.3 |
   6696.2 |####################
   6742.1 |####################
   6788.0 |########################################
   6833.9 |####################
   6879.8 |####################
   6925.7 |
   6971.6 |####################
   7017.5 |####################
   7063.4 |####################
   7109.3 |########################################
   7155.2 |
   7201.1 |
   7247.0 |####################
   7292.9 |
   7338.8 |
  (2 below, 3 above range)

kernel_nway (n=18, range 6537.5-21877.1 ns)
   6537.5 |########################################
   7304.5 |########################
   8071.5 |########
   8838.5 |
   9605.4 |########
  10372.4 |
  11139.4 |
  11906.4 |
  12673.4 |
  13440.3 |
  14207.3 |
  14974.3 |
  15741.3 |
  16508.2 |################
  17275.2 |########
  18042.2 |########
  18809.2 |
  19576.2 |
  20343.1 |
  21110.1 |
  (2 below, 2 above range)

kernel_seq (n=18, range 5626.4-6277.1 ns)
   5626.4 |####################
   5658.9 |########################################
   5691.5 |####################
   5724.0 |####################
   5756.5 |####################
   5789.1 |####################
   5821.6 |####################
   5854.1 |
   5886.7 |########################################
   5919.2 |####################
   5951.7 |
   5984.3 |####################
   6016.8 |
   6049.3 |
   6081.9 |####################
   6114.4 |
   6146.9 |####################
   6179.5 |
   6212.0 |
   6244.5 |####################
  (1 below, 2 above range)

nway (n=18, range 21147.2-22616.0 ns)
  21147.2 |####################
  21220.6 |####################
  21294.1 |####################
  21367.5 |########################################
  21441.0 |####################
  21514.4 |########################################
  21587.8 |
  21661.3 |
  21734.7 |####################
  21808.1 |
  21881.6 |####################
  21955.0 |
  22028.5 |####################
  22101.9 |####################
  22175.3 |########################################
  22248.8 |
  22322.2 |
  22395.7 |####################
  22469.1 |
  22542.5 |
  (1 below, 2 above range)

sequential (n=18, range 12738.9-14145.8 ns)
  12738.9 |####################
  12809.2 |
  12879.6 |########################################
  12949.9 |####################
  13020.3 |
  13090.6 |
  13161.0 |
  13231.3 |
  13301.7 |########################################
  13372.0 |####################
  13442.4 |####################
  13512.7 |
  13583.0 |########################################
  13653.4 |####################
  13723.7 |
  13794.1 |####################
  13864.4 |
  13934.8 |
  14005.1 |
  14075.5 |
  (2 below, 4 above range)

std_parallel (n=18, range 31373.6-42743.0 ns)
  31373.6 |######
  31942.1 |####################
  32510.5 |######
  33079.0 |######
  33647.5 |
  34216.0 |
  34784.4 |########################################
  35352.9 |
  35921.4 |
  36489.9 |
  37058.3 |
  37626.8 |######
  38195.3 |
  38763.7 |######
  39332.2 |
  39900.7 |
  40469.2 |
  41037.6 |
  41606.1 |
  42174.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=1303.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1263.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=63.3% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **kernel_nway**: bridge=1138.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1547.7% of algo (FFI overhead may distort results)
- **nway**: bridge=463.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=668.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=300.7% of algo (FFI overhead may distort results)
