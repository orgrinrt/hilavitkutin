# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_iter is fastest but the noisiest (CV 7.1%)

bc_iter wins on median (375 ns) yet has the highest variance (CV 7.1%), while bc_index is the steadiest (CV 6.4%, 379 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (bc_iter, bc_index) are a dead heat (<1%)

bc_iter (375 ns) and bc_index (379 ns) differ by 0.95%, inside the noise, even though the wider field spreads 79.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader bc_iter vs stability leader bc_index (+1% speed for 1.1x steadier)

bc_iter is fastest (375 ns, CV 7.1%); bc_index gives up 0.9% median for 1.1x lower variance (CV 6.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bc_iter** at 375.4 ns median (-0.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.79x (fastest 375.4 ns, slowest 672.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2640ns | 2592ns | 2450ns | 2649ns | 2803ns | base |
| bc_iter | 2610ns | 2598ns | 2413ns | 2607ns | 2817ns | -1.13% |
| bc_unchecked | 2948ns | 2931ns | 2702ns | 2965ns | 3143ns | +11.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 383ns | 353ns | 413ns | base | 42.754 |
| bc_iter | 379ns | 348ns | 414ns | -1.22% | 43.282 |
| bc_unchecked | 679ns | 621ns | 731ns | +77.17% | 24.132 |

## Performance model

- Peak throughput: **47.135 Gops/s** (bc_iter; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 43.230 | 91.7% |
| bc_iter | 43.638 | 92.6% |
| bc_unchecked | 24.356 | 51.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2640ns | 2640ns | base |
| bc_iter | 2610ns | 2610ns | -1.13% |
| bc_unchecked | 2948ns | 2948ns | +11.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 379ns | base | --- | [376, 385] | --- | --- | --- | --- |
| bc_iter | 375ns | no significant difference | [-9, +3]ns | [365, 383] | no | 0.6343 | 0.6343 | 1 |
| bc_unchecked | 673ns | +308.1ns (+81.3%) | [+290, +312]ns | [663, 711] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 404ns | -12.9% | +77.3% |
| 1 | 2 | 390ns | -9.9% | +83.9% |
| 1 | 3 | 399ns | +1.2% | +80.3% |
| 1 | 4 | 348ns | +17.4% | +84.5% |
| 1 | 5 | 371ns | +7.8% | +75.4% |
| 1 | 6 | 404ns | -13.4% | +56.5% |
| 1 | 7 | 365ns | +10.6% | +72.1% |
| 1 | 8 | 376ns | -2.6% | +89.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.138 | ok |
| bc_iter | -0.050 | ok |
| bc_unchecked | 0.343 | moderate+ |

**Consistency summary:**

- **bc_iter**: won 4/8, lost 4/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 3.6ns | 383.2ns | 0.9% |  |
| bc_iter | 3.4ns | 378.5ns | 0.9% |  |
| bc_unchecked | 3.3ns | 678.9ns | 0.5% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 353.1-413.2 ns)
    353.1 |#############
    356.1 |##########
    359.1 |#############
    362.1 |#############
    365.1 |##########
    368.1 |##################################
    371.1 |################################
    374.1 |#####################################
    377.1 |#####################
    380.1 |#############
    383.1 |#####
    386.1 |#####
    389.1 |
    392.2 |########
    395.2 |#############
    398.2 |#####################
    401.2 |########################################
    404.2 |#####################################
    407.2 |#####################
    410.2 |#####################
  (16 below, 4 above range)

bc_iter (n=160, range 347.6-414.1 ns)
    347.6 |#################################
    350.9 |########################################
    354.2 |##############
    357.6 |##############
    360.9 |########
    364.2 |##########
    367.5 |######
    370.9 |######
    374.2 |######
    377.5 |############
    380.8 |############
    384.1 |##
    387.5 |##
    390.8 |##
    394.1 |######
    397.4 |############
    400.8 |#######################
    404.1 |###############################
    407.4 |#####################
    410.7 |#####################
  (15 below, 8 above range)

bc_unchecked (n=160, range 621.2-731.1 ns)
    621.2 |##########
    626.7 |######
    632.2 |########
    637.7 |#########
    643.2 |#########
    648.7 |#####
    654.2 |####
    659.7 |########
    665.2 |########
    670.7 |######
    676.2 |#
    681.7 |#
    687.1 |
    692.6 |
    698.1 |#
    703.6 |
    709.1 |#################
    714.6 |########################################
    720.1 |#################
    725.6 |####
  (17 below, 4 above range)

```
