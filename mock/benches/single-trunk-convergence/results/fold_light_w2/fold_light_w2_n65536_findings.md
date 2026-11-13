# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 27% faster than the next best (head_tail)

sequential (27.04 us) leads head_tail (34.44 us) by 27%, a clear separation rather than a photo finish. CV 21.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 47% (significant)

sequential is -16.07 us (47%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.5x slower than the field

std_parallel (67.07 us) is 2.5x the fastest (27.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### nway shows alternating (throttle bounce) (autocorr -0.67)

nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### sequential is inconsistent: worst-20% is 2.0x its best-20%

sequential's best 20% of batches run at 15.00 us but its worst 20% at 29.36 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 27037.6 ns median (-21.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.48x (fastest 27037.6 ns, slowest 67071.9 ns)

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
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 39582ns | 34873ns | 31064ns | 33801ns | 52512ns | base |
| head_tail_dyn | 39879ns | 40727ns | 34681ns | 39692ns | 42758ns | +0.75% |
| kernel_nway | 38708ns | 38859ns | 34989ns | 37889ns | 41795ns | -2.21% |
| kernel_seq | 37674ns | 37386ns | 32319ns | 36336ns | 42360ns | -4.82% |
| nway | 53055ns | 52590ns | 48741ns | 51736ns | 57190ns | +34.04% |
| sequential | 24602ns | 27410ns | 15278ns | 24255ns | 29783ns | -37.85% |
| std_parallel | 76742ns | 67540ns | 58975ns | 66286ns | 101309ns | +93.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 39035ns | 30502ns | 51873ns | base | 1.679 |
| head_tail_dyn | 39343ns | 33996ns | 42304ns | +0.79% | 1.666 |
| kernel_nway | 38181ns | 34424ns | 41267ns | -2.19% | 1.716 |
| kernel_seq | 37111ns | 31937ns | 41553ns | -4.93% | 1.766 |
| nway | 52661ns | 48326ns | 56786ns | +34.91% | 1.244 |
| sequential | 24238ns | 15000ns | 29363ns | -37.91% | 2.704 |
| std_parallel | 76127ns | 58429ns | 100511ns | +95.02% | 0.861 |

## Performance model

- Peak throughput: **4.369 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.903 | 43.6% |
| head_tail_dyn | 1.635 | 37.4% |
| kernel_nway | 1.713 | 39.2% |
| kernel_seq | 1.775 | 40.6% |
| nway | 1.255 | 28.7% |
| sequential | 2.424 | 55.5% |
| std_parallel | 0.977 | 22.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 39582ns | 39582ns | base |
| head_tail_dyn | 39879ns | 39879ns | +0.75% |
| kernel_nway | 38708ns | 38708ns | -2.21% |
| kernel_seq | 37674ns | 37674ns | -4.82% |
| nway | 53055ns | 53055ns | +34.04% |
| sequential | 24602ns | 24602ns | -37.85% |
| std_parallel | 76742ns | 76742ns | +93.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 34443ns | base | --- | [32037, 47094] | --- | --- | --- | --- |
| head_tail_dyn | 40090ns | no significant difference | [-4857, +7721]ns | [38691, 41370] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 38252ns | no significant difference | [-5900, +5735]ns | [36004, 39797] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 36924ns | no significant difference | [-10393, +5366]ns | [34419, 38151] | no | 0.4807 | 0.4807 | 0 |
| nway | 52213ns | +18421.6ns (+53.5%) | [+4304, +21815]ns | [50424, 54812] | YES | 0.0026 | 0.0013 | 0 |
| sequential | 27038ns | -16073.9ns (-46.7%) | [-19420, -6023]ns | [18821, 27840] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 67072ns | +31150.6ns (+90.4%) | [+26914, +33010]ns | [61734, 77543] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 49876ns | -17.1% | -27.7% | -33.4% | +2.0% | -43.3% | +53.3% |
| 1 | 2 | 34443ns | +2.5% | +15.7% | +5.7% | +65.4% | -54.8% | +95.8% |
| 1 | 3 | 30968ns | +28.0% | +19.8% | +21.8% | +64.9% | -12.7% | +103.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.078 | ok |
| head_tail_dyn | -0.616 | HIGH- (thermal bounce) |
| kernel_nway | -0.620 | HIGH- (thermal bounce) |
| kernel_seq | -0.037 | ok |
| nway | -0.666 | HIGH- (thermal bounce) |
| sequential | -0.661 | HIGH- (thermal bounce) |
| std_parallel | -0.023 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 39393.0ns | 39034.8ns | 100.9% | HIGH |
| head_tail_dyn | 40945.8ns | 39343.1ns | 104.1% | HIGH |
| kernel_nway | 38735.8ns | 38180.8ns | 101.5% | HIGH |
| kernel_seq | 36898.9ns | 37110.8ns | 99.4% | HIGH |
| nway | 54064.1ns | 52660.8ns | 102.7% | HIGH |
| sequential | 24280.7ns | 24238.3ns | 100.2% | HIGH |
| std_parallel | 70778.2ns | 76126.5ns | 93.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 30502.2-51873.0 ns)
  30502.2 |########################################
  31570.7 |##########################
  32639.3 |##########################
  33707.8 |########################################
  34776.4 |#############
  35844.9 |
  36913.4 |
  37982.0 |
  39050.5 |
  40119.1 |
  41187.6 |
  42256.1 |
  43324.7 |
  44393.2 |
  45461.8 |
  46530.3 |##########################
  47598.9 |#############
  48667.4 |
  49735.9 |
  50804.5 |#############
  (1 below, 2 above range)

head_tail_dyn (n=18, range 33995.7-42303.9 ns)
  33995.7 |#############
  34411.1 |
  34826.5 |#############
  35242.0 |
  35657.4 |#############
  36072.8 |
  36488.2 |
  36903.6 |
  37319.0 |
  37734.4 |
  38149.8 |#############
  38565.2 |##########################
  38980.6 |#############
  39396.0 |
  39811.4 |##########################
  40226.9 |##########################
  40642.3 |
  41057.7 |#############
  41473.1 |########################################
  41888.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 34424.2-41267.0 ns)
  34424.2 |########################################
  34766.3 |
  35108.5 |
  35450.6 |####################
  35792.8 |
  36134.9 |####################
  36477.0 |
  36819.2 |
  37161.3 |####################
  37503.5 |
  37845.6 |########################################
  38187.7 |####################
  38529.9 |####################
  38872.0 |####################
  39214.2 |
  39556.3 |####################
  39898.4 |####################
  40240.6 |####################
  40582.7 |####################
  40924.9 |####################
  (2 below, 1 above range)

kernel_seq (n=18, range 31937.3-41552.9 ns)
  31937.3 |
  32418.1 |####################
  32898.9 |
  33379.6 |####################
  33860.4 |####################
  34341.2 |
  34822.0 |####################
  35302.8 |########################################
  35783.5 |
  36264.3 |####################
  36745.1 |
  37225.9 |########################################
  37706.6 |########################################
  38187.4 |########################################
  38668.2 |
  39149.0 |
  39629.8 |
  40110.5 |
  40591.3 |
  41072.1 |
  (2 below, 3 above range)

nway (n=18, range 48326.0-56786.2 ns)
  48326.0 |
  48749.0 |#############
  49172.0 |#############
  49595.0 |#############
  50018.0 |#############
  50441.1 |##########################
  50864.1 |#############
  51287.1 |
  51710.1 |#############
  52133.1 |#############
  52556.1 |########################################
  52979.1 |
  53402.1 |
  53825.2 |
  54248.2 |
  54671.2 |
  55094.2 |
  55517.2 |
  55940.2 |
  56363.2 |#############
  (1 below, 4 above range)

sequential (n=18, range 15000.3-29363.4 ns)
  15000.3 |################
  15718.5 |
  16436.6 |
  17154.8 |########
  17873.0 |
  18591.1 |
  19309.3 |########
  20027.4 |
  20745.6 |
  21463.7 |
  22181.9 |
  22900.0 |
  23618.2 |
  24336.3 |
  25054.5 |
  25772.7 |
  26490.8 |########################################
  27209.0 |################
  27927.1 |################
  28645.3 |########
  (2 below, 2 above range)

std_parallel (n=18, range 58429.2-100510.8 ns)
  58429.2 |##########
  60533.3 |########################################
  62637.4 |##########
  64741.4 |####################
  66845.5 |####################
  68949.6 |
  71053.7 |
  73157.8 |
  75261.9 |####################
  77365.9 |##########
  79470.0 |####################
  81574.1 |##########
  83678.2 |
  85782.3 |
  87886.3 |
  89990.4 |
  92094.5 |
  94198.6 |
  96302.7 |
  98406.8 |
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=24.9% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=96.6% of algo (FFI overhead may distort results)
- **nway**: bridge=101.0% of algo (FFI overhead may distort results)
- **sequential**: CV=24.0% (high variance, measurements may be unstable)
- **sequential**: bridge=101.7% of algo (FFI overhead may distort results)
- **std_parallel**: CV=43.0% (high variance, measurements may be unstable)
- **std_parallel**: bridge=101.9% of algo (FFI overhead may distort results)
