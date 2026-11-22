# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is fastest but the noisiest (CV 17.4%)

nway wins on median (11.41 ms) yet has the highest variance (CV 17.4%), while head_tail_dyn is the steadiest (CV 5.1%, 17.31 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### nway is inconsistent: worst-20% is 1.5x its best-20%

nway's best 20% of batches run at 8.80 ms but its worst 20% at 13.34 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: nway** at 11409833.3 ns median (-7.5% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.52x (fastest 11409833.3 ns, slowest 17313864.6 ns)

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
| head_tail | 11858696ns | 12338188ns | 9284677ns | 11364378ns | 13887182ns | base |
| head_tail_dyn | 17201872ns | 17317068ns | 16056469ns | 16928399ns | 18184781ns | +45.06% |
| kernel_nway | 11101863ns | 11537667ns | 9229479ns | 10796163ns | 12496604ns | -6.38% |
| kernel_seq | 14143222ns | 14589646ns | 12357396ns | 13944288ns | 15334536ns | +19.26% |
| nway | 11207080ns | 11413990ns | 8797521ns | 10586809ns | 13342266ns | -5.49% |
| sequential | 13941606ns | 14427172ns | 12069406ns | 13669750ns | 15285490ns | +17.56% |
| std_parallel | 11330226ns | 12201646ns | 8859906ns | 11146420ns | 12841094ns | -4.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 11854602ns | 9282083ns | 13883479ns | base | 11.322 |
| head_tail_dyn | 17198080ns | 16052521ns | 18180750ns | +45.08% | 7.804 |
| kernel_nway | 11098014ns | 9224635ns | 12492422ns | -6.38% | 12.094 |
| kernel_seq | 14140526ns | 12355000ns | 15331703ns | +19.28% | 9.492 |
| nway | 11203227ns | 8795490ns | 13337865ns | -5.49% | 11.980 |
| sequential | 13938602ns | 12067604ns | 15281620ns | +17.58% | 9.629 |
| std_parallel | 11327823ns | 8858344ns | 12838885ns | -4.44% | 11.849 |

## Performance model

- Peak throughput: **15.260 Gops/s** (nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.883 | 71.3% |
| head_tail_dyn | 7.752 | 50.8% |
| kernel_nway | 11.637 | 76.3% |
| kernel_seq | 9.201 | 60.3% |
| nway | 11.763 | 77.1% |
| sequential | 9.305 | 61.0% |
| std_parallel | 11.003 | 72.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 11858696ns | 11858696ns | base |
| head_tail_dyn | 17201872ns | 17201872ns | +45.06% |
| kernel_nway | 11101863ns | 11101863ns | -6.38% |
| kernel_seq | 14143222ns | 14143222ns | +19.26% |
| nway | 11207080ns | 11207080ns | -5.49% |
| sequential | 13941606ns | 13941606ns | +17.56% |
| std_parallel | 11330226ns | 11330226ns | -4.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12332922ns | base | --- | [9347406, 13883479] | --- | --- | --- | --- |
| head_tail_dyn | 17313865ns | +5392880.2ns (+43.7%) | [+3299255, +7338297]ns | [16099625, 18180750] | YES | 0.0469 | 0.0313 | 0 |
| kernel_nway | 11533490ns | no significant difference | [-2016214, +134932]ns | [9268130, 12492422] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 14586630ns | +2543833.4ns (+20.6%) | [+1158099, +3155839]ns | [12503245, 15331703] | YES | 0.0469 | 0.0313 | 0 |
| nway | 11409833ns | -485421.9ns (-3.9%) | [-1378036, -90667]ns | [8861984, 13337865] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 14423745ns | +2566041.7ns (+20.8%) | [+819802, +2866156]ns | [12110443, 15281620] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| std_parallel | 12198609ns | -401432.2ns (-3.3%) | [-1103990, -74917]ns | [8945974, 12838885] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12612458ns | +37.1% | -6.7% | +17.7% | -5.8% | +21.2% | -3.3% |
| 1 | 2 | 13603943ns | +29.5% | -9.9% | +10.8% | -5.4% | +6.0% | -5.6% |
| 1 | 3 | 9347406ns | +78.5% | -0.8% | +33.8% | -5.2% | +29.6% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.308 | moderate- |
| head_tail_dyn | -0.389 | moderate- |
| kernel_nway | -0.260 | moderate- |
| kernel_seq | -0.213 | moderate- |
| nway | -0.318 | moderate- |
| sequential | -0.044 | ok |
| std_parallel | -0.262 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 10800861.1ns | 11854602.4ns | 91.1% | HIGH |
| head_tail_dyn | 17309435.8ns | 17198079.9ns | 100.6% | HIGH |
| kernel_nway | 11421427.1ns | 11098013.9ns | 102.9% | HIGH |
| kernel_seq | 13809885.4ns | 14140526.0ns | 97.7% | HIGH |
| nway | 11168546.9ns | 11203227.5ns | 99.7% | HIGH |
| sequential | 13879572.9ns | 13938602.4ns | 99.6% | HIGH |
| std_parallel | 10922854.2ns | 11327822.9ns | 96.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 9282083.3-13883479.1 ns)
  9282083.3 |########################################
  9512153.1 |
  9742222.9 |
  9972292.7 |
  10202362.5 |
  10432432.3 |
  10662502.1 |
  10892571.8 |
  11122641.6 |
  11352711.4 |
  11582781.2 |
  11812851.0 |
  12042920.8 |####################
  12272990.6 |
  12503060.4 |####################
  12733130.2 |
  12963200.0 |####################
  13193269.8 |
  13423339.6 |
  13653409.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 16052520.8-18180750.0 ns)
  16052520.8 |########################################
  16158932.3 |
  16265343.7 |
  16371755.2 |
  16478166.6 |
  16584578.1 |
  16690989.6 |
  16797401.0 |
  16903812.5 |
  17010223.9 |
  17116635.4 |
  17223046.9 |####################
  17329458.3 |####################
  17435869.8 |
  17542281.2 |
  17648692.7 |
  17755104.2 |####################
  17861515.6 |
  17967927.1 |
  18074338.5 |
  (0 below, 1 above range)

kernel_nway (n=6, range 9224635.4-12492421.9 ns)
  9224635.4 |########################################
  9388024.7 |
  9551414.0 |
  9714803.4 |
  9878192.7 |
  10041582.0 |
  10204971.3 |
  10368360.7 |
  10531750.0 |
  10695139.3 |
  10858528.6 |
  11021917.9 |####################
  11185307.3 |
  11348696.6 |
  11512085.9 |
  11675475.2 |
  11838864.6 |####################
  12002253.9 |
  12165643.2 |
  12329032.5 |####################
  (0 below, 1 above range)

kernel_seq (n=6, range 12355000.0-15331703.1 ns)
  12355000.0 |########################################
  12503835.2 |########################################
  12652670.3 |
  12801505.5 |
  12950340.6 |
  13099175.8 |
  13248010.9 |
  13396846.1 |
  13545681.2 |
  13694516.4 |
  13843351.6 |
  13992186.7 |
  14141021.9 |
  14289857.0 |########################################
  14438692.2 |
  14587527.3 |
  14736362.5 |########################################
  14885197.6 |
  15034032.8 |
  15182867.9 |########################################
  (0 below, 1 above range)

nway (n=6, range 8795489.6-13337864.6 ns)
  8795489.6 |########################################
  9022608.3 |
  9249727.1 |
  9476845.8 |
  9703964.6 |
  9931083.3 |
  10158202.1 |
  10385320.8 |
  10612439.6 |
  10839558.3 |
  11066677.1 |####################
  11293795.8 |
  11520914.6 |####################
  11748033.3 |
  11975152.1 |####################
  12202270.8 |
  12429389.6 |
  12656508.3 |
  12883627.1 |
  13110745.8 |
  (0 below, 1 above range)

sequential (n=6, range 12067604.2-15281619.8 ns)
  12067604.2 |########################################
  12228305.0 |
  12389005.8 |
  12549706.5 |
  12710407.3 |
  12871108.1 |
  13031808.9 |
  13192509.7 |
  13353210.4 |
  13513911.2 |
  13674612.0 |
  13835312.8 |
  13996013.6 |
  14156714.3 |
  14317415.1 |####################
  14478115.9 |####################
  14638816.7 |
  14799517.5 |
  14960218.2 |####################
  15120919.0 |
  (0 below, 1 above range)

std_parallel (n=6, range 8858343.8-12838885.4 ns)
  8858343.8 |########################################
  9057370.9 |
  9256398.0 |
  9455425.0 |
  9654452.1 |
  9853479.2 |
  10052506.3 |
  10251533.4 |
  10450560.5 |
  10649587.5 |
  10848614.6 |
  11047641.7 |
  11246668.8 |
  11445695.9 |
  11644723.0 |
  11843750.0 |####################
  12042777.1 |
  12241804.2 |####################
  12440831.3 |####################
  12639858.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=89.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=95.9% of algo (FFI overhead may distort results)
- **nway**: bridge=103.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=97.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=93.0% of algo (FFI overhead may distort results)
