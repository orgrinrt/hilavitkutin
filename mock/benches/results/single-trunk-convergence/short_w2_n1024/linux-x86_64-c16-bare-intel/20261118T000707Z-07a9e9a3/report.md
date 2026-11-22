# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 73% faster than the next best (kernel_seq)

sequential (7.17 us) leads kernel_seq (12.43 us) by 73%, a clear separation rather than a photo finish. CV 121.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 6.9x slower than the field

std_parallel (49.80 us) is 6.9x the fastest (7.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 121.4%)

sequential wins on median (7.17 us) yet has the highest variance (CV 121.4%), while kernel_nway is the steadiest (CV 12.0%, 12.93 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.9x the fastest

Fastest sequential (7.17 us) to slowest std_parallel (49.80 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.83 us but its worst 20% at 24.85 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7168.0 ns median (-48.8% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 6.95x (fastest 7168.0 ns, slowest 49801.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
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
| head_tail | 15268ns | 15011ns | 12489ns | 14393ns | 17971ns | base |
| head_tail_dyn | 14205ns | 13826ns | 12103ns | 13528ns | 16272ns | -6.96% |
| kernel_nway | 14166ns | 13802ns | 11789ns | 13666ns | 16106ns | -7.22% |
| kernel_seq | 14460ns | 13122ns | 11703ns | 13080ns | 17910ns | -5.29% |
| nway | 35565ns | 34255ns | 29941ns | 33263ns | 41831ns | +132.94% |
| sequential | 14327ns | 7304ns | 6972ns | 9412ns | 25376ns | -6.16% |
| std_parallel | 52560ns | 50592ns | 41542ns | 49940ns | 61997ns | +244.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 14418ns | 11892ns | 16876ns | base | 0.071 |
| head_tail_dyn | 13395ns | 11500ns | 15235ns | -7.10% | 0.076 |
| kernel_nway | 13369ns | 11203ns | 15101ns | -7.28% | 0.077 |
| kernel_seq | 13559ns | 11136ns | 16629ns | -5.96% | 0.076 |
| nway | 35071ns | 29534ns | 41235ns | +143.24% | 0.029 |
| sequential | 14037ns | 6831ns | 24855ns | -2.65% | 0.073 |
| std_parallel | 51879ns | 41129ns | 61073ns | +259.81% | 0.020 |

## Performance model

- Peak throughput: **0.150 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.073 | 48.8% |
| head_tail_dyn | 0.078 | 52.0% |
| kernel_nway | 0.079 | 52.8% |
| kernel_seq | 0.082 | 55.0% |
| nway | 0.030 | 20.2% |
| sequential | 0.143 | 95.3% |
| std_parallel | 0.021 | 13.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15268ns | 15268ns | base |
| head_tail_dyn | 14205ns | 14205ns | -6.96% |
| kernel_nway | 14166ns | 14166ns | -7.22% |
| kernel_seq | 14460ns | 14460ns | -5.29% |
| nway | 35565ns | 35565ns | +132.94% |
| sequential | 14327ns | 14327ns | -6.16% |
| std_parallel | 52560ns | 52560ns | +244.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 14006ns | base | --- | [12756, 15165] | --- | --- | --- | --- |
| head_tail_dyn | 13127ns | no significant difference | [-2511, +781]ns | [12636, 13909] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 12927ns | no significant difference | [-3188, +966]ns | [12375, 14531] | no | 0.7210 | 0.4807 | 0 |
| kernel_seq | 12427ns | no significant difference | [-2738, +1857]ns | [11291, 15351] | no | 0.4758 | 0.2379 | 0 |
| nway | 33806ns | +19939.8ns (+142.4%) | [+18384, +21395]ns | [31253, 36916] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7168ns | no significant difference | [-5600, +3058]ns | [7048, 20095] | no | 0.9774 | 0.8145 | 0 |
| std_parallel | 49802ns | +36053.5ns (+257.4%) | [+33304, +38807]ns | [48391, 52750] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12255ns | +5.5% | +16.9% | -8.7% | +154.8% | -42.8% | +308.1% |
| 1 | 2 | 15189ns | -16.8% | -22.5% | -18.2% | +121.8% | -18.7% | +223.6% |
| 1 | 3 | 13713ns | +8.5% | +5.0% | +17.1% | +153.6% | -14.4% | +277.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.502 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.240 | moderate- |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.050 | ok |
| nway | -0.032 | ok |
| sequential | -0.231 | moderate- |
| std_parallel | -0.377 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 14793.1ns | 14418.4ns | 102.6% | HIGH |
| head_tail_dyn | 14051.2ns | 13395.0ns | 104.9% | HIGH |
| kernel_nway | 13709.3ns | 13368.9ns | 102.5% | HIGH |
| kernel_seq | 14230.7ns | 13559.0ns | 105.0% | HIGH |
| nway | 35644.2ns | 35071.1ns | 101.6% | HIGH |
| sequential | 14370.7ns | 14036.7ns | 102.4% | HIGH |
| std_parallel | 51322.8ns | 51878.8ns | 98.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11892.0-16876.4 ns)
  11892.0 |##########################
  12141.2 |
  12390.4 |#############
  12639.6 |##########################
  12888.9 |
  13138.1 |##########################
  13387.3 |
  13636.5 |#############
  13885.8 |
  14135.0 |#############
  14384.2 |
  14633.4 |
  14882.6 |##########################
  15131.9 |########################################
  15381.1 |
  15630.3 |#############
  15879.5 |
  16128.8 |
  16378.0 |
  16627.2 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 11500.1-15235.1 ns)
  11500.1 |####################
  11686.8 |##########
  11873.6 |
  12060.3 |
  12247.1 |
  12433.8 |
  12620.6 |########################################
  12807.3 |
  12994.1 |##########
  13180.8 |##############################
  13367.6 |##########
  13554.3 |
  13741.1 |
  13927.8 |
  14114.6 |
  14301.3 |##########
  14488.1 |
  14674.8 |##########
  14861.6 |##########
  15048.3 |
  (1 below, 2 above range)

kernel_nway (n=18, range 11203.5-15100.7 ns)
  11203.5 |
  11398.3 |
  11593.2 |################
  11788.0 |########
  11982.9 |
  12177.8 |
  12372.6 |
  12567.5 |
  12762.4 |########################################
  12957.2 |
  13152.1 |
  13346.9 |
  13541.8 |
  13736.7 |
  13931.5 |########
  14126.4 |
  14321.2 |################
  14516.1 |########
  14711.0 |################
  14905.8 |
  (2 below, 2 above range)

kernel_seq (n=18, range 11136.0-16629.5 ns)
  11136.0 |########################################
  11410.7 |########
  11685.3 |
  11960.0 |########
  12234.7 |########
  12509.4 |########
  12784.0 |
  13058.7 |
  13333.4 |
  13608.1 |
  13882.7 |########
  14157.4 |
  14432.1 |
  14706.8 |################
  14981.4 |
  15256.1 |
  15530.8 |########
  15805.4 |########
  16080.1 |########
  16354.8 |
  (1 below, 2 above range)

nway (n=18, range 29534.0-41234.9 ns)
  29534.0 |########################################
  30119.1 |
  30704.1 |########################################
  31289.2 |####################
  31874.2 |####################
  32459.3 |####################
  33044.3 |
  33629.3 |########################################
  34214.4 |########################################
  34799.4 |
  35384.5 |####################
  35969.5 |
  36554.6 |
  37139.6 |
  37724.7 |####################
  38309.7 |
  38894.8 |
  39479.8 |
  40064.8 |
  40649.9 |####################
  (1 below, 3 above range)

sequential (n=18, range 6831.4-24854.9 ns)
   6831.4 |########################################
   7732.6 |
   8633.7 |
   9534.9 |
  10436.1 |
  11337.3 |
  12238.4 |
  13139.6 |
  14040.8 |
  14942.0 |
  15843.1 |####
  16744.3 |########
  17645.5 |
  18546.7 |
  19447.8 |
  20349.0 |
  21250.2 |
  22151.3 |####
  23052.5 |####
  23953.7 |
  (1 below, 3 above range)

std_parallel (n=18, range 41129.3-61073.0 ns)
  41129.3 |
  42126.5 |#############
  43123.7 |
  44120.8 |
  45118.0 |#############
  46115.2 |
  47112.4 |#############
  48109.6 |########################################
  49106.8 |########################################
  50103.9 |##########################
  51101.1 |
  52098.3 |#############
  53095.5 |#############
  54092.7 |#############
  55089.9 |#############
  56087.1 |
  57084.2 |
  58081.4 |
  59078.6 |
  60075.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=102.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=103.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=104.0% of algo (FFI overhead may distort results)
- **nway**: bridge=101.7% of algo (FFI overhead may distort results)
- **sequential**: CV=62.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=101.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.9% of algo (FFI overhead may distort results)
