# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq is an outlier: 4.7x slower than the field

kernel_seq (10.97 ms) is 4.7x the fastest (2.32 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail is fastest but the noisiest (CV 40.7%)

head_tail wins on median (2.32 ms) yet has the highest variance (CV 40.7%), while sequential is the steadiest (CV 3.6%, 10.72 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (2.32 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.7x the fastest

Fastest head_tail (2.32 ms) to slowest kernel_seq (10.97 ms): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_seq is inconsistent: worst-20% is 3.3x its best-20%

kernel_seq's best 20% of batches run at 3.52 ms but its worst 20% at 11.50 ms (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 2323484.4 ns median
- 4 variants significantly slower than baseline
- Spread: 4.72x (fastest 2323484.4 ns, slowest 10965189.1 ns)

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
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 2680112ns | 2325547ns | 1930191ns | 2200648ns | 3774269ns | base |
| head_tail_dyn | 5716803ns | 5920586ns | 4471486ns | 5453371ns | 6734609ns | +113.30% |
| kernel_nway | 2576051ns | 2374965ns | 1835547ns | 2286585ns | 3380500ns | -3.88% |
| kernel_seq | 8666084ns | 10970028ns | 3522978ns | 8488739ns | 11503656ns | +223.35% |
| nway | 2558475ns | 2427472ns | 1689287ns | 2185888ns | 3551949ns | -4.54% |
| sequential | 10720641ns | 10727058ns | 10146785ns | 10628135ns | 11146328ns | +300.01% |
| std_parallel | 4891494ns | 5209008ns | 2525726ns | 4952847ns | 5982348ns | +82.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 2676717ns | 1929118ns | 3768401ns | base | 6.268 |
| head_tail_dyn | 5709895ns | 4459749ns | 6725326ns | +113.32% | 2.938 |
| kernel_nway | 2571922ns | 1833654ns | 3374696ns | -3.92% | 6.523 |
| kernel_seq | 8661175ns | 3519601ns | 11496062ns | +223.57% | 1.937 |
| nway | 2556433ns | 1687952ns | 3548818ns | -4.49% | 6.563 |
| sequential | 10716815ns | 10143256ns | 11141986ns | +300.37% | 1.566 |
| std_parallel | 4888174ns | 2523997ns | 5977547ns | +82.62% | 3.432 |

## Performance model

- Peak throughput: **9.939 Gops/s** (nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.221 | 72.6% |
| head_tail_dyn | 2.835 | 28.5% |
| kernel_nway | 7.077 | 71.2% |
| kernel_seq | 1.530 | 15.4% |
| nway | 6.917 | 69.6% |
| sequential | 1.565 | 15.7% |
| std_parallel | 3.222 | 32.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 2680112ns | 2680112ns | base |
| head_tail_dyn | 5716803ns | 5716803ns | +113.30% |
| kernel_nway | 2576051ns | 2576051ns | -3.88% |
| kernel_seq | 8666084ns | 8666084ns | +223.35% |
| nway | 2558475ns | 2558475ns | -4.54% |
| sequential | 10720641ns | 10720641ns | +300.01% |
| std_parallel | 4891494ns | 4891494ns | +82.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 2323484ns | base | --- | [1938265, 3768401] | --- | --- | --- | --- |
| head_tail_dyn | 5917588ns | +2537482.6ns (+109.2%) | [+2260079, +4301972]ns | [4486769, 6725326] | YES | 0.0469 | 0.0313 | 0 |
| kernel_nway | 2370783ns | no significant difference | [-747274, +432517]ns | [1970287, 3374696] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 10965189ns | +7727661.1ns (+332.6%) | [+1538938, +8686776]ns | [3522275, 11496062] | YES | 0.0469 | 0.0313 | 0 |
| nway | 2425537ns | no significant difference | [-965666, +893207]ns | [1694943, 3548818] | no | 0.8250 | 0.6875 | 0 |
| sequential | 10723433ns | +8301689.4ns (+357.3%) | [+7036825, +8781781]ns | [10285026, 11141986] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 5206481ns | +2679868.2ns (+115.3%) | [+987492, +2967011]ns | [3480494, 5977547] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 3624504ns | +63.3% | -20.6% | +213.3% | -26.6% | +207.4% | +59.9% |
| 1 | 2 | 2422310ns | +177.6% | +11.8% | +358.5% | +36.9% | +342.7% | +122.5% |
| 1 | 3 | 1983337ns | +126.2% | +7.5% | +77.6% | -14.5% | +418.6% | +75.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.045 | ok |
| head_tail_dyn | -0.401 | moderate- |
| kernel_nway | -0.060 | ok |
| kernel_seq | -0.151 | ok |
| nway | -0.434 | moderate- |
| sequential | -0.000 | ok |
| std_parallel | -0.082 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 2869719.4ns | 2676716.8ns | 107.2% | HIGH |
| head_tail_dyn | 5807835.8ns | 5709894.5ns | 101.7% | HIGH |
| kernel_nway | 2781303.6ns | 2571921.8ns | 108.1% | HIGH |
| kernel_seq | 8725453.3ns | 8661175.2ns | 100.7% | HIGH |
| nway | 2801558.6ns | 2556432.6ns | 109.6% | HIGH |
| sequential | 10543557.2ns | 10716815.2ns | 98.4% | HIGH |
| std_parallel | 5007625.0ns | 4888173.9ns | 102.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1929118.4-3768400.6 ns)
  1929118.4 |########################################
  2021082.5 |
  2113046.6 |
  2205010.7 |
  2296974.8 |
  2388938.9 |
  2480903.1 |
  2572867.2 |#############
  2664831.3 |
  2756795.4 |
  2848759.5 |#############
  2940723.6 |
  3032687.7 |
  3124651.8 |
  3216615.9 |
  3308580.0 |
  3400544.2 |
  3492508.3 |
  3584472.4 |
  3676436.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 4459748.8-6725325.8 ns)
  4459748.8 |########################################
  4573027.6 |
  4686306.5 |
  4799585.3 |
  4912864.2 |
  5026143.0 |####################
  5139421.9 |
  5252700.8 |
  5365979.6 |
  5479258.5 |
  5592537.3 |
  5705816.2 |
  5819095.0 |
  5932373.9 |
  6045652.7 |
  6158931.6 |
  6272210.4 |
  6385489.3 |
  6498768.1 |
  6612047.0 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 1833653.7-3374695.9 ns)
  1833653.7 |########################################
  1910705.8 |
  1987757.9 |
  2064810.0 |########################################
  2141862.1 |
  2218914.2 |
  2295966.4 |########################################
  2373018.5 |########################################
  2450070.6 |
  2527122.7 |
  2604174.8 |
  2681226.9 |
  2758279.0 |
  2835331.1 |
  2912383.2 |
  2989435.3 |
  3066487.5 |########################################
  3143539.6 |
  3220591.7 |
  3297643.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 3519600.8-11496061.7 ns)
  3519600.8 |########################################
  3918423.8 |
  4317246.9 |
  4716069.9 |
  5114893.0 |
  5513716.0 |
  5912539.1 |
  6311362.1 |
  6710185.2 |
  7109008.2 |
  7507831.2 |
  7906654.3 |
  8305477.3 |
  8704300.4 |
  9103123.4 |
  9501946.5 |
  9900769.5 |
  10299592.6 |
  10698415.6 |########################################
  11097238.7 |####################
  (0 below, 1 above range)

nway (n=6, range 1687951.6-3548817.5 ns)
  1687951.6 |########################################
  1780994.9 |
  1874038.2 |
  1967081.5 |
  2060124.8 |
  2153168.1 |####################
  2246211.4 |
  2339254.7 |
  2432298.0 |
  2525341.3 |
  2618384.6 |####################
  2711427.9 |
  2804471.2 |
  2897514.5 |
  2990557.8 |
  3083601.1 |####################
  3176644.4 |
  3269687.7 |
  3362731.0 |
  3455774.3 |
  (0 below, 1 above range)

sequential (n=6, range 10143256.1-11141986.3 ns)
  10143256.1 |####################
  10193192.6 |
  10243129.1 |
  10293065.6 |
  10343002.1 |
  10392938.7 |####################
  10442875.2 |
  10492811.7 |
  10542748.2 |
  10592684.7 |
  10642621.2 |
  10692557.7 |########################################
  10742494.2 |
  10792430.7 |
  10842367.2 |####################
  10892303.8 |
  10942240.3 |
  10992176.8 |
  11042113.3 |
  11092049.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 2523997.2-5977547.1 ns)
  2523997.2 |########################################
  2696674.7 |
  2869352.2 |
  3042029.7 |
  3214707.2 |
  3387384.7 |
  3560062.2 |
  3732739.7 |
  3905417.2 |
  4078094.7 |
  4250772.2 |
  4423449.6 |########################################
  4596127.1 |
  4768804.6 |########################################
  4941482.1 |
  5114159.6 |
  5286837.1 |
  5459514.6 |########################################
  5632192.1 |
  5804869.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=35.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=108.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=24.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=108.6% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=42.0% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **kernel_seq**: bridge=100.2% of algo (FFI overhead may distort results)
- **nway**: CV=31.8% (high variance, measurements may be unstable)
- **nway**: bridge=101.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.9% of algo (FFI overhead may distort results)
- **std_parallel**: CV=24.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=107.6% of algo (FFI overhead may distort results)
