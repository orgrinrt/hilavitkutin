# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway is fastest but the noisiest (CV 93.8%)

kernel_nway wins on median (19.74 ms) yet has the highest variance (CV 93.8%), while std_parallel is the steadiest (CV 1.2%, 20.22 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (kernel_nway, nway) are a dead heat (<1%)

kernel_nway (19.74 ms) and nway (19.79 ms) differ by 0.26%, inside the noise, even though the wider field spreads 49.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.67)

kernel_nway's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (9.74 ms) is smaller than the fastest variant's own run-to-run std-dev (18.51 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {kernel_nway, nway, head_tail, std_parallel} vs {head_tail_dyn, sequential, kernel_seq} (34% apart)

The field splits into a fast tier {kernel_nway, nway, head_tail, std_parallel} and a slow tier {head_tail_dyn, sequential, kernel_seq} with a 34% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader kernel_nway vs stability leader std_parallel (+2% speed for 79.3x steadier)

kernel_nway is fastest (19.74 ms, CV 93.8%); std_parallel gives up 2.5% median for 79.3x lower variance (CV 1.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### kernel_nway is inconsistent: worst-20% is 2.3x its best-20%

kernel_nway's best 20% of batches run at 19.26 ms but its worst 20% at 44.54 ms (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 19738470.0 ns median (-0.4% vs baseline)
- Spread: 1.49x (fastest 19738470.0 ns, slowest 29478569.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 23439273ns | 19817697ns | 18820796ns | 19524461ns | 31620728ns | base |
| head_tail_dyn | 27244024ns | 27081223ns | 24507339ns | 26231668ns | 30130900ns | +16.23% |
| kernel_nway | 27898839ns | 19745137ns | 19267314ns | 19674115ns | 44551688ns | +19.03% |
| kernel_seq | 29064250ns | 29488203ns | 26233681ns | 29211777ns | 30258244ns | +24.00% |
| nway | 22344242ns | 19792281ns | 19159932ns | 19776087ns | 27788628ns | -4.67% |
| sequential | 28349917ns | 27980758ns | 26346164ns | 27448321ns | 30704188ns | +20.95% |
| std_parallel | 20293785ns | 20226366ns | 19925534ns | 20213216ns | 20598763ns | -13.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 23432835ns | 18816661ns | 31611891ns | base | 5.728 |
| head_tail_dyn | 27234353ns | 24505054ns | 30121708ns | +16.22% | 4.928 |
| kernel_nway | 27891693ns | 19261893ns | 44541363ns | +19.03% | 4.812 |
| kernel_seq | 29057649ns | 26232318ns | 30249904ns | +24.00% | 4.619 |
| nway | 22342522ns | 19158075ns | 27786872ns | -4.65% | 6.007 |
| sequential | 28348861ns | 26345037ns | 30703279ns | +20.98% | 4.735 |
| std_parallel | 20289713ns | 19920105ns | 20593449ns | -13.41% | 6.615 |

## Performance model

- Peak throughput: **7.133 Gops/s** (head_tail; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.775 | 95.0% |
| head_tail_dyn | 4.958 | 69.5% |
| kernel_nway | 6.800 | 95.3% |
| kernel_seq | 4.553 | 63.8% |
| nway | 6.782 | 95.1% |
| sequential | 4.797 | 67.3% |
| std_parallel | 6.637 | 93.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 23439273ns | 23439273ns | base |
| head_tail_dyn | 27244024ns | 27244024ns | +16.23% |
| kernel_nway | 27898839ns | 27898839ns | +19.03% |
| kernel_seq | 29064250ns | 29064250ns | +24.00% |
| nway | 22344242ns | 22344242ns | -4.67% |
| sequential | 28349917ns | 28349917ns | +20.95% |
| std_parallel | 20293785ns | 20293785ns | -13.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 19810081ns | base | --- | [18876532, 31611891] | --- | --- | --- | --- |
| head_tail_dyn | 27070027ns | no significant difference | [-4602073, +10311627]ns | [24511323, 30121708] | no | 0.4375 | 0.2188 | 0 |
| kernel_nway | 19738470ns | no significant difference | [-11561631, +24472344]ns | [19395244, 44541363] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 29478569ns | no significant difference | [-3862620, +10870140]ns | [27444474, 30249904] | no | 0.4375 | 0.2188 | 0 |
| nway | 19790697ns | no significant difference | [-11777453, +8866600]ns | [19449997, 27786872] | no | 1.0000 | 1.0000 | 0 |
| sequential | 27979647ns | no significant difference | [-5248233, +11826747]ns | [26363657, 30703279] | no | 0.4375 | 0.2188 | 0 |
| std_parallel | 20224151ns | no significant difference | [-11195964, +1175006]ns | [20051538, 20593449] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 19810081ns | +52.1% | -0.7% | +52.7% | -1.8% | +41.2% | +3.0% |
| 1 | 2 | 31611891ns | -14.4% | +40.0% | -12.2% | -37.3% | -16.6% | -35.4% |
| 1 | 3 | 18876532ns | +29.9% | +4.6% | +54.6% | +47.0% | +62.7% | +6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.664 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.002 | ok |
| kernel_nway | -0.667 | HIGH- (thermal bounce) |
| kernel_seq | -0.544 | HIGH- (thermal bounce) |
| nway | -0.144 | ok |
| sequential | -0.410 | moderate- |
| std_parallel | -0.187 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 23152067.0ns | 23432834.6ns | 98.8% | HIGH |
| head_tail_dyn | 27262669.7ns | 27234352.7ns | 100.1% | HIGH |
| kernel_nway | 32026601.8ns | 27891692.6ns | 114.8% | HIGH |
| kernel_seq | 29361458.3ns | 29057649.4ns | 101.0% | HIGH |
| nway | 22265060.1ns | 22342522.0ns | 99.7% | HIGH |
| sequential | 28352452.1ns | 28348861.2ns | 100.0% | HIGH |
| std_parallel | 21489621.8ns | 20289712.6ns | 105.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 18816660.9-31611890.7 ns)
  18816660.9 |########################################
  19456422.4 |########################################
  20096183.9 |
  20735945.4 |####################
  21375706.9 |
  22015468.4 |
  22655229.8 |
  23294991.3 |
  23934752.8 |
  24574514.3 |
  25214275.8 |
  25854037.3 |
  26493798.8 |
  27133560.3 |
  27773321.8 |
  28413083.2 |
  29052844.7 |
  29692606.2 |
  30332367.7 |
  30972129.2 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 24505053.7-30121708.4 ns)
  24505053.7 |########################################
  24785886.4 |
  25066719.2 |
  25347551.9 |
  25628384.6 |
  25909217.4 |
  26190050.1 |
  26470882.8 |
  26751715.6 |####################
  27032548.3 |####################
  27313381.0 |
  27594213.8 |
  27875046.5 |
  28155879.2 |
  28436712.0 |
  28717544.7 |
  28998377.4 |
  29279210.2 |
  29560042.9 |
  29840875.6 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 19261893.1-44541363.3 ns)
  19261893.1 |########################################
  20525866.6 |
  21789840.1 |
  23053813.6 |
  24317787.1 |
  25581760.6 |
  26845734.2 |
  28109707.7 |
  29373681.2 |
  30637654.7 |
  31901628.2 |
  33165601.7 |
  34429575.2 |
  35693548.7 |
  36957522.2 |
  38221495.8 |
  39485469.3 |
  40749442.8 |
  42013416.3 |
  43277389.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 26232318.4-30249904.3 ns)
  26232318.4 |########################################
  26433197.7 |
  26634077.0 |
  26834956.3 |
  27035835.6 |
  27236714.9 |
  27437594.2 |
  27638473.5 |
  27839352.8 |
  28040232.1 |
  28241111.3 |
  28441990.6 |
  28642869.9 |########################################
  28843749.2 |
  29044628.5 |
  29245507.8 |########################################
  29446387.1 |
  29647266.4 |########################################
  29848145.7 |########################################
  30049025.0 |
  (0 below, 1 above range)

nway (n=6, range 19158075.0-27786872.4 ns)
  19158075.0 |##########
  19589514.9 |########################################
  20020954.7 |
  20452394.6 |
  20883834.5 |
  21315274.3 |
  21746714.2 |
  22178154.1 |
  22609593.9 |
  23041033.8 |
  23472473.7 |
  23903913.5 |
  24335353.4 |
  24766793.3 |
  25198233.1 |
  25629673.0 |
  26061112.9 |
  26492552.7 |
  26923992.6 |
  27355432.5 |
  (0 below, 1 above range)

sequential (n=6, range 26345037.3-30703279.5 ns)
  26345037.3 |########################################
  26562949.4 |
  26780861.5 |
  26998773.6 |
  27216685.7 |
  27434597.8 |
  27652509.9 |
  27870422.1 |########################################
  28088334.2 |
  28306246.3 |
  28524158.4 |
  28742070.5 |
  28959982.6 |
  29177894.7 |
  29395806.8 |
  29613718.9 |
  29831631.0 |
  30049543.1 |
  30267455.2 |
  30485367.3 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 19920104.8-20593448.6 ns)
  19920104.8 |########################################
  19953772.0 |
  19987439.2 |
  20021106.4 |
  20054773.6 |
  20088440.8 |
  20122107.9 |
  20155775.1 |########################################
  20189442.3 |########################################
  20223109.5 |########################################
  20256776.7 |
  20290443.9 |
  20324111.1 |
  20357778.3 |
  20391445.5 |
  20425112.7 |
  20458779.8 |
  20492447.0 |
  20526114.2 |
  20559781.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=35.5% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=66.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.4% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=102.6% of algo (FFI overhead may distort results)
- **nway**: CV=26.8% (high variance, measurements may be unstable)
- **nway**: bridge=100.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.2% of algo (FFI overhead may distort results)
