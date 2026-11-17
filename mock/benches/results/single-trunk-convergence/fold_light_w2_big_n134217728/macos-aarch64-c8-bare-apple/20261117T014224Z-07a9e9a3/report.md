# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (std_parallel, head_tail) are a dead heat (<1%)

std_parallel (8.66 ms) and head_tail (8.68 ms) differ by 0.26%, inside the noise, even though the wider field spreads 78.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.65)

nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader std_parallel vs stability leader head_tail (+0% speed for 12.4x steadier)

std_parallel is fastest (8.66 ms, CV 11.8%); head_tail gives up 0.3% median for 12.4x lower variance (CV 1.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: std_parallel** at 8658755.2 ns median (-0.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.79x (fastest 8658755.2 ns, slowest 15470046.9 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 8698903ns | 8685182ns | 8615667ns | 8663024ns | 8794339ns | base |
| head_tail_dyn | 15558809ns | 15473411ns | 15334615ns | 15464493ns | 15812380ns | +78.86% |
| kernel_nway | 9404109ns | 8799302ns | 8555208ns | 8724580ns | 10847854ns | +8.11% |
| kernel_seq | 12490380ns | 12174438ns | 11546760ns | 11968521ns | 13744979ns | +43.59% |
| nway | 9621309ns | 9103068ns | 8604333ns | 8937163ns | 11156016ns | +10.60% |
| sequential | 11961872ns | 11552292ns | 11414281ns | 11523910ns | 12892609ns | +37.51% |
| std_parallel | 9299391ns | 8660828ns | 8626875ns | 8650976ns | 10608271ns | +6.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 8695345ns | 8612292ns | 8791052ns | base | 15.436 |
| head_tail_dyn | 15555851ns | 15332250ns | 15810021ns | +78.90% | 8.628 |
| kernel_nway | 9400898ns | 8553062ns | 10844333ns | +8.11% | 14.277 |
| kernel_seq | 12488097ns | 11545281ns | 13742115ns | +43.62% | 10.748 |
| nway | 9616977ns | 8598979ns | 11152229ns | +10.60% | 13.956 |
| sequential | 11959870ns | 11411958ns | 12890385ns | +37.54% | 11.222 |
| std_parallel | 9297462ns | 8625521ns | 10606026ns | +6.92% | 14.436 |

## Performance model

- Peak throughput: **15.692 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 15.461 | 98.5% |
| head_tail_dyn | 8.676 | 55.3% |
| kernel_nway | 15.259 | 97.2% |
| kernel_seq | 11.027 | 70.3% |
| nway | 14.751 | 94.0% |
| sequential | 11.620 | 74.0% |
| std_parallel | 15.501 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 8698903ns | 8698903ns | base |
| head_tail_dyn | 15558809ns | 15558809ns | +78.86% |
| kernel_nway | 9404109ns | 9404109ns | +8.11% |
| kernel_seq | 12490380ns | 12490380ns | +43.59% |
| nway | 9621309ns | 9621309ns | +10.60% |
| sequential | 11961872ns | 11961872ns | +37.51% |
| std_parallel | 9299391ns | 9299391ns | +6.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8681260ns | base | --- | [8613724, 8791052] | --- | --- | --- | --- |
| head_tail_dyn | 15470047ns | +6763099.0ns (+77.9%) | [+6623682, +7194734]ns | [15387484, 15810021] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_nway | 8795927ns | no significant difference | [-128484, +2146052]ns | [8562432, 10844333] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 12172271ns | +3465776.0ns (+39.9%) | [+2851625, +5060854]ns | [11549906, 13742115] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 9098995ns | no significant difference | [-14146, +2361177]ns | [8599708, 11152229] | no | 0.8250 | 0.6875 | 0 |
| sequential | 11550521ns | +2936666.7ns (+33.8%) | [+2664802, +4192104]ns | [11438703, 12890385] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 8658755ns | no significant difference | [-61536, +1832125]ns | [8627604, 10606026] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8698281ns | +80.0% | +24.7% | +32.8% | +17.8% | +48.2% | -0.5% |
| 1 | 2 | 8613854ns | +81.3% | -0.6% | +44.1% | -0.2% | +34.1% | +0.2% |
| 1 | 3 | 8773901ns | +75.5% | +0.3% | +53.9% | +14.0% | +30.4% | +20.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.518 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.094 | ok |
| kernel_nway | -0.223 | moderate- |
| kernel_seq | -0.003 | ok |
| nway | -0.654 | HIGH- (thermal bounce) |
| sequential | -0.128 | ok |
| std_parallel | -0.175 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 8713496.5ns | 8695345.5ns | 100.2% | HIGH |
| head_tail_dyn | 15721309.0ns | 15555850.7ns | 101.1% | HIGH |
| kernel_nway | 9213560.8ns | 9400897.6ns | 98.0% | HIGH |
| kernel_seq | 12839380.2ns | 12488097.2ns | 102.8% | HIGH |
| nway | 9480333.3ns | 9616977.4ns | 98.6% | HIGH |
| sequential | 12162515.6ns | 11959869.8ns | 101.7% | HIGH |
| std_parallel | 9602909.7ns | 9297461.8ns | 103.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 8612291.7-8791052.1 ns)
  8612291.7 |########################################
  8621229.7 |
  8630167.7 |
  8639105.8 |
  8648043.8 |
  8656981.8 |
  8665919.8 |
  8674857.8 |
  8683795.8 |
  8692733.9 |
  8701671.9 |
  8710609.9 |
  8719547.9 |
  8728485.9 |
  8737423.9 |
  8746362.0 |#############
  8755300.0 |
  8764238.0 |
  8773176.0 |#############
  8782114.0 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 15332250.0-15810020.8 ns)
  15332250.0 |########################################
  15356138.5 |
  15380027.1 |
  15403915.6 |
  15427804.2 |########################################
  15451692.7 |########################################
  15475581.2 |########################################
  15499469.8 |
  15523358.3 |
  15547246.9 |
  15571135.4 |
  15595023.9 |
  15618912.5 |
  15642801.0 |
  15666689.6 |
  15690578.1 |
  15714466.6 |
  15738355.2 |
  15762243.7 |
  15786132.3 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 8553062.5-10844333.3 ns)
  8553062.5 |########################################
  8667626.0 |
  8782189.6 |
  8896753.1 |#############
  9011316.7 |
  9125880.2 |
  9240443.8 |
  9355007.3 |
  9469570.8 |
  9584134.4 |
  9698697.9 |#############
  9813261.5 |
  9927825.0 |
  10042388.6 |
  10156952.1 |
  10271515.6 |
  10386079.2 |
  10500642.7 |
  10615206.3 |
  10729769.8 |
  (0 below, 1 above range)

kernel_seq (n=6, range 11545281.2-13742114.6 ns)
  11545281.2 |########################################
  11655122.9 |
  11764964.5 |
  11874806.2 |
  11984647.9 |
  12094489.5 |
  12204331.2 |
  12314172.9 |
  12424014.6 |
  12533856.2 |
  12643697.9 |#############
  12753539.6 |
  12863381.2 |
  12973222.9 |
  13083064.6 |#############
  13192906.2 |
  13302747.9 |
  13412589.6 |
  13522431.3 |
  13632272.9 |
  (0 below, 1 above range)

nway (n=6, range 8598979.2-11152229.2 ns)
  8598979.2 |########################################
  8726641.7 |
  8854304.2 |####################
  8981966.7 |
  9109629.2 |
  9237291.7 |####################
  9364954.2 |
  9492616.7 |
  9620279.2 |
  9747941.7 |
  9875604.2 |
  10003266.7 |
  10130929.2 |
  10258591.7 |
  10386254.2 |
  10513916.7 |
  10641579.2 |
  10769241.7 |
  10896904.2 |
  11024566.7 |####################
  (0 below, 1 above range)

sequential (n=6, range 11411958.3-12890385.4 ns)
  11411958.3 |########################################
  11485879.7 |####################
  11559801.0 |####################
  11633722.4 |
  11707643.7 |
  11781565.1 |
  11855486.4 |
  11929407.8 |
  12003329.2 |
  12077250.5 |
  12151171.9 |
  12225093.2 |
  12299014.6 |
  12372935.9 |
  12446857.3 |####################
  12520778.7 |
  12594700.0 |
  12668621.4 |
  12742542.7 |
  12816464.1 |
  (0 below, 1 above range)

std_parallel (n=6, range 8625520.8-10606026.0 ns)
  8625520.8 |########################################
  8724546.1 |
  8823571.3 |
  8922596.6 |
  9021621.8 |
  9120647.1 |
  9219672.4 |
  9318697.6 |
  9417722.9 |
  9516748.1 |
  9615773.4 |
  9714798.7 |
  9813823.9 |##########
  9912849.2 |
  10011874.4 |
  10110899.7 |
  10209925.0 |
  10308950.2 |
  10407975.5 |
  10507000.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=99.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=98.6% of algo (FFI overhead may distort results)
- **nway**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.2% of algo (FFI overhead may distort results)
