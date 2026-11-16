# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 2.5x slower than the field

ctz_loop (150 ns) is 2.5x the fastest (60 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_debruijn is fastest but the noisiest (CV 7.7%)

ctz_debruijn wins on median (60 ns) yet has the highest variance (CV 7.7%), while ctz_loop is the steadiest (CV 6.8%, 150 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (ctz_debruijn)

The baseline ctz_debruijn is the fastest (60 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (ctz_debruijn) is the fastest** at 59.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.52x (fastest 59.6 ns, slowest 150.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2149ns | 2154ns | 2117ns | 2149ns | 2180ns | base |
| ctz_intrinsic | 2152ns | 2154ns | 2114ns | 2152ns | 2194ns | +0.18% |
| ctz_loop | 2245ns | 2243ns | 2204ns | 2243ns | 2290ns | +4.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 61ns | 54ns | 67ns | base | 16.916 |
| ctz_intrinsic | 61ns | 54ns | 67ns | -0.05% | 16.925 |
| ctz_loop | 150ns | 138ns | 164ns | +148.57% | 6.806 |

## Performance model

- Peak throughput: **18.841 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 17.181 | 91.2% |
| ctz_intrinsic | 16.954 | 90.0% |
| ctz_loop | 6.809 | 36.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2149ns | 2149ns | base |
| ctz_intrinsic | 2152ns | 2152ns | +0.18% |
| ctz_loop | 2245ns | 2245ns | +4.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 60ns | base | --- | [59, 61] | --- | --- | --- | --- |
| ctz_intrinsic | 60ns | no significant difference | [-0, +0]ns | [59, 61] | no | 0.8682 | 0.8682 | 15 |
| ctz_loop | 150ns | +89.0ns (+149.3%) | [+87, +92]ns | [149, 152] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 60ns | +2.8% | +138.6% |
| 1 | 2 | 59ns | +0.3% | +158.8% |
| 1 | 3 | 60ns | +1.0% | +157.0% |
| 1 | 4 | 61ns | -0.7% | +137.2% |
| 1 | 5 | 60ns | +3.2% | +128.7% |
| 1 | 6 | 59ns | +2.1% | +165.3% |
| 1 | 7 | 60ns | -1.1% | +148.2% |
| 1 | 8 | 59ns | +1.7% | +167.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | 0.220 | moderate+ |
| ctz_intrinsic | -0.156 | ok |
| ctz_loop | -0.206 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 2/8, lost 6/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.5ns | 60.5ns | 4.1% |  |
| ctz_intrinsic | 2.6ns | 60.5ns | 4.2% |  |
| ctz_loop | 3.2ns | 150.5ns | 2.2% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 54.4-67.1 ns)
     54.4 |#############
     55.1 |##
     55.7 |################
     56.3 |################
     57.0 |########################################
     57.6 |################
     58.2 |########################
     58.9 |################################
     59.5 |################################
     60.1 |########
     60.8 |########################
     61.4 |##
     62.0 |#############
     62.7 |########
     63.3 |################
     63.9 |#####
     64.6 |##################################
     65.2 |##########################
     65.8 |##########
     66.5 |#####
  (15 below, 15 above range)

ctz_intrinsic (n=160, range 54.4-66.7 ns)
     54.4 |#####
     55.0 |######################
     55.6 |############################
     56.2 |###########
     56.8 |########
     57.4 |#########################
     58.1 |#################
     58.7 |########################################
     59.3 |###########
     59.9 |########################################
     60.5 |##############
     61.2 |#################
     61.8 |##############
     62.4 |###############################
     63.0 |##############
     63.6 |#################
     64.3 |########
     64.9 |##################################
     65.5 |###########
     66.1 |#################
  (12 below, 11 above range)

ctz_loop (n=160, range 138.1-163.9 ns)
    138.1 |########
    139.4 |################################
    140.7 |#############
    142.0 |##################
    143.3 |################
    144.6 |#############
    145.9 |##################
    147.1 |#####################
    148.4 |#############################
    149.7 |#############
    151.0 |################################
    152.3 |#############
    153.6 |########################
    154.9 |################
    156.1 |########################################
    157.4 |##################
    158.7 |#####
    160.0 |#####################
    161.3 |#####
    162.6 |########
  (14 below, 8 above range)

```
