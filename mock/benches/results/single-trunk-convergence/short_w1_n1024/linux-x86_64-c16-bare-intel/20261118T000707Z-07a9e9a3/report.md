# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 3.8x slower than the field

std_parallel (36.61 us) is 3.8x the fastest (9.69 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {kernel_seq, kernel_nway, sequential, head_tail, head_tail_dyn} vs {nway, std_parallel} (162% apart)

The field splits into a fast tier {kernel_seq, kernel_nway, sequential, head_tail, head_tail_dyn} and a slow tier {nway, std_parallel} with a 162% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.8x the fastest

Fastest kernel_seq (9.69 us) to slowest std_parallel (36.61 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.4x its best-20%

sequential's best 20% of batches run at 6.76 us but its worst 20% at 22.89 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_seq** at 9692.4 ns median (-18.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.78x (fastest 9692.4 ns, slowest 36612.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12416ns | 12526ns | 10687ns | 12154ns | 13673ns | base |
| head_tail_dyn | 13162ns | 13076ns | 10347ns | 12521ns | 15531ns | +6.01% |
| kernel_nway | 11218ns | 11052ns | 9696ns | 10833ns | 12557ns | -9.65% |
| kernel_seq | 10905ns | 10330ns | 8532ns | 10095ns | 13307ns | -12.17% |
| nway | 33975ns | 32275ns | 28765ns | 31649ns | 40068ns | +173.64% |
| sequential | 14585ns | 11690ns | 6894ns | 11285ns | 23382ns | +17.47% |
| std_parallel | 37369ns | 37352ns | 28581ns | 36144ns | 43600ns | +200.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 11605ns | 10083ns | 12513ns | base | 0.088 |
| head_tail_dyn | 12415ns | 9872ns | 14551ns | +6.98% | 0.082 |
| kernel_nway | 10511ns | 9143ns | 11677ns | -9.43% | 0.097 |
| kernel_seq | 10213ns | 8090ns | 12358ns | -11.99% | 0.100 |
| nway | 33478ns | 28368ns | 39436ns | +188.46% | 0.031 |
| sequential | 14292ns | 6758ns | 22893ns | +23.15% | 0.072 |
| std_parallel | 36601ns | 28171ns | 42576ns | +215.38% | 0.028 |

## Performance model

- Peak throughput: **0.152 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.086 | 57.0% |
| head_tail_dyn | 0.084 | 55.7% |
| kernel_nway | 0.099 | 65.2% |
| kernel_seq | 0.106 | 69.7% |
| nway | 0.032 | 21.2% |
| sequential | 0.089 | 59.0% |
| std_parallel | 0.028 | 18.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12416ns | 12416ns | base |
| head_tail_dyn | 13162ns | 13162ns | +6.01% |
| kernel_nway | 11218ns | 11218ns | -9.65% |
| kernel_seq | 10905ns | 10905ns | -12.17% |
| nway | 33975ns | 33975ns | +173.64% |
| sequential | 14585ns | 14585ns | +17.47% |
| std_parallel | 37369ns | 37369ns | +200.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11860ns | base | --- | [11167, 12236] | --- | --- | --- | --- |
| head_tail_dyn | 12138ns | no significant difference | [-449, +2178]ns | [11218, 13596] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 10363ns | -603.2ns (-5.1%) | [-2014, -317]ns | [9800, 11344] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9692ns | -1853.2ns (-15.6%) | [-2249, -674]ns | [9025, 11095] | YES | 0.0113 | 0.0075 | 0 |
| nway | 31811ns | +20840.1ns (+175.7%) | [+18346, +22963]ns | [29879, 34906] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 11462ns | no significant difference | [-3574, +9233]ns | [7090, 21095] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 36612ns | +24655.2ns (+207.9%) | [+22781, +26591]ns | [34328, 39074] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11701ns | -10.9% | -3.8% | -19.2% | +193.1% | +21.8% | +157.6% |
| 1 | 2 | 12483ns | +17.0% | -20.5% | -2.5% | +144.9% | -10.4% | +204.4% |
| 1 | 3 | 11742ns | +3.4% | -9.5% | -22.1% | +160.8% | +17.9% | +225.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.665 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.556 | HIGH- (thermal bounce) |
| kernel_nway | -0.517 | HIGH- (thermal bounce) |
| kernel_seq | -0.661 | HIGH- (thermal bounce) |
| nway | -0.174 | ok |
| sequential | -0.657 | HIGH- (thermal bounce) |
| std_parallel | -0.151 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12354.3ns | 11605.4ns | 106.5% | HIGH |
| head_tail_dyn | 12767.1ns | 12415.0ns | 102.8% | HIGH |
| kernel_nway | 10859.6ns | 10511.2ns | 103.3% | HIGH |
| kernel_seq | 10532.7ns | 10213.4ns | 103.1% | HIGH |
| nway | 34629.1ns | 33477.5ns | 103.4% | HIGH |
| sequential | 14779.7ns | 14292.4ns | 103.4% | HIGH |
| std_parallel | 37636.9ns | 36601.4ns | 102.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 10083.1-12512.8 ns)
  10083.1 |
  10204.6 |#############
  10326.1 |#############
  10447.6 |#############
  10569.1 |
  10690.5 |
  10812.0 |
  10933.5 |
  11055.0 |#############
  11176.5 |#############
  11297.9 |
  11419.4 |
  11540.9 |##########################
  11662.4 |#############
  11783.9 |
  11905.3 |########################################
  12026.8 |
  12148.3 |#############
  12269.8 |#############
  12391.3 |#############
  (1 below, 3 above range)

head_tail_dyn (n=18, range 9871.9-14551.1 ns)
   9871.9 |#############
  10105.9 |
  10339.8 |##########################
  10573.8 |
  10807.7 |
  11041.7 |##########################
  11275.7 |
  11509.6 |########################################
  11743.6 |
  11977.5 |
  12211.5 |
  12445.5 |#############
  12679.4 |#############
  12913.4 |
  13147.3 |#############
  13381.3 |#############
  13615.2 |#############
  13849.2 |
  14083.2 |#############
  14317.1 |
  (1 below, 3 above range)

kernel_nway (n=18, range 9142.8-11677.3 ns)
   9142.8 |########################################
   9269.5 |
   9396.3 |
   9523.0 |
   9649.7 |########################################
   9776.4 |####################
   9903.2 |####################
  10029.9 |########################################
  10156.6 |
  10283.3 |
  10410.1 |
  10536.8 |########################################
  10663.5 |
  10790.2 |
  10917.0 |####################
  11043.7 |
  11170.4 |
  11297.2 |####################
  11423.9 |####################
  11550.6 |####################
  (1 below, 3 above range)

kernel_seq (n=18, range 8089.7-12358.1 ns)
   8089.7 |#############
   8303.1 |#############
   8516.5 |##########################
   8729.9 |
   8943.4 |
   9156.8 |#############
   9370.2 |#############
   9583.6 |########################################
   9797.0 |#############
  10010.5 |
  10223.9 |##########################
  10437.3 |
  10650.7 |
  10864.1 |
  11077.6 |
  11291.0 |
  11504.4 |
  11717.8 |#############
  11931.2 |#############
  12144.7 |#############
  (1 below, 2 above range)

nway (n=18, range 28368.4-39436.0 ns)
  28368.4 |########################################
  28921.7 |####################
  29475.1 |####################
  30028.5 |####################
  30581.9 |####################
  31135.3 |########################################
  31688.7 |####################
  32242.0 |####################
  32795.4 |####################
  33348.8 |
  33902.2 |
  34455.6 |####################
  35008.9 |####################
  35562.3 |
  36115.7 |####################
  36669.1 |
  37222.5 |
  37775.8 |
  38329.2 |
  38882.6 |
  (1 below, 3 above range)

sequential (n=18, range 6758.1-22892.7 ns)
   6758.1 |########################################
   7564.9 |#####
   8371.6 |
   9178.3 |
   9985.0 |
  10791.8 |
  11598.5 |
  12405.2 |
  13212.0 |
  14018.7 |#####
  14825.4 |
  15632.1 |
  16438.9 |
  17245.6 |
  18052.3 |
  18859.1 |
  19665.8 |###########
  20472.5 |#####
  21279.2 |#####
  22086.0 |#####
  (1 below, 3 above range)

std_parallel (n=18, range 28170.6-42575.5 ns)
  28170.6 |
  28890.8 |#############
  29611.1 |
  30331.3 |#############
  31051.6 |
  31771.8 |
  32492.0 |
  33212.3 |#############
  33932.5 |
  34652.8 |#############
  35373.0 |########################################
  36093.3 |
  36813.5 |########################################
  37533.8 |
  38254.0 |#############
  38974.3 |##########################
  39694.5 |##########################
  40414.8 |
  41135.0 |
  41855.3 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=105.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.6% of algo (FFI overhead may distort results)
- **nway**: bridge=103.1% of algo (FFI overhead may distort results)
- **sequential**: CV=51.9% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=104.8% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.1% of algo (FFI overhead may distort results)
