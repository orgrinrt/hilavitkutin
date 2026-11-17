# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail is fastest but the noisiest (CV 8.7%)

head_tail wins on median (8.81 ms) yet has the highest variance (CV 8.7%), while nway is the steadiest (CV 0.2%, 8.89 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (head_tail, nway) are a dead heat (<1%)

head_tail (8.81 ms) and nway (8.89 ms) differ by 0.92%, inside the noise, even though the wider field spreads 79.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### std_parallel shows alternating (throttle bounce) (autocorr -0.67)

std_parallel's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (8.81 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader head_tail vs stability leader nway (+1% speed for 35.1x steadier)

head_tail is fastest (8.81 ms, CV 8.7%); nway gives up 0.9% median for 35.1x lower variance (CV 0.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (head_tail) is the fastest** at 8809552.1 ns median
- 3 variants significantly slower than baseline
- Spread: 1.80x (fastest 8809552.1 ns, slowest 15820927.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9184108ns | 8812828ns | 8619740ns | 8759444ns | 10103286ns | base |
| head_tail_dyn | 15895592ns | 15823729ns | 15657229ns | 15808035ns | 16146109ns | +73.08% |
| kernel_nway | 9232286ns | 8944682ns | 8848365ns | 8930840ns | 9876417ns | +0.52% |
| kernel_seq | 11840465ns | 11782995ns | 11571042ns | 11714892ns | 12163536ns | +28.92% |
| nway | 8886019ns | 8893938ns | 8837688ns | 8890729ns | 8903120ns | -3.25% |
| sequential | 12386278ns | 12457995ns | 11607573ns | 12267035ns | 12954495ns | +34.87% |
| std_parallel | 9296670ns | 8893984ns | 8882885ns | 8890507ns | 10112807ns | +1.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9180462ns | 8616146ns | 10098917ns | base | 14.620 |
| head_tail_dyn | 15892615ns | 15654552ns | 16142542ns | +73.11% | 8.445 |
| kernel_nway | 9228823ns | 8844719ns | 9872656ns | +0.53% | 14.543 |
| kernel_seq | 11838516ns | 11569583ns | 12161542ns | +28.95% | 11.337 |
| nway | 8882384ns | 8834948ns | 8898135ns | -3.25% | 15.111 |
| sequential | 12384464ns | 11605896ns | 12952500ns | +34.90% | 10.838 |
| std_parallel | 9294318ns | 8880656ns | 10110036ns | +1.24% | 14.441 |

## Performance model

- Peak throughput: **15.577 Gops/s** (head_tail; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 15.235 | 97.8% |
| head_tail_dyn | 8.484 | 54.5% |
| kernel_nway | 15.011 | 96.4% |
| kernel_seq | 11.393 | 73.1% |
| nway | 15.096 | 96.9% |
| sequential | 10.775 | 69.2% |
| std_parallel | 15.095 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9184108ns | 9184108ns | base |
| head_tail_dyn | 15895592ns | 15895592ns | +73.08% |
| kernel_nway | 9232286ns | 9232286ns | +0.52% |
| kernel_seq | 11840465ns | 11840465ns | +28.92% |
| nway | 8886019ns | 8886019ns | -3.25% |
| sequential | 12386278ns | 12386278ns | +34.87% |
| std_parallel | 9296670ns | 9296670ns | +1.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8809552ns | base | --- | [8632917, 10098917] | --- | --- | --- | --- |
| head_tail_dyn | 15820927ns | +6941000.0ns (+78.8%) | [+5746411, +7449047]ns | [15714375, 16142542] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_nway | 8941583ns | no significant difference | [-367792, +308667]ns | [8872229, 9872656] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 11780865ns | +2944849.0ns (+33.4%) | [+1736630, +3292682]ns | [11573141, 12161542] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 8890984ns | no significant difference | [-1208245, +261833]ns | [8858031, 8898135] | no | 1.0000 | 1.0000 | 0 |
| sequential | 12456271ns | +3267968.7ns (+37.1%) | [+2024453, +4319583]ns | [11744620, 12952500] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 8891818ns | no significant difference | [-674083, +777557]ns | [8881099, 10110036] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8808349ns | +78.8% | +0.9% | +35.2% | +0.6% | +41.9% | +0.8% |
| 1 | 2 | 10098917ns | +56.9% | -2.2% | +17.2% | -12.0% | +20.5% | +0.1% |
| 1 | 3 | 8634120ns | +86.3% | +3.4% | +36.4% | +3.0% | +44.6% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.659 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.038 | ok |
| kernel_nway | -0.666 | HIGH- (thermal bounce) |
| kernel_seq | -0.001 | ok |
| nway | -0.094 | ok |
| sequential | -0.666 | HIGH- (thermal bounce) |
| std_parallel | -0.667 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 9122467.0ns | 9180461.8ns | 99.4% | HIGH |
| head_tail_dyn | 15811821.2ns | 15892614.6ns | 99.5% | HIGH |
| kernel_nway | 9223711.8ns | 9228822.9ns | 99.9% | HIGH |
| kernel_seq | 11915256.9ns | 11838515.6ns | 100.6% | HIGH |
| nway | 8931046.9ns | 8882383.7ns | 100.5% | HIGH |
| sequential | 12518269.1ns | 12384463.5ns | 101.1% | HIGH |
| std_parallel | 9246263.9ns | 9294317.7ns | 99.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 8616145.8-10098916.7 ns)
  8616145.8 |########################################
  8690284.3 |
  8764422.9 |
  8838561.4 |
  8912700.0 |#############
  8986838.5 |
  9060977.1 |
  9135115.6 |
  9209254.2 |
  9283392.7 |
  9357531.2 |#############
  9431669.8 |
  9505808.3 |
  9579946.9 |
  9654085.4 |
  9728224.0 |
  9802362.5 |
  9876501.1 |
  9950639.6 |
  10024778.2 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 15654552.1-16142541.7 ns)
  15654552.1 |########################################
  15678951.6 |
  15703351.1 |
  15727750.5 |
  15752150.0 |########################################
  15776549.5 |########################################
  15800949.0 |
  15825348.4 |########################################
  15849747.9 |
  15874147.4 |########################################
  15898546.9 |
  15922946.4 |
  15947345.8 |
  15971745.3 |
  15996144.8 |
  16020544.3 |
  16044943.7 |
  16069343.2 |
  16093742.7 |
  16118142.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 8844718.8-9872656.2 ns)
  8844718.8 |####################
  8896115.7 |########################################
  8947512.5 |####################
  8998909.4 |
  9050306.3 |
  9101703.2 |
  9153100.0 |
  9204496.9 |
  9255893.8 |
  9307290.7 |
  9358687.5 |
  9410084.4 |
  9461481.3 |
  9512878.1 |
  9564275.0 |####################
  9615671.9 |
  9667068.8 |
  9718465.6 |
  9769862.5 |
  9821259.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 11569583.3-12161541.7 ns)
  11569583.3 |########################################
  11599181.2 |
  11628779.1 |
  11658377.1 |
  11687975.0 |
  11717572.9 |
  11747170.8 |
  11776768.7 |
  11806366.6 |
  11835964.6 |
  11865562.5 |
  11895160.4 |
  11924758.3 |
  11954356.2 |#############
  11983954.1 |
  12013552.1 |
  12043150.0 |
  12072747.9 |#############
  12102345.8 |
  12131943.7 |
  (0 below, 1 above range)

nway (n=6, range 8834947.9-8898135.4 ns)
  8834947.9 |####################
  8838107.3 |
  8841266.7 |
  8844426.0 |
  8847585.4 |
  8850744.8 |
  8853904.2 |
  8857063.5 |
  8860222.9 |
  8863382.3 |
  8866541.7 |
  8869701.1 |
  8872860.4 |
  8876019.8 |
  8879179.2 |####################
  8882338.6 |
  8885497.9 |####################
  8888657.3 |
  8891816.7 |
  8894976.1 |########################################
  (0 below, 1 above range)

sequential (n=6, range 11605895.8-12952500.0 ns)
  11605895.8 |####################
  11673226.0 |
  11740556.2 |
  11807886.4 |
  11875216.6 |####################
  11942546.9 |
  12009877.1 |
  12077207.3 |
  12144537.5 |
  12211867.7 |
  12279197.9 |
  12346528.1 |
  12413858.3 |########################################
  12481188.5 |####################
  12548518.7 |
  12615848.9 |
  12683179.2 |
  12750509.4 |
  12817839.6 |
  12885169.8 |
  (0 below, 1 above range)

std_parallel (n=6, range 8880656.2-10110036.4 ns)
  8880656.2 |########################################
  8942125.2 |
  9003594.2 |
  9065063.2 |
  9126532.2 |
  9188001.3 |
  9249470.3 |
  9310939.3 |
  9372408.3 |
  9433877.3 |
  9495346.3 |##########
  9556815.3 |
  9618284.3 |
  9679753.4 |
  9741222.4 |
  9802691.4 |
  9864160.4 |
  9925629.4 |
  9987098.4 |
  10048567.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: bridge=99.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.4% of algo (FFI overhead may distort results)
