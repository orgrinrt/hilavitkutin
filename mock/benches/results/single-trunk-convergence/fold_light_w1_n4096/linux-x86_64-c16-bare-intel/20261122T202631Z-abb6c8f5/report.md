# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 3.4x slower than the field

nway (44.21 us) is 3.4x the fastest (12.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn is fastest but the noisiest (CV 32.9%)

head_tail_dyn wins on median (12.90 us) yet has the highest variance (CV 32.9%), while kernel_nway is the steadiest (CV 9.6%, 16.86 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (head_tail_dyn, head_tail) are a dead heat (<1%)

head_tail_dyn (12.90 us) and head_tail (12.98 us) differ by 0.66%, inside the noise, even though the wider field spreads 242.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest head_tail_dyn (12.90 us) to slowest nway (44.21 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 2.0x its best-20%

sequential's best 20% of batches run at 17.65 us but its worst 20% at 35.86 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 12896.0 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.43x (fastest 12896.0 ns, slowest 44213.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112795948 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 15958ns | 14035ns | 12492ns | 13843ns | 20862ns | base |
| head_tail_dyn | 16088ns | 14217ns | 11173ns | 13944ns | 21759ns | +0.81% |
| kernel_nway | 17869ns | 17643ns | 15184ns | 17006ns | 20505ns | +11.98% |
| kernel_seq | 15435ns | 14830ns | 12261ns | 13957ns | 19240ns | -3.27% |
| nway | 44677ns | 44830ns | 37041ns | 42940ns | 51101ns | +179.97% |
| sequential | 28914ns | 28981ns | 18030ns | 27417ns | 36601ns | +81.19% |
| std_parallel | 42410ns | 41153ns | 36873ns | 40327ns | 48303ns | +165.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 14805ns | 11888ns | 19329ns | base | 0.277 |
| head_tail_dyn | 15103ns | 10692ns | 20574ns | +2.02% | 0.271 |
| kernel_nway | 16711ns | 14545ns | 18448ns | +12.87% | 0.245 |
| kernel_seq | 14245ns | 11612ns | 17355ns | -3.78% | 0.288 |
| nway | 44021ns | 36523ns | 50354ns | +197.34% | 0.093 |
| sequential | 28335ns | 17653ns | 35858ns | +91.39% | 0.145 |
| std_parallel | 41399ns | 36135ns | 46719ns | +179.63% | 0.099 |

## Performance model

- Peak throughput: **0.383 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.316 | 82.4% |
| head_tail_dyn | 0.318 | 82.9% |
| kernel_nway | 0.243 | 63.4% |
| kernel_seq | 0.311 | 81.3% |
| nway | 0.093 | 24.2% |
| sequential | 0.144 | 37.6% |
| std_parallel | 0.101 | 26.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15958ns | 15958ns | base |
| head_tail_dyn | 16088ns | 16088ns | +0.81% |
| kernel_nway | 17869ns | 17869ns | +11.98% |
| kernel_seq | 15435ns | 15435ns | -3.27% |
| nway | 44677ns | 44677ns | +179.97% |
| sequential | 28914ns | 28914ns | +81.19% |
| std_parallel | 42410ns | 42410ns | +165.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12981ns | base | --- | [12412, 18975] | --- | --- | --- | --- |
| head_tail_dyn | 12896ns | no significant difference | [-5545, +6345]ns | [11812, 19320] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 16863ns | no significant difference | [-605, +4348]ns | [15541, 17106] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 13152ns | -668.1ns (-5.1%) | [-2533, -28]ns | [12468, 14732] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| nway | 44214ns | +28978.1ns (+223.2%) | [+25081, +32276]ns | [40123, 48356] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 28431ns | +13267.9ns (+102.2%) | [+11218, +16761]ns | [23695, 33257] | YES | 0.0003 | 0.0001 | 0 |
| std_parallel | 40457ns | +26279.5ns (+202.5%) | [+24665, +27847]ns | [38433, 43986] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12412ns | -6.5% | +31.1% | -5.7% | +220.7% | +141.4% | +214.7% |
| 1 | 2 | 12981ns | +52.6% | +36.1% | -3.5% | +245.3% | +82.5% | +210.3% |
| 1 | 3 | 19225ns | -36.0% | -14.2% | -23.3% | +144.1% | +77.4% | +121.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.125 | ok |
| head_tail_dyn | -0.663 | HIGH- (thermal bounce) |
| kernel_nway | -0.651 | HIGH- (thermal bounce) |
| kernel_seq | -0.043 | ok |
| nway | -0.035 | ok |
| sequential | -0.563 | HIGH- (thermal bounce) |
| std_parallel | -0.020 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 15408.7ns | 14804.8ns | 104.1% | HIGH |
| head_tail_dyn | 15188.5ns | 15103.2ns | 100.6% | HIGH |
| kernel_nway | 18203.4ns | 16710.9ns | 108.9% | HIGH |
| kernel_seq | 15073.5ns | 14244.6ns | 105.8% | HIGH |
| nway | 47677.2ns | 44020.6ns | 108.3% | HIGH |
| sequential | 29338.5ns | 28334.5ns | 103.5% | HIGH |
| std_parallel | 43977.2ns | 41399.3ns | 106.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11887.6-19329.3 ns)
  11887.6 |################
  12259.7 |########################################
  12631.8 |########
  13003.9 |################
  13375.9 |########
  13748.0 |
  14120.1 |
  14492.2 |
  14864.3 |
  15236.4 |
  15608.5 |
  15980.5 |
  16352.6 |
  16724.7 |
  17096.8 |
  17468.9 |
  17841.0 |
  18213.1 |
  18585.1 |########
  18957.2 |########################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 10692.0-20574.2 ns)
  10692.0 |
  11186.1 |##########################
  11680.2 |########################################
  12174.4 |##########################
  12668.5 |
  13162.6 |#############
  13656.7 |#############
  14150.8 |
  14644.9 |
  15139.0 |
  15633.1 |
  16127.2 |
  16621.3 |#############
  17115.4 |
  17609.5 |
  18103.7 |
  18597.8 |
  19091.9 |########################################
  19586.0 |
  20080.1 |#############
  (2 below, 2 above range)

kernel_nway (n=18, range 14544.6-18448.3 ns)
  14544.6 |########################################
  14739.8 |
  14935.0 |########################################
  15130.2 |
  15325.3 |
  15520.5 |
  15715.7 |
  15910.9 |########################################
  16106.1 |
  16301.3 |
  16496.5 |####################
  16691.6 |########################################
  16886.8 |########################################
  17082.0 |########################################
  17277.2 |
  17472.4 |
  17667.6 |
  17862.8 |
  18057.9 |####################
  18253.1 |
  (1 below, 3 above range)

kernel_seq (n=18, range 11612.2-17355.5 ns)
  11612.2 |########################
  11899.4 |
  12186.6 |########
  12473.7 |########################
  12760.9 |########
  13048.0 |
  13335.2 |########
  13622.4 |
  13909.5 |
  14196.7 |
  14483.9 |########################################
  14771.0 |
  15058.2 |
  15345.3 |
  15632.5 |
  15919.7 |
  16206.8 |
  16494.0 |################
  16781.2 |
  17068.3 |
  (1 below, 1 above range)

nway (n=18, range 36522.9-50353.7 ns)
  36522.9 |
  37214.4 |##########################
  37906.0 |
  38597.5 |#############
  39289.1 |
  39980.6 |########################################
  40672.1 |
  41363.7 |
  42055.2 |#############
  42746.8 |
  43438.3 |#############
  44129.8 |#############
  44821.4 |##########################
  45512.9 |
  46204.4 |
  46896.0 |
  47587.5 |
  48279.1 |##########################
  48970.6 |#############
  49662.1 |#############
  (1 below, 2 above range)

sequential (n=18, range 17652.9-35857.6 ns)
  17652.9 |
  18563.2 |
  19473.4 |
  20383.6 |
  21293.9 |####################
  22204.1 |####################
  23114.3 |####################
  24024.6 |####################
  24934.8 |####################
  25845.0 |
  26755.3 |####################
  27665.5 |########################################
  28575.7 |
  29486.0 |
  30396.2 |####################
  31306.4 |####################
  32216.7 |####################
  33126.9 |
  34037.1 |####################
  34947.4 |####################
  (2 below, 3 above range)

std_parallel (n=18, range 36135.3-46719.0 ns)
  36135.3 |#############
  36664.5 |#############
  37193.7 |
  37722.9 |
  38252.0 |########################################
  38781.2 |#############
  39310.4 |#############
  39839.6 |
  40368.8 |
  40898.0 |
  41427.2 |##########################
  41956.4 |
  42485.5 |#############
  43014.7 |
  43543.9 |#############
  44073.1 |#############
  44602.3 |
  45131.5 |
  45660.7 |##########################
  46189.9 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=21.9% (high variance, measurements may be unstable)
- **head_tail**: bridge=103.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=28.1% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=97.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=104.5% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=23.7% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.0% of algo (FFI overhead may distort results)
- **nway**: bridge=102.4% of algo (FFI overhead may distort results)
- **sequential**: CV=24.1% (high variance, measurements may be unstable)
- **sequential**: bridge=104.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=108.3% of algo (FFI overhead may distort results)
