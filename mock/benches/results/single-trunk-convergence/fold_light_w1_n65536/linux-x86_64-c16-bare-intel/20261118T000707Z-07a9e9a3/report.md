# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 36% faster than the next best (head_tail_dyn)

sequential (22.29 us) leads head_tail_dyn (30.30 us) by 36%, a clear separation rather than a photo finish. CV 63.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 34% (significant)

sequential is -12.18 us (34%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 3.0x slower than the field

std_parallel (67.83 us) is 3.0x the fastest (22.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 63.1%)

sequential wins on median (22.29 us) yet has the highest variance (CV 63.1%), while head_tail_dyn is the steadiest (CV 18.4%, 30.30 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.53)

head_tail's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.0x the fastest

Fastest sequential (22.29 us) to slowest std_parallel (67.83 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 12.17 us but its worst 20% at 42.50 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 22290.8 ns median (-37.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.04x (fastest 22290.8 ns, slowest 67831.5 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 37625ns | 36412ns | 27754ns | 34758ns | 46861ns | base |
| head_tail_dyn | 33934ns | 31229ns | 27939ns | 31232ns | 40986ns | -9.81% |
| kernel_nway | 38655ns | 39297ns | 24110ns | 35438ns | 50753ns | +2.74% |
| kernel_seq | 34991ns | 32906ns | 28270ns | 31692ns | 43302ns | -7.00% |
| nway | 62503ns | 57835ns | 47820ns | 56209ns | 79287ns | +66.12% |
| sequential | 26487ns | 22579ns | 12329ns | 20142ns | 43085ns | -29.60% |
| std_parallel | 73077ns | 69211ns | 58357ns | 66273ns | 90644ns | +94.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 36624ns | 26953ns | 45134ns | base | 1.789 |
| head_tail_dyn | 33013ns | 27384ns | 40103ns | -9.86% | 1.985 |
| kernel_nway | 37528ns | 23557ns | 48949ns | +2.47% | 1.746 |
| kernel_seq | 34081ns | 27685ns | 42074ns | -6.94% | 1.923 |
| nway | 61724ns | 47276ns | 78248ns | +68.54% | 1.062 |
| sequential | 26120ns | 12168ns | 42499ns | -28.68% | 2.509 |
| std_parallel | 72066ns | 57796ns | 89339ns | +96.78% | 0.909 |

## Performance model

- Peak throughput: **5.386 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.829 | 34.0% |
| head_tail_dyn | 2.163 | 40.2% |
| kernel_nway | 1.703 | 31.6% |
| kernel_seq | 2.036 | 37.8% |
| nway | 1.146 | 21.3% |
| sequential | 2.940 | 54.6% |
| std_parallel | 0.966 | 17.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 37625ns | 37625ns | base |
| head_tail_dyn | 33934ns | 33934ns | -9.81% |
| kernel_nway | 38655ns | 38655ns | +2.74% |
| kernel_seq | 34991ns | 34991ns | -7.00% |
| nway | 62503ns | 62503ns | +66.12% |
| sequential | 26487ns | 26487ns | -29.60% |
| std_parallel | 73077ns | 73077ns | +94.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 35830ns | base | --- | [31155, 40518] | --- | --- | --- | --- |
| head_tail_dyn | 30302ns | -3725.4ns (-10.4%) | [-5897, -790]ns | [28611, 37357] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| kernel_nway | 38475ns | no significant difference | [-7021, +9765]ns | [30487, 44106] | no | 0.9774 | 0.8145 | 0 |
| kernel_seq | 32196ns | no significant difference | [-9781, +3134]ns | [29292, 36565] | no | 1.0000 | 1.0000 | 0 |
| nway | 57177ns | +20340.7ns (+56.8%) | [+15594, +29426]ns | [50946, 64994] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 22291ns | -12180.2ns (-34.0%) | [-20907, -1925]ns | [12620, 38536] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| std_parallel | 67831ns | +33495.1ns (+93.5%) | [+26039, +41509]ns | [61038, 78925] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 35075ns | -19.6% | +4.0% | +19.0% | +44.5% | -63.5% | +115.4% |
| 1 | 2 | 27747ns | +6.0% | +48.8% | +2.2% | +135.1% | +19.9% | +136.8% |
| 1 | 3 | 41257ns | -3.9% | -40.5% | -25.3% | +46.2% | -40.7% | +62.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.527 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.115 | ok |
| kernel_nway | -0.347 | moderate- |
| kernel_seq | -0.274 | moderate- |
| nway | -0.385 | moderate- |
| sequential | -0.451 | moderate- |
| std_parallel | -0.243 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 36602.9ns | 36623.6ns | 99.9% | HIGH |
| head_tail_dyn | 33028.6ns | 33012.8ns | 100.0% | HIGH |
| kernel_nway | 38364.5ns | 37528.4ns | 102.2% | HIGH |
| kernel_seq | 34233.1ns | 34081.4ns | 100.4% | HIGH |
| nway | 63887.6ns | 61723.7ns | 103.5% | HIGH |
| sequential | 27091.6ns | 26120.3ns | 103.7% | HIGH |
| std_parallel | 72345.3ns | 72066.0ns | 100.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 26952.7-45134.1 ns)
  26952.7 |####################
  27861.8 |####################
  28770.8 |####################
  29679.9 |
  30589.0 |
  31498.0 |
  32407.1 |########################################
  33316.2 |####################
  34225.2 |
  35134.3 |########################################
  36043.4 |
  36952.5 |
  37861.5 |####################
  38770.6 |####################
  39679.7 |########################################
  40588.7 |
  41497.8 |####################
  42406.9 |
  43315.9 |
  44225.0 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 27384.4-40102.5 ns)
  27384.4 |####################
  28020.3 |########################################
  28656.2 |########################################
  29292.1 |########################################
  29928.0 |
  30563.9 |####################
  31199.8 |####################
  31835.7 |
  32471.7 |
  33107.6 |
  33743.5 |
  34379.4 |
  35015.3 |####################
  35651.2 |
  36287.1 |
  36923.0 |####################
  37558.9 |####################
  38194.8 |####################
  38830.7 |
  39466.6 |
  (2 below, 3 above range)

kernel_nway (n=18, range 23556.6-48948.9 ns)
  23556.6 |####################
  24826.2 |####################
  26095.8 |
  27365.4 |
  28635.0 |
  29904.6 |########################################
  31174.3 |####################
  32443.9 |
  33713.5 |
  34983.1 |####################
  36252.7 |
  37522.3 |####################
  38792.0 |########################################
  40061.6 |####################
  41331.2 |
  42600.8 |####################
  43870.4 |
  45140.0 |####################
  46409.7 |####################
  47679.3 |
  (2 below, 3 above range)

kernel_seq (n=18, range 27685.2-42073.8 ns)
  27685.2 |####################
  28404.6 |
  29124.1 |########################################
  29843.5 |
  30562.9 |
  31282.4 |##########
  32001.8 |####################
  32721.2 |
  33440.6 |
  34160.1 |##########
  34879.5 |
  35598.9 |##########
  36318.4 |##########
  37037.8 |
  37757.2 |
  38476.7 |
  39196.1 |##########
  39915.5 |
  40635.0 |
  41354.4 |
  (2 below, 3 above range)

nway (n=18, range 47276.3-78247.5 ns)
  47276.3 |##########################
  48824.9 |
  50373.5 |########################################
  51922.0 |#############
  53470.6 |
  55019.1 |#############
  56567.7 |##########################
  58116.2 |
  59664.8 |
  61213.4 |#############
  62761.9 |##########################
  64310.5 |
  65859.0 |##########################
  67407.6 |#############
  68956.2 |
  70504.7 |
  72053.3 |
  73601.8 |
  75150.4 |
  76699.0 |
  (1 below, 2 above range)

sequential (n=18, range 12167.6-42499.5 ns)
  12167.6 |########################################
  13684.2 |
  15200.8 |
  16717.4 |
  18234.0 |
  19750.5 |
  21267.1 |
  22783.7 |
  24300.3 |
  25816.9 |
  27333.5 |
  28850.1 |
  30366.7 |#####
  31883.3 |
  33399.9 |
  34916.5 |##########
  36433.1 |
  37949.7 |#####
  39466.3 |##########
  40982.9 |
  (1 below, 3 above range)

std_parallel (n=18, range 57796.1-89338.9 ns)
  57796.1 |########################################
  59373.2 |########################################
  60950.4 |########################################
  62527.5 |
  64104.7 |
  65681.8 |####################
  67258.9 |########################################
  68836.1 |####################
  70413.2 |####################
  71990.4 |
  73567.5 |
  75144.6 |
  76721.8 |####################
  78298.9 |####################
  79876.1 |
  81453.2 |####################
  83030.4 |####################
  84607.5 |
  86184.6 |
  87761.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=20.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.8% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=26.7% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=98.2% of algo (FFI overhead may distort results)
- **nway**: CV=25.7% (high variance, measurements may be unstable)
- **nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **sequential**: CV=53.8% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=112.4% of algo (FFI overhead may distort results)
- **std_parallel**: CV=20.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=102.1% of algo (FFI overhead may distort results)
