# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Highlights

Baseline for all deltas below: **fxpmul_hot**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fxpmul_hot is fastest but the noisiest (CV 7.9%)

fxpmul_hot wins on median (101 ns) yet has the highest variance (CV 7.9%), while fxpmul_precise is the steadiest (CV 6.8%, 111 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fxpmul_hot)

The baseline fxpmul_hot is the fastest (101 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader fxpmul_hot vs stability leader fxpmul_precise (+10% speed for 1.2x steadier)

fxpmul_hot is fastest (101 ns, CV 7.9%); fxpmul_precise gives up 9.6% median for 1.2x lower variance (CV 6.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 101.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.10x (fastest 101.5 ns, slowest 111.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5555666677770007 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 2368ns | 2475ns | 2159ns | 2392ns | 2505ns | base |
| fxpmul_precise | 2300ns | 2255ns | 2170ns | 2270ns | 2521ns | -2.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 100ns | 89ns | 110ns | base | 10.208 |
| fxpmul_precise | 113ns | 104ns | 125ns | +12.46% | 9.077 |

## Performance model

- Peak throughput: **11.534 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 10.094 | 87.5% |
| fxpmul_precise | 9.209 | 79.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 2368ns | 2368ns | base |
| fxpmul_precise | 2300ns | 2300ns | -2.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 101ns | base | --- | [98, 104] | --- | --- | --- | --- |
| fxpmul_precise | 111ns | +12.7ns (+12.5%) | [+8, +15]ns | [110, 112] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|---|
| 1 | 1 | 105ns | +6.5% |
| 1 | 2 | 91ns | +27.9% |
| 1 | 3 | 96ns | +10.4% |
| 1 | 4 | 104ns | +17.3% |
| 1 | 5 | 104ns | +5.0% |
| 1 | 6 | 105ns | +4.4% |
| 1 | 7 | 105ns | +2.9% |
| 1 | 8 | 102ns | +19.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.197 | ok |
| fxpmul_precise | -0.483 | moderate- |

**Consistency summary:**

- **fxpmul_precise**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 3.5ns | 100.3ns | 3.4% |  |
| fxpmul_precise | 3.1ns | 112.8ns | 2.7% |  |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 88.8-110.5 ns)
     88.8 |###
     89.9 |#########################
     91.0 |##########
     92.0 |#########################
     93.1 |########################################
     94.2 |#####################
     95.3 |#####################
     96.4 |##################
     97.5 |##################
     98.5 |#####################
     99.6 |#######
    100.7 |#############################
    101.8 |##########
    102.9 |#############################
    104.0 |#############################
    105.0 |##################
    106.1 |########################################
    107.2 |########################################
    108.3 |####################################
    109.4 |####################################
  (17 below, 10 above range)

fxpmul_precise (n=160, range 103.9-125.0 ns)
    103.9 |######
    105.0 |###########################
    106.0 |##########
    107.1 |###########################
    108.2 |########################################
    109.2 |######
    110.3 |###########################
    111.3 |########
    112.4 |#################################
    113.4 |######
    114.5 |############
    115.5 |##########
    116.6 |##############
    117.6 |######
    118.7 |##########
    119.8 |##
    120.8 |########
    121.9 |##
    122.9 |####
    124.0 |############
  (13 below, 15 above range)

```
